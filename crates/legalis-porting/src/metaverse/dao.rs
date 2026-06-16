//! DAO governance porting.
//!
//! A [`DaoGovernance`] captures the on-chain governance of a decentralized
//! autonomous organization: token-weighted [`VotingPower`], a quorum
//! ([`QuorumRule`]), a passing [`VoteThreshold`], a [`TreasuryRule`] for spending
//! the treasury, and a set of [`DaoProposal`]s. A [`LegalEntityGovernance`]
//! captures the comparable governance of a conventional legal entity (a company
//! or cooperative): share/seat-weighted [`VotingClass`]es, a board, bylaws, and a
//! board-spend authority limit.
//!
//! The two are *isomorphic enough to port*: a token holder maps to a share
//! class, a quorum-of-supply maps to a quorum-of-shares, a super-majority token
//! threshold maps to a super-majority share threshold, and a treasury spend cap
//! maps to a board spend-authority limit. [`DaoGovernance::port_to_legal_entity`]
//! and [`LegalEntityGovernance::port_to_dao`] perform those mappings in both
//! directions and report the structural adaptations made.

use crate::PortingError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type DaoResult<T> = Result<T, PortingError>;

/// A holder's voting weight in a DAO, by governance-token balance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VotingPower {
    /// Holder identifier (address or member id).
    pub holder: String,
    /// Governance tokens held (the holder's raw voting weight).
    pub tokens: u64,
}

impl VotingPower {
    /// Creates a voting-power entry.
    pub fn new(holder: impl Into<String>, tokens: u64) -> Self {
        Self {
            holder: holder.into(),
            tokens,
        }
    }
}

/// The fraction of voting weight that must participate for a vote to be valid.
///
/// Stored as basis points (1/10000) to stay exact and serde-stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuorumRule {
    /// Required participation, in basis points of total voting weight.
    pub quorum_bps: u32,
}

impl QuorumRule {
    /// Creates a quorum rule from basis points (clamped to 0..=10000).
    pub fn from_bps(quorum_bps: u32) -> Self {
        Self {
            quorum_bps: quorum_bps.min(10_000),
        }
    }

    /// Creates a quorum rule from a percentage (clamped to 0..=100).
    pub fn from_percent(percent: u32) -> Self {
        Self::from_bps(percent.min(100) * 100)
    }

    /// Whether `participating` out of `total` meets the quorum.
    pub fn is_met(&self, participating: u64, total: u64) -> bool {
        if total == 0 {
            return self.quorum_bps == 0;
        }
        // participating / total >= quorum_bps / 10000, cross-multiplied.
        (participating as u128) * 10_000 >= (self.quorum_bps as u128) * (total as u128)
    }
}

/// The fraction of *cast* weight that must vote "yes" for a proposal to pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoteThreshold {
    /// Required "yes" share, in basis points of weight cast.
    pub threshold_bps: u32,
}

impl VoteThreshold {
    /// A simple majority (>50%): 5001 bps.
    pub fn simple_majority() -> Self {
        Self {
            threshold_bps: 5_001,
        }
    }

    /// A two-thirds super-majority: 6667 bps.
    pub fn super_majority() -> Self {
        Self {
            threshold_bps: 6_667,
        }
    }

    /// A threshold from basis points (clamped to 0..=10000).
    pub fn from_bps(threshold_bps: u32) -> Self {
        Self {
            threshold_bps: threshold_bps.min(10_000),
        }
    }

    /// Whether `yes` out of `cast` meets the threshold.
    pub fn is_met(&self, yes: u64, cast: u64) -> bool {
        if cast == 0 {
            return false;
        }
        (yes as u128) * 10_000 >= (self.threshold_bps as u128) * (cast as u128)
    }
}

/// Rules constraining how a DAO may spend its treasury.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreasuryRule {
    /// Total treasury balance (in the DAO's accounting unit).
    pub balance: u64,
    /// Maximum a single proposal may disburse without an elevated threshold.
    pub per_proposal_cap: u64,
    /// Threshold required for spends at or below the cap.
    pub standard_threshold: VoteThreshold,
    /// Threshold required for spends above the cap.
    pub elevated_threshold: VoteThreshold,
}

impl TreasuryRule {
    /// Creates a treasury rule.
    pub fn new(balance: u64, per_proposal_cap: u64) -> Self {
        Self {
            balance,
            per_proposal_cap,
            standard_threshold: VoteThreshold::simple_majority(),
            elevated_threshold: VoteThreshold::super_majority(),
        }
    }

