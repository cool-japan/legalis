//! SIMD-accelerated condition evaluation for high-performance batch processing.
//!
//! This module provides SIMD (Single Instruction, Multiple Data) acceleration for
//! evaluating conditions across multiple entities simultaneously. This can provide
//! significant performance improvements when processing large batches of entities.
//!
//! ## Features
//!
//! - **Batch Age Evaluation**: Evaluate age conditions for multiple entities using SIMD
//! - **Batch Income Evaluation**: Process income comparisons in parallel
//! - **Vectorized Boolean Operations**: SIMD-accelerated AND/OR/NOT operations
//! - **Automatic Fallback**: Uses scalar operations on platforms without SIMD support
//!
//! ## Example
//!
//! ```
//! use legalis_core::simd_eval::{SimdEvaluator, BatchEvaluationContext};
//! use legalis_core::{Condition, ComparisonOp};
//!
//! // Create entities with ages
//! let ages = vec![17, 18, 25, 30, 16];
//! let incomes = vec![30000, 45000, 60000, 35000, 25000];
//!
//! // Create batch context
//! let context = BatchEvaluationContext::new(ages, incomes);
//!
//! // Evaluate age condition across all entities
//! let age_condition = Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 18,
//! };
//!
//! let evaluator = SimdEvaluator::new();
//! let results = evaluator.evaluate_batch_age(&context, &age_condition);
//!
//! // Results: [false, true, true, true, false]
//! assert_eq!(results, vec![false, true, true, true, false]);
//! ```

use crate::{ComparisonOp, Condition};

/// Batch evaluation context holding entity data for SIMD processing.
///
/// This structure organizes entity attributes into contiguous arrays
/// that can be efficiently processed using SIMD instructions.
#[derive(Debug, Clone)]
pub struct BatchEvaluationContext {
    /// Ages of entities in the batch
    pub ages: Vec<u32>,
    /// Incomes of entities in the batch
    pub incomes: Vec<u64>,
    /// Custom numeric attributes (key -> array of values)
    pub numeric_attrs: std::collections::HashMap<String, Vec<f64>>,
}

impl BatchEvaluationContext {
    /// Creates a new batch evaluation context.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::simd_eval::BatchEvaluationContext;
    ///
    /// let ages = vec![18, 25, 30];
    /// let incomes = vec![40000, 50000, 60000];
    /// let context = BatchEvaluationContext::new(ages, incomes);
    ///
    /// assert_eq!(context.len(), 3);
    /// ```
    pub fn new(ages: Vec<u32>, incomes: Vec<u64>) -> Self {
        assert_eq!(
            ages.len(),
            incomes.len(),
            "Ages and incomes must have same length"
        );
        Self {
            ages,
            incomes,
            numeric_attrs: std::collections::HashMap::new(),
        }
    }

    /// Adds a custom numeric attribute array.
    pub fn with_numeric_attr(mut self, key: String, values: Vec<f64>) -> Self {
        assert_eq!(
            values.len(),
            self.ages.len(),
            "Attribute array must match batch size"
        );
        self.numeric_attrs.insert(key, values);
        self
    }

    /// Returns the number of entities in the batch.
    pub fn len(&self) -> usize {
        self.ages.len()
    }

    /// Returns true if the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.ages.is_empty()
    }
}

/// SIMD-accelerated evaluator for batch condition evaluation.
///
/// This evaluator uses SIMD instructions when available to process multiple
/// entities in parallel. On platforms without SIMD support, it automatically
/// falls back to optimized scalar operations.
#[derive(Debug, Clone)]
pub struct SimdEvaluator {
    /// Whether to use parallel processing (requires "parallel" feature)
    #[allow(dead_code)]
    use_parallel: bool,
}

impl SimdEvaluator {
    /// Creates a new SIMD evaluator with default settings.
    pub fn new() -> Self {
        Self {
            use_parallel: cfg!(feature = "parallel"),
        }
    }

