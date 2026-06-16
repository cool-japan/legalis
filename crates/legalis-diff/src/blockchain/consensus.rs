//! Distributed consensus for diff verification.
//!
//! A [`ConsensusEngine`] coordinates a set of weighted [`Validator`]s that vote
//! on whether a proposed diff (a [`Proposal`]) should be committed to the
//! ledger. Three complementary mechanisms are provided:
//!
//! - **Proof-of-authority** round-robin leader election among authority
//!   validators ([`ConsensusEngine::next_proposer`]).
//! - **Weighted proof-of-stake** deterministic leader election, where the
//!   probability of selection is proportional to stake and the outcome is
//!   reproducible from a seed ([`ConsensusEngine::weighted_proposer`]).
//! - A **Byzantine-fault-tolerant voting tally** that commits a proposal once
//!   approving stake reaches a configurable quorum (2/3 by default), rejects it
//!   once a quorum becomes unreachable, and detects equivocation (a validator
//!   casting conflicting votes on the same proposal).

use super::ledger::DiffRecord;
use super::sha256_parts;
use crate::{DiffError, DiffResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A consensus participant with voting weight.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Validator {
    /// Unique validator identifier.
    pub id: String,
    /// Voting weight / stake (must be non-zero to count).
    pub stake: u64,
    /// Whether this validator may propose blocks under proof-of-authority.
    pub authority: bool,
}

impl Validator {
    /// Creates an authority validator with the given stake.
    pub fn authority(id: impl Into<String>, stake: u64) -> Self {
        Self {
            id: id.into(),
            stake,
            authority: true,
        }
    }

    /// Creates a non-authority (voting-only) validator with the given stake.
    pub fn voter(id: impl Into<String>, stake: u64) -> Self {
        Self {
            id: id.into(),
            stake,
            authority: false,
        }
    }
}

/// How a proposer/leader is selected for a round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMethod {
    /// Round-robin over authority validators.
    RoundRobin,
    /// Stake-weighted, deterministic from a seed.
    WeightedStake,
}

/// A validator's vote on a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteChoice {
    /// The validator accepts the proposal.
    Approve,
    /// The validator rejects the proposal.
    Reject,
}

/// A single cast vote.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vote {
    /// The voting validator's id.
    pub validator_id: String,
    /// The proposal being voted on.
    pub proposal_hash: String,
    /// The validator's choice.
    pub choice: VoteChoice,
}

impl Vote {
    /// Creates a vote.
    pub fn new(
        validator_id: impl Into<String>,
        proposal_hash: impl Into<String>,
        choice: VoteChoice,
    ) -> Self {
        Self {
            validator_id: validator_id.into(),
            proposal_hash: proposal_hash.into(),
            choice,
        }
    }
}

/// A diff put forward for consensus verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposal {
    /// Content hash uniquely identifying the proposal.
    pub hash: String,
    /// Statute the proposed diff applies to.
    pub statute_id: String,
    /// The validator that proposed it.
    pub proposer: String,
}

/// Whether a proposal has reached a terminal decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusStatus {
    /// Quorum reached; the proposal is accepted.
    Committed,
    /// Quorum unreachable; the proposal is rejected.
    Rejected,
    /// Not yet decided.
    Pending,
}

/// A running tally for a proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposalTally {
    /// Approving stake.
    pub approve_weight: u64,
    /// Rejecting stake.
    pub reject_weight: u64,
    /// Total stake in the validator set.
    pub total_weight: u64,
    /// Stake threshold required to commit.
    pub quorum_weight: u64,
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
    /// Number of distinct validators that voted.
    pub voters: usize,
}

/// Coordinates validators voting on diff proposals.
#[derive(Debug, Clone)]
pub struct ConsensusEngine {
    validators: Vec<Validator>,
    quorum_numerator: u64,
    quorum_denominator: u64,
    round: u64,
    /// proposal hash -> (validator id -> choice)
    votes: HashMap<String, HashMap<String, VoteChoice>>,
}

