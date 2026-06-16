//! Performance & scale layer for billion-record audit corpora.
//!
//! This module is the orchestration layer that turns the single-trail audit
//! primitives into something that stays fast and memory-bounded as the corpus
//! grows. It is deliberately *additive* and reuses the existing
//! [`AuditRecord`](crate::AuditRecord), [`AuditStorage`](crate::storage::AuditStorage),
//! and [`crate::storage::tiered`] types rather than re-modelling them.
//!
//! ## Building blocks
//! - [`index`] — a compact, multi-field **inverted index** ([`AuditIndex`]) with
//!   a `BTreeMap` time index and lazy (tombstone) deletion.
//! - [`query_accel`] — an [`QueryAccelerator`] / planner that turns a query into
//!   the cheapest set of index probes ("most selective first") and executes it,
//!   or transparently accelerates an existing
//!   [`QueryBuilder`](crate::query::QueryBuilder).
//! - [`cache`] — a [`ReadCache`] of whole query results with explicit,
//!   tag-based invalidation.
//! - [`codec`] — a pluggable [`Codec`] abstraction with a whole-batch
//!   [`DeflateCodec`] and an optimised columnar [`ColumnarCodec`].
//! - [`tiered_backend`] — a [`MultiTierStore`] that routes records to three
//!   pluggable backends and physically migrates them between tiers.
//!
//! ## Billion-record strategy
//! [`ScaleEngine`] ties the index and cache together over **segments**: the
//! index is sharded into fixed-capacity, time-bounded segments so per-index
//! memory and rebuild cost stay constant, and queries with a time range prune
//! whole segments before probing. A [`ReadCache`] short-circuits repeated
//! queries, and every ingest performs precise cache invalidation. The engine
//! resolves matches to ids and fetches the records from whatever
//! [`AuditStorage`](crate::storage::AuditStorage) backend the caller passes in,
//! so it composes with tiered/compressed storage without owning it.

pub mod cache;
pub mod codec;
pub mod index;
pub mod query_accel;
pub mod tiered_backend;

pub use cache::{ReadCache, ReadCacheConfig, ReadCacheStats};
pub use codec::{
    Codec, CodecComparison, ColumnarCodec, DeflateCodec, EncodedBlock, compare_codecs,
};
pub use index::{ActorKind, AuditIndex, IndexStats, ResultKind, RowId};
pub use query_accel::{AccessPath, AccessPlan, IndexQuery, QueryAccelerator, QuerySignature};
pub use tiered_backend::{MigrationReport, MultiTierStore, TierDistribution};

use crate::scale::query_accel::{fetch_records, finalize_records};
use crate::storage::AuditStorage;
use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Configuration for a [`ScaleEngine`].
#[derive(Debug, Clone)]
pub struct ScaleConfig {
    /// Maximum number of rows per index segment (bounds per-segment memory and
    /// rebuild cost). New rows open a fresh segment once this is reached.
    pub segment_capacity: usize,
    /// Read-cache configuration.
    pub cache: ReadCacheConfig,
}

impl Default for ScaleConfig {
    fn default() -> Self {
        Self {
            segment_capacity: 1_000_000,
            cache: ReadCacheConfig::default(),
        }
    }
}

impl ScaleConfig {
    /// Creates a config with an explicit segment capacity and default cache.
    pub fn with_segment_capacity(segment_capacity: usize) -> Self {
        Self {
            segment_capacity: segment_capacity.max(1),
            cache: ReadCacheConfig::default(),
        }
    }
}

/// A time-bounded shard of the inverted index.
struct Segment {
    index: AuditIndex,
    min_ts: Option<DateTime<Utc>>,
    max_ts: Option<DateTime<Utc>>,
}

impl Segment {
    fn new() -> Self {
        Self {
            index: AuditIndex::new(),
            min_ts: None,
            max_ts: None,
        }
    }

    fn add(&mut self, record: &AuditRecord) {
        self.index.insert(record);
        self.min_ts = Some(match self.min_ts {
            Some(cur) => cur.min(record.timestamp),
            None => record.timestamp,
        });
        self.max_ts = Some(match self.max_ts {
            Some(cur) => cur.max(record.timestamp),
            None => record.timestamp,
        });
    }

