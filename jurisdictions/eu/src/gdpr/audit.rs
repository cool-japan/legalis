//! GDPR Audit Trail Integration (Articles 15 and 22)
//!
//! This module provides GDPR-compliant audit trail capabilities for data processing decisions,
//! integrating with legalis-audit for:
//!
//! - **Article 15**: Data Subject Access Request (DSAR) support
//! - **Article 22**: Right to explanation for automated decision-making
//! - **Article 30**: Records of Processing Activities (ROPA) audit support
//! - **Article 5(2)**: Accountability principle demonstration
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_eu::gdpr::audit::{GdprAuditTrail, GdprDecisionRecord};
//! use legalis_eu::gdpr::{LawfulBasis, PersonalDataCategory};
//!
//! // Create a GDPR-compliant audit trail
//! let mut audit = GdprAuditTrail::new();
//!
//! // Record a data processing decision
//! let record = GdprDecisionRecord::new()
//!     .with_statute("GDPR_Art6")
//!     .with_lawful_basis(LawfulBasis::Consent {
//!         freely_given: true,
//!         specific: true,
//!         informed: true,
//!         unambiguous: true,
//!     })
//!     .with_purpose("Marketing email processing")
//!     .with_data_subject(data_subject_id);
//!
//! audit.record_decision(record)?;
//!
//! // Handle a Data Subject Access Request (Article 15)
//! let dsar_report = audit.handle_dsar(data_subject_id)?;
//!
//! // Generate explanation for automated decision (Article 22)
//! let explanation = audit.explain_decision(decision_id)?;
//! ```

use chrono::{DateTime, Utc};
use legalis_audit::{
    Actor, AuditRecord, AuditResult, AuditTrail, DecisionContext, DecisionResult, EventType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::types::{DataSubjectRight, LawfulBasis};

/// GDPR-compliant audit trail wrapper.
///
/// Provides GDPR-specific audit functionality on top of legalis-audit,
/// ensuring compliance with Articles 15 (access), 22 (explanation),
/// and 5(2) (accountability).
pub struct GdprAuditTrail {
    /// Underlying audit trail
    inner: AuditTrail,
    /// Controller identification
    controller_id: String,
    /// DPO contact information
    dpo_contact: Option<String>,
}

impl std::fmt::Debug for GdprAuditTrail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GdprAuditTrail")
            .field("controller_id", &self.controller_id)
            .field("dpo_contact", &self.dpo_contact)
            .finish_non_exhaustive()
    }
}

impl Default for GdprAuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

