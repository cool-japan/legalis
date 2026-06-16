//! Quantum-proof integrity for audit trails.
//!
//! This module future-proofs the crate's tamper-evidence against quantum
//! adversaries using **hash-based cryptography** — the one integrity/signature
//! family that is both believed quantum-resistant and implementable in pure Rust
//! with no elliptic-curve or lattice machinery. It is strictly *additive* and
//! reuses the existing [`AuditRecord`](crate::AuditRecord) and
//! [`crate::integrity::MerkleTree`] types rather than re-modelling them.
//!
//! ## Building blocks
//! - [`pq_hash`] — clean-room **SHA-256 / SHA-512** (FIPS 180-4) and a
//!   post-quantum [`PqHashChain`] over records (crypto-agile via
//!   [`PqHashAlgorithm`]).
//! - [`signatures`] — quantum-resistant **hash-based signatures**: Lamport and
//!   Winternitz (WOTS) one-time schemes plus a [`MerkleSignatureScheme`]
//!   (XMSS-style) many-time scheme with a compact [`MerklePublicKey`] root.
//! - [`key_management`] — a [`QuantumKeyStore`] that enforces one-time-leaf
//!   reuse protection and key lifecycle (rotation / revocation / exhaustion).
//! - [`hybrid`] — [`HybridProof`]s that bind a record set with *both* the
//!   classical Merkle root and a post-quantum chain head + signature.
//! - [`beacon`] — a verifiable [`QuantumRandomBeacon`] over a pluggable
//!   [`EntropySource`] (the seam where real QRNG hardware would attach).
//!
//! ## Orchestration
//! [`QuantumIntegrityEngine`] ties a signing key (in an internal
//! [`QuantumKeyStore`]) to the hybrid-proof pipeline so callers can
//! [`seal`](QuantumIntegrityEngine::seal) and
//! [`verify`](QuantumIntegrityEngine::verify) a record set in one call, and spin
//! up a matching [`QuantumRandomBeacon`].

pub mod beacon;
pub mod hybrid;
pub mod key_management;
pub mod pq_hash;
pub mod signatures;

pub use beacon::{
    BeaconRound, EntropySource, QuantumRandomBeacon, SeededEntropySource, SystemEntropySource,
    verify_rounds,
};
pub use hybrid::HybridProof;
pub use key_management::{KeyStatus, ManagedKey, QuantumKeyStore};
// Note: the `pq_hash` *function* is intentionally not re-exported here because a
// `pq_hash` *module* already occupies this path; call `quantum::pq_hash::pq_hash`.
pub use pq_hash::{PqChainLink, PqHashAlgorithm, PqHashChain, sha256, sha512, to_hex};
pub use signatures::{
    LamportKeyPair, LamportPublicKey, LamportSecretKey, LamportSignature, MAX_MERKLE_HEIGHT,
    MerklePublicKey, MerkleSignature, MerkleSignatureScheme, WotsSecretKey, WotsSignature,
    wots_keygen, wots_recover_public, wots_sign, wots_verify,
};

use crate::{AuditRecord, AuditResult};
use uuid::Uuid;

/// High-level orchestrator for quantum-proof sealing and verification.
///
/// Owns a [`QuantumKeyStore`] with a single active signing key; each
/// [`seal`](Self::seal) consumes one one-time leaf of that key (rotate before
/// exhaustion).
pub struct QuantumIntegrityEngine {
    algorithm: PqHashAlgorithm,
    store: QuantumKeyStore,
    signing_key: Uuid,
}

impl QuantumIntegrityEngine {
    /// Creates an engine with a freshly generated signing key of the given
    /// algorithm and Merkle `height` (`2^height` available seals).
    pub fn new(algorithm: PqHashAlgorithm, height: u32) -> AuditResult<Self> {
        let mut store = QuantumKeyStore::new();
        let signing_key = store.generate(algorithm, height)?;
        Ok(Self {
            algorithm,
            store,
            signing_key,
        })
    }

    /// Algorithm in use.
    pub fn algorithm(&self) -> PqHashAlgorithm {
        self.algorithm
    }

