//! Quantum-safe verification of the registry's audit (event) log.
//!
//! The registry already records every mutation as a [`RegistryEvent`]. This
//! module turns that log into a *tamper-evident, quantum-safe* structure:
//!
//! - Each event is committed with a quantum-resistant content hash
//!   ([`QuantumHashAlgorithm`]) and folded into a 32-byte commitment.
//! - The commitments are chained: every [`AuditChainLink`] hashes the previous
//!   link's hash together with the current event commitment and its sequence
//!   number, so altering, reordering, inserting or deleting any event breaks the
//!   chain from that point onward (a hash-chain / blockchain-style ledger).
//! - A Merkle root over the link hashes gives a single fixed-size commitment to
//!   the whole log, which can be post-quantum signed via [`SignedAuditTrail`]
//!   using the hash-based [`MerkleSigner`].
//!
//! Verification re-derives the trail from a fresh event slice and reports exactly
//! which sequence numbers diverged, plus whether the Merkle root, head hash and
//! post-quantum signature still hold.

use super::hash_sig::{MerklePublicKey, MerkleSignature, MerkleSigner, merkle_verify};
use super::hashing::QuantumHashAlgorithm;
use super::{
    SHA256_BYTES, canonical_json_bytes, constant_time_eq, from_hex_array, merkle_root, now_rfc3339,
    tagged_hash, to_hex,
};
use crate::{RegistryEvent, RegistryResult, StatuteRegistry};
use serde::{Deserialize, Serialize};

const CTX_GENESIS: &[u8] = b"audit-genesis";
const CTX_EVENT: &[u8] = b"audit-event";
const CTX_LINK: &[u8] = b"audit-link";

/// The genesis (pre-first-event) chain hash.
fn genesis_hash() -> [u8; SHA256_BYTES] {
    tagged_hash(CTX_GENESIS, &[])
}

/// One link in the tamper-evident audit hash-chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditChainLink {
    /// Zero-based position of the event in the log.
    pub sequence: u64,
    /// Lowercase-hex quantum-resistant content digest of the event.
    pub event_digest: String,
    /// Lowercase-hex hash of the preceding link (or genesis for the first).
    pub prev_hash: String,
    /// Lowercase-hex hash binding `prev_hash`, the event commitment and the
    /// sequence number.
    pub link_hash: String,
}

/// A tamper-evident, quantum-safe commitment over a registry's event log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuantumAuditTrail {
    /// Content-hash algorithm used for every event digest.
    pub algorithm: QuantumHashAlgorithm,
    /// The chain links, in event order.
    pub links: Vec<AuditChainLink>,
    /// Merkle root (hex) over the link hashes.
    pub merkle_root: String,
    /// Hash (hex) of the final link, or the genesis hash for an empty log.
    pub head_hash: String,
    /// RFC 3339 timestamp of construction.
    pub created_at: String,
}

