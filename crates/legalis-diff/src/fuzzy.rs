//! Fuzzy matching for statute changes.
//!
//! This module provides fuzzy matching capabilities to detect similar
//! changes even when they're not identical. Useful for finding patterns
//! and related modifications across different statutes.
//!
//! # Example
//!
//! ```
//! use legalis_diff::fuzzy::{similarity_score, are_similar};
//!
//! let text1 = "Age must be greater than or equal to 65";
//! let text2 = "Age must be greater than or equal to 60";
//!
//! let score = similarity_score(text1, text2);
//! assert!(score > 0.8); // Very similar texts
//!
//! assert!(are_similar(text1, text2, 0.7));
//! ```

use crate::{Change, StatuteDiff};
use std::collections::HashMap;

#[cfg(test)]
use crate::ChangeType;

/// Calculates the Levenshtein distance between two strings.
///
/// The Levenshtein distance is the minimum number of single-character
/// edits (insertions, deletions, or substitutions) required to change
/// one string into another.
#[allow(clippy::needless_range_loop)]
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };

            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1, // deletion
                    matrix[i][j - 1] + 1, // insertion
                ),
                matrix[i - 1][j - 1] + cost, // substitution
            );
        }
    }

    matrix[len1][len2]
}

/// Calculates a normalized similarity score between two strings.
///
/// Returns a value between 0.0 (completely different) and 1.0 (identical).
/// The score is computed based on the Levenshtein distance normalized
/// by the length of the longer string.
///
/// # Examples
///
/// ```
/// use legalis_diff::fuzzy::similarity_score;
///
/// let score = similarity_score("hello", "hello");
/// assert_eq!(score, 1.0);
///
/// let score = similarity_score("hello", "hallo");
/// assert!(score > 0.7 && score < 1.0);
///
/// let score = similarity_score("hello", "world");
/// assert!(score < 0.5);
/// ```
pub fn similarity_score(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }

    let distance = levenshtein_distance(s1, s2);
    let max_len = std::cmp::max(s1.chars().count(), s2.chars().count());

    if max_len == 0 {
        return 1.0;
    }

    1.0 - (distance as f64 / max_len as f64)
}

/// Checks if two strings are similar based on a threshold.
///
/// # Examples
///
/// ```
/// use legalis_diff::fuzzy::are_similar;
///
/// assert!(are_similar("hello world", "hello world", 0.9));
/// assert!(are_similar("hello world", "hallo world", 0.8));
/// assert!(!are_similar("hello world", "goodbye", 0.8));
/// ```
pub fn are_similar(s1: &str, s2: &str, threshold: f64) -> bool {
    similarity_score(s1, s2) >= threshold
}

/// Represents a fuzzy match between two changes.
#[derive(Debug, Clone)]
pub struct FuzzyMatch {
    /// The first change.
    pub change1: Change,
    /// The second change.
    pub change2: Change,
    /// Similarity score (0.0 to 1.0).
    pub similarity: f64,
    /// Explanation of the match.
    pub explanation: String,
}

/// Finds similar changes between two statute diffs using fuzzy matching.
///
/// This is useful for detecting related changes across different statutes
/// or versions.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, fuzzy::find_similar_changes};
///
/// let old1 = Statute::new("law1", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new1 = old1.clone();
/// new1.title = "New Title".to_string();
///
/// let old2 = Statute::new("law2", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new2 = old2.clone();
/// new2.title = "New Title".to_string();
///
/// let diff1 = diff(&old1, &new1).unwrap();
/// let diff2 = diff(&old2, &new2).unwrap();
///
/// let matches = find_similar_changes(&diff1, &diff2, 0.8);
/// assert!(!matches.is_empty());
/// ```
pub fn find_similar_changes(
    diff1: &StatuteDiff,
    diff2: &StatuteDiff,
    threshold: f64,
) -> Vec<FuzzyMatch> {
    let mut matches = Vec::new();

    for change1 in &diff1.changes {
        for change2 in &diff2.changes {
            if change1.change_type == change2.change_type {
                let desc_similarity = similarity_score(&change1.description, &change2.description);

                if desc_similarity >= threshold {
                    let explanation = format!(
                        "Similar {} changes detected (similarity: {:.2})",
                        format!("{:?}", change1.change_type).to_lowercase(),
                        desc_similarity
                    );

                    matches.push(FuzzyMatch {
                        change1: change1.clone(),
                        change2: change2.clone(),
                        similarity: desc_similarity,
                        explanation,
                    });
                }
            }
        }
    }

    matches
}