impl GdprAuditTrail {
    /// Creates a new GDPR audit trail with in-memory storage.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: AuditTrail::new(),
            controller_id: String::new(),
            dpo_contact: None,
        }
    }

    /// Creates a GDPR audit trail with JSONL file storage.
    ///
    /// # Errors
    /// Returns error if file cannot be created or accessed.
    pub fn with_jsonl_file<P: AsRef<std::path::Path>>(path: P) -> AuditResult<Self> {
        Ok(Self {
            inner: AuditTrail::with_jsonl_file(path)?,
            controller_id: String::new(),
            dpo_contact: None,
        })
    }

    /// Creates a GDPR audit trail with SQLite storage.
    ///
    /// # Errors
    /// Returns error if database cannot be created or accessed.
    pub fn with_sqlite<P: AsRef<std::path::Path>>(path: P) -> AuditResult<Self> {
        Ok(Self {
            inner: AuditTrail::with_sqlite(path)?,
            controller_id: String::new(),
            dpo_contact: None,
        })
    }

    /// Sets the data controller identification.
    #[must_use]
    pub fn with_controller(mut self, controller_id: impl Into<String>) -> Self {
        self.controller_id = controller_id.into();
        self
    }

    /// Sets the DPO contact information.
    #[must_use]
    pub fn with_dpo_contact(mut self, contact: impl Into<String>) -> Self {
        self.dpo_contact = Some(contact.into());
        self
    }

    /// Records a GDPR processing decision.
    ///
    /// # Errors
    /// Returns error if record cannot be stored.
    pub fn record_decision(&mut self, decision: GdprDecisionRecord) -> AuditResult<Uuid> {
        let record = decision.into_audit_record();
        self.inner.record(record)
    }

    /// Handles a Data Subject Access Request (GDPR Article 15).
    ///
    /// Returns all decisions and processing records related to the data subject,
    /// formatted for DSAR response within the 30-day deadline.
    ///
    /// # Errors
    /// Returns error if records cannot be retrieved.
    pub fn handle_dsar(&self, data_subject_id: Uuid) -> AuditResult<DsarResponse> {
        let export = self.inner.export_subject_data(data_subject_id)?;

        let records = self.inner.query_by_subject(data_subject_id)?;
        let processing_activities: Vec<ProcessingActivitySummary> = records
            .iter()
            .map(|r| ProcessingActivitySummary {
                decision_id: r.id,
                timestamp: r.timestamp,
                purpose: r.context.metadata.get("purpose").cloned(),
                lawful_basis: r.context.metadata.get("lawful_basis").cloned(),
                data_categories: r
                    .context
                    .metadata
                    .get("data_categories")
                    .cloned()
                    .unwrap_or_default(),
                recipients: r.context.metadata.get("recipients").cloned(),
                retention_period: r.context.metadata.get("retention_period").cloned(),
            })
            .collect();

        Ok(DsarResponse {
            data_subject_id,
            controller_id: self.controller_id.clone(),
            dpo_contact: self.dpo_contact.clone(),
            generated_at: Utc::now(),
            total_records: export.total_records,
            processing_activities,
            rights_information: Self::data_subject_rights_info(),
            export_formats_available: vec![
                "JSON".to_string(),
                "CSV".to_string(),
                "PDF".to_string(),
            ],
        })
    }

    /// Generates an explanation for an automated decision (GDPR Article 22).
    ///
    /// Article 22(3) requires that data subjects have the right to obtain
    /// meaningful information about the logic involved in automated decision-making.
    ///
    /// # Errors
    /// Returns error if decision cannot be found.
    pub fn explain_decision(&self, decision_id: Uuid) -> AuditResult<Article22Explanation> {
        let explanation = self.inner.explain_decision(decision_id)?;
        let record = self.inner.get(decision_id)?;

        Ok(Article22Explanation {
            decision_id,
            timestamp: record.timestamp,
            base_explanation: explanation.explanation,
            logic_description: self.extract_logic_description(&record),
            significance: self.assess_decision_significance(&record),
            consequences: self.extract_consequences(&record),
            data_categories_used: self.extract_data_categories(&record),
            safeguards: self.list_article22_safeguards(),
            right_to_contest: true,
            human_review_available: matches!(
                record.result,
                DecisionResult::RequiresDiscretion { .. }
            ),
        })
    }

    /// Verifies audit trail integrity for accountability (Article 5(2)).
    ///
    /// # Errors
    /// Returns error if integrity check fails.
    pub fn verify_accountability(&self) -> AuditResult<AccountabilityReport> {
        let integrity_verified = self.inner.verify_integrity()?;
        let compliance_report = self.inner.generate_report()?;

        Ok(AccountabilityReport {
            integrity_verified,
            total_decisions: compliance_report.total_decisions,
            automatic_decisions: compliance_report.automatic_decisions,
            human_overrides: compliance_report.human_overrides,
            discretionary_decisions: compliance_report.discretionary_decisions,
            generated_at: compliance_report.generated_at,
            controller_id: self.controller_id.clone(),
            dpo_notified: self.dpo_contact.is_some(),
        })
    }

    /// Applies GDPR retention policy for data minimization (Article 5(1)(e)).
    ///
    /// Returns records that should be deleted based on the retention period.
    ///
    /// # Errors
    /// Returns error if records cannot be retrieved.
    pub fn apply_retention_policy(
        &self,
        retention_days: i64,
    ) -> AuditResult<Vec<RetentionCandidate>> {
        let policy = legalis_audit::retention::RetentionPolicy::new(retention_days);
        let to_delete = self.inner.apply_retention_policy(&policy)?;

        Ok(to_delete
            .into_iter()
            .map(|r| RetentionCandidate {
                record_id: r.id,
                timestamp: r.timestamp,
                statute_id: r.statute_id,
                reason: format!("Exceeds {} day retention period", retention_days),
            })
            .collect())
    }

    /// Exports audit trail in GDPR-compliant format.
    ///
    /// # Errors
    /// Returns error if export fails.
    pub fn export_for_regulator(&self) -> AuditResult<serde_json::Value> {
        self.inner.export_json()
    }

    fn data_subject_rights_info() -> Vec<DataSubjectRightInfo> {
        vec![
            DataSubjectRightInfo {
                right: DataSubjectRight::Access,
                article: "Article 15".to_string(),
                description: "Right to obtain confirmation and access to your personal data"
                    .to_string(),
            },
            DataSubjectRightInfo {
                right: DataSubjectRight::Rectification,
                article: "Article 16".to_string(),
                description: "Right to have inaccurate personal data rectified".to_string(),
            },
            DataSubjectRightInfo {
                right: DataSubjectRight::Erasure,
                article: "Article 17".to_string(),
                description: "Right to erasure ('right to be forgotten')".to_string(),
            },
            DataSubjectRightInfo {
                right: DataSubjectRight::RestrictionOfProcessing,
                article: "Article 18".to_string(),
                description: "Right to obtain restriction of processing".to_string(),
            },
            DataSubjectRightInfo {
                right: DataSubjectRight::DataPortability,
                article: "Article 20".to_string(),
                description: "Right to receive your data in a portable format".to_string(),
            },
            DataSubjectRightInfo {
                right: DataSubjectRight::Object,
                article: "Article 21".to_string(),
                description: "Right to object to processing".to_string(),
            },
            DataSubjectRightInfo {
                right: DataSubjectRight::AutomatedDecisionMaking,
                article: "Article 22".to_string(),
                description:
                    "Right not to be subject to solely automated decision-making with legal effects"
                        .to_string(),
            },
        ]
    }

    fn extract_logic_description(&self, record: &AuditRecord) -> String {
        let mut logic = String::new();
        logic.push_str("Decision logic:\n");

        for condition in &record.context.evaluated_conditions {
            logic.push_str(&format!(
                "- {}: {} (input: {})\n",
                condition.description,
                if condition.result { "met" } else { "not met" },
                condition.input_value.as_deref().unwrap_or("N/A")
            ));
        }

        if logic == "Decision logic:\n" {
            logic.push_str("- Standard processing rules applied based on lawful basis\n");
        }

        logic
    }

    fn assess_decision_significance(&self, record: &AuditRecord) -> DecisionSignificance {
        match &record.result {
            DecisionResult::Deterministic { effect_applied, .. } => {
                if effect_applied.contains("reject")
                    || effect_applied.contains("deny")
                    || effect_applied.contains("revoke")
                {
                    DecisionSignificance::High {
                        reason: "Negative outcome affecting data subject rights".to_string(),
                    }
                } else {
                    DecisionSignificance::Normal
                }
            }
            DecisionResult::RequiresDiscretion { issue, .. } => DecisionSignificance::High {
                reason: format!("Requires human review: {}", issue),
            },
            DecisionResult::Void { reason } => DecisionSignificance::High {
                reason: format!("Decision voided: {}", reason),
            },
            DecisionResult::Overridden { justification, .. } => DecisionSignificance::High {
                reason: format!("Human override applied: {}", justification),
            },
        }
    }

    fn extract_consequences(&self, record: &AuditRecord) -> Vec<String> {
        let mut consequences = Vec::new();

        match &record.result {
            DecisionResult::Deterministic { effect_applied, .. } => {
                consequences.push(format!("Effect applied: {}", effect_applied));
            }
            DecisionResult::RequiresDiscretion { issue, .. } => {
                consequences.push(format!("Pending human review: {}", issue));
            }
            DecisionResult::Void { reason } => {
                consequences.push(format!("No effect due to: {}", reason));
            }
            DecisionResult::Overridden { justification, .. } => {
                consequences.push(format!("Original decision overridden: {}", justification));
            }
        }

        consequences
    }

    fn extract_data_categories(&self, record: &AuditRecord) -> Vec<String> {
        record
            .context
            .metadata
            .get("data_categories")
            .map(|s| s.split(',').map(|c| c.trim().to_string()).collect())
            .unwrap_or_default()
    }

    fn list_article22_safeguards(&self) -> Vec<String> {
        vec![
            "Right to obtain human intervention".to_string(),
            "Right to express your point of view".to_string(),
            "Right to contest the decision".to_string(),
            "Regular testing for bias and fairness".to_string(),
            "Technical and organizational measures to minimize errors".to_string(),
        ]
    }
}

