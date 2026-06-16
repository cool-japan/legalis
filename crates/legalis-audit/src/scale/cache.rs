//! Query-result read cache with explicit invalidation.
//!
//! This complements (it does not duplicate) [`crate::storage::cached`], which is
//! a per-*record* `get(id)` LRU. Here the unit of caching is a whole **query
//! result set**, keyed by a [`QuerySignature`], which is what makes repeated
//! dashboard / report queries cheap.
//!
//! Correctness hinges on invalidation. Each cached entry records the *tags* it
//! depends on — the statutes and subjects its query constrained — plus a
//! "broad" flag for unconstrained queries. Writes then invalidate precisely:
//!
//! - [`ReadCache::invalidate_statute`] / [`ReadCache::invalidate_subject`] drop
//!   only the entries that depend on that statute/subject (`O(affected)` via a
//!   reverse tag index).
//! - [`ReadCache::invalidate_record`] drops every entry a freshly written record
//!   could affect: those tagged with its statute, those tagged with its
//!   subject, and *all broad* (unconstrained) entries — a conservative,
//!   provably-safe superset.
//! - [`ReadCache::invalidate_all`] clears the cache and bumps the generation
//!   counter.
//!
//! TTL- and capacity-based (LRU) eviction run alongside the explicit hooks.

use crate::AuditRecord;
use crate::scale::query_accel::{IndexQuery, QuerySignature};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, PoisonError};
use uuid::Uuid;

/// Configuration for a [`ReadCache`].
#[derive(Debug, Clone)]
pub struct ReadCacheConfig {
    /// Maximum number of cached query results (0 disables capacity eviction).
    pub max_entries: usize,
    /// Time-to-live in seconds (0 disables time-based expiry).
    pub ttl_secs: u64,
}

impl Default for ReadCacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 256,
            ttl_secs: 300,
        }
    }
}

impl ReadCacheConfig {
    /// Creates a config with the given capacity and default TTL.
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries,
            ..Default::default()
        }
    }

    /// Sets the TTL in seconds.
    pub fn with_ttl(mut self, ttl_secs: u64) -> Self {
        self.ttl_secs = ttl_secs;
        self
    }
}

/// Runtime statistics for a [`ReadCache`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReadCacheStats {
    /// Cache hits.
    pub hits: usize,
    /// Cache misses.
    pub misses: usize,
    /// Entries inserted.
    pub insertions: usize,
    /// Entries dropped by explicit invalidation.
    pub invalidations: usize,
    /// Entries dropped by TTL/LRU eviction.
    pub evictions: usize,
    /// Current number of live entries.
    pub entries: usize,
}

impl ReadCacheStats {
    /// Hit ratio over all lookups (0.0 when there were none).
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

struct Entry {
    records: Vec<AuditRecord>,
    inserted_at: DateTime<Utc>,
    last_accessed: DateTime<Utc>,
    generation: u64,
    statute_tags: Vec<String>,
    subject_tags: Vec<Uuid>,
    broad: bool,
}

#[derive(Default)]
struct Inner {
    entries: HashMap<QuerySignature, Entry>,
    statute_index: HashMap<String, HashSet<QuerySignature>>,
    subject_index: HashMap<Uuid, HashSet<QuerySignature>>,
    broad: HashSet<QuerySignature>,
    stats: ReadCacheStats,
    generation: u64,
}

/// A thread-safe query-result cache with explicit, tag-based invalidation.
pub struct ReadCache {
    config: ReadCacheConfig,
    inner: Mutex<Inner>,
}

impl ReadCache {
    /// Creates a cache with the given configuration.
    pub fn new(config: ReadCacheConfig) -> Self {
        Self {
            config,
            inner: Mutex::new(Inner::default()),
        }
    }

