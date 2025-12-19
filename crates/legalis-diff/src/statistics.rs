//! Statistical analysis of statute changes and diff patterns.
//!
//! This module provides statistical tools for analyzing change patterns,
//! frequencies, and trends across single or multiple statute diffs.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::{diff, statistics::{compute_statistics, aggregate_statistics}};
//!
//! let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
//! let mut new1 = old.clone();
//! new1.title = "Updated Title".to_string();
//!
//! let diff1 = diff(&old, &new1).unwrap();
//! let stats = compute_statistics(&diff1);
//! assert_eq!(stats.total_changes, 1);
//!
//! // Aggregate across multiple diffs
//! let mut new2 = old.clone();
//! new2.title = "Another Title".to_string();
//! let diff2 = diff(&old, &new2).unwrap();
//!
//! let agg = aggregate_statistics(&[diff1, diff2]);
//! assert_eq!(agg.diff_count, 2);
//! ```

use crate::{ChangeTarget, ChangeType, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Statistical summary of changes in a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffStatistics {
    /// Total number of changes.
    pub total_changes: usize,
    /// Changes by type.
    pub changes_by_type: HashMap<ChangeType, usize>,
    /// Changes by target category.
    pub changes_by_target: HashMap<String, usize>,
    /// Severity distribution.
    pub severity: Severity,
    /// Average change impact score (0.0 to 1.0).
    pub impact_score: f64,
    /// Change density (changes per precondition).
    pub change_density: f64,
}

/// Aggregated statistics across multiple diffs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateStatistics {
    /// Number of diffs analyzed.
    pub diff_count: usize,
    /// Total changes across all diffs.
    pub total_changes: usize,
    /// Average changes per diff.
    pub avg_changes_per_diff: f64,
    /// Most common change type.
    pub most_common_change_type: Option<ChangeType>,
    /// Most common target type.
    pub most_common_target: Option<String>,
    /// Distribution of severities.
    pub severity_distribution: HashMap<Severity, usize>,
    /// Change patterns detected.
    pub patterns: Vec<ChangePattern>,
}

/// A detected pattern in changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePattern {
    /// Pattern description.
    pub description: String,
    /// Frequency of this pattern.
    pub frequency: usize,
    /// Pattern confidence (0.0 to 1.0).
    pub confidence: f64,
}

/// Computes statistics for a single diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, statistics::compute_statistics};
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let stats = compute_statistics(&diff_result);
///
/// assert_eq!(stats.total_changes, 1);
/// ```
pub fn compute_statistics(diff: &StatuteDiff) -> DiffStatistics {
    let total_changes = diff.changes.len();

    let mut changes_by_type: HashMap<ChangeType, usize> = HashMap::new();
    let mut changes_by_target: HashMap<String, usize> = HashMap::new();

    for change in &diff.changes {
        *changes_by_type.entry(change.change_type).or_insert(0) += 1;

        let target_category = match &change.target {
            ChangeTarget::Title => "Title".to_string(),
            ChangeTarget::Precondition { .. } => "Precondition".to_string(),
            ChangeTarget::Effect => "Effect".to_string(),
            ChangeTarget::DiscretionLogic => "DiscretionLogic".to_string(),
            ChangeTarget::Metadata { .. } => "Metadata".to_string(),
        };
        *changes_by_target.entry(target_category).or_insert(0) += 1;
    }

    let impact_score = calculate_impact_score(diff);
    let change_density = if total_changes == 0 {
        0.0
    } else {
        total_changes as f64 / (total_changes + 1) as f64
    };

    DiffStatistics {
        total_changes,
        changes_by_type,
        changes_by_target,
        severity: diff.impact.severity,
        impact_score,
        change_density,
    }
}