impl QuantumAuditTrail {
    /// Builds a trail over a borrowed slice of events.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn from_events(
        events: &[&RegistryEvent],
        algorithm: QuantumHashAlgorithm,
    ) -> RegistryResult<Self> {
        let mut links = Vec::with_capacity(events.len());
        let mut link_commitments = Vec::with_capacity(events.len());
        let mut prev = genesis_hash();
        for (index, event) in events.iter().enumerate() {
            let canonical = canonical_json_bytes(event)?;
            let event_digest_bytes = algorithm.digest(&canonical);
            let folded = tagged_hash(CTX_EVENT, &[&event_digest_bytes]);
            let sequence = index as u64;
            let link = tagged_hash(CTX_LINK, &[&prev, &folded, &sequence.to_be_bytes()]);
            links.push(AuditChainLink {
                sequence,
                event_digest: to_hex(&event_digest_bytes),
                prev_hash: to_hex(&prev),
                link_hash: to_hex(&link),
            });
            link_commitments.push(link);
            prev = link;
        }
        let head_hash = to_hex(&prev);
        Ok(Self {
            algorithm,
            links,
            merkle_root: to_hex(&merkle_root(&link_commitments)),
            head_hash,
            created_at: now_rfc3339(),
        })
    }

    /// Builds a trail over a registry's full event log.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn from_registry(
        registry: &StatuteRegistry,
        algorithm: QuantumHashAlgorithm,
    ) -> RegistryResult<Self> {
        let events = registry.all_events();
        Self::from_events(&events, algorithm)
    }

    /// Number of events committed.
    #[must_use]
    pub fn len(&self) -> usize {
        self.links.len()
    }

    /// Whether the trail commits to no events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }

    /// Re-derives a trail from `events` and reports how it compares to `self`.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn verify(&self, events: &[&RegistryEvent]) -> RegistryResult<AuditVerification> {
        let recomputed = Self::from_events(events, self.algorithm)?;
        let event_count_matches = recomputed.links.len() == self.links.len();

        let mut tampered_sequences = Vec::new();
        let max_len = recomputed.links.len().max(self.links.len());
        for index in 0..max_len {
            match (self.links.get(index), recomputed.links.get(index)) {
                (Some(stored), Some(fresh)) if stored.link_hash == fresh.link_hash => {}
                _ => tampered_sequences.push(index as u64),
            }
        }

        let merkle_root_matches = constant_time_eq(
            self.merkle_root.as_bytes(),
            recomputed.merkle_root.as_bytes(),
        );
        let head_hash_matches =
            constant_time_eq(self.head_hash.as_bytes(), recomputed.head_hash.as_bytes());
        let chain_intact = tampered_sequences.is_empty() && event_count_matches;

        Ok(AuditVerification {
            event_count_matches,
            chain_intact,
            merkle_root_matches,
            head_hash_matches,
            signature_valid: None,
            tampered_sequences,
            verified: chain_intact && merkle_root_matches && head_hash_matches,
        })
    }

    /// Post-quantum signs this trail's Merkle root with `signer`, consuming
    /// one-time leaf `leaf_index`.
    ///
    /// # Errors
    ///
    /// Returns [`crate::RegistryError::InvalidOperation`] if the leaf is unusable
    /// or the Merkle root is malformed.
    pub fn sign(
        &self,
        signer: &mut MerkleSigner,
        leaf_index: u32,
    ) -> RegistryResult<SignedAuditTrail> {
        let root = from_hex_array::<SHA256_BYTES>(&self.merkle_root)?;
        let signature = signer.sign(leaf_index, &root)?;
        Ok(SignedAuditTrail {
            public_key: signer.public_key(),
            trail: self.clone(),
            signature,
            signed_at: now_rfc3339(),
        })
    }
}

/// The result of verifying a [`QuantumAuditTrail`] against a fresh event slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditVerification {
    /// Whether the number of events matches.
    pub event_count_matches: bool,
    /// Whether the hash-chain is fully intact (no link diverged, same length).
    pub chain_intact: bool,
    /// Whether the Merkle root still matches.
    pub merkle_root_matches: bool,
    /// Whether the head (latest link) hash still matches.
    pub head_hash_matches: bool,
    /// Whether the post-quantum signature verified (`None` when unsigned).
    pub signature_valid: Option<bool>,
    /// Sequence numbers at which the stored and recomputed chains diverged.
    pub tampered_sequences: Vec<u64>,
    /// Overall verdict: chain intact, Merkle root and head match, and (if
    /// present) the signature is valid.
    pub verified: bool,
}

/// A [`QuantumAuditTrail`] sealed with a post-quantum signature over its Merkle
/// root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedAuditTrail {
    /// The committed audit trail.
    pub trail: QuantumAuditTrail,
    /// Hash-based Merkle signature over the trail's Merkle root.
    pub signature: MerkleSignature,
    /// The signer's long-lived public key.
    pub public_key: MerklePublicKey,
    /// RFC 3339 timestamp of signing.
    pub signed_at: String,
}

