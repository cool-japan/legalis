#![allow(clippy::manual_clamp)]

//! Legalis-Diff: Statute diffing and change detection for Legalis-RS.
//!
//! This crate provides tools for detecting and analyzing changes between
//! statute versions:
//! - Structural diff between statutes
//! - Change categorization
//! - Impact analysis
//! - Amendment tracking
//! - Multiple output formats (JSON, HTML, Markdown)
//!
//! # Quick Start
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_diff::{diff, summarize};
//!
//! // Create two versions of a statute
//! let old = Statute::new(
//!     "benefit-123",
//!     "Senior Tax Credit",
//!     Effect::new(EffectType::Grant, "Tax credit granted"),
//! ).with_precondition(Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 65,
//! });
//!
//! let mut new = old.clone();
//! new.preconditions[0] = Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 60, // Lowered age requirement
//! };
//!
//! // Compute the diff
//! let diff_result = diff(&old, &new).unwrap();
//!
//! // Check impact
//! assert!(diff_result.impact.affects_eligibility);
//!
//! // Generate summary
//! let summary = summarize(&diff_result);
//! println!("{}", summary);
//! ```

use legalis_core::{Condition, Statute};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod adaptive;
pub mod advanced_visual;
pub mod algorithms;
pub mod analysis;
pub mod api;
pub mod audit;
pub mod cloud;
pub mod collaborative;
pub mod collaborative_review;
pub mod compliance;
pub mod cross_jurisdiction;
pub mod distributed;
pub mod dsl;
pub mod enterprise;
pub mod export;
pub mod export_plugins;
pub mod formats;
pub mod fuzzy;
pub mod git;
pub mod gpu;
pub mod integration;
pub mod integration_examples;
pub mod legal_domain;
pub mod legislative_history;
pub mod llm;
pub mod machine_readable;
pub mod merge;
pub mod ml;
pub mod multilingual;
pub mod nlp;
pub mod optimization;
pub mod parallel;
pub mod patterns;
pub mod plugins;
pub mod quantum;
pub mod recommendation;
pub mod rollback;
pub mod scripting;
pub mod security;
pub mod semantic;
pub mod simd;
pub mod statistics;
pub mod streaming;
pub mod templates;
pub mod time_travel;
pub mod timeline;
pub mod timeseries;
pub mod validation;
pub mod vcs;
pub mod vcs_integration;
pub mod visual;

/// Errors during diff operations.
#[derive(Debug, Error)]
pub enum DiffError {
    #[error("Cannot compare statutes with different IDs: {0} vs {1}")]
    IdMismatch(String, String),

    #[error("Invalid comparison: {0}")]
    InvalidComparison(String),

    #[error("Empty statute provided: {0}")]
    EmptyStatute(String),

    #[error("Version conflict: {old_version} -> {new_version}")]
    VersionConflict { old_version: u32, new_version: u32 },

    #[error("Merge conflict detected at {location}: {description}")]
    MergeConflict {
        location: String,
        description: String,
    },

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Result type for diff operations.
pub type DiffResult<T> = Result<T, DiffError>;

/// A diff between two statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteDiff {
    /// Statute ID
    pub statute_id: String,
    /// Version comparison (if available)
    pub version_info: Option<VersionInfo>,
    /// List of changes
    pub changes: Vec<Change>,
    /// Impact assessment
    pub impact: ImpactAssessment,
}

/// Version information for the diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub old_version: Option<u32>,
    pub new_version: Option<u32>,
}

/// A single change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    /// Type of change
    pub change_type: ChangeType,
    /// What was changed
    pub target: ChangeTarget,
    /// Description of the change
    pub description: String,
    /// Old value (if applicable)
    pub old_value: Option<String>,
    /// New value (if applicable)
    pub new_value: Option<String>,
}

/// Types of changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeType {
    /// Something was added
    Added,
    /// Something was removed
    Removed,
    /// Something was modified
    Modified,
    /// Order was changed
    Reordered,
}

/// What was changed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeTarget {
    Title,
    Precondition { index: usize },
    Effect,
    DiscretionLogic,
    Metadata { key: String },
}

impl std::fmt::Display for ChangeTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Title => write!(f, "Title"),
            Self::Precondition { index } => write!(f, "Precondition #{}", index + 1),
            Self::Effect => write!(f, "Effect"),
            Self::DiscretionLogic => write!(f, "Discretion Logic"),
            Self::Metadata { key } => write!(f, "Metadata[{}]", key),
        }
    }
}

