//! Decentralized approval consensus for cross-jurisdiction ports.
//!
//! An [`ApprovalConsensus`] engine coordinates a set of weighted, jurisdiction
//! scoped [`Approver`]s that vote on whether a proposed port (a
//! [`PortingProposal`]) should be committed. It combines three complementary
//! mechanisms:
//!
//! - **Proof-of-authority** round-robin proposer election among authority
//!   approvers ([`ApprovalConsensus::next_proposer`]).
//! - **Weighted proof-of-stake** deterministic proposer election, where the
//!   chance of selection is proportional to stake and reproducible from a seed
//!   ([`ApprovalConsensus::weighted_proposer`]).
//! - A **Byzantine-fault-tolerant voting tally** that commits a proposal once
//!   approving stake reaches a configurable quorum (2/3 by default) *and*, for
//!   cross-border ports, both the source and target jurisdictions have at least
//!   one approving approver (a dual-jurisdiction "two-key" rule). It rejects a
//!   proposal once either requirement becomes unreachable, and detects
//!   equivocation (an approver casting conflicting votes on the same proposal).
//!
//! The dual-jurisdiction rule is what makes this *porting* consensus rather than
//! a generic ledger vote: a statute crossing a border is only committed when
//! both sides of the border have signed off, even if one well-staked side could
//! otherwise meet the quorum alone.

use super::ledger::PortingLedgerRecord;
use super::{hash_to_u128, sha256_parts};
use crate::PortingError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

type ConsensusResult<T> = Result<T, PortingError>;

/// A consensus participant with a home jurisdiction and voting weight.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Approver {
    /// Unique approver identifier.
    pub id: String,
    /// The jurisdiction this approver represents (e.g. `"JP"`).
    pub jurisdiction: String,
    /// Voting weight / stake (must be non-zero to count).
    pub stake: u64,
    /// Whether this approver may propose under proof-of-authority.
    pub authority: bool,
}

impl Approver {
    /// Creates an authority approver for `jurisdiction` with the given stake.
    pub fn authority(id: impl Into<String>, jurisdiction: impl Into<String>, stake: u64) -> Self {
        Self {
            id: id.into(),
            jurisdiction: jurisdiction.into(),
            stake,
            authority: true,
        }
    }

    /// Creates a non-authority (voting-only) approver for `jurisdiction`.
    pub fn voter(id: impl Into<String>, jurisdiction: impl Into<String>, stake: u64) -> Self {
        Self {
            id: id.into(),
            jurisdiction: jurisdiction.into(),
            stake,
            authority: false,
        }
    }
}

/// How a proposer is selected for a round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMethod {
    /// Round-robin over authority approvers.
    RoundRobin,
    /// Stake-weighted, deterministic from a seed.
    WeightedStake,
}

/// An approver's choice on a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteChoice {
    /// The approver accepts the proposed port.
    Approve,
    /// The approver rejects the proposed port.
    Reject,
}

/// A single cast vote.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalVote {
    /// The voting approver's id.
    pub approver_id: String,
    /// The proposal being voted on.
    pub proposal_hash: String,
    /// The approver's choice.
    pub choice: VoteChoice,
}

impl ApprovalVote {
    /// Creates a vote.
    pub fn new(
        approver_id: impl Into<String>,
        proposal_hash: impl Into<String>,
        choice: VoteChoice,
    ) -> Self {
        Self {
            approver_id: approver_id.into(),
            proposal_hash: proposal_hash.into(),
            choice,
        }
    }
}

/// A port put forward for consensus approval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortingProposal {
    /// Content hash uniquely identifying the proposal (the record leaf hash).
    pub hash: String,
    /// Source statute that was ported.
    pub original_id: String,
    /// Source jurisdiction code.
    pub source_jurisdiction: String,
    /// Target jurisdiction code.
    pub target_jurisdiction: String,
    /// The approver that proposed it.
    pub proposer: String,
}

