//! Quantum key management for hash-based signatures.
//!
//! Hash-based one-time signatures (OTS) are catastrophically insecure if a key
//! signs two different messages. A many-time [`MerkleSignatureScheme`] is really
//! a bounded pool of `2^height` one-time leaves, so safe operation *requires*
//! tracking which leaves have been consumed and refusing to reuse them. That
//! state — together with key lifecycle (generation, rotation, revocation,
//! exhaustion) — is exactly what [`QuantumKeyStore`] provides.
//!
//! Secret material lives only in the compact per-key `seed`; the full Merkle
//! tree is rebuilt deterministically on demand and cached in memory (it is not
//! serialised), so a store round-trips through `serde` carrying only seeds,
//! public roots and usage counters.

use super::pq_hash::{PqHashAlgorithm, to_hex};
use super::signatures::{MerklePublicKey, MerkleSignature, MerkleSignatureScheme};
use crate::{AuditError, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Lifecycle status of a managed key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyStatus {
    /// Available for signing.
    Active,
    /// Superseded by a rotation; retained for verification only.
    Rotated,
    /// Administratively revoked; must not be used.
    Revoked,
    /// All one-time leaves consumed.
    Exhausted,
}

/// A managed hash-based key: metadata plus the compact secret seed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedKey {
    /// Stable identifier.
    pub id: Uuid,
    /// Hash algorithm.
    pub algorithm: PqHashAlgorithm,
    /// Merkle tree height (`2^height` one-time signatures available).
    pub height: u32,
    /// Compact secret seed (expands to the full key deterministically).
    seed: Vec<u8>,
    /// Public Merkle root.
    public_root: Vec<u8>,
    /// Number of one-time leaves consumed so far.
    pub used_leaves: u64,
    /// Lifecycle status.
    pub status: KeyStatus,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// When this key was rotated, if applicable.
    pub rotated_at: Option<DateTime<Utc>>,
    /// Identifier of the key that superseded this one, if rotated.
    pub successor: Option<Uuid>,
    /// Optional human-readable label.
    pub label: Option<String>,
}

impl ManagedKey {
    /// Total number of one-time signatures this key can ever produce.
    pub fn capacity(&self) -> u64 {
        1u64 << self.height
    }

    /// Remaining one-time signatures.
    pub fn remaining(&self) -> u64 {
        self.capacity().saturating_sub(self.used_leaves)
    }

    /// Whether the key can currently sign.
    pub fn is_signable(&self) -> bool {
        matches!(self.status, KeyStatus::Active) && self.remaining() > 0
    }

    /// The public verification key.
    pub fn public_key(&self) -> MerklePublicKey {
        MerklePublicKey {
            algorithm: self.algorithm,
            height: self.height,
            root: self.public_root.clone(),
        }
    }

    /// Hex fingerprint of the public root.
    pub fn fingerprint(&self) -> String {
        to_hex(&self.public_root)
    }
}

/// A store of managed hash-based keys with one-time-leaf reuse protection.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QuantumKeyStore {
    keys: HashMap<Uuid, ManagedKey>,
    /// In-memory cache of expanded schemes (rebuilt lazily; never serialised).
    #[serde(skip)]
    schemes: HashMap<Uuid, MerkleSignatureScheme>,
}

impl QuantumKeyStore {
    /// Creates an empty key store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a new active key with the given algorithm and height, seeded
    /// from system randomness, and returns its identifier.
    pub fn generate(&mut self, algorithm: PqHashAlgorithm, height: u32) -> AuditResult<Uuid> {
        let seed = random_seed(algorithm.digest_len());
        self.generate_from_seed(algorithm, height, &seed, None)
    }

    /// Generates a new active key from a caller-supplied `seed` (useful for
    /// deterministic provisioning and testing).
    pub fn generate_from_seed(
        &mut self,
        algorithm: PqHashAlgorithm,
        height: u32,
        seed: &[u8],
        label: Option<String>,
    ) -> AuditResult<Uuid> {
        let scheme = MerkleSignatureScheme::from_seed(algorithm, height, seed)?;
        let public = scheme.public_key();
        let id = Uuid::new_v4();
        let key = ManagedKey {
            id,
            algorithm,
            height,
            seed: seed.to_vec(),
            public_root: public.root.clone(),
            used_leaves: 0,
            status: KeyStatus::Active,
            created_at: Utc::now(),
            rotated_at: None,
            successor: None,
            label,
        };
        self.keys.insert(id, key);
        self.schemes.insert(id, scheme);
        Ok(id)
    }

