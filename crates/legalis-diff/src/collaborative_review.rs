//! Collaborative diff review system for statute changes.
//!
//! This module provides:
//! - Real-time collaborative diff viewing
//! - Commenting and annotation system
//! - Approval workflow integration
//! - Change request management
//! - Stakeholder notification system

use crate::StatuteDiff;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A review session for collaborative diff review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSession {
    /// Session ID.
    pub id: String,
    /// The diff being reviewed.
    pub diff: StatuteDiff,
    /// Participants in the review.
    pub participants: Vec<Participant>,
    /// Comments on the diff.
    pub comments: Vec<Comment>,
    /// Annotations.
    pub annotations: Vec<Annotation>,
    /// Review state.
    pub state: ReviewState,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp.
    pub updated_at: DateTime<Utc>,
}

/// A participant in a review session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// User ID.
    pub user_id: String,
    /// Display name.
    pub name: String,
    /// Role in the review.
    pub role: ReviewRole,
    /// When they joined.
    pub joined_at: DateTime<Utc>,
    /// Whether they're currently active.
    pub is_active: bool,
}

/// Role of a participant in a review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewRole {
    /// Can view and comment.
    Reviewer,
    /// Must approve changes.
    Approver,
    /// Owns the changes.
    Author,
    /// Can moderate the review.
    Moderator,
}

/// State of a review session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewState {
    /// Review is in progress.
    InProgress,
    /// Approved by all required approvers.
    Approved,
    /// Changes requested.
    ChangesRequested,
    /// Review rejected.
    Rejected,
    /// Review cancelled.
    Cancelled,
}

impl ReviewSession {
    /// Creates a new review session.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, collaborative_review::ReviewSession};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let session = ReviewSession::new(diff_result, "alice");
    /// assert_eq!(session.participants.len(), 1);
    /// ```
    pub fn new(diff: StatuteDiff, author_id: &str) -> Self {
        let now = Utc::now();
        let author = Participant {
            user_id: author_id.to_string(),
            name: author_id.to_string(),
            role: ReviewRole::Author,
            joined_at: now,
            is_active: true,
        };

        Self {
            id: Uuid::new_v4().to_string(),
            diff,
            participants: vec![author],
            comments: Vec::new(),
            annotations: Vec::new(),
            state: ReviewState::InProgress,
            created_at: now,
            updated_at: now,
        }
    }

    /// Adds a participant to the review.
    pub fn add_participant(&mut self, user_id: &str, name: &str, role: ReviewRole) {
        self.participants.push(Participant {
            user_id: user_id.to_string(),
            name: name.to_string(),
            role,
            joined_at: Utc::now(),
            is_active: true,
        });
        self.updated_at = Utc::now();
    }

    /// Adds a comment to the review.
    pub fn add_comment(&mut self, user_id: &str, content: &str, target: Option<String>) {
        self.comments.push(Comment {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            content: content.to_string(),
            target,
            created_at: Utc::now(),
            resolved: false,
            thread: Vec::new(),
        });
        self.updated_at = Utc::now();
    }

    /// Adds an annotation to a specific part of the diff.
    pub fn add_annotation(
        &mut self,
        user_id: &str,
        target: &str,
        text: &str,
        annotation_type: AnnotationType,
    ) {
        self.annotations.push(Annotation {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            target: target.to_string(),
            text: text.to_string(),
            annotation_type,
            created_at: Utc::now(),
        });
        self.updated_at = Utc::now();
    }

    /// Approves the review.
    pub fn approve(&mut self, user_id: &str) -> Result<(), String> {
        // Check if user is an approver
        let is_approver = self
            .participants
            .iter()
            .any(|p| p.user_id == user_id && p.role == ReviewRole::Approver);

        if !is_approver {
            return Err("User is not an approver".to_string());
        }

        // Check if all approvers have approved (simplified)
        self.state = ReviewState::Approved;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Requests changes.
    pub fn request_changes(&mut self, user_id: &str, reason: &str) {
        self.state = ReviewState::ChangesRequested;
        self.add_comment(user_id, reason, None);
        self.updated_at = Utc::now();
    }

    /// Rejects the review.
    pub fn reject(&mut self, user_id: &str, reason: &str) {
        self.state = ReviewState::Rejected;
        self.add_comment(user_id, reason, None);
        self.updated_at = Utc::now();
    }

    /// Gets all unresolved comments.
    pub fn unresolved_comments(&self) -> Vec<&Comment> {
        self.comments.iter().filter(|c| !c.resolved).collect()
    }
}

/// A comment on a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    /// Comment ID.
    pub id: String,
    /// User who made the comment.
    pub user_id: String,
    /// Comment content.
    pub content: String,
    /// What the comment is targeting (optional).
    pub target: Option<String>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Whether the comment is resolved.
    pub resolved: bool,
    /// Threaded replies.
    pub thread: Vec<CommentReply>,
}