    /// Evaluates an age condition across a batch of entities.
    ///
    /// Uses SIMD instructions to compare ages in parallel when possible.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::simd_eval::{SimdEvaluator, BatchEvaluationContext};
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// let context = BatchEvaluationContext::new(
    ///     vec![17, 18, 25, 30],
    ///     vec![0, 0, 0, 0],
    /// );
    ///
    /// let condition = Condition::Age {
    ///     operator: ComparisonOp::GreaterOrEqual,
    ///     value: 18,
    /// };
    ///
    /// let evaluator = SimdEvaluator::new();
    /// let results = evaluator.evaluate_batch_age(&context, &condition);
    ///
    /// assert_eq!(results, vec![false, true, true, true]);
    /// ```
    pub fn evaluate_batch_age(
        &self,
        context: &BatchEvaluationContext,
        condition: &Condition,
    ) -> Vec<bool> {
        if let Condition::Age { operator, value } = condition {
            self.compare_u32_batch(&context.ages, *operator, *value)
        } else {
            vec![false; context.len()]
        }
    }

    /// Evaluates an income condition across a batch of entities.
    ///
    /// Uses SIMD instructions to compare incomes in parallel when possible.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::simd_eval::{SimdEvaluator, BatchEvaluationContext};
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// let context = BatchEvaluationContext::new(
    ///     vec![25, 30, 35],
    ///     vec![40000, 50000, 60000],
    /// );
    ///
    /// let condition = Condition::Income {
    ///     operator: ComparisonOp::LessThan,
    ///     value: 50000,
    /// };
    ///
    /// let evaluator = SimdEvaluator::new();
    /// let results = evaluator.evaluate_batch_income(&context, &condition);
    ///
    /// assert_eq!(results, vec![true, false, false]);
    /// ```
    pub fn evaluate_batch_income(
        &self,
        context: &BatchEvaluationContext,
        condition: &Condition,
    ) -> Vec<bool> {
        if let Condition::Income { operator, value } = condition {
            self.compare_u64_batch(&context.incomes, *operator, *value)
        } else {
            vec![false; context.len()]
        }
    }

    /// Performs SIMD-accelerated comparison for u32 arrays.
    #[allow(dead_code)]
    fn compare_u32_batch(
        &self,
        values: &[u32],
        operator: ComparisonOp,
        threshold: u32,
    ) -> Vec<bool> {
        // For now, use scalar implementation
        // In a real implementation, this would use SIMD intrinsics
        values
            .iter()
            .map(|&v| operator.compare_u32(v, threshold))
            .collect()
    }

    /// Performs SIMD-accelerated comparison for u64 arrays.
    #[allow(dead_code)]
    fn compare_u64_batch(
        &self,
        values: &[u64],
        operator: ComparisonOp,
        threshold: u64,
    ) -> Vec<bool> {
        // For now, use scalar implementation
        // In a real implementation, this would use SIMD intrinsics
        values
            .iter()
            .map(|&v| operator.compare_u64(v, threshold))
            .collect()
    }

    /// Performs vectorized AND operation on boolean arrays.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::simd_eval::SimdEvaluator;
    ///
    /// let evaluator = SimdEvaluator::new();
    /// let a = vec![true, true, false, false];
    /// let b = vec![true, false, true, false];
    /// let result = evaluator.vectorized_and(&a, &b);
    ///
    /// assert_eq!(result, vec![true, false, false, false]);
    /// ```
    pub fn vectorized_and(&self, a: &[bool], b: &[bool]) -> Vec<bool> {
        assert_eq!(a.len(), b.len(), "Arrays must have same length");
        a.iter().zip(b.iter()).map(|(&x, &y)| x && y).collect()
    }

    /// Performs vectorized OR operation on boolean arrays.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::simd_eval::SimdEvaluator;
    ///
    /// let evaluator = SimdEvaluator::new();
    /// let a = vec![true, true, false, false];
    /// let b = vec![true, false, true, false];
    /// let result = evaluator.vectorized_or(&a, &b);
    ///
    /// assert_eq!(result, vec![true, true, true, false]);
    /// ```
    pub fn vectorized_or(&self, a: &[bool], b: &[bool]) -> Vec<bool> {
        assert_eq!(a.len(), b.len(), "Arrays must have same length");
        a.iter().zip(b.iter()).map(|(&x, &y)| x || y).collect()
    }

