//! Parse result caching for the Legalis DSL.
//!
//! This module provides caching mechanisms to avoid re-parsing
//! documents that haven't changed.

use crate::{DslResult, LegalDslParser, ast};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

/// A hash-based cache key derived from the input text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CacheKey(u64);

impl CacheKey {
    /// Creates a cache key from input text.
    pub fn from_text(text: &str) -> Self {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        Self(hasher.finish())
    }
}

/// A cached parse result with metadata.
#[derive(Debug, Clone)]
struct CachedEntry<T> {
    /// The cached value
    value: T,
    /// When this entry was created
    created_at: Instant,
    /// How many times this entry has been accessed
    access_count: usize,
}

/// A cache for parse results with LRU (Least Recently Used) eviction.
#[derive(Debug)]
pub struct ParseCache<T> {
    /// The cache storage
    cache: HashMap<CacheKey, CachedEntry<T>>,
    /// Maximum number of entries to keep
    max_size: usize,
    /// Maximum age of entries (optional)
    max_age: Option<Duration>,
    /// Cache statistics
    hits: usize,
    misses: usize,
}

impl<T: Clone> ParseCache<T> {
    /// Creates a new cache with the specified maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            max_age: None,
            hits: 0,
            misses: 0,
        }
    }

    /// Creates a new cache with a maximum age for entries.
    pub fn with_max_age(max_size: usize, max_age: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            max_age: Some(max_age),
            hits: 0,
            misses: 0,
        }
    }

    /// Gets a value from the cache if it exists and hasn't expired.
    pub fn get(&mut self, key: &CacheKey) -> Option<T> {
        if let Some(entry) = self.cache.get_mut(key) {
            // Check if entry has expired
            if let Some(max_age) = self.max_age {
                if entry.created_at.elapsed() > max_age {
                    self.cache.remove(key);
                    self.misses += 1;
                    return None;
                }
            }

            // Update access count
            entry.access_count += 1;
            self.hits += 1;
            Some(entry.value.clone())
        } else {
            self.misses += 1;
            None
        }
    }

    /// Inserts a value into the cache.
    pub fn insert(&mut self, key: CacheKey, value: T) {
        // Evict old entries if cache is full
        if self.cache.len() >= self.max_size {
            self.evict_lru();
        }

        self.cache.insert(
            key,
            CachedEntry {
                value,
                created_at: Instant::now(),
                access_count: 0,
            },
        );
    }

    /// Evicts the least recently used entry.
    fn evict_lru(&mut self) {
        if let Some((&key_to_remove, _)) = self
            .cache
            .iter()
            .min_by_key(|(_, entry)| entry.access_count)
        {
            self.cache.remove(&key_to_remove);
        }
    }

    /// Clears all entries from the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Returns the cache hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Returns cache statistics.
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            size: self.cache.len(),
            max_size: self.max_size,
            hits: self.hits,
            misses: self.misses,
            hit_rate: self.hit_rate(),
        }
    }
}

/// Cache statistics.
#[derive(Debug, Clone, PartialEq)]
pub struct CacheStats {
    /// Current number of entries
    pub size: usize,
    /// Maximum number of entries
    pub max_size: usize,
    /// Number of cache hits
    pub hits: usize,
    /// Number of cache misses
    pub misses: usize,
    /// Hit rate (0.0 to 1.0)
    pub hit_rate: f64,
}

/// A caching parser that wraps LegalDslParser with automatic caching.
#[derive(Debug)]
pub struct CachingParser {
    /// The underlying parser
    parser: LegalDslParser,
    /// Cache for parsed documents
    cache: ParseCache<ast::LegalDocument>,
}

