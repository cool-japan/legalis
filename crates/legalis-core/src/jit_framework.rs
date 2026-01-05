//! JIT (Just-In-Time) compilation framework for hot evaluation paths.
//!
//! This module provides a trait-based framework for JIT compilation of frequently
//! evaluated conditions. The framework is designed to be backend-agnostic and can
//! be integrated with various JIT engines (Cranelift, LLVM, etc.).
//!
//! ## Features
//!
//! - **Hot Path Detection**: Automatically identifies frequently evaluated conditions
//! - **Compilation Cache**: Caches compiled code for reuse
//! - **Backend Abstraction**: Trait-based design for pluggable JIT backends
//! - **Fallback Support**: Gracefully falls back to interpretation when JIT is unavailable
//!
//! ## Example
//!
//! ```
//! use legalis_core::jit_framework::{JitCompiler, HotPathTracker};
//! use legalis_core::{Condition, ComparisonOp};
//!
//! let mut tracker = HotPathTracker::new(100); // Compile after 100 evaluations
//!
//! let condition = Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 18,
//! };
//!
//! // Track evaluations
//! for _ in 0..150 {
//!     tracker.record_evaluation(&condition);
//! }
//!
//! // Check if condition is hot
//! assert!(tracker.is_hot(&condition));
//! ```

use crate::Condition;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Trait for JIT compilation backends.
///
/// Implementors can provide custom JIT compilation logic using
/// their preferred JIT engine (Cranelift, LLVM, etc.).
pub trait JitBackend: Send + Sync {
    /// Compiles a condition to native code.
    ///
    /// Returns a function pointer that can be called to evaluate the condition,
    /// or None if compilation failed.
    fn compile_condition(&mut self, condition: &Condition) -> Option<CompiledCode>;

    /// Returns the name of the JIT backend.
    fn backend_name(&self) -> &str;

    /// Returns whether the backend supports the given condition type.
    fn supports_condition(&self, condition: &Condition) -> bool;
}

/// Compiled code representation.
///
/// This is an opaque handle to compiled code. The actual implementation
/// depends on the JIT backend being used.
#[derive(Debug, Clone)]
pub struct CompiledCode {
    /// Unique identifier for the compiled code
    pub id: u64,
    /// Estimated execution time in nanoseconds
    pub estimated_cost: u64,
    /// Backend-specific data (opaque)
    #[allow(dead_code)]
    backend_data: Vec<u8>,
}

impl CompiledCode {
    /// Creates a new compiled code handle.
    pub fn new(id: u64, estimated_cost: u64) -> Self {
        Self {
            id,
            estimated_cost,
            backend_data: Vec::new(),
        }
    }

    /// Returns the compilation ID.
    pub fn id(&self) -> u64 {
        self.id
    }
}

/// Tracks hot evaluation paths to identify compilation candidates.
///
/// # Example
///
/// ```
/// use legalis_core::jit_framework::HotPathTracker;
/// use legalis_core::{Condition, ComparisonOp};
///
/// let mut tracker = HotPathTracker::new(50);
///
/// let condition = Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 21,
/// };
///
/// for i in 0..60 {
///     tracker.record_evaluation(&condition);
///     if i >= 50 {
///         assert!(tracker.is_hot(&condition));
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct HotPathTracker {
    /// Evaluation counts per condition
    eval_counts: HashMap<u64, u64>,
    /// Threshold for considering a path "hot"
    hot_threshold: u64,
    /// Total evaluations
    total_evaluations: u64,
}

impl HotPathTracker {
    /// Creates a new hot path tracker.
    ///
    /// # Arguments
    ///
    /// * `hot_threshold` - Number of evaluations before a path is considered hot
    pub fn new(hot_threshold: u64) -> Self {
        Self {
            eval_counts: HashMap::new(),
            hot_threshold,
            total_evaluations: 0,
        }
    }

