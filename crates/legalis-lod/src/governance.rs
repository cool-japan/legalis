//! Knowledge governance workflows for enterprise RDF management.
//!
//! This module provides tools for managing knowledge governance:
//! - Data stewardship workflows
//! - Approval processes for changes
//! - Data quality reviews
//! - Lifecycle management

use crate::Triple;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workflow state for governance processes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowState {
    /// Initial state - draft
    Draft,
    /// Submitted for review
    PendingReview,
    /// Under review
    InReview,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Published/active
    Published,
    /// Deprecated
    Deprecated,
    /// Archived
    Archived,
}

/// A governance workflow for knowledge changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceWorkflow {
    /// Workflow ID
    pub id: String,
    /// Workflow name
    pub name: String,
    /// Current state
    pub state: WorkflowState,
    /// Triples being governed
    pub triples: Vec<Triple>,
    /// Submitter
    pub submitter: String,
    /// Assigned reviewers
    pub reviewers: Vec<String>,
    /// Approvers
    pub approvers: Vec<String>,
    /// Reviews received
    pub reviews: Vec<Review>,
    /// Approval decisions
    pub approvals: Vec<Approval>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated
    pub updated_at: DateTime<Utc>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl GovernanceWorkflow {
    /// Creates a new workflow.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        submitter: impl Into<String>,
        triples: Vec<Triple>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            name: name.into(),
            state: WorkflowState::Draft,
            triples,
            submitter: submitter.into(),
            reviewers: Vec::new(),
            approvers: Vec::new(),
            reviews: Vec::new(),
            approvals: Vec::new(),
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        }
    }

    /// Assigns a reviewer.
    pub fn assign_reviewer(&mut self, reviewer: impl Into<String>) {
        self.reviewers.push(reviewer.into());
        self.updated_at = Utc::now();
    }

    /// Assigns an approver.
    pub fn assign_approver(&mut self, approver: impl Into<String>) {
        self.approvers.push(approver.into());
        self.updated_at = Utc::now();
    }

    /// Submits for review.
    pub fn submit_for_review(&mut self) -> Result<(), String> {
        if self.state != WorkflowState::Draft {
            return Err("Can only submit from Draft state".to_string());
        }

        if self.reviewers.is_empty() {
            return Err("At least one reviewer must be assigned".to_string());
        }

        self.state = WorkflowState::PendingReview;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Adds a review.
    pub fn add_review(&mut self, review: Review) -> Result<(), String> {
        if self.state != WorkflowState::PendingReview && self.state != WorkflowState::InReview {
            return Err("Cannot review in current state".to_string());
        }

        self.reviews.push(review);
        self.state = WorkflowState::InReview;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Adds an approval decision.
    pub fn add_approval(&mut self, approval: Approval) -> Result<(), String> {
        if self.state != WorkflowState::InReview {
            return Err("Must be in review state to approve".to_string());
        }

        self.approvals.push(approval);
        self.updated_at = Utc::now();

        // Check if all approvers have decided
        if self.approvals.len() >= self.approvers.len() {
            let all_approved = self.approvals.iter().all(|a| a.approved);
            self.state = if all_approved {
                WorkflowState::Approved
            } else {
                WorkflowState::Rejected
            };
        }

        Ok(())
    }

    /// Publishes the workflow (applies changes).
    pub fn publish(&mut self) -> Result<(), String> {
        if self.state != WorkflowState::Approved {
            return Err("Must be approved before publishing".to_string());
        }

        self.state = WorkflowState::Published;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Archives the workflow.
    pub fn archive(&mut self) {
        self.state = WorkflowState::Archived;
        self.updated_at = Utc::now();
    }
}

/// A review of knowledge changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// Reviewer ID
    pub reviewer: String,
    /// Review timestamp
    pub timestamp: DateTime<Utc>,
    /// Review decision
    pub decision: ReviewDecision,
    /// Comments
    pub comments: String,
    /// Quality rating (1-5)
    pub quality_rating: Option<u8>,
}

/// Review decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewDecision {
    /// Looks good
    Approve,
    /// Needs changes
    RequestChanges,
    /// Cannot proceed
    Reject,
}

