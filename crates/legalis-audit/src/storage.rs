//! Storage backends for audit trails.

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod append_only;
pub mod cached;
pub mod encrypted;
pub mod jsonl;
pub mod memory;
pub mod partition_tolerant;
pub mod partitioned;
pub mod postgres;
pub mod s3;
pub mod sqlite;
pub mod tiered;

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

    /// Removes a record by ID, returning `true` if a record was removed.
    ///
    /// This is an *optional* capability. The default implementation is a no-op
    /// that returns `Ok(false)` ("removal not supported by this backend"), so
    /// append-only / forensic backends stay immutable and every existing
    /// implementation keeps compiling unchanged.
    ///
    /// Backends that own their storage (e.g. [`memory::MemoryStorage`]) override
    /// it to physically delete the record. It is used by
    /// [`crate::scale::MultiTierStore`] to physically migrate a record out of
    /// its source tier; when a source tier does not support removal the
    /// migration degrades gracefully to a logical (copy-only) migration.
    fn remove(&mut self, _id: Uuid) -> AuditResult<bool> {
        Ok(false)
    }
}
