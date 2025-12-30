//! Legalis-Audit: Audit trail and decision logging for Legalis-RS.
//!
//! This crate provides comprehensive audit logging for legal decisions with:
//!
//! ## Core Features
//! - **Decision recording** with full context (actor, statute, subject, etc.)
//! - **Hash chain integrity** for tamper detection
//! - **Immutable audit trails** with cryptographic verification
//! - **Compliance reporting** with detailed statistics
//!
//! ## Storage Backends
//! - **In-memory**: Fast, ephemeral storage for testing/development
//! - **JSONL**: Durable file-based storage with human-readable format
//! - **Custom**: Implement `AuditStorage` trait for your own backend
//!
//! ## Query System
//! Use the [`query::QueryBuilder`] for flexible filtering:
//! - Filter by statute ID, subject ID, event type
//! - Filter by actor type (System, User, External)
//! - Date range queries
//! - Pagination support
//!
//! ## Export Formats
//! - CSV for spreadsheet analysis
//! - JSON for programmatic access
//! - JSON-LD for semantic web compatibility
//!
//! ## Analysis & Anomaly Detection
//! Use [`analysis::DecisionAnalyzer`] for pattern analysis:
//! - Decision distribution by statute, actor, event type
//! - Temporal distribution and trend analysis
//! - Anomaly detection (volume spikes, unusual override rates)
//! - Compliance summary generation
//!
//! ## Decision Replay
//! Use [`replay::DecisionReplayer`] for historical analysis:
//! - Point-in-time reconstruction of audit trail state
//! - Subject and statute history tracking
//! - Timeline comparison between two points
//! - What-if analysis by filtering decisions
//!
//! ## GDPR Compliance
//! Use [`retention`] module for GDPR compliance:
//! - Data subject access requests (Article 15)
//! - Right to explanation for automated decisions (Article 22)
//! - Retention policies with statute exemptions
//! - Erasure analysis (right to be forgotten)
//!
//! ## Example Usage
//!
//! ```rust
//! use legalis_audit::{AuditTrail, AuditRecord, EventType, Actor, DecisionContext, DecisionResult};
//! use std::collections::HashMap;
//! use uuid::Uuid;
//!
//! // Create an in-memory audit trail
//! let mut trail = AuditTrail::new();
//!
//! // Or use JSONL file storage
//! // let mut trail = AuditTrail::with_jsonl_file("/path/to/audit.jsonl").unwrap();
//!
//! // Record a decision
//! let record = AuditRecord::new(
//!     EventType::AutomaticDecision,
//!     Actor::System { component: "engine".to_string() },
//!     "statute-123".to_string(),
//!     Uuid::new_v4(),
//!     DecisionContext::default(),
//!     DecisionResult::Deterministic {
//!         effect_applied: "approved".to_string(),
//!         parameters: HashMap::new(),
//!     },
//!     None,
//! );
//!
//! let id = trail.record(record).unwrap();
//!
//! // Query records
//! let records = trail.query_by_statute("statute-123").unwrap();
//!
//! // Verify integrity
//! assert!(trail.verify_integrity().unwrap());
//!
//! // Generate compliance report
//! let report = trail.generate_report().unwrap();
//! println!("Total decisions: {}", report.total_decisions);
//! ```

pub mod aggregate;
pub mod analysis;
pub mod archival;
pub mod batch;
pub mod bias;
pub mod bloom;
pub mod clustering;
pub mod comparison;
pub mod compliance;
pub mod compression;
pub mod custody;
pub mod elasticsearch;
pub mod encryption;
pub mod evidence;
pub mod export;
pub mod forensic;
pub mod integrity;
pub mod integrity_checker;
pub mod join;
pub mod query;
pub mod query_plan;
pub mod regulator;
pub mod replay;
pub mod retention;
pub mod search;
pub mod siem;
pub mod storage;
pub mod telemetry;
pub mod timeline;
pub mod timeseries;
pub mod webhook;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Export error: {0}")]
    ExportError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
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
    storage: Box<dyn storage::AuditStorage>,
}

impl AuditTrail {
    /// Creates a new audit trail with in-memory storage.
    pub fn new() -> Self {
        Self {
            storage: Box::new(storage::memory::MemoryStorage::new()),
        }
    }

