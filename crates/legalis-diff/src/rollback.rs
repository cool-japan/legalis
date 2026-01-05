//! Rollback analysis for statute changes.
//!
//! This module provides tools for analyzing what it would take to reverse
//! a set of changes, including generating rollback diffs and assessing
//! rollback feasibility.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::{diff, rollback::generate_rollback};
//!
//! let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
//! let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
//!
//! let forward_diff = diff(&old, &new).unwrap();
//! let rollback_diff = generate_rollback(&forward_diff);
//!
//! // Rollback reverses the changes
//! assert_eq!(rollback_diff.changes.len(), forward_diff.changes.len());
//! ```

use crate::{Change, ChangeTarget, ChangeType, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};

/// Generates a rollback diff that reverses the changes in a forward diff.
///
/// The rollback diff can be used to understand what needs to be done to
/// reverse a set of changes.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, rollback::generate_rollback};
///
/// let old = Statute::new("law", "Version 1", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "Version 2", Effect::new(EffectType::Grant, "Benefit"));
///
/// let forward = diff(&old, &new).unwrap();
/// let rollback = generate_rollback(&forward);
///
/// assert_eq!(rollback.statute_id, forward.statute_id);
/// ```
pub fn generate_rollback(forward_diff: &StatuteDiff) -> StatuteDiff {
    let mut rollback_changes = Vec::new();

    for change in &forward_diff.changes {
        let rollback_change = match change.change_type {
            ChangeType::Added => Change {
                change_type: ChangeType::Removed,
                target: change.target.clone(),
                description: format!("Rollback: Remove {}", change.description),
                old_value: change.new_value.clone(),
                new_value: None,
            },
            ChangeType::Removed => Change {
                change_type: ChangeType::Added,
                target: change.target.clone(),
                description: format!("Rollback: Restore {}", change.description),
                old_value: None,
                new_value: change.old_value.clone(),
            },
            ChangeType::Modified => Change {
                change_type: ChangeType::Modified,
                target: change.target.clone(),
                description: format!("Rollback: Revert {}", change.description),
                old_value: change.new_value.clone(),
                new_value: change.old_value.clone(),
            },
            ChangeType::Reordered => Change {
                change_type: ChangeType::Reordered,
                target: change.target.clone(),
                description: "Rollback: Restore original order".to_string(),
                old_value: change.new_value.clone(),
                new_value: change.old_value.clone(),
            },
        };

        rollback_changes.push(rollback_change);
    }

    StatuteDiff {
        statute_id: forward_diff.statute_id.clone(),
        version_info: forward_diff
            .version_info
            .as_ref()
            .map(|v| crate::VersionInfo {
                old_version: v.new_version,
                new_version: v.old_version,
            }),
        changes: rollback_changes,
        impact: forward_diff.impact.clone(),
    }
}

/// Analysis of rollback feasibility and complexity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackAnalysis {
    /// Whether the rollback is feasible.
    pub is_feasible: bool,
    /// Complexity of the rollback operation.
    pub complexity: RollbackComplexity,
    /// Potential issues with rolling back.
    pub issues: Vec<RollbackIssue>,
    /// Recommendations for the rollback.
    pub recommendations: Vec<String>,
    /// Estimated risk level of the rollback.
    pub risk_level: RollbackRisk,
}

/// Complexity level of a rollback operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackComplexity {
    /// Simple rollback with no dependencies.
    Trivial,
    /// Straightforward rollback with minimal impact.
    Simple,
    /// Moderate complexity requiring careful execution.
    Moderate,
    /// Complex rollback with many interdependencies.
    Complex,
    /// Very complex rollback requiring extensive analysis.
    VeryComplex,
}

/// Risk level associated with a rollback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RollbackRisk {
    /// Low risk - safe to rollback.
    Low,
    /// Medium risk - proceed with caution.
    Medium,
    /// High risk - requires careful planning.
    High,
    /// Critical risk - may have severe consequences.
    Critical,
}

/// Potential issue with a rollback operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackIssue {
    /// Type of issue.
    pub issue_type: RollbackIssueType,
    /// Description of the issue.
    pub description: String,
    /// Severity of the issue.
    pub severity: Severity,
    /// Affected change targets.
    pub affected_targets: Vec<ChangeTarget>,
}

