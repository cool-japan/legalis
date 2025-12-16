//! In-memory storage backend.

use crate::storage::AuditStorage;
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// In-memory audit storage.
#[derive(Clone)]
pub struct MemoryStorage {
    records: Arc<RwLock<Vec<AuditRecord>>>,
    last_hash: Arc<RwLock<Option<String>>>,
}

impl MemoryStorage {
    /// Creates a new in-memory storage.
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(Vec::new())),
            last_hash: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditStorage for MemoryStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        let mut records = self.records.write().map_err(|e| {
            AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;
        records.push(record);
        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
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

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(records.clone())
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
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

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
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

    fn get_by_time_range(
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

    fn count(&self) -> AuditResult<usize> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(records.len())
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        let hash = self
            .last_hash
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire hash lock: {}", e)))?;
        Ok(hash.clone())
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        let mut last_hash = self
            .last_hash
            .write()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire hash lock: {}", e)))?;
        *last_hash = hash;
        Ok(())
    }
}
