//! Blockchain-verified porting primitives (v0.3.3).
//!
//! This module records cross-jurisdiction porting operations onto an append-only,
//! cryptographically linked ledger and layers higher-level distributed-ledger
//! features on top of it. Everything here is pure Rust and self-contained — no
//! network calls are required for the core algorithms:
//!
//! - [`ledger`] — an immutable, hash-linked block-chain of [`PortedStatute`]
//!   records anchored by a Merkle root per block, with proof-of-work sealing,
//!   full-chain validation and compact inclusion proofs. This delivers
//!   *immutable porting records* and the *cryptographic audit trail*.
//! - [`consensus`] — a decentralized, multi-party approval mechanism. Approvers
//!   are scoped to jurisdictions and carry voting stake; a proposal commits once
//!   a stake quorum is reached *and* (for cross-border ports) both the source and
//!   target jurisdictions have approved. Proposer selection supports
//!   round-robin proof-of-authority and deterministic weighted-stake election,
//!   and equivocation is detected.
//! - [`contract`] — a deterministic, gas-metered rule engine that *gates a port*
//!   on a set of conditions being met. Gates reuse [`legalis_core::Condition`]
//!   (evaluated against ported-statute attributes) alongside porting-specific
//!   predicates (compatibility floor, change budget, consensus, notarization).
//! - [`notary`] — cross-border digital notarization. Notaries are bound to
//!   jurisdictions and produce keyed-hash attestations (signatures-as-data) over
//!   a document hash; a cross-border notarization is complete only once notaries
//!   from both the source and target jurisdictions have attested.
//!
//! # Relationship to the rest of the crate
//!
//! The ledger commits the crate's own [`PortedStatute`] values — it does not
//! re-model what a port is. Errors are reported through the crate's existing
//! [`PortingError`] (predominantly [`PortingError::InvalidInput`]) so callers do
//! not have to learn a second error vocabulary.
//!
//! # Deferred external bindings
//!
//! Two bindings are intentionally deferred and abstracted behind data shapes so
//! they can be substituted without touching callers:
//!
//! - Settlement onto a *public* chain requires a live JSON-RPC endpoint and
//!   signing keys this offline workspace does not have.
//! - The notary attestation scheme is a self-contained keyed hash; swapping in a
//!   public-key signature (Ed25519/secp256k1) only changes how a
//!   [`notary::NotarySignature`] seal is produced and verified.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_i18n::Locale;
//! use legalis_porting::{PortedStatute, PortingChange};
//! use legalis_porting::blockchain::{PortingLedger, record_port};
//!
//! let ported = PortedStatute {
//!     original_id: "jp-civil-4".to_string(),
//!     statute: Statute::new("us-civil-4", "Adult Rights", Effect::new(EffectType::Grant, "Capacity")),
//!     changes: Vec::<PortingChange>::new(),
//!     locale: Locale::new("en").with_country("US"),
//!     compatibility_score: 0.92,
//! };
//!
//! let mut ledger = PortingLedger::new(2);
//! record_port(&mut ledger, &ported, "JP", "US", "registrar").unwrap();
//! ledger.seal_pending().unwrap();
//! assert!(ledger.validate().is_ok());
//! ```

use sha2::{Digest, Sha256};

pub mod consensus;
pub mod contract;
pub mod ledger;
pub mod notary;

pub use consensus::{
    ApprovalConsensus, ApprovalTally, ApprovalVote, Approver, ConsensusOutcome, ConsensusStatus,
    PortingProposal, SelectionMethod, VoteChoice,
};
pub use contract::{
    Clause, ClauseKind, ContractEngine, DEFAULT_GAS_LIMIT, EnforcementReceipt, Gate,
    PortingCovenant, PortingFacts, Violation,
};
pub use ledger::{
    Block, MerkleProof, MerkleTree, PortingLedger, PortingLedgerRecord, SiblingHash, record_port,
};
pub use notary::{
    CrossBorderNotarization, DigitalNotary, NotarizationStatus, NotaryRegistry, NotarySignature,
};

// Re-exported so downstream code can name the crate's own error/types alongside
// the blockchain API without a second `use` path.
pub use crate::PortedStatute;

/// The all-zero hash used as the genesis block's predecessor and as the Merkle
/// root of an empty leaf set.
pub(crate) const ZERO_HASH: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

/// Computes a lowercase hex SHA-256 digest over a single byte slice.
pub(crate) fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Computes a lowercase hex SHA-256 digest over several byte slices.
///
/// Each part is length-prefixed before hashing so that, for example,
/// `["ab", "c"]` and `["a", "bc"]` produce different digests (domain separation
/// against trivial concatenation collisions).
pub(crate) fn sha256_parts(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    hex::encode(hasher.finalize())
}

/// Current UNIX timestamp in seconds, saturating to `0` before the epoch.
///
/// Kept panic-free so ledger and notary construction never abort on a
/// misconfigured system clock.
pub(crate) fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Interprets the first 16 bytes of a hex digest as a big-endian `u128`.
///
/// Used by stake-weighted leader election to derive a reproducible point inside
/// the cumulative-stake interval. A malformed digest yields `0`, which is still
/// a valid (if biased toward the first validator) selection point.
pub(crate) fn hash_to_u128(hex_digest: &str) -> u128 {
    let bytes = hex::decode(hex_digest).unwrap_or_default();
    let mut value: u128 = 0;
    for byte in bytes.into_iter().take(16) {
        value = (value << 8) | byte as u128;
    }
    value
}

/// A short, deterministic party identifier derived from key material.
///
/// Parties (registrars, approvers, notaries) are identified on the ledger by a
/// `PartyId` rather than by raw key material. The identifier is `0x` followed by
/// the first 40 hex characters (20 bytes) of the SHA-256 of the material,
/// mirroring the 20-byte address convention used by account-based chains. Two
/// callers that start from the same material obtain the same `PartyId`, and the
/// material cannot be recovered from the identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PartyId(String);

impl PartyId {
    /// Derives a `PartyId` from arbitrary key material (e.g. a public key).
    pub fn from_key(material: &[u8]) -> Self {
        let digest = sha256_hex(material);
        Self(format!("0x{}", &digest[..40]))
    }

    /// Wraps an already-formatted identifier string verbatim.
    ///
    /// Useful for well-known labels such as a registry account.
    pub fn from_label(label: impl Into<String>) -> Self {
        Self(label.into())
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PartyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hex_is_deterministic_and_hex() {
        let a = sha256_hex(b"legalis-porting");
        let b = sha256_hex(b"legalis-porting");
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_sha256_parts_domain_separation() {
        let x = sha256_parts(&[b"ab", b"c"]);
        let y = sha256_parts(&[b"a", b"bc"]);
        assert_ne!(x, y);
    }

    #[test]
    fn test_party_id_from_key_deterministic() {
        let a = PartyId::from_key(b"notary-tokyo");
        let b = PartyId::from_key(b"notary-tokyo");
        let c = PartyId::from_key(b"notary-berlin");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a.as_str().starts_with("0x"));
        assert_eq!(a.as_str().len(), 42);
    }

    #[test]
    fn test_party_id_label_and_display() {
        let registry = PartyId::from_label("registry");
        assert_eq!(registry.as_str(), "registry");
        assert_eq!(format!("{}", registry), "registry");
    }

    #[test]
    fn test_hash_to_u128_stable_and_bounded() {
        let h = sha256_hex(b"seed");
        let a = hash_to_u128(&h);
        let b = hash_to_u128(&h);
        assert_eq!(a, b);
        assert_eq!(hash_to_u128("not-hex"), 0);
    }
}