/// Impact assessment of changes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// Overall severity
    pub severity: Severity,
    /// Whether the change affects eligibility criteria
    pub affects_eligibility: bool,
    /// Whether the change affects the outcome
    pub affects_outcome: bool,
    /// Whether discretion requirements changed
    pub discretion_changed: bool,
    /// Detailed impact notes
    pub notes: Vec<String>,
}

/// Severity of changes.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Severity {
    /// No significant impact
    #[default]
    None,
    /// Minor changes (typos, clarifications)
    Minor,
    /// Moderate changes (adjusted thresholds)
    Moderate,
    /// Major changes (new requirements, different outcomes)
    Major,
    /// Breaking changes (complete restructure)
    Breaking,
}

/// Computes the diff between two statutes.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::diff;
///
/// let old = Statute::new(
///     "tax-credit",
///     "Tax Credit for Seniors",
///     Effect::new(EffectType::Grant, "Tax credit granted"),
/// ).with_precondition(Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 65,
/// });
///
/// let mut new = old.clone();
/// new.title = "Enhanced Tax Credit for Seniors".to_string();
///
/// let result = diff(&old, &new).unwrap();
/// assert_eq!(result.changes.len(), 1);
/// assert!(result.changes[0].description.contains("Title"));
/// ```
///
/// # Errors
///
/// Returns [`DiffError::IdMismatch`] if the statute IDs don't match.
pub fn diff(old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
    if old.id != new.id {
        return Err(DiffError::IdMismatch(old.id.clone(), new.id.clone()));
    }

    let mut changes = Vec::new();
    let mut impact = ImpactAssessment::default();

    // Check title
    if old.title != new.title {
        changes.push(Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Title,
            description: "Title was modified".to_string(),
            old_value: Some(old.title.clone()),
            new_value: Some(new.title.clone()),
        });
        impact.severity = impact.severity.max(Severity::Minor);
    }

    // Check preconditions
    diff_preconditions(
        &old.preconditions,
        &new.preconditions,
        &mut changes,
        &mut impact,
    );

    // Check effect
    if old.effect != new.effect {
        changes.push(Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Effect,
            description: "Effect was modified".to_string(),
            old_value: Some(format!("{:?}", old.effect)),
            new_value: Some(format!("{:?}", new.effect)),
        });
        impact.affects_outcome = true;
        impact.severity = impact.severity.max(Severity::Major);
        impact
            .notes
            .push("Outcome of statute application changed".to_string());
    }

    // Check discretion logic
    match (&old.discretion_logic, &new.discretion_logic) {
        (None, Some(logic)) => {
            changes.push(Change {
                change_type: ChangeType::Added,
                target: ChangeTarget::DiscretionLogic,
                description: "Discretion logic was added".to_string(),
                old_value: None,
                new_value: Some(logic.clone()),
            });
            impact.discretion_changed = true;
            impact.severity = impact.severity.max(Severity::Major);
            impact.notes.push("Human judgment now required".to_string());
        }
        (Some(old_logic), None) => {
            changes.push(Change {
                change_type: ChangeType::Removed,
                target: ChangeTarget::DiscretionLogic,
                description: "Discretion logic was removed".to_string(),
                old_value: Some(old_logic.clone()),
                new_value: None,
            });
            impact.discretion_changed = true;
            impact.severity = impact.severity.max(Severity::Major);
            impact
                .notes
                .push("Human judgment no longer required - now deterministic".to_string());
        }
        (Some(old_logic), Some(new_logic)) if old_logic != new_logic => {
            changes.push(Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::DiscretionLogic,
                description: "Discretion logic was modified".to_string(),
                old_value: Some(old_logic.clone()),
                new_value: Some(new_logic.clone()),
            });
            impact.discretion_changed = true;
            impact.severity = impact.severity.max(Severity::Moderate);
        }
        _ => {}
    }

    Ok(StatuteDiff {
        statute_id: old.id.clone(),
        version_info: None,
        changes,
        impact,
    })
}