/// Whether a proposal has reached a terminal decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusStatus {
    /// Quorum (and dual-jurisdiction rule, if enabled) reached; accepted.
    Committed,
    /// Quorum or dual-jurisdiction rule unreachable; rejected.
    Rejected,
    /// Not yet decided.
    Pending,
}

/// A running tally for a proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTally {
    /// Approving stake.
    pub approve_weight: u64,
    /// Rejecting stake.
    pub reject_weight: u64,
    /// Total stake in the approver set.
    pub total_weight: u64,
    /// Stake threshold required to commit.
    pub quorum_weight: u64,
    /// Whether the source jurisdiction has an approving approver.
    pub source_approved: bool,
    /// Whether the target jurisdiction has an approving approver.
    pub target_approved: bool,
    /// Current decision status.
    pub status: ConsensusStatus,
}

/// The final outcome of consensus on a proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusOutcome {
    /// The proposal that was decided.
    pub proposal_hash: String,
    /// Decision status.
    pub status: ConsensusStatus,
    /// Approving stake at decision time.
    pub approve_weight: u64,
    /// Rejecting stake at decision time.
    pub reject_weight: u64,
    /// Quorum threshold used.
    pub quorum_weight: u64,
    /// Total stake.
    pub total_weight: u64,
    /// Whether both border jurisdictions approved.
    pub dual_jurisdiction_met: bool,
    /// Number of distinct approvers that voted.
    pub voters: usize,
}

/// Coordinates approvers voting on porting proposals.
#[derive(Debug, Clone)]
pub struct ApprovalConsensus {
    approvers: Vec<Approver>,
    quorum_numerator: u64,
    quorum_denominator: u64,
    require_dual_jurisdiction: bool,
    round: u64,
    /// proposal hash -> (approver id -> choice)
    votes: HashMap<String, HashMap<String, VoteChoice>>,
    /// proposal hash -> (source jurisdiction, target jurisdiction)
    corridors: HashMap<String, (String, String)>,
}

impl ApprovalConsensus {
    /// Creates an engine over the given approvers with a default 2/3 quorum and
    /// the dual-jurisdiction (cross-border two-key) rule enabled.
    pub fn new(approvers: Vec<Approver>) -> Self {
        Self {
            approvers,
            quorum_numerator: 2,
            quorum_denominator: 3,
            require_dual_jurisdiction: true,
            round: 0,
            votes: HashMap::new(),
            corridors: HashMap::new(),
        }
    }

    /// Overrides the quorum fraction (numerator/denominator).
    ///
    /// Values are clamped so the numerator never exceeds the denominator and the
    /// denominator is at least one.
    pub fn with_quorum(mut self, numerator: u64, denominator: u64) -> Self {
        let denominator = denominator.max(1);
        self.quorum_numerator = numerator.min(denominator);
        self.quorum_denominator = denominator;
        self
    }

    /// Enables or disables the dual-jurisdiction approval requirement.
    ///
    /// When disabled, a proposal commits on stake quorum alone (useful for
    /// intra-jurisdiction harmonization where there is no border to cross).
    pub fn with_dual_jurisdiction(mut self, required: bool) -> Self {
        self.require_dual_jurisdiction = required;
        self
    }

    /// Number of approvers.
    pub fn approver_count(&self) -> usize {
        self.approvers.len()
    }

    /// Total stake across all approvers.
    pub fn total_stake(&self) -> u64 {
        self.approvers.iter().map(|a| a.stake).sum()
    }

    /// Maximum number of Byzantine approvers tolerated, `f = (n - 1) / 3`.
    pub fn fault_tolerance(&self) -> usize {
        let n = self.approvers.len();
        if n == 0 { 0 } else { (n - 1) / 3 }
    }

    /// Stake threshold required to commit a proposal (ceil of the quorum fraction
    /// of total stake).
    pub fn quorum_weight(&self) -> u64 {
        let total = self.total_stake();
        let num = total.saturating_mul(self.quorum_numerator);
        num.div_ceil(self.quorum_denominator)
    }