impl CachingParser {
    /// Creates a new caching parser with default cache size (100 entries).
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Creates a new caching parser with specified cache capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parser: LegalDslParser::new(),
            cache: ParseCache::new(capacity),
        }
    }

    /// Creates a new caching parser with a maximum age for cached entries.
    pub fn with_max_age(capacity: usize, max_age: Duration) -> Self {
        Self {
            parser: LegalDslParser::new(),
            cache: ParseCache::with_max_age(capacity, max_age),
        }
    }

    /// Parses a document, using the cache if available.
    pub fn parse_document(&mut self, text: &str) -> DslResult<ast::LegalDocument> {
        let key = CacheKey::from_text(text);

        if let Some(cached) = self.cache.get(&key) {
            return Ok(cached);
        }

        let doc = self.parser.parse_document(text)?;
        self.cache.insert(key, doc.clone());
        Ok(doc)
    }

    /// Returns cache statistics.
    pub fn cache_stats(&self) -> CacheStats {
        self.cache.stats()
    }

    /// Clears the cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Returns the parser's warnings.
    pub fn warnings(&self) -> Vec<crate::DslWarning> {
        self.parser.warnings()
    }
}

impl Default for CachingParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_from_text() {
        let key1 = CacheKey::from_text("test");
        let key2 = CacheKey::from_text("test");
        let key3 = CacheKey::from_text("different");

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_parse_cache_basic() {
        let mut cache = ParseCache::<String>::new(10);

        let key = CacheKey::from_text("test");
        cache.insert(key, "value".to_string());

        assert_eq!(cache.get(&key), Some("value".to_string()));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_parse_cache_miss() {
        let mut cache = ParseCache::<String>::new(10);
        let key = CacheKey::from_text("nonexistent");

        assert_eq!(cache.get(&key), None);
    }

    #[test]
    fn test_parse_cache_eviction() {
        let mut cache = ParseCache::<String>::new(2);

        let key1 = CacheKey::from_text("one");
        let key2 = CacheKey::from_text("two");
        let key3 = CacheKey::from_text("three");

        cache.insert(key1, "value1".to_string());
        cache.insert(key2, "value2".to_string());

        // Access key1 to increase its access count
        cache.get(&key1);

        // Insert key3, which should evict key2 (least recently used)
        cache.insert(key3, "value3".to_string());

        assert_eq!(cache.len(), 2);
        assert!(cache.get(&key1).is_some());
        assert!(cache.get(&key3).is_some());
    }

    #[test]
    fn test_parse_cache_clear() {
        let mut cache = ParseCache::<String>::new(10);
        let key = CacheKey::from_text("test");
        cache.insert(key, "value".to_string());

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.get(&key), None);
    }

    #[test]
    fn test_cache_hit_rate() {
        let mut cache = ParseCache::<String>::new(10);
        let key = CacheKey::from_text("test");
        cache.insert(key, "value".to_string());

        // 2 hits
        cache.get(&key);
        cache.get(&key);

        // 1 miss
        cache.get(&CacheKey::from_text("other"));

        let stats = cache.stats();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert!((stats.hit_rate - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_caching_parser() {
        let input = r#"
            STATUTE test: "Test" {
                WHEN AGE >= 18
                THEN GRANT "Access"
            }
        "#;

        let mut parser = CachingParser::new();

        // First parse - cache miss
        let doc1 = parser.parse_document(input).unwrap();
        assert_eq!(doc1.statutes.len(), 1);

        // Second parse - cache hit
        let doc2 = parser.parse_document(input).unwrap();
        assert_eq!(doc2.statutes.len(), 1);

        let stats = parser.cache_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[test]
    fn test_caching_parser_different_inputs() {
        let input1 = r#"STATUTE test1: "Test1" { WHEN AGE >= 18 THEN GRANT "Access" }"#;
        let input2 = r#"STATUTE test2: "Test2" { WHEN AGE >= 21 THEN GRANT "Access" }"#;

        let mut parser = CachingParser::new();

        parser.parse_document(input1).unwrap();
        parser.parse_document(input2).unwrap();

        let stats = parser.cache_stats();
        assert_eq!(stats.size, 2);
        assert_eq!(stats.misses, 2);
        assert_eq!(stats.hits, 0);
    }

    #[test]
    fn test_caching_parser_clear() {
        let input = r#"STATUTE test: "Test" { WHEN AGE >= 18 THEN GRANT "Access" }"#;

        let mut parser = CachingParser::new();
        parser.parse_document(input).unwrap();

        parser.clear_cache();

        let stats = parser.cache_stats();
        assert_eq!(stats.size, 0);
    }
}
