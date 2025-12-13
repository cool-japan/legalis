//! Legalis-Diff: Statute diffing and change detection for Legalis-RS.
//!
//! This crate provides tools for detecting and analyzing changes between
//! statute versions:
//! - Structural diff between statutes
//! - Change categorization
//! - Impact analysis
//! - Amendment tracking

use legalis_core::{Condition, Statute};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

    #[test]
    fn test_no_changes() {
        let statute = test_statute();
        let result = diff(&statute, &statute).unwrap();
        assert!(result.changes.is_empty());
        assert_eq!(result.impact.severity, Severity::None);
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
    fn test_discretion_added() {
        let old = test_statute();
        let new = old
            .clone()
            .with_discretion("Consider special circumstances");

        let result = diff(&old, &new).unwrap();
        assert!(result.impact.discretion_changed);
    }

    #[test]
    fn test_id_mismatch_error() {
        let old = test_statute();
        let mut new = test_statute();
        new.id = "different-id".to_string();

        let result = diff(&old, &new);
        assert!(matches!(result, Err(DiffError::IdMismatch(_, _))));
    }
}