/// Types of rollback issues.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackIssueType {
    /// Data loss would occur.
    DataLoss,
    /// Breaking change that can't be cleanly reversed.
    IrreversibleChange,
    /// Dependencies on the current state.
    DependencyConflict,
    /// Missing information needed for rollback.
    MissingInformation,
    /// Potential for state inconsistency.
    StateInconsistency,
}

/// Analyzes the feasibility and complexity of rolling back a diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, rollback::analyze_rollback};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 65,
///     });
///
/// let mut new = old.clone();
/// new.title = "New Title".to_string();
///
/// let forward = diff(&old, &new).unwrap();
/// let analysis = analyze_rollback(&forward);
///
/// assert!(analysis.is_feasible);
/// ```
pub fn analyze_rollback(forward_diff: &StatuteDiff) -> RollbackAnalysis {
    let mut issues = Vec::new();
    let mut recommendations = Vec::new();

    // Check for irreversible changes
    let has_effect_change = forward_diff
        .changes
        .iter()
        .any(|c| matches!(c.target, ChangeTarget::Effect));

    let has_precondition_removal = forward_diff.changes.iter().any(|c| {
        matches!(c.target, ChangeTarget::Precondition { .. })
            && c.change_type == ChangeType::Removed
    });

    // Analyze impact
    let is_breaking = forward_diff.impact.severity >= Severity::Major;

    if has_effect_change {
        issues.push(RollbackIssue {
            issue_type: RollbackIssueType::IrreversibleChange,
            description:
                "Effect changes may have already impacted decisions made under the new version"
                    .to_string(),
            severity: Severity::Major,
            affected_targets: vec![ChangeTarget::Effect],
        });
        recommendations.push("Consider the impact on any decisions or actions taken under the new effect before rolling back".to_string());
    }

    if has_precondition_removal {
        issues.push(RollbackIssue {
            issue_type: RollbackIssueType::StateInconsistency,
            description:
                "Rolling back removed preconditions may exclude currently eligible entities"
                    .to_string(),
            severity: Severity::Moderate,
            affected_targets: forward_diff
                .changes
                .iter()
                .filter(|c| {
                    matches!(c.target, ChangeTarget::Precondition { .. })
                        && c.change_type == ChangeType::Removed
                })
                .map(|c| c.target.clone())
                .collect(),
        });
        recommendations.push("Verify that re-adding preconditions won't invalidate existing applications or approvals".to_string());
    }

    // Determine complexity
    let change_count = forward_diff.changes.len();
    let complexity = match change_count {
        0 => RollbackComplexity::Trivial,
        1..=2 => {
            if is_breaking {
                RollbackComplexity::Moderate
            } else {
                RollbackComplexity::Simple
            }
        }
        3..=5 => {
            if is_breaking {
                RollbackComplexity::Complex
            } else {
                RollbackComplexity::Moderate
            }
        }
        _ => RollbackComplexity::VeryComplex,
    };

    // Determine risk level
    let risk_level = if issues.iter().any(|i| i.severity >= Severity::Major) {
        RollbackRisk::High
    } else if is_breaking || !issues.is_empty() {
        RollbackRisk::Medium
    } else {
        RollbackRisk::Low
    };

    // Add general recommendations
    if is_breaking {
        recommendations.push(
            "This is a breaking change - coordinate rollback with all stakeholders".to_string(),
        );
    }

    if forward_diff.impact.affects_eligibility {
        recommendations
            .push("Eligibility criteria changed - review impact on current applicants".to_string());
    }

    if forward_diff.impact.discretion_changed {
        recommendations.push(
            "Discretion requirements changed - ensure consistency in application".to_string(),
        );
    }

    // Rollback is generally feasible, but may have risks
    let is_feasible = !issues
        .iter()
        .any(|i| matches!(i.issue_type, RollbackIssueType::MissingInformation));

    RollbackAnalysis {
        is_feasible,
        complexity,
        issues,
        recommendations,
        risk_level,
    }
}

