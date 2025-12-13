//! Legalis-Audit: Audit trail and decision logging for Legalis-RS.
//!
//! This crate provides comprehensive audit logging for legal decisions:
//! - Decision recording with full context
//! - Immutable audit trails
//! - Compliance reporting
//! - Decision replay and analysis

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

/// Errors during audit operations.
#[derive(Debug, Error)]
pub enum AuditError {
    #[error("Record not found: {0}")]
    RecordNotFound(Uuid),

    #[error("Invalid record: {0}")]
    InvalidRecord(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Tamper detected: {0}")]
    TamperDetected(String),
}

/// Result type for audit operations.
pub type AuditResult<T> = Result<T, AuditError>;

/// An audit record for a legal decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    /// Unique identifier for this record
    pub id: Uuid,
    /// Timestamp of the decision
    pub timestamp: DateTime<Utc>,
    /// Type of event
    pub event_type: EventType,
    /// Actor who triggered the decision (system, user ID, etc.)
    pub actor: Actor,
    /// The statute that was applied
    pub statute_id: String,
    /// Entity the decision was made about
    pub subject_id: Uuid,
    /// The input context (attributes, parameters)
    pub context: DecisionContext,
    /// The result of the decision
    pub result: DecisionResult,
    /// Hash of previous record (for chain integrity)
    pub previous_hash: Option<String>,
    /// Hash of this record
    pub record_hash: String,
}

impl AuditRecord {
    /// Creates a new audit record.
    pub fn new(
        event_type: EventType,
        actor: Actor,
        statute_id: String,
        subject_id: Uuid,
        context: DecisionContext,
        result: DecisionResult,
        previous_hash: Option<String>,
    ) -> Self {
        let mut record = Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            actor,
            statute_id,
            subject_id,
            context,
            result,
            previous_hash,
            record_hash: String::new(),
        };
        record.record_hash = record.compute_hash();
        record
    }

    /// Computes the hash for this record.
    fn compute_hash(&self) -> String {
        // Simple hash implementation - in production, use a proper cryptographic hash
        let data = format!(
            "{}{}{}{}{}{}",
            self.id,
            self.timestamp.timestamp(),
            self.statute_id,
            self.subject_id,
            self.previous_hash.as_deref().unwrap_or(""),
            serde_json::to_string(&self.result).unwrap_or_default()
        );
        format!("{:x}", md5_hash(&data))
    }

    /// Verifies the integrity of this record.
    pub fn verify(&self) -> bool {
        let computed = self.compute_hash();
        computed == self.record_hash
    }
}

/// Simple MD5-like hash (for demonstration - use proper crypto in production).
fn md5_hash(input: &str) -> u64 {
    let mut hash: u64 = 0;
    for byte in input.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

/// Type of audit event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    /// Automatic decision by the system
    AutomaticDecision,
    /// Decision requiring human review
    DiscretionaryReview,
    /// Human override of automatic decision
    HumanOverride,
    /// Appeal or review request
    Appeal,
    /// Statute was modified
    StatuteModified,
    /// Simulation run
    SimulationRun,
}

/// Actor who triggered the event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actor {
    /// Automated system
    System { component: String },
    /// Human user
    User { user_id: String, role: String },
    /// External system
    External { system_id: String },
}

/// Context for a decision.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecisionContext {
    /// Input attributes used in the decision
    pub attributes: HashMap<String, String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Conditions that were evaluated
    pub evaluated_conditions: Vec<EvaluatedCondition>,
}

/// A condition that was evaluated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatedCondition {
    /// Description of the condition
    pub description: String,
    /// Result of evaluation
    pub result: bool,
    /// Input value used
    pub input_value: Option<String>,
    /// Threshold/expected value
    pub threshold: Option<String>,
}

/// Result of a decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionResult {
    /// Deterministic result
    Deterministic {
        effect_applied: String,
        parameters: HashMap<String, String>,
    },
    /// Requires discretion
    RequiresDiscretion {
        issue: String,
        narrative_hint: Option<String>,
        assigned_to: Option<String>,
    },
    /// Void due to logical error
    Void { reason: String },
    /// Overridden by human
    Overridden {
        original_result: Box<DecisionResult>,
        new_result: Box<DecisionResult>,
        justification: String,
    },
}