/// GDPR-specific decision record for audit trail.
#[derive(Debug, Clone, Default)]
pub struct GdprDecisionRecord {
    /// Statute ID (e.g., "GDPR_Art6")
    pub statute_id: String,
    /// Data subject UUID
    pub data_subject_id: Uuid,
    /// Lawful basis for processing
    pub lawful_basis: Option<LawfulBasisMetadata>,
    /// Purpose of processing
    pub purpose: String,
    /// Data categories processed
    pub data_categories: Vec<String>,
    /// Recipients of data
    pub recipients: Vec<String>,
    /// Retention period
    pub retention_period: Option<String>,
    /// Processing operations performed
    pub operations: Vec<String>,
    /// Decision result
    pub effect: String,
    /// Actor who triggered the decision
    pub actor: Option<GdprActor>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Lawful basis metadata for audit records.
#[derive(Debug, Clone)]
pub struct LawfulBasisMetadata {
    /// Type of lawful basis
    pub basis_type: String,
    /// Additional details
    pub details: String,
}

/// GDPR-aware actor types.
#[derive(Debug, Clone)]
pub enum GdprActor {
    /// Data controller system
    Controller { component: String },
    /// Data processor system
    Processor { processor_id: String },
    /// Human user (e.g., DPO, administrator)
    User { user_id: String, role: String },
    /// Data subject (self-service)
    DataSubject { subject_id: Uuid },
}

impl GdprDecisionRecord {
    /// Creates a new GDPR decision record.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the statute ID.
    #[must_use]
    pub fn with_statute(mut self, statute_id: impl Into<String>) -> Self {
        self.statute_id = statute_id.into();
        self
    }