    /// Records an evaluation of a condition.
    pub fn record_evaluation(&mut self, condition: &Condition) {
        let hash = self.hash_condition(condition);
        *self.eval_counts.entry(hash).or_insert(0) += 1;
        self.total_evaluations += 1;
    }

    /// Checks if a condition is a hot path.
    pub fn is_hot(&self, condition: &Condition) -> bool {
        let hash = self.hash_condition(condition);
        self.eval_counts.get(&hash).copied().unwrap_or(0) >= self.hot_threshold
    }

    /// Returns the evaluation count for a condition.
    pub fn eval_count(&self, condition: &Condition) -> u64 {
        let hash = self.hash_condition(condition);
        self.eval_counts.get(&hash).copied().unwrap_or(0)
    }

    /// Returns the total number of evaluations tracked.
    pub fn total_evaluations(&self) -> u64 {
        self.total_evaluations
    }

    /// Returns all hot paths.
    pub fn hot_paths(&self) -> Vec<u64> {
        self.eval_counts
            .iter()
            .filter(|(_, count)| **count >= self.hot_threshold)
            .map(|(hash, _)| *hash)
            .collect()
    }

    /// Resets all tracking data.
    pub fn reset(&mut self) {
        self.eval_counts.clear();
        self.total_evaluations = 0;
    }

    /// Computes a hash for a condition.
    fn hash_condition(&self, condition: &Condition) -> u64 {
        let mut hasher = DefaultHasher::new();
        // Simple hash based on condition type (in real impl, would be more sophisticated)
        match condition {
            Condition::Age { operator, value } => {
                "age".hash(&mut hasher);
                operator.hash(&mut hasher);
                value.hash(&mut hasher);
            }
            Condition::Income { operator, value } => {
                "income".hash(&mut hasher);
                operator.hash(&mut hasher);
                value.hash(&mut hasher);
            }
            Condition::And(left, right) => {
                "and".hash(&mut hasher);
                self.hash_condition(left).hash(&mut hasher);
                self.hash_condition(right).hash(&mut hasher);
            }
            Condition::Or(left, right) => {
                "or".hash(&mut hasher);
                self.hash_condition(left).hash(&mut hasher);
                self.hash_condition(right).hash(&mut hasher);
            }
            Condition::Not(inner) => {
                "not".hash(&mut hasher);
                self.hash_condition(inner).hash(&mut hasher);
            }
            _ => {
                "other".hash(&mut hasher);
            }
        }
        hasher.finish()
    }
}

/// JIT compiler that manages hot path compilation and caching.
///
/// # Example
///
/// ```
/// use legalis_core::jit_framework::{JitCompiler, HotPathTracker};
///
/// let tracker = HotPathTracker::new(100);
/// let compiler = JitCompiler::new(tracker);
///
/// assert_eq!(compiler.compilation_count(), 0);
/// ```
pub struct JitCompiler {
    /// Hot path tracker
    tracker: HotPathTracker,
    /// Compiled code cache
    compiled_cache: HashMap<u64, CompiledCode>,
    /// JIT backend (optional)
    #[allow(dead_code)]
    backend: Option<Box<dyn JitBackend>>,
    /// Statistics
    compilation_count: u64,
    cache_hits: u64,
}

impl JitCompiler {
    /// Creates a new JIT compiler.
    pub fn new(tracker: HotPathTracker) -> Self {
        Self {
            tracker,
            compiled_cache: HashMap::new(),
            backend: None,
            compilation_count: 0,
            cache_hits: 0,
        }
    }

    /// Sets the JIT backend.
    #[allow(dead_code)]
    pub fn with_backend(mut self, backend: Box<dyn JitBackend>) -> Self {
        self.backend = Some(backend);
        self
    }

    /// Records an evaluation and potentially triggers compilation.
    pub fn record_evaluation(&mut self, condition: &Condition) {
        self.tracker.record_evaluation(condition);

        if self.tracker.is_hot(condition) && !self.is_compiled(condition) {
            self.try_compile(condition);
        }
    }