    /// Returns the managed key metadata for `id`.
    pub fn get(&self, id: Uuid) -> AuditResult<&ManagedKey> {
        self.keys
            .get(&id)
            .ok_or_else(|| AuditError::InvalidRecord(format!("unknown key {id}")))
    }

    /// Returns the public verification key for `id`.
    pub fn public_key(&self, id: Uuid) -> AuditResult<MerklePublicKey> {
        Ok(self.get(id)?.public_key())
    }

    /// Number of keys held.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Whether the store holds no keys.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Lists all managed keys.
    pub fn keys(&self) -> impl Iterator<Item = &ManagedKey> {
        self.keys.values()
    }

    /// Lazily rebuilds (and caches) the expanded scheme for `id` from its seed.
    fn scheme_for(&mut self, id: Uuid) -> AuditResult<&MerkleSignatureScheme> {
        if !self.schemes.contains_key(&id) {
            let key = self
                .keys
                .get(&id)
                .ok_or_else(|| AuditError::InvalidRecord(format!("unknown key {id}")))?;
            let scheme = MerkleSignatureScheme::from_seed(key.algorithm, key.height, &key.seed)?;
            self.schemes.insert(id, scheme);
        }
        self.schemes
            .get(&id)
            .ok_or_else(|| AuditError::InvalidRecord(format!("unknown key {id}")))
    }

    /// Signs `message` with the next unused one-time leaf of key `id`,
    /// advancing the usage counter so no leaf is ever reused.
    ///
    /// Errors if the key is not active or has exhausted all of its one-time
    /// leaves.
    pub fn sign_next(&mut self, id: Uuid, message: &[u8]) -> AuditResult<MerkleSignature> {
        let (status, used, capacity) = {
            let key = self.get(id)?;
            (key.status, key.used_leaves, key.capacity())
        };
        if status != KeyStatus::Active {
            return Err(AuditError::InvalidRecord(format!(
                "key {id} is not active ({status:?})"
            )));
        }
        if used >= capacity {
            // Mark exhausted and refuse — reusing a leaf would break OTS safety.
            if let Some(key) = self.keys.get_mut(&id) {
                key.status = KeyStatus::Exhausted;
            }
            return Err(AuditError::InvalidRecord(format!(
                "key {id} has exhausted all {capacity} one-time signatures"
            )));
        }

        let leaf_index = used;
        let signature = {
            let scheme = self.scheme_for(id)?;
            scheme.sign(message, leaf_index)?
        };

        if let Some(key) = self.keys.get_mut(&id) {
            key.used_leaves += 1;
            if key.used_leaves >= capacity {
                key.status = KeyStatus::Exhausted;
            }
        }
        Ok(signature)
    }

    /// Rotates key `id`: marks it `Rotated` and generates a fresh active key
    /// with the same parameters, returning the new key's identifier.
    pub fn rotate(&mut self, id: Uuid) -> AuditResult<Uuid> {
        let (algorithm, height, label) = {
            let key = self.get(id)?;
            (key.algorithm, key.height, key.label.clone())
        };
        let seed = random_seed(algorithm.digest_len());
        let new_id = self.generate_from_seed(algorithm, height, &seed, label)?;
        if let Some(key) = self.keys.get_mut(&id) {
            if key.status == KeyStatus::Active {
                key.status = KeyStatus::Rotated;
            }
            key.rotated_at = Some(Utc::now());
            key.successor = Some(new_id);
        }
        Ok(new_id)
    }

    /// Revokes key `id` (it can no longer sign).
    pub fn revoke(&mut self, id: Uuid) -> AuditResult<()> {
        let key = self
            .keys
            .get_mut(&id)
            .ok_or_else(|| AuditError::InvalidRecord(format!("unknown key {id}")))?;
        key.status = KeyStatus::Revoked;
        Ok(())
    }
}