    /// Sets the data subject.
    #[must_use]
    pub fn with_data_subject(mut self, subject_id: Uuid) -> Self {
        self.data_subject_id = subject_id;
        self
    }

    /// Sets the lawful basis.
    #[must_use]
    pub fn with_lawful_basis(mut self, basis: &LawfulBasis) -> Self {
        self.lawful_basis = Some(LawfulBasisMetadata::from_lawful_basis(basis));
        self
    }

    /// Sets the processing purpose.
    #[must_use]
    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = purpose.into();
        self
    }

    /// Adds a data category.
    #[must_use]
    pub fn add_data_category(mut self, category: impl Into<String>) -> Self {
        self.data_categories.push(category.into());
        self
    }

    /// Adds a recipient.
    #[must_use]
    pub fn add_recipient(mut self, recipient: impl Into<String>) -> Self {
        self.recipients.push(recipient.into());
        self
    }

    /// Sets the retention period.
    #[must_use]
    pub fn with_retention_period(mut self, period: impl Into<String>) -> Self {
        self.retention_period = Some(period.into());
        self
    }

    /// Sets the decision effect.
    #[must_use]
    pub fn with_effect(mut self, effect: impl Into<String>) -> Self {
        self.effect = effect.into();
        self
    }

    /// Sets the actor.
    #[must_use]
    pub fn with_actor(mut self, actor: GdprActor) -> Self {
        self.actor = Some(actor);
        self
    }