    /// The threshold required to approve a spend of `amount`.
    pub fn threshold_for(&self, amount: u64) -> VoteThreshold {
        if amount > self.per_proposal_cap {
            self.elevated_threshold
        } else {
            self.standard_threshold
        }
    }
}

/// The lifecycle state of a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalState {
    /// Open for voting.
    Active,
    /// Passed (quorum met and threshold met).
    Passed,
    /// Failed (quorum or threshold not met).
    Rejected,
}

/// A governance proposal with tallied votes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaoProposal {
    /// Proposal identifier.
    pub id: String,
    /// Short title.
    pub title: String,
    /// Treasury amount requested (0 for non-spending proposals).
    pub requested_amount: u64,
    /// Weight voting "yes".
    pub yes_weight: u64,
    /// Weight voting "no".
    pub no_weight: u64,
    /// Weight abstaining (counts toward quorum, not toward the threshold).
    pub abstain_weight: u64,
}

impl DaoProposal {
    /// Creates a proposal with no votes yet.
    pub fn new(id: impl Into<String>, title: impl Into<String>, requested_amount: u64) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            requested_amount,
            yes_weight: 0,
            no_weight: 0,
            abstain_weight: 0,
        }
    }

    /// Total weight that participated (yes + no + abstain), counting toward
    /// quorum.
    pub fn participating_weight(&self) -> u64 {
        self.yes_weight
            .saturating_add(self.no_weight)
            .saturating_add(self.abstain_weight)
    }

    /// Weight cast for/against (yes + no), the denominator for the threshold.
    pub fn decisive_weight(&self) -> u64 {
        self.yes_weight.saturating_add(self.no_weight)
    }
}

/// The on-chain governance configuration of a DAO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaoGovernance {
    /// DAO identifier.
    pub id: String,
    /// Token holders and their balances.
    pub holders: Vec<VotingPower>,
    /// Participation quorum.
    pub quorum: QuorumRule,
    /// Default passing threshold (overridden for treasury spends).
    pub default_threshold: VoteThreshold,
    /// Treasury spending rules.
    pub treasury: TreasuryRule,
    /// Proposals.
    pub proposals: Vec<DaoProposal>,
}

impl DaoGovernance {
    /// Creates a DAO governance configuration.
    pub fn new(id: impl Into<String>, treasury: TreasuryRule) -> Self {
        Self {
            id: id.into(),
            holders: Vec::new(),
            quorum: QuorumRule::from_percent(20),
            default_threshold: VoteThreshold::simple_majority(),
            treasury,
            proposals: Vec::new(),
        }
    }

    /// Builder: sets the quorum.
    pub fn with_quorum(mut self, quorum: QuorumRule) -> Self {
        self.quorum = quorum;
        self
    }

    /// Builder: sets the default threshold.
    pub fn with_default_threshold(mut self, threshold: VoteThreshold) -> Self {
        self.default_threshold = threshold;
        self
    }

    /// Builder: adds a holder.
    pub fn with_holder(mut self, holder: VotingPower) -> Self {
        self.holders.push(holder);
        self
    }

    /// Builder: adds a proposal.
    pub fn with_proposal(mut self, proposal: DaoProposal) -> Self {
        self.proposals.push(proposal);
        self
    }

    /// Total governance tokens across all holders.
    pub fn total_voting_power(&self) -> u64 {
        self.holders
            .iter()
            .map(|h| h.tokens)
            .fold(0u64, u64::saturating_add)
    }

    /// Evaluates a proposal's outcome under this configuration.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if no proposal with `proposal_id`
    /// exists.
    pub fn evaluate(&self, proposal_id: &str) -> DaoResult<ProposalState> {
        let proposal = self
            .proposals
            .iter()
            .find(|p| p.id == proposal_id)
            .ok_or_else(|| {
                PortingError::InvalidInput(format!(
                    "DAO '{}': no proposal '{proposal_id}'",
                    self.id
                ))
            })?;
        let total = self.total_voting_power();
        if !self.quorum.is_met(proposal.participating_weight(), total) {
            return Ok(ProposalState::Rejected);
        }
        let threshold = if proposal.requested_amount > 0 {
            self.treasury.threshold_for(proposal.requested_amount)
        } else {
            self.default_threshold
        };
        if threshold.is_met(proposal.yes_weight, proposal.decisive_weight()) {
            Ok(ProposalState::Passed)
        } else {
            Ok(ProposalState::Rejected)
        }
    }

