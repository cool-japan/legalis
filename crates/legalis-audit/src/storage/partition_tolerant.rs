//! Partition-tolerant storage backend for distributed audit trails
//!
//! This module provides a storage backend that can tolerate network partitions
//! and continue to operate using eventual consistency and conflict resolution.

use crate::distributed::{NodeId, VectorClock};
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictStrategy {
    /// Last write wins (based on timestamp)
    LastWriteWins,
    /// First write wins (based on vector clock)
    FirstWriteWins,
    /// Keep all versions
    KeepAll,
    /// Custom resolution (requires manual intervention)
    Custom,
}

/// Configuration for partition-tolerant storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionTolerantConfig {
    pub conflict_strategy: ConflictStrategy,
    pub max_pending_writes: usize,
    pub enable_read_repair: bool,
    pub quorum_reads: bool,
    pub quorum_writes: bool,
}

impl Default for PartitionTolerantConfig {
    fn default() -> Self {
        Self {
            conflict_strategy: ConflictStrategy::LastWriteWins,
            max_pending_writes: 10000,
            enable_read_repair: true,
            quorum_reads: false,
            quorum_writes: true,
        }
    }
}

/// A versioned record with conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedRecord {
    pub record: AuditRecord,
    pub vector_clock: VectorClock,
    pub origin_node: NodeId,
    pub versions: Vec<AuditRecord>,
    pub is_conflicted: bool,
}

impl VersionedRecord {
    pub fn new(record: AuditRecord, vector_clock: VectorClock, origin_node: NodeId) -> Self {
        Self {
            record,
            vector_clock,
            origin_node,
            versions: Vec::new(),
            is_conflicted: false,
        }
    }

    pub fn add_version(&mut self, record: AuditRecord) {
        self.versions.push(record);
        self.is_conflicted = true;
    }

    pub fn resolve_conflict(&mut self, strategy: ConflictStrategy) {
        if !self.is_conflicted || self.versions.is_empty() {
            return;
        }

        match strategy {
            ConflictStrategy::LastWriteWins => {
                // Find the record with the latest timestamp
                let mut all_records = vec![self.record.clone()];
                all_records.extend(self.versions.clone());

                if let Some(latest) = all_records.into_iter().max_by_key(|r| r.timestamp) {
                    self.record = latest;
                    self.versions.clear();
                    self.is_conflicted = false;
                }
            }
            ConflictStrategy::FirstWriteWins => {
                // Keep the original record
                self.versions.clear();
                self.is_conflicted = false;
            }
            ConflictStrategy::KeepAll => {
                // Do nothing, keep all versions
            }
            ConflictStrategy::Custom => {
                // Requires manual resolution
            }
        }
    }
}

/// Pending write operation during a partition
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PendingWrite {
    record: AuditRecord,
    vector_clock: VectorClock,
    timestamp: DateTime<Utc>,
}

/// Partition-tolerant storage backend
pub struct PartitionTolerantStorage {
    config: PartitionTolerantConfig,
    node_id: NodeId,
    records: Arc<RwLock<HashMap<Uuid, VersionedRecord>>>,
    pending_writes: Arc<RwLock<VecDeque<PendingWrite>>>,
    vector_clock: Arc<RwLock<VectorClock>>,
    is_partitioned: Arc<RwLock<bool>>,
    last_hash: Arc<RwLock<Option<String>>>,
}

impl PartitionTolerantStorage {
    /// Create a new partition-tolerant storage
    pub fn new(node_id: NodeId, config: PartitionTolerantConfig) -> Self {
        Self {
            config,
            node_id,
            records: Arc::new(RwLock::new(HashMap::new())),
            pending_writes: Arc::new(RwLock::new(VecDeque::new())),
            vector_clock: Arc::new(RwLock::new(VectorClock::new())),
            is_partitioned: Arc::new(RwLock::new(false)),
            last_hash: Arc::new(RwLock::new(None)),
        }
    }