    /// Converts to a legalis-audit AuditRecord.
    fn into_audit_record(self) -> AuditRecord {
        let actor = match self.actor {
            Some(GdprActor::Controller { component }) => Actor::System { component },
            Some(GdprActor::Processor { processor_id }) => Actor::External {
                system_id: processor_id,
            },
            Some(GdprActor::User { user_id, role }) => Actor::User { user_id, role },
            Some(GdprActor::DataSubject { subject_id }) => Actor::User {
                user_id: subject_id.to_string(),
                role: "data_subject".to_string(),
            },
            None => Actor::System {
                component: "gdpr_engine".to_string(),
            },
        };

        let mut context = DecisionContext::default();
        context
            .metadata
            .insert("purpose".to_string(), self.purpose.clone());
        if let Some(basis) = &self.lawful_basis {
            context
                .metadata
                .insert("lawful_basis".to_string(), basis.basis_type.clone());
            context
                .metadata
                .insert("lawful_basis_details".to_string(), basis.details.clone());
        }
        if !self.data_categories.is_empty() {
            context.metadata.insert(
                "data_categories".to_string(),
                self.data_categories.join(","),
            );
        }
        if !self.recipients.is_empty() {
            context
                .metadata
                .insert("recipients".to_string(), self.recipients.join(","));
        }
        if let Some(retention) = &self.retention_period {
            context
                .metadata
                .insert("retention_period".to_string(), retention.clone());
        }
        for (key, value) in self.metadata {
            context.metadata.insert(key, value);
        }

        let result = DecisionResult::Deterministic {
            effect_applied: self.effect,
            parameters: HashMap::new(),
        };

        AuditRecord::new(
            EventType::AutomaticDecision,
            actor,
            self.statute_id,
            self.data_subject_id,
            context,
            result,
            None,
        )
    }
}

impl LawfulBasisMetadata {
    fn from_lawful_basis(basis: &LawfulBasis) -> Self {
        match basis {
            LawfulBasis::Consent {
                freely_given,
                specific,
                informed,
                unambiguous,
            } => Self {
                basis_type: "Consent (Art. 6(1)(a))".to_string(),
                details: format!(
                    "freely_given={}, specific={}, informed={}, unambiguous={}",
                    freely_given, specific, informed, unambiguous
                ),
            },
            LawfulBasis::Contract {
                necessary_for_performance,
            } => Self {
                basis_type: "Contract (Art. 6(1)(b))".to_string(),
                details: format!("necessary_for_performance={}", necessary_for_performance),
            },
            LawfulBasis::LegalObligation {
                eu_law,
                member_state_law,
            } => Self {
                basis_type: "Legal Obligation (Art. 6(1)(c))".to_string(),
                details: format!(
                    "eu_law={:?}, member_state_law={:?}",
                    eu_law, member_state_law
                ),
            },
            LawfulBasis::VitalInterests { life_threatening } => Self {
                basis_type: "Vital Interests (Art. 6(1)(d))".to_string(),
                details: format!("life_threatening={}", life_threatening),
            },
            LawfulBasis::PublicTask { task_basis } => Self {
                basis_type: "Public Task (Art. 6(1)(e))".to_string(),
                details: task_basis.clone(),
            },
            LawfulBasis::LegitimateInterests {
                controller_interest,
                balancing_test_passed,
            } => Self {
                basis_type: "Legitimate Interests (Art. 6(1)(f))".to_string(),
                details: format!(
                    "interest={}, balancing_test_passed={}",
                    controller_interest, balancing_test_passed
                ),
            },
        }
    }
}

