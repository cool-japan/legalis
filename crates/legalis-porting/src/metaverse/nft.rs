//! NFT rights portability.
//!
//! An NFT is, legally, a *bundle of rights* attached to a token rather than the
//! token itself. [`NftRightsBundle`] models that bundle: who owns it, what
//! licence it conveys, what royalty follows it on resale, and what restrictions
//! constrain its transfer. Each entry is an [`NftRight`] tagged with an
//! [`NftRightKind`].
//!
//! Porting a bundle "across systems" means re-expressing each right under a
//! destination legal system's vocabulary and enforceability. A perpetual,
//! creator-set resale royalty enforced on-chain in a token system, for example,
//! is *re-expressed* as a contractual resale-royalty covenant in a common-law
//! system (where chattel royalties do not run with the good by default) — and the
//! change in enforceability is recorded. [`NftRightsBundle::port_to`] performs
//! that translation and returns an [`NftPort`] describing every adaptation.

use crate::PortingError;
use serde::{Deserialize, Serialize};

type NftResult<T> = Result<T, PortingError>;

/// The legal nature of a single right in an NFT bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NftRightKind {
    /// Title to the token itself (the "ownership" right).
    Ownership,
    /// A licence to use the underlying work (display, remix, commercial use).
    License,
    /// A resale royalty entitlement that follows the token.
    Royalty,
    /// A restriction constraining transfer (lockup, allowlist, soulbound).
    TransferRestriction,
    /// Moral / attribution rights of the original creator.
    MoralRights,
}

/// How strongly a right can be enforced in a given legal system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Enforceability {
    /// Self-executing (e.g. enforced by the token contract itself).
    OnChainAutomatic,
    /// Enforceable as a contract between parties.
    Contractual,
    /// Recognised but only enforceable through a court / registry.
    JudicialOnly,
    /// Not recognised in the destination system (informational only).
    Unrecognized,
}

/// The royalty terms attached to a [`NftRightKind::Royalty`] right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoyaltyTerms {
    /// Royalty rate on resale, in basis points (1/10000) of sale price.
    pub rate_bps: u32,
    /// Whether the royalty is perpetual (vs. expiring with the licence term).
    pub perpetual: bool,
}

impl RoyaltyTerms {
    /// Creates royalty terms (rate clamped to 0..=10000 bps).
    pub fn new(rate_bps: u32, perpetual: bool) -> Self {
        Self {
            rate_bps: rate_bps.min(10_000),
            perpetual,
        }
    }

    /// The royalty owed on a sale of `price`.
    pub fn royalty_on(&self, price: u64) -> u64 {
        ((price as u128) * (self.rate_bps as u128) / 10_000) as u64
    }
}

/// The kind of constraint a [`NftRightKind::TransferRestriction`] imposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferRestriction {
    /// Token cannot be transferred at all ("soulbound").
    NonTransferable,
    /// Transfers allowed only to addresses on an allowlist.
    Allowlist(Vec<String>),
    /// Transfers locked until a UNIX timestamp (seconds).
    LockupUntil(u64),
    /// Transfers require the issuer's approval.
    IssuerApproval,
}

impl TransferRestriction {
    /// A short machine tag for the restriction (used in change logs).
    pub fn tag(&self) -> &'static str {
        match self {
            TransferRestriction::NonTransferable => "non_transferable",
            TransferRestriction::Allowlist(_) => "allowlist",
            TransferRestriction::LockupUntil(_) => "lockup",
            TransferRestriction::IssuerApproval => "issuer_approval",
        }
    }
}

/// A single right within an NFT bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NftRight {
    /// The legal nature of this right.
    pub kind: NftRightKind,
    /// Free-text description of the right's substance.
    pub description: String,
    /// How enforceable the right currently is.
    pub enforceability: Enforceability,
    /// Royalty terms (present only for [`NftRightKind::Royalty`]).
    pub royalty: Option<RoyaltyTerms>,
    /// Transfer restriction (present only for
    /// [`NftRightKind::TransferRestriction`]).
    pub restriction: Option<TransferRestriction>,
}

impl NftRight {
    /// Creates a plain right (no royalty or restriction payload).
    pub fn new(
        kind: NftRightKind,
        description: impl Into<String>,
        enforceability: Enforceability,
    ) -> Self {
        Self {
            kind,
            description: description.into(),
            enforceability,
            royalty: None,
            restriction: None,
        }
    }

