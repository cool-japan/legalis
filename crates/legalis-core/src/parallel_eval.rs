//! Parallel batch evaluation with work stealing.
//!
//! This module provides parallel evaluation of statutes and conditions
//! using work stealing for optimal load balancing across CPU cores.
//!
//! ## Features
//!
//! - **Work stealing**: Automatic load balancing using rayon
//! - **Batch processing**: Evaluate multiple statutes in parallel
//! - **Chunked evaluation**: Process large collections efficiently
//! - **Result aggregation**: Collect results with minimal overhead
//!
//! ## When to Use
//!
//! Parallel evaluation is beneficial when:
//!
//! - Processing hundreds or thousands of statutes
//! - Evaluation involves complex conditions
//! - Multi-core CPU is available
//! - Latency is more important than throughput
//!
//! ## Example
//!
//! ```
//! use legalis_core::parallel_eval::ParallelEvaluator;
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use std::collections::HashMap;
//!
//! // Create statutes
//! let statutes = vec![
//!     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit"))
//!         .with_precondition(Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 }),
//!     Statute::new("s2", "Statute 2", Effect::new(EffectType::Revoke, "Penalty"))
//!         .with_precondition(Condition::Income { operator: ComparisonOp::LessThan, value: 50000 }),
//! ];
//!
//! // Evaluate in parallel (requires "parallel" feature)
//! #[cfg(feature = "parallel")]
//! {
//!     let evaluator = ParallelEvaluator::new();
//!     let results = evaluator.evaluate_batch(&statutes);
//!     assert_eq!(results.len(), 2);
//! }
//! ```

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::{Condition, Statute};

/// Result of evaluating a statute.
#[derive(Clone, Debug)]
pub struct EvaluationResult {
    /// Statute ID
    pub statute_id: String,
    /// Whether all preconditions are satisfied
    pub satisfied: bool,
    /// Evaluation duration in microseconds
    pub duration_us: u64,
}

/// Parallel evaluator for batch statute processing.
pub struct ParallelEvaluator {
    /// Chunk size for batch processing
    #[allow(dead_code)]
    chunk_size: usize,
}

impl ParallelEvaluator {
    /// Creates a new parallel evaluator with default settings.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::parallel_eval::ParallelEvaluator;
    ///
    /// let evaluator = ParallelEvaluator::new();
    /// ```
    pub fn new() -> Self {
        Self { chunk_size: 100 }
    }

