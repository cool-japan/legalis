//! Crowdsourced ontology evolution framework.
//!
//! This module provides tools for collaborative ontology development:
//! - Change proposals from community members
//! - Review and voting mechanisms
//! - Conflict detection and resolution
//! - Automated change application

use crate::Triple;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of change proposed to the ontology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeProposalType {
    /// Add a new class
    AddClass,
    /// Remove an existing class
    RemoveClass,
    /// Modify class definition
    ModifyClass,
    /// Add a new property
    AddProperty,
    /// Remove an existing property
    RemoveProperty,
    /// Modify property definition
    ModifyProperty,
    /// Add documentation
    AddDocumentation,
    /// Fix inconsistency
    FixInconsistency,
    /// Other change
    Other(String),
}

/// Status of a change proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    /// Newly submitted, awaiting review
    Submitted,
    /// Under review
    UnderReview,
    /// Approved and ready to apply
    Approved,
    /// Rejected
    Rejected,
    /// Applied to ontology
    Applied,
    /// Withdrawn by author
    Withdrawn,
}

/// A proposed change to the ontology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeProposal {
    /// Unique proposal ID
    pub id: String,
    /// Type of change
    pub change_type: ChangeProposalType,
    /// Status of the proposal
    pub status: ProposalStatus,
    /// Author of the proposal
    pub author: String,
    /// Submission timestamp
    pub submitted_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Title of the proposal
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Rationale for the change
    pub rationale: String,
    /// Triples to add
    pub triples_to_add: Vec<Triple>,
    /// Triples to remove
    pub triples_to_remove: Vec<Triple>,
    /// Votes received
    pub votes: Vec<Vote>,
    /// Review comments
    pub comments: Vec<Comment>,
    /// Related proposals (by ID)
    pub related_proposals: Vec<String>,
}

impl ChangeProposal {
    /// Creates a new change proposal.
    pub fn new(
        id: impl Into<String>,
        change_type: ChangeProposalType,
        author: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            change_type,
            status: ProposalStatus::Submitted,
            author: author.into(),
            submitted_at: now,
            modified_at: now,
            title: title.into(),
            description: description.into(),
            rationale: String::new(),
            triples_to_add: Vec::new(),
            triples_to_remove: Vec::new(),
            votes: Vec::new(),
            comments: Vec::new(),
            related_proposals: Vec::new(),
        }
    }

    /// Sets the rationale.
    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = rationale.into();
        self
    }

    /// Adds triples to add.
    pub fn with_triples_to_add(mut self, triples: Vec<Triple>) -> Self {
        self.triples_to_add = triples;
        self
    }

    /// Adds triples to remove.
    #[allow(dead_code)]
    pub fn with_triples_to_remove(mut self, triples: Vec<Triple>) -> Self {
        self.triples_to_remove = triples;
        self
    }

    /// Adds a vote.
    pub fn add_vote(&mut self, vote: Vote) {
        // Remove any existing vote from the same voter
        self.votes.retain(|v| v.voter != vote.voter);
        self.votes.push(vote);
        self.modified_at = Utc::now();
    }

    /// Adds a comment.
    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
        self.modified_at = Utc::now();
    }

    /// Gets vote counts.
    pub fn get_vote_counts(&self) -> (usize, usize, usize) {
        let approve = self
            .votes
            .iter()
            .filter(|v| v.vote_type == VoteType::Approve)
            .count();
        let reject = self
            .votes
            .iter()
            .filter(|v| v.vote_type == VoteType::Reject)
            .count();
        let abstain = self
            .votes
            .iter()
            .filter(|v| v.vote_type == VoteType::Abstain)
            .count();
        (approve, reject, abstain)
    }

    /// Checks if proposal has enough votes to be approved.
    pub fn has_consensus(&self, min_votes: usize, approval_threshold: f64) -> bool {
        let (approve, reject, _abstain) = self.get_vote_counts();
        let total = approve + reject;

        if total < min_votes {
            return false;
        }

        let approval_rate = approve as f64 / total as f64;
        approval_rate >= approval_threshold
    }
}

