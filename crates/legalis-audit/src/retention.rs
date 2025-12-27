//! Retention policies and GDPR compliance for audit trails.

use crate::{AuditRecord, DecisionContext};
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

/// Data minimization strategy for compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinimizationStrategy {
    /// Redact sensitive fields with asterisks
    Redact,
    /// Replace with pseudonyms (consistent hashing)
    Pseudonymize,
    /// Remove entirely
    Remove,
}

/// Data minimization configuration.
#[derive(Debug, Clone)]
pub struct DataMinimization {
    /// Fields to minimize in context attributes
    pub attribute_fields: Vec<String>,
    /// Fields to minimize in metadata
    pub metadata_fields: Vec<String>,
    /// Minimization strategy
    pub strategy: MinimizationStrategy,
}

impl DataMinimization {
    /// Creates a new data minimization configuration.
    pub fn new(strategy: MinimizationStrategy) -> Self {
        Self {
            attribute_fields: Vec::new(),
            metadata_fields: Vec::new(),
            strategy,
        }
    }

    /// Adds attribute fields to minimize.
    pub fn with_attribute_fields(mut self, fields: Vec<String>) -> Self {
        self.attribute_fields = fields;
        self
    }

    /// Adds metadata fields to minimize.
    pub fn with_metadata_fields(mut self, fields: Vec<String>) -> Self {
        self.metadata_fields = fields;
        self
    }

    /// Applies data minimization to a record.
    pub fn apply(&self, record: &mut AuditRecord) {
        match self.strategy {
            MinimizationStrategy::Redact => {
                self.redact_fields(&mut record.context);
            }
            MinimizationStrategy::Pseudonymize => {
                self.pseudonymize_fields(&mut record.context);
            }
            MinimizationStrategy::Remove => {
                self.remove_fields(&mut record.context);
            }
        }
    }

    /// Applies data minimization to multiple records.
    pub fn apply_batch(&self, records: &mut [AuditRecord]) {
        for record in records {
            self.apply(record);
        }
    }

    fn redact_fields(&self, context: &mut DecisionContext) {
        for field in &self.attribute_fields {
            if context.attributes.contains_key(field) {
                context.attributes.insert(field.clone(), "***REDACTED***".to_string());
            }
        }

        for field in &self.metadata_fields {
            if context.metadata.contains_key(field) {
                context.metadata.insert(field.clone(), "***REDACTED***".to_string());
            }
        }
    }

    fn pseudonymize_fields(&self, context: &mut DecisionContext) {
        for field in &self.attribute_fields {
            if let Some(value) = context.attributes.get(field) {
                let pseudo = Self::compute_pseudonym(value);
                context.attributes.insert(field.clone(), pseudo);
            }
        }

        for field in &self.metadata_fields {
            if let Some(value) = context.metadata.get(field) {
                let pseudo = Self::compute_pseudonym(value);
                context.metadata.insert(field.clone(), pseudo);
            }
        }
    }

    fn remove_fields(&self, context: &mut DecisionContext) {
        for field in &self.attribute_fields {
            context.attributes.remove(field);
        }

        for field in &self.metadata_fields {
            context.metadata.remove(field);
        }
    }

    fn compute_pseudonym(value: &str) -> String {
        // Simple hash-based pseudonymization
        let hash = {
            let mut h: u64 = 0;
            for byte in value.bytes() {
                h = h.wrapping_mul(31).wrapping_add(byte as u64);
            }
            h
        };
        format!("PSEUDO_{:016x}", hash)
    }
}

/// Automatic data minimization policy.
#[derive(Debug, Clone)]
pub struct AutoMinimizationPolicy {
    /// Apply after this many days
    pub apply_after_days: i64,
    /// Minimization configuration
    pub minimization: DataMinimization,
    /// Exemptions (statutes that should not be minimized)
    pub exemptions: Vec<String>,
}

impl AutoMinimizationPolicy {
    /// Creates a new auto-minimization policy.
    pub fn new(apply_after_days: i64, minimization: DataMinimization) -> Self {
        Self {
            apply_after_days,
            minimization,
            exemptions: Vec::new(),
        }
    }

    /// Adds statute exemptions.
    pub fn with_exemptions(mut self, exemptions: Vec<String>) -> Self {
        self.exemptions = exemptions;
        self
    }