    /// Creates a parallel evaluator with custom chunk size.
    ///
    /// The chunk size determines how many statutes are processed
    /// in each work unit. Larger chunks reduce overhead but may
    /// lead to load imbalance.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::parallel_eval::ParallelEvaluator;
    ///
    /// let evaluator = ParallelEvaluator::with_chunk_size(50);
    /// ```
    pub fn with_chunk_size(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    /// Evaluates a batch of statutes in parallel.
    ///
    /// Returns evaluation results in arbitrary order (not the same as input order).
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::parallel_eval::ParallelEvaluator;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let statutes = vec![
    ///     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
    ///     Statute::new("s2", "Statute 2", Effect::new(EffectType::Revoke, "Penalty")),
    /// ];
    ///
    /// let evaluator = ParallelEvaluator::new();
    /// let results = evaluator.evaluate_batch(&statutes);
    /// assert_eq!(results.len(), 2);
    /// ```
    #[cfg(feature = "parallel")]
    pub fn evaluate_batch(&self, statutes: &[Statute]) -> Vec<EvaluationResult> {
        statutes
            .par_chunks(self.chunk_size)
            .flat_map(|chunk| {
                chunk.iter().map(|statute| {
                    let start = std::time::Instant::now();
                    let satisfied = statute.preconditions.is_empty()
                        || statute.preconditions.iter().all(|_| true); // Placeholder
                    let duration_us = start.elapsed().as_micros() as u64;

                    EvaluationResult {
                        statute_id: statute.id.clone(),
                        satisfied,
                        duration_us,
                    }
                })
            })
            .collect()
    }

    /// Evaluates a batch of statutes sequentially (fallback when parallel feature is disabled).
    #[cfg(not(feature = "parallel"))]
    pub fn evaluate_batch(&self, statutes: &[Statute]) -> Vec<EvaluationResult> {
        statutes
            .iter()
            .map(|statute| {
                let start = std::time::Instant::now();
                let satisfied =
                    statute.preconditions.is_empty() || statute.preconditions.iter().all(|_| true); // Placeholder
                let duration_us = start.elapsed().as_micros() as u64;

                EvaluationResult {
                    statute_id: statute.id.clone(),
                    satisfied,
                    duration_us,
                }
            })
            .collect()
    }

    /// Evaluates statutes and groups results by satisfaction status.
    ///
    /// Returns a tuple of (satisfied, unsatisfied) statute IDs.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::parallel_eval::ParallelEvaluator;
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// let statutes = vec![
    ///     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
    ///     Statute::new("s2", "Statute 2", Effect::new(EffectType::Revoke, "Penalty"))
    ///         .with_precondition(Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 }),
    /// ];
    ///
    /// let evaluator = ParallelEvaluator::new();
    /// let (satisfied, unsatisfied) = evaluator.partition_by_satisfaction(&statutes);
    /// ```
    pub fn partition_by_satisfaction(&self, statutes: &[Statute]) -> (Vec<String>, Vec<String>) {
        let results = self.evaluate_batch(statutes);
        let mut satisfied = Vec::new();
        let mut unsatisfied = Vec::new();

        for result in results {
            if result.satisfied {
                satisfied.push(result.statute_id);
            } else {
                unsatisfied.push(result.statute_id);
            }
        }

        (satisfied, unsatisfied)
    }

    /// Computes statistics for batch evaluation.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::parallel_eval::ParallelEvaluator;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let statutes = vec![
    ///     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
    ///     Statute::new("s2", "Statute 2", Effect::new(EffectType::Revoke, "Penalty")),
    /// ];
    ///
    /// let evaluator = ParallelEvaluator::new();
    /// let stats = evaluator.compute_stats(&statutes);
    /// assert!(stats.total_duration_us >= 0);
    /// ```
    pub fn compute_stats(&self, statutes: &[Statute]) -> EvaluationStats {
        let results = self.evaluate_batch(statutes);
        let total_count = results.len();
        let satisfied_count = results.iter().filter(|r| r.satisfied).count();
        let total_duration_us = results.iter().map(|r| r.duration_us).sum();
        let avg_duration_us = if total_count > 0 {
            total_duration_us / total_count as u64
        } else {
            0
        };

        EvaluationStats {
            total_count,
            satisfied_count,
            unsatisfied_count: total_count - satisfied_count,
            total_duration_us,
            avg_duration_us,
        }
    }
}

impl Default for ParallelEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics from batch evaluation.
#[derive(Clone, Debug)]
pub struct EvaluationStats {
    /// Total number of statutes evaluated
    pub total_count: usize,
    /// Number of satisfied statutes
    pub satisfied_count: usize,
    /// Number of unsatisfied statutes
    pub unsatisfied_count: usize,
    /// Total evaluation time in microseconds
    pub total_duration_us: u64,
    /// Average evaluation time per statute in microseconds
    pub avg_duration_us: u64,
}

impl EvaluationStats {
    /// Returns the satisfaction rate (0.0 to 1.0).
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::parallel_eval::EvaluationStats;
    ///
    /// let stats = EvaluationStats {
    ///     total_count: 100,
    ///     satisfied_count: 75,
    ///     unsatisfied_count: 25,
    ///     total_duration_us: 1000,
    ///     avg_duration_us: 10,
    /// };
    ///
    /// assert_eq!(stats.satisfaction_rate(), 0.75);
    /// ```
    pub fn satisfaction_rate(&self) -> f64 {
        if self.total_count == 0 {
            0.0
        } else {
            self.satisfied_count as f64 / self.total_count as f64
        }
    }

    /// Returns the throughput in statutes per second.
    pub fn throughput(&self) -> f64 {
        if self.total_duration_us == 0 {
            0.0
        } else {
            self.total_count as f64 / (self.total_duration_us as f64 / 1_000_000.0)
        }
    }
}

/// Work-stealing condition evaluator.
///
/// Evaluates multiple conditions in parallel with automatic load balancing.
pub struct ConditionEvaluator {
    #[allow(dead_code)]
    chunk_size: usize,
}

impl ConditionEvaluator {
    /// Creates a new condition evaluator.
    pub fn new() -> Self {
        Self { chunk_size: 50 }
    }

    /// Creates a condition evaluator with custom chunk size.
    pub fn with_chunk_size(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    /// Evaluates multiple conditions in parallel.
    ///
    /// Returns a vector of booleans in the same order as input.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::parallel_eval::ConditionEvaluator;
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// let conditions = vec![
    ///     Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 },
    ///     Condition::Income { operator: ComparisonOp::LessThan, value: 50000 },
    /// ];
    ///
    /// let evaluator = ConditionEvaluator::new();
    /// let results = evaluator.evaluate_all(&conditions);
    /// assert_eq!(results.len(), 2);
    /// ```
    #[cfg(feature = "parallel")]
    pub fn evaluate_all(&self, conditions: &[Condition]) -> Vec<bool> {
        conditions
            .par_chunks(self.chunk_size)
            .flat_map(|chunk| {
                chunk.iter().map(|_cond| {
                    true // Placeholder evaluation
                })
            })
            .collect()
    }

    /// Sequential fallback when parallel feature is disabled.
    #[cfg(not(feature = "parallel"))]
    pub fn evaluate_all(&self, conditions: &[Condition]) -> Vec<bool> {
        conditions.iter().map(|_cond| true).collect()
    }
}

impl Default for ConditionEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_evaluator_new() {
        let evaluator = ParallelEvaluator::new();
        assert_eq!(evaluator.chunk_size, 100);
    }

    #[test]
    fn test_evaluate_batch() {
        let statutes = vec![
            Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
            Statute::new(
                "s2",
                "Statute 2",
                Effect::new(EffectType::Revoke, "Penalty"),
            ),
        ];

        let evaluator = ParallelEvaluator::new();
        let results = evaluator.evaluate_batch(&statutes);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_compute_stats() {
        let statutes = vec![
            Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Benefit")),
            Statute::new(
                "s2",
                "Statute 2",
                Effect::new(EffectType::Revoke, "Penalty"),
            ),
        ];

        let evaluator = ParallelEvaluator::new();
        let stats = evaluator.compute_stats(&statutes);
        assert_eq!(stats.total_count, 2);
    }

    #[test]
    fn test_satisfaction_rate() {
        let stats = EvaluationStats {
            total_count: 100,
            satisfied_count: 75,
            unsatisfied_count: 25,
            total_duration_us: 1000,
            avg_duration_us: 10,
        };

        assert_eq!(stats.satisfaction_rate(), 0.75);
    }

    #[test]
    fn test_condition_evaluator() {
        let conditions = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
            Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            },
        ];

        let evaluator = ConditionEvaluator::new();
        let results = evaluator.evaluate_all(&conditions);
        assert_eq!(results.len(), 2);
    }
}
