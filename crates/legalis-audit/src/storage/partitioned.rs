//! Partitioned storage for efficient querying by date and jurisdiction.
//!
//! This module provides partitioned storage that organizes audit records
//! into separate partitions based on date and/or jurisdiction. This allows
//! for more efficient querying and management of large audit trails.
//!
//! ## Partitioning Strategies
//! - **By Date**: Records are partitioned by day, week, month, or year
//! - **By Jurisdiction**: Records are partitioned by legal jurisdiction
//! - **By Both**: Records are partitioned by both date and jurisdiction
//!
//! ## Benefits
//! - Faster queries for date-specific or jurisdiction-specific data
//! - Easier archival and retention management
//! - Better performance for time-series analysis
//! - Simplified compliance with jurisdiction-specific regulations

use crate::storage::AuditStorage;
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// Partitioning strategy for audit records.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PartitionStrategy {
    /// Partition by date only (daily, weekly, monthly, yearly)
    ByDate(DatePartitionGranularity),
    /// Partition by jurisdiction only
    ByJurisdiction,
    /// Partition by both date and jurisdiction
    ByDateAndJurisdiction(DatePartitionGranularity),
}

/// Granularity for date-based partitioning.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatePartitionGranularity {
    /// One partition per day (YYYY-MM-DD)
    Daily,
    /// One partition per week (YYYY-Www)
    Weekly,
    /// One partition per month (YYYY-MM)
    Monthly,
    /// One partition per year (YYYY)
    Yearly,
}

impl DatePartitionGranularity {
    /// Gets the partition key for a given timestamp.
    fn partition_key(&self, timestamp: DateTime<Utc>) -> String {
        match self {
            DatePartitionGranularity::Daily => {
                format!(
                    "{:04}-{:02}-{:02}",
                    timestamp.year(),
                    timestamp.month(),
                    timestamp.day()
                )
            }
            DatePartitionGranularity::Weekly => {
                let week = timestamp.iso_week().week();
                format!("{:04}-W{:02}", timestamp.year(), week)
            }
            DatePartitionGranularity::Monthly => {
                format!("{:04}-{:02}", timestamp.year(), timestamp.month())
            }
            DatePartitionGranularity::Yearly => {
                format!("{:04}", timestamp.year())
            }
        }
    }
}

/// Metadata about a partition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionMetadata {
    /// Partition key (e.g., "2025-01" for monthly, "US-CA" for jurisdiction)
    pub key: String,
    /// Number of records in this partition
    pub record_count: usize,
    /// First record timestamp
    pub first_timestamp: Option<DateTime<Utc>>,
    /// Last record timestamp
    pub last_timestamp: Option<DateTime<Utc>>,
    /// Jurisdictions in this partition (if partitioned by date)
    pub jurisdictions: Vec<String>,
}

/// Configuration for partitioned storage.
#[derive(Debug, Clone)]
pub struct PartitionedConfig {
    /// Base directory for partitions
    pub base_dir: PathBuf,
    /// Partitioning strategy
    pub strategy: PartitionStrategy,
}

impl PartitionedConfig {
    /// Creates a new partitioned storage configuration.
    pub fn new(base_dir: PathBuf, strategy: PartitionStrategy) -> Self {
        Self { base_dir, strategy }
    }
}

/// A single partition containing audit records.
struct Partition {
    /// Partition key
    key: String,
    /// Records in this partition
    records: Vec<AuditRecord>,
    /// Index by record ID
    by_id: HashMap<Uuid, usize>,
    /// Index by statute ID
    by_statute: HashMap<String, Vec<usize>>,
    /// Index by subject ID
    by_subject: HashMap<Uuid, Vec<usize>>,
    /// Last hash in this partition
    last_hash: Option<String>,
}

impl Partition {
    fn new(key: String) -> Self {
        Self {
            key,
            records: Vec::new(),
            by_id: HashMap::new(),
            by_statute: HashMap::new(),
            by_subject: HashMap::new(),
            last_hash: None,
        }
    }