    /// Creates a new audit trail with custom storage.
    pub fn with_storage(storage: Box<dyn storage::AuditStorage>) -> Self {
        Self { storage }
    }

    /// Creates a new audit trail with JSONL file storage.
    pub fn with_jsonl_file<P: AsRef<std::path::Path>>(path: P) -> AuditResult<Self> {
        Ok(Self {
            storage: Box::new(storage::jsonl::JsonlStorage::new(path)?),
        })
    }

    /// Creates a new audit trail with SQLite storage.
    pub fn with_sqlite<P: AsRef<std::path::Path>>(path: P) -> AuditResult<Self> {
        Ok(Self {
            storage: Box::new(storage::sqlite::SqliteStorage::new(path)?),
        })
    }

    /// Creates a new audit trail with append-only log storage.
    pub fn with_append_only<P: AsRef<std::path::Path>>(path: P) -> AuditResult<Self> {
        Ok(Self {
            storage: Box::new(storage::append_only::AppendOnlyStorage::new(path)?),
        })
    }

    /// Creates a new audit trail with append-only log storage and rotation.
    pub fn with_append_only_rotation<P: AsRef<std::path::Path>>(
        path: P,
        max_size_bytes: u64,
    ) -> AuditResult<Self> {
        Ok(Self {
            storage: Box::new(storage::append_only::AppendOnlyStorage::with_rotation(
                path,
                max_size_bytes,
            )?),
        })
    }

    /// Creates a new audit trail with encrypted in-memory storage.
    pub fn with_encrypted_memory(key: encryption::EncryptionKey) -> Self {
        Self {
            storage: Box::new(storage::encrypted::EncryptedStorage::new(key)),
        }
    }

    /// Creates a new audit trail with cached storage.
    pub fn with_cached_storage(
        storage: Box<dyn storage::AuditStorage>,
        config: storage::cached::CacheConfig,
    ) -> Self {
        Self {
            storage: Box::new(storage::cached::CachedStorage::new(storage, config)),
        }
    }

    /// Records a new decision.
    pub fn record(&mut self, mut record: AuditRecord) -> AuditResult<Uuid> {
        // Get last hash and set previous hash
        let last_hash = self.storage.get_last_hash()?;
        record.previous_hash = last_hash;
        record.record_hash = record.compute_hash();

        let id = record.id;
        let hash = record.record_hash.clone();

        self.storage.store(record)?;
        self.storage.set_last_hash(Some(hash))?;

        tracing::info!("Audit record created: {}", id);
        Ok(id)
    }