    /// Ports this DAO governance to a conventional legal entity.
    ///
    /// Token holders become a single common voting class whose seat weights are
    /// the token balances; the quorum and thresholds carry over as basis points;
    /// the treasury per-proposal cap becomes the board's spend-authority limit.
    /// Structural adaptations are recorded in the returned report.
    pub fn port_to_legal_entity(
        &self,
        entity_name: impl Into<String>,
    ) -> GovernancePortReport<LegalEntityGovernance> {
        let entity_name = entity_name.into();
        let mut notes = Vec::new();

        let mut class = VotingClass::new("common", "Common Members");
        for holder in &self.holders {
            class.add_seat(&holder.holder, holder.tokens);
        }
        notes.push(format!(
            "Mapped {} token holders to a single 'common' voting class ({} total seats)",
            self.holders.len(),
            class.total_seats()
        ));
        notes.push(format!(
            "Carried quorum {} bps and default threshold {} bps onto the entity",
            self.quorum.quorum_bps, self.default_threshold.threshold_bps
        ));
        notes.push(format!(
            "Mapped treasury per-proposal cap {} to board spend-authority limit",
            self.treasury.per_proposal_cap
        ));

        let entity = LegalEntityGovernance {
            id: entity_name,
            classes: vec![class],
            quorum_bps: self.quorum.quorum_bps,
            ordinary_resolution_bps: self.default_threshold.threshold_bps,
            special_resolution_bps: self.treasury.elevated_threshold.threshold_bps,
            board_spend_limit: self.treasury.per_proposal_cap,
            capital: self.treasury.balance,
        };

        GovernancePortReport {
            ported: entity,
            adaptations: notes,
            source_kind: "DAO".to_string(),
            target_kind: "LegalEntity".to_string(),
        }
    }
}

/// A class of shares/seats in a conventional legal entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VotingClass {
    /// Class identifier (e.g. "common", "preferred").
    pub id: String,
    /// Display name.
    pub name: String,
    /// Members and their seat/share counts.
    pub seats: BTreeMap<String, u64>,
}

impl VotingClass {
    /// Creates an empty voting class.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            seats: BTreeMap::new(),
        }
    }

    /// Adds (or accumulates) seats for a member.
    pub fn add_seat(&mut self, member: impl Into<String>, seats: u64) {
        *self.seats.entry(member.into()).or_insert(0) += seats;
    }

    /// Total seats across all members in this class.
    pub fn total_seats(&self) -> u64 {
        self.seats.values().copied().fold(0u64, u64::saturating_add)
    }
}

/// The governance configuration of a conventional legal entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalEntityGovernance {
    /// Entity identifier.
    pub id: String,
    /// Voting classes (share classes / membership classes).
    pub classes: Vec<VotingClass>,
    /// Quorum, in basis points of total seats.
    pub quorum_bps: u32,
    /// Threshold for an ordinary resolution, in basis points of seats voting.
    pub ordinary_resolution_bps: u32,
    /// Threshold for a special resolution, in basis points of seats voting.
    pub special_resolution_bps: u32,
    /// Maximum the board may authorise without a special resolution.
    pub board_spend_limit: u64,
    /// Capital / reserves available to the entity.
    pub capital: u64,
}

impl LegalEntityGovernance {
    /// Total seats across all classes.
    pub fn total_seats(&self) -> u64 {
        self.classes
            .iter()
            .map(VotingClass::total_seats)
            .fold(0u64, u64::saturating_add)
    }

