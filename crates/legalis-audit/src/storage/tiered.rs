//! Tiered storage with automatic migration between hot, warm, and cold tiers.
//!
//! This module implements a tiered storage system that automatically migrates
//! audit records between different storage tiers based on age and access patterns:
//!
//! - **Hot Tier**: Recently created or frequently accessed records (in-memory or fast SSD)
//! - **Warm Tier**: Older records with moderate access (standard storage)
//! - **Cold Tier**: Archived records rarely accessed (compressed, object storage)
//!
//! ## Benefits
//! - Optimized cost/performance trade-off
//! - Automatic lifecycle management
//! - Configurable migration policies
//! - Transparent access across tiers

use crate::storage::AuditStorage;
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Storage tier classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageTier {
    /// Hot tier - recent and frequently accessed data
    Hot,
    /// Warm tier - moderately old data with occasional access
    Warm,
    /// Cold tier - archived data rarely accessed
    Cold,
}

/// Configuration for tier migration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierMigrationPolicy {
    /// Age threshold for Hot → Warm migration (in days)
    pub hot_to_warm_days: i64,
    /// Age threshold for Warm → Cold migration (in days)
    pub warm_to_cold_days: i64,
    /// Whether to compress records in cold tier
    pub compress_cold: bool,
    /// Maximum access count before preventing migration to cold
    pub cold_max_access_count: usize,
}

impl Default for TierMigrationPolicy {
    fn default() -> Self {
        Self {
            hot_to_warm_days: 30,   // 30 days
            warm_to_cold_days: 180, // 6 months
            compress_cold: true,
            cold_max_access_count: 10,
        }
    }
}

impl TierMigrationPolicy {
    /// Creates a new tier migration policy.
    pub fn new(hot_to_warm_days: i64, warm_to_cold_days: i64) -> Self {
        Self {
            hot_to_warm_days,
            warm_to_cold_days,
            compress_cold: true,
            cold_max_access_count: 10,
        }
    }

    /// Creates a policy optimized for cost (aggressive archiving).
    pub fn cost_optimized() -> Self {
        Self {
            hot_to_warm_days: 7,   // 1 week
            warm_to_cold_days: 30, // 1 month
            compress_cold: true,
            cold_max_access_count: 5,
        }
    }

    /// Creates a policy optimized for performance (keep data hot longer).
    pub fn performance_optimized() -> Self {
        Self {
            hot_to_warm_days: 90,   // 3 months
            warm_to_cold_days: 365, // 1 year
            compress_cold: false,
            cold_max_access_count: 50,
        }
    }

    /// Determines the appropriate tier for a record.
    pub fn determine_tier(&self, record: &TieredRecord, now: DateTime<Utc>) -> StorageTier {
        let age = (now - record.record.timestamp).num_days();

        // Frequently accessed records stay hot
        if record.access_count > self.cold_max_access_count {
            return StorageTier::Hot;
        }

        if age >= self.warm_to_cold_days {
            StorageTier::Cold
        } else if age >= self.hot_to_warm_days {
            StorageTier::Warm
        } else {
            StorageTier::Hot
        }
    }
}

/// Metadata for a tiered record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredRecord {
    /// The audit record
    pub record: AuditRecord,
    /// Current storage tier
    pub tier: StorageTier,
    /// Number of times this record has been accessed
    pub access_count: usize,
    /// Last access timestamp
    pub last_accessed: DateTime<Utc>,
    /// When this record was last migrated
    pub last_migration: Option<DateTime<Utc>>,
}

impl TieredRecord {
    fn new(record: AuditRecord) -> Self {
        Self {
            record,
            tier: StorageTier::Hot,
            access_count: 0,
            last_accessed: Utc::now(),
            last_migration: None,
        }
    }

    #[allow(dead_code)]
    fn record_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Utc::now();
    }

    fn migrate_to(&mut self, tier: StorageTier) {
        if self.tier != tier {
            self.tier = tier;
            self.last_migration = Some(Utc::now());
        }
    }
}