fn diff_preconditions(
    old: &[Condition],
    new: &[Condition],
    changes: &mut Vec<Change>,
    impact: &mut ImpactAssessment,
) {
    let old_len = old.len();
    let new_len = new.len();

    // Check for added/removed conditions
    if new_len > old_len {
        for (i, cond) in new.iter().enumerate().skip(old_len) {
            changes.push(Change {
                change_type: ChangeType::Added,
                target: ChangeTarget::Precondition { index: i },
                description: format!("New precondition added at position {}", i + 1),
                old_value: None,
                new_value: Some(format!("{:?}", cond)),
            });
        }
        impact.affects_eligibility = true;
        impact.severity = impact.severity.max(Severity::Major);
        impact
            .notes
            .push("New eligibility conditions added".to_string());
    } else if old_len > new_len {
        for (i, cond) in old.iter().enumerate().skip(new_len) {
            changes.push(Change {
                change_type: ChangeType::Removed,
                target: ChangeTarget::Precondition { index: i },
                description: format!("Precondition removed from position {}", i + 1),
                old_value: Some(format!("{:?}", cond)),
                new_value: None,
            });
        }
        impact.affects_eligibility = true;
        impact.severity = impact.severity.max(Severity::Major);
        impact
            .notes
            .push("Eligibility conditions removed".to_string());
    }

    // Check for modified conditions
    for i in 0..old_len.min(new_len) {
        if old[i] != new[i] {
            changes.push(Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::Precondition { index: i },
                description: format!("Precondition {} was modified", i + 1),
                old_value: Some(format!("{:?}", old[i])),
                new_value: Some(format!("{:?}", new[i])),
            });
            impact.affects_eligibility = true;
            impact.severity = impact.severity.max(Severity::Moderate);
        }
    }
}

/// Summary of changes for display.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, summarize};
///
/// let old = Statute::new(
///     "benefit-123",
///     "Old Title",
///     Effect::new(EffectType::Grant, "Benefit granted"),
/// );
///
/// let new = old.clone().with_precondition(Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 18,
/// });
///
/// let diff_result = diff(&old, &new).unwrap();
/// let summary = summarize(&diff_result);
///
/// assert!(summary.contains("benefit-123"));
/// assert!(summary.contains("Severity"));
/// ```
pub fn summarize(diff: &StatuteDiff) -> String {
    let mut summary = format!("Diff for statute '{}'\n", diff.statute_id);
    summary.push_str(&format!("Severity: {:?}\n", diff.impact.severity));
    summary.push_str(&format!("Changes: {}\n\n", diff.changes.len()));

    for change in &diff.changes {
        summary.push_str(&format!(
            "  [{:?}] {}: {}\n",
            change.change_type, change.target, change.description
        ));
    }

    if !diff.impact.notes.is_empty() {
        summary.push_str("\nImpact Notes:\n");
        for note in &diff.impact.notes {
            summary.push_str(&format!("  - {}\n", note));
        }
    }

    summary
}

/// Filters changes by type from a diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, filter_changes_by_type, ChangeType};
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone()
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 18,
///     });
///
/// let diff_result = diff(&old, &new).unwrap();
/// let added = filter_changes_by_type(&diff_result, ChangeType::Added);
///
/// assert_eq!(added.len(), 1);
/// ```
pub fn filter_changes_by_type(diff: &StatuteDiff, change_type: ChangeType) -> Vec<Change> {
    diff.changes
        .iter()
        .filter(|c| c.change_type == change_type)
        .cloned()
        .collect()
}

/// Checks if a diff contains any breaking changes.
///
/// Breaking changes include:
/// - Effect modifications
/// - Precondition additions (tightens eligibility)
/// - Changes in discretion requirements
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, has_breaking_changes};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.effect = Effect::new(EffectType::Revoke, "Revoke benefit"); // Breaking change!
///
/// let diff_result = diff(&old, &new).unwrap();
/// assert!(has_breaking_changes(&diff_result));
/// ```
pub fn has_breaking_changes(diff: &StatuteDiff) -> bool {
    use crate::Severity;

    diff.impact.severity >= Severity::Major
        || diff.impact.affects_outcome
        || diff.impact.discretion_changed
}

/// Counts the number of changes by target type.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, count_changes_by_target};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let counts = count_changes_by_target(&diff_result);
///
/// assert!(counts.contains_key("Title"));
/// ```
pub fn count_changes_by_target(diff: &StatuteDiff) -> std::collections::HashMap<String, usize> {
    use std::collections::HashMap;

    let mut counts: HashMap<String, usize> = HashMap::new();

    for change in &diff.changes {
        let key = match &change.target {
            ChangeTarget::Title => "Title".to_string(),
            ChangeTarget::Precondition { .. } => "Precondition".to_string(),
            ChangeTarget::Effect => "Effect".to_string(),
            ChangeTarget::DiscretionLogic => "DiscretionLogic".to_string(),
            ChangeTarget::Metadata { .. } => "Metadata".to_string(),
        };
        *counts.entry(key).or_insert(0) += 1;
    }

    counts
}