/// Draws `len` bytes of seed material from the system CSPRNG.
fn random_seed(len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len);
    while out.len() < len {
        let block = rand::random::<[u8; 32]>();
        let take = (len - out.len()).min(block.len());
        out.extend_from_slice(&block[..take]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_public_key() {
        let mut store = QuantumKeyStore::new();
        assert!(store.is_empty());
        let id = store
            .generate(PqHashAlgorithm::Sha256, 3)
            .expect("generate");
        assert_eq!(store.len(), 1);
        let key = store.get(id).expect("get");
        assert_eq!(key.capacity(), 8);
        assert_eq!(key.remaining(), 8);
        assert!(key.is_signable());
        assert!(!key.fingerprint().is_empty());
        let public = store.public_key(id).expect("public");
        assert_eq!(public.height, 3);
    }

    #[test]
    fn test_sign_next_advances_and_verifies() {
        let mut store = QuantumKeyStore::new();
        let id = store
            .generate_from_seed(PqHashAlgorithm::Sha256, 3, b"det-seed", None)
            .expect("generate");
        let public = store.public_key(id).expect("public");

        let sig0 = store.sign_next(id, b"msg-0").expect("sign 0");
        let sig1 = store.sign_next(id, b"msg-1").expect("sign 1");
        assert_eq!(sig0.leaf_index, 0);
        assert_eq!(sig1.leaf_index, 1);
        assert!(public.verify(b"msg-0", &sig0));
        assert!(public.verify(b"msg-1", &sig1));
        assert_eq!(store.get(id).expect("get").used_leaves, 2);
        assert_eq!(store.get(id).expect("get").remaining(), 6);
    }

    #[test]
    fn test_no_leaf_reuse_distinct_indices() {
        let mut store = QuantumKeyStore::new();
        let id = store
            .generate(PqHashAlgorithm::Sha256, 3)
            .expect("generate");
        let mut indices = std::collections::HashSet::new();
        for i in 0..8 {
            let sig = store
                .sign_next(id, format!("m{i}").as_bytes())
                .expect("sign");
            assert!(indices.insert(sig.leaf_index), "leaf index reused!");
        }
        assert_eq!(indices.len(), 8);
    }

    #[test]
    fn test_exhaustion_is_enforced() {
        let mut store = QuantumKeyStore::new();
        let id = store
            .generate(PqHashAlgorithm::Sha256, 1)
            .expect("generate");
        assert!(store.sign_next(id, b"a").is_ok());
        assert!(store.sign_next(id, b"b").is_ok());
        // Capacity is 2; the third signature must be refused.
        assert!(store.sign_next(id, b"c").is_err());
        assert_eq!(store.get(id).expect("get").status, KeyStatus::Exhausted);
        assert_eq!(store.get(id).expect("get").remaining(), 0);
    }

    #[test]
    fn test_rotation_links_successor() {
        let mut store = QuantumKeyStore::new();
        let old = store
            .generate(PqHashAlgorithm::Sha256, 2)
            .expect("generate");
        let new = store.rotate(old).expect("rotate");
        assert_ne!(old, new);
        let old_key = store.get(old).expect("get");
        assert_eq!(old_key.status, KeyStatus::Rotated);
        assert_eq!(old_key.successor, Some(new));
        assert!(old_key.rotated_at.is_some());
        assert!(store.get(new).expect("get").is_signable());
        // A rotated key must refuse to sign.
        assert!(store.sign_next(old, b"x").is_err());
    }

    #[test]
    fn test_revoke_blocks_signing() {
        let mut store = QuantumKeyStore::new();
        let id = store
            .generate(PqHashAlgorithm::Sha256, 2)
            .expect("generate");
        store.revoke(id).expect("revoke");
        assert_eq!(store.get(id).expect("get").status, KeyStatus::Revoked);
        assert!(store.sign_next(id, b"x").is_err());
    }

    #[test]
    fn test_serde_roundtrip_rebuilds_scheme() {
        let mut store = QuantumKeyStore::new();
        let id = store
            .generate_from_seed(
                PqHashAlgorithm::Sha256,
                3,
                b"persist-seed",
                Some("k".into()),
            )
            .expect("generate");
        let _ = store.sign_next(id, b"before").expect("sign");

        let json = serde_json::to_string(&store).expect("serialize");
        let mut restored: QuantumKeyStore = serde_json::from_str(&json).expect("deserialize");

        // Usage counter survived serialization.
        assert_eq!(restored.get(id).expect("get").used_leaves, 1);
        // Signing still works after deserialization (scheme rebuilt from seed),
        // and continues from the persisted leaf index.
        let public = restored.public_key(id).expect("public");
        let sig = restored.sign_next(id, b"after").expect("sign");
        assert_eq!(sig.leaf_index, 1);
        assert!(public.verify(b"after", &sig));
    }

    #[test]
    fn test_unknown_key_errors() {
        let store = QuantumKeyStore::new();
        assert!(store.get(Uuid::new_v4()).is_err());
    }
}