/// Generates a chain of rollback diffs for a sequence of changes.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff_sequence, rollback::generate_rollback_chain};
///
/// let v1 = Statute::new("law", "Version 1", Effect::new(EffectType::Grant, "Benefit"));
/// let v2 = Statute::new("law", "Version 2", Effect::new(EffectType::Grant, "Benefit"));
/// let v3 = Statute::new("law", "Version 3", Effect::new(EffectType::Grant, "Benefit"));
///
/// let versions = vec![v1, v2, v3];
/// let forward_diffs = diff_sequence(&versions).unwrap();
/// let rollback_chain = generate_rollback_chain(&forward_diffs);
///
/// assert_eq!(rollback_chain.len(), forward_diffs.len());
/// ```
pub fn generate_rollback_chain(forward_diffs: &[StatuteDiff]) -> Vec<StatuteDiff> {
    forward_diffs.iter().rev().map(generate_rollback).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn test_statute() -> Statute {
        Statute::new(
            "test-law",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_generate_rollback_title_change() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "Modified Title".to_string();

        let forward = diff(&old, &new).unwrap();
        let rollback = generate_rollback(&forward);

        assert_eq!(rollback.changes.len(), 1);
        assert_eq!(rollback.changes[0].change_type, ChangeType::Modified);
        assert_eq!(
            rollback.changes[0].old_value,
            Some("Modified Title".to_string())
        );
        assert_eq!(
            rollback.changes[0].new_value,
            Some("Test Statute".to_string())
        );
    }

    #[test]
    fn test_generate_rollback_added_precondition() {
        let old = test_statute();
        let mut new = old.clone();
        new.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let forward = diff(&old, &new).unwrap();
        let rollback = generate_rollback(&forward);

        let rollback_changes: Vec<_> = rollback
            .changes
            .iter()
            .filter(|c| matches!(c.target, ChangeTarget::Precondition { .. }))
            .collect();

        assert!(!rollback_changes.is_empty());
        assert!(
            rollback_changes
                .iter()
                .any(|c| c.change_type == ChangeType::Removed)
        );
    }

    #[test]
    fn test_generate_rollback_removed_precondition() {
        let mut old = test_statute();
        old.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });
        let new = test_statute();

        let forward = diff(&old, &new).unwrap();
        let rollback = generate_rollback(&forward);

        let rollback_changes: Vec<_> = rollback
            .changes
            .iter()
            .filter(|c| matches!(c.target, ChangeTarget::Precondition { .. }))
            .collect();

        assert!(!rollback_changes.is_empty());
        assert!(
            rollback_changes
                .iter()
                .any(|c| c.change_type == ChangeType::Added)
        );
    }

    #[test]
    fn test_analyze_rollback_simple() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let forward = diff(&old, &new).unwrap();
        let analysis = analyze_rollback(&forward);

        assert!(analysis.is_feasible);
        assert_eq!(analysis.complexity, RollbackComplexity::Simple);
        assert_eq!(analysis.risk_level, RollbackRisk::Low);
    }

    #[test]
    fn test_analyze_rollback_effect_change() {
        let old = test_statute();
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke benefit");

        let forward = diff(&old, &new).unwrap();
        let analysis = analyze_rollback(&forward);

        assert!(analysis.is_feasible);
        assert!(analysis.risk_level >= RollbackRisk::Medium);
        assert!(!analysis.issues.is_empty());
        assert!(
            analysis
                .issues
                .iter()
                .any(|i| matches!(i.issue_type, RollbackIssueType::IrreversibleChange))
        );
    }

    #[test]
    fn test_analyze_rollback_precondition_removal() {
        let mut old = test_statute();
        old.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });
        let new = test_statute();

        let forward = diff(&old, &new).unwrap();
        let analysis = analyze_rollback(&forward);

        assert!(analysis.is_feasible);
        assert!(!analysis.issues.is_empty());
        assert!(
            analysis
                .issues
                .iter()
                .any(|i| matches!(i.issue_type, RollbackIssueType::StateInconsistency))
        );
    }

    #[test]
    fn test_analyze_rollback_complex() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();
        new.effect = Effect::new(EffectType::Revoke, "Revoke");
        new.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });
        new.preconditions.push(Condition::HasAttribute {
            key: "residence".to_string(),
        });

        let forward = diff(&old, &new).unwrap();
        let analysis = analyze_rollback(&forward);

        assert!(analysis.is_feasible);
        assert!(matches!(
            analysis.complexity,
            RollbackComplexity::Complex | RollbackComplexity::VeryComplex
        ));
        assert!(!analysis.recommendations.is_empty());
    }

    #[test]
    fn test_generate_rollback_chain() {
        let v1 = test_statute();
        let mut v2 = v1.clone();
        v2.title = "Version 2".to_string();
        let mut v3 = v2.clone();
        v3.title = "Version 3".to_string();

        let versions = vec![v1, v2, v3];
        let forward_diffs = crate::diff_sequence(&versions).unwrap();
        let rollback_chain = generate_rollback_chain(&forward_diffs);

        assert_eq!(rollback_chain.len(), 2);
        // Chain should be reversed
        assert_eq!(
            rollback_chain[0].changes[0].new_value,
            Some("Version 2".to_string())
        );
        assert_eq!(
            rollback_chain[1].changes[0].new_value,
            Some("Test Statute".to_string())
        );
    }

    #[test]
    fn test_rollback_version_info() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let mut forward = diff(&old, &new).unwrap();
        forward.version_info = Some(crate::VersionInfo {
            old_version: Some(1),
            new_version: Some(2),
        });

        let rollback = generate_rollback(&forward);

        assert!(rollback.version_info.is_some());
        assert_eq!(rollback.version_info.as_ref().unwrap().old_version, Some(2));
        assert_eq!(rollback.version_info.as_ref().unwrap().new_version, Some(1));
    }

    #[test]
    fn test_rollback_breaking_changes() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();
        new.effect = Effect::new(EffectType::Obligation, "Different effect type");

        let forward = diff(&old, &new).unwrap();
        let analysis = analyze_rollback(&forward);

        assert!(analysis.risk_level >= RollbackRisk::Medium);
        assert!(
            analysis
                .recommendations
                .iter()
                .any(|r| r.contains("breaking"))
        );
    }
}