    /// Whether this segment's time span overlaps the (optional) query range.
    fn overlaps(&self, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> bool {
        match (self.min_ts, self.max_ts) {
            (Some(lo), Some(hi)) => {
                let above_start = start.map(|s| hi >= s).unwrap_or(true);
                let below_end = end.map(|e| lo <= e).unwrap_or(true);
                above_start && below_end
            }
            // An empty segment cannot match anything.
            _ => false,
        }
    }
}

/// Aggregate statistics for a [`ScaleEngine`].
#[derive(Debug, Clone)]
pub struct ScaleStats {
    /// Number of index segments.
    pub segments: usize,
    /// Total rows ingested (including tombstoned).
    pub total_rows: usize,
    /// Live (non-tombstoned) rows across all segments.
    pub live_rows: usize,
    /// Read-cache statistics.
    pub cache: ReadCacheStats,
}

/// The billion-record scale engine: a segmented index plus a read cache.
pub struct ScaleEngine {
    segments: Vec<Segment>,
    config: ScaleConfig,
    cache: ReadCache,
    total_rows: usize,
}

impl ScaleEngine {
    /// Creates an engine with the given configuration.
    pub fn new(config: ScaleConfig) -> Self {
        let cache = ReadCache::new(config.cache.clone());
        Self {
            segments: Vec::new(),
            config,
            cache,
            total_rows: 0,
        }
    }

    /// Creates an engine with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(ScaleConfig::default())
    }

    /// Builds an engine pre-populated from existing records (e.g. recovered from
    /// a backend at start-up).
    pub fn from_records(config: ScaleConfig, records: &[AuditRecord]) -> Self {
        let mut engine = Self::new(config);
        engine.ingest_many(records);
        engine
    }

    /// Indexes a single record and invalidates any cached query it could affect.
    pub fn ingest(&mut self, record: &AuditRecord) {
        let need_new = self
            .segments
            .last()
            .map(|s| s.index.len() >= self.config.segment_capacity)
            .unwrap_or(true);
        if need_new {
            self.segments.push(Segment::new());
        }
        if let Some(segment) = self.segments.last_mut() {
            segment.add(record);
        }
        self.total_rows += 1;
        self.cache.invalidate_record(record);
    }

    /// Indexes a batch of records.
    pub fn ingest_many(&mut self, records: &[AuditRecord]) {
        for record in records {
            self.ingest(record);
        }
    }

    /// Number of index segments.
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Runs a query: cache → segment pruning → per-segment index probe → fetch →
    /// exact filter / order / paginate → cache fill.
    pub fn query(
        &self,
        query: &IndexQuery,
        storage: &dyn AuditStorage,
    ) -> AuditResult<Vec<AuditRecord>> {
        if let Some(hit) = self.cache.get_query(query) {
            return Ok(hit);
        }

        let ids = self.candidate_ids(query);
        let records = fetch_records(storage, &ids)?;
        let result = finalize_records(records, query);

        self.cache.put_query(query, result.clone());
        Ok(result)
    }

    /// Accelerates a [`QueryBuilder`] across all segments. This path does not use
    /// the result cache (the builder carries predicates beyond the index
    /// signature), but still benefits from index probing and segment pruning.
    pub fn query_builder(
        &self,
        builder: &crate::query::QueryBuilder,
        storage: &dyn AuditStorage,
    ) -> AuditResult<Vec<AuditRecord>> {
        let iq = IndexQuery::from_query_builder(builder);
        let ids = self.candidate_ids(&iq);
        let records = fetch_records(storage, &ids)?;
        Ok(builder.execute(&records))
    }

    /// Collects candidate record ids across all relevant segments.
    fn candidate_ids(&self, query: &IndexQuery) -> Vec<Uuid> {
        let mut ids: Vec<Uuid> = Vec::new();
        for segment in &self.segments {
            if !segment.overlaps(query.start, query.end) {
                continue;
            }
            let accelerator = QueryAccelerator::new(&segment.index);
            let rows = accelerator.candidate_rows(query);
            ids.extend(segment.index.resolve(&rows));
        }
        ids
    }

    /// Invalidates cached queries that depend on a statute.
    pub fn invalidate_statute(&self, statute_id: &str) -> usize {
        self.cache.invalidate_statute(statute_id)
    }

    /// Invalidates cached queries that depend on a subject.
    pub fn invalidate_subject(&self, subject_id: Uuid) -> usize {
        self.cache.invalidate_subject(subject_id)
    }

    /// Clears the entire read cache.
    pub fn invalidate_all(&self) -> usize {
        self.cache.invalidate_all()
    }

    /// Compacts every segment, reclaiming tombstoned rows.
    pub fn compact(&mut self) {
        for segment in &mut self.segments {
            segment.index.compact();
        }
    }

    /// A snapshot of engine statistics.
    pub fn stats(&self) -> ScaleStats {
        let live_rows = self.segments.iter().map(|s| s.index.live_len()).sum();
        ScaleStats {
            segments: self.segments.len(),
            total_rows: self.total_rows,
            live_rows,
            cache: self.cache.stats(),
        }
    }
}

impl Default for ScaleEngine {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::memory::MemoryStorage;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use chrono::Duration;
    use std::collections::HashMap as Map;