    /// Checks if a condition has been compiled.
    pub fn is_compiled(&self, condition: &Condition) -> bool {
        let hash = self.tracker.hash_condition(condition);
        self.compiled_cache.contains_key(&hash)
    }

    /// Attempts to compile a condition.
    fn try_compile(&mut self, condition: &Condition) {
        let hash = self.tracker.hash_condition(condition);

        // Placeholder compilation (real implementation would use JIT backend)
        let compiled = CompiledCode::new(hash, 10);
        self.compiled_cache.insert(hash, compiled);
        self.compilation_count += 1;
    }

    /// Returns the number of compiled conditions.
    pub fn compilation_count(&self) -> u64 {
        self.compilation_count
    }

    /// Returns the number of cache hits.
    pub fn cache_hits(&self) -> u64 {
        self.cache_hits
    }

    /// Returns the hot path tracker.
    pub fn tracker(&self) -> &HotPathTracker {
        &self.tracker
    }

    /// Clears all compiled code.
    pub fn clear_cache(&mut self) {
        self.compiled_cache.clear();
    }
}

/// Statistics for JIT compilation.
#[derive(Debug, Clone, Default)]
pub struct JitStats {
    /// Number of compilations performed
    pub compilations: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Total compilation time in nanoseconds
    pub compilation_time_ns: u64,
}

impl JitStats {
    /// Creates new JIT statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the cache hit rate (0.0 to 1.0).
    pub fn hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    /// Returns the average compilation time in nanoseconds.
    pub fn avg_compilation_time(&self) -> f64 {
        if self.compilations == 0 {
            0.0
        } else {
            self.compilation_time_ns as f64 / self.compilations as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ComparisonOp;

    #[test]
    fn test_hot_path_tracker() {
        let mut tracker = HotPathTracker::new(50);

        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        };

        assert!(!tracker.is_hot(&condition));

        for _ in 0..60 {
            tracker.record_evaluation(&condition);
        }

        assert!(tracker.is_hot(&condition));
        assert_eq!(tracker.eval_count(&condition), 60);
    }

    #[test]
    fn test_hot_paths() {
        let mut tracker = HotPathTracker::new(10);

        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let cond2 = Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        };

        for _ in 0..15 {
            tracker.record_evaluation(&cond1);
        }

        for _ in 0..5 {
            tracker.record_evaluation(&cond2);
        }

        let hot = tracker.hot_paths();
        assert_eq!(hot.len(), 1); // Only cond1 is hot
    }

    #[test]
    fn test_jit_compiler() {
        let tracker = HotPathTracker::new(100);
        let mut compiler = JitCompiler::new(tracker);

        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        assert_eq!(compiler.compilation_count(), 0);

        for _ in 0..150 {
            compiler.record_evaluation(&condition);
        }

        assert!(compiler.is_compiled(&condition));
        assert_eq!(compiler.compilation_count(), 1);
    }

    #[test]
    fn test_tracker_reset() {
        let mut tracker = HotPathTracker::new(10);

        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        for _ in 0..20 {
            tracker.record_evaluation(&condition);
        }

        assert!(tracker.is_hot(&condition));

        tracker.reset();
        assert!(!tracker.is_hot(&condition));
        assert_eq!(tracker.total_evaluations(), 0);
    }

    #[test]
    fn test_jit_stats() {
        let mut stats = JitStats::new();
        stats.compilations = 10;
        stats.cache_hits = 80;
        stats.cache_misses = 20;
        stats.compilation_time_ns = 1000000;

        assert_eq!(stats.hit_rate(), 0.8);
        assert_eq!(stats.avg_compilation_time(), 100000.0);
    }

    #[test]
    fn test_compiled_code() {
        let code = CompiledCode::new(12345, 50);
        assert_eq!(code.id(), 12345);
        assert_eq!(code.estimated_cost, 50);
    }
}