    /// Ports this legal-entity governance back to a DAO.
    ///
    /// Every seat across every class becomes a token holder (the inverse of the
    /// flattening done on the way in); quorum and resolution thresholds carry
    /// over; the board spend limit becomes the treasury per-proposal cap. A
    /// single class round-trips exactly; multiple classes are merged with a note.
    pub fn port_to_dao(&self, dao_id: impl Into<String>) -> GovernancePortReport<DaoGovernance> {
        let dao_id = dao_id.into();
        let mut notes = Vec::new();

        let mut treasury = TreasuryRule::new(self.capital, self.board_spend_limit);
        treasury.standard_threshold = VoteThreshold::from_bps(self.ordinary_resolution_bps);
        treasury.elevated_threshold = VoteThreshold::from_bps(self.special_resolution_bps);

        let mut holders: BTreeMap<String, u64> = BTreeMap::new();
        for class in &self.classes {
            for (member, seats) in &class.seats {
                *holders.entry(member.clone()).or_insert(0) += *seats;
            }
        }
        if self.classes.len() > 1 {
            notes.push(format!(
                "Merged {} voting classes into a single token supply (DAOs use one fungible governance token)",
                self.classes.len()
            ));
        }
        notes.push(format!(
            "Mapped {} seat-holders to token holders ({} tokens total)",
            holders.len(),
            holders.values().copied().fold(0u64, u64::saturating_add)
        ));
        notes.push(format!(
            "Mapped board spend-authority limit {} to treasury per-proposal cap",
            self.board_spend_limit
        ));

        let dao = DaoGovernance {
            id: dao_id,
            holders: holders
                .into_iter()
                .map(|(holder, tokens)| VotingPower { holder, tokens })
                .collect(),
            quorum: QuorumRule::from_bps(self.quorum_bps),
            default_threshold: VoteThreshold::from_bps(self.ordinary_resolution_bps),
            treasury,
            proposals: Vec::new(),
        };

        GovernancePortReport {
            ported: dao,
            adaptations: notes,
            source_kind: "LegalEntity".to_string(),
            target_kind: "DAO".to_string(),
        }
    }
}

