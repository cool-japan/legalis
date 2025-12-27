//! Parallel diff computation for batch operations.
//!
//! This module provides parallel implementations of diff operations using rayon
//! for improved performance when processing multiple statutes.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::parallel::parallel_diff_pairs;
//!
//! let old1 = Statute::new("law1", "Title 1", Effect::new(EffectType::Grant, "Benefit 1"));
//! let new1 = Statute::new("law1", "Title 1 Updated", Effect::new(EffectType::Grant, "Benefit 1"));
//!
//! let old2 = Statute::new("law2", "Title 2", Effect::new(EffectType::Grant, "Benefit 2"));
//! let new2 = Statute::new("law2", "Title 2 Updated", Effect::new(EffectType::Grant, "Benefit 2"));
//!
//! let pairs = vec![(old1, new1), (old2, new2)];
//! let results = parallel_diff_pairs(&pairs);
//!
//! assert_eq!(results.len(), 2);
//! ```

use crate::{DiffResult, StatuteDiff, diff};
use legalis_core::Statute;
use rayon::prelude::*;

/// Computes diffs for multiple statute pairs in parallel.
///
/// This is significantly faster than sequential processing when you have
/// many statute pairs to compare.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::parallel::parallel_diff_pairs;
///
/// let pairs = vec![
///     (
///         Statute::new("law1", "Old Title 1", Effect::new(EffectType::Grant, "Benefit")),
///         Statute::new("law1", "New Title 1", Effect::new(EffectType::Grant, "Benefit")),
///     ),
///     (
///         Statute::new("law2", "Old Title 2", Effect::new(EffectType::Grant, "Benefit")),
///         Statute::new("law2", "New Title 2", Effect::new(EffectType::Grant, "Benefit")),
///     ),
/// ];
///
/// let results = parallel_diff_pairs(&pairs);
/// assert_eq!(results.len(), 2);
/// ```
pub fn parallel_diff_pairs(pairs: &[(Statute, Statute)]) -> Vec<DiffResult<StatuteDiff>> {
    pairs.par_iter().map(|(old, new)| diff(old, new)).collect()
}

/// Computes diffs for a sequence of statute versions in parallel.
///
/// Given a vector of statute versions, computes all adjacent diffs in parallel.
/// This is useful for analyzing amendment histories.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::parallel::parallel_diff_sequence;
///
/// let versions = vec![
///     Statute::new("law", "Version 1", Effect::new(EffectType::Grant, "Benefit")),
///     Statute::new("law", "Version 2", Effect::new(EffectType::Grant, "Benefit")),
///     Statute::new("law", "Version 3", Effect::new(EffectType::Grant, "Benefit")),
/// ];
///
/// let diffs = parallel_diff_sequence(&versions);
/// assert_eq!(diffs.len(), 2); // v1->v2, v2->v3
/// ```
pub fn parallel_diff_sequence(versions: &[Statute]) -> Vec<DiffResult<StatuteDiff>> {
    if versions.len() < 2 {
        return Vec::new();
    }

    (0..versions.len() - 1)
        .into_par_iter()
        .map(|i| diff(&versions[i], &versions[i + 1]))
        .collect()
}

/// Computes diffs for multiple sequences in parallel.
///
/// This is useful when you have multiple statute families and want to
/// compute their amendment histories in parallel.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::parallel::parallel_diff_multiple_sequences;
///
/// let sequence1 = vec![
///     Statute::new("law1", "V1", Effect::new(EffectType::Grant, "Benefit")),
///     Statute::new("law1", "V2", Effect::new(EffectType::Grant, "Benefit")),
/// ];
///
/// let sequence2 = vec![
///     Statute::new("law2", "V1", Effect::new(EffectType::Grant, "Benefit")),
///     Statute::new("law2", "V2", Effect::new(EffectType::Grant, "Benefit")),
/// ];
///
/// let sequences = vec![sequence1, sequence2];
/// let results = parallel_diff_multiple_sequences(&sequences);
///
/// assert_eq!(results.len(), 2);
/// ```
pub fn parallel_diff_multiple_sequences(
    sequences: &[Vec<Statute>],
) -> Vec<Vec<DiffResult<StatuteDiff>>> {
    sequences
        .par_iter()
        .map(|seq| parallel_diff_sequence(seq))
        .collect()
}

/// Filters successful diffs from a batch result.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::parallel::{parallel_diff_pairs, filter_successful};
///
/// let pairs = vec![
///     (
///         Statute::new("law1", "Old", Effect::new(EffectType::Grant, "Benefit")),
///         Statute::new("law1", "New", Effect::new(EffectType::Grant, "Benefit")),
///     ),
/// ];
///
/// let results = parallel_diff_pairs(&pairs);
/// let successful = filter_successful(results);
///
/// assert_eq!(successful.len(), 1);
/// ```
pub fn filter_successful(results: Vec<DiffResult<StatuteDiff>>) -> Vec<StatuteDiff> {
    results.into_iter().filter_map(|r| r.ok()).collect()
}