/// Computes diffs for a sequence of statute versions.
///
/// Returns a vector of diffs, where each diff represents the changes from
/// one version to the next.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::diff_sequence;
///
/// let v1 = Statute::new("law", "Version 1", Effect::new(EffectType::Grant, "Benefit"));
/// let v2 = Statute::new("law", "Version 2", Effect::new(EffectType::Grant, "Benefit"));
/// let v3 = Statute::new("law", "Version 3", Effect::new(EffectType::Grant, "Benefit"));
///
/// let versions = vec![v1, v2, v3];
/// let diffs = diff_sequence(&versions).unwrap();
///
/// // Should have 2 diffs for 3 versions (v1->v2, v2->v3)
/// assert_eq!(diffs.len(), 2);
/// ```
///
/// # Errors
///
/// Returns [`DiffError::IdMismatch`] if any statutes have different IDs.
pub fn diff_sequence(versions: &[Statute]) -> DiffResult<Vec<StatuteDiff>> {
    if versions.len() < 2 {
        return Ok(Vec::new());
    }

    let mut diffs = Vec::new();
    for i in 0..versions.len() - 1 {
        diffs.push(diff(&versions[i], &versions[i + 1])?);
    }

    Ok(diffs)
}

/// Detailed summary with confidence scores for each aspect of the diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedSummary {
    /// The statute ID.
    pub statute_id: String,
    /// Overall confidence score (0.0 to 1.0).
    pub overall_confidence: f64,
    /// Number of changes detected.
    pub change_count: usize,
    /// Severity level.
    pub severity: Severity,
    /// Summary text.
    pub summary_text: String,
    /// Confidence in change detection (0.0 to 1.0).
    pub change_detection_confidence: f64,
    /// Confidence in impact assessment (0.0 to 1.0).
    pub impact_assessment_confidence: f64,
    /// Key insights from the analysis.
    pub insights: Vec<String>,
}

/// Creates a detailed summary with confidence scores.
///
/// This provides more information than the basic `summarize` function,
/// including confidence metrics and analytical insights.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, detailed_summary};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let diff_result = diff(&old, &new).unwrap();
/// let summary = detailed_summary(&diff_result);
///
/// assert_eq!(summary.statute_id, "law");
/// assert!(summary.overall_confidence > 0.0);
/// ```
pub fn detailed_summary(diff: &StatuteDiff) -> DetailedSummary {
    let mut insights = Vec::new();
    let change_count = diff.changes.len();

    // Calculate change detection confidence
    let change_detection_confidence = if change_count == 0 {
        1.0 // High confidence in no changes
    } else {
        0.95 // High confidence in detected changes
    };

    // Calculate impact assessment confidence based on severity and flags
    let impact_assessment_confidence = if diff.impact.affects_outcome
        || diff.impact.affects_eligibility
        || diff.impact.discretion_changed
    {
        0.9 // High confidence in significant impact
    } else if diff.impact.severity >= Severity::Moderate {
        0.85
    } else if diff.impact.severity == Severity::Minor {
        0.8
    } else {
        0.95 // Very high confidence in no impact
    };

    // Generate insights
    if diff.impact.affects_eligibility {
        insights
            .push("This change affects who is eligible for the statute's provisions.".to_string());
    }

    if diff.impact.affects_outcome {
        insights.push("This change modifies the outcome or effect of the statute.".to_string());
    }

    if diff.impact.discretion_changed {
        insights.push("Discretionary judgment requirements have been modified.".to_string());
    }

    // Analyze change patterns
    let added_count = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Added)
        .count();
    let removed_count = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Removed)
        .count();
    let modified_count = diff
        .changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Modified)
        .count();

    if added_count > 0 {
        insights.push(format!("{} new element(s) added.", added_count));
    }
    if removed_count > 0 {
        insights.push(format!("{} element(s) removed.", removed_count));
    }
    if modified_count > 0 {
        insights.push(format!("{} element(s) modified.", modified_count));
    }

    // Calculate overall confidence
    let overall_confidence = (change_detection_confidence + impact_assessment_confidence) / 2.0;

    // Build summary text
    let summary_text = summarize(diff);

    DetailedSummary {
        statute_id: diff.statute_id.clone(),
        overall_confidence,
        change_count,
        severity: diff.impact.severity,
        summary_text,
        change_detection_confidence,
        impact_assessment_confidence,
        insights,
    }
}

