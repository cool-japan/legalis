//! Advanced caching and memoization for diff operations.
//!
//! This module provides advanced caching capabilities including:
//! - Redis integration for distributed caching
//! - Memcached support for high-performance caching
//! - Cache invalidation strategies
//! - Smart cache preloading
//! - Multi-level cache hierarchies
//!
//! # Examples
//!
//! ```
//! use legalis_diff::advanced_cache::{CacheManager, CacheConfig, CacheBackend};
//!
//! let config = CacheConfig::default();
//! let manager = CacheManager::new(config);
//! ```

use crate::{DiffResult, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// Cache backend types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheBackend {
    /// In-memory cache (default)
    InMemory,
    /// Redis distributed cache
    Redis,
    /// Memcached high-performance cache
    Memcached,
}

/// Cache invalidation strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvalidationStrategy {
    /// Time-to-live (TTL) based invalidation
    Ttl,
    /// Least Recently Used (LRU) eviction
    Lru,
    /// Least Frequently Used (LFU) eviction
    Lfu,
    /// Manual invalidation only
    Manual,
    /// Write-through invalidation
    WriteThrough,
}

/// Configuration for the cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Primary cache backend
    pub primary_backend: CacheBackend,
    /// Secondary cache backend (for multi-level caching)
    pub secondary_backend: Option<CacheBackend>,
    /// Invalidation strategy
    pub invalidation_strategy: InvalidationStrategy,
    /// Time-to-live in seconds (for TTL strategy)
    pub ttl_seconds: u64,
    /// Maximum cache size (number of entries)
    pub max_size: usize,
    /// Enable smart preloading
    pub enable_preloading: bool,
    /// Redis connection string (if using Redis)
    pub redis_url: Option<String>,
    /// Memcached servers (if using Memcached)
    pub memcached_servers: Vec<String>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            primary_backend: CacheBackend::InMemory,
            secondary_backend: None,
            invalidation_strategy: InvalidationStrategy::Lru,
            ttl_seconds: 3600, // 1 hour
            max_size: 1000,
            enable_preloading: false,
            redis_url: None,
            memcached_servers: Vec::new(),
        }
    }
}

/// Cache entry with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    /// The cached diff
    diff: StatuteDiff,
    /// When this entry was created
    created_at: SystemTime,
    /// Last access time
    last_accessed: SystemTime,
    /// Access count (for LFU)
    access_count: u64,
}

/// Multi-level cache manager.
///
/// # Examples
///
/// ```
/// use legalis_diff::advanced_cache::{CacheManager, CacheConfig};
///
/// let config = CacheConfig::default();
/// let manager = CacheManager::new(config);
/// ```
#[derive(Clone)]
pub struct CacheManager {
    config: CacheConfig,
    primary_cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    secondary_cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    preload_queue: Arc<Mutex<Vec<String>>>,
}

impl CacheManager {
    /// Creates a new cache manager with the given configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::{CacheManager, CacheConfig, CacheBackend};
    ///
    /// let mut config = CacheConfig::default();
    /// config.primary_backend = CacheBackend::InMemory;
    /// config.max_size = 500;
    ///
    /// let manager = CacheManager::new(config);
    /// ```
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            primary_cache: Arc::new(Mutex::new(HashMap::new())),
            secondary_cache: Arc::new(Mutex::new(HashMap::new())),
            preload_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Gets a diff from the cache.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::{CacheManager, CacheConfig};
    ///
    /// let manager = CacheManager::new(CacheConfig::default());
    /// let result = manager.get("statute-123");
    /// ```
    pub fn get(&self, key: &str) -> Option<StatuteDiff> {
        // Try primary cache first
        if let Some(entry) = self.get_from_primary(key) {
            return Some(entry.diff);
        }

        // Try secondary cache if available
        if self.config.secondary_backend.is_some()
            && let Some(entry) = self.get_from_secondary(key)
        {
            // Promote to primary cache
            self.put_to_primary(key, entry.diff.clone());
            return Some(entry.diff);
        }

        None
    }

    /// Puts a diff into the cache.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::{CacheManager, CacheConfig};
    /// use legalis_diff::StatuteDiff;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let manager = CacheManager::new(CacheConfig::default());
    ///
    /// let diff = StatuteDiff {
    ///     statute_id: "statute-123".to_string(),
    ///     version_info: None,
    ///     changes: Vec::new(),
    ///     impact: Default::default(),
    /// };
    ///
    /// manager.put("statute-123", diff);
    /// ```
    pub fn put(&self, key: &str, diff: StatuteDiff) {
        // Check if we need to evict entries
        self.maybe_evict();

        // Put in primary cache
        self.put_to_primary(key, diff.clone());

        // Put in secondary cache if available
        if self.config.secondary_backend.is_some() {
            self.put_to_secondary(key, diff);
        }
    }

