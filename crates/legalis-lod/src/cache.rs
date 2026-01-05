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
#[derive(Debug, Clone, Default)]
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

/// Generates a cache key for statute exports.
pub fn statute_cache_key(statute_id: &str, format: &str) -> String {
    format!("{}:{}", statute_id, format)
}

/// Query result cache for SPARQL-like queries.
#[derive(Debug)]
pub struct QueryCache<V> {
    cache: ExportCache<String, V>,
}

impl<V: Clone> QueryCache<V> {
    /// Creates a new query cache.
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            cache: ExportCache::new(max_size, ttl),
        }
    }

    /// Gets cached query results.
    pub fn get_query_result(&mut self, query: &str) -> Option<V> {
        self.cache.get(&Self::hash_query(query))
    }

    /// Caches query results.
    pub fn cache_query_result(&mut self, query: &str, result: V) {
        self.cache.insert(Self::hash_query(query), result);
    }

    /// Invalidates cache for a specific query.
    pub fn invalidate(&mut self, query: &str) {
        let key = Self::hash_query(query);
        self.cache.cache.remove(&key);
        self.cache.access_order.retain(|k| k != &key);
    }

    /// Invalidates all cached queries.
    pub fn invalidate_all(&mut self) {
        self.cache.clear();
    }

    /// Cleans up expired entries.
    pub fn cleanup(&mut self) {
        self.cache.cleanup_expired();
    }

    /// Returns cache statistics.
    pub fn stats(&self) -> CacheStats {
        self.cache.stats()
    }

    /// Creates a hash for a query string (simple implementation).
    fn hash_query(query: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let mut hasher = DefaultHasher::new();
        hasher.write(query.as_bytes());
        format!("query:{:x}", hasher.finish())
    }
}

/// Incremental update tracker for RDF graphs.
#[derive(Debug, Clone)]
pub struct IncrementalUpdateTracker {
    /// Last update timestamp
    last_update: Instant,
    /// Modified subjects
    modified_subjects: Vec<String>,
    /// Deleted subjects
    deleted_subjects: Vec<String>,
    /// Added subjects
    added_subjects: Vec<String>,
}

impl Default for IncrementalUpdateTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalUpdateTracker {
    /// Creates a new incremental update tracker.
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            modified_subjects: Vec::new(),
            deleted_subjects: Vec::new(),
            added_subjects: Vec::new(),
        }
    }

    /// Records a modification to a subject.
    pub fn record_modification(&mut self, subject: impl Into<String>) {
        let subject = subject.into();
        if !self.modified_subjects.contains(&subject) {
            self.modified_subjects.push(subject);
        }
        self.last_update = Instant::now();
    }

    /// Records deletion of a subject.
    pub fn record_deletion(&mut self, subject: impl Into<String>) {
        let subject = subject.into();
        if !self.deleted_subjects.contains(&subject) {
            self.deleted_subjects.push(subject);
        }
        self.last_update = Instant::now();
    }

    /// Records addition of a subject.
    pub fn record_addition(&mut self, subject: impl Into<String>) {
        let subject = subject.into();
        if !self.added_subjects.contains(&subject) {
            self.added_subjects.push(subject);
        }
        self.last_update = Instant::now();
    }

    /// Returns all modified subjects.
    pub fn get_modifications(&self) -> &[String] {
        &self.modified_subjects
    }

    /// Returns all deleted subjects.
    pub fn get_deletions(&self) -> &[String] {
        &self.deleted_subjects
    }

    /// Returns all added subjects.
    pub fn get_additions(&self) -> &[String] {
        &self.added_subjects
    }

    /// Checks if there are any pending updates.
    pub fn has_updates(&self) -> bool {
        !self.modified_subjects.is_empty()
            || !self.deleted_subjects.is_empty()
            || !self.added_subjects.is_empty()
    }

    /// Returns the time since last update.
    pub fn time_since_update(&self) -> Duration {
        self.last_update.elapsed()
    }

    /// Clears all tracked updates.
    pub fn clear(&mut self) {
        self.modified_subjects.clear();
        self.deleted_subjects.clear();
        self.added_subjects.clear();
        self.last_update = Instant::now();
    }

    /// Returns update statistics.
    pub fn stats(&self) -> UpdateStats {
        UpdateStats {
            modified_count: self.modified_subjects.len(),
            deleted_count: self.deleted_subjects.len(),
            added_count: self.added_subjects.len(),
            last_update_seconds: self.last_update.elapsed().as_secs(),
        }
    }
}

/// Update statistics.
#[derive(Debug, Clone, Default)]
pub struct UpdateStats {
    /// Number of modified subjects
    pub modified_count: usize,
    /// Number of deleted subjects
    pub deleted_count: usize,
    /// Number of added subjects
    pub added_count: usize,
    /// Seconds since last update
    pub last_update_seconds: u64,
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

    #[test]
    fn test_query_cache() {
        let mut cache: QueryCache<Vec<String>> = QueryCache::new(10, Duration::from_secs(60));

        let query = "SELECT * WHERE { ?s ?p ?o }";
        let results = vec!["result1".to_string(), "result2".to_string()];

        cache.cache_query_result(query, results.clone());
        assert_eq!(cache.get_query_result(query), Some(results));
    }

    #[test]
    fn test_query_cache_invalidate() {
        let mut cache: QueryCache<Vec<String>> = QueryCache::new(10, Duration::from_secs(60));

        let query = "SELECT * WHERE { ?s ?p ?o }";
        let results = vec!["result1".to_string()];

        cache.cache_query_result(query, results);
        assert!(cache.get_query_result(query).is_some());

        cache.invalidate(query);
        assert!(cache.get_query_result(query).is_none());
    }

    #[test]
    fn test_query_cache_invalidate_all() {
        let mut cache: QueryCache<Vec<String>> = QueryCache::new(10, Duration::from_secs(60));

        cache.cache_query_result("query1", vec!["result1".to_string()]);
        cache.cache_query_result("query2", vec!["result2".to_string()]);

        cache.invalidate_all();
        assert!(cache.get_query_result("query1").is_none());
        assert!(cache.get_query_result("query2").is_none());
    }

    #[test]
    fn test_incremental_update_tracker() {
        let mut tracker = IncrementalUpdateTracker::new();

        tracker.record_addition("subject1");
        tracker.record_modification("subject2");
        tracker.record_deletion("subject3");

        assert!(tracker.has_updates());
        assert_eq!(tracker.get_additions().len(), 1);
        assert_eq!(tracker.get_modifications().len(), 1);
        assert_eq!(tracker.get_deletions().len(), 1);
    }

    #[test]
    fn test_incremental_update_tracker_stats() {
        let mut tracker = IncrementalUpdateTracker::new();

        tracker.record_addition("subject1");
        tracker.record_modification("subject2");

        let stats = tracker.stats();
        assert_eq!(stats.added_count, 1);
        assert_eq!(stats.modified_count, 1);
        assert_eq!(stats.deleted_count, 0);
    }

    #[test]
    fn test_incremental_update_tracker_clear() {
        let mut tracker = IncrementalUpdateTracker::new();

        tracker.record_addition("subject1");
        tracker.record_modification("subject2");

        assert!(tracker.has_updates());

        tracker.clear();
        assert!(!tracker.has_updates());
        assert_eq!(tracker.get_additions().len(), 0);
    }

    #[test]
    fn test_incremental_update_tracker_no_duplicates() {
        let mut tracker = IncrementalUpdateTracker::new();

        tracker.record_addition("subject1");
        tracker.record_addition("subject1");

        assert_eq!(tracker.get_additions().len(), 1);
    }
}
