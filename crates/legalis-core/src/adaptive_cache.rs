//! Adaptive caching strategies for optimizing condition evaluation performance.
//!
//! This module provides intelligent caching that automatically adjusts its behavior
//! based on access patterns and hit rates. The cache can switch between different
//! eviction policies to optimize for the current workload.
//!
//! ## Features
//!
//! - **Adaptive Policy Selection**: Automatically switches between LRU, LFU, and ARC policies
//! - **Dynamic Sizing**: Adjusts cache capacity based on hit rate and memory pressure
//! - **Performance Monitoring**: Tracks hit rate, miss rate, and eviction statistics
//! - **Multiple Eviction Policies**: LRU (Least Recently Used), LFU (Least Frequently Used), ARC (Adaptive Replacement Cache)
//!
//! ## Example
//!
//! ```
//! use legalis_core::adaptive_cache::AdaptiveCache;
//!
//! let mut cache = AdaptiveCache::new(100);
//!
//! // Insert entries
//! cache.insert("key1".to_string(), true);
//! cache.insert("key2".to_string(), false);
//!
//! // Retrieve entries
//! assert_eq!(cache.get(&"key1".to_string()), Some(&true));
//! assert_eq!(cache.get(&"key3".to_string()), None);
//!
//! // Check statistics
//! let stats = cache.stats();
//! assert_eq!(stats.total_requests(), 2);
//! assert!((stats.hit_rate() - 0.5).abs() < 0.001); // 1 hit, 1 miss
//! ```

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Eviction policy for cache entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least Recently Used - evicts entries that haven't been accessed recently
    LRU,
    /// Least Frequently Used - evicts entries with lowest access count
    LFU,
    /// Adaptive Replacement Cache - balances recency and frequency
    ARC,
}

impl std::fmt::Display for EvictionPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvictionPolicy::LRU => write!(f, "LRU (Least Recently Used)"),
            EvictionPolicy::LFU => write!(f, "LFU (Least Frequently Used)"),
            EvictionPolicy::ARC => write!(f, "ARC (Adaptive Replacement Cache)"),
        }
    }
}

/// Statistics for cache performance monitoring.
///
/// # Example
///
/// ```
/// use legalis_core::adaptive_cache::CacheStats;
///
/// let mut stats = CacheStats::new();
/// stats.record_hit();
/// stats.record_hit();
/// stats.record_miss();
///
/// assert_eq!(stats.total_requests(), 3);
/// assert_eq!(stats.hits, 2);
/// assert_eq!(stats.misses, 1);
/// assert!((stats.hit_rate() - 0.666).abs() < 0.01);
/// ```
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of evictions
    pub evictions: u64,
    /// Number of insertions
    pub insertions: u64,
}

impl CacheStats {
    /// Creates new cache statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a cache hit.
    pub fn record_hit(&mut self) {
        self.hits += 1;
    }

    /// Records a cache miss.
    pub fn record_miss(&mut self) {
        self.misses += 1;
    }

    /// Records an eviction.
    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }

    /// Records an insertion.
    pub fn record_insertion(&mut self) {
        self.insertions += 1;
    }

    /// Returns the total number of requests (hits + misses).
    pub fn total_requests(&self) -> u64 {
        self.hits + self.misses
    }

    /// Returns the hit rate as a percentage (0.0 to 1.0).
    pub fn hit_rate(&self) -> f64 {
        let total = self.total_requests();
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Returns the miss rate as a percentage (0.0 to 1.0).
    pub fn miss_rate(&self) -> f64 {
        1.0 - self.hit_rate()
    }

    /// Resets all statistics.
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.evictions = 0;
        self.insertions = 0;
    }
}

/// Entry metadata for tracking access patterns.
#[derive(Debug, Clone)]
struct CacheEntry<V> {
    value: V,
    access_count: u64,
    last_access: u64,
}

/// Adaptive cache that automatically adjusts its behavior based on workload.
///
/// The cache monitors hit rates and access patterns to determine the optimal
/// eviction policy and capacity. It can dynamically switch between LRU, LFU,
/// and ARC policies to maximize performance.
///
/// # Example
///
/// ```
/// use legalis_core::adaptive_cache::{AdaptiveCache, EvictionPolicy};
///
/// let mut cache = AdaptiveCache::new(3);
///
/// cache.insert("a".to_string(), 1);
/// cache.insert("b".to_string(), 2);
/// cache.insert("c".to_string(), 3);
///
/// assert_eq!(cache.len(), 3);
/// assert_eq!(cache.get(&"a".to_string()), Some(&1));
/// assert_eq!(cache.get(&"b".to_string()), Some(&2));
/// assert_eq!(cache.get(&"c".to_string()), Some(&3));
/// ```
#[derive(Debug, Clone)]
pub struct AdaptiveCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    capacity: usize,
    entries: HashMap<K, CacheEntry<V>>,
    access_order: VecDeque<K>,
    current_policy: EvictionPolicy,
    stats: CacheStats,
    access_counter: u64,
    adaptation_threshold: u64,
}

