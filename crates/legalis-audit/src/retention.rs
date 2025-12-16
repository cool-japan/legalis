//! Retention policies and GDPR compliance for audit trails.

use crate::AuditRecord;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Retention policy for audit records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Maximum age of records to keep
    pub max_age: Duration,
    /// Whether to archive before deletion
    pub archive_before_delete: bool,
    /// Exemptions (statutes that should be kept longer)
    pub exemptions: Vec<String>,
}

impl RetentionPolicy {
    /// Creates a new retention policy with the given max age in days.
    pub fn new(days: i64) -> Self {
        Self {
            max_age: Duration::days(days),
            archive_before_delete: false,
            exemptions: Vec::new(),
        }
    }

    /// Sets whether to archive before deletion.
    pub fn with_archival(mut self, archive: bool) -> Self {
        self.archive_before_delete = archive;
        self
    }

    /// Adds statute exemptions (records with these statutes are kept longer).
    pub fn with_exemptions(mut self, exemptions: Vec<String>) -> Self {
        self.exemptions = exemptions;
        self
    }

    /// Checks if a record should be retained according to the policy.
    pub fn should_retain(&self, record: &AuditRecord) -> bool {
        // Check exemptions first
        if self.exemptions.contains(&record.statute_id) {
            return true;
        }

        // Check age
        let age = Utc::now().signed_duration_since(record.timestamp);
        age <= self.max_age
    }

    /// Filters records according to the retention policy.
    pub fn apply(&self, records: &[AuditRecord]) -> Vec<AuditRecord> {
        records
            .iter()
            .filter(|r| self.should_retain(r))
            .cloned()
            .collect()
    }

    /// Identifies records that should be deleted.
    pub fn records_to_delete(&self, records: &[AuditRecord]) -> Vec<AuditRecord> {
        records
            .iter()
            .filter(|r| !self.should_retain(r))
            .cloned()
            .collect()
    }
}

/// GDPR data subject access request handler.
pub struct DataSubjectAccessRequest;

impl DataSubjectAccessRequest {
    /// Retrieves all data for a specific subject (GDPR Article 15).
    pub fn export_subject_data(records: &[AuditRecord], subject_id: Uuid) -> SubjectDataExport {
        let subject_records: Vec<_> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .cloned()
            .collect();

        let first_seen = subject_records.iter().map(|r| r.timestamp).min();
        let last_seen = subject_records.iter().map(|r| r.timestamp).max();

        let statutes_applied: Vec<_> = subject_records
            .iter()
            .map(|r| r.statute_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        SubjectDataExport {
            subject_id,
            export_date: Utc::now(),
            first_seen,
            last_seen,
            total_records: subject_records.len(),
            statutes_applied,
            records: subject_records,
        }
    }

    /// Identifies records that should be deleted for a subject (right to erasure).
    pub fn identify_for_erasure(
        records: &[AuditRecord],
        subject_id: Uuid,
        legal_basis_check: impl Fn(&AuditRecord) -> bool,
    ) -> ErasureAnalysis {
        let subject_records: Vec<_> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .cloned()
            .collect();

        let mut can_erase = Vec::new();
        let mut must_retain = Vec::new();

        for record in subject_records {
            if legal_basis_check(&record) {
                must_retain.push(record);
            } else {
                can_erase.push(record);
            }
        }

        ErasureAnalysis {
            subject_id,
            total_records: can_erase.len() + must_retain.len(),
            can_erase_count: can_erase.len(),
            must_retain_count: must_retain.len(),
            can_erase,
            must_retain,
        }
    }
}

/// Export of all data for a subject (GDPR Article 15).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectDataExport {
    pub subject_id: Uuid,
    pub export_date: DateTime<Utc>,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub total_records: usize,
    pub statutes_applied: Vec<String>,
    pub records: Vec<AuditRecord>,
}

/// Analysis of records that can be erased vs must be retained.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErasureAnalysis {
    pub subject_id: Uuid,
    pub total_records: usize,
    pub can_erase_count: usize,
    pub must_retain_count: usize,
    pub can_erase: Vec<AuditRecord>,
    pub must_retain: Vec<AuditRecord>,
}

/// Right to explanation support (GDPR Article 22).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionExplanation {
    /// The decision record
    pub record: AuditRecord,
    /// Human-readable explanation
    pub explanation: String,
    /// Supporting reasoning
    pub reasoning: Vec<String>,
    /// Data inputs used
    pub inputs_used: Vec<String>,
    /// Generated timestamp
    pub generated_at: DateTime<Utc>,
}

