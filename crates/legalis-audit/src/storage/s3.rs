//! S3-compatible object storage backend for audit trails.
//!
//! This module provides an S3-compatible storage backend that can work with:
//! - Amazon S3
//! - MinIO
//! - Ceph
//! - DigitalOcean Spaces
//! - Any S3-compatible object storage
//!
//! Records are stored as individual JSON objects with the record ID as the key.
//! Metadata and indices are maintained in separate objects for efficient querying.

use crate::storage::AuditStorage;
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for S3-compatible storage.
#[derive(Debug, Clone)]
pub struct S3Config {
    /// S3 endpoint URL (e.g., "<https://s3.amazonaws.com>" or "<http://localhost:9000>" for MinIO)
    pub endpoint: String,
    /// S3 bucket name
    pub bucket: String,
    /// Access key ID
    pub access_key: String,
    /// Secret access key
    pub secret_key: String,
    /// Region (e.g., "us-east-1")
    pub region: String,
    /// Path prefix for all audit records (e.g., "audit-logs/")
    pub prefix: String,
}

impl S3Config {
    /// Creates a new S3 configuration.
    pub fn new(
        endpoint: String,
        bucket: String,
        access_key: String,
        secret_key: String,
        region: String,
        prefix: String,
    ) -> Self {
        Self {
            endpoint,
            bucket,
            access_key,
            secret_key,
            region,
            prefix,
        }
    }

    /// Creates a configuration for Amazon S3.
    pub fn aws(bucket: String, access_key: String, secret_key: String, region: String) -> Self {
        Self::new(
            format!("https://s3.{}.amazonaws.com", region),
            bucket,
            access_key,
            secret_key,
            region,
            String::new(),
        )
    }

    /// Creates a configuration for MinIO.
    pub fn minio(endpoint: String, bucket: String, access_key: String, secret_key: String) -> Self {
        Self::new(
            endpoint,
            bucket,
            access_key,
            secret_key,
            "us-east-1".to_string(),
            String::new(),
        )
    }
}

/// In-memory index for efficient querying.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct S3Index {
    /// Map from record ID to S3 key
    records: HashMap<Uuid, String>,
    /// Map from statute ID to list of record IDs
    by_statute: HashMap<String, Vec<Uuid>>,
    /// Map from subject ID to list of record IDs
    by_subject: HashMap<Uuid, Vec<Uuid>>,
    /// List of all record IDs sorted by timestamp
    chronological: Vec<Uuid>,
    /// Last hash in the chain
    last_hash: Option<String>,
}

impl S3Index {
    fn new() -> Self {
        Self {
            records: HashMap::new(),
            by_statute: HashMap::new(),
            by_subject: HashMap::new(),
            chronological: Vec::new(),
            last_hash: None,
        }
    }

    fn add_record(&mut self, record: &AuditRecord, key: String) {
        self.records.insert(record.id, key);

        self.by_statute
            .entry(record.statute_id.clone())
            .or_default()
            .push(record.id);

        self.by_subject
            .entry(record.subject_id)
            .or_default()
            .push(record.id);

        self.chronological.push(record.id);
    }
}

/// S3-compatible object storage backend.
///
/// # Note
/// This is a simplified implementation that uses in-memory storage
/// for demonstration purposes. In production, you would use a proper
/// S3 client library like `rusoto_s3` or `aws-sdk-s3`.
pub struct S3Storage {
    config: S3Config,
    index: S3Index,
    /// In-memory cache of records (for demonstration)
    cache: HashMap<Uuid, AuditRecord>,
}

impl S3Storage {
    /// Creates a new S3 storage backend.
    ///
    /// # Note
    /// This implementation uses in-memory storage for demonstration.
    /// In production, replace this with actual S3 API calls.
    #[allow(dead_code)]
    pub fn new(config: S3Config) -> AuditResult<Self> {
        Ok(Self {
            config,
            index: S3Index::new(),
            cache: HashMap::new(),
        })
    }

    /// Gets the S3 key for a record.
    fn get_record_key(&self, record_id: Uuid) -> String {
        format!("{}records/{}.json", self.config.prefix, record_id)
    }

    /// Gets the S3 key for the index.
    #[allow(dead_code)]
    fn get_index_key(&self) -> String {
        format!("{}index.json", self.config.prefix)
    }

    /// Simulates uploading a record to S3.
    ///
    /// In production, this would use the S3 API to PUT the object.
    fn upload_record(&mut self, record: &AuditRecord) -> AuditResult<()> {
        let key = self.get_record_key(record.id);
        // In production: use S3 client to PUT the record as JSON
        // For now, store in memory cache
        self.cache.insert(record.id, record.clone());
        self.index.add_record(record, key);
        Ok(())
    }

    /// Simulates downloading a record from S3.
    ///
    /// In production, this would use the S3 API to GET the object.
    fn download_record(&self, record_id: Uuid) -> AuditResult<AuditRecord> {
        // In production: use S3 client to GET the record JSON
        // For now, retrieve from memory cache
        self.cache
            .get(&record_id)
            .cloned()
            .ok_or_else(|| AuditError::RecordNotFound(record_id))
    }

    /// Simulates listing objects with a prefix.
    ///
    /// In production, this would use the S3 API to LIST objects.
    fn list_records(&self) -> Vec<Uuid> {
        self.index.chronological.clone()
    }
}