/// Compares only the preconditions of two statutes.
///
/// This is useful when you only need to check for eligibility criteria changes
/// without analyzing the entire statute.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::diff_preconditions_only;
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 65,
///     });
///
/// let new = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 60,
///     });
///
/// let changes = diff_preconditions_only(&old, &new).unwrap();
/// assert!(!changes.is_empty());
/// ```
///
/// # Errors
///
/// Returns [`DiffError::IdMismatch`] if the statute IDs don't match.
pub fn diff_preconditions_only(old: &Statute, new: &Statute) -> DiffResult<Vec<Change>> {
    if old.id != new.id {
        return Err(DiffError::IdMismatch(old.id.clone(), new.id.clone()));
    }

    let mut changes = Vec::new();
    let mut impact = ImpactAssessment::default();

    diff_preconditions(
        &old.preconditions,
        &new.preconditions,
        &mut changes,
        &mut impact,
    );

    Ok(changes)
}

/// Compares only the effect of two statutes.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::diff_effect_only;
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Old benefit"));
/// let new = Statute::new("law", "Title", Effect::new(EffectType::Grant, "New benefit"));
///
/// let change = diff_effect_only(&old, &new).unwrap();
/// assert!(change.is_some());
/// ```
///
/// # Errors
///
/// Returns [`DiffError::IdMismatch`] if the statute IDs don't match.
pub fn diff_effect_only(old: &Statute, new: &Statute) -> DiffResult<Option<Change>> {
    if old.id != new.id {
        return Err(DiffError::IdMismatch(old.id.clone(), new.id.clone()));
    }

    if old.effect != new.effect {
        Ok(Some(Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Effect,
            description: "Effect was modified".to_string(),
            old_value: Some(format!("{:?}", old.effect)),
            new_value: Some(format!("{:?}", new.effect)),
        }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Effect, EffectType};

    fn test_statute() -> Statute {
        Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    fn empty_statute() -> Statute {
        Statute::new(
            "empty-statute",
            "Empty Statute",
            Effect::new(EffectType::Grant, "Empty effect"),
        )
    }

    #[test]
    fn test_no_changes() {
        let statute = test_statute();
        let result = diff(&statute, &statute).unwrap();
        assert!(result.changes.is_empty());
        assert_eq!(result.impact.severity, Severity::None);
    }

    #[test]
    fn test_identical_statutes() {
        let statute1 = test_statute();
        let statute2 = test_statute();
        let result = diff(&statute1, &statute2).unwrap();
        assert!(result.changes.is_empty());
        assert_eq!(result.impact.severity, Severity::None);
        assert!(!result.impact.affects_eligibility);
        assert!(!result.impact.affects_outcome);
    }

    #[test]
    fn test_empty_statutes() {
        let statute1 = empty_statute();
        let statute2 = empty_statute();
        let result = diff(&statute1, &statute2).unwrap();
        assert!(result.changes.is_empty());
        assert_eq!(result.impact.severity, Severity::None);
    }

    #[test]
    fn test_empty_to_populated() {
        let old = empty_statute();
        let mut new = old.clone();
        new.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let result = diff(&old, &new).unwrap();
        assert_eq!(result.changes.len(), 1);
        assert!(result.impact.affects_eligibility);
        assert!(matches!(result.changes[0].change_type, ChangeType::Added));
    }

    #[test]
    fn test_title_change() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "Modified Title".to_string();

        let result = diff(&old, &new).unwrap();
        assert_eq!(result.changes.len(), 1);
        assert!(matches!(result.changes[0].target, ChangeTarget::Title));
    }

    #[test]
    fn test_precondition_added() {
        let old = test_statute();
        let mut new = old.clone();
        new.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let result = diff(&old, &new).unwrap();
        assert!(result.impact.affects_eligibility);
        assert!(result.impact.severity >= Severity::Major);
    }

    #[test]
    fn test_precondition_removed() {
        let mut old = test_statute();
        old.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });
        let new = test_statute(); // Has only the age condition

        let result = diff(&old, &new).unwrap();
        assert!(result.impact.affects_eligibility);
        assert!(result.impact.severity >= Severity::Major);
        let has_removed = result
            .changes
            .iter()
            .any(|c| matches!(c.change_type, ChangeType::Removed));
        assert!(has_removed);
    }

    #[test]
    fn test_precondition_modified() {
        let old = test_statute();
        let mut new = old.clone();
        new.preconditions[0] = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        };

        let result = diff(&old, &new).unwrap();
        assert!(result.impact.affects_eligibility);
        assert!(!result.changes.is_empty());
        let has_modified = result
            .changes
            .iter()
            .any(|c| matches!(c.change_type, ChangeType::Modified));
        assert!(has_modified);
    }

    #[test]
    fn test_effect_change() {
        let old = test_statute();
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke instead");

        let result = diff(&old, &new).unwrap();
        assert!(result.impact.affects_outcome);
        assert_eq!(result.impact.severity, Severity::Major);
    }

    #[test]
    fn test_discretion_added() {
        let old = test_statute();
        let new = old
            .clone()
            .with_discretion("Consider special circumstances");

        let result = diff(&old, &new).unwrap();
        assert!(result.impact.discretion_changed);
    }

    #[test]
    fn test_discretion_removed() {
        let old = test_statute().with_discretion("Consider special circumstances");
        let new = test_statute();

        let result = diff(&old, &new).unwrap();
        assert!(result.impact.discretion_changed);
        assert!(result.impact.severity >= Severity::Major);
    }

    #[test]
    fn test_discretion_modified() {
        let old = test_statute().with_discretion("Consider special circumstances");
        let new = test_statute().with_discretion("Consider different circumstances");

        let result = diff(&old, &new).unwrap();
        assert!(result.impact.discretion_changed);
    }

    #[test]
    fn test_multiple_changes() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();
        new.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });
        new.effect = Effect::new(EffectType::Obligation, "New effect");

        let result = diff(&old, &new).unwrap();
        assert!(result.changes.len() >= 3);
        assert!(result.impact.affects_eligibility);
        assert!(result.impact.affects_outcome);
        assert_eq!(result.impact.severity, Severity::Major);
    }

    #[test]
    fn test_id_mismatch_error() {
        let old = test_statute();
        let mut new = test_statute();
        new.id = "different-id".to_string();

        let result = diff(&old, &new);
        assert!(matches!(result, Err(DiffError::IdMismatch(_, _))));
    }

    #[test]
    fn test_summarize() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "Modified Title".to_string();

        let result = diff(&old, &new).unwrap();
        let summary = summarize(&result);

        assert!(summary.contains("test-statute"));
        assert!(summary.contains("Modified"));
    }

    #[test]
    fn test_all_preconditions_removed() {
        let old = test_statute();
        let mut new = old.clone();
        new.preconditions.clear();

        let result = diff(&old, &new).unwrap();
        assert!(result.impact.affects_eligibility);
        assert!(result.impact.severity >= Severity::Major);
    }

    #[test]
    fn test_version_info_present() {
        let old = test_statute();
        let new = test_statute();

        let mut result = diff(&old, &new).unwrap();
        result.version_info = Some(VersionInfo {
            old_version: Some(1),
            new_version: Some(2),
        });

        assert!(result.version_info.is_some());
        assert_eq!(result.version_info.as_ref().unwrap().old_version, Some(1));
        assert_eq!(result.version_info.as_ref().unwrap().new_version, Some(2));
    }

    #[test]
    fn test_detailed_summary() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "Modified Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let summary = detailed_summary(&diff_result);

        assert_eq!(summary.statute_id, "test-statute");
        assert!(summary.overall_confidence > 0.0);
        assert!(summary.overall_confidence <= 1.0);
        assert_eq!(summary.change_count, 1);
        assert!(!summary.insights.is_empty());
    }

    #[test]
    fn test_diff_preconditions_only() {
        let old = test_statute();
        let mut new = old.clone();
        new.preconditions[0] = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        };

        let changes = diff_preconditions_only(&old, &new).unwrap();
        assert!(!changes.is_empty());
        assert!(matches!(
            changes[0].target,
            ChangeTarget::Precondition { .. }
        ));
    }

    #[test]
    fn test_diff_effect_only() {
        let old = test_statute();
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Different effect");

        let change = diff_effect_only(&old, &new).unwrap();
        assert!(change.is_some());
        let c = change.unwrap();
        assert!(matches!(c.target, ChangeTarget::Effect));
    }

    #[test]
    fn test_diff_effect_only_no_change() {
        let old = test_statute();
        let new = old.clone();

        let change = diff_effect_only(&old, &new).unwrap();
        assert!(change.is_none());
    }
}