/// Type of vote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteType {
    /// Approve the proposal
    Approve,
    /// Reject the proposal
    Reject,
    /// Abstain from voting
    Abstain,
}

/// A vote on a proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Voter identifier
    pub voter: String,
    /// Type of vote
    pub vote_type: VoteType,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Optional comment
    pub comment: Option<String>,
}

impl Vote {
    /// Creates a new vote.
    pub fn new(voter: impl Into<String>, vote_type: VoteType) -> Self {
        Self {
            voter: voter.into(),
            vote_type,
            timestamp: Utc::now(),
            comment: None,
        }
    }

    /// Sets a comment.
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
}

/// A comment on a proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    /// Commenter identifier
    pub author: String,
    /// Comment text
    pub text: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl Comment {
    /// Creates a new comment.
    pub fn new(author: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            author: author.into(),
            text: text.into(),
            timestamp: Utc::now(),
        }
    }
}

/// Manages the collaborative evolution process.
pub struct EvolutionManager {
    /// All proposals
    proposals: HashMap<String, ChangeProposal>,
    /// Configuration
    config: EvolutionConfig,
}

/// Configuration for the evolution process.
#[derive(Debug, Clone)]
pub struct EvolutionConfig {
    /// Minimum number of votes required
    pub min_votes: usize,
    /// Approval threshold (0.0 to 1.0)
    pub approval_threshold: f64,
    /// Require rationale
    pub require_rationale: bool,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            min_votes: 3,
            approval_threshold: 0.66,
            require_rationale: true,
        }
    }
}