/// Audit trail storage.
pub struct AuditTrail {
    records: Arc<RwLock<Vec<AuditRecord>>>,
    last_hash: Arc<RwLock<Option<String>>>,
}

impl AuditTrail {
    /// Creates a new audit trail.
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(Vec::new())),
            last_hash: Arc::new(RwLock::new(None)),
        }
    }

    /// Records a new decision.
    pub fn record(&self, mut record: AuditRecord) -> AuditResult<Uuid> {
        let mut records = self.records.write().map_err(|e| {
            AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;

        let mut last_hash = self
            .last_hash
            .write()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire hash lock: {}", e)))?;

        // Set previous hash and recompute
        record.previous_hash = last_hash.clone();
        record.record_hash = record.compute_hash();

        let id = record.id;
        *last_hash = Some(record.record_hash.clone());
        records.push(record);

        tracing::info!("Audit record created: {}", id);
        Ok(id)
    }

    /// Gets a record by ID.
    pub fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        records
            .iter()
            .find(|r| r.id == id)
            .cloned()
            .ok_or(AuditError::RecordNotFound(id))
    }

    /// Queries records by statute ID.
    pub fn query_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        Ok(records
            .iter()
            .filter(|r| r.statute_id == statute_id)
            .cloned()
            .collect())
    }

    /// Queries records by subject ID.
    pub fn query_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        Ok(records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .cloned()
            .collect())
    }

    /// Queries records within a time range.
    pub fn query_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        Ok(records
            .iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .cloned()
            .collect())
    }

    /// Verifies the integrity of the entire audit trail.
    pub fn verify_integrity(&self) -> AuditResult<bool> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        let mut expected_prev_hash: Option<String> = None;

        for record in records.iter() {
            // Verify record hash
            if !record.verify() {
                return Err(AuditError::TamperDetected(format!(
                    "Record {} has invalid hash",
                    record.id
                )));
            }

            // Verify chain
            if record.previous_hash != expected_prev_hash {
                return Err(AuditError::TamperDetected(format!(
                    "Record {} has broken chain link",
                    record.id
                )));
            }

            expected_prev_hash = Some(record.record_hash.clone());
        }

        Ok(true)
    }

    /// Returns the total number of records.
    pub fn count(&self) -> usize {
        self.records.read().map(|r| r.len()).unwrap_or(0)
    }

    /// Generates a compliance report.
    pub fn generate_report(&self) -> AuditResult<ComplianceReport> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        let total = records.len();
        let automatic = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Deterministic { .. }))
            .count();
        let discretionary = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::RequiresDiscretion { .. }))
            .count();
        let overridden = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
            .count();

        Ok(ComplianceReport {
            total_decisions: total,
            automatic_decisions: automatic,
            discretionary_decisions: discretionary,
            human_overrides: overridden,
            integrity_verified: self.verify_integrity().is_ok(),
            generated_at: Utc::now(),
        })
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

/// Compliance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub total_decisions: usize,
    pub automatic_decisions: usize,
    pub discretionary_decisions: usize,
    pub human_overrides: usize,
    pub integrity_verified: bool,
    pub generated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_record_creation() {
        let record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );

        assert!(record.verify());
    }

    #[test]
    fn test_audit_trail() {
        let trail = AuditTrail::new();

        let record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );

        let id = trail.record(record).unwrap();
        assert!(trail.get(id).is_ok());
        assert_eq!(trail.count(), 1);
    }

    #[test]
    fn test_audit_integrity() {
        let trail = AuditTrail::new();

        for i in 0..5 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                format!("statute-{}", i),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            );
            trail.record(record).unwrap();
        }

        assert!(trail.verify_integrity().unwrap());
    }

    #[test]
    fn test_compliance_report() {
        let trail = AuditTrail::new();

        let record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );
        trail.record(record).unwrap();

        let report = trail.generate_report().unwrap();
        assert_eq!(report.total_decisions, 1);
        assert_eq!(report.automatic_decisions, 1);
    }
}