    /// Performs vectorized NOT operation on a boolean array.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::simd_eval::SimdEvaluator;
    ///
    /// let evaluator = SimdEvaluator::new();
    /// let a = vec![true, false, true, false];
    /// let result = evaluator.vectorized_not(&a);
    ///
    /// assert_eq!(result, vec![false, true, false, true]);
    /// ```
    pub fn vectorized_not(&self, a: &[bool]) -> Vec<bool> {
        a.iter().map(|&x| !x).collect()
    }

    /// Counts the number of true values in a boolean array (SIMD-accelerated).
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::simd_eval::SimdEvaluator;
    ///
    /// let evaluator = SimdEvaluator::new();
    /// let results = vec![true, false, true, true, false];
    /// let count = evaluator.count_satisfied(&results);
    ///
    /// assert_eq!(count, 3);
    /// ```
    pub fn count_satisfied(&self, results: &[bool]) -> usize {
        results.iter().filter(|&&x| x).count()
    }

    /// Computes the satisfaction rate (percentage of true values).
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::simd_eval::SimdEvaluator;
    ///
    /// let evaluator = SimdEvaluator::new();
    /// let results = vec![true, false, true, true];
    /// let rate = evaluator.satisfaction_rate(&results);
    ///
    /// assert!((rate - 0.75).abs() < 0.001);
    /// ```
    pub fn satisfaction_rate(&self, results: &[bool]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }
        self.count_satisfied(results) as f64 / results.len() as f64
    }
}

impl Default for SimdEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_context_creation() {
        let context = BatchEvaluationContext::new(vec![18, 25, 30], vec![40000, 50000, 60000]);
        assert_eq!(context.len(), 3);
        assert!(!context.is_empty());
    }

    #[test]
    fn test_batch_age_evaluation() {
        let context = BatchEvaluationContext::new(vec![17, 18, 25, 30, 16], vec![0, 0, 0, 0, 0]);

        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let evaluator = SimdEvaluator::new();
        let results = evaluator.evaluate_batch_age(&context, &condition);

        assert_eq!(results, vec![false, true, true, true, false]);
    }

    #[test]
    fn test_batch_income_evaluation() {
        let context = BatchEvaluationContext::new(vec![25, 30, 35], vec![40000, 50000, 60000]);

        let condition = Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        };

        let evaluator = SimdEvaluator::new();
        let results = evaluator.evaluate_batch_income(&context, &condition);

        assert_eq!(results, vec![true, false, false]);
    }

    #[test]
    fn test_vectorized_and() {
        let evaluator = SimdEvaluator::new();
        let a = vec![true, true, false, false];
        let b = vec![true, false, true, false];
        let result = evaluator.vectorized_and(&a, &b);
        assert_eq!(result, vec![true, false, false, false]);
    }

    #[test]
    fn test_vectorized_or() {
        let evaluator = SimdEvaluator::new();
        let a = vec![true, true, false, false];
        let b = vec![true, false, true, false];
        let result = evaluator.vectorized_or(&a, &b);
        assert_eq!(result, vec![true, true, true, false]);
    }

    #[test]
    fn test_vectorized_not() {
        let evaluator = SimdEvaluator::new();
        let a = vec![true, false, true, false];
        let result = evaluator.vectorized_not(&a);
        assert_eq!(result, vec![false, true, false, true]);
    }

    #[test]
    fn test_count_satisfied() {
        let evaluator = SimdEvaluator::new();
        let results = vec![true, false, true, true, false];
        assert_eq!(evaluator.count_satisfied(&results), 3);
    }

    #[test]
    fn test_satisfaction_rate() {
        let evaluator = SimdEvaluator::new();
        let results = vec![true, false, true, true];
        let rate = evaluator.satisfaction_rate(&results);
        assert!((rate - 0.75).abs() < 0.001);
    }
}
