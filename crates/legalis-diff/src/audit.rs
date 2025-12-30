//! Audit trail for tracking changes, attribution, and lifecycle management.
//!
//! This module provides functionality for:
//! - Change attribution (who changed what)
//! - Change justification tracking
//! - Approval workflow integration
//! - Change lifecycle tracking (proposed → approved → enacted)
//! - Rollback planning from diffs

use crate::{DiffError, DiffResult, StatuteDiff};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the lifecycle state of a change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeLifecycle {
    /// Change has been proposed but not yet reviewed
    Proposed,
    /// Change is under review
    UnderReview,
    /// Change has been approved but not yet enacted
    Approved,
    /// Change has been enacted and is active
    Enacted,
    /// Change was rejected
    Rejected,
    /// Change was rolled back
    RolledBack,
}

impl std::fmt::Display for ChangeLifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Proposed => write!(f, "Proposed"),
            Self::UnderReview => write!(f, "Under Review"),
            Self::Approved => write!(f, "Approved"),
            Self::Enacted => write!(f, "Enacted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::RolledBack => write!(f, "Rolled Back"),
        }
    }
}

/// Attribution information for a change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeAttribution {
    /// User who made the change
    pub author: String,
    /// Email of the author
    pub email: Option<String>,
    /// Timestamp when the change was made
    pub timestamp: DateTime<Utc>,
    /// System or tool that generated the change
    pub source_system: Option<String>,
}

impl ChangeAttribution {
    /// Creates a new change attribution.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::audit::ChangeAttribution;
    ///
    /// let attribution = ChangeAttribution::new("Jane Doe");
    /// assert_eq!(attribution.author, "Jane Doe");
    /// ```
    pub fn new(author: impl Into<String>) -> Self {
        Self {
            author: author.into(),
            email: None,
            timestamp: Utc::now(),
            source_system: None,
        }
    }

    /// Sets the email address.
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Sets the source system.
    pub fn with_source_system(mut self, system: impl Into<String>) -> Self {
        self.source_system = Some(system.into());
        self
    }

    /// Sets a custom timestamp.
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }
}

/// Justification for a change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeJustification {
    /// Reason for the change
    pub reason: String,
    /// Supporting documentation or references
    pub references: Vec<String>,
    /// Legal or policy basis
    pub legal_basis: Option<String>,
    /// Impact assessment notes
    pub impact_notes: Vec<String>,
}

impl ChangeJustification {
    /// Creates a new change justification.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::audit::ChangeJustification;
    ///
    /// let justification = ChangeJustification::new("Update age requirement to align with federal law");
    /// assert!(justification.reason.contains("federal law"));
    /// ```
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            references: Vec::new(),
            legal_basis: None,
            impact_notes: Vec::new(),
        }
    }

    /// Adds a reference document.
    pub fn add_reference(mut self, reference: impl Into<String>) -> Self {
        self.references.push(reference.into());
        self
    }

    /// Sets the legal basis.
    pub fn with_legal_basis(mut self, basis: impl Into<String>) -> Self {
        self.legal_basis = Some(basis.into());
        self
    }

    /// Adds an impact note.
    pub fn add_impact_note(mut self, note: impl Into<String>) -> Self {
        self.impact_notes.push(note.into());
        self
    }
}

/// Approval information for a change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeApproval {
    /// Person who approved the change
    pub approver: String,
    /// Approval timestamp
    pub approval_timestamp: DateTime<Utc>,
    /// Approval comments
    pub comments: Option<String>,
    /// Conditions or requirements for the approval
    pub conditions: Vec<String>,
}

impl ChangeApproval {
    /// Creates a new change approval.
    pub fn new(approver: impl Into<String>) -> Self {
        Self {
            approver: approver.into(),
            approval_timestamp: Utc::now(),
            comments: None,
            conditions: Vec::new(),
        }
    }

    /// Sets approval comments.
    pub fn with_comments(mut self, comments: impl Into<String>) -> Self {
        self.comments = Some(comments.into());
        self
    }

    /// Adds a condition.
    pub fn add_condition(mut self, condition: impl Into<String>) -> Self {
        self.conditions.push(condition.into());
        self
    }

    /// Sets a custom approval timestamp.
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.approval_timestamp = timestamp;
        self
    }
}

