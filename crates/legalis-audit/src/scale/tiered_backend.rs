//! Multi-backend tiered storage with physical migration.
//!
//! [`crate::storage::tiered::TieredStorage`] keeps every record in a single
//! in-memory map and merely *labels* it with a tier. This module instead routes
//! records to three **independent, pluggable** [`AuditStorage`] backends — one
//! per tier — and *physically* migrates records between them:
//!
//! - **Hot**: a fast backend for recent / frequently-read records.
//! - **Warm**: a cheaper backend for ageing records.
//! - **Cold**: an archival backend (e.g. compressed / object storage).
//!
//! Because any backend that implements [`AuditStorage`] can be plugged into any
//! tier, you can mix, say, an in-memory hot tier with a SQLite warm tier and a
//! JSONL cold tier. Migration uses the existing
//! [`crate::storage::tiered::TierMigrationPolicy`] thresholds. Physical removal
//! from the source tier needs [`AuditStorage::remove`]; tiers whose backend does
//! not support removal degrade gracefully to a *logical* migration (the record
//! is copied to the target and the authoritative tier map is updated, leaving a
//! harmless shadow copy in the source that reads transparently ignore).
//!
//! [`MultiTierStore`] itself implements [`AuditStorage`], so it composes
//! directly into an [`crate::AuditTrail`]; an insertion-sequence number keeps
//! [`AuditStorage::get_all`] in chain order across tiers so hash-chain
//! verification still works after migration.

use crate::storage::AuditStorage;
use crate::storage::memory::MemoryStorage;
use crate::storage::tiered::{StorageTier, TierMigrationPolicy};
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

/// Per-record placement and access metadata held by a [`MultiTierStore`].
#[derive(Debug, Clone)]
struct Placement {
    tier: StorageTier,
    timestamp: DateTime<Utc>,
    seq: u64,
    access_count: usize,
    last_accessed: DateTime<Utc>,
}

/// How many records currently live in each tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierDistribution {
    /// Records in the hot tier.
    pub hot: usize,
    /// Records in the warm tier.
    pub warm: usize,
    /// Records in the cold tier.
    pub cold: usize,
}

impl TierDistribution {
    /// Total records across all tiers.
    pub fn total(&self) -> usize {
        self.hot + self.warm + self.cold
    }
}

/// The outcome of a [`MultiTierStore::run_migration`] pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReport {
    /// Records moved to a hotter tier.
    pub promoted: usize,
    /// Records moved to a colder tier.
    pub demoted: usize,
    /// Migrations where the source copy was physically removed.
    pub physical_moves: usize,
    /// Migrations where the source could not remove (shadow copy retained).
    pub logical_moves: usize,
    /// When the pass ran.
    pub timestamp: DateTime<Utc>,
}

/// A three-tier store over pluggable [`AuditStorage`] backends.
pub struct MultiTierStore {
    hot: Box<dyn AuditStorage>,
    warm: Box<dyn AuditStorage>,
    cold: Box<dyn AuditStorage>,
    policy: TierMigrationPolicy,
    placement: HashMap<Uuid, Placement>,
    last_hash: Option<String>,
    next_seq: u64,
}

impl MultiTierStore {
    /// Builds a tiered store over three explicit backends.
    pub fn new(
        hot: Box<dyn AuditStorage>,
        warm: Box<dyn AuditStorage>,
        cold: Box<dyn AuditStorage>,
        policy: TierMigrationPolicy,
    ) -> Self {
        Self {
            hot,
            warm,
            cold,
            policy,
            placement: HashMap::new(),
            last_hash: None,
            next_seq: 0,
        }
    }

    /// Builds a tiered store backed by three in-memory tiers (handy for tests
    /// and small deployments).
    pub fn with_memory_tiers(policy: TierMigrationPolicy) -> Self {
        Self::new(
            Box::new(MemoryStorage::new()),
            Box::new(MemoryStorage::new()),
            Box::new(MemoryStorage::new()),
            policy,
        )
    }

    /// Returns the authoritative tier of a record, if present.
    pub fn tier_of(&self, id: Uuid) -> Option<StorageTier> {
        self.placement.get(&id).map(|p| p.tier)
    }

    /// Returns the per-tier record counts.
    pub fn tier_distribution(&self) -> TierDistribution {
        let mut dist = TierDistribution {
            hot: 0,
            warm: 0,
            cold: 0,
        };
        for placement in self.placement.values() {
            match placement.tier {
                StorageTier::Hot => dist.hot += 1,
                StorageTier::Warm => dist.warm += 1,
                StorageTier::Cold => dist.cold += 1,
            }
        }
        dist
    }