impl EvolutionManager {
    /// Creates a new evolution manager.
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            config: EvolutionConfig::default(),
        }
    }

    /// Creates a manager with custom config.
    pub fn with_config(config: EvolutionConfig) -> Self {
        Self {
            proposals: HashMap::new(),
            config,
        }
    }

    /// Submits a new proposal.
    pub fn submit_proposal(&mut self, proposal: ChangeProposal) -> Result<(), String> {
        // Validate proposal
        if self.config.require_rationale && proposal.rationale.is_empty() {
            return Err("Rationale is required".to_string());
        }

        if proposal.triples_to_add.is_empty() && proposal.triples_to_remove.is_empty() {
            return Err("Proposal must add or remove at least one triple".to_string());
        }

        self.proposals.insert(proposal.id.clone(), proposal);
        Ok(())
    }

    /// Gets a proposal by ID.
    pub fn get_proposal(&self, id: &str) -> Option<&ChangeProposal> {
        self.proposals.get(id)
    }

    /// Gets all proposals with a given status.
    pub fn get_proposals_by_status(&self, status: ProposalStatus) -> Vec<&ChangeProposal> {
        self.proposals
            .values()
            .filter(|p| p.status == status)
            .collect()
    }

    /// Votes on a proposal.
    pub fn vote_on_proposal(&mut self, proposal_id: &str, vote: Vote) -> Result<(), String> {
        let proposal = self
            .proposals
            .get_mut(proposal_id)
            .ok_or("Proposal not found")?;

        if proposal.status != ProposalStatus::Submitted
            && proposal.status != ProposalStatus::UnderReview
        {
            return Err("Proposal is not open for voting".to_string());
        }

        proposal.add_vote(vote);

        // Check if consensus is reached
        if proposal.has_consensus(self.config.min_votes, self.config.approval_threshold) {
            proposal.status = ProposalStatus::Approved;
        }

        Ok(())
    }

    /// Adds a comment to a proposal.
    pub fn comment_on_proposal(
        &mut self,
        proposal_id: &str,
        comment: Comment,
    ) -> Result<(), String> {
        let proposal = self
            .proposals
            .get_mut(proposal_id)
            .ok_or("Proposal not found")?;

        proposal.add_comment(comment);
        Ok(())
    }

    /// Applies an approved proposal to the ontology.
    pub fn apply_proposal(
        &mut self,
        proposal_id: &str,
        current_triples: &mut Vec<Triple>,
    ) -> Result<(), String> {
        let proposal = self
            .proposals
            .get_mut(proposal_id)
            .ok_or("Proposal not found")?;

        if proposal.status != ProposalStatus::Approved {
            return Err("Proposal is not approved".to_string());
        }

        // Remove triples
        for triple_to_remove in &proposal.triples_to_remove {
            current_triples.retain(|t| {
                t.subject != triple_to_remove.subject
                    || t.predicate != triple_to_remove.predicate
                    || t.object != triple_to_remove.object
            });
        }

        // Add triples
        current_triples.extend(proposal.triples_to_add.clone());

        proposal.status = ProposalStatus::Applied;
        Ok(())
    }

    /// Detects conflicts between proposals.
    pub fn detect_conflicts(&self, proposal_id: &str) -> Vec<String> {
        let mut conflicts = Vec::new();

        if let Some(proposal) = self.proposals.get(proposal_id) {
            for (other_id, other_proposal) in &self.proposals {
                if other_id == proposal_id {
                    continue;
                }

                // Check for conflicting changes
                if self.proposals_conflict(proposal, other_proposal) {
                    conflicts.push(other_id.clone());
                }
            }
        }

        conflicts
    }

    /// Checks if two proposals conflict.
    fn proposals_conflict(&self, p1: &ChangeProposal, p2: &ChangeProposal) -> bool {
        // Check if they modify the same entities
        let p1_subjects: Vec<_> = p1
            .triples_to_add
            .iter()
            .chain(p1.triples_to_remove.iter())
            .map(|t| &t.subject)
            .collect();

        let p2_subjects: Vec<_> = p2
            .triples_to_add
            .iter()
            .chain(p2.triples_to_remove.iter())
            .map(|t| &t.subject)
            .collect();

        p1_subjects.iter().any(|s| p2_subjects.contains(s))
    }

    /// Gets statistics about the evolution process.
    pub fn get_statistics(&self) -> EvolutionStatistics {
        let mut stats = EvolutionStatistics {
            total_proposals: self.proposals.len(),
            submitted: 0,
            under_review: 0,
            approved: 0,
            rejected: 0,
            applied: 0,
            withdrawn: 0,
        };

        for proposal in self.proposals.values() {
            match proposal.status {
                ProposalStatus::Submitted => stats.submitted += 1,
                ProposalStatus::UnderReview => stats.under_review += 1,
                ProposalStatus::Approved => stats.approved += 1,
                ProposalStatus::Rejected => stats.rejected += 1,
                ProposalStatus::Applied => stats.applied += 1,
                ProposalStatus::Withdrawn => stats.withdrawn += 1,
            }
        }

        stats
    }
}

impl Default for EvolutionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the evolution process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStatistics {
    pub total_proposals: usize,
    pub submitted: usize,
    pub under_review: usize,
    pub approved: usize,
    pub rejected: usize,
    pub applied: usize,
    pub withdrawn: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RdfValue;

    fn sample_proposal() -> ChangeProposal {
        ChangeProposal::new(
            "prop-1",
            ChangeProposalType::AddClass,
            "user1",
            "Add LegalPerson class",
            "Add a new class for legal persons",
        )
        .with_rationale("Need to distinguish natural and legal persons")
        .with_triples_to_add(vec![Triple {
            subject: "http://example.org/LegalPerson".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Class".to_string()),
        }])
    }

    #[test]
    fn test_proposal_creation() {
        let proposal = sample_proposal();
        assert_eq!(proposal.id, "prop-1");
        assert_eq!(proposal.status, ProposalStatus::Submitted);
        assert_eq!(proposal.triples_to_add.len(), 1);
    }

    #[test]
    fn test_voting() {
        let mut proposal = sample_proposal();

        proposal.add_vote(Vote::new("voter1", VoteType::Approve));
        proposal.add_vote(Vote::new("voter2", VoteType::Approve));
        proposal.add_vote(Vote::new("voter3", VoteType::Reject));

        let (approve, reject, _) = proposal.get_vote_counts();
        assert_eq!(approve, 2);
        assert_eq!(reject, 1);
    }

