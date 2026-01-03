//! Collaborative features for diff review and approval.
//!
//! This module provides data structures and functions for collaborative
//! review of statute diffs, including comments, approvals, and voting.
//!
//! # Examples
//!
//! ```
//! use legalis_diff::collaborative::{Comment, ApprovalWorkflow, Vote, VoteType};
//! use chrono::Utc;
//!
//! // Create a comment on a change
//! let comment = Comment::new(
//!     "user@example.com",
//!     "This change looks good but needs clarification",
//!     Some(0), // Referring to change index 0
//! );
//!
//! // Create an approval workflow
//! let mut workflow = ApprovalWorkflow::new("statute-123", vec![
//!     "reviewer1@example.com".to_string(),
//!     "reviewer2@example.com".to_string(),
//! ]);
//!
//! // Add a vote
//! let vote = Vote::new("reviewer1@example.com", VoteType::Approve);
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A comment on a diff or specific change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    /// Unique identifier for the comment.
    pub id: String,
    /// Author of the comment (email or user ID).
    pub author: String,
    /// Comment text.
    pub text: String,
    /// Timestamp when the comment was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp when the comment was last edited.
    pub edited_at: Option<DateTime<Utc>>,
    /// Index of the change this comment refers to (None for general comments).
    pub change_index: Option<usize>,
    /// Replies to this comment.
    pub replies: Vec<Comment>,
    /// Whether the comment has been resolved.
    pub resolved: bool,
}

impl Comment {
    /// Creates a new comment.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::collaborative::Comment;
    ///
    /// let comment = Comment::new(
    ///     "user@example.com",
    ///     "This needs review",
    ///     Some(0),
    /// );
    ///
    /// assert_eq!(comment.author, "user@example.com");
    /// assert!(!comment.resolved);
    /// ```
    pub fn new(author: &str, text: &str, change_index: Option<usize>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            author: author.to_string(),
            text: text.to_string(),
            created_at: Utc::now(),
            edited_at: None,
            change_index,
            replies: Vec::new(),
            resolved: false,
        }
    }

    /// Adds a reply to this comment.
    pub fn add_reply(&mut self, reply: Comment) {
        self.replies.push(reply);
    }

    /// Marks the comment as resolved.
    pub fn resolve(&mut self) {
        self.resolved = true;
    }

    /// Edits the comment text.
    pub fn edit(&mut self, new_text: &str) {
        self.text = new_text.to_string();
        self.edited_at = Some(Utc::now());
    }
}

/// Type of vote on a diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteType {
    /// Approve the change.
    Approve,
    /// Reject the change.
    Reject,
    /// Request changes before approval.
    RequestChanges,
    /// Abstain from voting.
    Abstain,
}

/// A vote on a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Voter identifier (email or user ID).
    pub voter: String,
    /// Type of vote.
    pub vote_type: VoteType,
    /// Timestamp of the vote.
    pub voted_at: DateTime<Utc>,
    /// Optional comment explaining the vote.
    pub comment: Option<String>,
}

impl Vote {
    /// Creates a new vote.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::collaborative::{Vote, VoteType};
    ///
    /// let vote = Vote::new("reviewer@example.com", VoteType::Approve);
    /// assert_eq!(vote.voter, "reviewer@example.com");
    /// assert_eq!(vote.vote_type, VoteType::Approve);
    /// ```
    pub fn new(voter: &str, vote_type: VoteType) -> Self {
        Self {
            voter: voter.to_string(),
            vote_type,
            voted_at: Utc::now(),
            comment: None,
        }
    }

    /// Creates a new vote with a comment.
    pub fn with_comment(voter: &str, vote_type: VoteType, comment: &str) -> Self {
        Self {
            voter: voter.to_string(),
            vote_type,
            voted_at: Utc::now(),
            comment: Some(comment.to_string()),
        }
    }
}

/// Status of an approval workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    /// Pending approval.
    Pending,
    /// Approved by all required reviewers.
    Approved,
    /// Rejected by one or more reviewers.
    Rejected,
    /// Changes requested before approval.
    ChangesRequested,
}

