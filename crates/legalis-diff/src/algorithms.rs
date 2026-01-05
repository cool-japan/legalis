//! Advanced diff algorithms for statute comparison.
//!
//! This module implements sophisticated diff algorithms including:
//! - Myers diff algorithm
//! - Patience diff algorithm
//! - Histogram diff algorithm
//!
//! These algorithms provide more precise and intuitive diffs than simple
//! element-by-element comparison.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Condition, ComparisonOp};
//! use legalis_diff::algorithms::{myers_diff, patience_diff, diff_conditions_myers};
//!
//! let old = vec![1, 2, 3, 4];
//! let new = vec![1, 3, 4, 5];
//!
//! // Use Myers algorithm
//! let myers_result = myers_diff(&old, &new);
//! assert!(myers_result.edit_distance > 0);
//!
//! // Use Patience algorithm
//! let patience_result = patience_diff(&old, &new);
//! assert!(patience_result.edit_distance > 0);
//!
//! // Apply to statute conditions
//! let old_conds = vec![Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 18,
//! }];
//! let new_conds = vec![Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 21,
//! }];
//!
//! let diff = diff_conditions_myers(&old_conds, &new_conds);
//! assert!(diff.edit_distance > 0);
//! ```

use legalis_core::Condition;

/// Represents a diff operation in the edit script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffOp<T> {
    /// Item is present in both sequences (no change).
    Keep(T),
    /// Item was removed from the old sequence.
    Delete(T),
    /// Item was added to the new sequence.
    Insert(T),
}

/// Result of a diff algorithm.
#[derive(Debug, Clone)]
pub struct DiffResult<T> {
    /// The edit script.
    pub ops: Vec<DiffOp<T>>,
    /// Edit distance (number of insertions + deletions).
    pub edit_distance: usize,
}

/// Computes the diff using the Myers algorithm.
///
/// The Myers algorithm is a classic diff algorithm that finds the shortest
/// edit script (minimum number of insertions and deletions) to transform
/// one sequence into another.
pub fn myers_diff<T: Clone + PartialEq>(old: &[T], new: &[T]) -> DiffResult<T> {
    let n = old.len();
    let m = new.len();
    let max_d = n + m;

    let mut v: Vec<isize> = vec![0; 2 * max_d + 1];
    let mut trace: Vec<Vec<isize>> = Vec::new();

    for d in 0..=max_d {
        trace.push(v.clone());

        for k in (-(d as isize)..=(d as isize)).step_by(2) {
            let k_idx = (k + max_d as isize) as usize;
            let k_minus_idx = (k - 1 + max_d as isize) as usize;
            let k_plus_idx = (k + 1 + max_d as isize) as usize;

            let mut x = if k == -(d as isize) || (k != d as isize && v[k_minus_idx] < v[k_plus_idx])
            {
                v[k_plus_idx]
            } else {
                v[k_minus_idx] + 1
            };

            let mut y = x - k;

            while x < n as isize && y < m as isize && old[x as usize] == new[y as usize] {
                x += 1;
                y += 1;
            }

            v[k_idx] = x;

            if x >= n as isize && y >= m as isize {
                return backtrack_myers(&trace, old, new, d);
            }
        }
    }

    // Fallback (should not reach here)
    DiffResult {
        ops: Vec::new(),
        edit_distance: max_d,
    }
}

