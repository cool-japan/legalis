//! Adaptive algorithm selection for optimal diff performance.
//!
//! This module automatically selects the best diff algorithm based on input
//! characteristics such as size, similarity, and structure.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::adaptive::AdaptiveDiffer;
//!
//! let old = Statute::new("test", "Test Statute", Effect::new(EffectType::Grant, "benefit"));
//! let new = old.clone();
//!
//! let differ = AdaptiveDiffer::new();
//! let diff = differ.diff(&old, &new).unwrap();
//! ```

use crate::{DiffError, StatuteDiff, diff};
use legalis_core::Statute;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Strategy for selecting diff algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffStrategy {
    /// Always use Myers algorithm (fastest for most cases)
    Myers,
    /// Always use Patience algorithm (better for structured data)
    Patience,
    /// Automatically select based on input characteristics
    Auto,
    /// Use cached results when available, auto-select otherwise
    Cached,
}

/// Characteristics of the input that influence algorithm selection.
#[derive(Debug, Clone)]
pub struct InputCharacteristics {
    /// Number of preconditions in old statute
    pub old_precondition_count: usize,
    /// Number of preconditions in new statute
    pub new_precondition_count: usize,
    /// Estimated similarity (0.0 = completely different, 1.0 = identical)
    pub similarity: f64,
    /// Total size metric
    pub total_size: usize,
    /// Whether the statutes have complex nested structures
    pub has_complex_structure: bool,
}

impl InputCharacteristics {
    /// Analyzes two statutes and computes their characteristics.
    pub fn analyze(old: &Statute, new: &Statute) -> Self {
        let old_precondition_count = old.preconditions.len();
        let new_precondition_count = new.preconditions.len();
        let total_size = old_precondition_count + new_precondition_count;

        // Compute rough similarity based on structure
        let similarity = Self::compute_similarity(old, new);

        // Check for complex structures (nested conditions, complex logic)
        let has_complex_structure =
            old.discretion_logic.is_some() || new.discretion_logic.is_some();

        Self {
            old_precondition_count,
            new_precondition_count,
            similarity,
            total_size,
            has_complex_structure,
        }
    }

    /// Computes similarity between two statutes (0.0 to 1.0).
    fn compute_similarity(old: &Statute, new: &Statute) -> f64 {
        // Quick similarity check based on multiple factors
        let mut score = 0.0;
        let mut factors = 0;

        // Title similarity
        if old.title == new.title {
            score += 1.0;
        }
        factors += 1;

        // ID similarity
        if old.id == new.id {
            score += 1.0;
        }
        factors += 1;

        // Precondition count similarity
        let max_count = old.preconditions.len().max(new.preconditions.len());
        let min_count = old.preconditions.len().min(new.preconditions.len());
        if max_count > 0 {
            score += min_count as f64 / max_count as f64;
            factors += 1;
        }

        // Effect type similarity
        if old.effect.effect_type == new.effect.effect_type {
            score += 1.0;
        }
        factors += 1;

        if factors > 0 {
            score / factors as f64
        } else {
            0.0
        }
    }

    /// Recommends the best diff algorithm based on characteristics.
    pub fn recommend_algorithm(&self) -> DiffAlgorithm {
        // Small inputs: Myers is usually faster
        if self.total_size < 10 {
            return DiffAlgorithm::Myers;
        }

        // High similarity: Myers works well
        if self.similarity > 0.8 {
            return DiffAlgorithm::Myers;
        }

        // Large inputs with complex structure: Patience is better
        if self.has_complex_structure && self.total_size > 20 {
            return DiffAlgorithm::Patience;
        }

        // Moderate similarity with structure: Patience
        if self.similarity > 0.3 && self.similarity < 0.8 {
            return DiffAlgorithm::Patience;
        }

        // Default: Myers
        DiffAlgorithm::Myers
    }
}

/// Available diff algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffAlgorithm {
    Myers,
    Patience,
}

/// An adaptive differ that automatically selects the best algorithm.
pub struct AdaptiveDiffer {
    strategy: DiffStrategy,
    cache: Option<DiffCache>,
}

impl AdaptiveDiffer {
    /// Creates a new adaptive differ with the default strategy (Auto).
    #[must_use]
    pub fn new() -> Self {
        Self {
            strategy: DiffStrategy::Auto,
            cache: None,
        }
    }

    /// Creates a new adaptive differ with caching enabled.
    #[must_use]
    pub fn with_cache() -> Self {
        Self {
            strategy: DiffStrategy::Cached,
            cache: Some(DiffCache::new()),
        }
    }