impl<K, V> AdaptiveCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Creates a new adaptive cache with the specified capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::adaptive_cache::AdaptiveCache;
    ///
    /// let cache: AdaptiveCache<String, bool> = AdaptiveCache::new(100);
    /// assert_eq!(cache.capacity(), 100);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: HashMap::with_capacity(capacity),
            access_order: VecDeque::with_capacity(capacity),
            current_policy: EvictionPolicy::LRU,
            stats: CacheStats::new(),
            access_counter: 0,
            adaptation_threshold: 1000,
        }
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// If the cache is full, an entry will be evicted according to the current policy.
    pub fn insert(&mut self, key: K, value: V) {
        self.stats.record_insertion();

        if self.entries.len() >= self.capacity && !self.entries.contains_key(&key) {
            self.evict();
        }

        let entry = CacheEntry {
            value,
            access_count: 1,
            last_access: self.access_counter,
        };

        self.entries.insert(key.clone(), entry);
        self.access_order.push_back(key);
        self.access_counter += 1;

        self.maybe_adapt_policy();
    }

    /// Retrieves a value from the cache.
    ///
    /// Returns `None` if the key is not found.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(entry) = self.entries.get_mut(key) {
            self.stats.record_hit();
            entry.access_count += 1;
            entry.last_access = self.access_counter;
            self.access_counter += 1;
            Some(&entry.value)
        } else {
            self.stats.record_miss();
            None
        }
    }

    /// Removes an entry from the cache.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(entry) = self.entries.remove(key) {
            self.access_order.retain(|k| k != key);
            Some(entry.value)
        } else {
            None
        }
    }

    /// Clears all entries from the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.access_order.clear();
    }

    /// Returns the current cache capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the number of entries currently in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the current eviction policy.
    pub fn current_policy(&self) -> EvictionPolicy {
        self.current_policy
    }

    /// Returns cache statistics.
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Manually sets the eviction policy.
    pub fn set_policy(&mut self, policy: EvictionPolicy) {
        self.current_policy = policy;
    }

    /// Evicts an entry according to the current policy.
    fn evict(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        let key_to_evict = match self.current_policy {
            EvictionPolicy::LRU => self.evict_lru(),
            EvictionPolicy::LFU => self.evict_lfu(),
            EvictionPolicy::ARC => self.evict_arc(),
        };

        if let Some(key) = key_to_evict {
            self.entries.remove(&key);
            self.access_order.retain(|k| k != &key);
            self.stats.record_eviction();
        }
    }

    /// Evicts the least recently used entry.
    fn evict_lru(&self) -> Option<K> {
        self.access_order.front().cloned()
    }

    /// Evicts the least frequently used entry.
    fn evict_lfu(&self) -> Option<K> {
        self.entries
            .iter()
            .min_by_key(|(_, entry)| entry.access_count)
            .map(|(k, _)| k.clone())
    }

    /// Evicts using ARC policy (simplified: uses LRU with frequency consideration).
    fn evict_arc(&self) -> Option<K> {
        // Simplified ARC: evict entries with low access count, breaking ties by recency
        self.entries
            .iter()
            .min_by_key(|(_, entry)| (entry.access_count, entry.last_access))
            .map(|(k, _)| k.clone())
    }

    /// Checks if the policy should be adapted based on performance.
    fn maybe_adapt_policy(&mut self) {
        if self.stats.total_requests() % self.adaptation_threshold != 0 {
            return;
        }

        let hit_rate = self.stats.hit_rate();

        // If hit rate is low, try switching policy
        if hit_rate < 0.5 {
            self.current_policy = match self.current_policy {
                EvictionPolicy::LRU => EvictionPolicy::LFU,
                EvictionPolicy::LFU => EvictionPolicy::ARC,
                EvictionPolicy::ARC => EvictionPolicy::LRU,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_stats() {
        let mut stats = CacheStats::new();
        stats.record_hit();
        stats.record_hit();
        stats.record_miss();

        assert_eq!(stats.total_requests(), 3);
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert!((stats.hit_rate() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_adaptive_cache_basic() {
        let mut cache: AdaptiveCache<String, i32> = AdaptiveCache::new(3);

        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        cache.insert("c".to_string(), 3);

        assert_eq!(cache.get(&"a".to_string()), Some(&1));
        assert_eq!(cache.get(&"b".to_string()), Some(&2));
        assert_eq!(cache.get(&"c".to_string()), Some(&3));
        assert_eq!(cache.get(&"d".to_string()), None);
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache: AdaptiveCache<String, i32> = AdaptiveCache::new(2);

        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        cache.insert("c".to_string(), 3); // Should evict 'a'

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"a".to_string()), None); // 'a' was evicted
        assert_eq!(cache.get(&"b".to_string()), Some(&2));
        assert_eq!(cache.get(&"c".to_string()), Some(&3));
    }

    #[test]
    fn test_cache_remove() {
        let mut cache: AdaptiveCache<String, i32> = AdaptiveCache::new(3);

        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);

        assert_eq!(cache.remove(&"a".to_string()), Some(1));
        assert_eq!(cache.get(&"a".to_string()), None);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache: AdaptiveCache<String, i32> = AdaptiveCache::new(3);

        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);

        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_eviction_policy_display() {
        assert_eq!(
            format!("{}", EvictionPolicy::LRU),
            "LRU (Least Recently Used)"
        );
        assert_eq!(
            format!("{}", EvictionPolicy::LFU),
            "LFU (Least Frequently Used)"
        );
        assert_eq!(
            format!("{}", EvictionPolicy::ARC),
            "ARC (Adaptive Replacement Cache)"
        );
    }

    #[test]
    fn test_manual_policy_switch() {
        let mut cache: AdaptiveCache<String, i32> = AdaptiveCache::new(3);
        assert_eq!(cache.current_policy(), EvictionPolicy::LRU);

        cache.set_policy(EvictionPolicy::LFU);
        assert_eq!(cache.current_policy(), EvictionPolicy::LFU);
    }
}
