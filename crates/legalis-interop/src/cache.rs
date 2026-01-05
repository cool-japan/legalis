//! Conversion caching for improved performance.
//!
//! This module provides a simple cache for conversion results to avoid
//! redundant parsing and conversion operations.

use crate::{ConversionReport, LegalFormat};
use legalis_core::Statute;
use std::collections::HashMap;

/// Cache key for conversion results.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    /// Hash of the source content
    content_hash: u64,
    /// Source format
    source_format: LegalFormat,
    /// Target format (None for imports)
    target_format: Option<LegalFormat>,
}

impl CacheKey {
    /// Creates a new cache key for conversion.
    fn new(content: &str, source: LegalFormat, target: Option<LegalFormat>) -> Self {
        Self {
            content_hash: Self::hash_content(content),
            source_format: source,
            target_format: target,
        }
    }

    /// Simple hash function for content.
    fn hash_content(content: &str) -> u64 {
        // Use a simple FNV-1a hash
        const FNV_PRIME: u64 = 1099511628211;
        const FNV_OFFSET: u64 = 14695981039346656037;

        content.bytes().fold(FNV_OFFSET, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME)
        })
    }
}

/// Cached conversion result.
#[derive(Debug, Clone)]
enum CachedResult {
    /// Cached import result
    Import(Vec<Statute>, ConversionReport),
    /// Cached export/conversion result
    Export(String, ConversionReport),
}

/// Conversion cache for improving performance.
pub struct ConversionCache {
    /// Maximum number of cached entries
    max_entries: usize,
    /// Cache storage
    cache: HashMap<CacheKey, CachedResult>,
    /// Access counter for LRU eviction
    access_count: u64,
    /// Access times for each key (for LRU)
    access_times: HashMap<CacheKey, u64>,
}

impl ConversionCache {
    /// Creates a new conversion cache with default capacity (100 entries).
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Creates a new conversion cache with specified capacity.
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            max_entries,
            cache: HashMap::with_capacity(max_entries),
            access_count: 0,
            access_times: HashMap::with_capacity(max_entries),
        }
    }

    /// Gets cached import result if available.
    pub fn get_import(
        &mut self,
        source: &str,
        format: LegalFormat,
    ) -> Option<(Vec<Statute>, ConversionReport)> {
        let key = CacheKey::new(source, format, None);
        self.access_count += 1;

        if let Some(CachedResult::Import(statutes, report)) = self.cache.get(&key) {
            self.access_times.insert(key, self.access_count);
            Some((statutes.clone(), report.clone()))
        } else {
            None
        }
    }

    /// Caches an import result.
    pub fn put_import(
        &mut self,
        source: &str,
        format: LegalFormat,
        statutes: Vec<Statute>,
        report: ConversionReport,
    ) {
        self.evict_if_needed();

        let key = CacheKey::new(source, format, None);
        self.access_count += 1;
        self.cache
            .insert(key.clone(), CachedResult::Import(statutes, report));
        self.access_times.insert(key, self.access_count);
    }

    /// Gets cached export result if available.
    pub fn get_export(
        &mut self,
        source: &str,
        source_format: LegalFormat,
        target_format: LegalFormat,
    ) -> Option<(String, ConversionReport)> {
        let key = CacheKey::new(source, source_format, Some(target_format));
        self.access_count += 1;

        if let Some(CachedResult::Export(output, report)) = self.cache.get(&key) {
            self.access_times.insert(key, self.access_count);
            Some((output.clone(), report.clone()))
        } else {
            None
        }
    }

    /// Caches an export/conversion result.
    pub fn put_export(
        &mut self,
        source: &str,
        source_format: LegalFormat,
        target_format: LegalFormat,
        output: String,
        report: ConversionReport,
    ) {
        self.evict_if_needed();

        let key = CacheKey::new(source, source_format, Some(target_format));
        self.access_count += 1;
        self.cache
            .insert(key.clone(), CachedResult::Export(output, report));
        self.access_times.insert(key, self.access_count);
    }

    /// Clears all cached entries.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_times.clear();
        self.access_count = 0;
    }

    /// Returns the number of cached entries.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Evicts least recently used entry if cache is full.
    fn evict_if_needed(&mut self) {
        if self.cache.len() >= self.max_entries {
            // Find LRU entry
            if let Some((lru_key, _)) = self
                .access_times
                .iter()
                .min_by_key(|&(_, &access_time)| access_time)
            {
                let lru_key = lru_key.clone();
                self.cache.remove(&lru_key);
                self.access_times.remove(&lru_key);
            }
        }
    }

    /// Returns cache statistics.
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.cache.len(),
            max_entries: self.max_entries,
            access_count: self.access_count,
        }
    }
}

