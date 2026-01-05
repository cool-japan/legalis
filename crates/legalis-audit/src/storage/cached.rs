//! Cached storage wrapper for improved read performance.

use crate::storage::AuditStorage;
use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// LRU cache configuration.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of records to cache
    pub max_entries: usize,
    /// Time-to-live for cache entries (seconds)
    pub ttl_secs: u64,
    /// Enable cache statistics
    pub enable_stats: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            ttl_secs: 300, // 5 minutes
            enable_stats: true,
        }
    }
}

impl CacheConfig {
    /// Creates a new cache configuration.
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries,
            ..Default::default()
        }
    }

    /// Sets the TTL for cache entries.
    pub fn with_ttl(mut self, ttl_secs: u64) -> Self {
        self.ttl_secs = ttl_secs;
        self
    }

    /// Enables or disables cache statistics.
    pub fn with_stats(mut self, enable: bool) -> Self {
        self.enable_stats = enable;
        self
    }
}

/// Cache entry with expiration.
#[derive(Debug, Clone)]
struct CacheEntry {
    record: AuditRecord,
    inserted_at: DateTime<Utc>,
    last_accessed: DateTime<Utc>,
    access_count: usize,
}

impl CacheEntry {
    fn new(record: AuditRecord) -> Self {
        let now = Utc::now();
        Self {
            record,
            inserted_at: now,
            last_accessed: now,
            access_count: 1,
        }
    }

    fn is_expired(&self, ttl_secs: u64) -> bool {
        let elapsed = Utc::now().signed_duration_since(self.inserted_at);
        elapsed.num_seconds() > ttl_secs as i64
    }

    fn touch(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count += 1;
    }
}

/// Cache statistics.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: usize,
    /// Number of cache misses
    pub misses: usize,
    /// Number of evictions
    pub evictions: usize,
    /// Current cache size
    pub current_size: usize,
}

impl CacheStats {
    /// Returns the cache hit ratio.
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Resets the statistics.
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.evictions = 0;
    }
}

/// Cached storage wrapper with LRU eviction.
pub struct CachedStorage {
    storage: Box<dyn AuditStorage>,
    cache: Arc<RwLock<HashMap<Uuid, CacheEntry>>>,
    config: CacheConfig,
    stats: Arc<RwLock<CacheStats>>,
}

impl CachedStorage {
    /// Creates a new cached storage wrapper.
    pub fn new(storage: Box<dyn AuditStorage>, config: CacheConfig) -> Self {
        Self {
            storage,
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    /// Returns cache statistics.
    pub fn stats(&self) -> CacheStats {
        self.stats.read().unwrap().clone()
    }

    /// Clears the cache.
    pub fn clear_cache(&mut self) {
        self.cache.write().unwrap().clear();
        if self.config.enable_stats {
            let mut stats = self.stats.write().unwrap();
            stats.current_size = 0;
        }
    }

    /// Evicts expired entries from the cache.
    fn evict_expired(&self) {
        let mut cache = self.cache.write().unwrap();
        let ttl = self.config.ttl_secs;

        let expired_keys: Vec<_> = cache
            .iter()
            .filter(|(_, entry)| entry.is_expired(ttl))
            .map(|(k, _)| *k)
            .collect();

        let evicted_count = expired_keys.len();
        for key in expired_keys {
            cache.remove(&key);
        }

        if self.config.enable_stats && evicted_count > 0 {
            let mut stats = self.stats.write().unwrap();
            stats.evictions += evicted_count;
            stats.current_size = cache.len();
        }
    }

    /// Evicts least recently used entries if cache is full.
    fn evict_lru(&self) {
        let mut cache = self.cache.write().unwrap();

        if cache.len() >= self.config.max_entries {
            // Find LRU entry
            if let Some((&lru_key, _)) = cache.iter().min_by_key(|(_, entry)| entry.last_accessed) {
                cache.remove(&lru_key);

                if self.config.enable_stats {
                    let mut stats = self.stats.write().unwrap();
                    stats.evictions += 1;
                    stats.current_size = cache.len();
                }
            }
        }
    }

    /// Gets a record from cache or storage.
    fn get_cached(&self, id: Uuid) -> AuditResult<AuditRecord> {
        // Try cache first
        {
            let mut cache = self.cache.write().unwrap();
            if let Some(entry) = cache.get_mut(&id) {
                if !entry.is_expired(self.config.ttl_secs) {
                    entry.touch();

                    if self.config.enable_stats {
                        let mut stats = self.stats.write().unwrap();
                        stats.hits += 1;
                    }

                    return Ok(entry.record.clone());
                } else {
                    // Remove expired entry
                    cache.remove(&id);
                    if self.config.enable_stats {
                        let mut stats = self.stats.write().unwrap();
                        stats.evictions += 1;
                        stats.current_size = cache.len();
                    }
                }
            }
        }

        // Cache miss - fetch from storage
        if self.config.enable_stats {
            let mut stats = self.stats.write().unwrap();
            stats.misses += 1;
        }

        let record = self.storage.get(id)?;

        // Add to cache
        self.evict_lru();
        let mut cache = self.cache.write().unwrap();
        cache.insert(id, CacheEntry::new(record.clone()));

        if self.config.enable_stats {
            let mut stats = self.stats.write().unwrap();
            stats.current_size = cache.len();
        }

        Ok(record)
    }

    /// Invalidates a cache entry.
    fn invalidate(&self, id: Uuid) {
        let mut cache = self.cache.write().unwrap();
        cache.remove(&id);

        if self.config.enable_stats {
            let mut stats = self.stats.write().unwrap();
            stats.current_size = cache.len();
        }
    }
}

impl AuditStorage for CachedStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        let id = record.id;

        // Store in underlying storage
        self.storage.store(record.clone())?;

        // Invalidate cache entry to ensure consistency
        self.invalidate(id);

        // Evict expired entries periodically
        self.evict_expired();

        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        self.get_cached(id)
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        // For bulk operations, bypass cache and go directly to storage
        self.storage.get_all()
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        // For filtered queries, bypass cache
        self.storage.get_by_statute(statute_id)
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        // For filtered queries, bypass cache
        self.storage.get_by_subject(subject_id)
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        // For filtered queries, bypass cache
        self.storage.get_by_time_range(start, end)
    }