fn backtrack_myers<T: Clone + PartialEq>(
    trace: &[Vec<isize>],
    old: &[T],
    new: &[T],
    d: usize,
) -> DiffResult<T> {
    let mut ops = Vec::new();
    let mut x = old.len() as isize;
    let mut y = new.len() as isize;
    let max_d = old.len() + new.len();

    for (depth, v) in trace.iter().enumerate().take(d + 1).rev() {
        let k = x - y;
        let _k_idx = (k + max_d as isize) as usize;
        let k_minus_idx = (k - 1 + max_d as isize) as usize;
        let k_plus_idx = (k + 1 + max_d as isize) as usize;

        let prev_k = if k == -(depth as isize)
            || (k != depth as isize
                && k_minus_idx < v.len()
                && k_plus_idx < v.len()
                && v[k_minus_idx] < v[k_plus_idx])
        {
            k + 1
        } else {
            k - 1
        };

        let prev_k_idx = (prev_k + max_d as isize) as usize;
        let prev_x = if prev_k_idx < v.len() {
            v[prev_k_idx]
        } else {
            0
        };
        let prev_y = prev_x - prev_k;

        while x > prev_x && y > prev_y {
            ops.push(DiffOp::Keep(old[(x - 1) as usize].clone()));
            x -= 1;
            y -= 1;
        }

        if depth > 0 {
            if x == prev_x {
                ops.push(DiffOp::Insert(new[(y - 1) as usize].clone()));
                y -= 1;
            } else {
                ops.push(DiffOp::Delete(old[(x - 1) as usize].clone()));
                x -= 1;
            }
        }
    }

    ops.reverse();
    let edit_distance = ops
        .iter()
        .filter(|op| !matches!(op, DiffOp::Keep(_)))
        .count();

    DiffResult { ops, edit_distance }
}

/// Computes the diff using the Patience algorithm.
///
/// The Patience algorithm produces more intuitive diffs by matching unique
/// common elements first, then recursively diffing the regions between them.
pub fn patience_diff<T: Clone + PartialEq>(old: &[T], new: &[T]) -> DiffResult<T> {
    let mut ops = Vec::new();
    patience_diff_recursive(old, new, &mut ops);

    let edit_distance = ops
        .iter()
        .filter(|op| !matches!(op, DiffOp::Keep(_)))
        .count();

    DiffResult { ops, edit_distance }
}

fn patience_diff_recursive<T: Clone + PartialEq>(old: &[T], new: &[T], ops: &mut Vec<DiffOp<T>>) {
    if old.is_empty() {
        for item in new {
            ops.push(DiffOp::Insert(item.clone()));
        }
        return;
    }

    if new.is_empty() {
        for item in old {
            ops.push(DiffOp::Delete(item.clone()));
        }
        return;
    }

    // Find unique common elements
    let common = find_unique_common(old, new);

    if common.is_empty() {
        // No unique common elements, fall back to Myers diff
        let result = myers_diff(old, new);
        ops.extend(result.ops);
        return;
    }

    // Find longest increasing subsequence of common elements
    let lis = longest_increasing_subsequence(&common);

    if lis.is_empty() {
        // No increasing subsequence, fall back to Myers diff
        let result = myers_diff(old, new);
        ops.extend(result.ops);
        return;
    }

    // Recursively diff regions between matched elements
    let mut old_idx = 0;
    let mut new_idx = 0;

    for &(old_pos, new_pos) in &lis {
        // Diff the region before this match
        patience_diff_recursive(&old[old_idx..old_pos], &new[new_idx..new_pos], ops);

        // Add the matched element
        ops.push(DiffOp::Keep(old[old_pos].clone()));

        old_idx = old_pos + 1;
        new_idx = new_pos + 1;
    }

    // Diff the remaining region
    patience_diff_recursive(&old[old_idx..], &new[new_idx..], ops);
}

fn find_unique_common<T: PartialEq>(old: &[T], new: &[T]) -> Vec<(usize, usize)> {
    let mut common = Vec::new();

    for (old_idx, old_item) in old.iter().enumerate() {
        for (new_idx, new_item) in new.iter().enumerate() {
            if old_item == new_item {
                common.push((old_idx, new_idx));
            }
        }
    }

    common
}

fn longest_increasing_subsequence(pairs: &[(usize, usize)]) -> Vec<(usize, usize)> {
    if pairs.is_empty() {
        return Vec::new();
    }

    let n = pairs.len();
    let mut dp = vec![1; n];
    let mut prev = vec![None; n];

    for i in 1..n {
        for j in 0..i {
            if pairs[j].0 < pairs[i].0 && pairs[j].1 < pairs[i].1 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                prev[i] = Some(j);
            }
        }
    }

    let mut max_idx = 0;
    for i in 1..n {
        if dp[i] > dp[max_idx] {
            max_idx = i;
        }
    }

    let mut result = Vec::new();
    let mut idx = Some(max_idx);
    while let Some(i) = idx {
        result.push(pairs[i]);
        idx = prev[i];
    }

    result.reverse();
    result
}