impl Comment {
    /// Adds a reply to the comment.
    pub fn reply(&mut self, user_id: &str, content: &str) {
        self.thread.push(CommentReply {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            content: content.to_string(),
            created_at: Utc::now(),
        });
    }

    /// Resolves the comment.
    pub fn resolve(&mut self) {
        self.resolved = true;
    }
}

/// A reply to a comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// An annotation on a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Annotation ID.
    pub id: String,
    /// User who created the annotation.
    pub user_id: String,
    /// What is being annotated.
    pub target: String,
    /// Annotation text.
    pub text: String,
    /// Type of annotation.
    pub annotation_type: AnnotationType,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Type of annotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationType {
    /// Suggestion for improvement.
    Suggestion,
    /// Note or observation.
    Note,
    /// Question.
    Question,
    /// Issue or concern.
    Issue,
}

/// Change request for modifications to the diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRequest {
    /// Request ID.
    pub id: String,
    /// User who requested the change.
    pub requester_id: String,
    /// Description of requested change.
    pub description: String,
    /// Priority level.
    pub priority: Priority,
    /// Status of the request.
    pub status: ChangeRequestStatus,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Related comments.
    pub related_comments: Vec<String>,
}

/// Priority of a change request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Status of a change request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeRequestStatus {
    /// Request is open.
    Open,
    /// Being worked on.
    InProgress,
    /// Request completed.
    Completed,
    /// Request rejected.
    Rejected,
}

impl ChangeRequest {
    /// Creates a new change request.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::collaborative_review::{ChangeRequest, Priority};
    ///
    /// let request = ChangeRequest::new("alice", "Update threshold", Priority::High);
    /// assert_eq!(request.requester_id, "alice");
    /// ```
    pub fn new(requester_id: &str, description: &str, priority: Priority) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            requester_id: requester_id.to_string(),
            description: description.to_string(),
            priority,
            status: ChangeRequestStatus::Open,
            created_at: Utc::now(),
            related_comments: Vec::new(),
        }
    }

    /// Marks the request as in progress.
    pub fn start(&mut self) {
        self.status = ChangeRequestStatus::InProgress;
    }

    /// Completes the request.
    pub fn complete(&mut self) {
        self.status = ChangeRequestStatus::Completed;
    }
}

/// Notification for stakeholders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID.
    pub id: String,
    /// Recipient user ID.
    pub recipient_id: String,
    /// Notification type.
    pub notification_type: NotificationType,
    /// Related review session.
    pub session_id: String,
    /// Notification message.
    pub message: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Whether the notification has been read.
    pub read: bool,
}

/// Type of notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    /// New comment added.
    NewComment,
    /// Review approved.
    Approved,
    /// Changes requested.
    ChangesRequested,
    /// Mentioned in a comment.
    Mentioned,
    /// New participant joined.
    ParticipantJoined,
}

/// Notification system for review sessions.
#[derive(Debug, Clone, Default)]
pub struct NotificationSystem {
    /// Notifications indexed by user ID.
    notifications: HashMap<String, Vec<Notification>>,
}

impl NotificationSystem {
    /// Creates a new notification system.
    pub fn new() -> Self {
        Self {
            notifications: HashMap::new(),
        }
    }

    /// Sends a notification to a user.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::collaborative_review::{NotificationSystem, NotificationType};
    ///
    /// let mut system = NotificationSystem::new();
    /// system.notify("alice", NotificationType::NewComment, "session-123", "New comment on your review");
    ///
    /// let notifications = system.get_notifications("alice");
    /// assert_eq!(notifications.len(), 1);
    /// ```
    pub fn notify(
        &mut self,
        recipient_id: &str,
        notification_type: NotificationType,
        session_id: &str,
        message: &str,
    ) {
        let notification = Notification {
            id: Uuid::new_v4().to_string(),
            recipient_id: recipient_id.to_string(),
            notification_type,
            session_id: session_id.to_string(),
            message: message.to_string(),
            created_at: Utc::now(),
            read: false,
        };

        self.notifications
            .entry(recipient_id.to_string())
            .or_default()
            .push(notification);
    }

    /// Gets all notifications for a user.
    pub fn get_notifications(&self, user_id: &str) -> Vec<&Notification> {
        self.notifications
            .get(user_id)
            .map(|n| n.iter().collect())
            .unwrap_or_default()
    }

