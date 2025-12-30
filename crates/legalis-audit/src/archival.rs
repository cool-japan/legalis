//! Archival functionality for audit records.
//!
//! This module provides functionality to archive old audit records
//! for long-term storage, reducing the size of active audit trails
//! while maintaining access to historical data.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Duration, Utc};
use flate2::{Compression, write::GzEncoder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Archive policy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivePolicy {
    /// Minimum age of records to archive (in days)
    pub min_age_days: i64,
    /// Archive records by statute ID (specific statutes to archive)
    pub statute_filters: Vec<String>,
    /// Compression level (0-9)
    pub compression_level: u32,
    /// Maximum records per archive file
    pub max_records_per_file: Option<usize>,
}

impl ArchivePolicy {
    /// Creates a new archive policy.
    pub fn new(min_age_days: i64) -> Self {
        Self {
            min_age_days,
            statute_filters: Vec::new(),
            compression_level: 6,
            max_records_per_file: Some(10000),
        }
    }

    /// Sets the compression level (0-9).
    pub fn with_compression(mut self, level: u32) -> Self {
        self.compression_level = level.min(9);
        self
    }

    /// Adds statute ID filters for selective archival.
    pub fn with_statute_filter(mut self, statute_id: String) -> Self {
        self.statute_filters.push(statute_id);
        self
    }

    /// Sets the maximum number of records per archive file.
    pub fn with_max_records(mut self, max: usize) -> Self {
        self.max_records_per_file = Some(max);
        self
    }

    /// Determines if a record should be archived based on this policy.
    pub fn should_archive(&self, record: &AuditRecord) -> bool {
        let age = Utc::now() - record.timestamp;
        let min_age = Duration::days(self.min_age_days);

        // Check age requirement
        if age < min_age {
            return false;
        }

        // Check statute filters if any
        if !self.statute_filters.is_empty() {
            return self.statute_filters.contains(&record.statute_id);
        }

        true
    }
}

impl Default for ArchivePolicy {
    fn default() -> Self {
        Self::new(365) // Default: archive records older than 1 year
    }
}

/// Archive metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveMetadata {
    /// Archive ID
    pub archive_id: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Number of records in this archive
    pub record_count: usize,
    /// Time range of archived records
    pub time_range: (DateTime<Utc>, DateTime<Utc>),
    /// Compression algorithm used
    pub compression: String,
    /// Archive file path
    pub file_path: PathBuf,
    /// Integrity hash of the archive
    pub integrity_hash: String,
}

/// Archive manager for handling audit record archival.
pub struct ArchiveManager {
    archive_dir: PathBuf,
    policy: ArchivePolicy,
    metadata_index: HashMap<String, ArchiveMetadata>,
}

impl ArchiveManager {
    /// Creates a new archive manager.
    pub fn new<P: AsRef<Path>>(archive_dir: P, policy: ArchivePolicy) -> AuditResult<Self> {
        let archive_dir = archive_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&archive_dir)?;

        let mut manager = Self {
            archive_dir,
            policy,
            metadata_index: HashMap::new(),
        };

        manager.load_metadata_index()?;
        Ok(manager)
    }

    /// Archives records that match the policy.
    pub fn archive_records(&mut self, records: &[AuditRecord]) -> AuditResult<ArchiveMetadata> {
        let records_to_archive: Vec<_> = records
            .iter()
            .filter(|r| self.policy.should_archive(r))
            .cloned()
            .collect();

        if records_to_archive.is_empty() {
            return Err(AuditError::InvalidRecord(
                "No records match archival policy".to_string(),
            ));
        }

        let archive_id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let file_name = format!("archive_{}.jsonl.gz", timestamp.format("%Y%m%d_%H%M%S"));
        let file_path = self.archive_dir.join(&file_name);

        // Determine time range
        let mut min_time = records_to_archive[0].timestamp;
        let mut max_time = records_to_archive[0].timestamp;
        for record in &records_to_archive {
            if record.timestamp < min_time {
                min_time = record.timestamp;
            }
            if record.timestamp > max_time {
                max_time = record.timestamp;
            }
        }

        // Write compressed archive
        let file = std::fs::File::create(&file_path)?;
        let compression = Compression::new(self.policy.compression_level);
        let mut encoder = GzEncoder::new(file, compression);

        for record in &records_to_archive {
            let json = serde_json::to_string(record)?;
            writeln!(encoder, "{}", json)?;
        }

        encoder.finish()?;

        // Compute integrity hash
        let archive_data = std::fs::read(&file_path)?;
        let integrity_hash = format!("{:x}", compute_hash(&archive_data));

        // Create metadata
        let metadata = ArchiveMetadata {
            archive_id: archive_id.clone(),
            created_at: timestamp,
            record_count: records_to_archive.len(),
            time_range: (min_time, max_time),
            compression: "gzip".to_string(),
            file_path: file_path.clone(),
            integrity_hash,
        };

        // Save metadata
        self.metadata_index
            .insert(archive_id.clone(), metadata.clone());
        self.save_metadata_index()?;

        tracing::info!(
            "Archived {} records to {}",
            records_to_archive.len(),
            file_name
        );

        Ok(metadata)
    }

    /// Retrieves records from a specific archive.
    pub fn retrieve_archive(&self, archive_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let metadata = self.metadata_index.get(archive_id).ok_or_else(|| {
            AuditError::InvalidRecord(format!("Archive not found: {}", archive_id))
        })?;

        // Verify integrity
        let archive_data = std::fs::read(&metadata.file_path)?;
        let computed_hash = format!("{:x}", compute_hash(&archive_data));
        if computed_hash != metadata.integrity_hash {
            return Err(AuditError::TamperDetected(format!(
                "Archive {} failed integrity check",
                archive_id
            )));
        }

        // Decompress and read records
        let file = std::fs::File::open(&metadata.file_path)?;
        let decoder = flate2::read::GzDecoder::new(file);
        let reader = std::io::BufReader::new(decoder);

        let mut records = Vec::new();
        for line in std::io::BufRead::lines(reader) {
            let line = line?;
            if !line.is_empty() {
                let record: AuditRecord = serde_json::from_str(&line)?;
                records.push(record);
            }
        }

        Ok(records)
    }

    /// Lists all available archives.
    pub fn list_archives(&self) -> Vec<&ArchiveMetadata> {
        self.metadata_index.values().collect()
    }

    /// Searches archives by time range.
    pub fn find_archives_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&ArchiveMetadata> {
        self.metadata_index
            .values()
            .filter(|m| {
                let (archive_start, archive_end) = m.time_range;
                // Check if ranges overlap
                !(archive_end < start || archive_start > end)
            })
            .collect()
    }

    /// Verifies the integrity of all archives.
    pub fn verify_all_archives(&self) -> AuditResult<bool> {
        for metadata in self.metadata_index.values() {
            let archive_data = std::fs::read(&metadata.file_path)?;
            let computed_hash = format!("{:x}", compute_hash(&archive_data));
            if computed_hash != metadata.integrity_hash {
                return Err(AuditError::TamperDetected(format!(
                    "Archive {} failed integrity check",
                    metadata.archive_id
                )));
            }
        }
        Ok(true)
    }

    /// Loads the metadata index from disk.
    fn load_metadata_index(&mut self) -> AuditResult<()> {
        let index_path = self.archive_dir.join("archive_index.json");
        if !index_path.exists() {
            return Ok(());
        }

        let data = std::fs::read_to_string(&index_path)?;
        self.metadata_index = serde_json::from_str(&data)?;
        Ok(())
    }

    /// Saves the metadata index to disk.
    fn save_metadata_index(&self) -> AuditResult<()> {
        let index_path = self.archive_dir.join("archive_index.json");
        let data = serde_json::to_string_pretty(&self.metadata_index)?;
        std::fs::write(&index_path, data)?;
        Ok(())
    }
}