    /// Looks up a previously cached result by signature.
    pub fn get(&self, signature: QuerySignature) -> Option<Vec<AuditRecord>> {
        let mut inner = self.lock();
        let now = Utc::now();
        let generation = inner.generation;
        let expired = match inner.entries.get(&signature) {
            Some(entry) => self.is_expired(entry, now, generation),
            None => {
                inner.stats.misses += 1;
                return None;
            }
        };
        if expired {
            remove_entry(&mut inner, signature);
            inner.stats.evictions += 1;
            inner.stats.misses += 1;
            inner.stats.entries = inner.entries.len();
            return None;
        }
        if let Some(entry) = inner.entries.get_mut(&signature) {
            entry.last_accessed = now;
            let records = entry.records.clone();
            inner.stats.hits += 1;
            return Some(records);
        }
        inner.stats.misses += 1;
        None
    }

    /// Convenience: look up by an [`IndexQuery`].
    pub fn get_query(&self, query: &IndexQuery) -> Option<Vec<AuditRecord>> {
        self.get(query.signature())
    }

    /// Inserts a result under an explicit signature with explicit tags.
    pub fn put(
        &self,
        signature: QuerySignature,
        records: Vec<AuditRecord>,
        statute_tags: Vec<String>,
        subject_tags: Vec<Uuid>,
    ) {
        let mut inner = self.lock();
        // Replace any existing entry (and its tag links) first.
        if inner.entries.contains_key(&signature) {
            remove_entry(&mut inner, signature);
        }

        let broad = statute_tags.is_empty() && subject_tags.is_empty();
        for tag in &statute_tags {
            inner
                .statute_index
                .entry(tag.clone())
                .or_default()
                .insert(signature);
        }
        for tag in &subject_tags {
            inner
                .subject_index
                .entry(*tag)
                .or_default()
                .insert(signature);
        }
        if broad {
            inner.broad.insert(signature);
        }

        let now = Utc::now();
        let generation = inner.generation;
        inner.entries.insert(
            signature,
            Entry {
                records,
                inserted_at: now,
                last_accessed: now,
                generation,
                statute_tags,
                subject_tags,
                broad,
            },
        );
        inner.stats.insertions += 1;
        inner.stats.entries = inner.entries.len();

        self.evict_if_needed(&mut inner);
    }

    /// Convenience: insert a result for an [`IndexQuery`], deriving signature and
    /// tags from the query itself.
    pub fn put_query(&self, query: &IndexQuery, records: Vec<AuditRecord>) {
        self.put(
            query.signature(),
            records,
            query.statute_tags(),
            query.subject_tags(),
        );
    }

    /// Invalidates a single entry by signature. Returns `true` if present.
    pub fn invalidate_key(&self, signature: QuerySignature) -> bool {
        let mut inner = self.lock();
        if inner.entries.contains_key(&signature) {
            remove_entry(&mut inner, signature);
            inner.stats.invalidations += 1;
            inner.stats.entries = inner.entries.len();
            true
        } else {
            false
        }
    }

    /// Invalidates every entry that depends on `statute_id`. Returns the count.
    pub fn invalidate_statute(&self, statute_id: &str) -> usize {
        let mut inner = self.lock();
        let sigs: Vec<QuerySignature> = inner
            .statute_index
            .get(statute_id)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        self.drop_all(&mut inner, &sigs)
    }

    /// Invalidates every entry that depends on `subject_id`. Returns the count.
    pub fn invalidate_subject(&self, subject_id: Uuid) -> usize {
        let mut inner = self.lock();
        let sigs: Vec<QuerySignature> = inner
            .subject_index
            .get(&subject_id)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        self.drop_all(&mut inner, &sigs)
    }

    /// Invalidates every entry a newly written `record` could affect: those
    /// tagged with its statute, those tagged with its subject, and *all broad*
    /// (unconstrained) entries. Returns the count of distinct entries dropped.
    pub fn invalidate_record(&self, record: &AuditRecord) -> usize {
        let mut inner = self.lock();
        let mut sigs: HashSet<QuerySignature> = HashSet::new();
        if let Some(s) = inner.statute_index.get(&record.statute_id) {
            sigs.extend(s.iter().copied());
        }
        if let Some(s) = inner.subject_index.get(&record.subject_id) {
            sigs.extend(s.iter().copied());
        }
        sigs.extend(inner.broad.iter().copied());
        let sigs: Vec<QuerySignature> = sigs.into_iter().collect();
        self.drop_all(&mut inner, &sigs)
    }

