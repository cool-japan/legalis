//! Export caching for improved performance.
//!
//! This module provides caching mechanisms for RDF exports to avoid
//! redundant serialization of the same statutes.

use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

/// A cache entry with expiration time.
#[derive(Debug, Clone)]
struct CacheEntry<V> {
    value: V,
    created_at: Instant,
}

impl<V> CacheEntry<V> {
    fn new(value: V) -> Self {
        Self {
            value,
            created_at: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }
}

/// A simple LRU cache with TTL (Time To Live) support.
#[derive(Debug)]
pub struct ExportCache<K, V> {
    cache: HashMap<K, CacheEntry<V>>,
    max_size: usize,
    ttl: Duration,
    access_order: Vec<K>,
}

impl<K: Clone + Eq + Hash, V: Clone> ExportCache<K, V> {
    /// Creates a new export cache.
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            ttl,
            access_order: Vec::new(),
        }
    }

    /// Gets a value from the cache.
    pub fn get(&mut self, key: &K) -> Option<V> {
        // Check if entry exists and is not expired
        if let Some(entry) = self.cache.get(key) {
            if entry.is_expired(self.ttl) {
                self.cache.remove(key);
                self.access_order.retain(|k| k != key);
                return None;
            }

            // Update access order (move to end = most recently used)
            self.access_order.retain(|k| k != key);
            self.access_order.push(key.clone());

            return Some(entry.value.clone());
        }

        None
    }

    /// Inserts a value into the cache.
    pub fn insert(&mut self, key: K, value: V) {
        // Remove if exists
        if self.cache.contains_key(&key) {
            self.access_order.retain(|k| k != &key);
        }

        // Evict least recently used if at capacity
        if self.cache.len() >= self.max_size && !self.cache.contains_key(&key) {
            if let Some(lru_key) = self.access_order.first().cloned() {
                self.cache.remove(&lru_key);
                self.access_order.remove(0);
            }
        }

        // Insert new entry
        self.cache.insert(key.clone(), CacheEntry::new(value));
        self.access_order.push(key);
    }

    /// Clears the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Removes expired entries from the cache.
    pub fn cleanup_expired(&mut self) {
        let expired_keys: Vec<K> = self
            .cache
            .iter()
            .filter(|(_, entry)| entry.is_expired(self.ttl))
            .map(|(k, _)| k.clone())
            .collect();

        for key in expired_keys {
            self.cache.remove(&key);
            self.access_order.retain(|k| k != &key);
        }
    }

    /// Returns cache statistics.
    pub fn stats(&self) -> CacheStats {
        let mut expired_count = 0;
        for entry in self.cache.values() {
            if entry.is_expired(self.ttl) {
                expired_count += 1;
            }
        }

        CacheStats {
            total_entries: self.cache.len(),
            expired_entries: expired_count,
            max_size: self.max_size,
            ttl_seconds: self.ttl.as_secs(),
        }
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total number of entries in cache
    pub total_entries: usize,
    /// Number of expired entries
    pub expired_entries: usize,
    /// Maximum cache size
    pub max_size: usize,
    /// TTL in seconds
    pub ttl_seconds: u64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            total_entries: 0,
            expired_entries: 0,
            max_size: 0,
            ttl_seconds: 0,
        }
    }
}

/// Generates a cache key for statute exports.
pub fn statute_cache_key(statute_id: &str, format: &str) -> String {
    format!("{}:{}", statute_id, format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_insert_and_get() {
        let mut cache: ExportCache<String, String> = ExportCache::new(10, Duration::from_secs(60));

        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache: ExportCache<String, String> = ExportCache::new(3, Duration::from_secs(60));

        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());
        cache.insert("key3".to_string(), "value3".to_string());

        // Cache should be at capacity
        assert_eq!(cache.len(), 3);

        // Insert another key, should evict least recently used (key1)
        cache.insert("key4".to_string(), "value4".to_string());
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&"key1".to_string()), None);
        assert_eq!(cache.get(&"key2".to_string()), Some("value2".to_string()));
    }

    #[test]
    fn test_cache_lru_order() {
        let mut cache: ExportCache<String, String> = ExportCache::new(3, Duration::from_secs(60));

        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());
        cache.insert("key3".to_string(), "value3".to_string());

        // Access key1, making it more recently used
        let _ = cache.get(&"key1".to_string());

        // Insert key4, should evict key2 (now least recently used)
        cache.insert("key4".to_string(), "value4".to_string());
        assert_eq!(cache.get(&"key2".to_string()), None);
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
    }

    #[test]
    fn test_cache_expiration() {
        let mut cache: ExportCache<String, String> =
            ExportCache::new(10, Duration::from_millis(100));

        cache.insert("key1".to_string(), "value1".to_string());

        // Value should be available immediately
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(150));

        // Value should be expired
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache: ExportCache<String, String> = ExportCache::new(10, Duration::from_secs(60));

        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());

        assert_eq!(cache.len(), 2);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_cleanup_expired() {
        let mut cache: ExportCache<String, String> =
            ExportCache::new(10, Duration::from_millis(100));

        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(150));

        // Insert a new key (not expired)
        cache.insert("key3".to_string(), "value3".to_string());

        assert_eq!(cache.len(), 3);

        // Cleanup expired
        cache.cleanup_expired();

        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&"key3".to_string()), Some("value3".to_string()));
    }

    #[test]
    fn test_cache_stats() {
        let mut cache: ExportCache<String, String> = ExportCache::new(10, Duration::from_secs(60));

        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());

        let stats = cache.stats();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.max_size, 10);
        assert_eq!(stats.ttl_seconds, 60);
    }

    #[test]
    fn test_statute_cache_key() {
        let key1 = statute_cache_key("statute-1", "turtle");
        let key2 = statute_cache_key("statute-1", "json-ld");
        let key3 = statute_cache_key("statute-2", "turtle");

        assert_eq!(key1, "statute-1:turtle");
        assert_ne!(key1, key2);
        assert_ne!(key1, key3);
    }
}