    /// Selects the next proof-of-authority proposer by round-robin over the
    /// authority approvers, advancing the internal round counter.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if there are no authority approvers.
    pub fn next_proposer(&mut self) -> ConsensusResult<Approver> {
        let authorities = self.sorted_authorities();
        if authorities.is_empty() {
            return Err(PortingError::InvalidInput(
                "consensus: no authority approvers available".to_string(),
            ));
        }
        let index = (self.round % authorities.len() as u64) as usize;
        self.round = self.round.wrapping_add(1);
        Ok(authorities[index].clone())
    }

    /// Deterministically selects a stake-weighted proposer for `seed`.
    ///
    /// The same seed always yields the same proposer, and the chance of selection
    /// is proportional to stake.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if total stake is zero.
    pub fn weighted_proposer(&self, seed: &str) -> ConsensusResult<Approver> {
        let total = self.total_stake();
        if total == 0 {
            return Err(PortingError::InvalidInput(
                "consensus: cannot select a weighted proposer with zero total stake".to_string(),
            ));
        }
        let digest = sha256_parts(&[seed.as_bytes()]);
        let point = hash_to_u128(&digest) % total as u128;
        let mut sorted = self.approvers.clone();
        sorted.sort_by(|a, b| a.id.cmp(&b.id));
        let mut cumulative: u128 = 0;
        for approver in &sorted {
            cumulative += approver.stake as u128;
            if point < cumulative {
                return Ok(approver.clone());
            }
        }
        sorted.into_iter().next_back().ok_or_else(|| {
            PortingError::InvalidInput("consensus: no approvers to select".to_string())
        })
    }

    /// Selects a proposer using the given [`SelectionMethod`].
    ///
    /// `seed` is consulted only by [`SelectionMethod::WeightedStake`].
    ///
    /// # Errors
    ///
    /// Propagates the errors of [`ApprovalConsensus::next_proposer`] /
    /// [`ApprovalConsensus::weighted_proposer`].
    pub fn select_proposer(
        &mut self,
        method: SelectionMethod,
        seed: &str,
    ) -> ConsensusResult<Approver> {
        match method {
            SelectionMethod::RoundRobin => self.next_proposer(),
            SelectionMethod::WeightedStake => self.weighted_proposer(seed),
        }
    }

    /// Opens a proposal from a recorded port. `proposer` must be a known
    /// approver. The proposal's corridor is registered so later tallies can
    /// enforce the dual-jurisdiction rule.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if `proposer` is not an approver.
    pub fn open_proposal(
        &mut self,
        record: &PortingLedgerRecord,
        proposer: &str,
    ) -> ConsensusResult<PortingProposal> {
        if !self.approvers.iter().any(|a| a.id == proposer) {
            return Err(PortingError::InvalidInput(format!(
                "consensus: '{proposer}' is not an approver and cannot propose"
            )));
        }
        let hash = record.leaf_hash();
        self.corridors.insert(
            hash.clone(),
            (
                record.source_jurisdiction.clone(),
                record.target_jurisdiction.clone(),
            ),
        );
        Ok(PortingProposal {
            hash,
            original_id: record.original_id.clone(),
            source_jurisdiction: record.source_jurisdiction.clone(),
            target_jurisdiction: record.target_jurisdiction.clone(),
            proposer: proposer.to_string(),
        })
    }

    /// Records a vote, detecting equivocation.
    ///
    /// Re-casting the same choice is idempotent; casting a conflicting choice on
    /// the same proposal is an equivocation and is rejected.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if the approver is unknown, has zero
    /// stake, or equivocates.
    pub fn cast_vote(&mut self, vote: ApprovalVote) -> ConsensusResult<()> {
        let approver = self
            .approvers
            .iter()
            .find(|a| a.id == vote.approver_id)
            .ok_or_else(|| {
                PortingError::InvalidInput(format!(
                    "consensus: unknown approver '{}'",
                    vote.approver_id
                ))
            })?;
        if approver.stake == 0 {
            return Err(PortingError::InvalidInput(format!(
                "consensus: approver '{}' has zero stake and cannot vote",
                vote.approver_id
            )));
        }
        let proposal_votes = self.votes.entry(vote.proposal_hash.clone()).or_default();
        if let Some(existing) = proposal_votes.get(&vote.approver_id) {
            if *existing != vote.choice {
                return Err(PortingError::InvalidInput(format!(
                    "consensus: approver '{}' equivocated on proposal {}",
                    vote.approver_id, vote.proposal_hash
                )));
            }
            return Ok(());
        }
        proposal_votes.insert(vote.approver_id, vote.choice);
        Ok(())
    }