/// Groups similar changes across multiple diffs.
///
/// Returns a map where each key is a representative change description,
/// and the value is a vector of similar changes from different diffs.
pub fn group_similar_changes(
    diffs: &[StatuteDiff],
    threshold: f64,
) -> HashMap<String, Vec<Change>> {
    let mut groups: HashMap<String, Vec<Change>> = HashMap::new();

    for diff in diffs {
        for change in &diff.changes {
            let mut found_group = false;

            for (key, group) in groups.iter_mut() {
                if are_similar(key, &change.description, threshold) {
                    group.push(change.clone());
                    found_group = true;
                    break;
                }
            }

            if !found_group {
                groups.insert(change.description.clone(), vec![change.clone()]);
            }
        }
    }

    groups
}

/// Finds the most similar change in a diff to a given change.
pub fn find_most_similar_change(target: &Change, diff: &StatuteDiff) -> Option<(Change, f64)> {
    let mut best_match: Option<(Change, f64)> = None;

    for change in &diff.changes {
        if change.change_type == target.change_type {
            let score = similarity_score(&target.description, &change.description);

            if let Some((_, best_score)) = &best_match {
                if score > *best_score {
                    best_match = Some((change.clone(), score));
                }
            } else {
                best_match = Some((change.clone(), score));
            }
        }
    }

    best_match
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChangeTarget, diff};
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("hello", "hallo"), 1);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
    }

    #[test]
    fn test_similarity_score() {
        assert_eq!(similarity_score("hello", "hello"), 1.0);
        assert!(similarity_score("hello", "hallo") > 0.7); // 1 char difference out of 5 = 0.8
        assert!(similarity_score("hello", "world") < 0.5);
    }

    #[test]
    fn test_are_similar() {
        assert!(are_similar("hello", "hello", 1.0));
        assert!(are_similar("hello", "hallo", 0.8));
        assert!(!are_similar("hello", "world", 0.8));
    }

    #[test]
    fn test_find_similar_changes() {
        let old1 = Statute::new(
            "law1",
            "Original Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let mut new1 = old1.clone();
        new1.title = "Modified Title".to_string();

        let old2 = Statute::new(
            "law2",
            "Original Title",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let mut new2 = old2.clone();
        new2.title = "Modified Title".to_string();

        let diff1 = diff(&old1, &new1).unwrap();
        let diff2 = diff(&old2, &new2).unwrap();

        let matches = find_similar_changes(&diff1, &diff2, 0.8);
        assert!(!matches.is_empty());
        assert!(matches[0].similarity > 0.9);
    }

    #[test]
    fn test_group_similar_changes() {
        let old1 = Statute::new("law1", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let mut new1 = old1.clone();
        new1.title = "New Title".to_string();

        let old2 = Statute::new("law2", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let mut new2 = old2.clone();
        new2.title = "New Title".to_string();

        let diff1 = diff(&old1, &new1).unwrap();
        let diff2 = diff(&old2, &new2).unwrap();

        let groups = group_similar_changes(&[diff1, diff2], 0.9);
        assert!(!groups.is_empty());
    }

    #[test]
    fn test_find_most_similar_change() {
        let old = Statute::new(
            "law",
            "Old Title",
            Effect::new(EffectType::Grant, "Benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        });

        let mut new = old.clone();
        new.title = "New Title".to_string();
        new.preconditions[0] = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 60,
        };

        let diff_result = diff(&old, &new).unwrap();

        let target_change = Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Title,
            description: "Title was changed".to_string(),
            old_value: Some("Old Title".to_string()),
            new_value: Some("New Title".to_string()),
        };

        let result = find_most_similar_change(&target_change, &diff_result);
        assert!(result.is_some());
        let (matched_change, score) = result.unwrap();
        assert!(score > 0.5);
        assert!(matches!(matched_change.target, ChangeTarget::Title));
    }
}
