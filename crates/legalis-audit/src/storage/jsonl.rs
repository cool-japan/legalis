//! JSONL (JSON Lines) file-based storage backend.

use crate::storage::AuditStorage;
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// JSONL file-based audit storage.
pub struct JsonlStorage {
    path: PathBuf,
    cache: Arc<RwLock<Vec<AuditRecord>>>,
    last_hash: Arc<RwLock<Option<String>>>,
}

impl JsonlStorage {
    /// Creates a new JSONL storage at the given path.
    pub fn new<P: AsRef<Path>>(path: P) -> AuditResult<Self> {
        let path = path.as_ref().to_path_buf();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut storage = Self {
            path,
            cache: Arc::new(RwLock::new(Vec::new())),
            last_hash: Arc::new(RwLock::new(None)),
        };

        // Load existing records
        storage.load_from_file()?;

        Ok(storage)
    }

    /// Loads all records from the file into the cache.
    fn load_from_file(&mut self) -> AuditResult<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        let mut records = Vec::new();
        let mut last_hash = None;

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let record: AuditRecord = serde_json::from_str(&line)?;
            last_hash = Some(record.record_hash.clone());
            records.push(record);
        }

        let mut cache = self.cache.write().map_err(|e| {
            AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;
        *cache = records;

        let mut hash_lock = self
            .last_hash
            .write()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire hash lock: {}", e)))?;
        *hash_lock = last_hash;

        Ok(())
    }

    /// Appends a record to the file.
    fn append_to_file(&self, record: &AuditRecord) -> AuditResult<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let json = serde_json::to_string(record)?;
        writeln!(file, "{}", json)?;
        file.sync_all()?;

        Ok(())
    }
}

impl AuditStorage for JsonlStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        // Append to file first (for durability)
        self.append_to_file(&record)?;

        // Then update cache
        let mut cache = self.cache.write().map_err(|e| {
            AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;
        cache.push(record);

        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        let cache = self
            .cache
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        cache
            .iter()
            .find(|r| r.id == id)
            .cloned()
            .ok_or(AuditError::RecordNotFound(id))
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        let cache = self
            .cache
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(cache.clone())
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let cache = self
            .cache
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(cache
            .iter()
            .filter(|r| r.statute_id == statute_id)
            .cloned()
            .collect())
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let cache = self
            .cache
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(cache
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
        let cache = self
            .cache
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(cache
            .iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .cloned()
            .collect())
    }

    fn count(&self) -> AuditResult<usize> {
        let cache = self
            .cache
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(cache.len())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    #[test]
    fn test_jsonl_storage() {
        let temp_file = "/tmp/test_audit_trail.jsonl";
        let _ = std::fs::remove_file(temp_file);

        let mut storage = JsonlStorage::new(temp_file).unwrap();

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

        let id = record.id;
        storage.store(record).unwrap();

        let retrieved = storage.get(id).unwrap();
        assert_eq!(retrieved.id, id);

        // Clean up
        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn test_jsonl_persistence() {
        let temp_file = "/tmp/test_audit_persistence.jsonl";
        let _ = std::fs::remove_file(temp_file);

        let id = {
            let mut storage = JsonlStorage::new(temp_file).unwrap();
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
            let id = record.id;
            storage.store(record).unwrap();
            id
        };

        // Load in a new storage instance
        let storage = JsonlStorage::new(temp_file).unwrap();
        let retrieved = storage.get(id).unwrap();
        assert_eq!(retrieved.id, id);

        // Clean up
        let _ = std::fs::remove_file(temp_file);
    }
}