    /// Checks if a record should be minimized.
    pub fn should_minimize(&self, record: &AuditRecord) -> bool {
        // Check exemptions
        if self.exemptions.contains(&record.statute_id) {
            return false;
        }

        // Check age
        let age = Utc::now().signed_duration_since(record.timestamp);
        age.num_days() >= self.apply_after_days
    }

    /// Applies minimization to eligible records.
    pub fn apply(&self, records: &mut [AuditRecord]) -> usize {
        let mut minimized_count = 0;

        for record in records {
            if self.should_minimize(record) {
                self.minimization.apply(record);
                minimized_count += 1;
            }
        }

        minimized_count
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

    #[test]
    fn test_data_minimization_redact() {
        let mut record = create_test_record("statute-1", Uuid::new_v4(), 5);
        record.context.attributes.insert("name".to_string(), "John Doe".to_string());
        record.context.attributes.insert("email".to_string(), "john@example.com".to_string());

        let minimization = DataMinimization::new(MinimizationStrategy::Redact)
            .with_attribute_fields(vec!["name".to_string(), "email".to_string()]);

        minimization.apply(&mut record);

        assert_eq!(record.context.attributes.get("name").unwrap(), "***REDACTED***");
        assert_eq!(record.context.attributes.get("email").unwrap(), "***REDACTED***");
    }

    #[test]
    fn test_data_minimization_pseudonymize() {
        let mut record = create_test_record("statute-1", Uuid::new_v4(), 5);
        record.context.attributes.insert("ssn".to_string(), "123-45-6789".to_string());

        let minimization = DataMinimization::new(MinimizationStrategy::Pseudonymize)
            .with_attribute_fields(vec!["ssn".to_string()]);

        minimization.apply(&mut record);

        let pseudonym = record.context.attributes.get("ssn").unwrap();
        assert!(pseudonym.starts_with("PSEUDO_"));
        assert_ne!(pseudonym, "123-45-6789");
    }

    #[test]
    fn test_data_minimization_remove() {
        let mut record = create_test_record("statute-1", Uuid::new_v4(), 5);
        record.context.attributes.insert("sensitive".to_string(), "data".to_string());
        record.context.attributes.insert("public".to_string(), "info".to_string());

        let minimization = DataMinimization::new(MinimizationStrategy::Remove)
            .with_attribute_fields(vec!["sensitive".to_string()]);

        minimization.apply(&mut record);

        assert!(!record.context.attributes.contains_key("sensitive"));
        assert!(record.context.attributes.contains_key("public"));
    }

    #[test]
    fn test_auto_minimization_policy() {
        let mut records = vec![
            create_test_record("statute-1", Uuid::new_v4(), 5),
            create_test_record("statute-1", Uuid::new_v4(), 50),
        ];

        for record in &mut records {
            record.context.attributes.insert("name".to_string(), "Test".to_string());
        }

        let minimization = DataMinimization::new(MinimizationStrategy::Redact)
            .with_attribute_fields(vec!["name".to_string()]);

        let policy = AutoMinimizationPolicy::new(30, minimization);
        let count = policy.apply(&mut records);

        assert_eq!(count, 1); // Only the 50-day-old record should be minimized
        assert_eq!(records[0].context.attributes.get("name").unwrap(), "Test");
        assert_eq!(records[1].context.attributes.get("name").unwrap(), "***REDACTED***");
    }

    #[test]
    fn test_auto_minimization_with_exemptions() {
        let mut records = vec![
            create_test_record("statute-normal", Uuid::new_v4(), 50),
            create_test_record("statute-exempt", Uuid::new_v4(), 50),
        ];

        for record in &mut records {
            record.context.attributes.insert("data".to_string(), "sensitive".to_string());
        }

        let minimization = DataMinimization::new(MinimizationStrategy::Redact)
            .with_attribute_fields(vec!["data".to_string()]);

        let policy = AutoMinimizationPolicy::new(30, minimization)
            .with_exemptions(vec!["statute-exempt".to_string()]);

        let count = policy.apply(&mut records);

        assert_eq!(count, 1); // Only the non-exempt record
        assert_eq!(records[0].context.attributes.get("data").unwrap(), "***REDACTED***");
        assert_eq!(records[1].context.attributes.get("data").unwrap(), "sensitive");
    }
}