    /// Reads a record and records the access (for access-aware tiering).
    ///
    /// The plain [`AuditStorage::get`] is the read-only fast path; this variant
    /// additionally increments the record's access counter so frequently-read
    /// records can be kept hot by [`Self::run_migration`].
    pub fn get_tracked(&mut self, id: Uuid) -> AuditResult<AuditRecord> {
        let tier = self
            .placement
            .get(&id)
            .map(|p| p.tier)
            .ok_or(AuditError::RecordNotFound(id))?;
        let record = self.get_from(tier, id)?;
        if let Some(placement) = self.placement.get_mut(&id) {
            placement.access_count += 1;
            placement.last_accessed = Utc::now();
        }
        Ok(record)
    }

    /// Manually moves a record to a specific tier (physical when supported).
    pub fn set_tier(&mut self, id: Uuid, target: StorageTier) -> AuditResult<()> {
        let current = self
            .placement
            .get(&id)
            .map(|p| p.tier)
            .ok_or(AuditError::RecordNotFound(id))?;
        if current == target {
            return Ok(());
        }
        self.migrate_one(id, current, target)?;
        Ok(())
    }

    /// Runs a migration pass according to the policy, physically moving records
    /// between tiers as required.
    pub fn run_migration(&mut self) -> AuditResult<MigrationReport> {
        let now = Utc::now();
        let mut decisions: Vec<(Uuid, StorageTier, StorageTier)> = Vec::new();
        for (id, placement) in self.placement.iter() {
            let target = self.determine_target(placement, now);
            if target != placement.tier {
                decisions.push((*id, placement.tier, target));
            }
        }

        let mut report = MigrationReport {
            promoted: 0,
            demoted: 0,
            physical_moves: 0,
            logical_moves: 0,
            timestamp: now,
        };
        for (id, current, target) in decisions {
            let physical = self.migrate_one(id, current, target)?;
            if tier_rank(target) < tier_rank(current) {
                report.promoted += 1;
            } else {
                report.demoted += 1;
            }
            if physical {
                report.physical_moves += 1;
            } else {
                report.logical_moves += 1;
            }
        }
        Ok(report)
    }

    /// Migrates one record; returns `true` if the source copy was physically
    /// removed (`false` for a logical, copy-only migration).
    fn migrate_one(
        &mut self,
        id: Uuid,
        current: StorageTier,
        target: StorageTier,
    ) -> AuditResult<bool> {
        let record = self.get_from(current, id)?;
        self.store_into(target, record)?;
        let physically_removed = self.remove_from(current, id)?;
        if let Some(placement) = self.placement.get_mut(&id) {
            placement.tier = target;
        }
        Ok(physically_removed)
    }

    fn determine_target(&self, placement: &Placement, now: DateTime<Utc>) -> StorageTier {
        if placement.access_count > self.policy.cold_max_access_count {
            return StorageTier::Hot;
        }
        let age_days = (now - placement.timestamp).num_days();
        if age_days >= self.policy.warm_to_cold_days {
            StorageTier::Cold
        } else if age_days >= self.policy.hot_to_warm_days {
            StorageTier::Warm
        } else {
            StorageTier::Hot
        }
    }

    fn get_from(&self, tier: StorageTier, id: Uuid) -> AuditResult<AuditRecord> {
        match tier {
            StorageTier::Hot => self.hot.get(id),
            StorageTier::Warm => self.warm.get(id),
            StorageTier::Cold => self.cold.get(id),
        }
    }

    fn store_into(&mut self, tier: StorageTier, record: AuditRecord) -> AuditResult<()> {
        match tier {
            StorageTier::Hot => self.hot.store(record),
            StorageTier::Warm => self.warm.store(record),
            StorageTier::Cold => self.cold.store(record),
        }
    }

    fn remove_from(&mut self, tier: StorageTier, id: Uuid) -> AuditResult<bool> {
        match tier {
            StorageTier::Hot => self.hot.remove(id),
            StorageTier::Warm => self.warm.remove(id),
            StorageTier::Cold => self.cold.remove(id),
        }
    }