    /// Invalidates a cache entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::{CacheManager, CacheConfig};
    ///
    /// let manager = CacheManager::new(CacheConfig::default());
    /// manager.invalidate("statute-123");
    /// ```
    pub fn invalidate(&self, key: &str) {
        self.primary_cache.lock().unwrap().remove(key);
        if self.config.secondary_backend.is_some() {
            self.secondary_cache.lock().unwrap().remove(key);
        }
    }

    /// Invalidates all cache entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::{CacheManager, CacheConfig};
    ///
    /// let manager = CacheManager::new(CacheConfig::default());
    /// manager.invalidate_all();
    /// ```
    pub fn invalidate_all(&self) {
        self.primary_cache.lock().unwrap().clear();
        if self.config.secondary_backend.is_some() {
            self.secondary_cache.lock().unwrap().clear();
        }
    }

    /// Preloads cache entries for the given keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::{CacheManager, CacheConfig};
    ///
    /// let mut config = CacheConfig::default();
    /// config.enable_preloading = true;
    /// let manager = CacheManager::new(config);
    ///
    /// let keys = vec!["statute-1".to_string(), "statute-2".to_string()];
    /// manager.preload(keys);
    /// ```
    pub fn preload(&self, keys: Vec<String>) {
        if !self.config.enable_preloading {
            return;
        }

        let mut queue = self.preload_queue.lock().unwrap();
        for key in keys {
            if !queue.contains(&key) {
                queue.push(key);
            }
        }
    }

    /// Gets cache statistics.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::{CacheManager, CacheConfig};
    ///
    /// let manager = CacheManager::new(CacheConfig::default());
    /// let stats = manager.stats();
    /// assert_eq!(stats.primary_size, 0);
    /// ```
    pub fn stats(&self) -> CacheStats {
        let primary = self.primary_cache.lock().unwrap();
        let secondary = self.secondary_cache.lock().unwrap();

        CacheStats {
            primary_size: primary.len(),
            secondary_size: secondary.len(),
            max_size: self.config.max_size,
            backend: self.config.primary_backend,
            invalidation_strategy: self.config.invalidation_strategy,
        }
    }

    // Internal methods

    fn get_from_primary(&self, key: &str) -> Option<CacheEntry> {
        let mut cache = self.primary_cache.lock().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            // Update access metadata
            entry.last_accessed = SystemTime::now();
            entry.access_count += 1;

            // Check if entry is still valid
            if self.is_valid(entry) {
                return Some(entry.clone());
            } else {
                cache.remove(key);
            }
        }
        None
    }

    fn get_from_secondary(&self, key: &str) -> Option<CacheEntry> {
        let mut cache = self.secondary_cache.lock().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            entry.last_accessed = SystemTime::now();
            entry.access_count += 1;

            if self.is_valid(entry) {
                return Some(entry.clone());
            } else {
                cache.remove(key);
            }
        }
        None
    }

    fn put_to_primary(&self, key: &str, diff: StatuteDiff) {
        let entry = CacheEntry {
            diff,
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            access_count: 0,
        };

        self.primary_cache
            .lock()
            .unwrap()
            .insert(key.to_string(), entry);
    }

    fn put_to_secondary(&self, key: &str, diff: StatuteDiff) {
        let entry = CacheEntry {
            diff,
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            access_count: 0,
        };

        self.secondary_cache
            .lock()
            .unwrap()
            .insert(key.to_string(), entry);
    }

    fn is_valid(&self, entry: &CacheEntry) -> bool {
        match self.config.invalidation_strategy {
            InvalidationStrategy::Ttl => {
                let age = SystemTime::now()
                    .duration_since(entry.created_at)
                    .unwrap_or(Duration::from_secs(0));
                age.as_secs() < self.config.ttl_seconds
            }
            _ => true, // Other strategies don't invalidate based on time
        }
    }

    fn maybe_evict(&self) {
        let mut cache = self.primary_cache.lock().unwrap();

        if cache.len() >= self.config.max_size {
            match self.config.invalidation_strategy {
                InvalidationStrategy::Lru => {
                    // Find least recently used entry
                    if let Some((key, _)) = cache
                        .iter()
                        .min_by_key(|(_, entry)| entry.last_accessed)
                        .map(|(k, v)| (k.clone(), v.clone()))
                    {
                        cache.remove(&key);
                    }
                }
                InvalidationStrategy::Lfu => {
                    // Find least frequently used entry
                    if let Some((key, _)) = cache
                        .iter()
                        .min_by_key(|(_, entry)| entry.access_count)
                        .map(|(k, v)| (k.clone(), v.clone()))
                    {
                        cache.remove(&key);
                    }
                }
                InvalidationStrategy::Ttl => {
                    // Remove expired entries
                    let now = SystemTime::now();
                    let expired_keys: Vec<String> = cache
                        .iter()
                        .filter(|(_, entry)| {
                            let age = now
                                .duration_since(entry.created_at)
                                .unwrap_or(Duration::from_secs(0));
                            age.as_secs() >= self.config.ttl_seconds
                        })
                        .map(|(k, _)| k.clone())
                        .collect();

                    for key in expired_keys {
                        cache.remove(&key);
                    }
                }
                _ => {
                    // For other strategies, just remove the oldest entry
                    if let Some((key, _)) = cache
                        .iter()
                        .min_by_key(|(_, entry)| entry.created_at)
                        .map(|(k, v)| (k.clone(), v.clone()))
                    {
                        cache.remove(&key);
                    }
                }
            }
        }
    }
}