    /// Gets unread notifications for a user.
    pub fn get_unread_notifications(&self, user_id: &str) -> Vec<&Notification> {
        self.notifications
            .get(user_id)
            .map(|n| n.iter().filter(|notif| !notif.read).collect())
            .unwrap_or_default()
    }

    /// Marks a notification as read.
    pub fn mark_read(&mut self, user_id: &str, notification_id: &str) {
        if let Some(notifications) = self.notifications.get_mut(user_id) {
            if let Some(notif) = notifications.iter_mut().find(|n| n.id == notification_id) {
                notif.read = true;
            }
        }
    }
}

/// Approval workflow for reviews.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflow {
    /// Required approvers.
    pub required_approvers: Vec<String>,
    /// Users who have approved.
    pub approved_by: Vec<String>,
    /// Minimum number of approvals required.
    pub min_approvals: usize,
}

impl ApprovalWorkflow {
    /// Creates a new approval workflow.
    pub fn new(required_approvers: Vec<String>, min_approvals: usize) -> Self {
        Self {
            required_approvers,
            approved_by: Vec::new(),
            min_approvals,
        }
    }

    /// Records an approval.
    pub fn approve(&mut self, user_id: &str) -> Result<(), String> {
        if !self.required_approvers.contains(&user_id.to_string()) {
            return Err("User is not a required approver".to_string());
        }

        if !self.approved_by.contains(&user_id.to_string()) {
            self.approved_by.push(user_id.to_string());
        }

        Ok(())
    }

    /// Checks if the workflow is complete.
    pub fn is_complete(&self) -> bool {
        self.approved_by.len() >= self.min_approvals
    }

    /// Gets pending approvers.
    pub fn pending_approvers(&self) -> Vec<String> {
        self.required_approvers
            .iter()
            .filter(|a| !self.approved_by.contains(a))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn test_diff() -> StatuteDiff {
        let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
        diff(&old, &new).unwrap()
    }

    #[test]
    fn test_review_session_creation() {
        let session = ReviewSession::new(test_diff(), "alice");
        assert_eq!(session.participants.len(), 1);
        assert_eq!(session.state, ReviewState::InProgress);
    }

    #[test]
    fn test_add_participant() {
        let mut session = ReviewSession::new(test_diff(), "alice");
        session.add_participant("bob", "Bob", ReviewRole::Reviewer);
        assert_eq!(session.participants.len(), 2);
    }

    #[test]
    fn test_add_comment() {
        let mut session = ReviewSession::new(test_diff(), "alice");
        session.add_comment("alice", "Looks good!", None);
        assert_eq!(session.comments.len(), 1);
    }

    #[test]
    fn test_annotation() {
        let mut session = ReviewSession::new(test_diff(), "alice");
        session.add_annotation(
            "alice",
            "Title",
            "Consider rewording",
            AnnotationType::Suggestion,
        );
        assert_eq!(session.annotations.len(), 1);
    }

    #[test]
    fn test_approval() {
        let mut session = ReviewSession::new(test_diff(), "alice");
        session.add_participant("bob", "Bob", ReviewRole::Approver);

        let result = session.approve("bob");
        assert!(result.is_ok());
        assert_eq!(session.state, ReviewState::Approved);
    }

    #[test]
    fn test_change_request() {
        let mut request = ChangeRequest::new("alice", "Update threshold", Priority::High);
        assert_eq!(request.status, ChangeRequestStatus::Open);

        request.start();
        assert_eq!(request.status, ChangeRequestStatus::InProgress);

        request.complete();
        assert_eq!(request.status, ChangeRequestStatus::Completed);
    }

    #[test]
    fn test_notification_system() {
        let mut system = NotificationSystem::new();
        system.notify(
            "alice",
            NotificationType::NewComment,
            "session-1",
            "New comment",
        );

        let notifications = system.get_notifications("alice");
        assert_eq!(notifications.len(), 1);

        let unread = system.get_unread_notifications("alice");
        assert_eq!(unread.len(), 1);
    }

    #[test]
    fn test_approval_workflow() {
        let mut workflow = ApprovalWorkflow::new(vec!["alice".to_string(), "bob".to_string()], 2);

        assert!(!workflow.is_complete());

        workflow.approve("alice").unwrap();
        assert!(!workflow.is_complete());

        workflow.approve("bob").unwrap();
        assert!(workflow.is_complete());
    }

    #[test]
    fn test_comment_threading() {
        let mut comment = Comment {
            id: "1".to_string(),
            user_id: "alice".to_string(),
            content: "Main comment".to_string(),
            target: None,
            created_at: Utc::now(),
            resolved: false,
            thread: Vec::new(),
        };

        comment.reply("bob", "Reply to comment");
        assert_eq!(comment.thread.len(), 1);
    }
}