/// Collects errors from a batch diff operation.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::parallel::{parallel_diff_pairs, collect_errors};
///
/// // Create a pair with mismatched IDs (will error)
/// let pairs = vec![
///     (
///         Statute::new("law1", "Old", Effect::new(EffectType::Grant, "Benefit")),
///         Statute::new("law2", "New", Effect::new(EffectType::Grant, "Benefit")), // Different ID!
///     ),
/// ];
///
/// let results = parallel_diff_pairs(&pairs);
/// let errors = collect_errors(results);
///
/// assert_eq!(errors.len(), 1);
/// ```
pub fn collect_errors(results: Vec<DiffResult<StatuteDiff>>) -> Vec<crate::DiffError> {
    results.into_iter().filter_map(|r| r.err()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn create_test_statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Test effect"))
    }

    #[test]
    fn test_parallel_diff_pairs() {
        let pairs = vec![
            (
                create_test_statute("law1", "Old Title 1"),
                create_test_statute("law1", "New Title 1"),
            ),
            (
                create_test_statute("law2", "Old Title 2"),
                create_test_statute("law2", "New Title 2"),
            ),
            (
                create_test_statute("law3", "Old Title 3"),
                create_test_statute("law3", "New Title 3"),
            ),
        ];

        let results = parallel_diff_pairs(&pairs);
        assert_eq!(results.len(), 3);

        for result in results {
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_parallel_diff_sequence() {
        let versions = vec![
            create_test_statute("law", "Version 1"),
            create_test_statute("law", "Version 2"),
            create_test_statute("law", "Version 3"),
            create_test_statute("law", "Version 4"),
        ];

        let diffs = parallel_diff_sequence(&versions);
        assert_eq!(diffs.len(), 3); // v1->v2, v2->v3, v3->v4

        for diff in diffs {
            assert!(diff.is_ok());
        }
    }

    #[test]
    fn test_parallel_diff_sequence_empty() {
        let versions: Vec<Statute> = vec![];
        let diffs = parallel_diff_sequence(&versions);
        assert_eq!(diffs.len(), 0);
    }

    #[test]
    fn test_parallel_diff_sequence_single() {
        let versions = vec![create_test_statute("law", "Version 1")];
        let diffs = parallel_diff_sequence(&versions);
        assert_eq!(diffs.len(), 0);
    }

    #[test]
    fn test_parallel_diff_multiple_sequences() {
        let seq1 = vec![
            create_test_statute("law1", "V1"),
            create_test_statute("law1", "V2"),
            create_test_statute("law1", "V3"),
        ];

        let seq2 = vec![
            create_test_statute("law2", "V1"),
            create_test_statute("law2", "V2"),
        ];

        let sequences = vec![seq1, seq2];
        let results = parallel_diff_multiple_sequences(&sequences);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].len(), 2); // 2 diffs for 3 versions
        assert_eq!(results[1].len(), 1); // 1 diff for 2 versions
    }

    #[test]
    fn test_filter_successful() {
        let pairs = vec![
            (
                create_test_statute("law1", "Old"),
                create_test_statute("law1", "New"),
            ),
            (
                create_test_statute("law2", "Old"),
                create_test_statute("law3", "New"), // ID mismatch - will error
            ),
        ];

        let results = parallel_diff_pairs(&pairs);
        let successful = filter_successful(results);

        assert_eq!(successful.len(), 1);
        assert_eq!(successful[0].statute_id, "law1");
    }

    #[test]
    fn test_collect_errors() {
        let pairs = vec![
            (
                create_test_statute("law1", "Old"),
                create_test_statute("law2", "New"), // ID mismatch - will error
            ),
            (
                create_test_statute("law3", "Old"),
                create_test_statute("law4", "New"), // ID mismatch - will error
            ),
        ];

        let results = parallel_diff_pairs(&pairs);
        let errors = collect_errors(results);

        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_parallel_performance_large_batch() {
        // Create a large batch to test parallel performance
        let pairs: Vec<_> = (0..100)
            .map(|i| {
                let id = format!("law{}", i);
                (
                    create_test_statute(&id, "Old Title"),
                    create_test_statute(&id, "New Title"),
                )
            })
            .collect();

        let results = parallel_diff_pairs(&pairs);
        assert_eq!(results.len(), 100);

        let successful = filter_successful(results);
        assert_eq!(successful.len(), 100);
    }

    #[test]
    fn test_parallel_with_complex_changes() {
        let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 65,
            });

        let new = Statute::new(
            "law",
            "New Title",
            Effect::new(EffectType::Grant, "Benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 60,
        })
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let pairs = vec![(old, new)];
        let results = parallel_diff_pairs(&pairs);

        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());

        let diff = results[0].as_ref().unwrap();
        assert!(diff.changes.len() >= 2); // Title change + precondition changes
    }
}