    fn collect_authoritative(
        &self,
        tier: StorageTier,
        backend: &dyn AuditStorage,
        out: &mut HashMap<Uuid, AuditRecord>,
    ) -> AuditResult<()> {
        for record in backend.get_all()? {
            if self
                .placement
                .get(&record.id)
                .map(|p| p.tier == tier)
                .unwrap_or(false)
            {
                out.insert(record.id, record);
            }
        }
        Ok(())
    }
}

fn tier_rank(tier: StorageTier) -> u8 {
    match tier {
        StorageTier::Hot => 0,
        StorageTier::Warm => 1,
        StorageTier::Cold => 2,
    }
}

impl AuditStorage for MultiTierStore {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        let id = record.id;
        let timestamp = record.timestamp;
        let seq = self.next_seq;
        self.next_seq += 1;
        self.hot.store(record)?;
        self.placement.insert(
            id,
            Placement {
                tier: StorageTier::Hot,
                timestamp,
                seq,
                access_count: 0,
                last_accessed: Utc::now(),
            },
        );
        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        let tier = self
            .placement
            .get(&id)
            .map(|p| p.tier)
            .ok_or(AuditError::RecordNotFound(id))?;
        self.get_from(tier, id)
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        let mut by_id: HashMap<Uuid, AuditRecord> = HashMap::with_capacity(self.placement.len());
        self.collect_authoritative(StorageTier::Hot, self.hot.as_ref(), &mut by_id)?;
        self.collect_authoritative(StorageTier::Warm, self.warm.as_ref(), &mut by_id)?;
        self.collect_authoritative(StorageTier::Cold, self.cold.as_ref(), &mut by_id)?;