    #[test]
    fn test_consensus() {
        let mut proposal = sample_proposal();

        proposal.add_vote(Vote::new("voter1", VoteType::Approve));
        proposal.add_vote(Vote::new("voter2", VoteType::Approve));
        proposal.add_vote(Vote::new("voter3", VoteType::Approve));

        assert!(proposal.has_consensus(3, 0.66));
    }

    #[test]
    fn test_evolution_manager() {
        let mut manager = EvolutionManager::new();
        let proposal = sample_proposal();

        manager.submit_proposal(proposal).unwrap();
        assert_eq!(manager.proposals.len(), 1);
        assert!(manager.get_proposal("prop-1").is_some());
    }

    #[test]
    fn test_vote_on_proposal() {
        let mut manager = EvolutionManager::new();
        let proposal = sample_proposal();
        manager.submit_proposal(proposal).unwrap();

        let vote = Vote::new("voter1", VoteType::Approve);
        manager.vote_on_proposal("prop-1", vote).unwrap();

        let proposal = manager.get_proposal("prop-1").unwrap();
        assert_eq!(proposal.votes.len(), 1);
    }

    #[test]
    fn test_apply_proposal() {
        let mut manager = EvolutionManager::new();
        let mut proposal = sample_proposal();

        // Approve the proposal
        proposal.add_vote(Vote::new("voter1", VoteType::Approve));
        proposal.add_vote(Vote::new("voter2", VoteType::Approve));
        proposal.add_vote(Vote::new("voter3", VoteType::Approve));
        proposal.status = ProposalStatus::Approved;

        manager.submit_proposal(proposal).unwrap();

        let mut triples = Vec::new();
        manager.apply_proposal("prop-1", &mut triples).unwrap();

        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].subject, "http://example.org/LegalPerson");
    }

    #[test]
    fn test_conflict_detection() {
        let mut manager = EvolutionManager::new();

        let proposal1 = ChangeProposal::new(
            "prop-1",
            ChangeProposalType::AddClass,
            "user1",
            "Add Person",
            "Add Person class",
        )
        .with_rationale("Need Person class")
        .with_triples_to_add(vec![Triple {
            subject: "http://example.org/Person".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Class".to_string()),
        }]);

        let proposal2 = ChangeProposal::new(
            "prop-2",
            ChangeProposalType::RemoveClass,
            "user2",
            "Remove Person",
            "Remove Person class",
        )
        .with_rationale("Not needed")
        .with_triples_to_remove(vec![Triple {
            subject: "http://example.org/Person".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Class".to_string()),
        }]);

        manager.submit_proposal(proposal1).unwrap();
        manager.submit_proposal(proposal2).unwrap();

        let conflicts = manager.detect_conflicts("prop-1");
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0], "prop-2");
    }

    #[test]
    fn test_statistics() {
        let mut manager = EvolutionManager::new();

        manager.submit_proposal(sample_proposal()).unwrap();

        let mut proposal2 = ChangeProposal::new(
            "prop-2",
            ChangeProposalType::AddProperty,
            "user2",
            "Add property",
            "Description",
        )
        .with_rationale("Needed")
        .with_triples_to_add(vec![Triple {
            subject: "http://example.org/hasProperty".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:ObjectProperty".to_string()),
        }]);
        proposal2.status = ProposalStatus::Approved;
        manager.submit_proposal(proposal2).unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_proposals, 2);
        assert_eq!(stats.submitted, 1);
        assert_eq!(stats.approved, 1);
    }

    #[test]
    fn test_comment_on_proposal() {
        let mut manager = EvolutionManager::new();
        manager.submit_proposal(sample_proposal()).unwrap();

        let comment = Comment::new("user2", "This looks good!");
        manager.comment_on_proposal("prop-1", comment).unwrap();

        let proposal = manager.get_proposal("prop-1").unwrap();
        assert_eq!(proposal.comments.len(), 1);
    }
}