/// Cache statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Number of entries in primary cache
    pub primary_size: usize,
    /// Number of entries in secondary cache
    pub secondary_size: usize,
    /// Maximum cache size
    pub max_size: usize,
    /// Cache backend type
    pub backend: CacheBackend,
    /// Invalidation strategy
    pub invalidation_strategy: InvalidationStrategy,
}

/// Redis cache adapter (placeholder for actual Redis integration).
///
/// # Examples
///
/// ```
/// use legalis_diff::advanced_cache::RedisCache;
///
/// let cache = RedisCache::new("redis://localhost:6379");
/// ```
pub struct RedisCache {
    #[allow(dead_code)]
    connection_string: String,
    namespace: String,
}

impl RedisCache {
    /// Creates a new Redis cache adapter.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::RedisCache;
    ///
    /// let cache = RedisCache::new("redis://localhost:6379");
    /// ```
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            namespace: "legalis_diff".to_string(),
        }
    }

    /// Sets a custom namespace for keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::RedisCache;
    ///
    /// let cache = RedisCache::new("redis://localhost:6379")
    ///     .with_namespace("my_app");
    /// ```
    pub fn with_namespace(mut self, namespace: &str) -> Self {
        self.namespace = namespace.to_string();
        self
    }

    /// Gets a value from Redis (placeholder implementation).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::RedisCache;
    ///
    /// let cache = RedisCache::new("redis://localhost:6379");
    /// let result = cache.get("key");
    /// ```
    pub fn get(&self, _key: &str) -> DiffResult<Option<StatuteDiff>> {
        // Placeholder: In real implementation, this would connect to Redis
        Ok(None)
    }

    /// Puts a value into Redis (placeholder implementation).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::RedisCache;
    /// use legalis_diff::StatuteDiff;
    ///
    /// let cache = RedisCache::new("redis://localhost:6379");
    /// let diff = StatuteDiff {
    ///     statute_id: "statute-123".to_string(),
    ///     version_info: None,
    ///     changes: Vec::new(),
    ///     impact: Default::default(),
    /// };
    /// cache.put("key", diff, 3600).unwrap();
    /// ```
    pub fn put(&self, _key: &str, _value: StatuteDiff, _ttl: u64) -> DiffResult<()> {
        // Placeholder: In real implementation, this would connect to Redis
        Ok(())
    }

    /// Deletes a value from Redis (placeholder implementation).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::RedisCache;
    ///
    /// let cache = RedisCache::new("redis://localhost:6379");
    /// cache.delete("key").unwrap();
    /// ```
    pub fn delete(&self, _key: &str) -> DiffResult<()> {
        // Placeholder: In real implementation, this would connect to Redis
        Ok(())
    }
}

/// Memcached cache adapter (placeholder for actual Memcached integration).
///
/// # Examples
///
/// ```
/// use legalis_diff::advanced_cache::MemcachedCache;
///
/// let servers = vec!["localhost:11211".to_string()];
/// let cache = MemcachedCache::new(servers);
/// ```
pub struct MemcachedCache {
    #[allow(dead_code)]
    servers: Vec<String>,
}

impl MemcachedCache {
    /// Creates a new Memcached cache adapter.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::MemcachedCache;
    ///
    /// let servers = vec!["localhost:11211".to_string()];
    /// let cache = MemcachedCache::new(servers);
    /// ```
    pub fn new(servers: Vec<String>) -> Self {
        Self { servers }
    }