impl ConsensusEngine {
    /// Creates an engine over the given validators with a default 2/3 quorum.
    pub fn new(validators: Vec<Validator>) -> Self {
        Self {
            validators,
            quorum_numerator: 2,
            quorum_denominator: 3,
            round: 0,
            votes: HashMap::new(),
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

    /// Number of validators.
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }

    /// Total stake across all validators.
    pub fn total_stake(&self) -> u64 {
        self.validators.iter().map(|v| v.stake).sum()
    }

    /// Maximum number of Byzantine validators tolerated, `f = (n - 1) / 3`.
    pub fn fault_tolerance(&self) -> usize {
        let n = self.validators.len();
        if n == 0 { 0 } else { (n - 1) / 3 }
    }

    /// Stake threshold required to commit a proposal (ceil of the quorum
    /// fraction of total stake).
    pub fn quorum_weight(&self) -> u64 {
        let total = self.total_stake();
        let num = total.saturating_mul(self.quorum_numerator);
        num.div_ceil(self.quorum_denominator)
    }

    /// Selects the next proof-of-authority proposer by round-robin over the
    /// authority validators, advancing the internal round counter.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ConsensusFailure`] if there are no authority
    /// validators.
    pub fn next_proposer(&mut self) -> DiffResult<Validator> {
        let authorities = self.sorted_authorities();
        if authorities.is_empty() {
            return Err(DiffError::ConsensusFailure(
                "no authority validators available".to_string(),
            ));
        }
        let index = (self.round % authorities.len() as u64) as usize;
        self.round = self.round.wrapping_add(1);
        Ok(authorities[index].clone())
    }

    /// Deterministically selects a stake-weighted proposer for `seed`.
    ///
    /// The same seed always yields the same proposer, and the chance of
    /// selection is proportional to stake.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ConsensusFailure`] if total stake is zero.
    pub fn weighted_proposer(&self, seed: &str) -> DiffResult<Validator> {
        let total = self.total_stake();
        if total == 0 {
            return Err(DiffError::ConsensusFailure(
                "cannot select a weighted proposer with zero total stake".to_string(),
            ));
        }
        let digest = sha256_parts(&[seed.as_bytes()]);
        let point = hash_to_u128(&digest) % total as u128;
        // Walk validators (sorted for determinism) accumulating stake.
        let mut sorted = self.validators.clone();
        sorted.sort_by(|a, b| a.id.cmp(&b.id));
        let mut cumulative: u128 = 0;
        for validator in &sorted {
            cumulative += validator.stake as u128;
            if point < cumulative {
                return Ok(validator.clone());
            }
        }
        // Unreachable given point < total, but return the last validator rather
        // than panic to remain total.
        sorted
            .into_iter()
            .next_back()
            .ok_or_else(|| DiffError::ConsensusFailure("no validators".to_string()))
    }

    /// Selects a proposer using the given [`SelectionMethod`].
    ///
    /// `seed` is consulted only by [`SelectionMethod::WeightedStake`].
    ///
    /// # Errors
    ///
    /// Propagates the errors of [`ConsensusEngine::next_proposer`] /
    /// [`ConsensusEngine::weighted_proposer`].
    pub fn select_proposer(
        &mut self,
        method: SelectionMethod,
        seed: &str,
    ) -> DiffResult<Validator> {
        match method {
            SelectionMethod::RoundRobin => self.next_proposer(),
            SelectionMethod::WeightedStake => self.weighted_proposer(seed),
        }
    }

    /// Opens a proposal from a recorded diff. `proposer` must be a known
    /// validator.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ConsensusFailure`] if `proposer` is not a validator.
    pub fn open_proposal(&self, record: &DiffRecord, proposer: &str) -> DiffResult<Proposal> {
        if !self.validators.iter().any(|v| v.id == proposer) {
            return Err(DiffError::ConsensusFailure(format!(
                "'{}' is not a validator and cannot propose",
                proposer
            )));
        }
        Ok(Proposal {
            hash: record.leaf_hash(),
            statute_id: record.statute_id.clone(),
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
    /// Returns [`DiffError::ConsensusFailure`] if the validator is unknown, has
    /// zero stake, or equivocates.
    pub fn cast_vote(&mut self, vote: Vote) -> DiffResult<()> {
        let validator = self
            .validators
            .iter()
            .find(|v| v.id == vote.validator_id)
            .ok_or_else(|| {
                DiffError::ConsensusFailure(format!("unknown validator '{}'", vote.validator_id))
            })?;
        if validator.stake == 0 {
            return Err(DiffError::ConsensusFailure(format!(
                "validator '{}' has zero stake and cannot vote",
                vote.validator_id
            )));
        }
        let proposal_votes = self.votes.entry(vote.proposal_hash.clone()).or_default();
        if let Some(existing) = proposal_votes.get(&vote.validator_id) {
            if *existing != vote.choice {
                return Err(DiffError::ConsensusFailure(format!(
                    "validator '{}' equivocated on proposal {}",
                    vote.validator_id, vote.proposal_hash
                )));
            }
            return Ok(());
        }
        proposal_votes.insert(vote.validator_id, vote.choice);
        Ok(())
    }

    /// Computes the current tally for a proposal.
    pub fn tally(&self, proposal_hash: &str) -> ProposalTally {
        let total = self.total_stake();
        let quorum = self.quorum_weight();
        let mut approve = 0u64;
        let mut reject = 0u64;
        if let Some(votes) = self.votes.get(proposal_hash) {
            for (validator_id, choice) in votes {
                let stake = self
                    .validators
                    .iter()
                    .find(|v| &v.id == validator_id)
                    .map(|v| v.stake)
                    .unwrap_or(0);
                match choice {
                    VoteChoice::Approve => approve += stake,
                    VoteChoice::Reject => reject += stake,
                }
            }
        }
        let status = decide(approve, reject, total, quorum);
        ProposalTally {
            approve_weight: approve,
            reject_weight: reject,
            total_weight: total,
            quorum_weight: quorum,
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
            voters,
        }
    }

    fn sorted_authorities(&self) -> Vec<Validator> {
        let mut authorities: Vec<Validator> = self
            .validators
            .iter()
            .filter(|v| v.authority)
            .cloned()
            .collect();
        authorities.sort_by(|a, b| a.id.cmp(&b.id));
        authorities
    }
}

/// Decides a proposal's status from current tallies.
///
/// Committed once approving stake meets the quorum; rejected once the maximum
/// achievable approving stake (total minus already-rejecting stake) can no
/// longer reach the quorum; otherwise pending.
fn decide(approve: u64, reject: u64, total: u64, quorum: u64) -> ConsensusStatus {
    if approve >= quorum {
        ConsensusStatus::Committed
    } else if total.saturating_sub(reject) < quorum {
        ConsensusStatus::Rejected
    } else {
        ConsensusStatus::Pending
    }
}

/// Interprets the first 16 bytes of a hex digest as a big-endian `u128`.
fn hash_to_u128(hex_digest: &str) -> u128 {
    let bytes = hex::decode(hex_digest).unwrap_or_default();
    let mut value: u128 = 0;
    for byte in bytes.into_iter().take(16) {
        value = (value << 8) | byte as u128;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::ledger::DiffRecord;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn validators() -> Vec<Validator> {
        vec![
            Validator::authority("alice", 30),
            Validator::authority("bob", 30),
            Validator::voter("carol", 30),
            Validator::voter("dave", 10),
        ]
    }

    fn record() -> DiffRecord {
        let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        let d = diff(&old, &new).expect("diff");
        DiffRecord::from_diff(&d, "alice").expect("record")
    }

    #[test]
    fn test_total_stake_and_quorum() {
        let engine = ConsensusEngine::new(validators());
        assert_eq!(engine.total_stake(), 100);
        // ceil(100 * 2 / 3) = 67
        assert_eq!(engine.quorum_weight(), 67);
        assert_eq!(engine.validator_count(), 4);
    }

    #[test]
    fn test_fault_tolerance() {
        let engine = ConsensusEngine::new(validators());
        // (4 - 1) / 3 = 1
        assert_eq!(engine.fault_tolerance(), 1);
        let big = ConsensusEngine::new(
            (0..10)
                .map(|i| Validator::voter(format!("v{}", i), 1))
                .collect(),
        );
        assert_eq!(big.fault_tolerance(), 3);
    }

    #[test]
    fn test_round_robin_proposer() {
        let mut engine = ConsensusEngine::new(validators());
        // Authorities sorted: alice, bob.
        let p1 = engine.next_proposer().expect("p1");
        let p2 = engine.next_proposer().expect("p2");
        let p3 = engine.next_proposer().expect("p3");
        assert_eq!(p1.id, "alice");
        assert_eq!(p2.id, "bob");
        assert_eq!(p3.id, "alice");
    }

    #[test]
    fn test_round_robin_requires_authorities() {
        let mut engine = ConsensusEngine::new(vec![Validator::voter("x", 5)]);
        assert!(engine.next_proposer().is_err());
    }

    #[test]
    fn test_select_proposer_dispatch() {
        let mut engine = ConsensusEngine::new(validators());
        let rr = engine
            .select_proposer(SelectionMethod::RoundRobin, "")
            .expect("rr");
        assert_eq!(rr.id, "alice");
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
        let engine = ConsensusEngine::new(validators());
        let a = engine.weighted_proposer("seed-42").expect("a");
        let b = engine.weighted_proposer("seed-42").expect("b");
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn test_weighted_proposer_distribution() {
        // A heavily-staked validator should be selected far more often.
        let engine = ConsensusEngine::new(vec![
            Validator::voter("whale", 900),
            Validator::voter("minnow", 100),
        ]);
        let mut whale = 0;
        for i in 0..200 {
            let p = engine.weighted_proposer(&format!("seed-{}", i)).expect("p");
            if p.id == "whale" {
                whale += 1;
            }
        }
        assert!(whale > 130, "whale selected {}/200 times", whale);
    }

    #[test]
    fn test_weighted_proposer_zero_stake() {
        let engine = ConsensusEngine::new(vec![Validator::voter("x", 0)]);
        assert!(engine.weighted_proposer("s").is_err());
    }

    #[test]
    fn test_consensus_commit() {
        let mut engine = ConsensusEngine::new(validators());
        let proposal = engine.open_proposal(&record(), "alice").expect("open");
        let h = proposal.hash.clone();
        // alice(30) + bob(30) + carol(30) = 90 >= 67 quorum
        engine
            .cast_vote(Vote::new("alice", &h, VoteChoice::Approve))
            .expect("v1");
        engine
            .cast_vote(Vote::new("bob", &h, VoteChoice::Approve))
            .expect("v2");
        assert_eq!(engine.tally(&h).status, ConsensusStatus::Pending);
        engine
            .cast_vote(Vote::new("carol", &h, VoteChoice::Approve))
            .expect("v3");
        let outcome = engine.outcome(&h);
        assert_eq!(outcome.status, ConsensusStatus::Committed);
        assert_eq!(outcome.approve_weight, 90);
        assert_eq!(outcome.voters, 3);
    }

    #[test]
    fn test_consensus_reject() {
        let mut engine = ConsensusEngine::new(validators());
        let proposal = engine.open_proposal(&record(), "bob").expect("open");
        let h = proposal.hash;
        // reject 30 + 30 = 60; max approve = 100 - 60 = 40 < 67 -> Rejected
        engine
            .cast_vote(Vote::new("alice", &h, VoteChoice::Reject))
            .expect("v1");
        engine
            .cast_vote(Vote::new("bob", &h, VoteChoice::Reject))
            .expect("v2");
        assert_eq!(engine.tally(&h).status, ConsensusStatus::Rejected);
    }

    #[test]
    fn test_equivocation_detected() {
        let mut engine = ConsensusEngine::new(validators());
        let h = "proposal-1";
        engine
            .cast_vote(Vote::new("alice", h, VoteChoice::Approve))
            .expect("v1");
        // Same choice again is idempotent.
        engine
            .cast_vote(Vote::new("alice", h, VoteChoice::Approve))
            .expect("idempotent");
        // Conflicting choice equivocates.
        assert!(
            engine
                .cast_vote(Vote::new("alice", h, VoteChoice::Reject))
                .is_err()
        );
    }

    #[test]
    fn test_unknown_validator_cannot_vote() {
        let mut engine = ConsensusEngine::new(validators());
        assert!(
            engine
                .cast_vote(Vote::new("mallory", "p", VoteChoice::Approve))
                .is_err()
        );
    }

    #[test]
    fn test_non_validator_cannot_propose() {
        let engine = ConsensusEngine::new(validators());
        assert!(engine.open_proposal(&record(), "mallory").is_err());
    }

    #[test]
    fn test_custom_quorum() {
        let engine = ConsensusEngine::new(validators()).with_quorum(1, 2);
        // ceil(100 / 2) = 50
        assert_eq!(engine.quorum_weight(), 50);
    }

    #[test]
    fn test_zero_stake_validator_rejected() {
        let mut engine =
            ConsensusEngine::new(vec![Validator::voter("z", 0), Validator::voter("a", 5)]);
        assert!(
            engine
                .cast_vote(Vote::new("z", "p", VoteChoice::Approve))
                .is_err()
        );
        assert!(
            engine
                .cast_vote(Vote::new("a", "p", VoteChoice::Approve))
                .is_ok()
        );
    }

    #[test]
    fn test_outcome_serde_roundtrip() {
        let mut engine = ConsensusEngine::new(validators());
        let h = "p";
        engine
            .cast_vote(Vote::new("alice", h, VoteChoice::Approve))
            .expect("v");
        let outcome = engine.outcome(h);
        let json = serde_json::to_string(&outcome).expect("ser");
        let back: ConsensusOutcome = serde_json::from_str(&json).expect("de");
        assert_eq!(outcome, back);
    }
}