impl DecisionExplanation {
    /// Generates an explanation for a decision.
    pub fn generate(record: &AuditRecord) -> Self {
        let mut reasoning = Vec::new();
        let mut inputs_used = Vec::new();

        // Extract reasoning from evaluated conditions
        for condition in &record.context.evaluated_conditions {
            reasoning.push(format!(
                "{}: {} (input: {})",
                condition.description,
                if condition.result {
                    "satisfied"
                } else {
                    "not satisfied"
                },
                condition.input_value.as_deref().unwrap_or("N/A")
            ));

            if let Some(input) = &condition.input_value {
                inputs_used.push(input.clone());
            }
        }

        let explanation = match &record.result {
            crate::DecisionResult::Deterministic {
                effect_applied,
                parameters,
            } => {
                format!(
                    "Automatic decision applied statute '{}' resulting in effect '{}' with {} parameters. \
                     The decision was deterministic based on the evaluated conditions.",
                    record.statute_id,
                    effect_applied,
                    parameters.len()
                )
            }
            crate::DecisionResult::RequiresDiscretion { issue, .. } => {
                format!(
                    "The application of statute '{}' required human discretion due to: {}. \
                     A human review was required to make the final determination.",
                    record.statute_id, issue
                )
            }
            crate::DecisionResult::Void { reason } => {
                format!(
                    "The decision under statute '{}' was voided due to: {}",
                    record.statute_id, reason
                )
            }
            crate::DecisionResult::Overridden { justification, .. } => {
                format!(
                    "The automatic decision under statute '{}' was overridden by a human with justification: {}",
                    record.statute_id, justification
                )
            }
        };

        Self {
            record: record.clone(),
            explanation,
            reasoning,
            inputs_used,
            generated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn create_test_record(statute_id: &str, subject_id: Uuid, days_ago: i64) -> AuditRecord {
        let mut record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute_id.to_string(),
            subject_id,
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );
        record.timestamp = Utc::now() - Duration::days(days_ago);
        record
    }

    #[test]
    fn test_retention_policy() {
        let records = vec![
            create_test_record("statute-1", Uuid::new_v4(), 5), // 5 days old
            create_test_record("statute-1", Uuid::new_v4(), 10), // 10 days old
            create_test_record("statute-1", Uuid::new_v4(), 100), // 100 days old
        ];

        let policy = RetentionPolicy::new(30); // Keep for 30 days
        let retained = policy.apply(&records);

        assert_eq!(retained.len(), 2); // Should keep the 5 and 10 day old records
    }

    #[test]
    fn test_retention_policy_with_exemptions() {
        let records = vec![
            create_test_record("statute-1", Uuid::new_v4(), 100),
            create_test_record("statute-exempt", Uuid::new_v4(), 100),
        ];

        let policy = RetentionPolicy::new(30).with_exemptions(vec!["statute-exempt".to_string()]);

        let retained = policy.apply(&records);
        assert_eq!(retained.len(), 1); // Only the exempt one should be kept
        assert_eq!(retained[0].statute_id, "statute-exempt");
    }

    #[test]
    fn test_data_subject_access() {
        let subject_id = Uuid::new_v4();
        let records = vec![
            create_test_record("statute-1", subject_id, 5),
            create_test_record("statute-2", subject_id, 10),
            create_test_record("statute-1", Uuid::new_v4(), 5),
        ];

        let export = DataSubjectAccessRequest::export_subject_data(&records, subject_id);

        assert_eq!(export.total_records, 2);
        assert_eq!(export.subject_id, subject_id);
        assert!(export.first_seen.is_some());
        assert!(export.last_seen.is_some());
    }

    #[test]
    fn test_erasure_analysis() {
        let subject_id = Uuid::new_v4();
        let records = vec![
            create_test_record("statute-legal", subject_id, 5),
            create_test_record("statute-normal", subject_id, 10),
        ];

        let analysis = DataSubjectAccessRequest::identify_for_erasure(
            &records,
            subject_id,
            |r| r.statute_id == "statute-legal", // Legal basis to retain
        );

        assert_eq!(analysis.must_retain_count, 1);
        assert_eq!(analysis.can_erase_count, 1);
    }

    #[test]
    fn test_decision_explanation() {
        let record = create_test_record("statute-1", Uuid::new_v4(), 5);
        let explanation = DecisionExplanation::generate(&record);

        assert!(!explanation.explanation.is_empty());
        assert!(explanation.explanation.contains("statute-1"));
    }
}