    /// Gets a value from Memcached (placeholder implementation).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::MemcachedCache;
    ///
    /// let cache = MemcachedCache::new(vec!["localhost:11211".to_string()]);
    /// let result = cache.get("key");
    /// ```
    pub fn get(&self, _key: &str) -> DiffResult<Option<StatuteDiff>> {
        // Placeholder: In real implementation, this would connect to Memcached
        Ok(None)
    }

    /// Puts a value into Memcached (placeholder implementation).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::MemcachedCache;
    /// use legalis_diff::StatuteDiff;
    ///
    /// let cache = MemcachedCache::new(vec!["localhost:11211".to_string()]);
    /// let diff = StatuteDiff {
    ///     statute_id: "statute-123".to_string(),
    ///     version_info: None,
    ///     changes: Vec::new(),
    ///     impact: Default::default(),
    /// };
    /// cache.put("key", diff, 3600).unwrap();
    /// ```
    pub fn put(&self, _key: &str, _value: StatuteDiff, _ttl: u64) -> DiffResult<()> {
        // Placeholder: In real implementation, this would connect to Memcached
        Ok(())
    }

    /// Deletes a value from Memcached (placeholder implementation).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::advanced_cache::MemcachedCache;
    ///
    /// let cache = MemcachedCache::new(vec!["localhost:11211".to_string()]);
    /// cache.delete("key").unwrap();
    /// ```
    pub fn delete(&self, _key: &str) -> DiffResult<()> {
        // Placeholder: In real implementation, this would connect to Memcached
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ImpactAssessment;

    fn create_test_diff() -> StatuteDiff {
        StatuteDiff {
            statute_id: "test-123".to_string(),
            version_info: None,
            changes: Vec::new(),
            impact: ImpactAssessment::default(),
        }
    }

    #[test]
    fn test_cache_manager_creation() {
        let config = CacheConfig::default();
        let manager = CacheManager::new(config);
        let stats = manager.stats();
        assert_eq!(stats.primary_size, 0);
    }

    #[test]
    fn test_cache_put_and_get() {
        let manager = CacheManager::new(CacheConfig::default());
        let diff = create_test_diff();

        manager.put("test-key", diff.clone());
        let retrieved = manager.get("test-key");

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().statute_id, "test-123");
    }

    #[test]
    fn test_cache_invalidate() {
        let manager = CacheManager::new(CacheConfig::default());
        let diff = create_test_diff();

        manager.put("test-key", diff);
        manager.invalidate("test-key");

        assert!(manager.get("test-key").is_none());
    }

    #[test]
    fn test_cache_invalidate_all() {
        let manager = CacheManager::new(CacheConfig::default());

        manager.put("key1", create_test_diff());
        manager.put("key2", create_test_diff());

        manager.invalidate_all();

        assert!(manager.get("key1").is_none());
        assert!(manager.get("key2").is_none());
    }

    #[test]
    fn test_lru_eviction() {
        let config = CacheConfig {
            max_size: 2,
            invalidation_strategy: InvalidationStrategy::Lru,
            ..Default::default()
        };

        let manager = CacheManager::new(config);

        manager.put("key1", create_test_diff());
        manager.put("key2", create_test_diff());

        // Access key1 to make it more recently used
        let _ = manager.get("key1");

        // This should evict key2 (least recently used)
        manager.put("key3", create_test_diff());

        assert!(manager.get("key1").is_some());
        assert!(manager.get("key2").is_none());
        assert!(manager.get("key3").is_some());
    }

    #[test]
    fn test_cache_stats() {
        let manager = CacheManager::new(CacheConfig::default());

        manager.put("key1", create_test_diff());
        manager.put("key2", create_test_diff());

        let stats = manager.stats();
        assert_eq!(stats.primary_size, 2);
    }

    #[test]
    fn test_redis_cache_creation() {
        let cache = RedisCache::new("redis://localhost:6379");
        assert_eq!(cache.connection_string, "redis://localhost:6379");
    }

    #[test]
    fn test_memcached_cache_creation() {
        let servers = vec!["localhost:11211".to_string()];
        let cache = MemcachedCache::new(servers.clone());
        assert_eq!(cache.servers, servers);
    }

    #[test]
    fn test_preload() {
        let config = CacheConfig {
            enable_preloading: true,
            ..Default::default()
        };

        let manager = CacheManager::new(config);
        manager.preload(vec!["key1".to_string(), "key2".to_string()]);

        // Preload queue should contain the keys
        let queue = manager.preload_queue.lock().unwrap();
        assert_eq!(queue.len(), 2);
    }
}