/// An approval workflow for a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflow {
    /// The statute ID this workflow is for.
    pub statute_id: String,
    /// Required reviewers (must all approve).
    pub required_reviewers: Vec<String>,
    /// Optional reviewers.
    pub optional_reviewers: Vec<String>,
    /// Votes received so far.
    pub votes: HashMap<String, Vote>,
    /// Current status of the workflow.
    pub status: ApprovalStatus,
    /// Timestamp when workflow was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp when workflow was completed (approved or rejected).
    pub completed_at: Option<DateTime<Utc>>,
}

impl ApprovalWorkflow {
    /// Creates a new approval workflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::collaborative::ApprovalWorkflow;
    ///
    /// let workflow = ApprovalWorkflow::new("statute-123", vec![
    ///     "reviewer1@example.com".to_string(),
    ///     "reviewer2@example.com".to_string(),
    /// ]);
    ///
    /// assert_eq!(workflow.statute_id, "statute-123");
    /// ```
    pub fn new(statute_id: &str, required_reviewers: Vec<String>) -> Self {
        Self {
            statute_id: statute_id.to_string(),
            required_reviewers,
            optional_reviewers: Vec::new(),
            votes: HashMap::new(),
            status: ApprovalStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    /// Adds an optional reviewer.
    pub fn add_optional_reviewer(&mut self, reviewer: String) {
        if !self.optional_reviewers.contains(&reviewer) {
            self.optional_reviewers.push(reviewer);
        }
    }

    /// Adds a vote and updates the workflow status.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::collaborative::{ApprovalWorkflow, Vote, VoteType, ApprovalStatus};
    ///
    /// let mut workflow = ApprovalWorkflow::new("statute-123", vec![
    ///     "reviewer@example.com".to_string(),
    /// ]);
    ///
    /// let vote = Vote::new("reviewer@example.com", VoteType::Approve);
    /// workflow.add_vote(vote);
    ///
    /// assert_eq!(workflow.status, ApprovalStatus::Approved);
    /// ```
    pub fn add_vote(&mut self, vote: Vote) {
        self.votes.insert(vote.voter.clone(), vote);
        self.update_status();
    }

    /// Updates the workflow status based on votes.
    fn update_status(&mut self) {
        // Check if any required reviewer rejected or requested changes
        for reviewer in &self.required_reviewers {
            if let Some(vote) = self.votes.get(reviewer) {
                match vote.vote_type {
                    VoteType::Reject => {
                        self.status = ApprovalStatus::Rejected;
                        self.completed_at = Some(Utc::now());
                        return;
                    }
                    VoteType::RequestChanges => {
                        self.status = ApprovalStatus::ChangesRequested;
                        return;
                    }
                    _ => {}
                }
            }
        }

        // Check if all required reviewers have approved
        let all_approved = self.required_reviewers.iter().all(|reviewer| {
            self.votes
                .get(reviewer)
                .map(|v| v.vote_type == VoteType::Approve)
                .unwrap_or(false)
        });

        if all_approved {
            self.status = ApprovalStatus::Approved;
            self.completed_at = Some(Utc::now());
        } else {
            self.status = ApprovalStatus::Pending;
        }
    }

    /// Gets the list of reviewers who haven't voted yet.
    pub fn pending_reviewers(&self) -> Vec<String> {
        self.required_reviewers
            .iter()
            .filter(|r| !self.votes.contains_key(*r))
            .cloned()
            .collect()
    }

    /// Gets voting statistics.
    pub fn vote_stats(&self) -> VoteStats {
        let mut stats = VoteStats::default();

        for vote in self.votes.values() {
            match vote.vote_type {
                VoteType::Approve => stats.approvals += 1,
                VoteType::Reject => stats.rejections += 1,
                VoteType::RequestChanges => stats.change_requests += 1,
                VoteType::Abstain => stats.abstentions += 1,
            }
        }

        stats.total_reviewers = self.required_reviewers.len() + self.optional_reviewers.len();
        stats.total_votes = self.votes.len();

        stats
    }
}

/// Voting statistics.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VoteStats {
    /// Total number of reviewers.
    pub total_reviewers: usize,
    /// Total votes cast.
    pub total_votes: usize,
    /// Number of approvals.
    pub approvals: usize,
    /// Number of rejections.
    pub rejections: usize,
    /// Number of change requests.
    pub change_requests: usize,
    /// Number of abstentions.
    pub abstentions: usize,
}

/// A review session for collaborative diff analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSession {
    /// Session ID.
    pub id: String,
    /// The statute ID being reviewed.
    pub statute_id: String,
    /// Comments on the diff.
    pub comments: Vec<Comment>,
    /// The approval workflow.
    pub workflow: ApprovalWorkflow,
    /// Session metadata.
    pub metadata: HashMap<String, String>,
}