/// Complete audit trail for a statute diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrail {
    /// The diff being tracked
    pub diff: StatuteDiff,
    /// Change attribution
    pub attribution: ChangeAttribution,
    /// Justification for the change
    pub justification: Option<ChangeJustification>,
    /// Current lifecycle state
    pub lifecycle: ChangeLifecycle,
    /// Approval information (if approved)
    pub approval: Option<ChangeApproval>,
    /// Lifecycle history
    pub lifecycle_history: Vec<LifecycleTransition>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Represents a transition in the change lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleTransition {
    /// Previous state
    pub from_state: ChangeLifecycle,
    /// New state
    pub to_state: ChangeLifecycle,
    /// When the transition occurred
    pub timestamp: DateTime<Utc>,
    /// Who triggered the transition
    pub triggered_by: String,
    /// Notes about the transition
    pub notes: Option<String>,
}

impl AuditTrail {
    /// Creates a new audit trail for a diff.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, audit::{AuditTrail, ChangeAttribution}};
    ///
    /// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let attribution = ChangeAttribution::new("John Smith");
    /// let audit = AuditTrail::new(diff_result, attribution);
    ///
    /// assert_eq!(audit.attribution.author, "John Smith");
    /// ```
    pub fn new(diff: StatuteDiff, attribution: ChangeAttribution) -> Self {
        Self {
            diff,
            attribution,
            justification: None,
            lifecycle: ChangeLifecycle::Proposed,
            approval: None,
            lifecycle_history: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Sets the justification for the change.
    pub fn with_justification(mut self, justification: ChangeJustification) -> Self {
        self.justification = Some(justification);
        self
    }

    /// Adds metadata.
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Transitions to a new lifecycle state.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, audit::{AuditTrail, ChangeAttribution, ChangeLifecycle}};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let attribution = ChangeAttribution::new("John Smith");
    /// let mut audit = AuditTrail::new(diff_result, attribution);
    ///
    /// audit.transition_to(ChangeLifecycle::UnderReview, "Reviewer", None);
    /// assert_eq!(audit.lifecycle, ChangeLifecycle::UnderReview);
    /// assert_eq!(audit.lifecycle_history.len(), 1);
    /// ```
    pub fn transition_to(
        &mut self,
        new_state: ChangeLifecycle,
        triggered_by: impl Into<String>,
        notes: Option<String>,
    ) {
        let transition = LifecycleTransition {
            from_state: self.lifecycle,
            to_state: new_state,
            timestamp: Utc::now(),
            triggered_by: triggered_by.into(),
            notes,
        };

        self.lifecycle_history.push(transition);
        self.lifecycle = new_state;
    }

    /// Approves the change.
    ///
    /// # Errors
    ///
    /// Returns an error if the change is not in a reviewable state.
    pub fn approve(&mut self, approval: ChangeApproval) -> DiffResult<()> {
        if self.lifecycle != ChangeLifecycle::Proposed
            && self.lifecycle != ChangeLifecycle::UnderReview
        {
            return Err(DiffError::InvalidComparison(format!(
                "Cannot approve change in state: {}",
                self.lifecycle
            )));
        }

        self.approval = Some(approval.clone());
        self.transition_to(
            ChangeLifecycle::Approved,
            approval.approver,
            Some("Change approved".to_string()),
        );

        Ok(())
    }

    /// Rejects the change.
    ///
    /// # Errors
    ///
    /// Returns an error if the change is not in a reviewable state.
    pub fn reject(&mut self, reviewer: impl Into<String>, reason: String) -> DiffResult<()> {
        if self.lifecycle != ChangeLifecycle::Proposed
            && self.lifecycle != ChangeLifecycle::UnderReview
        {
            return Err(DiffError::InvalidComparison(format!(
                "Cannot reject change in state: {}",
                self.lifecycle
            )));
        }

        self.transition_to(ChangeLifecycle::Rejected, reviewer, Some(reason));

        Ok(())
    }

    /// Enacts the change.
    ///
    /// # Errors
    ///
    /// Returns an error if the change has not been approved.
    pub fn enact(&mut self, enactor: impl Into<String>) -> DiffResult<()> {
        if self.lifecycle != ChangeLifecycle::Approved {
            return Err(DiffError::InvalidComparison(format!(
                "Cannot enact change in state: {}. Must be approved first.",
                self.lifecycle
            )));
        }

        self.transition_to(
            ChangeLifecycle::Enacted,
            enactor,
            Some("Change enacted".to_string()),
        );

        Ok(())
    }

    /// Generates a rollback plan for this change.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, audit::{AuditTrail, ChangeAttribution}};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let attribution = ChangeAttribution::new("John Smith");
    /// let audit = AuditTrail::new(diff_result, attribution);
    ///
    /// let rollback_plan = audit.generate_rollback_plan();
    /// assert!(rollback_plan.contains("Rollback Plan"));
    /// ```
    pub fn generate_rollback_plan(&self) -> String {
        let mut plan = String::new();
        plan.push_str("=== Rollback Plan ===\n\n");
        plan.push_str(&format!("Statute ID: {}\n", self.diff.statute_id));
        plan.push_str(&format!("Current State: {}\n", self.lifecycle));
        plan.push_str(&format!("Author: {}\n", self.attribution.author));
        plan.push_str(&format!("Changes: {}\n\n", self.diff.changes.len()));

        plan.push_str("Steps to rollback:\n");
        for (i, change) in self.diff.changes.iter().enumerate() {
            plan.push_str(&format!(
                "{}. Revert {:?} to {:?}: {}\n",
                i + 1,
                change.target,
                change.change_type,
                change.description
            ));
            if let Some(old_val) = &change.old_value {
                plan.push_str(&format!("   Restore value: {}\n", old_val));
            }
        }

        if let Some(justification) = &self.justification {
            plan.push_str("\nOriginal Justification:\n");
            plan.push_str(&format!("  {}\n", justification.reason));
        }

        plan.push_str("\nImpact of Rollback:\n");
        plan.push_str(&format!("  - Severity: {:?}\n", self.diff.impact.severity));
        plan.push_str(&format!(
            "  - Affects Eligibility: {}\n",
            self.diff.impact.affects_eligibility
        ));
        plan.push_str(&format!(
            "  - Affects Outcome: {}\n",
            self.diff.impact.affects_outcome
        ));

        plan
    }

    /// Gets the complete audit history as a formatted string.
    pub fn get_audit_history(&self) -> String {
        let mut history = String::new();
        history.push_str("=== Audit History ===\n\n");

        history.push_str(&format!("Statute: {}\n", self.diff.statute_id));
        history.push_str(&format!("Author: {}\n", self.attribution.author));
        if let Some(email) = &self.attribution.email {
            history.push_str(&format!("Email: {}\n", email));
        }
        history.push_str(&format!(
            "Created: {}\n",
            self.attribution.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        history.push_str(&format!("Current State: {}\n\n", self.lifecycle));

        if !self.lifecycle_history.is_empty() {
            history.push_str("Lifecycle Transitions:\n");
            for transition in &self.lifecycle_history {
                history.push_str(&format!(
                    "  {} -> {} at {} by {}\n",
                    transition.from_state,
                    transition.to_state,
                    transition.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                    transition.triggered_by
                ));
                if let Some(notes) = &transition.notes {
                    history.push_str(&format!("    Notes: {}\n", notes));
                }
            }
            history.push('\n');
        }

        if let Some(justification) = &self.justification {
            history.push_str("Justification:\n");
            history.push_str(&format!("  Reason: {}\n", justification.reason));
            if let Some(legal_basis) = &justification.legal_basis {
                history.push_str(&format!("  Legal Basis: {}\n", legal_basis));
            }
            if !justification.references.is_empty() {
                history.push_str("  References:\n");
                for reference in &justification.references {
                    history.push_str(&format!("    - {}\n", reference));
                }
            }
            history.push('\n');
        }

        if let Some(approval) = &self.approval {
            history.push_str("Approval:\n");
            history.push_str(&format!("  Approver: {}\n", approval.approver));
            history.push_str(&format!(
                "  Approved: {}\n",
                approval.approval_timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ));
            if let Some(comments) = &approval.comments {
                history.push_str(&format!("  Comments: {}\n", comments));
            }
            if !approval.conditions.is_empty() {
                history.push_str("  Conditions:\n");
                for condition in &approval.conditions {
                    history.push_str(&format!("    - {}\n", condition));
                }
            }
        }

        history
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn test_diff() -> StatuteDiff {
        let old = Statute::new(
            "test-law",
            "Old Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let new = Statute::new(
            "test-law",
            "New Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        diff(&old, &new).unwrap()
    }

    #[test]
    fn test_change_attribution() {
        let attr = ChangeAttribution::new("John Doe")
            .with_email("john@example.com")
            .with_source_system("LegalisSystem");

        assert_eq!(attr.author, "John Doe");
        assert_eq!(attr.email, Some("john@example.com".to_string()));
        assert_eq!(attr.source_system, Some("LegalisSystem".to_string()));
    }

    #[test]
    fn test_change_justification() {
        let just = ChangeJustification::new("Update for compliance")
            .add_reference("Regulation XYZ-123")
            .with_legal_basis("Federal Law 456")
            .add_impact_note("Affects 1000 beneficiaries");

        assert!(just.reason.contains("compliance"));
        assert_eq!(just.references.len(), 1);
        assert!(just.legal_basis.is_some());
        assert_eq!(just.impact_notes.len(), 1);
    }

    #[test]
    fn test_audit_trail_creation() {
        let diff = test_diff();
        let attr = ChangeAttribution::new("Jane Smith");
        let audit = AuditTrail::new(diff, attr);

        assert_eq!(audit.lifecycle, ChangeLifecycle::Proposed);
        assert_eq!(audit.attribution.author, "Jane Smith");
        assert!(audit.lifecycle_history.is_empty());
    }

    #[test]
    fn test_lifecycle_transition() {
        let diff = test_diff();
        let attr = ChangeAttribution::new("Jane Smith");
        let mut audit = AuditTrail::new(diff, attr);

        audit.transition_to(
            ChangeLifecycle::UnderReview,
            "Reviewer",
            Some("Starting review".to_string()),
        );

        assert_eq!(audit.lifecycle, ChangeLifecycle::UnderReview);
        assert_eq!(audit.lifecycle_history.len(), 1);
        assert_eq!(
            audit.lifecycle_history[0].from_state,
            ChangeLifecycle::Proposed
        );
        assert_eq!(
            audit.lifecycle_history[0].to_state,
            ChangeLifecycle::UnderReview
        );
    }

    #[test]
    fn test_approval_workflow() {
        let diff = test_diff();
        let attr = ChangeAttribution::new("Jane Smith");
        let mut audit = AuditTrail::new(diff, attr);

        let approval = ChangeApproval::new("Approver")
            .with_comments("Looks good")
            .add_condition("Must be reviewed quarterly");

        let result = audit.approve(approval);
        assert!(result.is_ok());
        assert_eq!(audit.lifecycle, ChangeLifecycle::Approved);
        assert!(audit.approval.is_some());
    }

    #[test]
    fn test_rejection_workflow() {
        let diff = test_diff();
        let attr = ChangeAttribution::new("Jane Smith");
        let mut audit = AuditTrail::new(diff, attr);

        let result = audit.reject("Reviewer", "Needs more documentation".to_string());
        assert!(result.is_ok());
        assert_eq!(audit.lifecycle, ChangeLifecycle::Rejected);
    }

    #[test]
    fn test_enactment_requires_approval() {
        let diff = test_diff();
        let attr = ChangeAttribution::new("Jane Smith");
        let mut audit = AuditTrail::new(diff, attr);

        // Try to enact without approval
        let result = audit.enact("Enactor");
        assert!(result.is_err());

        // Approve first
        let approval = ChangeApproval::new("Approver");
        audit.approve(approval).unwrap();

        // Now enactment should work
        let result = audit.enact("Enactor");
        assert!(result.is_ok());
        assert_eq!(audit.lifecycle, ChangeLifecycle::Enacted);
    }

    #[test]
    fn test_rollback_plan_generation() {
        let diff = test_diff();
        let attr = ChangeAttribution::new("Jane Smith");
        let audit = AuditTrail::new(diff, attr);

        let plan = audit.generate_rollback_plan();
        assert!(plan.contains("Rollback Plan"));
        assert!(plan.contains("test-law"));
        assert!(plan.contains("Jane Smith"));
    }

    #[test]
    fn test_audit_history() {
        let diff = test_diff();
        let attr = ChangeAttribution::new("Jane Smith").with_email("jane@example.com");
        let mut audit = AuditTrail::new(diff, attr);

        audit.transition_to(ChangeLifecycle::UnderReview, "Reviewer", None);

        let history = audit.get_audit_history();
        assert!(history.contains("Audit History"));
        assert!(history.contains("Jane Smith"));
        assert!(history.contains("jane@example.com"));
        assert!(history.contains("Under Review"));
    }

    #[test]
    fn test_metadata() {
        let diff = test_diff();
        let attr = ChangeAttribution::new("Jane Smith");
        let audit = AuditTrail::new(diff, attr)
            .add_metadata("ticket_id", "TICKET-123")
            .add_metadata("priority", "high");

        assert_eq!(
            audit.metadata.get("ticket_id"),
            Some(&"TICKET-123".to_string())
        );
        assert_eq!(audit.metadata.get("priority"), Some(&"high".to_string()));
    }
}
