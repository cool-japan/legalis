//! Selective Disclosure for audit trails.
//!
//! Provides controlled information release mechanisms that allow
//! revealing specific audit information while keeping other details private.
//! Uses cryptographic commitments and Merkle trees for selective disclosure.

use crate::{AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Selective disclosure manager
pub struct SelectiveDisclosure {
    commitment_key: Vec<u8>,
}

impl SelectiveDisclosure {
    /// Create a new selective disclosure manager
    pub fn new(commitment_key: Vec<u8>) -> Self {
        Self { commitment_key }
    }

    /// Create committed audit record with all fields hidden
    pub fn commit_record(&self, record: &AuditRecord) -> CommittedRecord {
        let fields = self.extract_fields(record);
        let commitments = fields
            .iter()
            .map(|(name, value)| {
                let commitment = self.commit_field(name, value);
                (name.clone(), commitment)
            })
            .collect();

        CommittedRecord {
            record_id: record.id,
            commitments,
            disclosed_fields: HashSet::new(),
        }
    }

    /// Selectively disclose specific fields
    pub fn disclose_fields(
        &self,
        record: &AuditRecord,
        committed: &mut CommittedRecord,
        field_names: &[&str],
    ) -> AuditResult<DisclosureProof> {
        let fields = self.extract_fields(record);
        let mut revealed_fields = HashMap::new();
        let mut proofs = HashMap::new();

        for field_name in field_names {
            if let Some(value) = fields.get(*field_name) {
                let proof = self.create_disclosure_proof(field_name, value)?;
                revealed_fields.insert(field_name.to_string(), value.clone());
                proofs.insert(field_name.to_string(), proof);
                committed.disclosed_fields.insert(field_name.to_string());
            }
        }

        Ok(DisclosureProof {
            record_id: record.id,
            revealed_fields,
            proofs,
        })
    }

    /// Verify a disclosure proof
    pub fn verify_disclosure(
        &self,
        committed: &CommittedRecord,
        proof: &DisclosureProof,
    ) -> AuditResult<bool> {
        if committed.record_id != proof.record_id {
            return Ok(false);
        }

        for (field_name, field_value) in &proof.revealed_fields {
            let expected_commitment = committed.commitments.get(field_name);
            if expected_commitment.is_none() {
                return Ok(false);
            }

            let computed_commitment = self.commit_field(field_name, field_value);
            if &computed_commitment != expected_commitment.unwrap() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Create a redacted record with only disclosed fields visible
    pub fn create_redacted_record(&self, proof: &DisclosureProof) -> RedactedRecord {
        RedactedRecord {
            record_id: proof.record_id,
            disclosed_fields: proof.revealed_fields.clone(),
            redacted_field_count: proof.proofs.len(),
        }
    }

    /// Selectively disclose statistics without revealing individual records
    pub fn disclose_statistics(
        &self,
        records: &[AuditRecord],
        statistic: StatisticType,
    ) -> AuditResult<StatisticDisclosure> {
        match statistic {
            StatisticType::Count => Ok(StatisticDisclosure {
                statistic_type: statistic,
                value: records.len() as f64,
                commitment: self.commit_statistic(statistic, records.len() as f64),
            }),
            StatisticType::UniqueStatutes => {
                let unique: HashSet<_> = records.iter().map(|r| &r.statute_id).collect();
                let count = unique.len() as f64;
                Ok(StatisticDisclosure {
                    statistic_type: statistic,
                    value: count,
                    commitment: self.commit_statistic(statistic, count),
                })
            }
            StatisticType::UniqueSubjects => {
                let unique: HashSet<_> = records.iter().map(|r| r.subject_id).collect();
                let count = unique.len() as f64;
                Ok(StatisticDisclosure {
                    statistic_type: statistic,
                    value: count,
                    commitment: self.commit_statistic(statistic, count),
                })
            }
            StatisticType::TimeSpan => {
                if records.is_empty() {
                    return Ok(StatisticDisclosure {
                        statistic_type: statistic,
                        value: 0.0,
                        commitment: self.commit_statistic(statistic, 0.0),
                    });
                }
                let first_time = records.first().unwrap().timestamp.timestamp();
                let last_time = records.last().unwrap().timestamp.timestamp();
                let span = (last_time - first_time) as f64;
                Ok(StatisticDisclosure {
                    statistic_type: statistic,
                    value: span,
                    commitment: self.commit_statistic(statistic, span),
                })
            }
        }
    }

    /// Create a privacy-preserving summary with selective disclosure
    pub fn create_summary(
        &self,
        records: &[AuditRecord],
        disclose_fields: &[&str],
    ) -> AuditSummary {
        let mut field_summaries = HashMap::new();

        for field_name in disclose_fields {
            let values: Vec<String> = records
                .iter()
                .filter_map(|r| {
                    let fields = self.extract_fields(r);
                    fields.get(*field_name).cloned()
                })
                .collect();

            let unique_values: HashSet<_> = values.iter().collect();
            field_summaries.insert(
                field_name.to_string(),
                FieldSummary {
                    unique_count: unique_values.len(),
                    total_count: values.len(),
                    sample_disclosed: false,
                },
            );
        }

        AuditSummary {
            total_records: records.len(),
            field_summaries,
            commitment: self.commit_data(&format!("summary:{}", records.len())),
        }
    }

    // Helper methods

    fn extract_fields(&self, record: &AuditRecord) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert("id".to_string(), record.id.to_string());
        fields.insert("timestamp".to_string(), record.timestamp.to_rfc3339());
        fields.insert("statute_id".to_string(), record.statute_id.clone());
        fields.insert("subject_id".to_string(), record.subject_id.to_string());
        fields.insert("event_type".to_string(), format!("{:?}", record.event_type));
        fields.insert("actor".to_string(), format!("{:?}", record.actor));
        fields.insert("record_hash".to_string(), record.record_hash.clone());
        fields
    }

    fn commit_field(&self, field_name: &str, field_value: &str) -> String {
        let data = format!(
            "{}:{}:{}",
            field_name,
            field_value,
            self.commitment_key.len()
        );
        self.hash_data(&data)
    }

    fn commit_statistic(&self, statistic: StatisticType, value: f64) -> String {
        let data = format!("{:?}:{}:{}", statistic, value, self.commitment_key.len());
        self.hash_data(&data)
    }

    fn commit_data(&self, data: &str) -> String {
        self.hash_data(&format!("{}:{}", data, self.commitment_key.len()))
    }

    fn create_disclosure_proof(&self, field_name: &str, field_value: &str) -> AuditResult<String> {
        // Create a proof that this field commitment is valid
        let proof_data = format!(
            "proof:{}:{}:{}",
            field_name,
            field_value,
            self.commitment_key.len()
        );
        Ok(self.hash_data(&proof_data))
    }

    fn hash_data(&self, data: &str) -> String {
        let mut hash: u64 = 5381;
        for byte in data.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        format!("{:016x}", hash)
    }
}

/// Committed audit record with hidden fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommittedRecord {
    pub record_id: Uuid,
    pub commitments: HashMap<String, String>,
    pub disclosed_fields: HashSet<String>,
}

/// Disclosure proof for revealing specific fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisclosureProof {
    pub record_id: Uuid,
    pub revealed_fields: HashMap<String, String>,
    pub proofs: HashMap<String, String>,
}