impl ReviewSession {
    /// Creates a new review session.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::collaborative::ReviewSession;
    ///
    /// let session = ReviewSession::new("statute-123", vec![
    ///     "reviewer1@example.com".to_string(),
    /// ]);
    ///
    /// assert_eq!(session.statute_id, "statute-123");
    /// ```
    pub fn new(statute_id: &str, required_reviewers: Vec<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            statute_id: statute_id.to_string(),
            comments: Vec::new(),
            workflow: ApprovalWorkflow::new(statute_id, required_reviewers),
            metadata: HashMap::new(),
        }
    }

    /// Adds a comment to the session.
    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
    }

    /// Gets all comments for a specific change.
    pub fn comments_for_change(&self, change_index: usize) -> Vec<&Comment> {
        self.comments
            .iter()
            .filter(|c| c.change_index == Some(change_index))
            .collect()
    }

    /// Gets all unresolved comments.
    pub fn unresolved_comments(&self) -> Vec<&Comment> {
        self.comments.iter().filter(|c| !c.resolved).collect()
    }

    /// Gets general comments (not tied to specific changes).
    pub fn general_comments(&self) -> Vec<&Comment> {
        self.comments
            .iter()
            .filter(|c| c.change_index.is_none())
            .collect()
    }
}

/// Conflict resolution strategy for collaborative editing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    /// Use the version from the first reviewer.
    UseFirst,
    /// Use the version from the second reviewer.
    UseSecond,
    /// Merge both versions if possible.
    Merge,
    /// Require manual resolution.
    Manual,
}

/// A conflict detected during collaborative editing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditConflict {
    /// Location of the conflict.
    pub location: String,
    /// First conflicting change.
    pub change1: String,
    /// Second conflicting change.
    pub change2: String,
    /// Suggested resolution strategy.
    pub suggested_strategy: ConflictResolutionStrategy,
    /// Whether the conflict has been resolved.
    pub resolved: bool,
}

impl EditConflict {
    /// Creates a new edit conflict.
    pub fn new(location: &str, change1: &str, change2: &str) -> Self {
        Self {
            location: location.to_string(),
            change1: change1.to_string(),
            change2: change2.to_string(),
            suggested_strategy: ConflictResolutionStrategy::Manual,
            resolved: false,
        }
    }

