//! Distributed consensus for audit record agreement
//!
//! This module implements consensus mechanisms to ensure all nodes agree on
//! the audit trail state, even in the presence of failures or network partitions.

use super::{DistributedError, NodeId};
use crate::AuditRecord;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Consensus algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusAlgorithm {
    /// Simple majority voting
    Majority,
    /// Raft consensus algorithm
    Raft,
    /// Paxos consensus algorithm
    Paxos,
    /// Practical Byzantine Fault Tolerance
    Pbft,
}

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    pub algorithm: ConsensusAlgorithm,
    pub quorum_size: usize,
    pub timeout_ms: u64,
    pub max_retries: usize,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            algorithm: ConsensusAlgorithm::Majority,
            quorum_size: 3,
            timeout_ms: 5000,
            max_retries: 3,
        }
    }
}

/// Proposal for a new audit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordProposal {
    pub proposal_id: uuid::Uuid,
    pub record: AuditRecord,
    pub proposer: NodeId,
    pub timestamp: DateTime<Utc>,
}

impl RecordProposal {
    pub fn new(record: AuditRecord, proposer: NodeId) -> Self {
        Self {
            proposal_id: uuid::Uuid::new_v4(),
            record,
            proposer,
            timestamp: Utc::now(),
        }
    }
}

/// Vote on a proposal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vote {
    Accept,
    Reject,
    Abstain,
}

/// Vote cast by a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteCast {
    pub voter: NodeId,
    pub proposal_id: uuid::Uuid,
    pub vote: Vote,
    pub timestamp: DateTime<Utc>,
    pub reason: Option<String>,
}

impl VoteCast {
    pub fn new(voter: NodeId, proposal_id: uuid::Uuid, vote: Vote) -> Self {
        Self {
            voter,
            proposal_id,
            vote,
            timestamp: Utc::now(),
            reason: None,
        }
    }

    pub fn with_reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }
}

/// Status of a consensus round
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusStatus {
    /// Proposal is pending votes
    Pending,
    /// Consensus reached - proposal accepted
    Accepted,
    /// Consensus reached - proposal rejected
    Rejected,
    /// Timeout occurred before consensus
    Timeout,
    /// Not enough nodes available for consensus
    InsufficientNodes,
}

/// Result of a consensus round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub proposal_id: uuid::Uuid,
    pub status: ConsensusStatus,
    pub accept_votes: usize,
    pub reject_votes: usize,
    pub abstain_votes: usize,
    pub votes: Vec<VoteCast>,
    pub decided_at: DateTime<Utc>,
}

impl ConsensusResult {
    pub fn new(proposal_id: uuid::Uuid, status: ConsensusStatus) -> Self {
        Self {
            proposal_id,
            status,
            accept_votes: 0,
            reject_votes: 0,
            abstain_votes: 0,
            votes: Vec::new(),
            decided_at: Utc::now(),
        }
    }

    pub fn add_vote(&mut self, vote: VoteCast) {
        match vote.vote {
            Vote::Accept => self.accept_votes += 1,
            Vote::Reject => self.reject_votes += 1,
            Vote::Abstain => self.abstain_votes += 1,
        }
        self.votes.push(vote);
    }

    pub fn total_votes(&self) -> usize {
        self.accept_votes + self.reject_votes + self.abstain_votes
    }

    pub fn has_quorum(&self, quorum_size: usize) -> bool {
        self.total_votes() >= quorum_size
    }

    pub fn is_accepted(&self, quorum_size: usize) -> bool {
        self.accept_votes >= quorum_size
    }

    pub fn is_rejected(&self, quorum_size: usize) -> bool {
        self.reject_votes > (self.total_votes() - quorum_size)
    }
}

/// Consensus manager for coordinating agreement on records
pub struct ConsensusManager {
    node_id: NodeId,
    config: ConsensusConfig,
    active_proposals: HashMap<uuid::Uuid, RecordProposal>,
    consensus_results: HashMap<uuid::Uuid, ConsensusResult>,
}