/// The result of porting a governance structure between representations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernancePortReport<T> {
    /// The ported governance structure.
    pub ported: T,
    /// Human-readable structural adaptations made during porting.
    pub adaptations: Vec<String>,
    /// The source representation kind ("DAO" or "LegalEntity").
    pub source_kind: String,
    /// The target representation kind.
    pub target_kind: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_dao() -> DaoGovernance {
        DaoGovernance::new("dao-aurora", TreasuryRule::new(1_000_000, 100_000))
            .with_quorum(QuorumRule::from_percent(25))
            .with_default_threshold(VoteThreshold::simple_majority())
            .with_holder(VotingPower::new("alice", 600))
            .with_holder(VotingPower::new("bob", 300))
            .with_holder(VotingPower::new("carol", 100))
    }

    #[test]
    fn test_quorum_met_basis_points() {
        let q = QuorumRule::from_percent(20);
        assert!(q.is_met(200, 1000));
        assert!(!q.is_met(199, 1000));
        assert!(QuorumRule::from_bps(0).is_met(0, 0));
    }

    #[test]
    fn test_quorum_clamps() {
        assert_eq!(QuorumRule::from_percent(150).quorum_bps, 10_000);
        assert_eq!(QuorumRule::from_bps(99_999).quorum_bps, 10_000);
    }

    #[test]
    fn test_vote_threshold_majority_boundary() {
        let t = VoteThreshold::simple_majority();
        assert!(t.is_met(501, 1000));
        assert!(!t.is_met(500, 1000));
        assert!(!t.is_met(1, 0));
    }

    #[test]
    fn test_super_majority() {
        let t = VoteThreshold::super_majority();
        assert!(t.is_met(667, 1000));
        assert!(!t.is_met(666, 1000));
    }

    #[test]
    fn test_treasury_threshold_escalation() {
        let treasury = TreasuryRule::new(1_000_000, 100_000);
        assert_eq!(
            treasury.threshold_for(50_000),
            VoteThreshold::simple_majority()
        );
        assert_eq!(
            treasury.threshold_for(150_000),
            VoteThreshold::super_majority()
        );
    }

    #[test]
    fn test_evaluate_passes_with_quorum_and_threshold() {
        let dao = sample_dao().with_proposal(DaoProposal {
            id: "p1".to_string(),
            title: "Fund grant".to_string(),
            requested_amount: 0,
            yes_weight: 500,
            no_weight: 100,
            abstain_weight: 50,
        });
        assert_eq!(dao.evaluate("p1").expect("eval"), ProposalState::Passed);
    }

    #[test]
    fn test_evaluate_fails_quorum() {
        let dao = sample_dao().with_proposal(DaoProposal {
            id: "p2".to_string(),
            title: "Tiny turnout".to_string(),
            requested_amount: 0,
            yes_weight: 100,
            no_weight: 0,
            abstain_weight: 0,
        });
        // 100 / 1000 = 10% < 25% quorum.
        assert_eq!(dao.evaluate("p2").expect("eval"), ProposalState::Rejected);
    }

    #[test]
    fn test_evaluate_treasury_spend_needs_super_majority() {
        let dao = sample_dao().with_proposal(DaoProposal {
            id: "p3".to_string(),
            title: "Big spend".to_string(),
            requested_amount: 200_000, // above cap -> super majority
            yes_weight: 550,
            no_weight: 450,
            abstain_weight: 0,
        });
        // 550/1000 = 55% yes, passes simple majority but not 66.67%.
        assert_eq!(dao.evaluate("p3").expect("eval"), ProposalState::Rejected);
    }

    #[test]
    fn test_evaluate_unknown_proposal_errors() {
        let dao = sample_dao();
        assert!(dao.evaluate("nope").is_err());
    }

    #[test]
    fn test_port_dao_to_legal_entity() {
        let dao = sample_dao();
        let report = dao.port_to_legal_entity("Aurora Co-op");
        assert_eq!(report.source_kind, "DAO");
        assert_eq!(report.target_kind, "LegalEntity");
        let entity = &report.ported;
        assert_eq!(entity.classes.len(), 1);
        assert_eq!(entity.total_seats(), 1000);
        assert_eq!(entity.quorum_bps, 2500);
        assert_eq!(entity.board_spend_limit, 100_000);
        assert!(!report.adaptations.is_empty());
    }

    #[test]
    fn test_port_legal_entity_to_dao() {
        let mut class = VotingClass::new("common", "Common");
        class.add_seat("alice", 600);
        class.add_seat("bob", 400);
        let entity = LegalEntityGovernance {
            id: "Co".to_string(),
            classes: vec![class],
            quorum_bps: 3000,
            ordinary_resolution_bps: 5001,
            special_resolution_bps: 6667,
            board_spend_limit: 50_000,
            capital: 500_000,
        };
        let report = entity.port_to_dao("dao-co");
        let dao = &report.ported;
        assert_eq!(dao.total_voting_power(), 1000);
        assert_eq!(dao.quorum.quorum_bps, 3000);
        assert_eq!(dao.treasury.per_proposal_cap, 50_000);
        assert_eq!(dao.treasury.balance, 500_000);
    }

    #[test]
    fn test_dao_legal_entity_roundtrip_preserves_power() {
        let dao = sample_dao();
        let entity = dao.port_to_legal_entity("Aurora Co-op").ported;
        let back = entity.port_to_dao("dao-aurora-2").ported;
        assert_eq!(dao.total_voting_power(), back.total_voting_power());
        assert_eq!(dao.quorum.quorum_bps, back.quorum.quorum_bps);
        assert_eq!(
            dao.treasury.per_proposal_cap,
            back.treasury.per_proposal_cap
        );
        // Per-holder weights are preserved across the round trip.
        let mut original: Vec<_> = dao.holders.clone();
        let mut roundtripped: Vec<_> = back.holders.clone();
        original.sort_by(|a, b| a.holder.cmp(&b.holder));
        roundtripped.sort_by(|a, b| a.holder.cmp(&b.holder));
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn test_multi_class_entity_merges_to_dao_with_note() {
        let mut common = VotingClass::new("common", "Common");
        common.add_seat("alice", 100);
        let mut preferred = VotingClass::new("preferred", "Preferred");
        preferred.add_seat("bob", 200);
        let entity = LegalEntityGovernance {
            id: "Co".to_string(),
            classes: vec![common, preferred],
            quorum_bps: 2000,
            ordinary_resolution_bps: 5001,
            special_resolution_bps: 6667,
            board_spend_limit: 10_000,
            capital: 100_000,
        };
        let report = entity.port_to_dao("dao-co");
        assert_eq!(report.ported.total_voting_power(), 300);
        assert!(report.adaptations.iter().any(|n| n.contains("Merged")));
    }

    #[test]
    fn test_dao_serde_roundtrip() {
        let dao = sample_dao();
        let json = serde_json::to_string(&dao).expect("ser");
        let back: DaoGovernance = serde_json::from_str(&json).expect("de");
        assert_eq!(dao, back);
    }

    #[test]
    fn test_report_serde_roundtrip() {
        let report = sample_dao().port_to_legal_entity("Co");
        let json = serde_json::to_string(&report).expect("ser");
        let back: GovernancePortReport<LegalEntityGovernance> =
            serde_json::from_str(&json).expect("de");
        assert_eq!(report, back);
    }
}