    fn count(&self) -> AuditResult<usize> {
        self.storage.count()
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        self.storage.get_last_hash()
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        self.storage.set_last_hash(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::memory::MemoryStorage;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn create_test_record() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_cached_storage_basic() {
        let storage = Box::new(MemoryStorage::new());
        let config = CacheConfig::new(10);
        let mut cached = CachedStorage::new(storage, config);

        let record = create_test_record();
        let id = record.id;

        cached.store(record).unwrap();
        let retrieved = cached.get(id).unwrap();
        assert_eq!(retrieved.id, id);
    }

    #[test]
    fn test_cache_hit() {
        let storage = Box::new(MemoryStorage::new());
        let config = CacheConfig::new(10);
        let mut cached = CachedStorage::new(storage, config);

        let record = create_test_record();
        let id = record.id;

        cached.store(record).unwrap();

        // First get - cache miss
        cached.get(id).unwrap();
        let stats = cached.stats();
        assert_eq!(stats.misses, 1);

        // Second get - cache hit
        cached.get(id).unwrap();
        let stats = cached.stats();
        assert_eq!(stats.hits, 1);
    }

    #[test]
    fn test_cache_eviction() {
        let storage = Box::new(MemoryStorage::new());
        let config = CacheConfig::new(3); // Small cache
        let mut cached = CachedStorage::new(storage, config);

        // Store 5 records (exceeds cache size)
        let ids: Vec<_> = (0..5)
            .map(|_| {
                let record = create_test_record();
                let id = record.id;
                cached.store(record).unwrap();
                id
            })
            .collect();

        // Try to get all records
        for id in ids {
            cached.get(id).unwrap();
        }

        let stats = cached.stats();
        assert!(stats.evictions > 0);
        assert_eq!(stats.current_size, 3); // Cache size limited to 3
    }

    #[test]
    fn test_cache_invalidation() {
        let storage = Box::new(MemoryStorage::new());
        let config = CacheConfig::new(10);
        let mut cached = CachedStorage::new(storage, config);

        let record = create_test_record();
        let id = record.id;

        cached.store(record.clone()).unwrap();
        cached.get(id).unwrap(); // Populate cache

        // Store again (should invalidate)
        cached.store(record).unwrap();

        // Next get should be a miss
        let stats_before = cached.stats();
        cached.get(id).unwrap();
        let stats_after = cached.stats();

        assert_eq!(stats_after.misses, stats_before.misses + 1);
    }

    #[test]
    fn test_cache_stats() {
        let storage = Box::new(MemoryStorage::new());
        let config = CacheConfig::new(10);
        let mut cached = CachedStorage::new(storage, config);

        let record = create_test_record();
        let id = record.id;

        cached.store(record).unwrap();

        for _ in 0..5 {
            cached.get(id).unwrap();
        }

        let stats = cached.stats();
        assert!(stats.hit_ratio() > 0.0);
        assert_eq!(stats.hits + stats.misses, 5);
    }

    #[test]
    fn test_clear_cache() {
        let storage = Box::new(MemoryStorage::new());
        let config = CacheConfig::new(10);
        let mut cached = CachedStorage::new(storage, config);

        let record = create_test_record();
        cached.store(record).unwrap();

        cached.clear_cache();
        let stats = cached.stats();
        assert_eq!(stats.current_size, 0);
    }
}