    fn seed(n: usize) -> (MemoryStorage, Vec<AuditRecord>) {
        let mut storage = MemoryStorage::new();
        let base = Utc::now();
        let mut records = Vec::new();
        for i in 0..n {
            let mut r = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "engine".to_string(),
                },
                format!("statute-{}", i % 4),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "ok".to_string(),
                    parameters: Map::new(),
                },
                None,
            );
            r.timestamp = base + Duration::minutes(i as i64);
            storage.store(r.clone()).expect("store");
            records.push(r);
        }
        (storage, records)
    }

    #[test]
    fn test_engine_query_matches_filter() {
        let (storage, records) = seed(40);
        let engine = ScaleEngine::from_records(ScaleConfig::default(), &records);

        let iq = IndexQuery::new().statute("statute-2");
        let got = engine.query(&iq, &storage).expect("query");
        let want = records
            .iter()
            .filter(|r| r.statute_id == "statute-2")
            .count();
        assert_eq!(got.len(), want);
        assert!(got.iter().all(|r| r.statute_id == "statute-2"));
    }

    #[test]
    fn test_segmentation_bounds_memory() {
        let (storage, records) = seed(25);
        let engine = ScaleEngine::from_records(ScaleConfig::with_segment_capacity(10), &records);
        // 25 rows / 10 per segment => 3 segments.
        assert_eq!(engine.segment_count(), 3);

        // Queries still return correct results across segments.
        let iq = IndexQuery::new().statute("statute-0");
        let got = engine.query(&iq, &storage).expect("query");
        let want = records
            .iter()
            .filter(|r| r.statute_id == "statute-0")
            .count();
        assert_eq!(got.len(), want);
    }

    #[test]
    fn test_cache_hit_on_repeat() {
        let (storage, records) = seed(30);
        let engine = ScaleEngine::from_records(ScaleConfig::default(), &records);
        let iq = IndexQuery::new().statute("statute-1");

        let first = engine.query(&iq, &storage).expect("query");
        let second = engine.query(&iq, &storage).expect("query");
        assert_eq!(first.len(), second.len());

        let stats = engine.stats();
        assert_eq!(stats.cache.hits, 1);
        assert_eq!(stats.cache.misses, 1);
    }

    #[test]
    fn test_ingest_invalidates_cache() {
        let (storage, records) = seed(20);
        let mut engine = ScaleEngine::from_records(ScaleConfig::default(), &records);
        let iq = IndexQuery::new().statute("statute-1");
        let _ = engine.query(&iq, &storage).expect("query");
        assert_eq!(engine.stats().cache.entries, 1);

        // Ingesting a new statute-1 record must drop the cached result.
        let base = records[0].timestamp;
        let mut new_record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "engine".to_string(),
            },
            "statute-1".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "ok".to_string(),
                parameters: Map::new(),
            },
            None,
        );
        new_record.timestamp = base + Duration::minutes(999);
        engine.ingest(&new_record);
        assert_eq!(engine.stats().cache.entries, 0);
    }

    #[test]
    fn test_time_range_segment_pruning() {
        let (storage, records) = seed(30);
        let engine = ScaleEngine::from_records(ScaleConfig::with_segment_capacity(10), &records);
        let base = records[0].timestamp;
        // Window entirely inside the first segment (minutes 0..9).
        let iq =
            IndexQuery::new().time_range(base + Duration::minutes(2), base + Duration::minutes(5));
        let got = engine.query(&iq, &storage).expect("query");
        assert_eq!(got.len(), 4); // minutes 2,3,4,5
        assert!(got.windows(2).all(|w| w[0].timestamp <= w[1].timestamp));
    }

    #[test]
    fn test_query_builder_path() {
        use crate::query::{ActorFilter, QueryBuilder};
        let (storage, records) = seed(30);
        let engine = ScaleEngine::from_records(ScaleConfig::default(), &records);
        let builder = QueryBuilder::new()
            .statute_id("statute-3")
            .actor(ActorFilter::AnySystem)
            .limit(2);
        let got = engine.query_builder(&builder, &storage).expect("query");
        let want = builder.execute(&records);
        assert_eq!(got.len(), want.len());
        assert!(got.len() <= 2);
    }

    #[test]
    fn test_compact_after_no_ops() {
        let (_storage, records) = seed(15);
        let mut engine = ScaleEngine::from_records(ScaleConfig::with_segment_capacity(5), &records);
        engine.compact(); // no tombstones -> no-op, must not panic or change counts
        assert_eq!(engine.stats().live_rows, 15);
    }

    #[test]
    fn test_stats_report() {
        let (_storage, records) = seed(12);
        let engine = ScaleEngine::from_records(ScaleConfig::with_segment_capacity(5), &records);
        let stats = engine.stats();
        assert_eq!(stats.total_rows, 12);
        assert_eq!(stats.live_rows, 12);
        assert_eq!(stats.segments, 3);
    }
}