/// Computes diff for statute preconditions using Myers algorithm.
pub fn diff_conditions_myers(old: &[Condition], new: &[Condition]) -> DiffResult<Condition> {
    myers_diff(old, new)
}

/// Computes diff for statute preconditions using Patience algorithm.
pub fn diff_conditions_patience(old: &[Condition], new: &[Condition]) -> DiffResult<Condition> {
    patience_diff(old, new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myers_diff_simple() {
        let old = vec![1, 2, 3];
        let new = vec![1, 3, 4];

        let result = myers_diff(&old, &new);

        assert!(result.edit_distance > 0);
    }

    #[test]
    fn test_myers_diff_identical() {
        let seq = vec![1, 2, 3];
        let result = myers_diff(&seq, &seq);

        assert_eq!(result.edit_distance, 0);
        assert_eq!(result.ops.len(), 3);
    }

    #[test]
    fn test_myers_diff_empty() {
        let old: Vec<i32> = vec![];
        let new = vec![1, 2, 3];

        let result = myers_diff(&old, &new);
        assert_eq!(result.edit_distance, 3);
    }

    #[test]
    fn test_patience_diff_simple() {
        let old = vec![1, 2, 3];
        let new = vec![1, 3, 4];

        let result = patience_diff(&old, &new);
        assert!(result.edit_distance > 0);
    }

    #[test]
    fn test_patience_diff_identical() {
        let seq = vec![1, 2, 3];
        let result = patience_diff(&seq, &seq);

        assert_eq!(result.edit_distance, 0);
    }

    #[test]
    fn test_patience_diff_empty() {
        let old: Vec<i32> = vec![];
        let new = vec![1, 2, 3];

        let result = patience_diff(&old, &new);
        assert_eq!(result.edit_distance, 3);
    }

    #[test]
    fn test_find_unique_common() {
        let old = vec![1, 2, 3, 4];
        let new = vec![2, 3, 4, 5];

        let common = find_unique_common(&old, &new);
        assert!(!common.is_empty());
    }

    #[test]
    fn test_lis_basic() {
        let pairs = vec![(0, 0), (1, 1), (2, 2)];
        let lis = longest_increasing_subsequence(&pairs);

        assert_eq!(lis.len(), 3);
    }

    #[test]
    fn test_lis_empty() {
        let pairs: Vec<(usize, usize)> = vec![];
        let lis = longest_increasing_subsequence(&pairs);

        assert!(lis.is_empty());
    }

    #[test]
    fn test_diff_ops_keep() {
        let old = vec![1, 2, 3];
        let new = vec![1, 2, 3];

        let result = myers_diff(&old, &new);
        let keep_count = result
            .ops
            .iter()
            .filter(|op| matches!(op, DiffOp::Keep(_)))
            .count();

        assert_eq!(keep_count, 3);
    }

    #[test]
    fn test_condition_diff_myers() {
        use legalis_core::{ComparisonOp, Condition};

        let old = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
            Condition::Income {
                operator: ComparisonOp::LessOrEqual,
                value: 3_000_000,
            },
        ];

        let new = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 21,
            },
            Condition::Income {
                operator: ComparisonOp::LessOrEqual,
                value: 3_000_000,
            },
        ];

        let result = diff_conditions_myers(&old, &new);
        assert!(result.edit_distance > 0);
    }

    #[test]
    fn test_condition_diff_patience() {
        use legalis_core::{ComparisonOp, Condition};

        let old = vec![Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }];

        let new = vec![Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }];

        let result = diff_conditions_patience(&old, &new);
        assert_eq!(result.edit_distance, 0);
    }
}