    /// Computes the current tally for a proposal.
    pub fn tally(&self, proposal_hash: &str) -> ApprovalTally {
        let total = self.total_stake();
        let quorum = self.quorum_weight();
        let mut approve = 0u64;
        let mut reject = 0u64;
        let mut approving_jurisdictions: HashSet<&str> = HashSet::new();
        if let Some(votes) = self.votes.get(proposal_hash) {
            for (approver_id, choice) in votes {
                if let Some(approver) = self.approvers.iter().find(|a| &a.id == approver_id) {
                    match choice {
                        VoteChoice::Approve => {
                            approve += approver.stake;
                            approving_jurisdictions.insert(approver.jurisdiction.as_str());
                        }
                        VoteChoice::Reject => reject += approver.stake,
                    }
                }
            }
        }

        let (source, target) = self
            .corridors
            .get(proposal_hash)
            .map(|(s, t)| (s.as_str(), t.as_str()))
            .unwrap_or(("", ""));
        let source_approved = approving_jurisdictions.contains(source);
        let target_approved = approving_jurisdictions.contains(target);

        let status = self.decide(
            proposal_hash,
            approve,
            reject,
            total,
            quorum,
            source,
            target,
            source_approved,
            target_approved,
        );
        ApprovalTally {
            approve_weight: approve,
            reject_weight: reject,
            total_weight: total,
            quorum_weight: quorum,
            source_approved,
            target_approved,
            status,
        }
    }

    /// Produces the consensus outcome for a proposal from the current votes.
    pub fn outcome(&self, proposal_hash: &str) -> ConsensusOutcome {
        let tally = self.tally(proposal_hash);
        let voters = self.votes.get(proposal_hash).map(|v| v.len()).unwrap_or(0);
        ConsensusOutcome {
            proposal_hash: proposal_hash.to_string(),
            status: tally.status,
            approve_weight: tally.approve_weight,
            reject_weight: tally.reject_weight,
            quorum_weight: tally.quorum_weight,
            total_weight: tally.total_weight,
            dual_jurisdiction_met: tally.source_approved && tally.target_approved,
            voters,
        }
    }

    /// Decides a proposal's status, combining the stake quorum with the
    /// dual-jurisdiction rule and reachability analysis for rejection.
    #[allow(clippy::too_many_arguments)]
    fn decide(
        &self,
        proposal_hash: &str,
        approve: u64,
        reject: u64,
        total: u64,
        quorum: u64,
        source: &str,
        target: &str,
        source_approved: bool,
        target_approved: bool,
    ) -> ConsensusStatus {
        let dual_required = self.require_dual_jurisdiction;
        let dual_met = !dual_required || (source_approved && target_approved);

        // If the dual-jurisdiction rule can never be satisfied (a needed
        // jurisdiction has rejected with no remaining un-voted approver, or has
        // no approver at all), the proposal is permanently rejected.
        if dual_required && !self.dual_reachable(proposal_hash, source, target) {
            return ConsensusStatus::Rejected;
        }

        if approve >= quorum && dual_met {
            ConsensusStatus::Committed
        } else if total.saturating_sub(reject) < quorum {
            ConsensusStatus::Rejected
        } else {
            ConsensusStatus::Pending
        }
    }

    /// Whether both border jurisdictions can still be brought to approval, i.e.
    /// each has at least one approver that has either approved or not yet voted.
    fn dual_reachable(&self, proposal_hash: &str, source: &str, target: &str) -> bool {
        self.jurisdiction_reachable(proposal_hash, source)
            && self.jurisdiction_reachable(proposal_hash, target)
    }

