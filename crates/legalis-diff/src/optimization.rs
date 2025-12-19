//! Performance optimizations for diff operations.
//!
//! This module provides caching, memoization, and incremental diff support
//! to improve performance when working with multiple diffs.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::optimization::{DiffCache, IncrementalDiffer};
//!
//! // Using cache for repeated diffs
//! let mut cache = DiffCache::new(100);
//! let old = Statute::new("law", "V1", Effect::new(EffectType::Grant, "Benefit"));
//! let new = Statute::new("law", "V2", Effect::new(EffectType::Grant, "Benefit"));
//!
//! let result = cache.get_or_compute(&old, &new, legalis_diff::diff).unwrap();
//! assert_eq!(cache.stats().misses, 1);
//!
//! // Using incremental differ for version tracking
//! let mut differ = IncrementalDiffer::new();
//! let _ = differ.add_version(old);
//! let diff = differ.add_version(new).unwrap();
//! assert!(diff.is_some());
//! ```

use crate::{DiffResult, StatuteDiff};
use legalis_core::Statute;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// A cache key for statute diffs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DiffCacheKey {
    old_hash: u64,
    new_hash: u64,
}

impl DiffCacheKey {
    fn new(old: &Statute, new: &Statute) -> Self {
        Self {
            old_hash: hash_statute(old),
            new_hash: hash_statute(new),
        }
    }
}

/// Computes a hash for a statute.
fn hash_statute(statute: &Statute) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    statute.id.hash(&mut hasher);
    statute.title.hash(&mut hasher);
    hasher.finish()
}

/// A cache for diff results.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::optimization::DiffCache;
///
/// let mut cache = DiffCache::new(10);
/// let old = Statute::new("test", "Old", Effect::new(EffectType::Grant, "Test"));
/// let new = Statute::new("test", "New", Effect::new(EffectType::Grant, "Test"));
///
/// let result = cache.get_or_compute(&old, &new, legalis_diff::diff).unwrap();
/// assert_eq!(result.statute_id, "test");
///
/// let stats = cache.stats();
/// assert_eq!(stats.misses, 1);
/// ```
pub struct DiffCache {
    cache: HashMap<DiffCacheKey, StatuteDiff>,
    max_size: usize,
    hits: usize,
    misses: usize,
}

impl DiffCache {
    /// Creates a new diff cache with the specified maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }

    /// Gets a cached diff result or computes it.
    pub fn get_or_compute<F>(
        &mut self,
        old: &Statute,
        new: &Statute,
        compute: F,
    ) -> DiffResult<StatuteDiff>
    where
        F: FnOnce(&Statute, &Statute) -> DiffResult<StatuteDiff>,
    {
        let key = DiffCacheKey::new(old, new);

        if let Some(cached) = self.cache.get(&key) {
            self.hits += 1;
            return Ok(cached.clone());
        }

        self.misses += 1;
        let result = compute(old, new)?;

        // Evict oldest entry if cache is full
        if self.cache.len() >= self.max_size {
            if let Some(first_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&first_key);
            }
        }

        self.cache.insert(key, result.clone());
        Ok(result)
    }

    /// Returns cache statistics.
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            size: self.cache.len(),
            max_size: self.max_size,
            hits: self.hits,
            misses: self.misses,
            hit_rate: if self.hits + self.misses == 0 {
                0.0
            } else {
                self.hits as f64 / (self.hits + self.misses) as f64
            },
        }
    }

    /// Clears the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Current cache size.
    pub size: usize,
    /// Maximum cache size.
    pub max_size: usize,
    /// Number of cache hits.
    pub hits: usize,
    /// Number of cache misses.
    pub misses: usize,
    /// Hit rate (0.0 to 1.0).
    pub hit_rate: f64,
}

/// Incremental diff support.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::optimization::IncrementalDiffer;
///
/// let mut differ = IncrementalDiffer::new();
///
/// let v1 = Statute::new("law", "V1", Effect::new(EffectType::Grant, "Benefit"));
/// let result1 = differ.add_version(v1).unwrap();
/// assert!(result1.is_none()); // First version has no diff
///
/// let v2 = Statute::new("law", "V2", Effect::new(EffectType::Grant, "Benefit"));
/// let result2 = differ.add_version(v2).unwrap();
/// assert!(result2.is_some()); // Second version has a diff
/// ```
pub struct IncrementalDiffer {
    /// Previous statute state.
    previous: Option<Statute>,
    /// Accumulated diffs.
    diffs: Vec<StatuteDiff>,
}

impl IncrementalDiffer {
    /// Creates a new incremental differ.
    pub fn new() -> Self {
        Self {
            previous: None,
            diffs: Vec::new(),
        }
    }

    /// Adds a new statute version and computes the incremental diff.
    pub fn add_version(&mut self, statute: Statute) -> DiffResult<Option<StatuteDiff>> {
        if let Some(prev) = &self.previous {
            let diff = crate::diff(prev, &statute)?;
            self.diffs.push(diff.clone());
            self.previous = Some(statute);
            Ok(Some(diff))
        } else {
            self.previous = Some(statute);
            Ok(None)
        }
    }

    /// Returns all accumulated diffs.
    pub fn get_diffs(&self) -> &[StatuteDiff] {
        &self.diffs
    }