/// Aggregates statistics across multiple diffs.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, statistics::aggregate_statistics};
///
/// let old = Statute::new("law", "Original", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new1 = old.clone();
/// new1.title = "Version 1".to_string();
/// let mut new2 = old.clone();
/// new2.title = "Version 2".to_string();
///
/// let diff1 = diff(&old, &new1).unwrap();
/// let diff2 = diff(&old, &new2).unwrap();
///
/// let agg = aggregate_statistics(&[diff1, diff2]);
/// assert_eq!(agg.diff_count, 2);
/// ```
pub fn aggregate_statistics(diffs: &[StatuteDiff]) -> AggregateStatistics {
    let diff_count = diffs.len();
    let total_changes: usize = diffs.iter().map(|d| d.changes.len()).sum();
    let avg_changes_per_diff = if diff_count == 0 {
        0.0
    } else {
        total_changes as f64 / diff_count as f64
    };

    let mut all_change_types: HashMap<ChangeType, usize> = HashMap::new();
    let mut all_targets: HashMap<String, usize> = HashMap::new();
    let mut severity_distribution: HashMap<Severity, usize> = HashMap::new();

    for diff in diffs {
        for change in &diff.changes {
            *all_change_types.entry(change.change_type).or_insert(0) += 1;

            let target_category = match &change.target {
                ChangeTarget::Title => "Title".to_string(),
                ChangeTarget::Precondition { .. } => "Precondition".to_string(),
                ChangeTarget::Effect => "Effect".to_string(),
                ChangeTarget::DiscretionLogic => "DiscretionLogic".to_string(),
                ChangeTarget::Metadata { .. } => "Metadata".to_string(),
            };
            *all_targets.entry(target_category).or_insert(0) += 1;
        }

        *severity_distribution
            .entry(diff.impact.severity)
            .or_insert(0) += 1;
    }

    let most_common_change_type = all_change_types
        .iter()
        .max_by_key(|&(_, &count)| count)
        .map(|(&ct, _)| ct);

    let most_common_target = all_targets
        .iter()
        .max_by_key(|&(_, &count)| count)
        .map(|(t, _)| t.clone());

    let patterns = detect_patterns(diffs);

    AggregateStatistics {
        diff_count,
        total_changes,
        avg_changes_per_diff,
        most_common_change_type,
        most_common_target,
        severity_distribution,
        patterns,
    }
}

/// Calculates an impact score for a diff (0.0 to 1.0).
fn calculate_impact_score(diff: &StatuteDiff) -> f64 {
    let mut score: f64 = 0.0;

    // Severity contributes to score
    score += match diff.impact.severity {
        Severity::None => 0.0,
        Severity::Minor => 0.1,
        Severity::Moderate => 0.3,
        Severity::Major => 0.6,
        Severity::Breaking => 1.0,
    };

    // Impact flags contribute
    if diff.impact.affects_eligibility {
        score += 0.2;
    }
    if diff.impact.affects_outcome {
        score += 0.3;
    }
    if diff.impact.discretion_changed {
        score += 0.2;
    }

    // Normalize to 0.0-1.0 range
    score.min(1.0)
}

/// Detects patterns across multiple diffs.
fn detect_patterns(diffs: &[StatuteDiff]) -> Vec<ChangePattern> {
    let mut patterns = Vec::new();

    // Pattern 1: Frequent precondition modifications
    let precondition_mods = diffs
        .iter()
        .flat_map(|d| &d.changes)
        .filter(|c| {
            matches!(c.target, ChangeTarget::Precondition { .. })
                && c.change_type == ChangeType::Modified
        })
        .count();

    if precondition_mods > diffs.len() / 2 {
        patterns.push(ChangePattern {
            description: "Frequent precondition modifications detected".to_string(),
            frequency: precondition_mods,
            confidence: precondition_mods as f64 / diffs.len() as f64,
        });
    }

    // Pattern 2: Consistent effect changes
    let effect_changes = diffs
        .iter()
        .flat_map(|d| &d.changes)
        .filter(|c| matches!(c.target, ChangeTarget::Effect))
        .count();

    if effect_changes > diffs.len() / 3 {
        patterns.push(ChangePattern {
            description: "Pattern of effect modifications".to_string(),
            frequency: effect_changes,
            confidence: effect_changes as f64 / diffs.len() as f64,
        });
    }

    // Pattern 3: Discretion being added
    let discretion_additions = diffs
        .iter()
        .flat_map(|d| &d.changes)
        .filter(|c| {
            matches!(c.target, ChangeTarget::DiscretionLogic) && c.change_type == ChangeType::Added
        })
        .count();

    if discretion_additions > diffs.len() / 4 {
        patterns.push(ChangePattern {
            description: "Trend of adding discretion requirements".to_string(),
            frequency: discretion_additions,
            confidence: discretion_additions as f64 / diffs.len() as f64,
        });
    }

    patterns
}