    fn jurisdiction_reachable(&self, proposal_hash: &str, jurisdiction: &str) -> bool {
        let votes = self.votes.get(proposal_hash);
        self.approvers
            .iter()
            .filter(|a| a.jurisdiction == jurisdiction && a.stake > 0)
            .any(|a| {
                votes
                    .and_then(|v| v.get(&a.id))
                    .map(|choice| *choice == VoteChoice::Approve)
                    .unwrap_or(true) // not yet voted -> still able to approve
            })
    }

    fn sorted_authorities(&self) -> Vec<Approver> {
        let mut authorities: Vec<Approver> = self
            .approvers
            .iter()
            .filter(|a| a.authority)
            .cloned()
            .collect();
        authorities.sort_by(|a, b| a.id.cmp(&b.id));
        authorities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::ledger::PortingLedgerRecord;
    use crate::{ChangeType, PortedStatute, PortingChange};
    use legalis_core::{Effect, EffectType, Statute};
    use legalis_i18n::Locale;

    fn approvers() -> Vec<Approver> {
        vec![
            Approver::authority("jp-min", "JP", 30),
            Approver::authority("us-dept", "US", 30),
            Approver::voter("jp-bar", "JP", 30),
            Approver::voter("us-bar", "US", 10),
        ]
    }

    fn record(source: &str, target: &str) -> PortingLedgerRecord {
        let ported = PortedStatute {
            original_id: "stat-1".to_string(),
            statute: Statute::new("t-1", "T", Effect::new(EffectType::Grant, "B")),
            changes: vec![PortingChange {
                change_type: ChangeType::Translation,
                description: "d".to_string(),
                original: None,
                adapted: None,
                reason: "r".to_string(),
            }],
            locale: Locale::new("en").with_country(target),
            compatibility_score: 0.9,
        };
        PortingLedgerRecord::from_ported(&ported, source, target, "jp-min").expect("record")
    }

    #[test]
    fn test_total_stake_and_quorum() {
        let engine = ApprovalConsensus::new(approvers());
        assert_eq!(engine.total_stake(), 100);
        assert_eq!(engine.quorum_weight(), 67); // ceil(100 * 2 / 3)
        assert_eq!(engine.approver_count(), 4);
    }

    #[test]
    fn test_fault_tolerance() {
        let engine = ApprovalConsensus::new(approvers());
        assert_eq!(engine.fault_tolerance(), 1); // (4 - 1) / 3
        let big = ApprovalConsensus::new(
            (0..10)
                .map(|i| Approver::voter(format!("v{i}"), "JP", 1))
                .collect(),
        );
        assert_eq!(big.fault_tolerance(), 3);
    }

    #[test]
    fn test_round_robin_proposer() {
        let mut engine = ApprovalConsensus::new(approvers());
        // Authorities sorted by id: jp-min, us-dept.
        let p1 = engine.next_proposer().expect("p1");
        let p2 = engine.next_proposer().expect("p2");
        let p3 = engine.next_proposer().expect("p3");
        assert_eq!(p1.id, "jp-min");
        assert_eq!(p2.id, "us-dept");
        assert_eq!(p3.id, "jp-min");
    }

    #[test]
    fn test_round_robin_requires_authorities() {
        let mut engine = ApprovalConsensus::new(vec![Approver::voter("x", "JP", 5)]);
        assert!(engine.next_proposer().is_err());
    }

    #[test]
    fn test_select_proposer_dispatch() {
        let mut engine = ApprovalConsensus::new(approvers());
        let rr = engine
            .select_proposer(SelectionMethod::RoundRobin, "")
            .expect("rr");
        assert_eq!(rr.id, "jp-min");
        let weighted = engine
            .select_proposer(SelectionMethod::WeightedStake, "seed-42")
            .expect("weighted");
        assert_eq!(
            weighted.id,
            engine.weighted_proposer("seed-42").expect("w").id
        );
    }

    #[test]
    fn test_weighted_proposer_deterministic() {
        let engine = ApprovalConsensus::new(approvers());
        let a = engine.weighted_proposer("seed-42").expect("a");
        let b = engine.weighted_proposer("seed-42").expect("b");
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn test_weighted_proposer_distribution() {
        let engine = ApprovalConsensus::new(vec![
            Approver::voter("whale", "JP", 900),
            Approver::voter("minnow", "US", 100),
        ]);
        let mut whale = 0;
        for i in 0..200 {
            let p = engine.weighted_proposer(&format!("seed-{i}")).expect("p");
            if p.id == "whale" {
                whale += 1;
            }
        }
        assert!(whale > 130, "whale selected {whale}/200 times");
    }

    #[test]
    fn test_weighted_proposer_zero_stake() {
        let engine = ApprovalConsensus::new(vec![Approver::voter("x", "JP", 0)]);
        assert!(engine.weighted_proposer("s").is_err());
    }

    #[test]
    fn test_consensus_commit_requires_both_borders() {
        let mut engine = ApprovalConsensus::new(approvers());
        let proposal = engine
            .open_proposal(&record("JP", "US"), "jp-min")
            .expect("open");
        let h = proposal.hash.clone();
        // Two JP approvers approve (60) -> reaches stake but only one border.
        engine
            .cast_vote(ApprovalVote::new("jp-min", &h, VoteChoice::Approve))
            .expect("v1");
        engine
            .cast_vote(ApprovalVote::new("jp-bar", &h, VoteChoice::Approve))
            .expect("v2");
        assert_eq!(engine.tally(&h).status, ConsensusStatus::Pending);
        assert!(engine.tally(&h).source_approved);
        assert!(!engine.tally(&h).target_approved);
        // US approver pushes over quorum AND satisfies the dual-jurisdiction rule.
        engine
            .cast_vote(ApprovalVote::new("us-dept", &h, VoteChoice::Approve))
            .expect("v3");
        let outcome = engine.outcome(&h);
        assert_eq!(outcome.status, ConsensusStatus::Committed);
        assert_eq!(outcome.approve_weight, 90);
        assert!(outcome.dual_jurisdiction_met);
        assert_eq!(outcome.voters, 3);
    }

    #[test]
    fn test_dual_jurisdiction_can_be_disabled() {
        let mut engine = ApprovalConsensus::new(approvers()).with_dual_jurisdiction(false);
        let proposal = engine
            .open_proposal(&record("JP", "US"), "jp-min")
            .expect("open");
        let h = proposal.hash;
        // Same-border-only approvals now suffice once stake quorum is reached.
        engine
            .cast_vote(ApprovalVote::new("jp-min", &h, VoteChoice::Approve))
            .expect("v1");
        engine
            .cast_vote(ApprovalVote::new("jp-bar", &h, VoteChoice::Approve))
            .expect("v2");
        engine
            .cast_vote(ApprovalVote::new("us-dept", &h, VoteChoice::Approve))
            .expect("v3");
        assert_eq!(engine.tally(&h).status, ConsensusStatus::Committed);
    }

    #[test]
    fn test_consensus_reject_on_stake() {
        let mut engine = ApprovalConsensus::new(approvers());
        let proposal = engine
            .open_proposal(&record("JP", "US"), "us-dept")
            .expect("open");
        let h = proposal.hash;
        // reject 30 + 30 = 60; max approve = 100 - 60 = 40 < 67 -> Rejected.
        engine
            .cast_vote(ApprovalVote::new("jp-min", &h, VoteChoice::Reject))
            .expect("v1");
        engine
            .cast_vote(ApprovalVote::new("us-dept", &h, VoteChoice::Reject))
            .expect("v2");
        assert_eq!(engine.tally(&h).status, ConsensusStatus::Rejected);
    }

    #[test]
    fn test_consensus_reject_when_border_unreachable() {
        // US side has a single approver; if it rejects, the dual rule is
        // unreachable and the proposal is rejected even though JP could meet
        // the stake quorum.
        let engine_approvers = vec![
            Approver::voter("jp-a", "JP", 40),
            Approver::voter("jp-b", "JP", 40),
            Approver::voter("us-only", "US", 5),
        ];
        let mut engine = ApprovalConsensus::new(engine_approvers).with_quorum(1, 2);
        let proposal = engine
            .open_proposal(&record("JP", "US"), "jp-a")
            .expect("open");
        let h = proposal.hash;
        engine
            .cast_vote(ApprovalVote::new("jp-a", &h, VoteChoice::Approve))
            .expect("v1");
        engine
            .cast_vote(ApprovalVote::new("us-only", &h, VoteChoice::Reject))
            .expect("v2");
        assert_eq!(engine.tally(&h).status, ConsensusStatus::Rejected);
    }

    #[test]
    fn test_missing_target_jurisdiction_rejected() {
        // No approver represents the target "DE" -> dual rule never reachable.
        let mut engine = ApprovalConsensus::new(approvers());
        let proposal = engine
            .open_proposal(&record("JP", "DE"), "jp-min")
            .expect("open");
        let h = proposal.hash;
        engine
            .cast_vote(ApprovalVote::new("jp-min", &h, VoteChoice::Approve))
            .expect("v1");
        engine
            .cast_vote(ApprovalVote::new("us-dept", &h, VoteChoice::Approve))
            .expect("v2");
        engine
            .cast_vote(ApprovalVote::new("jp-bar", &h, VoteChoice::Approve))
            .expect("v3");
        assert_eq!(engine.tally(&h).status, ConsensusStatus::Rejected);
    }

    #[test]
    fn test_equivocation_detected() {
        let mut engine = ApprovalConsensus::new(approvers());
        let h = "proposal-1";
        engine
            .cast_vote(ApprovalVote::new("jp-min", h, VoteChoice::Approve))
            .expect("v1");
        engine
            .cast_vote(ApprovalVote::new("jp-min", h, VoteChoice::Approve))
            .expect("idem");
        assert!(
            engine
                .cast_vote(ApprovalVote::new("jp-min", h, VoteChoice::Reject))
                .is_err()
        );
    }

    #[test]
    fn test_unknown_approver_cannot_vote() {
        let mut engine = ApprovalConsensus::new(approvers());
        assert!(
            engine
                .cast_vote(ApprovalVote::new("mallory", "p", VoteChoice::Approve))
                .is_err()
        );
    }

    #[test]
    fn test_non_approver_cannot_propose() {
        let mut engine = ApprovalConsensus::new(approvers());
        assert!(
            engine
                .open_proposal(&record("JP", "US"), "mallory")
                .is_err()
        );
    }

    #[test]
    fn test_custom_quorum() {
        let engine = ApprovalConsensus::new(approvers()).with_quorum(1, 2);
        assert_eq!(engine.quorum_weight(), 50); // ceil(100 / 2)
    }

    #[test]
    fn test_zero_stake_approver_rejected() {
        let mut engine = ApprovalConsensus::new(vec![
            Approver::voter("z", "JP", 0),
            Approver::voter("a", "US", 5),
        ]);
        assert!(
            engine
                .cast_vote(ApprovalVote::new("z", "p", VoteChoice::Approve))
                .is_err()
        );
        assert!(
            engine
                .cast_vote(ApprovalVote::new("a", "p", VoteChoice::Approve))
                .is_ok()
        );
    }

    #[test]
    fn test_outcome_serde_roundtrip() {
        let mut engine = ApprovalConsensus::new(approvers());
        let proposal = engine
            .open_proposal(&record("JP", "US"), "jp-min")
            .expect("open");
        let h = proposal.hash;
        engine
            .cast_vote(ApprovalVote::new("jp-min", &h, VoteChoice::Approve))
            .expect("v");
        let outcome = engine.outcome(&h);
        let json = serde_json::to_string(&outcome).expect("ser");
        let back: ConsensusOutcome = serde_json::from_str(&json).expect("de");
        assert_eq!(outcome, back);
    }
}
