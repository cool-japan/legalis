//! Post-quantum signing and verification of statute entries and versions.
//!
//! [`StatuteSigner`] wraps the stateful hash-based [`MerkleSigner`] and binds each
//! signature to a *statute commitment* — a domain-separated hash over the statute
//! id, version and canonical content digest. Binding the id and version means a
//! signature cannot be transplanted onto a different record or a different
//! version of the same record, and re-signing always consumes a fresh one-time
//! Merkle leaf.
//!
//! The produced [`SignedStatute`] is a self-contained, serializable record
//! carrying the signature, the public key and the content digest that was
//! committed, so a verifier needs only the original entry to re-check it.

use super::hash_sig::{MerklePublicKey, MerkleSignature, MerkleSigner, merkle_verify};
use super::{canonical_json_bytes, constant_time_eq, now_rfc3339, sha256, tagged_hash, to_hex};
use crate::{RegistryError, RegistryResult, StatuteEntry, StatuteRegistry};
use serde::{Deserialize, Serialize};

const CTX_STATUTE_COMMIT: &[u8] = b"statute-commitment";

/// Computes the 32-byte sha256 content digest over a canonicalized entry.
///
/// Shared with [`super::hybrid`] so both layers commit to the *same* notion of
/// statute content.
pub(crate) fn content_digest(entry: &StatuteEntry) -> RegistryResult<[u8; 32]> {
    let bytes = canonical_json_bytes(entry)?;
    Ok(sha256(&bytes))
}

/// Computes the signing commitment binding id, version and content digest.
fn statute_commitment(statute_id: &str, version: u32, content: &[u8; 32]) -> [u8; 32] {
    tagged_hash(
        CTX_STATUTE_COMMIT,
        &[statute_id.as_bytes(), &version.to_be_bytes(), content],
    )
}

/// A self-contained, post-quantum signature over a statute entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedStatute {
    /// Identifier of the signed statute.
    pub statute_id: String,
    /// Version of the entry that was signed.
    pub version: u32,
    /// Lowercase-hex sha256 content digest that was committed.
    pub content_hash: String,
    /// The hash-based Merkle signature over the statute commitment.
    pub signature: MerkleSignature,
    /// The signer's long-lived public key.
    pub public_key: MerklePublicKey,
    /// RFC 3339 timestamp of signing.
    pub signed_at: String,
}

impl SignedStatute {
    /// Verifies the signature against `entry`.
    ///
    /// Returns `Ok(false)` (never an error) for any cryptographic mismatch — a
    /// wrong id/version, altered content, or a forged signature. Errors are
    /// reserved for canonicalization failures of the supplied entry.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn verify(&self, entry: &StatuteEntry) -> RegistryResult<bool> {
        if entry.statute.id != self.statute_id || entry.version != self.version {
            return Ok(false);
        }
        let digest = content_digest(entry)?;
        let expected_hex = to_hex(&digest);
        if !constant_time_eq(expected_hex.as_bytes(), self.content_hash.as_bytes()) {
            return Ok(false);
        }
        let commitment = statute_commitment(&self.statute_id, self.version, &digest);
        Ok(merkle_verify(
            &commitment,
            &self.signature,
            &self.public_key,
        ))
    }

    /// Verifies the signature against the public key it carries *and* an
    /// externally trusted public key, guarding against an attacker who re-signs
    /// tampered content under their own key.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn verify_with_key(
        &self,
        entry: &StatuteEntry,
        trusted_key: &MerklePublicKey,
    ) -> RegistryResult<bool> {
        if &self.public_key != trusted_key {
            return Ok(false);
        }
        self.verify(entry)
    }
}

/// A stateful post-quantum signer for statute records.
#[derive(Debug, Clone)]
pub struct StatuteSigner {
    signer: MerkleSigner,
}