/// Computes a simple hash for integrity checking.
fn compute_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0;
    for &byte in data {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_archive_policy() {
        let policy = ArchivePolicy::new(30);

        // Recent record should not be archived
        let recent_record = create_test_record(Utc::now());
        assert!(!policy.should_archive(&recent_record));

        // Old record should be archived
        let old_record = create_test_record(Utc::now() - Duration::days(60));
        assert!(policy.should_archive(&old_record));
    }

    #[test]
    fn test_archive_with_statute_filter() {
        let policy = ArchivePolicy::new(30).with_statute_filter("statute-1".to_string());

        let old_record =
            create_test_record_with_statute(Utc::now() - Duration::days(60), "statute-1");
        assert!(policy.should_archive(&old_record));

        let other_old_record =
            create_test_record_with_statute(Utc::now() - Duration::days(60), "statute-2");
        assert!(!policy.should_archive(&other_old_record));
    }

    #[test]
    fn test_archive_manager() {
        let temp_dir = std::env::temp_dir().join(format!("audit_archive_{}", Uuid::new_v4()));
        let policy = ArchivePolicy::new(30);
        let mut manager = ArchiveManager::new(&temp_dir, policy).unwrap();

        // Create old records
        let records: Vec<_> = (0..5)
            .map(|i| {
                create_test_record_with_statute(
                    Utc::now() - Duration::days(60),
                    &format!("statute-{}", i),
                )
            })
            .collect();

        // Archive records
        let metadata = manager.archive_records(&records).unwrap();
        assert_eq!(metadata.record_count, 5);

        // Retrieve archived records
        let retrieved = manager.retrieve_archive(&metadata.archive_id).unwrap();
        assert_eq!(retrieved.len(), 5);

        // Verify all archives
        assert!(manager.verify_all_archives().unwrap());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_archive_time_range_search() {
        let temp_dir = std::env::temp_dir().join(format!("audit_archive_{}", Uuid::new_v4()));
        let policy = ArchivePolicy::new(30);
        let mut manager = ArchiveManager::new(&temp_dir, policy).unwrap();

        // Create records in different time ranges
        let old_records: Vec<_> = (0..3)
            .map(|_| create_test_record(Utc::now() - Duration::days(90)))
            .collect();
        let very_old_records: Vec<_> = (0..2)
            .map(|_| create_test_record(Utc::now() - Duration::days(180)))
            .collect();

        manager.archive_records(&old_records).unwrap();
        manager.archive_records(&very_old_records).unwrap();

        // Search by time range
        let start = Utc::now() - Duration::days(100);
        let end = Utc::now() - Duration::days(80);
        let found = manager.find_archives_by_time_range(start, end);
        assert_eq!(found.len(), 1);

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    fn create_test_record(timestamp: DateTime<Utc>) -> AuditRecord {
        create_test_record_with_statute(timestamp, "test-statute")
    }

    fn create_test_record_with_statute(timestamp: DateTime<Utc>, statute_id: &str) -> AuditRecord {
        let mut record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute_id.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );
        record.timestamp = timestamp;
        record
    }
}