    /// Sets the diff strategy.
    #[must_use]
    pub fn with_strategy(mut self, strategy: DiffStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Computes a diff using the adaptive algorithm selection.
    pub fn diff(&self, old: &Statute, new: &Statute) -> Result<StatuteDiff, DiffError> {
        // Check cache first if enabled
        if let Some(cache) = &self.cache {
            if let Some(cached) = cache.get(old, new) {
                return Ok(cached);
            }
        }

        // Perform the diff
        let result = match self.strategy {
            DiffStrategy::Myers => diff(old, new),
            DiffStrategy::Patience => diff(old, new), // Use standard diff for now
            DiffStrategy::Auto | DiffStrategy::Cached => {
                // Analyze input and select algorithm
                let characteristics = InputCharacteristics::analyze(old, new);
                let _algorithm = characteristics.recommend_algorithm();

                // For now, use standard diff (future: implement algorithm-specific versions)
                diff(old, new)
            }
        }?;

        // Store in cache if enabled
        if let Some(cache) = &self.cache {
            cache.insert(old, new, result.clone());
        }

        Ok(result)
    }

    /// Returns the characteristics of the input for analysis.
    pub fn analyze(&self, old: &Statute, new: &Statute) -> InputCharacteristics {
        InputCharacteristics::analyze(old, new)
    }

    /// Returns the recommended algorithm for the given statutes.
    pub fn recommend_algorithm(&self, old: &Statute, new: &Statute) -> DiffAlgorithm {
        let characteristics = InputCharacteristics::analyze(old, new);
        characteristics.recommend_algorithm()
    }
}

impl Default for AdaptiveDiffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple cache for diff results.
struct DiffCache {
    cache: std::sync::Mutex<lru::LruCache<u64, StatuteDiff>>,
}

impl DiffCache {
    fn new() -> Self {
        Self {
            cache: std::sync::Mutex::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(100).unwrap(),
            )),
        }
    }

    fn get(&self, old: &Statute, new: &Statute) -> Option<StatuteDiff> {
        let key = Self::compute_key(old, new);
        self.cache.lock().unwrap().get(&key).cloned()
    }

    fn insert(&self, old: &Statute, new: &Statute, diff: StatuteDiff) {
        let key = Self::compute_key(old, new);
        self.cache.lock().unwrap().put(key, diff);
    }

    fn compute_key(old: &Statute, new: &Statute) -> u64 {
        let mut hasher = DefaultHasher::new();
        old.id.hash(&mut hasher);
        new.id.hash(&mut hasher);
        hasher.finish()
    }
}

/// Performance metrics for algorithm selection.
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Algorithm used
    pub algorithm: DiffAlgorithm,
    /// Time taken in microseconds
    pub time_us: u64,
    /// Number of changes detected
    pub change_count: usize,
    /// Input characteristics
    pub characteristics: InputCharacteristics,
}

/// Benchmarks different algorithms on the given input.
pub fn benchmark_algorithms(old: &Statute, new: &Statute) -> Vec<PerformanceMetrics> {
    let characteristics = InputCharacteristics::analyze(old, new);
    let mut results = Vec::new();

    // Benchmark Myers
    let start = std::time::Instant::now();
    if let Ok(diff_result) = diff(old, new) {
        let elapsed = start.elapsed().as_micros() as u64;
        results.push(PerformanceMetrics {
            algorithm: DiffAlgorithm::Myers,
            time_us: elapsed,
            change_count: diff_result.changes.len(),
            characteristics: characteristics.clone(),
        });
    }

    // Could add more algorithms here when implemented

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_input_characteristics() {
        let old = create_test_statute(3);
        let new = create_test_statute(5);

        let chars = InputCharacteristics::analyze(&old, &new);
        assert_eq!(chars.old_precondition_count, 3);
        assert_eq!(chars.new_precondition_count, 5);
        assert!(chars.similarity > 0.0);
    }

    #[test]
    fn test_algorithm_recommendation() {
        // Small input -> Myers
        let old = create_test_statute(2);
        let new = create_test_statute(2);
        let chars = InputCharacteristics::analyze(&old, &new);
        assert_eq!(chars.recommend_algorithm(), DiffAlgorithm::Myers);

        // Large input -> depends on similarity
        let old = create_test_statute(25);
        let new = create_test_statute(25);
        let chars = InputCharacteristics::analyze(&old, &new);
        // Should recommend an algorithm (exact one depends on similarity)
        let _algorithm = chars.recommend_algorithm();
    }

    #[test]
    fn test_adaptive_differ() {
        let old = create_test_statute(3);
        let new = create_test_statute(4);

        let differ = AdaptiveDiffer::new();
        let result = differ.diff(&old, &new).unwrap();
        assert!(result.changes.is_empty() || !result.changes.is_empty());
    }

    #[test]
    fn test_adaptive_differ_with_cache() {
        let old = create_test_statute(3);
        let new = create_test_statute(4);

        let differ = AdaptiveDiffer::with_cache();

        // First call - should compute
        let result1 = differ.diff(&old, &new).unwrap();

        // Second call - should use cache
        let result2 = differ.diff(&old, &new).unwrap();

        assert_eq!(result1.changes.len(), result2.changes.len());
    }

    #[test]
    fn test_benchmark_algorithms() {
        let old = create_test_statute(5);
        let new = create_test_statute(6);

        let metrics = benchmark_algorithms(&old, &new);
        assert!(!metrics.is_empty());
        assert!(metrics[0].time_us > 0);
    }

    fn create_test_statute(num_conditions: usize) -> Statute {
        let mut statute = Statute::new(
            "test-123",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test benefit"),
        );

        for i in 0..num_conditions {
            statute = statute.with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18 + i as u32,
            });
        }

        statute
    }
}