impl AuditStorage for S3Storage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        self.upload_record(&record)?;
        self.index.last_hash = Some(record.record_hash.clone());
        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        self.download_record(id)
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        let record_ids = self.list_records();
        let mut records = Vec::new();
        for id in record_ids {
            if let Ok(record) = self.download_record(id) {
                records.push(record);
            }
        }
        Ok(records)
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let record_ids = self
            .index
            .by_statute
            .get(statute_id)
            .cloned()
            .unwrap_or_default();

        let mut records = Vec::new();
        for id in record_ids {
            if let Ok(record) = self.download_record(id) {
                records.push(record);
            }
        }
        Ok(records)
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let record_ids = self
            .index
            .by_subject
            .get(&subject_id)
            .cloned()
            .unwrap_or_default();

        let mut records = Vec::new();
        for id in record_ids {
            if let Ok(record) = self.download_record(id) {
                records.push(record);
            }
        }
        Ok(records)
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
        Ok(self.index.records.len())
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        Ok(self.index.last_hash.clone())
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        self.index.last_hash = hash;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

    fn create_test_config() -> S3Config {
        S3Config::minio(
            "http://localhost:9000".to_string(),
            "test-bucket".to_string(),
            "minioadmin".to_string(),
            "minioadmin".to_string(),
        )
    }

    fn create_test_record(statute_id: &str) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute_id.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_s3_config_creation() {
        let config = create_test_config();
        assert_eq!(config.bucket, "test-bucket");
        assert_eq!(config.endpoint, "http://localhost:9000");
    }

    #[test]
    fn test_s3_config_aws() {
        let config = S3Config::aws(
            "my-bucket".to_string(),
            "access".to_string(),
            "secret".to_string(),
            "us-west-2".to_string(),
        );
        assert_eq!(config.bucket, "my-bucket");
        assert_eq!(config.region, "us-west-2");
        assert!(config.endpoint.contains("us-west-2"));
    }

    #[test]
    fn test_s3_storage_creation() {
        let config = create_test_config();
        let storage = S3Storage::new(config);
        assert!(storage.is_ok());
    }

    #[test]
    fn test_s3_storage_store_and_get() {
        let config = create_test_config();
        let mut storage = S3Storage::new(config).unwrap();

        let record = create_test_record("statute-1");
        let id = record.id;

        storage.store(record).unwrap();
        let retrieved = storage.get(id).unwrap();
        assert_eq!(retrieved.id, id);
        assert_eq!(retrieved.statute_id, "statute-1");
    }

    #[test]
    fn test_s3_storage_get_all() {
        let config = create_test_config();
        let mut storage = S3Storage::new(config).unwrap();

        for i in 0..5 {
            let record = create_test_record(&format!("statute-{}", i));
            storage.store(record).unwrap();
        }

        let all_records = storage.get_all().unwrap();
        assert_eq!(all_records.len(), 5);
    }

    #[test]
    fn test_s3_storage_get_by_statute() {
        let config = create_test_config();
        let mut storage = S3Storage::new(config).unwrap();

        for _ in 0..3 {
            let record = create_test_record("statute-1");
            storage.store(record).unwrap();
        }

        for _ in 0..2 {
            let record = create_test_record("statute-2");
            storage.store(record).unwrap();
        }

        let statute1_records = storage.get_by_statute("statute-1").unwrap();
        assert_eq!(statute1_records.len(), 3);

        let statute2_records = storage.get_by_statute("statute-2").unwrap();
        assert_eq!(statute2_records.len(), 2);
    }

    #[test]
    fn test_s3_storage_get_by_subject() {
        let config = create_test_config();
        let mut storage = S3Storage::new(config).unwrap();

        let subject_id = Uuid::new_v4();
        let mut record = create_test_record("statute-1");
        record.subject_id = subject_id;

        storage.store(record).unwrap();

        let subject_records = storage.get_by_subject(subject_id).unwrap();
        assert_eq!(subject_records.len(), 1);
    }

    #[test]
    fn test_s3_storage_count() {
        let config = create_test_config();
        let mut storage = S3Storage::new(config).unwrap();

        assert_eq!(storage.count().unwrap(), 0);

        for i in 0..10 {
            let record = create_test_record(&format!("statute-{}", i));
            storage.store(record).unwrap();
        }

        assert_eq!(storage.count().unwrap(), 10);
    }

    #[test]
    fn test_s3_storage_last_hash() {
        let config = create_test_config();
        let mut storage = S3Storage::new(config).unwrap();

        assert_eq!(storage.get_last_hash().unwrap(), None);

        let record = create_test_record("statute-1");
        let expected_hash = record.record_hash.clone();
        storage.store(record).unwrap();

        assert_eq!(storage.get_last_hash().unwrap(), Some(expected_hash));
    }

    #[test]
    fn test_s3_storage_record_key() {
        let config = create_test_config();
        let storage = S3Storage::new(config).unwrap();

        let record_id = Uuid::new_v4();
        let key = storage.get_record_key(record_id);
        assert!(key.contains(&record_id.to_string()));
        assert!(key.ends_with(".json"));
    }

    #[test]
    fn test_s3_storage_get_not_found() {
        let config = create_test_config();
        let storage = S3Storage::new(config).unwrap();

        let non_existent_id = Uuid::new_v4();
        assert!(storage.get(non_existent_id).is_err());
    }
}