impl Default for ConversionCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Current number of entries
    pub entries: usize,
    /// Maximum number of entries
    pub max_entries: usize,
    /// Total number of accesses
    pub access_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_cache_key_hash() {
        let key1 = CacheKey::new("test content", LegalFormat::Catala, None);
        let key2 = CacheKey::new("test content", LegalFormat::Catala, None);
        let key3 = CacheKey::new("different content", LegalFormat::Catala, None);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_cache_import() {
        let mut cache = ConversionCache::new();

        let source = "declaration scope Test:";
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let statutes = vec![statute];
        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);

        // First access - cache miss
        assert!(cache.get_import(source, LegalFormat::Catala).is_none());

        // Store in cache
        cache.put_import(
            source,
            LegalFormat::Catala,
            statutes.clone(),
            report.clone(),
        );

        // Second access - cache hit
        let cached = cache.get_import(source, LegalFormat::Catala);
        assert!(cached.is_some());
        let (cached_statutes, _) = cached.unwrap();
        assert_eq!(cached_statutes.len(), 1);
        assert_eq!(cached_statutes[0].id, "test");
    }

    #[test]
    fn test_cache_export() {
        let mut cache = ConversionCache::new();

        let source = "RULE Test WHEN true THEN Party MAY act";
        let output = "converted output".to_string();
        let report = ConversionReport::new(LegalFormat::L4, LegalFormat::Catala);

        // First access - cache miss
        assert!(
            cache
                .get_export(source, LegalFormat::L4, LegalFormat::Catala)
                .is_none()
        );

        // Store in cache
        cache.put_export(
            source,
            LegalFormat::L4,
            LegalFormat::Catala,
            output.clone(),
            report,
        );

        // Second access - cache hit
        let cached = cache.get_export(source, LegalFormat::L4, LegalFormat::Catala);
        assert!(cached.is_some());
        let (cached_output, _) = cached.unwrap();
        assert_eq!(cached_output, output);
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache = ConversionCache::with_capacity(2);

        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);

        // Add 3 entries (should evict oldest)
        cache.put_import("source1", LegalFormat::Catala, vec![], report.clone());
        cache.put_import("source2", LegalFormat::Catala, vec![], report.clone());
        cache.put_import("source3", LegalFormat::Catala, vec![], report);

        // Cache should have 2 entries
        assert_eq!(cache.len(), 2);

        // First entry should be evicted
        assert!(cache.get_import("source1", LegalFormat::Catala).is_none());

        // Second and third entries should still be present
        assert!(cache.get_import("source2", LegalFormat::Catala).is_some());
        assert!(cache.get_import("source3", LegalFormat::Catala).is_some());
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = ConversionCache::new();

        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);
        cache.put_import("source", LegalFormat::Catala, vec![], report);

        assert_eq!(cache.len(), 1);

        cache.clear();

        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = ConversionCache::with_capacity(10);

        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);
        cache.put_import("source", LegalFormat::Catala, vec![], report);
        cache.get_import("source", LegalFormat::Catala);

        let stats = cache.stats();
        assert_eq!(stats.entries, 1);
        assert_eq!(stats.max_entries, 10);
        assert_eq!(stats.access_count, 2); // 1 put + 1 get
    }
}