/// Data Subject Access Request response (Article 15).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DsarResponse {
    /// Data subject ID
    pub data_subject_id: Uuid,
    /// Controller identification
    pub controller_id: String,
    /// DPO contact information
    pub dpo_contact: Option<String>,
    /// When the response was generated
    pub generated_at: DateTime<Utc>,
    /// Total number of records found
    pub total_records: usize,
    /// Processing activities involving this data subject
    pub processing_activities: Vec<ProcessingActivitySummary>,
    /// Information about data subject rights
    pub rights_information: Vec<DataSubjectRightInfo>,
    /// Available export formats
    pub export_formats_available: Vec<String>,
}

/// Summary of a processing activity for DSAR.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProcessingActivitySummary {
    /// Decision ID
    pub decision_id: Uuid,
    /// When the processing occurred
    pub timestamp: DateTime<Utc>,
    /// Purpose of processing
    pub purpose: Option<String>,
    /// Lawful basis
    pub lawful_basis: Option<String>,
    /// Categories of data processed
    pub data_categories: String,
    /// Recipients of data
    pub recipients: Option<String>,
    /// Retention period
    pub retention_period: Option<String>,
}

/// Information about a data subject right.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DataSubjectRightInfo {
    /// The right
    pub right: DataSubjectRight,
    /// GDPR article reference
    pub article: String,
    /// Description of the right
    pub description: String,
}

/// Article 22 explanation for automated decision-making.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Article22Explanation {
    /// Decision ID
    pub decision_id: Uuid,
    /// When the decision was made
    pub timestamp: DateTime<Utc>,
    /// Base explanation from audit system
    pub base_explanation: String,
    /// Description of the logic involved
    pub logic_description: String,
    /// Significance of the decision
    pub significance: DecisionSignificance,
    /// Envisaged consequences
    pub consequences: Vec<String>,
    /// Data categories used in the decision
    pub data_categories_used: Vec<String>,
    /// Safeguards in place
    pub safeguards: Vec<String>,
    /// Whether the decision can be contested
    pub right_to_contest: bool,
    /// Whether human review is available
    pub human_review_available: bool,
}

/// Significance level of an automated decision.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DecisionSignificance {
    /// Normal processing decision
    Normal,
    /// High significance decision (legal effects or similar)
    High { reason: String },
}

/// Accountability report for Article 5(2).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccountabilityReport {
    /// Whether integrity is verified
    pub integrity_verified: bool,
    /// Total number of decisions
    pub total_decisions: usize,
    /// Automatic decisions
    pub automatic_decisions: usize,
    /// Human overrides
    pub human_overrides: usize,
    /// Discretionary decisions
    pub discretionary_decisions: usize,
    /// When the report was generated
    pub generated_at: DateTime<Utc>,
    /// Controller ID
    pub controller_id: String,
    /// Whether DPO has been notified
    pub dpo_notified: bool,
}

/// Record candidate for retention policy deletion.
#[derive(Debug, Clone)]
pub struct RetentionCandidate {
    /// Record ID
    pub record_id: Uuid,
    /// Record timestamp
    pub timestamp: DateTime<Utc>,
    /// Statute ID
    pub statute_id: String,
    /// Reason for deletion
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdpr_audit_trail_creation() {
        let audit = GdprAuditTrail::new()
            .with_controller("Test Controller")
            .with_dpo_contact("dpo@example.com");

        assert_eq!(audit.controller_id, "Test Controller");
        assert_eq!(audit.dpo_contact, Some("dpo@example.com".to_string()));
    }