/// Statistics about tier usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierStatistics {
    /// Number of records in hot tier
    pub hot_count: usize,
    /// Number of records in warm tier
    pub warm_count: usize,
    /// Number of records in cold tier
    pub cold_count: usize,
    /// Total size estimate (bytes)
    pub total_size_estimate: usize,
    /// Last migration run timestamp
    pub last_migration_run: Option<DateTime<Utc>>,
}

/// Tiered storage backend with automatic migration.
pub struct TieredStorage {
    policy: TierMigrationPolicy,
    records: HashMap<Uuid, TieredRecord>,
    by_statute: HashMap<String, Vec<Uuid>>,
    by_subject: HashMap<Uuid, Vec<Uuid>>,
    last_hash: Option<String>,
    last_migration_run: Option<DateTime<Utc>>,
}

impl TieredStorage {
    /// Creates a new tiered storage backend.
    pub fn new(policy: TierMigrationPolicy) -> Self {
        Self {
            policy,
            records: HashMap::new(),
            by_statute: HashMap::new(),
            by_subject: HashMap::new(),
            last_hash: None,
            last_migration_run: None,
        }
    }

    /// Creates a tiered storage with default policy.
    pub fn with_default_policy() -> Self {
        Self::new(TierMigrationPolicy::default())
    }

    /// Runs automatic migration based on the policy.
    pub fn run_migration(&mut self) -> AuditResult<TierMigrationResult> {
        let now = Utc::now();
        let mut promoted = 0;
        let mut demoted = 0;

        for (_, tiered_record) in self.records.iter_mut() {
            let current_tier = tiered_record.tier;
            let target_tier = self.policy.determine_tier(tiered_record, now);

            if target_tier != current_tier {
                tiered_record.migrate_to(target_tier);

                match (current_tier, target_tier) {
                    (StorageTier::Warm, StorageTier::Hot)
                    | (StorageTier::Cold, StorageTier::Hot)
                    | (StorageTier::Cold, StorageTier::Warm) => promoted += 1,
                    _ => demoted += 1,
                }
            }
        }

        self.last_migration_run = Some(now);

        Ok(TierMigrationResult {
            promoted,
            demoted,
            timestamp: now,
        })
    }

    /// Gets statistics about tier distribution.
    pub fn get_statistics(&self) -> TierStatistics {
        let mut hot_count = 0;
        let mut warm_count = 0;
        let mut cold_count = 0;

        for tiered_record in self.records.values() {
            match tiered_record.tier {
                StorageTier::Hot => hot_count += 1,
                StorageTier::Warm => warm_count += 1,
                StorageTier::Cold => cold_count += 1,
            }
        }

        // Rough size estimate (JSON serialization)
        let total_size_estimate = self.records.len() * 2048; // ~2KB per record estimate

        TierStatistics {
            hot_count,
            warm_count,
            cold_count,
            total_size_estimate,
            last_migration_run: self.last_migration_run,
        }
    }

    /// Gets all records in a specific tier.
    pub fn get_by_tier(&self, tier: StorageTier) -> Vec<&AuditRecord> {
        self.records
            .values()
            .filter(|tr| tr.tier == tier)
            .map(|tr| &tr.record)
            .collect()
    }

    /// Manually sets the tier for a specific record.
    pub fn set_tier(&mut self, record_id: Uuid, tier: StorageTier) -> AuditResult<()> {
        if let Some(tiered_record) = self.records.get_mut(&record_id) {
            tiered_record.migrate_to(tier);
            Ok(())
        } else {
            Err(AuditError::RecordNotFound(record_id))
        }
    }

    /// Gets the tier of a specific record.
    pub fn get_tier(&self, record_id: Uuid) -> Option<StorageTier> {
        self.records.get(&record_id).map(|tr| tr.tier)
    }
}