    /// Clears the cache and bumps the generation counter. Returns count cleared.
    pub fn invalidate_all(&self) -> usize {
        let mut inner = self.lock();
        let cleared = inner.entries.len();
        inner.entries.clear();
        inner.statute_index.clear();
        inner.subject_index.clear();
        inner.broad.clear();
        inner.generation = inner.generation.wrapping_add(1);
        inner.stats.invalidations += cleared;
        inner.stats.entries = 0;
        cleared
    }

    /// Number of live entries.
    pub fn len(&self) -> usize {
        self.lock().entries.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.lock().entries.is_empty()
    }

    /// The current generation counter (incremented by [`Self::invalidate_all`]).
    pub fn generation(&self) -> u64 {
        self.lock().generation
    }

    /// A snapshot of cache statistics.
    pub fn stats(&self) -> ReadCacheStats {
        let mut inner = self.lock();
        inner.stats.entries = inner.entries.len();
        inner.stats.clone()
    }

    fn drop_all(&self, inner: &mut Inner, sigs: &[QuerySignature]) -> usize {
        let mut count = 0;
        for sig in sigs {
            if inner.entries.contains_key(sig) {
                remove_entry(inner, *sig);
                count += 1;
            }
        }
        inner.stats.invalidations += count;
        inner.stats.entries = inner.entries.len();
        count
    }

    fn evict_if_needed(&self, inner: &mut Inner) {
        if self.config.max_entries == 0 {
            return;
        }
        while inner.entries.len() > self.config.max_entries {
            let lru = inner
                .entries
                .iter()
                .min_by_key(|(_, e)| e.last_accessed)
                .map(|(k, _)| *k);
            match lru {
                Some(sig) => {
                    remove_entry(inner, sig);
                    inner.stats.evictions += 1;
                }
                None => break,
            }
        }
        inner.stats.entries = inner.entries.len();
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn is_expired(&self, entry: &Entry, now: DateTime<Utc>, generation: u64) -> bool {
        if entry.generation != generation {
            return true;
        }
        if self.config.ttl_secs == 0 {
            return false;
        }
        now.signed_duration_since(entry.inserted_at).num_seconds() > self.config.ttl_secs as i64
    }
}

fn remove_entry(inner: &mut Inner, signature: QuerySignature) {
    if let Some(entry) = inner.entries.remove(&signature) {
        for tag in &entry.statute_tags {
            if let Some(set) = inner.statute_index.get_mut(tag) {
                set.remove(&signature);
                if set.is_empty() {
                    inner.statute_index.remove(tag);
                }
            }
        }
        for tag in &entry.subject_tags {
            if let Some(set) = inner.subject_index.get_mut(tag) {
                set.remove(&signature);
                if set.is_empty() {
                    inner.subject_index.remove(tag);
                }
            }
        }
        if entry.broad {
            inner.broad.remove(&signature);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::index::ResultKind;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as Map;

    fn record(statute: &str, subject: Uuid) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "engine".to_string(),
            },
            statute.to_string(),
            subject,
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "ok".to_string(),
                parameters: Map::new(),
            },
            None,
        )
    }