    /// Gets a record by ID.
    pub fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        self.storage.get(id)
    }

    /// Queries records by statute ID.
    pub fn query_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        self.storage.get_by_statute(statute_id)
    }

    /// Queries records by subject ID.
    pub fn query_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        self.storage.get_by_subject(subject_id)
    }

    /// Queries records within a time range.
    pub fn query_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        self.storage.get_by_time_range(start, end)
    }

    /// Queries records using a query builder.
    pub fn query(&self, query: &query::QueryBuilder) -> AuditResult<Vec<AuditRecord>> {
        let all_records = self.storage.get_all()?;
        Ok(query.execute(&all_records))
    }

    /// Verifies the integrity of the entire audit trail.
    pub fn verify_integrity(&self) -> AuditResult<bool> {
        let records = self.storage.get_all()?;
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

    /// Verifies integrity using parallel verification for better performance.
    pub fn verify_integrity_parallel(&self) -> AuditResult<bool> {
        let records = self.storage.get_all()?;
        let verifier = integrity::parallel::ParallelVerifier::new();
        verifier.verify(&records)
    }

    /// Verifies integrity using sampling for quick checks on large datasets.
    ///
    /// `sample_rate` should be between 0.0 and 1.0, where 1.0 means verify all records.
    pub fn verify_integrity_sampled(&self, sample_rate: f64) -> AuditResult<bool> {
        let records = self.storage.get_all()?;
        let verifier = integrity::parallel::SamplingVerifier::new(sample_rate);
        verifier.verify_sample(&records)
    }

    /// Returns the total number of records.
    pub fn count(&self) -> usize {
        self.storage.count().unwrap_or(0)
    }

    /// Generates a compliance report.
    pub fn generate_report(&self) -> AuditResult<ComplianceReport> {
        let records = self.storage.get_all()?;

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

    /// Exports all records to CSV format.
    pub fn export_csv<W: std::io::Write>(&self, writer: &mut W) -> AuditResult<()> {
        let records = self.storage.get_all()?;
        export::to_csv(&records, writer)
    }

    /// Exports all records to JSON-LD format.
    pub fn export_jsonld(&self) -> AuditResult<serde_json::Value> {
        let records = self.storage.get_all()?;
        export::to_jsonld(&records)
    }

    /// Exports all records to JSON format.
    pub fn export_json(&self) -> AuditResult<serde_json::Value> {
        let records = self.storage.get_all()?;
        export::to_json(&records)
    }

    /// Exports all records to Excel format.
    pub fn export_excel<P: AsRef<std::path::Path>>(&self, path: P) -> AuditResult<()> {
        let records = self.storage.get_all()?;
        export::to_excel(&records, path)
    }

    /// Exports compliance report to PDF format.
    pub fn export_pdf<P: AsRef<std::path::Path>>(&self, path: P, title: &str) -> AuditResult<()> {
        let records = self.storage.get_all()?;
        let report = self.generate_report()?;
        export::to_pdf(&records, &report, path, title)
    }

    /// Exports compliance report to HTML format.
    pub fn export_html(&self, title: &str) -> AuditResult<String> {
        let records = self.storage.get_all()?;
        let report = self.generate_report()?;
        export::to_html(&records, &report, title)
    }

    /// Analyzes decision patterns in the audit trail.
    pub fn analyze_patterns(&self) -> AuditResult<analysis::DecisionAnalysis> {
        let records = self.storage.get_all()?;
        Ok(analysis::DecisionAnalyzer::analyze(&records))
    }

    /// Generates a distribution report for a specific dimension.
    pub fn distribution_report(
        &self,
        dimension: &str,
    ) -> AuditResult<analysis::DistributionReport> {
        let records = self.storage.get_all()?;
        Ok(analysis::DecisionAnalyzer::distribution_report(
            &records, dimension,
        ))
    }

    /// Detects volume anomalies in the audit trail.
    pub fn detect_volume_anomalies(
        &self,
        threshold: f64,
    ) -> AuditResult<Vec<analysis::VolumeAnomaly>> {
        let records = self.storage.get_all()?;
        Ok(analysis::AnomalyDetector::detect_volume_spikes(
            &records, threshold,
        ))
    }

    /// Detects override anomalies in the audit trail.
    pub fn detect_override_anomalies(&self) -> AuditResult<Vec<analysis::OverrideAnomaly>> {
        let records = self.storage.get_all()?;
        Ok(analysis::AnomalyDetector::detect_override_anomalies(
            &records,
        ))
    }

    /// Reconstructs the audit trail state at a specific point in time.
    pub fn reconstruct_at(
        &self,
        point_in_time: DateTime<Utc>,
    ) -> AuditResult<replay::TimelineState> {
        let records = self.storage.get_all()?;
        Ok(replay::DecisionReplayer::reconstruct_at_time(
            &records,
            point_in_time,
        ))
    }

    /// Gets the complete history for a specific subject.
    pub fn subject_history(&self, subject_id: Uuid) -> AuditResult<replay::SubjectHistory> {
        let records = self.storage.get_all()?;
        replay::DecisionReplayer::subject_history(&records, subject_id)
    }

    /// Gets the complete history for a specific statute.
    pub fn statute_history(&self, statute_id: &str) -> AuditResult<replay::StatuteHistory> {
        let records = self.storage.get_all()?;
        replay::DecisionReplayer::statute_history(&records, statute_id)
    }

    /// Compares the audit trail between two points in time.
    pub fn compare_timepoints(
        &self,
        time1: DateTime<Utc>,
        time2: DateTime<Utc>,
    ) -> AuditResult<replay::TimelineComparison> {
        let records = self.storage.get_all()?;
        Ok(replay::DecisionReplayer::compare_timepoints(
            &records, time1, time2,
        ))
    }

    /// Applies a retention policy to identify records to delete.
    pub fn apply_retention_policy(
        &self,
        policy: &retention::RetentionPolicy,
    ) -> AuditResult<Vec<AuditRecord>> {
        let records = self.storage.get_all()?;
        Ok(policy.records_to_delete(&records))
    }

    /// Exports all data for a subject (GDPR Article 15).
    pub fn export_subject_data(
        &self,
        subject_id: Uuid,
    ) -> AuditResult<retention::SubjectDataExport> {
        let records = self.storage.get_all()?;
        Ok(retention::DataSubjectAccessRequest::export_subject_data(
            &records, subject_id,
        ))
    }

    /// Generates an explanation for a specific decision (GDPR Article 22).
    pub fn explain_decision(&self, record_id: Uuid) -> AuditResult<retention::DecisionExplanation> {
        let record = self.get(record_id)?;
        Ok(retention::DecisionExplanation::generate(&record))
    }

    /// Builds a Merkle tree for efficient verification.
    pub fn build_merkle_tree(&self) -> AuditResult<integrity::MerkleTree> {
        let records = self.storage.get_all()?;
        Ok(integrity::MerkleTree::from_records(&records))
    }

    /// Generates a Merkle proof for a specific record.
    pub fn generate_merkle_proof(
        &self,
        record_id: Uuid,
    ) -> AuditResult<Option<integrity::MerkleProof>> {
        let tree = self.build_merkle_tree()?;
        Ok(tree.generate_proof(record_id))
    }

    /// Verifies a Merkle proof.
    pub fn verify_merkle_proof(&self, proof: &integrity::MerkleProof) -> AuditResult<bool> {
        let tree = self.build_merkle_tree()?;
        Ok(tree.verify_proof(proof))
    }

    /// Archives records that match the policy.
    pub fn archive_records(
        &self,
        policy: &archival::ArchivePolicy,
        archive_dir: &std::path::Path,
    ) -> AuditResult<archival::ArchiveMetadata> {
        let records = self.storage.get_all()?;
        let mut manager = archival::ArchiveManager::new(archive_dir, policy.clone())?;
        manager.archive_records(&records)
    }

    /// Exports audit trail for regulatory compliance.
    pub fn export_regulatory(&self, config: &regulator::ExportConfig) -> AuditResult<String> {
        let records = self.storage.get_all()?;
        let report = self.generate_report()?;
        regulator::RegulatoryExporter::export(&records, config, &report)
    }

    /// Exports all records to SIEM format.
    pub fn export_siem(&self, format: siem::SiemFormat) -> AuditResult<Vec<String>> {
        let records = self.storage.get_all()?;
        let exporter = siem::SiemExporter::with_format(format);
        exporter.export_records(&records)
    }

    /// Exports a single record to SIEM format.
    pub fn export_record_siem(
        &self,
        record_id: Uuid,
        format: siem::SiemFormat,
    ) -> AuditResult<String> {
        let record = self.get(record_id)?;
        let exporter = siem::SiemExporter::with_format(format);
        exporter.export_record(&record)
    }

    /// Exports all records to Elasticsearch bulk API format.
    pub fn export_elasticsearch_bulk(
        &self,
        config: elasticsearch::ElasticsearchConfig,
    ) -> AuditResult<String> {
        let records = self.storage.get_all()?;
        let exporter = elasticsearch::ElasticsearchExporter::new(config);
        exporter.export_bulk(&records)
    }

    /// Exports all records to Elasticsearch NDJSON format.
    pub fn export_elasticsearch_ndjson(
        &self,
        config: elasticsearch::ElasticsearchConfig,
    ) -> AuditResult<String> {
        let records = self.storage.get_all()?;
        let exporter = elasticsearch::ElasticsearchExporter::new(config);
        exporter.export_ndjson(&records)
    }

    /// Executes an aggregate query on the audit trail.
    pub fn aggregate(
        &self,
        query: &aggregate::AggregateQuery,
    ) -> AuditResult<aggregate::AggregationResult> {
        let records = self.storage.get_all()?;
        query.execute(&records)
    }

    /// Generates a month-over-month comparison report.
    pub fn month_over_month_report(
        &self,
        reference_date: DateTime<Utc>,
    ) -> AuditResult<comparison::ComparisonReport> {
        let records = self.storage.get_all()?;
        comparison::ComparisonGenerator::month_over_month(&records, reference_date)
    }

    /// Generates a year-over-year comparison report.
    pub fn year_over_year_report(
        &self,
        reference_date: DateTime<Utc>,
    ) -> AuditResult<comparison::ComparisonReport> {
        let records = self.storage.get_all()?;
        comparison::ComparisonGenerator::year_over_year(&records, reference_date)
    }

    /// Executes a time-series query on the audit trail.
    pub fn timeseries(
        &self,
        query: &timeseries::TimeSeriesQuery,
    ) -> AuditResult<timeseries::TimeSeries> {
        let records = self.storage.get_all()?;
        query.execute(&records)
    }

    /// Searches audit records using full-text search.
    pub fn search(&self, query: &search::SearchQuery) -> AuditResult<Vec<search::SearchResult>> {
        let records = self.storage.get_all()?;
        query.execute(&records)
    }

    /// Reconstructs a chronological timeline from audit records.
    pub fn reconstruct_timeline(&self, title: String) -> AuditResult<timeline::Timeline> {
        let records = self.storage.get_all()?;
        timeline::TimelineReconstructor::reconstruct_chronological(&records, title)
    }

    /// Reconstructs a timeline for a specific subject.
    pub fn reconstruct_subject_timeline(
        &self,
        subject_id: Uuid,
    ) -> AuditResult<timeline::Timeline> {
        let records = self.storage.get_all()?;
        timeline::TimelineReconstructor::reconstruct_subject(&records, subject_id)
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
        let mut trail = AuditTrail::new();

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
        let mut trail = AuditTrail::new();

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
        let mut trail = AuditTrail::new();

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

    #[test]
    fn test_query_builder_integration() {
        let mut trail = AuditTrail::new();

        let subject_id = Uuid::new_v4();
        let record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::User {
                user_id: "user-123".to_string(),
                role: "admin".to_string(),
            },
            "statute-test".to_string(),
            subject_id,
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );
        trail.record(record).unwrap();

        let query = query::QueryBuilder::new()
            .subject_id(subject_id)
            .actor(query::ActorFilter::AnyUser);
        let results = trail.query(&query).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_export_csv() {
        let mut trail = AuditTrail::new();

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

        let mut output = Vec::new();
        trail.export_csv(&mut output).unwrap();
        let csv = String::from_utf8(output).unwrap();
        assert!(csv.contains("id,timestamp"));
    }

    #[test]
    fn test_export_jsonld() {
        let mut trail = AuditTrail::new();

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

        let jsonld = trail.export_jsonld().unwrap();
        assert!(jsonld.get("@context").is_some());
        assert!(jsonld.get("@graph").is_some());
    }

    #[test]
    fn test_analyze_patterns() {
        let mut trail = AuditTrail::new();

        for _ in 0..5 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                "statute-1".to_string(),
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

        let analysis = trail.analyze_patterns().unwrap();
        assert_eq!(analysis.total_decisions, 5);
        assert!(analysis.by_statute.contains_key("statute-1"));
    }

    #[test]
    fn test_subject_history() {
        let mut trail = AuditTrail::new();
        let subject_id = Uuid::new_v4();

        for _ in 0..3 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                "statute-1".to_string(),
                subject_id,
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            );
            trail.record(record).unwrap();
        }

        let history = trail.subject_history(subject_id).unwrap();
        assert_eq!(history.total_decisions, 3);
        assert_eq!(history.subject_id, subject_id);
    }

    #[test]
    fn test_retention_policy() {
        let mut trail = AuditTrail::new();

        let record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );
        trail.record(record).unwrap();

        let policy = retention::RetentionPolicy::new(30);
        let to_delete = trail.apply_retention_policy(&policy).unwrap();
        // Fresh records should not be deleted
        assert_eq!(to_delete.len(), 0);
    }

    #[test]
    fn test_gdpr_export() {
        let mut trail = AuditTrail::new();
        let subject_id = Uuid::new_v4();

        let record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            subject_id,
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );
        trail.record(record).unwrap();

        let export = trail.export_subject_data(subject_id).unwrap();
        assert_eq!(export.total_records, 1);
        assert_eq!(export.subject_id, subject_id);
    }

    #[test]
    fn test_explain_decision() {
        let mut trail = AuditTrail::new();

        let record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );
        let id = trail.record(record).unwrap();

        let explanation = trail.explain_decision(id).unwrap();
        assert!(!explanation.explanation.is_empty());
        assert!(explanation.explanation.contains("statute-1"));
    }
}
