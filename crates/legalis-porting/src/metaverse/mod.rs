//! Metaverse legal-system porting primitives (v0.3.4).
//!
//! This module ports legal systems and rule sets *into, out of and between*
//! virtual worlds. It is the metaverse counterpart of the [`crate::blockchain`]
//! module: pure Rust, self-contained, reusing the crate's own [`PortedStatute`]
//! and routing every failure through the existing [`PortingError`] so callers do
//! not learn a second error vocabulary.
//!
//! Five capabilities are layered on a shared spatial/identity vocabulary defined
//! here:
//!
//! - [`virtual_world`] — *virtual-world jurisdiction porting*. A
//!   [`virtual_world::VirtualJurisdiction`] models a virtual world as a tree of
//!   [`VirtualSpace`]s (realms, servers, shards, parcels). Real-world
//!   territorial concepts are projected onto that space tree by a
//!   [`virtual_world::TerritorialProjection`], and real statutes are ported
//!   into the virtual world — or back out — with the spatial scope rewritten and
//!   recorded as [`crate::PortingChange`]s.
//! - [`digital_twin`] — *digital-twin legal systems*. A
//!   [`digital_twin::LegalDigitalTwin`] mirrors a physical jurisdiction's rule
//!   set and tracks per-rule [`digital_twin::SyncState`], surfacing divergence
//!   (drift) between the physical original and its virtual mirror and proposing
//!   re-synchronisation.
//! - [`dao`] — *DAO governance porting*. A [`dao::DaoGovernance`] (proposals,
//!   voting thresholds, quorum, treasury rules) is ported to/from a
//!   conventional [`dao::LegalEntityGovernance`] (board, bylaws, voting classes),
//!   mapping token-weighted voting onto share/seat-weighted voting and back.
//! - [`nft`] — *NFT rights portability*. An [`nft::NftRightsBundle`] models the
//!   bundle of rights attached to a token (ownership, license, royalty,
//!   transfer restrictions) and is ported across systems, re-expressing each
//!   right under the destination system's legal vocabulary.
//! - [`harmonization`] — *cross-metaverse legal harmonization*. A
//!   [`harmonization::HarmonizationEngine`] reconciles differing rule sets
//!   across multiple metaverse jurisdictions: it detects conflicts and applies a
//!   [`harmonization::HarmonizationStrategy`] to produce a single harmonized
//!   rule set plus a residual conflict report.
//!
//! # Relationship to the rest of the crate
//!
//! Nothing here re-models what a *port* is. Where a metaverse port produces a
//! statute, it produces a [`PortedStatute`] with [`crate::PortingChange`]s, so it
//! composes with the rest of the crate (and can be committed to the
//! [`crate::blockchain`] ledger unchanged).
//!
//! # Example
//!
//! ```
//! use legalis_porting::metaverse::{SpaceKind, VirtualSpace, VirtualJurisdiction};
//!
//! let realm = VirtualSpace::new("realm-aurora", "Aurora", SpaceKind::Realm)
//!     .with_child(VirtualSpace::new("srv-eu-1", "EU Shard 1", SpaceKind::Server));
//! let mut world = VirtualJurisdiction::new("mv-aurora", "Aurora Metaverse");
//! world.set_root(realm);
//! assert_eq!(world.space_count(), 2);
//! assert!(world.find_space("srv-eu-1").is_some());
//! ```

use sha2::{Digest, Sha256};

pub mod dao;
pub mod digital_twin;
pub mod harmonization;
pub mod nft;
pub mod virtual_world;

pub use dao::{
    DaoGovernance, DaoProposal, GovernancePortReport, LegalEntityGovernance, ProposalState,
    QuorumRule, TreasuryRule, VoteThreshold, VotingClass, VotingPower,
};
pub use digital_twin::{
    DivergenceKind, DivergenceReport, LegalDigitalTwin, MirroredRule, SyncOutcome, SyncState,
    TwinSyncPlan,
};
pub use harmonization::{
    Bound, ConflictKind, HarmonizationEngine, HarmonizationReport, HarmonizationStrategy,
    HarmonizedRule, MetaverseRule, RuleConflict,
};
pub use nft::{
    Enforceability, NftPort, NftRight, NftRightChange, NftRightKind, NftRightsBundle, RoyaltyTerms,
    TransferRestriction,
};
pub use virtual_world::{
    SpaceKind, SpaceScope, TerritorialProjection, VirtualJurisdiction, VirtualSpace,
    VirtualWorldPort,
};

/// Computes a lowercase hex SHA-256 digest over a single byte slice.
pub(crate) fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Computes a lowercase hex SHA-256 digest over several byte slices.
///
/// Each part is length-prefixed before hashing so distinct part boundaries
/// produce distinct digests (domain separation against trivial concatenation
/// collisions).
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
/// Kept panic-free so construction never aborts on a misconfigured clock.
pub(crate) fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// A stable, content-derived identifier for a metaverse artefact.
///
/// Used to give realms, twins, bundles and harmonized sets reproducible ids when
/// the caller does not supply one: two callers that hash the same logical
/// content obtain the same [`MetaverseId`], and the content cannot be recovered
/// from the identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetaverseId(String);

impl MetaverseId {
    /// Derives an id from arbitrary content, prefixed `mv:`.
    pub fn from_content(content: &[u8]) -> Self {
        Self(format!("mv:{}", &sha256_hex(content)[..32]))
    }

    /// Wraps an already-formatted identifier verbatim.
    pub fn from_label(label: impl Into<String>) -> Self {
        Self(label.into())
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MetaverseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hex_is_deterministic_and_hex() {
        let a = sha256_hex(b"metaverse");
        let b = sha256_hex(b"metaverse");
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
    fn test_metaverse_id_from_content_deterministic() {
        let a = MetaverseId::from_content(b"realm-aurora");
        let b = MetaverseId::from_content(b"realm-aurora");
        let c = MetaverseId::from_content(b"realm-nova");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a.as_str().starts_with("mv:"));
    }

    #[test]
    fn test_metaverse_id_label_and_display() {
        let id = MetaverseId::from_label("mv:custom");
        assert_eq!(id.as_str(), "mv:custom");
        assert_eq!(format!("{id}"), "mv:custom");
    }

    #[test]
    fn test_current_timestamp_monotone_nonzero() {
        let t = current_timestamp();
        assert!(t > 0);
    }
}
