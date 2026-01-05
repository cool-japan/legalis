//! Append-only log storage for forensic audit trails.
//!
//! This backend provides:
//! - Append-only writes (no modifications allowed)
//! - Forensic guarantees (tamper-evident)
//! - Optional log rotation
//! - Fast lookups via in-memory index

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Append-only log storage backend.
///
/// Records are written to an append-only log file and never modified.
/// An in-memory index provides fast lookups by ID, statute, subject, and time.
///
/// # Example
///
/// ```
/// use legalis_audit::storage::append_only::AppendOnlyStorage;
/// use tempfile::tempdir;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let dir = tempdir()?;
/// let log_path = dir.path().join("audit.aol");
/// let storage = AppendOnlyStorage::new(&log_path)?;
/// # Ok(())
/// # }
/// ```
pub struct AppendOnlyStorage {
    log_path: PathBuf,
    log_file: File,
    index: LogIndex,
    last_hash: Option<String>,
    max_file_size: Option<u64>,
    rotation_count: usize,
}

#[derive(Default)]
struct LogIndex {
    by_id: HashMap<Uuid, u64>,
    by_statute: HashMap<String, Vec<Uuid>>,
    by_subject: HashMap<Uuid, Vec<Uuid>>,
    records: Vec<AuditRecord>,
}

impl AppendOnlyStorage {
    /// Creates a new append-only storage backend.
    ///
    /// If the log file exists, it will be loaded and indexed.
    pub fn new<P: AsRef<Path>>(log_path: P) -> AuditResult<Self> {
        let log_path = log_path.as_ref().to_path_buf();
        let file_exists = log_path.exists();

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&log_path)?;

        let mut storage = Self {
            log_path,
            log_file,
            index: LogIndex::default(),
            last_hash: None,
            max_file_size: None,
            rotation_count: 0,
        };

        if file_exists {
            storage.rebuild_index()?;
        }

        Ok(storage)
    }

    /// Creates a new append-only storage with log rotation.
    ///
    /// When the log file exceeds `max_size_bytes`, it will be rotated.
    pub fn with_rotation<P: AsRef<Path>>(log_path: P, max_size_bytes: u64) -> AuditResult<Self> {
        let mut storage = Self::new(log_path)?;
        storage.max_file_size = Some(max_size_bytes);
        Ok(storage)
    }

    /// Rebuilds the in-memory index from the log file.
    fn rebuild_index(&mut self) -> AuditResult<()> {
        let file = File::open(&self.log_path)?;
        let reader = BufReader::new(file);

        for (offset, line) in reader.lines().enumerate() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            let record: AuditRecord = serde_json::from_str(&line)?;
            self.index.add_record(record.clone(), offset as u64);
            self.last_hash = Some(record.record_hash.clone());
        }

        Ok(())
    }

    /// Checks if log rotation is needed and performs it.
    fn check_rotation(&mut self) -> AuditResult<()> {
        if let Some(max_size) = self.max_file_size {
            let metadata = self.log_file.metadata()?;
            if metadata.len() >= max_size {
                self.rotate_log()?;
            }
        }
        Ok(())
    }

    /// Rotates the log file.
    fn rotate_log(&mut self) -> AuditResult<()> {
        self.rotation_count += 1;
        let rotated_path = self
            .log_path
            .with_extension(format!("aol.{}", self.rotation_count));

        // Flush and sync current file before rotation
        self.log_file.flush()?;
        self.log_file.sync_all()?;

        // Rename current log to rotated name
        std::fs::rename(&self.log_path, &rotated_path)?;

        // Create new log file (old file handle will be dropped automatically)
        self.log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&self.log_path)?;

        tracing::info!("Log rotated: {} -> {:?}", self.rotation_count, rotated_path);

        Ok(())
    }

    /// Gets the current log file size.
    pub fn log_size(&self) -> u64 {
        self.log_file.metadata().map(|m| m.len()).unwrap_or(0)
    }

    /// Gets the total number of rotations performed.
    pub fn rotation_count(&self) -> usize {
        self.rotation_count
    }
}