    #[test]
    fn test_gdpr_decision_record_builder() {
        let subject_id = Uuid::new_v4();
        let record = GdprDecisionRecord::new()
            .with_statute("GDPR_Art6")
            .with_data_subject(subject_id)
            .with_purpose("Marketing")
            .add_data_category("email")
            .add_data_category("name")
            .with_effect("processing_approved");

        assert_eq!(record.statute_id, "GDPR_Art6");
        assert_eq!(record.data_subject_id, subject_id);
        assert_eq!(record.purpose, "Marketing");
        assert_eq!(record.data_categories.len(), 2);
        assert_eq!(record.effect, "processing_approved");
    }

    #[test]
    fn test_record_decision() {
        let mut audit = GdprAuditTrail::new();
        let subject_id = Uuid::new_v4();

        let record = GdprDecisionRecord::new()
            .with_statute("GDPR_Art6")
            .with_data_subject(subject_id)
            .with_lawful_basis(&LawfulBasis::Consent {
                freely_given: true,
                specific: true,
                informed: true,
                unambiguous: true,
            })
            .with_purpose("Test processing")
            .with_effect("approved");

        let result = audit.record_decision(record);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_dsar() {
        let mut audit = GdprAuditTrail::new()
            .with_controller("Test Controller")
            .with_dpo_contact("dpo@test.com");

        let subject_id = Uuid::new_v4();
        let record = GdprDecisionRecord::new()
            .with_statute("GDPR_Art6")
            .with_data_subject(subject_id)
            .with_purpose("DSAR Test")
            .with_effect("approved");

        audit.record_decision(record).expect("record failed");

        let dsar = audit.handle_dsar(subject_id).expect("DSAR failed");
        assert_eq!(dsar.data_subject_id, subject_id);
        assert_eq!(dsar.controller_id, "Test Controller");
        assert_eq!(dsar.total_records, 1);
        assert!(!dsar.rights_information.is_empty());
    }

    #[test]
    fn test_explain_decision() {
        let mut audit = GdprAuditTrail::new();
        let subject_id = Uuid::new_v4();

        let record = GdprDecisionRecord::new()
            .with_statute("GDPR_Art22")
            .with_data_subject(subject_id)
            .with_purpose("Automated scoring")
            .with_effect("score_calculated");

        let decision_id = audit.record_decision(record).expect("record failed");

        let explanation = audit.explain_decision(decision_id).expect("explain failed");
        assert_eq!(explanation.decision_id, decision_id);
        assert!(!explanation.safeguards.is_empty());
        assert!(explanation.right_to_contest);
    }

    #[test]
    fn test_verify_accountability() {
        let mut audit = GdprAuditTrail::new().with_controller("Controller");
        let subject_id = Uuid::new_v4();

        let record = GdprDecisionRecord::new()
            .with_statute("GDPR_Art5")
            .with_data_subject(subject_id)
            .with_effect("processed");

        audit.record_decision(record).expect("record failed");

        let report = audit
            .verify_accountability()
            .expect("accountability check failed");
        assert!(report.integrity_verified);
        assert_eq!(report.total_decisions, 1);
    }

    #[test]
    fn test_lawful_basis_metadata() {
        let consent = LawfulBasis::Consent {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
        };
        let metadata = LawfulBasisMetadata::from_lawful_basis(&consent);
        assert!(metadata.basis_type.contains("Consent"));
        assert!(metadata.details.contains("freely_given=true"));

        let legitimate = LawfulBasis::LegitimateInterests {
            controller_interest: "fraud prevention".to_string(),
            balancing_test_passed: true,
        };
        let metadata = LawfulBasisMetadata::from_lawful_basis(&legitimate);
        assert!(metadata.basis_type.contains("Legitimate Interests"));
    }

    #[test]
    fn test_data_subject_rights_info() {
        let rights = GdprAuditTrail::data_subject_rights_info();
        assert_eq!(rights.len(), 7);

        let access = rights.iter().find(|r| r.right == DataSubjectRight::Access);
        assert!(access.is_some());
        assert!(
            access
                .expect("access right not found")
                .article
                .contains("15")
        );
    }
}