    /// Resets the incremental differ.
    pub fn reset(&mut self) {
        self.previous = None;
        self.diffs.clear();
    }
}

impl Default for IncrementalDiffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Batch diff computation with optimizations.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::optimization::BatchDiffer;
///
/// let mut batch = BatchDiffer::new(100);
/// let pairs = vec![
///     (
///         Statute::new("law1", "Old", Effect::new(EffectType::Grant, "Benefit")),
///         Statute::new("law1", "New", Effect::new(EffectType::Grant, "Benefit")),
///     ),
/// ];
///
/// let diffs = batch.compute_batch(&pairs).unwrap();
/// assert_eq!(diffs.len(), 1);
/// ```
pub struct BatchDiffer {
    cache: DiffCache,
}

impl BatchDiffer {
    /// Creates a new batch differ with the specified cache size.
    pub fn new(cache_size: usize) -> Self {
        Self {
            cache: DiffCache::new(cache_size),
        }
    }

    /// Computes diffs for multiple statute pairs with caching.
    pub fn compute_batch(&mut self, pairs: &[(Statute, Statute)]) -> DiffResult<Vec<StatuteDiff>> {
        pairs
            .iter()
            .map(|(old, new)| self.cache.get_or_compute(old, new, crate::diff))
            .collect()
    }

    /// Returns cache statistics.
    pub fn cache_stats(&self) -> CacheStats {
        self.cache.stats()
    }

    /// Clears the cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for BatchDiffer {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn create_test_statute(id: &str, title: &str, age: u32) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Test effect")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: age,
            },
        )
    }

    #[test]
    fn test_diff_cache_basic() {
        let mut cache = DiffCache::new(10);
        let old = create_test_statute("test", "Old", 18);
        let new = create_test_statute("test", "New", 21);

        let result1 = cache.get_or_compute(&old, &new, crate::diff).unwrap();
        let result2 = cache.get_or_compute(&old, &new, crate::diff).unwrap();

        assert_eq!(result1.statute_id, result2.statute_id);
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache = DiffCache::new(2);

        for i in 0..5 {
            let old = create_test_statute("test", "Old", 18);
            let new = create_test_statute("test", &format!("New{}", i), 18 + i);
            let _ = cache.get_or_compute(&old, &new, crate::diff);
        }

        let stats = cache.stats();
        assert!(stats.size <= 2);
    }

    #[test]
    fn test_cache_stats() {
        let cache = DiffCache::new(10);
        let stats = cache.stats();
        assert_eq!(stats.size, 0);
        assert_eq!(stats.hit_rate, 0.0);
    }

    #[test]
    fn test_incremental_differ() {
        let mut differ = IncrementalDiffer::new();

        let v1 = create_test_statute("test", "V1", 18);
        let result1 = differ.add_version(v1).unwrap();
        assert!(result1.is_none()); // First version, no diff

        let v2 = create_test_statute("test", "V2", 21);
        let result2 = differ.add_version(v2).unwrap();
        assert!(result2.is_some()); // Second version, has diff

        assert_eq!(differ.get_diffs().len(), 1);
    }

    #[test]
    fn test_incremental_differ_multiple_versions() {
        let mut differ = IncrementalDiffer::new();

        for i in 0..5 {
            let v = create_test_statute("test", &format!("V{}", i), 18 + i);
            let _ = differ.add_version(v);
        }

        assert_eq!(differ.get_diffs().len(), 4); // 4 diffs for 5 versions
    }

    #[test]
    fn test_incremental_differ_reset() {
        let mut differ = IncrementalDiffer::new();
        let v1 = create_test_statute("test", "V1", 18);
        let v2 = create_test_statute("test", "V2", 21);

        let _ = differ.add_version(v1);
        let _ = differ.add_version(v2);
        assert_eq!(differ.get_diffs().len(), 1);

        differ.reset();
        assert_eq!(differ.get_diffs().len(), 0);
    }

    #[test]
    fn test_batch_differ() {
        let mut batch = BatchDiffer::new(10);
        let pairs = vec![
            (
                create_test_statute("test1", "Old1", 18),
                create_test_statute("test1", "New1", 21),
            ),
            (
                create_test_statute("test2", "Old2", 18),
                create_test_statute("test2", "New2", 21),
            ),
        ];

        let diffs = batch.compute_batch(&pairs).unwrap();
        assert_eq!(diffs.len(), 2);
    }

    #[test]
    fn test_batch_differ_caching() {
        let mut batch = BatchDiffer::new(10);
        let old = create_test_statute("test", "Old", 18);
        let new = create_test_statute("test", "New", 21);

        let pairs = vec![
            (old.clone(), new.clone()),
            (old.clone(), new.clone()), // Same pair should use cache
        ];

        let _ = batch.compute_batch(&pairs).unwrap();
        let stats = batch.cache_stats();
        assert_eq!(stats.hits, 1);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = DiffCache::new(10);
        let old = create_test_statute("test", "Old", 18);
        let new = create_test_statute("test", "New", 21);

        let _ = cache.get_or_compute(&old, &new, crate::diff);
        assert_eq!(cache.stats().size, 1);

        cache.clear();
        assert_eq!(cache.stats().size, 0);
        assert_eq!(cache.stats().hits, 0);
    }
}