/// Generates rollback diffs for multiple forward diffs in parallel.
///
/// This is significantly faster than sequential processing when you have
/// many diffs to reverse.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, rollback::parallel_generate_rollbacks};
///
/// let pairs: Vec<_> = (0..10)
///     .map(|i| {
///         let id = format!("law{}", i);
///         (
///             Statute::new(&id, "Old Title", Effect::new(EffectType::Grant, "Benefit")),
///             Statute::new(&id, "New Title", Effect::new(EffectType::Grant, "Benefit")),
///         )
///     })
///     .collect();
///
/// let forward_diffs: Vec<_> = pairs
///     .iter()
///     .map(|(old, new)| diff(old, new).unwrap())
///     .collect();
///
/// let rollbacks = parallel_generate_rollbacks(&forward_diffs);
/// assert_eq!(rollbacks.len(), 10);
/// ```
pub fn parallel_generate_rollbacks(forward_diffs: &[StatuteDiff]) -> Vec<StatuteDiff> {
    use rayon::prelude::*;
    forward_diffs.par_iter().map(generate_rollback).collect()
}

/// Analyzes rollback feasibility for multiple diffs in parallel.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, rollback::parallel_analyze_rollbacks};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 65,
///     });
///
/// let mut new1 = old.clone();
/// new1.title = "New Title 1".to_string();
///
/// let mut new2 = old.clone();
/// new2.effect = Effect::new(EffectType::Revoke, "Revoke benefit");
///
/// let diffs = vec![
///     diff(&old, &new1).unwrap(),
///     diff(&old, &new2).unwrap(),
/// ];
///
/// let analyses = parallel_analyze_rollbacks(&diffs);
/// assert_eq!(analyses.len(), 2);
/// ```
pub fn parallel_analyze_rollbacks(forward_diffs: &[StatuteDiff]) -> Vec<RollbackAnalysis> {
    use rayon::prelude::*;
    forward_diffs.par_iter().map(analyze_rollback).collect()
}

