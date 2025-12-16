//! Legalis-Diff: Statute diffing and change detection for Legalis-RS.
//!
//! This crate provides tools for detecting and analyzing changes between
//! statute versions:
//! - Structural diff between statutes
//! - Change categorization
//! - Impact analysis
//! - Amendment tracking
//! - Multiple output formats (JSON, HTML, Markdown)

use legalis_core::{Condition, Statute};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod algorithms;
pub mod analysis;
pub mod formats;
pub mod git;
pub mod merge;
pub mod optimization;
pub mod semantic;
pub mod statistics;
pub mod templates;
pub mod timeline;
pub mod vcs;
pub mod visual;

/// Errors during diff operations.
#[derive(Debug, Error)]
pub enum DiffError {
    #[error("Cannot compare statutes with different IDs: {0} vs {1}")]
    IdMismatch(String, String),

    #[error("Invalid comparison: {0}")]
    InvalidComparison(String),
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
}