impl SignedAuditTrail {
    /// Verifies the embedded trail against `events` *and* the post-quantum
    /// signature over its Merkle root.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn verify(&self, events: &[&RegistryEvent]) -> RegistryResult<AuditVerification> {
        let mut verification = self.trail.verify(events)?;
        let signature_valid = match from_hex_array::<SHA256_BYTES>(&self.trail.merkle_root) {
            Ok(root) => merkle_verify(&root, &self.signature, &self.public_key),
            Err(_) => false,
        };
        verification.signature_valid = Some(signature_valid);
        verification.verified = verification.verified && signature_valid;
        Ok(verification)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StatuteEntry, StatuteRegistry};
    use legalis_core::{Effect, EffectType, Statute};

    fn registry_with_events() -> StatuteRegistry {
        let mut registry = StatuteRegistry::new();
        for idx in 0..3 {
            let statute = Statute::new(
                format!("act-{idx}"),
                "An Act",
                Effect::new(EffectType::Grant, "grant"),
            );
            registry
                .register(StatuteEntry::new(statute, "US"))
                .expect("register");
        }
        registry
            .set_status("act-0", crate::StatuteStatus::Active)
            .expect("status");
        registry
    }

    #[test]
    fn test_trail_commits_to_events() {
        let registry = registry_with_events();
        let trail = QuantumAuditTrail::from_registry(&registry, QuantumHashAlgorithm::default())
            .expect("t");
        assert_eq!(trail.len(), registry.event_count());
        assert!(!trail.is_empty());
        // First link chains from genesis.
        assert_eq!(trail.links[0].prev_hash, to_hex(&genesis_hash()));
        // Subsequent links chain from the previous link hash.
        assert_eq!(trail.links[1].prev_hash, trail.links[0].link_hash);
        assert_eq!(trail.head_hash, trail.links.last().expect("last").link_hash);
    }

    #[test]
    fn test_verify_intact_trail() {
        let registry = registry_with_events();
        let trail =
            QuantumAuditTrail::from_registry(&registry, QuantumHashAlgorithm::Sha512).expect("t");
        let events = registry.all_events();
        let verification = trail.verify(&events).expect("verify");
        assert!(verification.chain_intact);
        assert!(verification.merkle_root_matches);
        assert!(verification.head_hash_matches);
        assert!(verification.verified);
        assert!(verification.tampered_sequences.is_empty());
        assert_eq!(verification.signature_valid, None);
    }

    #[test]
    fn test_verify_detects_reordering() {
        let registry = registry_with_events();
        let trail = QuantumAuditTrail::from_registry(&registry, QuantumHashAlgorithm::default())
            .expect("t");
        let mut events = registry.all_events();
        events.swap(0, 1);
        let verification = trail.verify(&events).expect("verify");
        assert!(!verification.chain_intact);
        assert!(!verification.verified);
        // The divergence starts at the first swapped position.
        assert!(verification.tampered_sequences.contains(&0));
    }

    #[test]
    fn test_verify_detects_deletion() {
        let registry = registry_with_events();
        let trail = QuantumAuditTrail::from_registry(&registry, QuantumHashAlgorithm::default())
            .expect("t");
        let mut events = registry.all_events();
        events.pop();
        let verification = trail.verify(&events).expect("verify");
        assert!(!verification.event_count_matches);
        assert!(!verification.chain_intact);
        assert!(!verification.verified);
    }

    #[test]
    fn test_signed_trail_roundtrip_and_tamper() {
        let registry = registry_with_events();
        let trail = QuantumAuditTrail::from_registry(&registry, QuantumHashAlgorithm::default())
            .expect("t");
        let mut signer = MerkleSigner::from_seed([21u8; 32], 3).expect("signer");
        let signed = trail.sign(&mut signer, 0).expect("sign");

        let events = registry.all_events();
        let ok = signed.verify(&events).expect("verify");
        assert_eq!(ok.signature_valid, Some(true));
        assert!(ok.verified);

        // Tampering the events invalidates the chain (signature still valid over
        // the now-mismatched stored root, but the overall verdict is false).
        let mut tampered = events.clone();
        tampered.swap(0, 2);
        let bad = signed.verify(&tampered).expect("verify");
        assert!(!bad.chain_intact);
        assert!(!bad.verified);

        // Serde roundtrip preserves verifiability.
        let json = serde_json::to_string(&signed).expect("ser");
        let back: SignedAuditTrail = serde_json::from_str(&json).expect("de");
        assert!(back.verify(&events).expect("verify back").verified);
    }

    #[test]
    fn test_empty_log_trail() {
        let registry = StatuteRegistry::new();
        let trail = QuantumAuditTrail::from_registry(&registry, QuantumHashAlgorithm::default())
            .expect("t");
        assert!(trail.is_empty());
        assert_eq!(trail.head_hash, to_hex(&genesis_hash()));
        let events = registry.all_events();
        assert!(trail.verify(&events).expect("verify").verified);
    }
}