impl Review {
    /// Creates a new review.
    pub fn new(
        reviewer: impl Into<String>,
        decision: ReviewDecision,
        comments: impl Into<String>,
    ) -> Self {
        Self {
            reviewer: reviewer.into(),
            timestamp: Utc::now(),
            decision,
            comments: comments.into(),
            quality_rating: None,
        }
    }

    /// Sets quality rating.
    pub fn with_rating(mut self, rating: u8) -> Self {
        self.quality_rating = Some(rating.min(5));
        self
    }
}

/// Approval decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    /// Approver ID
    pub approver: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Approved or rejected
    pub approved: bool,
    /// Reason
    pub reason: String,
}

impl Approval {
    /// Creates a new approval.
    pub fn new(approver: impl Into<String>, approved: bool, reason: impl Into<String>) -> Self {
        Self {
            approver: approver.into(),
            timestamp: Utc::now(),
            approved,
            reason: reason.into(),
        }
    }
}

/// Data stewardship assignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSteward {
    /// Steward ID
    pub id: String,
    /// Name
    pub name: String,
    /// Email
    pub email: String,
    /// Areas of responsibility (URI patterns)
    pub areas: Vec<String>,
    /// Active status
    pub active: bool,
}

impl DataSteward {
    /// Creates a new data steward.
    pub fn new(id: impl Into<String>, name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
            areas: Vec::new(),
            active: true,
        }
    }

    /// Adds an area of responsibility.
    pub fn add_area(&mut self, area: impl Into<String>) {
        self.areas.push(area.into());
    }

    /// Checks if responsible for a resource.
    pub fn is_responsible_for(&self, resource_uri: &str) -> bool {
        self.areas.iter().any(|area| {
            if area.contains('*') {
                let pattern = area.replace('*', "");
                resource_uri.contains(&pattern)
            } else {
                resource_uri.starts_with(area)
            }
        })
    }
}

/// Manager for governance workflows.
pub struct GovernanceManager {
    /// All workflows
    workflows: HashMap<String, GovernanceWorkflow>,
    /// Data stewards
    stewards: HashMap<String, DataSteward>,
}

impl GovernanceManager {
    /// Creates a new governance manager.
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
            stewards: HashMap::new(),
        }
    }

    /// Adds a workflow.
    pub fn add_workflow(&mut self, workflow: GovernanceWorkflow) -> Result<(), String> {
        if self.workflows.contains_key(&workflow.id) {
            return Err(format!("Workflow {} already exists", workflow.id));
        }
        self.workflows.insert(workflow.id.clone(), workflow);
        Ok(())
    }

    /// Gets a workflow.
    pub fn get_workflow(&self, id: &str) -> Option<&GovernanceWorkflow> {
        self.workflows.get(id)
    }

    /// Gets a mutable workflow.
    pub fn get_workflow_mut(&mut self, id: &str) -> Option<&mut GovernanceWorkflow> {
        self.workflows.get_mut(id)
    }

    /// Lists workflows by state.
    pub fn list_by_state(&self, state: WorkflowState) -> Vec<&GovernanceWorkflow> {
        self.workflows
            .values()
            .filter(|w| w.state == state)
            .collect()
    }

    /// Adds a data steward.
    pub fn add_steward(&mut self, steward: DataSteward) -> Result<(), String> {
        if self.stewards.contains_key(&steward.id) {
            return Err(format!("Steward {} already exists", steward.id));
        }
        self.stewards.insert(steward.id.clone(), steward);
        Ok(())
    }

    /// Finds steward responsible for a resource.
    pub fn find_steward_for(&self, resource_uri: &str) -> Option<&DataSteward> {
        self.stewards
            .values()
            .find(|s| s.active && s.is_responsible_for(resource_uri))
    }
}