impl StatuteSigner {
    /// Builds a signer from a 32-byte master seed and a Merkle tree height
    /// (`2^height` one-time signatures).
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] if `height` is out of range.
    pub fn from_seed(seed: [u8; 32], height: u8) -> RegistryResult<Self> {
        Ok(Self {
            signer: MerkleSigner::from_seed(seed, height)?,
        })
    }

    /// The long-lived public key.
    #[must_use]
    pub fn public_key(&self) -> MerklePublicKey {
        self.signer.public_key()
    }

    /// Number of unused one-time signing leaves remaining.
    #[must_use]
    pub fn remaining(&self) -> u32 {
        self.signer.remaining()
    }

    /// Total number of one-time leaves (`2^height`).
    #[must_use]
    pub fn capacity(&self) -> u32 {
        self.signer.leaf_count()
    }

    /// Borrows the underlying Merkle signer (advanced: lets callers sign other
    /// post-quantum artifacts such as audit roots with the same key).
    pub fn merkle_signer_mut(&mut self) -> &mut MerkleSigner {
        &mut self.signer
    }

    /// Borrows the underlying Merkle signer immutably.
    #[must_use]
    pub fn merkle_signer(&self) -> &MerkleSigner {
        &self.signer
    }

    /// Signs a statute entry with one-time leaf `leaf_index`.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] if the leaf is out of range or
    /// already used; propagates canonicalization failures.
    pub fn sign_entry(
        &mut self,
        leaf_index: u32,
        entry: &StatuteEntry,
    ) -> RegistryResult<SignedStatute> {
        let digest = content_digest(entry)?;
        let commitment = statute_commitment(&entry.statute.id, entry.version, &digest);
        let signature = self.signer.sign(leaf_index, &commitment)?;
        Ok(SignedStatute {
            statute_id: entry.statute.id.clone(),
            version: entry.version,
            content_hash: to_hex(&digest),
            signature,
            public_key: self.signer.public_key(),
            signed_at: now_rfc3339(),
        })
    }

    /// Signs the latest version of a statute looked up from `registry`.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::StatuteNotFound`] if the statute is absent, plus
    /// any signing/canonicalization failure.
    pub fn sign_statute(
        &mut self,
        leaf_index: u32,
        registry: &StatuteRegistry,
        statute_id: &str,
    ) -> RegistryResult<SignedStatute> {
        let entry = registry
            .get_uncached(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;
        self.sign_entry(leaf_index, entry)
    }

    /// Signs a specific version of a statute looked up from `registry`.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::VersionNotFound`] if the version is absent, plus
    /// any signing/canonicalization failure.
    pub fn sign_version(
        &mut self,
        leaf_index: u32,
        registry: &StatuteRegistry,
        statute_id: &str,
        version: u32,
    ) -> RegistryResult<SignedStatute> {
        let entry = registry.get_version(statute_id, version)?;
        self.sign_entry(leaf_index, entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    fn entry(id: &str, title: &str) -> StatuteEntry {
        let statute = Statute::new(id, title, Effect::new(EffectType::Grant, "grant"));
        StatuteEntry::new(statute, "US")
    }

    #[test]
    fn test_sign_and_verify_entry() {
        let mut signer = StatuteSigner::from_seed([3u8; 32], 3).expect("signer");
        assert_eq!(signer.capacity(), 8);
        let item = entry("act-1", "An Act");
        let signed = signer.sign_entry(0, &item).expect("sign");
        assert_eq!(signed.statute_id, "act-1");
        assert_eq!(signed.version, 1);
        assert!(signed.verify(&item).expect("verify"));
        assert_eq!(signer.remaining(), 7);
    }

    #[test]
    fn test_verify_rejects_tampered_content() {
        let mut signer = StatuteSigner::from_seed([4u8; 32], 2).expect("signer");
        let item = entry("act-1", "Original");
        let signed = signer.sign_entry(0, &item).expect("sign");

        // A different title (content) must fail.
        let mut tampered = item.clone();
        tampered.statute.title = "Tampered".to_string();
        assert!(!signed.verify(&tampered).expect("verify tampered"));

        // A different id must fail.
        let other = entry("act-2", "Original");
        assert!(!signed.verify(&other).expect("verify other"));
    }

    #[test]
    fn test_verify_with_trusted_key() {
        let mut signer = StatuteSigner::from_seed([5u8; 32], 2).expect("signer");
        let trusted = signer.public_key();
        let item = entry("act-1", "An Act");
        let signed = signer.sign_entry(0, &item).expect("sign");
        assert!(
            signed
                .verify_with_key(&item, &trusted)
                .expect("verify trusted")
        );

        // An attacker key must be rejected even with a self-consistent signature.
        let mut attacker = StatuteSigner::from_seed([99u8; 32], 2).expect("attacker");
        let forged = attacker.sign_entry(0, &item).expect("forge");
        assert!(
            !forged
                .verify_with_key(&item, &trusted)
                .expect("verify forged")
        );
    }

    #[test]
    fn test_sign_from_registry_latest_and_version() {
        let mut registry = StatuteRegistry::new();
        registry.register(entry("act-1", "v1")).expect("register");
        registry
            .update(
                "act-1",
                Statute::new("act-1", "v2", Effect::new(EffectType::Grant, "grant")),
            )
            .expect("update");
        let mut signer = StatuteSigner::from_seed([6u8; 32], 3).expect("signer");

        let latest = signer
            .sign_statute(0, &registry, "act-1")
            .expect("sign latest");
        assert_eq!(latest.version, 2);
        let v1 = signer
            .sign_version(1, &registry, "act-1", 1)
            .expect("sign v1");
        assert_eq!(v1.version, 1);

        // The v1 signature verifies against the v1 entry, not the latest.
        let v1_entry = registry.get_version("act-1", 1).expect("v1 entry").clone();
        assert!(v1.verify(&v1_entry).expect("verify v1"));
        assert!(signer.sign_statute(2, &registry, "missing").is_err());
    }

    #[test]
    fn test_signed_statute_serde_roundtrip() {
        let mut signer = StatuteSigner::from_seed([7u8; 32], 2).expect("signer");
        let item = entry("act-1", "An Act");
        let signed = signer.sign_entry(0, &item).expect("sign");
        let json = serde_json::to_string(&signed).expect("ser");
        let back: SignedStatute = serde_json::from_str(&json).expect("de");
        assert_eq!(signed, back);
        assert!(back.verify(&item).expect("verify"));
    }
}