impl ConsensusManager {
    pub fn new(node_id: NodeId, config: ConsensusConfig) -> Self {
        Self {
            node_id,
            config,
            active_proposals: HashMap::new(),
            consensus_results: HashMap::new(),
        }
    }

    /// Propose a new audit record for consensus
    pub fn propose_record(&mut self, record: AuditRecord) -> RecordProposal {
        let proposal = RecordProposal::new(record, self.node_id.clone());
        self.active_proposals
            .insert(proposal.proposal_id, proposal.clone());
        proposal
    }

    /// Cast a vote on a proposal
    pub fn cast_vote(
        &mut self,
        proposal_id: uuid::Uuid,
        vote: Vote,
    ) -> Result<VoteCast, DistributedError> {
        if !self.active_proposals.contains_key(&proposal_id) {
            return Err(DistributedError::ConsensusError(
                "Proposal not found".to_string(),
            ));
        }

        let vote_cast = VoteCast::new(self.node_id.clone(), proposal_id, vote);
        self.record_vote(vote_cast.clone())?;
        Ok(vote_cast)
    }

    /// Record a vote from any node
    pub fn record_vote(&mut self, vote: VoteCast) -> Result<(), DistributedError> {
        let result = self
            .consensus_results
            .entry(vote.proposal_id)
            .or_insert_with(|| ConsensusResult::new(vote.proposal_id, ConsensusStatus::Pending));

        result.add_vote(vote);

        // Check if consensus is reached
        if result.has_quorum(self.config.quorum_size) {
            if result.is_accepted(self.config.quorum_size) {
                result.status = ConsensusStatus::Accepted;
            } else if result.is_rejected(self.config.quorum_size) {
                result.status = ConsensusStatus::Rejected;
            }
            result.decided_at = Utc::now();
        }

        Ok(())
    }

    /// Get the consensus result for a proposal
    pub fn get_consensus_result(&self, proposal_id: &uuid::Uuid) -> Option<&ConsensusResult> {
        self.consensus_results.get(proposal_id)
    }

    /// Check if consensus has been reached for a proposal
    pub fn has_consensus(&self, proposal_id: &uuid::Uuid) -> bool {
        self.consensus_results
            .get(proposal_id)
            .map(|r| {
                matches!(
                    r.status,
                    ConsensusStatus::Accepted | ConsensusStatus::Rejected
                )
            })
            .unwrap_or(false)
    }

    /// Get all active proposals
    pub fn get_active_proposals(&self) -> Vec<&RecordProposal> {
        self.active_proposals.values().collect()
    }

    /// Get a specific proposal
    pub fn get_proposal(&self, proposal_id: &uuid::Uuid) -> Option<&RecordProposal> {
        self.active_proposals.get(proposal_id)
    }

    /// Remove a proposal after consensus is reached
    pub fn complete_proposal(&mut self, proposal_id: &uuid::Uuid) -> Option<RecordProposal> {
        self.active_proposals.remove(proposal_id)
    }

    /// Mark proposals as timed out
    pub fn check_timeouts(&mut self) -> Vec<uuid::Uuid> {
        let now = Utc::now();
        let timeout_duration = chrono::Duration::milliseconds(self.config.timeout_ms as i64);
        let mut timed_out = Vec::new();

        for (proposal_id, proposal) in &self.active_proposals {
            if now.signed_duration_since(proposal.timestamp) > timeout_duration {
                if let Some(result) = self.consensus_results.get_mut(proposal_id) {
                    if result.status == ConsensusStatus::Pending {
                        result.status = ConsensusStatus::Timeout;
                        result.decided_at = now;
                        timed_out.push(*proposal_id);
                    }
                }
            }
        }

        timed_out
    }

    /// Calculate the minimum quorum size based on node count
    pub fn calculate_quorum(total_nodes: usize) -> usize {
        (total_nodes / 2) + 1
    }

    /// Check if we have enough nodes for consensus
    pub fn has_sufficient_nodes(&self, available_nodes: usize) -> bool {
        available_nodes >= self.config.quorum_size
    }

