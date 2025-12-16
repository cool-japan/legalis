//! Storage backends for audit trails.

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod jsonl;
pub mod memory;

/// Trait for audit trail storage backends.
pub trait AuditStorage: Send + Sync {
    /// Stores a new audit record.
    fn store(&mut self, record: AuditRecord) -> AuditResult<()>;

    /// Retrieves a record by ID.
    fn get(&self, id: Uuid) -> AuditResult<AuditRecord>;

    /// Retrieves all records.
    fn get_all(&self) -> AuditResult<Vec<AuditRecord>>;

    /// Retrieves records by statute ID.
    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>>;

    /// Retrieves records by subject ID.
    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>>;

    /// Retrieves records within a time range.
    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>>;

    /// Returns the total number of records.
    fn count(&self) -> AuditResult<usize>;

    /// Gets the hash of the last record in the chain.
    fn get_last_hash(&self) -> AuditResult<Option<String>>;

    /// Updates the last hash.
    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()>;
}