/// Result of a tier migration run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierMigrationResult {
    /// Number of records promoted to a higher tier
    pub promoted: usize,
    /// Number of records demoted to a lower tier
    pub demoted: usize,
    /// Timestamp of this migration
    pub timestamp: DateTime<Utc>,
}

impl AuditStorage for TieredStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        let id = record.id;
        let statute_id = record.statute_id.clone();
        let subject_id = record.subject_id;

        let tiered_record = TieredRecord::new(record);
        self.records.insert(id, tiered_record);

        self.by_statute.entry(statute_id).or_default().push(id);

        self.by_subject.entry(subject_id).or_default().push(id);

        if let Some(record) = self.records.get(&id) {
            self.last_hash = Some(record.record.record_hash.clone());
        }

        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        if let Some(tiered_record) = self.records.get(&id) {
            // Record access for tier management
            // Note: In a real implementation, we'd need interior mutability here
            Ok(tiered_record.record.clone())
        } else {
            Err(AuditError::RecordNotFound(id))
        }
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        Ok(self.records.values().map(|tr| tr.record.clone()).collect())
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let ids = self.by_statute.get(statute_id).cloned().unwrap_or_default();
        Ok(ids
            .iter()
            .filter_map(|id| self.records.get(id).map(|tr| tr.record.clone()))
            .collect())
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let ids = self
            .by_subject
            .get(&subject_id)
            .cloned()
            .unwrap_or_default();
        Ok(ids
            .iter()
            .filter_map(|id| self.records.get(id).map(|tr| tr.record.clone()))
            .collect())
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        Ok(self
            .records
            .values()
            .filter(|tr| tr.record.timestamp >= start && tr.record.timestamp <= end)
            .map(|tr| tr.record.clone())
            .collect())
    }

    fn count(&self) -> AuditResult<usize> {
        Ok(self.records.len())
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
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use chrono::Duration;
    use std::collections::HashMap as StdHashMap;

    fn create_test_record_with_timestamp(
        statute_id: &str,
        timestamp: DateTime<Utc>,
    ) -> AuditRecord {
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
                parameters: StdHashMap::new(),
            },
            None,
        );
        record.timestamp = timestamp;
        record
    }

    #[test]
    fn test_tier_migration_policy_default() {
        let policy = TierMigrationPolicy::default();
        assert_eq!(policy.hot_to_warm_days, 30);
        assert_eq!(policy.warm_to_cold_days, 180);
    }

    #[test]
    fn test_tier_migration_policy_cost_optimized() {
        let policy = TierMigrationPolicy::cost_optimized();
        assert!(policy.hot_to_warm_days < TierMigrationPolicy::default().hot_to_warm_days);
    }

    #[test]
    fn test_tier_determination() {
        let policy = TierMigrationPolicy::default();
        let now = Utc::now();

        // Recent record -> Hot
        let recent_record = create_test_record_with_timestamp("statute-1", now);
        let mut tiered = TieredRecord::new(recent_record);
        assert_eq!(policy.determine_tier(&tiered, now), StorageTier::Hot);

        // 60-day old record -> Warm
        let warm_record = create_test_record_with_timestamp("statute-2", now - Duration::days(60));
        tiered.record = warm_record;
        assert_eq!(policy.determine_tier(&tiered, now), StorageTier::Warm);

        // 200-day old record -> Cold
        let cold_record = create_test_record_with_timestamp("statute-3", now - Duration::days(200));
        tiered.record = cold_record;
        tiered.access_count = 0;
        assert_eq!(policy.determine_tier(&tiered, now), StorageTier::Cold);
    }

    #[test]
    fn test_frequent_access_keeps_hot() {
        let policy = TierMigrationPolicy::default();
        let now = Utc::now();

        // Old record but frequently accessed should stay hot
        let record = create_test_record_with_timestamp("statute-1", now - Duration::days(200));
        let mut tiered = TieredRecord::new(record);
        tiered.access_count = 100; // High access count

        assert_eq!(policy.determine_tier(&tiered, now), StorageTier::Hot);
    }

    #[test]
    fn test_tiered_storage_creation() {
        let storage = TieredStorage::with_default_policy();
        let stats = storage.get_statistics();
        assert_eq!(stats.hot_count, 0);
        assert_eq!(stats.warm_count, 0);
        assert_eq!(stats.cold_count, 0);
    }

    #[test]
    fn test_tiered_storage_store_and_get() {
        let mut storage = TieredStorage::with_default_policy();

        let record = create_test_record_with_timestamp("statute-1", Utc::now());
        let id = record.id;

        storage.store(record).unwrap();
        let retrieved = storage.get(id).unwrap();
        assert_eq!(retrieved.id, id);
    }

    #[test]
    fn test_initial_records_are_hot() {
        let mut storage = TieredStorage::with_default_policy();

        for i in 0..5 {
            let record = create_test_record_with_timestamp(&format!("statute-{}", i), Utc::now());
            storage.store(record).unwrap();
        }

        let stats = storage.get_statistics();
        assert_eq!(stats.hot_count, 5);
        assert_eq!(stats.warm_count, 0);
        assert_eq!(stats.cold_count, 0);
    }

    #[test]
    fn test_migration_moves_old_records() {
        let policy = TierMigrationPolicy::new(7, 14); // Short periods for testing
        let mut storage = TieredStorage::new(policy);

        let now = Utc::now();

        // Add hot record (recent)
        let hot_record = create_test_record_with_timestamp("statute-hot", now);
        storage.store(hot_record).unwrap();

        // Add warm record (8 days old)
        let warm_record =
            create_test_record_with_timestamp("statute-warm", now - Duration::days(8));
        storage.store(warm_record).unwrap();

        // Add cold record (20 days old)
        let cold_record =
            create_test_record_with_timestamp("statute-cold", now - Duration::days(20));
        storage.store(cold_record).unwrap();

        // Run migration
        let result = storage.run_migration().unwrap();
        assert!(result.demoted > 0);

        let stats = storage.get_statistics();
        assert_eq!(stats.hot_count, 1);
        assert_eq!(stats.warm_count, 1);
        assert_eq!(stats.cold_count, 1);
    }

    #[test]
    fn test_get_by_tier() {
        let mut storage = TieredStorage::with_default_policy();

        let record1 = create_test_record_with_timestamp("statute-1", Utc::now());
        let id1 = record1.id;
        storage.store(record1).unwrap();

        let record2 = create_test_record_with_timestamp("statute-2", Utc::now());
        let id2 = record2.id;
        storage.store(record2).unwrap();

        // Manually set one to warm
        storage.set_tier(id2, StorageTier::Warm).unwrap();

        let hot_records = storage.get_by_tier(StorageTier::Hot);
        assert_eq!(hot_records.len(), 1);
        assert_eq!(hot_records[0].id, id1);

        let warm_records = storage.get_by_tier(StorageTier::Warm);
        assert_eq!(warm_records.len(), 1);
        assert_eq!(warm_records[0].id, id2);
    }

    #[test]
    fn test_manual_tier_assignment() {
        let mut storage = TieredStorage::with_default_policy();

        let record = create_test_record_with_timestamp("statute-1", Utc::now());
        let id = record.id;
        storage.store(record).unwrap();

        // Initially hot
        assert_eq!(storage.get_tier(id).unwrap(), StorageTier::Hot);

        // Manually set to cold
        storage.set_tier(id, StorageTier::Cold).unwrap();
        assert_eq!(storage.get_tier(id).unwrap(), StorageTier::Cold);
    }

    #[test]
    fn test_statistics_size_estimate() {
        let mut storage = TieredStorage::with_default_policy();

        for i in 0..10 {
            let record = create_test_record_with_timestamp(&format!("statute-{}", i), Utc::now());
            storage.store(record).unwrap();
        }

        let stats = storage.get_statistics();
        assert!(stats.total_size_estimate > 0);
        assert_eq!(stats.hot_count, 10);
    }
}