impl LogIndex {
    fn add_record(&mut self, record: AuditRecord, offset: u64) {
        self.by_id.insert(record.id, offset);

        self.by_statute
            .entry(record.statute_id.clone())
            .or_default()
            .push(record.id);

        self.by_subject
            .entry(record.subject_id)
            .or_default()
            .push(record.id);

        self.records.push(record);
    }
}

impl super::AuditStorage for AppendOnlyStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        // Check rotation before writing
        self.check_rotation()?;

        // Serialize record to JSON
        let json = serde_json::to_string(&record)?;

        // Get current offset before writing
        let offset = self.log_file.seek(SeekFrom::End(0))?;

        // Append to log
        writeln!(&mut self.log_file, "{}", json)?;
        self.log_file.flush()?;

        // Update index
        self.index.add_record(record, offset);

        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        self.index
            .records
            .iter()
            .find(|r| r.id == id)
            .cloned()
            .ok_or(AuditError::RecordNotFound(id))
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        Ok(self.index.records.clone())
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let ids = self.index.by_statute.get(statute_id);
        match ids {
            Some(ids) => Ok(self
                .index
                .records
                .iter()
                .filter(|r| ids.contains(&r.id))
                .cloned()
                .collect()),
            None => Ok(Vec::new()),
        }
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let ids = self.index.by_subject.get(&subject_id);
        match ids {
            Some(ids) => Ok(self
                .index
                .records
                .iter()
                .filter(|r| ids.contains(&r.id))
                .cloned()
                .collect()),
            None => Ok(Vec::new()),
        }
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        Ok(self
            .index
            .records
            .iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .cloned()
            .collect())
    }

    fn count(&self) -> AuditResult<usize> {
        Ok(self.index.records.len())
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        Ok(self.last_hash.clone())
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        self.last_hash = hash;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::AuditStorage;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_append_only_basic() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.aol");

        let mut storage = AppendOnlyStorage::new(&log_path).unwrap();

        let record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );

        let id = record.id;
        storage.store(record).unwrap();

        assert_eq!(storage.count().unwrap(), 1);
        assert!(storage.get(id).is_ok());
    }

    #[test]
    fn test_append_only_persistence() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.aol");

        // Write records
        {
            let mut storage = AppendOnlyStorage::new(&log_path).unwrap();
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
                        effect_applied: "approved".to_string(),
                        parameters: HashMap::new(),
                    },
                    None,
                );
                storage.store(record).unwrap();
            }
        }

        // Reload and verify
        {
            let storage = AppendOnlyStorage::new(&log_path).unwrap();
            assert_eq!(storage.count().unwrap(), 5);
            assert!(!storage.get_by_statute("statute-2").unwrap().is_empty());
        }
    }

    #[test]
    fn test_log_rotation() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.aol");

        // Create storage with small max size to force rotation
        let mut storage = AppendOnlyStorage::with_rotation(&log_path, 500).unwrap();

        // Write enough records to trigger rotation
        for i in 0..10 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                format!("statute-{}", i),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            );
            storage.store(record).unwrap();
        }

        // At least one rotation should have occurred
        assert!(storage.rotation_count() > 0);
    }

    #[test]
    fn test_query_operations() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("audit.aol");

        let mut storage = AppendOnlyStorage::new(&log_path).unwrap();
        let subject_id = Uuid::new_v4();

        for i in 0..3 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                "statute-test".to_string(),
                subject_id,
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: format!("result-{}", i),
                    parameters: HashMap::new(),
                },
                None,
            );
            storage.store(record).unwrap();
        }

        let by_statute = storage.get_by_statute("statute-test").unwrap();
        assert_eq!(by_statute.len(), 3);

        let by_subject = storage.get_by_subject(subject_id).unwrap();
        assert_eq!(by_subject.len(), 3);
    }
}
