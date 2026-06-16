//! Hybrid quantum-classical integrity proofs.
//!
//! During the migration to post-quantum cryptography, the prudent design is
//! *hybrid*: bind a record set with both a classical primitive and a
//! post-quantum one, and require **both** to verify. If the classical primitive
//! later falls to a quantum adversary, the post-quantum primitive still holds;
//! if a flaw is ever found in the (younger) post-quantum primitive, the classical
//! one still holds. Security is therefore at least as strong as the stronger of
//! the two.
//!
//! A [`HybridProof`] combines three existing pieces of this crate:
//! * the classical [`crate::integrity::MerkleTree`] root over the records,
//! * a [`super::pq_hash::PqHashChain`] head over the same records, and
//! * a hash-based [`super::signatures::MerkleSignature`] (from the
//!   [`super::key_management::QuantumKeyStore`]) over a digest binding both
//!   roots and the record count.

use super::key_management::QuantumKeyStore;
use super::pq_hash::{PqHashAlgorithm, PqHashChain, pq_hash, to_hex};
use super::signatures::{MerklePublicKey, MerkleSignature};
use crate::integrity::MerkleTree;
use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const HYBRID_BIND_TAG: u8 = 0x50;

/// A combined classical + post-quantum integrity proof over a set of records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridProof {
    /// Hash algorithm used for the post-quantum components.
    pub algorithm: PqHashAlgorithm,
    /// Number of records covered.
    pub record_count: usize,
    /// Classical Merkle root (hex), or `None` for an empty record set.
    pub classical_root: Option<String>,
    /// Post-quantum hash-chain head digest.
    pub pq_chain_head: Vec<u8>,
    /// Identifier of the signing key used.
    pub signing_key_id: Uuid,
    /// Public verification key for [`Self::pq_signature`].
    pub pq_public_key: MerklePublicKey,
    /// Hash-based signature over the bound digest of both roots.
    pub pq_signature: MerkleSignature,
    /// When the proof was produced.
    pub created_at: DateTime<Utc>,
}

/// Computes the digest that the post-quantum signature binds: a hash over the
/// classical root, the post-quantum chain head and the record count.
fn binding_digest(
    algorithm: PqHashAlgorithm,
    classical_root: &Option<String>,
    pq_chain_head: &[u8],
    record_count: usize,
) -> Vec<u8> {
    let mut buf = vec![HYBRID_BIND_TAG];
    match classical_root {
        Some(root) => {
            buf.push(1);
            buf.extend_from_slice(root.as_bytes());
        }
        None => buf.push(0),
    }
    buf.extend_from_slice(pq_chain_head);
    buf.extend_from_slice(&(record_count as u64).to_le_bytes());
    pq_hash(algorithm, &buf)
}

impl HybridProof {
    /// Builds a hybrid proof over `records`, signing with the next one-time leaf
    /// of `signing_key_id` in `store`.
    pub fn build(
        records: &[AuditRecord],
        algorithm: PqHashAlgorithm,
        store: &mut QuantumKeyStore,
        signing_key_id: Uuid,
    ) -> AuditResult<Self> {
        let classical_root = MerkleTree::from_records(records).root_hash();
        let pq_chain = PqHashChain::from_records(records, algorithm);
        let pq_chain_head = pq_chain.head();
        let record_count = records.len();

        let message = binding_digest(algorithm, &classical_root, &pq_chain_head, record_count);
        let pq_signature = store.sign_next(signing_key_id, &message)?;
        let pq_public_key = store.public_key(signing_key_id)?;

        Ok(Self {
            algorithm,
            record_count,
            classical_root,
            pq_chain_head,
            signing_key_id,
            pq_public_key,
            pq_signature,
            created_at: Utc::now(),
        })
    }

    /// Verifies the proof against `records`.
    ///
    /// Returns `true` only if the classical Merkle root recomputes, the
    /// post-quantum chain head recomputes, *and* the post-quantum signature over
    /// the binding digest verifies under the embedded public key.
    ///
    /// Note: the embedded public key must still be trusted out-of-band (pinned),
    /// exactly as for any signature scheme.
    pub fn verify(&self, records: &[AuditRecord]) -> bool {
        if records.len() != self.record_count {
            return false;
        }
        let classical_root = MerkleTree::from_records(records).root_hash();
        if classical_root != self.classical_root {
            return false;
        }
        let pq_chain = PqHashChain::from_records(records, self.algorithm);
        if pq_chain.head() != self.pq_chain_head {
            return false;
        }
        let message = binding_digest(
            self.algorithm,
            &self.classical_root,
            &self.pq_chain_head,
            self.record_count,
        );
        self.pq_public_key.verify(&message, &self.pq_signature)
    }