    #[test]
    fn test_put_get_hit_and_miss() {
        let cache = ReadCache::new(ReadCacheConfig::default());
        let q = IndexQuery::new().statute("s1");
        assert!(cache.get_query(&q).is_none());

        let recs = vec![record("s1", Uuid::new_v4())];
        cache.put_query(&q, recs.clone());
        let hit = cache.get_query(&q).expect("hit");
        assert_eq!(hit.len(), 1);

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert!((stats.hit_ratio() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_invalidate_statute_targets_only_dependents() {
        let cache = ReadCache::new(ReadCacheConfig::default());
        let q1 = IndexQuery::new().statute("s1");
        let q2 = IndexQuery::new().statute("s2");
        cache.put_query(&q1, vec![record("s1", Uuid::new_v4())]);
        cache.put_query(&q2, vec![record("s2", Uuid::new_v4())]);
        assert_eq!(cache.len(), 2);

        let dropped = cache.invalidate_statute("s1");
        assert_eq!(dropped, 1);
        assert!(cache.get_query(&q1).is_none());
        assert!(cache.get_query(&q2).is_some());
    }

    #[test]
    fn test_invalidate_subject() {
        let cache = ReadCache::new(ReadCacheConfig::default());
        let subj = Uuid::new_v4();
        let q = IndexQuery::new().subject(subj);
        cache.put_query(&q, vec![record("s1", subj)]);
        assert_eq!(cache.invalidate_subject(subj), 1);
        assert!(cache.get_query(&q).is_none());
        assert_eq!(cache.invalidate_subject(Uuid::new_v4()), 0);
    }

    #[test]
    fn test_invalidate_record_hits_statute_and_broad_keeps_unrelated() {
        let cache = ReadCache::new(ReadCacheConfig::default());
        let q_s1 = IndexQuery::new().statute("s1");
        let q_s2 = IndexQuery::new().statute("s2");
        // A "broad" query: constrained only by result kind, no statute/subject.
        let q_broad = IndexQuery::new().result_kind(ResultKind::Deterministic);
        cache.put_query(&q_s1, vec![record("s1", Uuid::new_v4())]);
        cache.put_query(&q_s2, vec![record("s2", Uuid::new_v4())]);
        cache.put_query(&q_broad, vec![record("s1", Uuid::new_v4())]);
        assert_eq!(cache.len(), 3);

        // A new record for statute s1 must drop the s1-scoped and the broad
        // entry, but leave the s2-scoped entry intact.
        let dropped = cache.invalidate_record(&record("s1", Uuid::new_v4()));
        assert_eq!(dropped, 2);
        assert!(cache.get_query(&q_s1).is_none());
        assert!(cache.get_query(&q_broad).is_none());
        assert!(cache.get_query(&q_s2).is_some());
    }

    #[test]
    fn test_invalidate_all_bumps_generation() {
        let cache = ReadCache::new(ReadCacheConfig::default());
        let g0 = cache.generation();
        cache.put_query(
            &IndexQuery::new().statute("s1"),
            vec![record("s1", Uuid::new_v4())],
        );
        let cleared = cache.invalidate_all();
        assert_eq!(cleared, 1);
        assert!(cache.is_empty());
        assert_eq!(cache.generation(), g0 + 1);
    }

    #[test]
    fn test_capacity_eviction() {
        let cache = ReadCache::new(ReadCacheConfig::new(2).with_ttl(0));
        for i in 0..3 {
            let q = IndexQuery::new().statute(format!("s{}", i));
            cache.put_query(&q, vec![record(&format!("s{}", i), Uuid::new_v4())]);
        }
        assert_eq!(cache.len(), 2);
        let stats = cache.stats();
        assert!(stats.evictions >= 1);
    }

    #[test]
    fn test_put_replaces_existing_entry() {
        let cache = ReadCache::new(ReadCacheConfig::default());
        let q = IndexQuery::new().statute("s1");
        cache.put_query(&q, vec![record("s1", Uuid::new_v4())]);
        cache.put_query(
            &q,
            vec![record("s1", Uuid::new_v4()), record("s1", Uuid::new_v4())],
        );
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get_query(&q).expect("hit").len(), 2);
    }

    #[test]
    fn test_no_ttl_does_not_expire() {
        let cache = ReadCache::new(ReadCacheConfig::new(8).with_ttl(0));
        let q = IndexQuery::new().statute("s1");
        cache.put_query(&q, vec![record("s1", Uuid::new_v4())]);
        // Multiple reads, all hits.
        for _ in 0..3 {
            assert!(cache.get_query(&q).is_some());
        }
        assert_eq!(cache.stats().hits, 3);
    }
}