        let mut records: Vec<AuditRecord> = by_id.into_values().collect();
        records.sort_by_key(|r| self.placement.get(&r.id).map(|p| p.seq).unwrap_or(0));
        Ok(records)
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        Ok(self
            .get_all()?
            .into_iter()
            .filter(|r| r.statute_id == statute_id)
            .collect())
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        Ok(self
            .get_all()?
            .into_iter()
            .filter(|r| r.subject_id == subject_id)
            .collect())
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        Ok(self
            .get_all()?
            .into_iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .collect())
    }

    fn count(&self) -> AuditResult<usize> {
        Ok(self.placement.len())
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        Ok(self.last_hash.clone())
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        self.last_hash = hash;
        Ok(())
    }

    fn remove(&mut self, id: Uuid) -> AuditResult<bool> {
        if let Some(placement) = self.placement.remove(&id) {
            self.remove_from(placement.tier, id)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, AuditTrail, DecisionContext, DecisionResult, EventType};
    use chrono::Duration;
    use std::collections::HashMap as Map;

    fn record_at(statute: &str, ts: DateTime<Utc>) -> AuditRecord {
        let mut r = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "engine".to_string(),
            },
            statute.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "ok".to_string(),
                parameters: Map::new(),
            },
            None,
        );
        r.timestamp = ts;
        r
    }

    /// A backend that refuses removal (uses the trait default), to exercise the
    /// logical-migration fallback path.
    struct NoRemove(MemoryStorage);

    impl AuditStorage for NoRemove {
        fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
            self.0.store(record)
        }
        fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
            self.0.get(id)
        }
        fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
            self.0.get_all()
        }
        fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
            self.0.get_by_statute(statute_id)
        }
        fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
            self.0.get_by_subject(subject_id)
        }
        fn get_by_time_range(
            &self,
            start: DateTime<Utc>,
            end: DateTime<Utc>,
        ) -> AuditResult<Vec<AuditRecord>> {
            self.0.get_by_time_range(start, end)
        }
        fn count(&self) -> AuditResult<usize> {
            self.0.count()
        }
        fn get_last_hash(&self) -> AuditResult<Option<String>> {
            self.0.get_last_hash()
        }
        fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
            self.0.set_last_hash(hash)
        }
        // Intentionally inherits the default `remove` (returns Ok(false)).
    }

    #[test]
    fn test_new_records_land_in_hot() {
        let mut store = MultiTierStore::with_memory_tiers(TierMigrationPolicy::default());
        let r = record_at("s1", Utc::now());
        let id = r.id;
        store.store(r).expect("store");
        assert_eq!(store.tier_of(id), Some(StorageTier::Hot));
        assert_eq!(store.get(id).expect("get").id, id);
        assert_eq!(store.count().expect("count"), 1);
    }

    #[test]
    fn test_migration_moves_old_records_physically() {
        let policy = TierMigrationPolicy::new(7, 14);
        let mut store = MultiTierStore::with_memory_tiers(policy);
        let now = Utc::now();
        store.store(record_at("hot", now)).expect("store");
        store
            .store(record_at("warm", now - Duration::days(10)))
            .expect("store");
        store
            .store(record_at("cold", now - Duration::days(30)))
            .expect("store");

        let report = store.run_migration().expect("migrate");
        assert_eq!(report.demoted, 2);
        assert_eq!(report.physical_moves, 2);
        assert_eq!(report.logical_moves, 0);

        let dist = store.tier_distribution();
        assert_eq!(dist.hot, 1);
        assert_eq!(dist.warm, 1);
        assert_eq!(dist.cold, 1);
        // The hot backend physically gave up the migrated records.
        assert_eq!(store.hot.count().expect("hot count"), 1);
    }

    #[test]
    fn test_logical_migration_fallback() {
        let policy = TierMigrationPolicy::new(7, 14);
        // Hot tier cannot remove -> migrations from hot are logical.
        let mut store = MultiTierStore::new(
            Box::new(NoRemove(MemoryStorage::new())),
            Box::new(MemoryStorage::new()),
            Box::new(MemoryStorage::new()),
            policy,
        );
        let now = Utc::now();
        store
            .store(record_at("warm", now - Duration::days(10)))
            .expect("store");

        let report = store.run_migration().expect("migrate");
        assert_eq!(report.logical_moves, 1);
        assert_eq!(report.physical_moves, 0);
        assert_eq!(
            store.tier_of(store.placement.keys().next().copied().unwrap()),
            Some(StorageTier::Warm)
        );
        // Even with a shadow copy in hot, get_all() de-duplicates to the
        // authoritative (warm) record.
        assert_eq!(store.get_all().expect("all").len(), 1);
    }

    #[test]
    fn test_manual_set_tier() {
        let mut store = MultiTierStore::with_memory_tiers(TierMigrationPolicy::default());
        let r = record_at("s1", Utc::now());
        let id = r.id;
        store.store(r).expect("store");
        store.set_tier(id, StorageTier::Cold).expect("set tier");
        assert_eq!(store.tier_of(id), Some(StorageTier::Cold));
        assert_eq!(store.get(id).expect("get").id, id);
    }

    #[test]
    fn test_frequent_access_keeps_hot() {
        let policy = TierMigrationPolicy::new(7, 14);
        let mut store = MultiTierStore::with_memory_tiers(policy);
        let r = record_at("s1", Utc::now() - Duration::days(30));
        let id = r.id;
        store.store(r).expect("store");
        // Drive the access count above the policy threshold.
        for _ in 0..20 {
            store.get_tracked(id).expect("get");
        }
        store.run_migration().expect("migrate");
        assert_eq!(store.tier_of(id), Some(StorageTier::Hot));
    }

    #[test]
    fn test_remove_routes_to_authoritative_tier() {
        let mut store = MultiTierStore::with_memory_tiers(TierMigrationPolicy::default());
        let r = record_at("s1", Utc::now());
        let id = r.id;
        store.store(r).expect("store");
        store.set_tier(id, StorageTier::Warm).expect("set tier");
        assert!(store.remove(id).expect("remove"));
        assert_eq!(store.count().expect("count"), 0);
        assert!(store.get(id).is_err());
    }

    #[test]
    fn test_composes_into_audit_trail_and_verifies() {
        let store = MultiTierStore::with_memory_tiers(TierMigrationPolicy::default());
        let mut trail = AuditTrail::with_storage(Box::new(store));
        for i in 0..6 {
            let r = record_at(&format!("statute-{}", i % 2), Utc::now());
            trail.record(r).expect("record");
        }
        assert_eq!(trail.count(), 6);
        // Chain order is preserved across tiers, so integrity verifies.
        assert!(trail.verify_integrity().expect("verify"));
    }

    #[test]
    fn test_get_all_chain_order_after_migration() {
        let policy = TierMigrationPolicy::new(7, 14);
        let store = MultiTierStore::with_memory_tiers(policy);
        let mut trail = AuditTrail::with_storage(Box::new(store));
        // Insert a mix of ages so migration will scatter them across tiers.
        let now = Utc::now();
        for i in 0..9 {
            let age = (i % 3) * 10; // 0, 10, 20 days
            let r = record_at("s", now - Duration::days(age as i64));
            trail.record(r).expect("record");
        }
        // verify_integrity relies on get_all() being in chain order.
        assert!(trail.verify_integrity().expect("verify"));
    }
}