    /// Verifies the proof against `records` *and* checks the embedded public key
    /// matches a caller-pinned `trusted_key`.
    pub fn verify_with_pinned_key(
        &self,
        records: &[AuditRecord],
        trusted_key: &MerklePublicKey,
    ) -> bool {
        &self.pq_public_key == trusted_key && self.verify(records)
    }

    /// Post-quantum chain head as a hex string.
    pub fn pq_chain_head_hex(&self) -> String {
        to_hex(&self.pq_chain_head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn record(statute: &str) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "ok".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    fn chain(records: &mut [AuditRecord]) {
        // Link records into a valid classical hash chain.
        let mut previous: Option<String> = None;
        for r in records.iter_mut() {
            r.relink(previous.clone());
            previous = Some(r.record_hash.clone());
        }
    }

    #[test]
    fn test_hybrid_build_and_verify() {
        let mut records: Vec<_> = (0..5).map(|i| record(&format!("s-{i}"))).collect();
        chain(&mut records);

        let mut store = QuantumKeyStore::new();
        let key = store.generate(PqHashAlgorithm::Sha256, 3).expect("key");
        let proof =
            HybridProof::build(&records, PqHashAlgorithm::Sha256, &mut store, key).expect("build");
        assert!(proof.verify(&records));
        assert_eq!(proof.record_count, 5);
        assert!(proof.classical_root.is_some());
        assert!(!proof.pq_chain_head_hex().is_empty());
    }

    #[test]
    fn test_hybrid_detects_record_tamper() {
        let mut records: Vec<_> = (0..4).map(|i| record(&format!("s-{i}"))).collect();
        chain(&mut records);
        let mut store = QuantumKeyStore::new();
        let key = store.generate(PqHashAlgorithm::Sha256, 3).expect("key");
        let proof =
            HybridProof::build(&records, PqHashAlgorithm::Sha256, &mut store, key).expect("build");
        assert!(proof.verify(&records));

        // Tamper with a record: both classical root and pq head change.
        records[1].statute_id = "evil".to_string();
        assert!(!proof.verify(&records));
    }

    #[test]
    fn test_hybrid_detects_count_mismatch() {
        let mut records: Vec<_> = (0..4).map(|i| record(&format!("s-{i}"))).collect();
        chain(&mut records);
        let mut store = QuantumKeyStore::new();
        let key = store.generate(PqHashAlgorithm::Sha256, 3).expect("key");
        let proof =
            HybridProof::build(&records, PqHashAlgorithm::Sha256, &mut store, key).expect("build");
        records.pop();
        assert!(!proof.verify(&records));
    }

    #[test]
    fn test_hybrid_pinned_key() {
        let mut records: Vec<_> = (0..3).map(|i| record(&format!("s-{i}"))).collect();
        chain(&mut records);
        let mut store = QuantumKeyStore::new();
        let key = store.generate(PqHashAlgorithm::Sha512, 2).expect("key");
        let trusted = store.public_key(key).expect("public");
        let proof =
            HybridProof::build(&records, PqHashAlgorithm::Sha512, &mut store, key).expect("build");
        assert!(proof.verify_with_pinned_key(&records, &trusted));

        // A different key's public key must be rejected even though the proof
        // itself is internally valid.
        let other = store.generate(PqHashAlgorithm::Sha512, 2).expect("key2");
        let other_pub = store.public_key(other).expect("public2");
        assert!(!proof.verify_with_pinned_key(&records, &other_pub));
    }

    #[test]
    fn test_hybrid_empty_records() {
        let records: Vec<AuditRecord> = Vec::new();
        let mut store = QuantumKeyStore::new();
        let key = store.generate(PqHashAlgorithm::Sha256, 2).expect("key");
        let proof =
            HybridProof::build(&records, PqHashAlgorithm::Sha256, &mut store, key).expect("build");
        assert!(proof.verify(&records));
        assert_eq!(proof.record_count, 0);
        assert!(proof.classical_root.is_none());
    }
}