    /// Mark the storage as partitioned
    pub fn mark_partitioned(&self) -> Result<(), AuditError> {
        let mut is_partitioned = self.is_partitioned.write().map_err(|e| {
            AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;
        *is_partitioned = true;
        Ok(())
    }

    /// Mark the storage as healed from partition
    pub fn mark_healed(&mut self) -> Result<(), AuditError> {
        {
            let mut is_partitioned = self.is_partitioned.write().map_err(|e| {
                AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
            })?;
            *is_partitioned = false;
        }

        // Flush pending writes
        self.flush_pending_writes()?;
        Ok(())
    }

    /// Check if currently partitioned
    pub fn is_partitioned_status(&self) -> Result<bool, AuditError> {
        let is_partitioned = self
            .is_partitioned
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(*is_partitioned)
    }

    /// Flush pending writes to storage
    fn flush_pending_writes(&mut self) -> Result<(), AuditError> {
        // Collect all pending writes first
        let writes: Vec<PendingWrite> = {
            let mut pending = self.pending_writes.write().map_err(|e| {
                AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
            })?;

            let mut writes = Vec::new();
            while let Some(write) = pending.pop_front() {
                writes.push(write);
            }
            writes
        };

        // Process writes after releasing the lock
        for write in writes {
            self.store_versioned_record(write.record, write.vector_clock)?;
        }

        Ok(())
    }

    /// Store a versioned record
    fn store_versioned_record(
        &mut self,
        record: AuditRecord,
        vector_clock: VectorClock,
    ) -> Result<(), AuditError> {
        let mut records = self.records.write().map_err(|e| {
            AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;

        if let Some(existing) = records.get_mut(&record.id) {
            // Check for conflicts using vector clock
            if existing.vector_clock.is_concurrent(&vector_clock) {
                // Concurrent update - conflict!
                existing.add_version(record.clone());
                existing.resolve_conflict(self.config.conflict_strategy);
            } else if vector_clock.happens_before(&existing.vector_clock) {
                // Old record, ignore
            } else {
                // New record, update
                existing.record = record.clone();
                existing.vector_clock = vector_clock;
            }
        } else {
            // New record
            let versioned =
                VersionedRecord::new(record.clone(), vector_clock, self.node_id.clone());
            records.insert(record.id, versioned);
        }

        Ok(())
    }

    /// Get pending write count
    pub fn pending_write_count(&self) -> Result<usize, AuditError> {
        let pending = self
            .pending_writes
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(pending.len())
    }

    /// Get conflicted records
    pub fn get_conflicted_records(&self) -> Result<Vec<VersionedRecord>, AuditError> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        Ok(records
            .values()
            .filter(|r| r.is_conflicted)
            .cloned()
            .collect())
    }

    /// Manually resolve a conflict
    pub fn resolve_conflict(
        &mut self,
        record_id: &Uuid,
        chosen_version: AuditRecord,
    ) -> Result<(), AuditError> {
        let mut records = self.records.write().map_err(|e| {
            AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;

        if let Some(versioned) = records.get_mut(record_id) {
            versioned.record = chosen_version;
            versioned.versions.clear();
            versioned.is_conflicted = false;
            Ok(())
        } else {
            Err(AuditError::RecordNotFound(*record_id))
        }
    }
}

impl super::AuditStorage for PartitionTolerantStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        // Increment vector clock
        let mut clock = self.vector_clock.write().map_err(|e| {
            AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;
        clock.increment(&self.node_id);
        let current_clock = clock.clone();
        drop(clock);

        // Check if partitioned
        let is_partitioned = self.is_partitioned_status()?;

        if is_partitioned {
            // Queue write for later
            let mut pending = self.pending_writes.write().map_err(|e| {
                AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
            })?;

            if pending.len() >= self.config.max_pending_writes {
                return Err(AuditError::StorageError(
                    "Pending write queue is full".to_string(),
                ));
            }

            pending.push_back(PendingWrite {
                record,
                vector_clock: current_clock,
                timestamp: Utc::now(),
            });

            Ok(())
        } else {
            // Normal write
            self.store_versioned_record(record, current_clock)
        }
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        records
            .get(&id)
            .map(|v| v.record.clone())
            .ok_or(AuditError::RecordNotFound(id))
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        let mut results: Vec<AuditRecord> = records.values().map(|v| v.record.clone()).collect();

        results.sort_by_key(|r| r.timestamp);
        Ok(results)
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        let mut results: Vec<AuditRecord> = records
            .values()
            .map(|v| &v.record)
            .filter(|r| r.statute_id == statute_id)
            .cloned()
            .collect();

        results.sort_by_key(|r| r.timestamp);
        Ok(results)
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;

        let mut results: Vec<AuditRecord> = records
            .values()
            .map(|v| &v.record)
            .filter(|r| r.subject_id == subject_id)
            .cloned()
            .collect();

        results.sort_by_key(|r| r.timestamp);
        Ok(results)
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

        let mut results: Vec<AuditRecord> = records
            .values()
            .map(|v| &v.record)
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .cloned()
            .collect();

        results.sort_by_key(|r| r.timestamp);
        Ok(results)
    }

    fn count(&self) -> AuditResult<usize> {
        let records = self
            .records
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(records.len())
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        let last_hash = self
            .last_hash
            .read()
            .map_err(|e| AuditError::StorageError(format!("Failed to acquire read lock: {}", e)))?;
        Ok(last_hash.clone())
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        let mut last_hash = self.last_hash.write().map_err(|e| {
            AuditError::StorageError(format!("Failed to acquire write lock: {}", e))
        })?;
        *last_hash = hash;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::AuditStorage;
    use crate::{Actor, EventType};

    fn create_test_record() -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::AutomaticDecision,
            statute_id: "TEST-1".to_string(),
            subject_id: Uuid::new_v4(),
            actor: Actor::System {
                component: "test".to_string(),
            },
            context: crate::DecisionContext::default(),
            result: crate::DecisionResult::Deterministic {
                effect_applied: "allowed".to_string(),
                parameters: std::collections::HashMap::new(),
            },
            previous_hash: None,
            record_hash: "hash123".to_string(),
        }
    }

    #[test]
    fn test_partition_tolerant_config() {
        let config = PartitionTolerantConfig::default();
        assert_eq!(config.conflict_strategy, ConflictStrategy::LastWriteWins);
        assert_eq!(config.max_pending_writes, 10000);
        assert!(config.enable_read_repair);
    }

    #[test]
    fn test_versioned_record() {
        let record = create_test_record();
        let clock = VectorClock::new();
        let node_id = NodeId::new("node-1");

        let mut versioned = VersionedRecord::new(record.clone(), clock, node_id);

        assert!(!versioned.is_conflicted);
        assert_eq!(versioned.versions.len(), 0);

        let conflicting_record = create_test_record();
        versioned.add_version(conflicting_record);

        assert!(versioned.is_conflicted);
        assert_eq!(versioned.versions.len(), 1);
    }

    #[test]
    fn test_conflict_resolution_last_write_wins() {
        let mut record1 = create_test_record();
        record1.timestamp = Utc::now();

        let mut record2 = create_test_record();
        record2.id = record1.id;
        record2.timestamp = Utc::now() + chrono::Duration::seconds(10);

        let clock = VectorClock::new();
        let node_id = NodeId::new("node-1");

        let mut versioned = VersionedRecord::new(record1, clock, node_id);
        versioned.add_version(record2.clone());

        versioned.resolve_conflict(ConflictStrategy::LastWriteWins);

        assert!(!versioned.is_conflicted);
        assert_eq!(versioned.record.timestamp, record2.timestamp);
    }

    #[test]
    fn test_conflict_resolution_first_write_wins() {
        let record1 = create_test_record();
        let mut record2 = create_test_record();
        record2.id = record1.id;

        let clock = VectorClock::new();
        let node_id = NodeId::new("node-1");

        let mut versioned = VersionedRecord::new(record1.clone(), clock, node_id);
        versioned.add_version(record2);

        versioned.resolve_conflict(ConflictStrategy::FirstWriteWins);

        assert!(!versioned.is_conflicted);
        assert_eq!(versioned.record.id, record1.id);
        assert_eq!(versioned.versions.len(), 0);
    }

    #[test]
    fn test_partition_tolerant_storage() {
        let node_id = NodeId::new("node-1");
        let config = PartitionTolerantConfig::default();
        let storage = PartitionTolerantStorage::new(node_id, config);

        assert!(!storage.is_partitioned_status().unwrap());
        assert_eq!(storage.count().unwrap(), 0);
    }

    #[test]
    fn test_append_during_partition() {
        let node_id = NodeId::new("node-1");
        let config = PartitionTolerantConfig::default();
        let mut storage = PartitionTolerantStorage::new(node_id, config);

        let record = create_test_record();

        // Mark as partitioned
        storage.mark_partitioned().unwrap();
        assert!(storage.is_partitioned_status().unwrap());

        // Store should queue the write
        storage.store(record.clone()).unwrap();
        assert_eq!(storage.pending_write_count().unwrap(), 1);
        assert_eq!(storage.count().unwrap(), 0); // Not yet written

        // Heal partition
        storage.mark_healed().unwrap();
        assert!(!storage.is_partitioned_status().unwrap());

        // Record should now be written
        assert_eq!(storage.pending_write_count().unwrap(), 0);
        assert_eq!(storage.count().unwrap(), 1);
    }

    #[test]
    fn test_store_normal() {
        let node_id = NodeId::new("node-1");
        let config = PartitionTolerantConfig::default();
        let mut storage = PartitionTolerantStorage::new(node_id, config);

        let record = create_test_record();
        storage.store(record).unwrap();

        assert_eq!(storage.count().unwrap(), 1);
        assert_eq!(storage.pending_write_count().unwrap(), 0);
    }

    #[test]
    fn test_get_record() {
        let node_id = NodeId::new("node-1");
        let config = PartitionTolerantConfig::default();
        let mut storage = PartitionTolerantStorage::new(node_id, config);

        let record = create_test_record();
        let record_id = record.id;

        storage.store(record).unwrap();

        let retrieved = storage.get(record_id).unwrap();
        assert_eq!(retrieved.id, record_id);
    }

    #[test]
    fn test_get_by_statute() {
        let node_id = NodeId::new("node-1");
        let config = PartitionTolerantConfig::default();
        let mut storage = PartitionTolerantStorage::new(node_id, config);

        let mut record = create_test_record();
        record.statute_id = "STATUTE-1".to_string();

        storage.store(record).unwrap();

        let results = storage.get_by_statute("STATUTE-1").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].statute_id, "STATUTE-1");
    }

    #[test]
    fn test_get_conflicted_records() {
        let node_id = NodeId::new("node-1");
        let config = PartitionTolerantConfig {
            conflict_strategy: ConflictStrategy::KeepAll,
            ..Default::default()
        };
        let mut storage = PartitionTolerantStorage::new(node_id, config);

        let record1 = create_test_record();
        storage.store(record1).unwrap();

        let conflicted = storage.get_conflicted_records().unwrap();
        assert_eq!(conflicted.len(), 0);
    }

    #[test]
    fn test_resolve_conflict_manually() {
        let node_id = NodeId::new("node-1");
        let config = PartitionTolerantConfig::default();
        let mut storage = PartitionTolerantStorage::new(node_id, config);

        let record = create_test_record();
        let record_id = record.id;

        storage.store(record.clone()).unwrap();

        let mut resolved_record = record;
        resolved_record.statute_id = "RESOLVED".to_string();

        storage
            .resolve_conflict(&record_id, resolved_record.clone())
            .unwrap();

        let retrieved = storage.get(record_id).unwrap();
        assert_eq!(retrieved.statute_id, "RESOLVED");
    }
}