    /// Marks the conflict as resolved.
    pub fn resolve(&mut self) {
        self.resolved = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_creation() {
        let comment = Comment::new("user@example.com", "Test comment", Some(0));
        assert_eq!(comment.author, "user@example.com");
        assert_eq!(comment.text, "Test comment");
        assert_eq!(comment.change_index, Some(0));
        assert!(!comment.resolved);
    }

    #[test]
    fn test_comment_reply() {
        let mut comment = Comment::new("user1@example.com", "Original", None);
        let reply = Comment::new("user2@example.com", "Reply", None);
        comment.add_reply(reply);
        assert_eq!(comment.replies.len(), 1);
    }

    #[test]
    fn test_comment_resolve() {
        let mut comment = Comment::new("user@example.com", "Test", None);
        comment.resolve();
        assert!(comment.resolved);
    }

    #[test]
    fn test_vote_creation() {
        let vote = Vote::new("reviewer@example.com", VoteType::Approve);
        assert_eq!(vote.voter, "reviewer@example.com");
        assert_eq!(vote.vote_type, VoteType::Approve);
        assert!(vote.comment.is_none());
    }

    #[test]
    fn test_vote_with_comment() {
        let vote = Vote::with_comment(
            "reviewer@example.com",
            VoteType::RequestChanges,
            "Need more details",
        );
        assert_eq!(vote.voter, "reviewer@example.com");
        assert_eq!(vote.vote_type, VoteType::RequestChanges);
        assert_eq!(vote.comment, Some("Need more details".to_string()));
    }

    #[test]
    fn test_approval_workflow_creation() {
        let workflow = ApprovalWorkflow::new(
            "statute-123",
            vec![
                "reviewer1@example.com".to_string(),
                "reviewer2@example.com".to_string(),
            ],
        );
        assert_eq!(workflow.statute_id, "statute-123");
        assert_eq!(workflow.status, ApprovalStatus::Pending);
        assert_eq!(workflow.required_reviewers.len(), 2);
    }

    #[test]
    fn test_approval_workflow_single_approval() {
        let mut workflow =
            ApprovalWorkflow::new("statute-123", vec!["reviewer@example.com".to_string()]);

        let vote = Vote::new("reviewer@example.com", VoteType::Approve);
        workflow.add_vote(vote);

        assert_eq!(workflow.status, ApprovalStatus::Approved);
        assert!(workflow.completed_at.is_some());
    }

    #[test]
    fn test_approval_workflow_rejection() {
        let mut workflow = ApprovalWorkflow::new(
            "statute-123",
            vec![
                "reviewer1@example.com".to_string(),
                "reviewer2@example.com".to_string(),
            ],
        );

        let vote = Vote::new("reviewer1@example.com", VoteType::Reject);
        workflow.add_vote(vote);

        assert_eq!(workflow.status, ApprovalStatus::Rejected);
    }

    #[test]
    fn test_approval_workflow_changes_requested() {
        let mut workflow =
            ApprovalWorkflow::new("statute-123", vec!["reviewer@example.com".to_string()]);

        let vote = Vote::new("reviewer@example.com", VoteType::RequestChanges);
        workflow.add_vote(vote);

        assert_eq!(workflow.status, ApprovalStatus::ChangesRequested);
    }

    #[test]
    fn test_approval_workflow_pending_reviewers() {
        let mut workflow = ApprovalWorkflow::new(
            "statute-123",
            vec![
                "reviewer1@example.com".to_string(),
                "reviewer2@example.com".to_string(),
            ],
        );

        let vote = Vote::new("reviewer1@example.com", VoteType::Approve);
        workflow.add_vote(vote);

        let pending = workflow.pending_reviewers();
        assert_eq!(pending.len(), 1);
        assert!(pending.contains(&"reviewer2@example.com".to_string()));
    }

    #[test]
    fn test_vote_stats() {
        let mut workflow = ApprovalWorkflow::new(
            "statute-123",
            vec![
                "reviewer1@example.com".to_string(),
                "reviewer2@example.com".to_string(),
            ],
        );

        workflow.add_vote(Vote::new("reviewer1@example.com", VoteType::Approve));
        workflow.add_vote(Vote::new("reviewer2@example.com", VoteType::Approve));

        let stats = workflow.vote_stats();
        assert_eq!(stats.total_reviewers, 2);
        assert_eq!(stats.total_votes, 2);
        assert_eq!(stats.approvals, 2);
    }

    #[test]
    fn test_review_session() {
        let mut session =
            ReviewSession::new("statute-123", vec!["reviewer@example.com".to_string()]);

        let comment = Comment::new("user@example.com", "Test comment", Some(0));
        session.add_comment(comment);

        assert_eq!(session.comments.len(), 1);
        assert_eq!(session.comments_for_change(0).len(), 1);
    }

    #[test]
    fn test_edit_conflict() {
        let mut conflict = EditConflict::new("Title", "Change A", "Change B");
        assert!(!conflict.resolved);

        conflict.resolve();
        assert!(conflict.resolved);
    }
}