    /// Creates a royalty right.
    pub fn royalty(terms: RoyaltyTerms, enforceability: Enforceability) -> Self {
        Self {
            kind: NftRightKind::Royalty,
            description: format!(
                "{} bps resale royalty{}",
                terms.rate_bps,
                if terms.perpetual { " (perpetual)" } else { "" }
            ),
            enforceability,
            royalty: Some(terms),
            restriction: None,
        }
    }

    /// Creates a transfer-restriction right.
    pub fn transfer_restriction(
        restriction: TransferRestriction,
        enforceability: Enforceability,
    ) -> Self {
        Self {
            kind: NftRightKind::TransferRestriction,
            description: format!("transfer restriction: {}", restriction.tag()),
            enforceability,
            royalty: None,
            restriction: Some(restriction),
        }
    }
}

/// The bundle of rights attached to a single NFT.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NftRightsBundle {
    /// Token identifier (e.g. `chain:contract:token_id`).
    pub token_id: String,
    /// The legal system the bundle is currently expressed under.
    pub system: String,
    /// Current owner (holder of the [`NftRightKind::Ownership`] right).
    pub owner: String,
    /// The rights making up the bundle.
    pub rights: Vec<NftRight>,
}

impl NftRightsBundle {
    /// Creates a bundle owned by `owner`, expressed under `system`, with an
    /// implicit on-chain ownership right.
    pub fn new(
        token_id: impl Into<String>,
        system: impl Into<String>,
        owner: impl Into<String>,
    ) -> Self {
        let owner = owner.into();
        Self {
            token_id: token_id.into(),
            system: system.into(),
            rights: vec![NftRight::new(
                NftRightKind::Ownership,
                format!("token title held by {owner}"),
                Enforceability::OnChainAutomatic,
            )],
            owner,
        }
    }

    /// Builder: adds a right to the bundle.
    pub fn with_right(mut self, right: NftRight) -> Self {
        self.rights.push(right);
        self
    }

    /// Adds a right in place.
    pub fn add_right(&mut self, right: NftRight) {
        self.rights.push(right);
    }

    /// All rights of a given kind.
    pub fn rights_of(&self, kind: NftRightKind) -> Vec<&NftRight> {
        self.rights.iter().filter(|r| r.kind == kind).collect()
    }

    /// Whether the bundle conveys at least one right of `kind`.
    pub fn has_right(&self, kind: NftRightKind) -> bool {
        self.rights.iter().any(|r| r.kind == kind)
    }

    /// The aggregate resale royalty rate (bps), summing all royalty rights.
    pub fn total_royalty_bps(&self) -> u32 {
        self.rights
            .iter()
            .filter_map(|r| r.royalty.map(|t| t.rate_bps))
            .sum::<u32>()
            .min(10_000)
    }

    /// Whether transfer is currently blocked outright by any restriction.
    pub fn is_transfer_blocked(&self) -> bool {
        self.rights
            .iter()
            .any(|r| matches!(r.restriction, Some(TransferRestriction::NonTransferable)))
    }

    /// Ports this bundle to another legal `system`, re-expressing each right
    /// under that system's enforceability model.
    ///
    /// The supplied `enforceability_of` decides how each [`NftRightKind`] is
    /// enforced in the destination. Whenever a right's enforceability changes,
    /// the change is recorded; a right that becomes [`Enforceability::Unrecognized`]
    /// is retained but flagged. The ported bundle is returned inside an
    /// [`NftPort`] alongside the change log.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if porting to the same system (a
    /// no-op that almost certainly indicates a caller mistake).
    pub fn port_to<F>(
        &self,
        target_system: impl Into<String>,
        mut enforceability_of: F,
    ) -> NftResult<NftPort>
    where
        F: FnMut(NftRightKind, Enforceability) -> Enforceability,
    {
        let target_system = target_system.into();
        if target_system == self.system {
            return Err(PortingError::InvalidInput(format!(
                "NFT bundle '{}': source and target system are both '{}'",
                self.token_id, target_system
            )));
        }

        let mut changes = Vec::new();
        let mut downgraded = 0usize;
        let mut unrecognized = 0usize;
        let mut ported_rights = Vec::with_capacity(self.rights.len());

        for right in &self.rights {
            let new_enf = enforceability_of(right.kind, right.enforceability);
            if new_enf != right.enforceability {
                if new_enf > right.enforceability {
                    downgraded += 1;
                }
                if new_enf == Enforceability::Unrecognized {
                    unrecognized += 1;
                }
                changes.push(NftRightChange {
                    kind: right.kind,
                    from: right.enforceability,
                    to: new_enf,
                    detail: format!(
                        "{:?} re-expressed from {:?} to {:?} under '{}'",
                        right.kind, right.enforceability, new_enf, target_system
                    ),
                });
            }
            let mut ported_right = right.clone();
            ported_right.enforceability = new_enf;
            ported_rights.push(ported_right);
        }

        let ported = NftRightsBundle {
            token_id: self.token_id.clone(),
            system: target_system.clone(),
            owner: self.owner.clone(),
            rights: ported_rights,
        };

        // A faithful port preserves every right kind and downgrades none to
        // unrecognized; score reflects how much enforceability survived.
        let total = self.rights.len().max(1) as f64;
        let fidelity = 1.0 - (downgraded as f64 * 0.1 + unrecognized as f64 * 0.4) / total;
        let fidelity = fidelity.clamp(0.0, 1.0);

        Ok(NftPort {
            source_system: self.system.clone(),
            target_system,
            ported,
            changes,
            unrecognized_rights: unrecognized,
            fidelity,
        })
    }
}