    fn add_record(&mut self, record: AuditRecord) {
        let idx = self.records.len();
        self.by_id.insert(record.id, idx);

        self.by_statute
            .entry(record.statute_id.clone())
            .or_default()
            .push(idx);

        self.by_subject
            .entry(record.subject_id)
            .or_default()
            .push(idx);

        self.last_hash = Some(record.record_hash.clone());
        self.records.push(record);
    }

    fn get(&self, id: Uuid) -> Option<&AuditRecord> {
        self.by_id.get(&id).and_then(|&idx| self.records.get(idx))
    }

    fn get_by_statute(&self, statute_id: &str) -> Vec<&AuditRecord> {
        self.by_statute
            .get(statute_id)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&idx| self.records.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn get_by_subject(&self, subject_id: Uuid) -> Vec<&AuditRecord> {
        self.by_subject
            .get(&subject_id)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&idx| self.records.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn metadata(&self) -> PartitionMetadata {
        let first_timestamp = self.records.first().map(|r| r.timestamp);
        let last_timestamp = self.records.last().map(|r| r.timestamp);

        // Extract unique jurisdictions (from metadata if available)
        let jurisdictions: Vec<String> = self
            .records
            .iter()
            .filter_map(|r| r.context.metadata.get("jurisdiction").cloned())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        PartitionMetadata {
            key: self.key.clone(),
            record_count: self.records.len(),
            first_timestamp,
            last_timestamp,
            jurisdictions,
        }
    }
}

/// Partitioned storage backend.
pub struct PartitionedStorage {
    config: PartitionedConfig,
    /// Map from partition key to partition
    partitions: HashMap<String, Partition>,
}

impl PartitionedStorage {
    /// Creates a new partitioned storage backend.
    pub fn new(config: PartitionedConfig) -> AuditResult<Self> {
        // Create base directory if it doesn't exist
        if !config.base_dir.exists() {
            std::fs::create_dir_all(&config.base_dir)?;
        }

        Ok(Self {
            config,
            partitions: HashMap::new(),
        })
    }

    /// Gets the partition key for a record.
    fn get_partition_key(&self, record: &AuditRecord) -> String {
        match &self.config.strategy {
            PartitionStrategy::ByDate(granularity) => granularity.partition_key(record.timestamp),
            PartitionStrategy::ByJurisdiction => {
                // Extract jurisdiction from record metadata
                record
                    .context
                    .metadata
                    .get("jurisdiction")
                    .cloned()
                    .unwrap_or_else(|| "UNKNOWN".to_string())
            }
            PartitionStrategy::ByDateAndJurisdiction(granularity) => {
                let date_key = granularity.partition_key(record.timestamp);
                let jurisdiction = record
                    .context
                    .metadata
                    .get("jurisdiction")
                    .cloned()
                    .unwrap_or_else(|| "UNKNOWN".to_string());
                format!("{}_{}", date_key, jurisdiction)
            }
        }
    }

    /// Gets or creates a partition.
    fn get_or_create_partition(&mut self, key: &str) -> &mut Partition {
        self.partitions
            .entry(key.to_string())
            .or_insert_with(|| Partition::new(key.to_string()))
    }

    /// Gets a partition by key (read-only).
    fn get_partition(&self, key: &str) -> Option<&Partition> {
        self.partitions.get(key)
    }

    /// Lists all partition keys.
    pub fn list_partitions(&self) -> Vec<String> {
        self.partitions.keys().cloned().collect()
    }

    /// Gets metadata for all partitions.
    pub fn partition_metadata(&self) -> Vec<PartitionMetadata> {
        self.partitions.values().map(|p| p.metadata()).collect()
    }

    /// Gets metadata for a specific partition.
    pub fn get_partition_metadata(&self, key: &str) -> Option<PartitionMetadata> {
        self.get_partition(key).map(|p| p.metadata())
    }

    /// Loads a partition from disk.
    #[allow(dead_code)]
    fn load_partition(&mut self, key: &str) -> AuditResult<()> {
        let partition_file = self.config.base_dir.join(format!("{}.json", key));
        if partition_file.exists() {
            let data = std::fs::read_to_string(&partition_file)?;
            let records: Vec<AuditRecord> = serde_json::from_str(&data)?;

            let mut partition = Partition::new(key.to_string());
            for record in records {
                partition.add_record(record);
            }
            self.partitions.insert(key.to_string(), partition);
        }
        Ok(())
    }

    /// Saves a partition to disk.
    #[allow(dead_code)]
    fn save_partition(&self, key: &str) -> AuditResult<()> {
        if let Some(partition) = self.get_partition(key) {
            let partition_file = self.config.base_dir.join(format!("{}.json", key));
            let data = serde_json::to_string_pretty(&partition.records)?;
            std::fs::write(&partition_file, data)?;
        }
        Ok(())
    }
}

impl AuditStorage for PartitionedStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        let key = self.get_partition_key(&record);
        let partition = self.get_or_create_partition(&key);
        partition.add_record(record);
        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        for partition in self.partitions.values() {
            if let Some(record) = partition.get(id) {
                return Ok(record.clone());
            }
        }
        Err(AuditError::RecordNotFound(id))
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        let mut all_records = Vec::new();
        for partition in self.partitions.values() {
            all_records.extend(partition.records.iter().cloned());
        }
        Ok(all_records)
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let mut results = Vec::new();
        for partition in self.partitions.values() {
            results.extend(partition.get_by_statute(statute_id).into_iter().cloned());
        }
        Ok(results)
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let mut results = Vec::new();
        for partition in self.partitions.values() {
            results.extend(partition.get_by_subject(subject_id).into_iter().cloned());
        }
        Ok(results)
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        let all_records = self.get_all()?;
        Ok(all_records
            .into_iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .collect())
    }