    /// The current signing key's identifier.
    pub fn signing_key_id(&self) -> Uuid {
        self.signing_key
    }

    /// The public verification key for the current signing key.
    pub fn public_key(&self) -> AuditResult<MerklePublicKey> {
        self.store.public_key(self.signing_key)
    }

    /// Remaining one-time seals before the signing key is exhausted.
    pub fn remaining_seals(&self) -> AuditResult<u64> {
        Ok(self.store.get(self.signing_key)?.remaining())
    }

    /// Produces a [`HybridProof`] over `records`, consuming one one-time leaf.
    pub fn seal(&mut self, records: &[AuditRecord]) -> AuditResult<HybridProof> {
        HybridProof::build(records, self.algorithm, &mut self.store, self.signing_key)
    }

    /// Verifies a hybrid `proof` against `records`.
    pub fn verify(&self, records: &[AuditRecord], proof: &HybridProof) -> bool {
        proof.verify(records)
    }

    /// Rotates the signing key, returning the new key's identifier. Subsequent
    /// seals use the new key; proofs from the old key still verify against their
    /// embedded public key.
    pub fn rotate_signing_key(&mut self) -> AuditResult<Uuid> {
        let new_id = self.store.rotate(self.signing_key)?;
        self.signing_key = new_id;
        Ok(new_id)
    }

    /// Read-only access to the underlying key store.
    pub fn key_store(&self) -> &QuantumKeyStore {
        &self.store
    }

    /// Creates a verifiable randomness beacon using the engine's algorithm and
    /// the system CSPRNG.
    pub fn system_beacon(&self) -> QuantumRandomBeacon {
        QuantumRandomBeacon::with_system_source(self.algorithm)
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
                component: "engine".to_string(),
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

    #[test]
    fn test_engine_seal_and_verify() {
        let mut records: Vec<_> = (0..6).map(|i| record(&format!("s-{i}"))).collect();
        let mut previous: Option<String> = None;
        for r in records.iter_mut() {
            r.relink(previous.clone());
            previous = Some(r.record_hash.clone());
        }

        let mut engine = QuantumIntegrityEngine::new(PqHashAlgorithm::Sha256, 3).expect("engine");
        let proof = engine.seal(&records).expect("seal");
        assert!(engine.verify(&records, &proof));
        assert_eq!(engine.remaining_seals().expect("remaining"), 7);
    }

    #[test]
    fn test_engine_detects_tamper() {
        let mut records: Vec<_> = (0..4).map(|i| record(&format!("s-{i}"))).collect();
        let mut engine = QuantumIntegrityEngine::new(PqHashAlgorithm::Sha512, 2).expect("engine");
        let proof = engine.seal(&records).expect("seal");
        assert!(engine.verify(&records, &proof));
        records[0].statute_id = "tampered".to_string();
        assert!(!engine.verify(&records, &proof));
    }

    #[test]
    fn test_engine_rotation_preserves_old_proofs() {
        let records: Vec<_> = (0..3).map(|i| record(&format!("s-{i}"))).collect();
        let mut engine = QuantumIntegrityEngine::new(PqHashAlgorithm::Sha256, 2).expect("engine");
        let old_key = engine.signing_key_id();
        let proof = engine.seal(&records).expect("seal");

        let new_key = engine.rotate_signing_key().expect("rotate");
        assert_ne!(old_key, new_key);
        // Old proof still verifies (uses its embedded public key).
        assert!(engine.verify(&records, &proof));
        // New seals work under the new key.
        let proof2 = engine.seal(&records).expect("seal2");
        assert!(engine.verify(&records, &proof2));
    }

    #[test]
    fn test_engine_beacon() {
        let engine = QuantumIntegrityEngine::new(PqHashAlgorithm::Sha256, 2).expect("engine");
        let mut beacon = engine.system_beacon();
        beacon.next_round().expect("round");
        beacon.next_round().expect("round");
        assert!(beacon.verify_chain());
        assert_eq!(beacon.len(), 2);
    }
}