/// Aggregate statistics about rollback operations across multiple diffs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackStatistics {
    /// Total number of diffs analyzed.
    pub total_diffs: usize,
    /// Number of feasible rollbacks.
    pub feasible_count: usize,
    /// Number of infeasible rollbacks.
    pub infeasible_count: usize,
    /// Complexity distribution.
    pub complexity_distribution: ComplexityDistribution,
    /// Risk distribution.
    pub risk_distribution: RiskDistribution,
    /// Total number of issues identified.
    pub total_issues: usize,
    /// Average number of recommendations per diff.
    pub average_recommendations: f64,
}

/// Distribution of rollback complexities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityDistribution {
    pub trivial: usize,
    pub simple: usize,
    pub moderate: usize,
    pub complex: usize,
    pub very_complex: usize,
}

/// Distribution of rollback risks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDistribution {
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub critical: usize,
}

/// Computes aggregate statistics for rollback operations across multiple diffs.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, rollback::{parallel_analyze_rollbacks, compute_rollback_statistics}};
///
/// let pairs: Vec<_> = (0..20)
///     .map(|i| {
///         let id = format!("law{}", i);
///         (
///             Statute::new(&id, "Old Title", Effect::new(EffectType::Grant, "Benefit")),
///             Statute::new(&id, "New Title", Effect::new(EffectType::Grant, "Benefit")),
///         )
///     })
///     .collect();
///
/// let diffs: Vec<_> = pairs
///     .iter()
///     .map(|(old, new)| diff(old, new).unwrap())
///     .collect();
///
/// let analyses = parallel_analyze_rollbacks(&diffs);
/// let stats = compute_rollback_statistics(&analyses);
///
/// assert_eq!(stats.total_diffs, 20);
/// assert!(stats.feasible_count <= 20);
/// ```
pub fn compute_rollback_statistics(analyses: &[RollbackAnalysis]) -> RollbackStatistics {
    let total_diffs = analyses.len();
    let feasible_count = analyses.iter().filter(|a| a.is_feasible).count();
    let infeasible_count = total_diffs - feasible_count;

    let mut complexity_dist = ComplexityDistribution {
        trivial: 0,
        simple: 0,
        moderate: 0,
        complex: 0,
        very_complex: 0,
    };

    let mut risk_dist = RiskDistribution {
        low: 0,
        medium: 0,
        high: 0,
        critical: 0,
    };

    for analysis in analyses {
        match analysis.complexity {
            RollbackComplexity::Trivial => complexity_dist.trivial += 1,
            RollbackComplexity::Simple => complexity_dist.simple += 1,
            RollbackComplexity::Moderate => complexity_dist.moderate += 1,
            RollbackComplexity::Complex => complexity_dist.complex += 1,
            RollbackComplexity::VeryComplex => complexity_dist.very_complex += 1,
        }

        match analysis.risk_level {
            RollbackRisk::Low => risk_dist.low += 1,
            RollbackRisk::Medium => risk_dist.medium += 1,
            RollbackRisk::High => risk_dist.high += 1,
            RollbackRisk::Critical => risk_dist.critical += 1,
        }
    }

    let total_issues: usize = analyses.iter().map(|a| a.issues.len()).sum();
    let total_recommendations: usize = analyses.iter().map(|a| a.recommendations.len()).sum();
    let average_recommendations = if total_diffs > 0 {
        total_recommendations as f64 / total_diffs as f64
    } else {
        0.0
    };

    RollbackStatistics {
        total_diffs,
        feasible_count,
        infeasible_count,
        complexity_distribution: complexity_dist,
        risk_distribution: risk_dist,
        total_issues,
        average_recommendations,
    }
}