impl Default for GovernanceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RdfValue;

    fn sample_triples() -> Vec<Triple> {
        vec![Triple {
            subject: "http://example.org/Entity1".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Class".to_string()),
        }]
    }

    #[test]
    fn test_workflow_creation() {
        let workflow = GovernanceWorkflow::new("wf1", "Test Workflow", "user1", sample_triples());
        assert_eq!(workflow.id, "wf1");
        assert_eq!(workflow.state, WorkflowState::Draft);
        assert_eq!(workflow.submitter, "user1");
    }

    #[test]
    fn test_workflow_submission() {
        let mut workflow =
            GovernanceWorkflow::new("wf1", "Test Workflow", "user1", sample_triples());
        workflow.assign_reviewer("reviewer1");

        assert!(workflow.submit_for_review().is_ok());
        assert_eq!(workflow.state, WorkflowState::PendingReview);
    }

    #[test]
    fn test_workflow_review() {
        let mut workflow =
            GovernanceWorkflow::new("wf1", "Test Workflow", "user1", sample_triples());
        workflow.assign_reviewer("reviewer1");
        workflow.submit_for_review().unwrap();

        let review = Review::new("reviewer1", ReviewDecision::Approve, "Looks good").with_rating(5);

        assert!(workflow.add_review(review).is_ok());
        assert_eq!(workflow.state, WorkflowState::InReview);
    }

    #[test]
    fn test_workflow_approval() {
        let mut workflow =
            GovernanceWorkflow::new("wf1", "Test Workflow", "user1", sample_triples());
        workflow.assign_reviewer("reviewer1");
        workflow.assign_approver("approver1");
        workflow.submit_for_review().unwrap();

        let review = Review::new("reviewer1", ReviewDecision::Approve, "Good");
        workflow.add_review(review).unwrap();

        let approval = Approval::new("approver1", true, "Approved");
        workflow.add_approval(approval).unwrap();

        assert_eq!(workflow.state, WorkflowState::Approved);
    }

    #[test]
    fn test_workflow_rejection() {
        let mut workflow =
            GovernanceWorkflow::new("wf1", "Test Workflow", "user1", sample_triples());
        workflow.assign_reviewer("reviewer1");
        workflow.assign_approver("approver1");
        workflow.submit_for_review().unwrap();

        let review = Review::new("reviewer1", ReviewDecision::Reject, "Issues found");
        workflow.add_review(review).unwrap();

        let approval = Approval::new("approver1", false, "Rejected due to issues");
        workflow.add_approval(approval).unwrap();

        assert_eq!(workflow.state, WorkflowState::Rejected);
    }

    #[test]
    fn test_workflow_publish() {
        let mut workflow =
            GovernanceWorkflow::new("wf1", "Test Workflow", "user1", sample_triples());
        workflow.state = WorkflowState::Approved;

        assert!(workflow.publish().is_ok());
        assert_eq!(workflow.state, WorkflowState::Published);
    }

    #[test]
    fn test_data_steward() {
        let mut steward = DataSteward::new("steward1", "John Doe", "john@example.com");
        steward.add_area("http://example.org/legal/*");

        assert!(steward.is_responsible_for("http://example.org/legal/statute1"));
        assert!(!steward.is_responsible_for("http://example.org/finance/data1"));
    }

    #[test]
    fn test_governance_manager() {
        let mut manager = GovernanceManager::new();

        let workflow = GovernanceWorkflow::new("wf1", "Test", "user1", sample_triples());
        assert!(manager.add_workflow(workflow).is_ok());

        assert!(manager.get_workflow("wf1").is_some());

        // Try to add duplicate
        let workflow2 = GovernanceWorkflow::new("wf1", "Duplicate", "user2", sample_triples());
        assert!(manager.add_workflow(workflow2).is_err());
    }

    #[test]
    fn test_list_by_state() {
        let mut manager = GovernanceManager::new();

        let mut wf1 = GovernanceWorkflow::new("wf1", "Test1", "user1", sample_triples());
        wf1.state = WorkflowState::Draft;
        manager.add_workflow(wf1).unwrap();

        let mut wf2 = GovernanceWorkflow::new("wf2", "Test2", "user2", sample_triples());
        wf2.state = WorkflowState::Approved;
        manager.add_workflow(wf2).unwrap();

        let draft_workflows = manager.list_by_state(WorkflowState::Draft);
        assert_eq!(draft_workflows.len(), 1);

        let approved_workflows = manager.list_by_state(WorkflowState::Approved);
        assert_eq!(approved_workflows.len(), 1);
    }

    #[test]
    fn test_find_steward_for_resource() {
        let mut manager = GovernanceManager::new();

        let mut steward = DataSteward::new("steward1", "John", "john@example.com");
        steward.add_area("http://example.org/legal/*");
        manager.add_steward(steward).unwrap();

        let found = manager.find_steward_for("http://example.org/legal/statute1");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "steward1");

        let not_found = manager.find_steward_for("http://example.org/other/data");
        assert!(not_found.is_none());
    }
}