    /// Get consensus statistics
    pub fn get_stats(&self) -> ConsensusStats {
        let total_proposals = self.active_proposals.len() + self.consensus_results.len();
        let accepted = self
            .consensus_results
            .values()
            .filter(|r| r.status == ConsensusStatus::Accepted)
            .count();
        let rejected = self
            .consensus_results
            .values()
            .filter(|r| r.status == ConsensusStatus::Rejected)
            .count();
        let pending = self
            .consensus_results
            .values()
            .filter(|r| r.status == ConsensusStatus::Pending)
            .count();
        let timed_out = self
            .consensus_results
            .values()
            .filter(|r| r.status == ConsensusStatus::Timeout)
            .count();

        ConsensusStats {
            total_proposals,
            accepted,
            rejected,
            pending,
            timed_out,
        }
    }
}

/// Statistics about consensus operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStats {
    pub total_proposals: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub pending: usize,
    pub timed_out: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, EventType};

    fn create_test_record() -> AuditRecord {
        AuditRecord {
            id: uuid::Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::AutomaticDecision,
            statute_id: "TEST-1".to_string(),
            subject_id: uuid::Uuid::new_v4(),
            actor: Actor::System {
                component: "test".to_string(),
            },
            context: crate::DecisionContext::default(),
            result: crate::DecisionResult::Deterministic {
                effect_applied: "allowed".to_string(),
                parameters: std::collections::HashMap::new(),
            },
            previous_hash: None,
            record_hash: "hash123".to_string(),
        }
    }

    #[test]
    fn test_consensus_config_default() {
        let config = ConsensusConfig::default();
        assert_eq!(config.algorithm, ConsensusAlgorithm::Majority);
        assert_eq!(config.quorum_size, 3);
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_record_proposal() {
        let record = create_test_record();
        let proposer = NodeId::new("node-1");
        let proposal = RecordProposal::new(record.clone(), proposer.clone());

        assert_eq!(proposal.record.id, record.id);
        assert_eq!(proposal.proposer, proposer);
    }

    #[test]
    fn test_vote_cast() {
        let voter = NodeId::new("node-1");
        let proposal_id = uuid::Uuid::new_v4();
        let vote = VoteCast::new(voter.clone(), proposal_id, Vote::Accept);

        assert_eq!(vote.voter, voter);
        assert_eq!(vote.proposal_id, proposal_id);
        assert_eq!(vote.vote, Vote::Accept);
        assert_eq!(vote.reason, None);
    }

    #[test]
    fn test_vote_with_reason() {
        let voter = NodeId::new("node-1");
        let proposal_id = uuid::Uuid::new_v4();
        let vote = VoteCast::new(voter.clone(), proposal_id, Vote::Reject)
            .with_reason("Invalid record".to_string());

        assert_eq!(vote.reason, Some("Invalid record".to_string()));
    }

    #[test]
    fn test_consensus_result() {
        let proposal_id = uuid::Uuid::new_v4();
        let mut result = ConsensusResult::new(proposal_id, ConsensusStatus::Pending);

        assert_eq!(result.accept_votes, 0);
        assert_eq!(result.reject_votes, 0);
        assert_eq!(result.total_votes(), 0);

        let vote1 = VoteCast::new(NodeId::new("node-1"), proposal_id, Vote::Accept);
        let vote2 = VoteCast::new(NodeId::new("node-2"), proposal_id, Vote::Accept);
        let vote3 = VoteCast::new(NodeId::new("node-3"), proposal_id, Vote::Reject);

        result.add_vote(vote1);
        result.add_vote(vote2);
        result.add_vote(vote3);

        assert_eq!(result.accept_votes, 2);
        assert_eq!(result.reject_votes, 1);
        assert_eq!(result.total_votes(), 3);
    }

    #[test]
    fn test_consensus_quorum() {
        let proposal_id = uuid::Uuid::new_v4();
        let mut result = ConsensusResult::new(proposal_id, ConsensusStatus::Pending);

        assert!(!result.has_quorum(3));

        let vote1 = VoteCast::new(NodeId::new("node-1"), proposal_id, Vote::Accept);
        let vote2 = VoteCast::new(NodeId::new("node-2"), proposal_id, Vote::Accept);
        let vote3 = VoteCast::new(NodeId::new("node-3"), proposal_id, Vote::Accept);

        result.add_vote(vote1);
        result.add_vote(vote2);
        result.add_vote(vote3);

        assert!(result.has_quorum(3));
        assert!(result.is_accepted(3));
    }

    #[test]
    fn test_consensus_manager() {
        let node_id = NodeId::new("node-1");
        let config = ConsensusConfig::default();
        let mut manager = ConsensusManager::new(node_id, config);

        let record = create_test_record();
        let proposal = manager.propose_record(record);

        assert_eq!(manager.get_active_proposals().len(), 1);
        assert!(manager.get_proposal(&proposal.proposal_id).is_some());
    }

    #[test]
    fn test_consensus_voting() {
        let node_id = NodeId::new("node-1");
        let config = ConsensusConfig {
            quorum_size: 2,
            ..Default::default()
        };
        let mut manager = ConsensusManager::new(node_id, config);

        let record = create_test_record();
        let proposal = manager.propose_record(record);

        // Cast votes
        manager
            .cast_vote(proposal.proposal_id, Vote::Accept)
            .unwrap();

        let vote2 = VoteCast::new(NodeId::new("node-2"), proposal.proposal_id, Vote::Accept);
        manager.record_vote(vote2).unwrap();

        // Check consensus
        assert!(manager.has_consensus(&proposal.proposal_id));

        let result = manager.get_consensus_result(&proposal.proposal_id).unwrap();
        assert_eq!(result.status, ConsensusStatus::Accepted);
        assert_eq!(result.accept_votes, 2);
    }

    #[test]
    fn test_consensus_rejection() {
        let node_id = NodeId::new("node-1");
        let config = ConsensusConfig {
            quorum_size: 2,
            ..Default::default()
        };
        let mut manager = ConsensusManager::new(node_id, config);

        let record = create_test_record();
        let proposal = manager.propose_record(record);

        // Cast reject votes
        manager
            .cast_vote(proposal.proposal_id, Vote::Reject)
            .unwrap();

        let vote2 = VoteCast::new(NodeId::new("node-2"), proposal.proposal_id, Vote::Reject);
        manager.record_vote(vote2).unwrap();

        let vote3 = VoteCast::new(NodeId::new("node-3"), proposal.proposal_id, Vote::Accept);
        manager.record_vote(vote3).unwrap();

        let result = manager.get_consensus_result(&proposal.proposal_id).unwrap();
        assert_eq!(result.reject_votes, 2);
    }

    #[test]
    fn test_calculate_quorum() {
        assert_eq!(ConsensusManager::calculate_quorum(3), 2);
        assert_eq!(ConsensusManager::calculate_quorum(5), 3);
        assert_eq!(ConsensusManager::calculate_quorum(7), 4);
    }

    #[test]
    fn test_consensus_stats() {
        let node_id = NodeId::new("node-1");
        let config = ConsensusConfig {
            quorum_size: 2,
            ..Default::default()
        };
        let mut manager = ConsensusManager::new(node_id, config);

        let record1 = create_test_record();
        let proposal1 = manager.propose_record(record1);

        manager
            .cast_vote(proposal1.proposal_id, Vote::Accept)
            .unwrap();
        let vote2 = VoteCast::new(NodeId::new("node-2"), proposal1.proposal_id, Vote::Accept);
        manager.record_vote(vote2).unwrap();

        let stats = manager.get_stats();
        // total_proposals includes both active and completed proposals
        assert_eq!(stats.total_proposals, 2);
        assert_eq!(stats.accepted, 1);
        assert_eq!(stats.rejected, 0);
    }

    #[test]
    fn test_complete_proposal() {
        let node_id = NodeId::new("node-1");
        let config = ConsensusConfig::default();
        let mut manager = ConsensusManager::new(node_id, config);

        let record = create_test_record();
        let proposal = manager.propose_record(record);
        let proposal_id = proposal.proposal_id;

        assert!(manager.get_proposal(&proposal_id).is_some());

        let completed = manager.complete_proposal(&proposal_id);
        assert!(completed.is_some());
        assert!(manager.get_proposal(&proposal_id).is_none());
    }
}