#[cfg(test)]
mod parallel_tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn create_test_statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Test benefit"))
    }

    #[test]
    fn test_parallel_generate_rollbacks() {
        let pairs: Vec<_> = (0..20)
            .map(|i| {
                let id = format!("law{}", i);
                (
                    create_test_statute(&id, "Old Title"),
                    create_test_statute(&id, "New Title"),
                )
            })
            .collect();

        let forward_diffs: Vec<_> = pairs
            .iter()
            .map(|(old, new)| diff(old, new).unwrap())
            .collect();

        let rollbacks = parallel_generate_rollbacks(&forward_diffs);

        assert_eq!(rollbacks.len(), 20);
        for (i, rollback) in rollbacks.iter().enumerate() {
            assert_eq!(rollback.statute_id, format!("law{}", i));
            assert!(!rollback.changes.is_empty());
        }
    }

    #[test]
    fn test_parallel_analyze_rollbacks() {
        let old = create_test_statute("law", "Title").with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        });

        let diffs: Vec<_> = (0..15)
            .map(|i| {
                let mut new = old.clone();
                new.title = format!("New Title {}", i);
                diff(&old, &new).unwrap()
            })
            .collect();

        let analyses = parallel_analyze_rollbacks(&diffs);

        assert_eq!(analyses.len(), 15);
        for analysis in &analyses {
            assert!(analysis.is_feasible);
        }
    }

    #[test]
    fn test_compute_rollback_statistics() {
        let old = create_test_statute("law", "Title");

        // Create diffs with varying complexity
        let mut simple = old.clone();
        simple.title = "Simple Change".to_string();

        let mut complex = old.clone();
        complex.title = "Complex Change".to_string();
        complex.effect = Effect::new(EffectType::Revoke, "Revoke");
        complex.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let diffs = vec![diff(&old, &simple).unwrap(), diff(&old, &complex).unwrap()];

        let analyses = parallel_analyze_rollbacks(&diffs);
        let stats = compute_rollback_statistics(&analyses);

        assert_eq!(stats.total_diffs, 2);
        assert_eq!(stats.feasible_count + stats.infeasible_count, 2);
        assert!(stats.average_recommendations >= 0.0);
    }

    #[test]
    fn test_complexity_distribution() {
        let old = create_test_statute("law", "Title");

        let mut diffs = Vec::new();

        // Simple change
        let mut simple = old.clone();
        simple.title = "Simple".to_string();
        diffs.push(diff(&old, &simple).unwrap());

        // Complex change
        let mut complex = old.clone();
        complex.effect = Effect::new(EffectType::Revoke, "Revoke");
        for i in 0..5 {
            complex.preconditions.push(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18 + i,
            });
        }
        diffs.push(diff(&old, &complex).unwrap());

        let analyses = parallel_analyze_rollbacks(&diffs);
        let stats = compute_rollback_statistics(&analyses);

        assert!(
            stats.complexity_distribution.trivial
                + stats.complexity_distribution.simple
                + stats.complexity_distribution.moderate
                + stats.complexity_distribution.complex
                + stats.complexity_distribution.very_complex
                == 2
        );
    }

    #[test]
    fn test_risk_distribution() {
        let old = create_test_statute("law", "Title");

        // Low risk change
        let mut low_risk = old.clone();
        low_risk.title = "Low Risk".to_string();

        // Higher risk change
        let mut high_risk = old.clone();
        high_risk.effect = Effect::new(EffectType::Revoke, "Revoke");

        let diffs = vec![
            diff(&old, &low_risk).unwrap(),
            diff(&old, &high_risk).unwrap(),
        ];

        let analyses = parallel_analyze_rollbacks(&diffs);
        let stats = compute_rollback_statistics(&analyses);

        assert_eq!(
            stats.risk_distribution.low
                + stats.risk_distribution.medium
                + stats.risk_distribution.high
                + stats.risk_distribution.critical,
            2
        );
    }

    #[test]
    fn test_empty_analyses() {
        let stats = compute_rollback_statistics(&[]);
        assert_eq!(stats.total_diffs, 0);
        assert_eq!(stats.feasible_count, 0);
        assert_eq!(stats.average_recommendations, 0.0);
    }

    #[test]
    fn test_parallel_rollback_chain() {
        let v1 = create_test_statute("law", "V1");
        let v2 = create_test_statute("law", "V2");
        let v3 = create_test_statute("law", "V3");

        let versions = vec![v1, v2, v3];
        let forward_diffs = crate::diff_sequence(&versions).unwrap();

        let rollback_chain = generate_rollback_chain(&forward_diffs);
        let parallel_rollbacks = parallel_generate_rollbacks(&forward_diffs);

        // Both should produce the same number of rollbacks
        assert_eq!(rollback_chain.len(), parallel_rollbacks.len());
    }
}