    fn count(&self) -> AuditResult<usize> {
        Ok(self.partitions.values().map(|p| p.records.len()).sum())
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        // Get the last hash from the most recent partition
        let mut last_hash = None;
        let mut latest_timestamp = None;

        for partition in self.partitions.values() {
            if let Some(last_record) = partition.records.last()
                && (latest_timestamp.is_none() || last_record.timestamp > latest_timestamp.unwrap())
            {
                latest_timestamp = Some(last_record.timestamp);
                last_hash = partition.last_hash.clone();
            }
        }

        Ok(last_hash)
    }

    fn set_last_hash(&mut self, _hash: Option<String>) -> AuditResult<()> {
        // Not directly applicable to partitioned storage
        // Each partition maintains its own last hash
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use chrono::TimeZone;
    use std::collections::HashMap as StdHashMap;

    fn create_test_record_with_date(
        statute_id: &str,
        timestamp: DateTime<Utc>,
        jurisdiction: Option<&str>,
    ) -> AuditRecord {
        let mut context = DecisionContext::default();
        if let Some(j) = jurisdiction {
            context
                .metadata
                .insert("jurisdiction".to_string(), j.to_string());
        }

        let mut record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute_id.to_string(),
            Uuid::new_v4(),
            context,
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        );
        record.timestamp = timestamp;
        record
    }

    #[test]
    fn test_date_partition_granularity() {
        let timestamp = Utc::now();

        let daily = DatePartitionGranularity::Daily;
        let monthly = DatePartitionGranularity::Monthly;
        let yearly = DatePartitionGranularity::Yearly;

        assert!(daily.partition_key(timestamp).len() == 10); // YYYY-MM-DD
        assert!(monthly.partition_key(timestamp).len() == 7); // YYYY-MM
        assert!(yearly.partition_key(timestamp).len() == 4); // YYYY
    }

    #[test]
    fn test_partitioned_storage_by_date() {
        let temp_dir = std::env::temp_dir().join("test_partitioned_date");
        let config = PartitionedConfig::new(
            temp_dir,
            PartitionStrategy::ByDate(DatePartitionGranularity::Monthly),
        );
        let mut storage = PartitionedStorage::new(config).unwrap();

        // Create records in different months
        let jan = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
        let feb = Utc.with_ymd_and_hms(2025, 2, 15, 0, 0, 0).unwrap();

        let record1 = create_test_record_with_date("statute-1", jan, None);
        let record2 = create_test_record_with_date("statute-2", feb, None);

        storage.store(record1).unwrap();
        storage.store(record2).unwrap();

        // Should have 2 partitions
        let partitions = storage.list_partitions();
        assert_eq!(partitions.len(), 2);
        assert!(partitions.contains(&"2025-01".to_string()));
        assert!(partitions.contains(&"2025-02".to_string()));
    }