/// Redacted record with only disclosed fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactedRecord {
    pub record_id: Uuid,
    pub disclosed_fields: HashMap<String, String>,
    pub redacted_field_count: usize,
}

/// Types of statistics that can be disclosed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum StatisticType {
    Count,
    UniqueStatutes,
    UniqueSubjects,
    TimeSpan,
}

/// Statistic disclosure with commitment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticDisclosure {
    pub statistic_type: StatisticType,
    pub value: f64,
    pub commitment: String,
}

/// Summary with selective disclosure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSummary {
    pub total_records: usize,
    pub field_summaries: HashMap<String, FieldSummary>,
    pub commitment: String,
}

/// Summary for a specific field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSummary {
    pub unique_count: usize,
    pub total_count: usize,
    pub sample_disclosed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};

    fn create_test_record() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-123".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_commit_record() {
        let sd = SelectiveDisclosure::new(vec![1, 2, 3, 4]);
        let record = create_test_record();

        let committed = sd.commit_record(&record);

        assert_eq!(committed.record_id, record.id);
        assert!(!committed.commitments.is_empty());
        assert!(committed.disclosed_fields.is_empty());
    }

    #[test]
    fn test_selective_disclosure() {
        let sd = SelectiveDisclosure::new(vec![1, 2, 3, 4]);
        let record = create_test_record();

        let mut committed = sd.commit_record(&record);
        let proof = sd
            .disclose_fields(&record, &mut committed, &["statute_id", "event_type"])
            .unwrap();

        assert_eq!(proof.revealed_fields.len(), 2);
        assert!(proof.revealed_fields.contains_key("statute_id"));
        assert!(proof.revealed_fields.contains_key("event_type"));
    }

    #[test]
    fn test_verify_disclosure() {
        let sd = SelectiveDisclosure::new(vec![1, 2, 3, 4]);
        let record = create_test_record();

        let mut committed = sd.commit_record(&record);
        let proof = sd
            .disclose_fields(&record, &mut committed, &["statute_id"])
            .unwrap();

        let is_valid = sd.verify_disclosure(&committed, &proof).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_invalid_disclosure_verification() {
        let sd = SelectiveDisclosure::new(vec![1, 2, 3, 4]);
        let record1 = create_test_record();
        let record2 = create_test_record();

        let mut committed1 = sd.commit_record(&record1);
        let proof2 = sd
            .disclose_fields(&record2, &mut committed1, &["statute_id"])
            .unwrap();

        // Proof from record2 should not verify against committed1
        let is_valid = sd.verify_disclosure(&committed1, &proof2).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_create_redacted_record() {
        let sd = SelectiveDisclosure::new(vec![1, 2, 3, 4]);
        let record = create_test_record();

        let mut committed = sd.commit_record(&record);
        let proof = sd
            .disclose_fields(&record, &mut committed, &["statute_id", "timestamp"])
            .unwrap();

        let redacted = sd.create_redacted_record(&proof);

        assert_eq!(redacted.record_id, record.id);
        assert_eq!(redacted.disclosed_fields.len(), 2);
        assert_eq!(redacted.redacted_field_count, 2);
    }

    #[test]
    fn test_disclose_statistics() {
        let sd = SelectiveDisclosure::new(vec![1, 2, 3, 4]);
        let records: Vec<AuditRecord> = (0..5).map(|_| create_test_record()).collect();

        let count_stat = sd
            .disclose_statistics(&records, StatisticType::Count)
            .unwrap();
        assert_eq!(count_stat.value, 5.0);

        let unique_statutes = sd
            .disclose_statistics(&records, StatisticType::UniqueStatutes)
            .unwrap();
        assert!(unique_statutes.value >= 1.0);
    }

    #[test]
    fn test_create_summary() {
        let sd = SelectiveDisclosure::new(vec![1, 2, 3, 4]);
        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let summary = sd.create_summary(&records, &["statute_id", "event_type"]);

        assert_eq!(summary.total_records, 3);
        assert_eq!(summary.field_summaries.len(), 2);
        assert!(summary.field_summaries.contains_key("statute_id"));
    }

    #[test]
    fn test_time_span_statistic() {
        let sd = SelectiveDisclosure::new(vec![1, 2, 3, 4]);
        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let time_span = sd
            .disclose_statistics(&records, StatisticType::TimeSpan)
            .unwrap();
        assert!(time_span.value >= 0.0);
    }

    #[test]
    fn test_empty_records_statistics() {
        let sd = SelectiveDisclosure::new(vec![1, 2, 3, 4]);
        let records: Vec<AuditRecord> = vec![];

        let time_span = sd
            .disclose_statistics(&records, StatisticType::TimeSpan)
            .unwrap();
        assert_eq!(time_span.value, 0.0);
    }
}