/// A single change to a right's enforceability during porting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NftRightChange {
    /// The right kind that changed.
    pub kind: NftRightKind,
    /// Enforceability in the source system.
    pub from: Enforceability,
    /// Enforceability in the target system.
    pub to: Enforceability,
    /// Human-readable explanation.
    pub detail: String,
}

/// The result of porting an [`NftRightsBundle`] across systems.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NftPort {
    /// The system the bundle was ported from.
    pub source_system: String,
    /// The system the bundle was ported to.
    pub target_system: String,
    /// The re-expressed bundle.
    pub ported: NftRightsBundle,
    /// Per-right enforceability changes.
    pub changes: Vec<NftRightChange>,
    /// Number of rights that became unrecognized in the target system.
    pub unrecognized_rights: usize,
    /// Porting fidelity in `0.0..=1.0` (1.0 = every right survived fully).
    pub fidelity: f64,
}

impl NftPort {
    /// Whether every right kept full recognition (none unrecognized).
    pub fn is_fully_recognized(&self) -> bool {
        self.unrecognized_rights == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_system_enforceability(kind: NftRightKind, current: Enforceability) -> Enforceability {
        // A "native" token system: everything is on-chain automatic.
        let _ = current;
        match kind {
            NftRightKind::MoralRights => Enforceability::JudicialOnly,
            _ => Enforceability::OnChainAutomatic,
        }
    }

    fn common_law_enforceability(kind: NftRightKind, current: Enforceability) -> Enforceability {
        // A common-law system: on-chain self-execution does not exist, so most
        // rights drop to contractual; royalties that "run with the good" are not
        // recognised; moral rights are judicial.
        let _ = current;
        match kind {
            NftRightKind::Ownership => Enforceability::JudicialOnly,
            NftRightKind::License => Enforceability::Contractual,
            NftRightKind::Royalty => Enforceability::Contractual,
            NftRightKind::TransferRestriction => Enforceability::Contractual,
            NftRightKind::MoralRights => Enforceability::JudicialOnly,
        }
    }

    fn unrecognizing_enforceability(
        kind: NftRightKind,
        _current: Enforceability,
    ) -> Enforceability {
        match kind {
            NftRightKind::Royalty => Enforceability::Unrecognized,
            other => token_system_enforceability(other, Enforceability::OnChainAutomatic),
        }
    }

    fn sample_bundle() -> NftRightsBundle {
        NftRightsBundle::new("eth:0xabc:1", "token", "alice")
            .with_right(NftRight::new(
                NftRightKind::License,
                "non-commercial display",
                Enforceability::OnChainAutomatic,
            ))
            .with_right(NftRight::royalty(
                RoyaltyTerms::new(750, true),
                Enforceability::OnChainAutomatic,
            ))
            .with_right(NftRight::transfer_restriction(
                TransferRestriction::LockupUntil(2_000_000_000),
                Enforceability::OnChainAutomatic,
            ))
    }

    #[test]
    fn test_new_bundle_has_ownership_right() {
        let bundle = NftRightsBundle::new("t", "token", "alice");
        assert!(bundle.has_right(NftRightKind::Ownership));
        assert_eq!(bundle.rights_of(NftRightKind::Ownership).len(), 1);
    }

    #[test]
    fn test_royalty_terms_clamp_and_compute() {
        let terms = RoyaltyTerms::new(50_000, true);
        assert_eq!(terms.rate_bps, 10_000);
        let terms = RoyaltyTerms::new(500, false); // 5%
        assert_eq!(terms.royalty_on(1_000), 50);
    }

    #[test]
    fn test_total_royalty_bps_sums_and_clamps() {
        let bundle = NftRightsBundle::new("t", "token", "alice")
            .with_right(NftRight::royalty(
                RoyaltyTerms::new(6_000, true),
                Enforceability::OnChainAutomatic,
            ))
            .with_right(NftRight::royalty(
                RoyaltyTerms::new(6_000, true),
                Enforceability::OnChainAutomatic,
            ));
        assert_eq!(bundle.total_royalty_bps(), 10_000);
    }

    #[test]
    fn test_transfer_blocked_detection() {
        let blocked =
            NftRightsBundle::new("t", "token", "alice").with_right(NftRight::transfer_restriction(
                TransferRestriction::NonTransferable,
                Enforceability::OnChainAutomatic,
            ));
        assert!(blocked.is_transfer_blocked());
        assert!(!sample_bundle().is_transfer_blocked());
    }

    #[test]
    fn test_restriction_tags() {
        assert_eq!(
            TransferRestriction::NonTransferable.tag(),
            "non_transferable"
        );
        assert_eq!(
            TransferRestriction::Allowlist(vec!["a".to_string()]).tag(),
            "allowlist"
        );
        assert_eq!(TransferRestriction::LockupUntil(1).tag(), "lockup");
        assert_eq!(TransferRestriction::IssuerApproval.tag(), "issuer_approval");
    }

    #[test]
    fn test_port_to_same_system_errors() {
        let bundle = sample_bundle();
        assert!(
            bundle
                .port_to("token", token_system_enforceability)
                .is_err()
        );
    }

    #[test]
    fn test_port_to_common_law_downgrades_enforceability() {
        let bundle = sample_bundle();
        let port = bundle
            .port_to("common_law", common_law_enforceability)
            .expect("port");
        assert_eq!(port.source_system, "token");
        assert_eq!(port.target_system, "common_law");
        assert!(port.is_fully_recognized());
        // Ownership: OnChain -> Judicial, License/Royalty/Restriction: -> Contractual.
        assert!(!port.changes.is_empty());
        let owner_change = port
            .changes
            .iter()
            .find(|c| c.kind == NftRightKind::Ownership)
            .expect("ownership change");
        assert_eq!(owner_change.to, Enforceability::JudicialOnly);
        // Every right is preserved.
        assert_eq!(port.ported.rights.len(), bundle.rights.len());
        assert!(port.fidelity < 1.0 && port.fidelity > 0.0);
    }

    #[test]
    fn test_port_unrecognized_royalty_flagged() {
        let bundle = sample_bundle();
        let port = bundle
            .port_to("strict_chattel", unrecognizing_enforceability)
            .expect("port");
        assert_eq!(port.unrecognized_rights, 1);
        assert!(!port.is_fully_recognized());
        // The unrecognized royalty right is retained, just flagged.
        assert!(
            port.ported
                .rights
                .iter()
                .any(|r| r.kind == NftRightKind::Royalty
                    && r.enforceability == Enforceability::Unrecognized)
        );
        assert!(port.fidelity < 1.0);
    }

    #[test]
    fn test_port_preserves_owner_and_token_id() {
        let bundle = sample_bundle();
        let port = bundle
            .port_to("common_law", common_law_enforceability)
            .expect("port");
        assert_eq!(port.ported.owner, "alice");
        assert_eq!(port.ported.token_id, "eth:0xabc:1");
    }

    #[test]
    fn test_port_no_change_when_enforceability_identical() {
        let bundle = NftRightsBundle::new("t", "token", "alice").with_right(NftRight::new(
            NftRightKind::License,
            "display",
            Enforceability::OnChainAutomatic,
        ));
        // Target keeps everything on-chain automatic -> no changes recorded.
        let port = bundle
            .port_to("mirror_token", token_system_enforceability)
            .expect("port");
        assert!(port.changes.is_empty());
        assert!((port.fidelity - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bundle_serde_roundtrip() {
        let bundle = sample_bundle();
        let json = serde_json::to_string(&bundle).expect("ser");
        let back: NftRightsBundle = serde_json::from_str(&json).expect("de");
        assert_eq!(bundle, back);
    }

    #[test]
    fn test_port_serde_roundtrip() {
        let port = sample_bundle()
            .port_to("common_law", common_law_enforceability)
            .expect("port");
        let json = serde_json::to_string(&port).expect("ser");
        let back: NftPort = serde_json::from_str(&json).expect("de");
        assert_eq!(port, back);
    }
}