/// Generates a textual summary of statistics.
pub fn summarize_statistics(stats: &DiffStatistics) -> String {
    let mut summary = String::new();

    summary.push_str(&format!("Total Changes: {}\n", stats.total_changes));
    summary.push_str(&format!("Severity: {:?}\n", stats.severity));
    summary.push_str(&format!("Impact Score: {:.2}\n", stats.impact_score));
    summary.push_str(&format!("Change Density: {:.2}\n\n", stats.change_density));

    summary.push_str("Changes by Type:\n");
    for (change_type, count) in &stats.changes_by_type {
        summary.push_str(&format!("  {:?}: {}\n", change_type, count));
    }

    summary.push_str("\nChanges by Target:\n");
    for (target, count) in &stats.changes_by_target {
        summary.push_str(&format!("  {}: {}\n", target, count));
    }

    summary
}

/// Generates a summary of aggregate statistics.
pub fn summarize_aggregate(stats: &AggregateStatistics) -> String {
    let mut summary = String::new();

    summary.push_str(&format!("Analyzed {} diffs\n", stats.diff_count));
    summary.push_str(&format!("Total Changes: {}\n", stats.total_changes));
    summary.push_str(&format!(
        "Average Changes per Diff: {:.2}\n\n",
        stats.avg_changes_per_diff
    ));

    if let Some(ct) = stats.most_common_change_type {
        summary.push_str(&format!("Most Common Change Type: {:?}\n", ct));
    }
    if let Some(target) = &stats.most_common_target {
        summary.push_str(&format!("Most Common Target: {}\n", target));
    }

    summary.push_str("\nSeverity Distribution:\n");
    for (severity, count) in &stats.severity_distribution {
        summary.push_str(&format!("  {:?}: {}\n", severity, count));
    }

    if !stats.patterns.is_empty() {
        summary.push_str("\nDetected Patterns:\n");
        for pattern in &stats.patterns {
            summary.push_str(&format!(
                "  - {} (frequency: {}, confidence: {:.2})\n",
                pattern.description, pattern.frequency, pattern.confidence
            ));
        }
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn create_test_statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Test effect")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        )
    }

    #[test]
    fn test_compute_statistics() {
        let old = create_test_statute("test", "Old Title");
        let mut new = old.clone();
        new.title = "New Title".to_string();
        new.preconditions[0] = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        };

        let diff = crate::diff(&old, &new).unwrap();
        let stats = compute_statistics(&diff);

        assert_eq!(stats.total_changes, 2);
        assert!(stats.changes_by_type.contains_key(&ChangeType::Modified));
    }

    #[test]
    fn test_impact_score() {
        let old = create_test_statute("test", "Old Title");
        let new = old.clone();

        let diff = crate::diff(&old, &new).unwrap();
        let stats = compute_statistics(&diff);

        assert_eq!(stats.impact_score, 0.0);
    }

    #[test]
    fn test_aggregate_statistics() {
        let old = create_test_statute("test", "Old Title");
        let mut new1 = old.clone();
        new1.title = "New Title 1".to_string();

        let mut new2 = old.clone();
        new2.title = "New Title 2".to_string();

        let diff1 = crate::diff(&old, &new1).unwrap();
        let diff2 = crate::diff(&old, &new2).unwrap();

        let agg = aggregate_statistics(&[diff1, diff2]);

        assert_eq!(agg.diff_count, 2);
        assert_eq!(agg.total_changes, 2);
        assert_eq!(agg.avg_changes_per_diff, 1.0);
    }

    #[test]
    fn test_pattern_detection() {
        let old = create_test_statute("test", "Old Title");
        let mut diffs = Vec::new();

        for i in 0..10 {
            let mut new = old.clone();
            new.preconditions[0] = Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18 + i,
            };
            diffs.push(crate::diff(&old, &new).unwrap());
        }

        let agg = aggregate_statistics(&diffs);
        assert!(!agg.patterns.is_empty());
    }

    #[test]
    fn test_summarize_statistics() {
        let old = create_test_statute("test", "Old Title");
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let diff = crate::diff(&old, &new).unwrap();
        let stats = compute_statistics(&diff);
        let summary = summarize_statistics(&stats);

        assert!(summary.contains("Total Changes"));
        assert!(summary.contains("Severity"));
    }

    #[test]
    fn test_empty_aggregate() {
        let agg = aggregate_statistics(&[]);
        assert_eq!(agg.diff_count, 0);
        assert_eq!(agg.total_changes, 0);
        assert_eq!(agg.avg_changes_per_diff, 0.0);
    }
}