    #[test]
    fn test_partitioned_storage_by_jurisdiction() {
        let temp_dir = std::env::temp_dir().join("test_partitioned_jurisdiction");
        let config = PartitionedConfig::new(temp_dir, PartitionStrategy::ByJurisdiction);
        let mut storage = PartitionedStorage::new(config).unwrap();

        let record1 = create_test_record_with_date("statute-1", Utc::now(), Some("US-CA"));
        let record2 = create_test_record_with_date("statute-2", Utc::now(), Some("US-NY"));

        storage.store(record1).unwrap();
        storage.store(record2).unwrap();

        let partitions = storage.list_partitions();
        assert_eq!(partitions.len(), 2);
        assert!(partitions.contains(&"US-CA".to_string()));
        assert!(partitions.contains(&"US-NY".to_string()));
    }

    #[test]
    fn test_partitioned_storage_by_both() {
        let temp_dir = std::env::temp_dir().join("test_partitioned_both");
        let config = PartitionedConfig::new(
            temp_dir,
            PartitionStrategy::ByDateAndJurisdiction(DatePartitionGranularity::Monthly),
        );
        let mut storage = PartitionedStorage::new(config).unwrap();

        let jan = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
        let record = create_test_record_with_date("statute-1", jan, Some("US-CA"));

        storage.store(record).unwrap();

        let partitions = storage.list_partitions();
        assert_eq!(partitions.len(), 1);
        assert!(partitions[0].contains("2025-01"));
        assert!(partitions[0].contains("US-CA"));
    }

    #[test]
    fn test_partition_metadata() {
        let temp_dir = std::env::temp_dir().join("test_partition_metadata");
        let config = PartitionedConfig::new(
            temp_dir,
            PartitionStrategy::ByDate(DatePartitionGranularity::Monthly),
        );
        let mut storage = PartitionedStorage::new(config).unwrap();

        let jan = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
        let record1 = create_test_record_with_date("statute-1", jan, Some("US-CA"));
        let record2 = create_test_record_with_date("statute-2", jan, Some("US-NY"));

        storage.store(record1).unwrap();
        storage.store(record2).unwrap();

        let metadata = storage.get_partition_metadata("2025-01").unwrap();
        assert_eq!(metadata.record_count, 2);
        assert!(metadata.jurisdictions.contains(&"US-CA".to_string()));
        assert!(metadata.jurisdictions.contains(&"US-NY".to_string()));
    }

    #[test]
    fn test_partitioned_storage_get_all() {
        let temp_dir = std::env::temp_dir().join("test_partitioned_get_all");
        let config = PartitionedConfig::new(
            temp_dir,
            PartitionStrategy::ByDate(DatePartitionGranularity::Monthly),
        );
        let mut storage = PartitionedStorage::new(config).unwrap();

        let jan = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
        let feb = Utc.with_ymd_and_hms(2025, 2, 15, 0, 0, 0).unwrap();

        for _ in 0..3 {
            storage
                .store(create_test_record_with_date("statute-1", jan, None))
                .unwrap();
        }
        for _ in 0..2 {
            storage
                .store(create_test_record_with_date("statute-2", feb, None))
                .unwrap();
        }

        let all_records = storage.get_all().unwrap();
        assert_eq!(all_records.len(), 5);
    }

    #[test]
    fn test_partitioned_storage_count() {
        let temp_dir = std::env::temp_dir().join("test_partitioned_count");
        let config = PartitionedConfig::new(
            temp_dir,
            PartitionStrategy::ByDate(DatePartitionGranularity::Yearly),
        );
        let mut storage = PartitionedStorage::new(config).unwrap();

        let timestamp = Utc::now();
        for i in 0..10 {
            storage
                .store(create_test_record_with_date(
                    &format!("statute-{}", i),
                    timestamp,
                    None,
                ))
                .unwrap();
        }

        assert_eq!(storage.count().unwrap(), 10);
    }
}
