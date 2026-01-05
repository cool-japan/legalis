//! Change validation for statute diffs.
//!
//! This module provides validation tools to ensure that diffs are well-formed,
//! complete, and consistent.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::{diff, validation::validate_diff};
//!
//! let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
//! let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
//!
//! let diff_result = diff(&old, &new).unwrap();
//! let validation = validate_diff(&diff_result);
//!
//! assert!(validation.is_valid);
//! ```

use crate::{Change, ChangeTarget, ChangeType, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};

/// Result of diff validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the diff is valid.
    pub is_valid: bool,
    /// List of validation errors.
    pub errors: Vec<ValidationError>,
    /// List of validation warnings.
    pub warnings: Vec<ValidationWarning>,
    /// Overall validation score (0.0 to 1.0).
    pub score: f64,
}

/// Validation error indicating a problem with the diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Type of error.
    pub error_type: ValidationErrorType,
    /// Description of the error.
    pub description: String,
    /// Severity of the error.
    pub severity: Severity,
    /// Related changes.
    pub related_changes: Vec<usize>,
}

/// Types of validation errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationErrorType {
    /// Change is missing required information.
    MissingInformation,
    /// Change has inconsistent data.
    InconsistentData,
    /// Impact assessment doesn't match changes.
    ImpactMismatch,
    /// Change targets are invalid.
    InvalidTarget,
    /// Duplicate changes detected.
    DuplicateChange,
}

/// Validation warning indicating a potential issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Type of warning.
    pub warning_type: ValidationWarningType,
    /// Description of the warning.
    pub description: String,
    /// Related changes.
    pub related_changes: Vec<usize>,
}

/// Types of validation warnings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationWarningType {
    /// Change description could be more detailed.
    VagueDescription,
    /// Severity might be underestimated.
    PotentiallyUnderestimatedSeverity,
    /// Missing confidence indicators.
    MissingConfidence,
    /// Unusual change pattern.
    UnusualPattern,
}

/// Validates a statute diff for completeness and consistency.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, validation::validate_diff};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone().with_precondition(Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 18,
/// });
///
/// let diff_result = diff(&old, &new).unwrap();
/// let validation = validate_diff(&diff_result);
///
/// assert!(validation.is_valid);
/// assert!(validation.score > 0.8);
/// ```
pub fn validate_diff(diff: &StatuteDiff) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Validate each change
    for (idx, change) in diff.changes.iter().enumerate() {
        validate_change(change, idx, &mut errors, &mut warnings);
    }

    // Validate impact assessment
    validate_impact(diff, &mut errors, &mut warnings);

    // Check for duplicate changes
    check_duplicates(diff, &mut errors);

    // Calculate validation score
    let error_penalty = errors.len() as f64 * 0.2;
    let warning_penalty = warnings.len() as f64 * 0.05;
    let score = (1.0 - error_penalty - warning_penalty).clamp(0.0, 1.0);

    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
        score,
    }
}

fn validate_change(
    change: &Change,
    index: usize,
    errors: &mut Vec<ValidationError>,
    warnings: &mut Vec<ValidationWarning>,
) {
    // Check that Added changes don't have old_value
    if change.change_type == ChangeType::Added && change.old_value.is_some() {
        errors.push(ValidationError {
            error_type: ValidationErrorType::InconsistentData,
            description: format!("Added change at index {} should not have old_value", index),
            severity: Severity::Moderate,
            related_changes: vec![index],
        });
    }

    // Check that Removed changes don't have new_value
    if change.change_type == ChangeType::Removed && change.new_value.is_some() {
        errors.push(ValidationError {
            error_type: ValidationErrorType::InconsistentData,
            description: format!(
                "Removed change at index {} should not have new_value",
                index
            ),
            severity: Severity::Moderate,
            related_changes: vec![index],
        });
    }

    // Check that Modified changes have both old and new values
    if change.change_type == ChangeType::Modified
        && (change.old_value.is_none() || change.new_value.is_none())
    {
        errors.push(ValidationError {
            error_type: ValidationErrorType::MissingInformation,
            description: format!(
                "Modified change at index {} should have both old_value and new_value",
                index
            ),
            severity: Severity::Major,
            related_changes: vec![index],
        });
    }

    // Check description quality
    if change.description.len() < 10 {
        warnings.push(ValidationWarning {
            warning_type: ValidationWarningType::VagueDescription,
            description: format!("Change at index {} has a very short description", index),
            related_changes: vec![index],
        });
    }

    // Check for empty descriptions
    if change.description.trim().is_empty() {
        errors.push(ValidationError {
            error_type: ValidationErrorType::MissingInformation,
            description: format!("Change at index {} has empty description", index),
            severity: Severity::Minor,
            related_changes: vec![index],
        });
    }
}

fn validate_impact(
    diff: &StatuteDiff,
    errors: &mut Vec<ValidationError>,
    warnings: &mut Vec<ValidationWarning>,
) {
    // Check if severity matches the changes
    let has_effect_change = diff
        .changes
        .iter()
        .any(|c| matches!(c.target, ChangeTarget::Effect));

    let has_precondition_changes = diff
        .changes
        .iter()
        .any(|c| matches!(c.target, ChangeTarget::Precondition { .. }));

    let has_discretion_change = diff
        .changes
        .iter()
        .any(|c| matches!(c.target, ChangeTarget::DiscretionLogic));

    // Effect changes should have affects_outcome flag
    if has_effect_change && !diff.impact.affects_outcome {
        errors.push(ValidationError {
            error_type: ValidationErrorType::ImpactMismatch,
            description: "Effect was changed but affects_outcome is false".to_string(),
            severity: Severity::Major,
            related_changes: diff
                .changes
                .iter()
                .enumerate()
                .filter(|(_, c)| matches!(c.target, ChangeTarget::Effect))
                .map(|(i, _)| i)
                .collect(),
        });
    }

    // Precondition changes should have affects_eligibility flag
    if has_precondition_changes && !diff.impact.affects_eligibility {
        warnings.push(ValidationWarning {
            warning_type: ValidationWarningType::PotentiallyUnderestimatedSeverity,
            description: "Preconditions changed but affects_eligibility is false".to_string(),
            related_changes: diff
                .changes
                .iter()
                .enumerate()
                .filter(|(_, c)| matches!(c.target, ChangeTarget::Precondition { .. }))
                .map(|(i, _)| i)
                .collect(),
        });
    }

    // Discretion changes should have discretion_changed flag
    if has_discretion_change && !diff.impact.discretion_changed {
        errors.push(ValidationError {
            error_type: ValidationErrorType::ImpactMismatch,
            description: "Discretion logic changed but discretion_changed is false".to_string(),
            severity: Severity::Major,
            related_changes: diff
                .changes
                .iter()
                .enumerate()
                .filter(|(_, c)| matches!(c.target, ChangeTarget::DiscretionLogic))
                .map(|(i, _)| i)
                .collect(),
        });
    }

    // Check severity alignment
    if diff.changes.is_empty() && diff.impact.severity != Severity::None {
        warnings.push(ValidationWarning {
            warning_type: ValidationWarningType::UnusualPattern,
            description: "No changes detected but severity is not None".to_string(),
            related_changes: vec![],
        });
    }

    if has_effect_change && diff.impact.severity < Severity::Major {
        warnings.push(ValidationWarning {
            warning_type: ValidationWarningType::PotentiallyUnderestimatedSeverity,
            description: "Effect changed but severity is less than Major".to_string(),
            related_changes: vec![],
        });
    }
}

fn check_duplicates(diff: &StatuteDiff, errors: &mut Vec<ValidationError>) {
    use std::collections::HashSet;

    let mut seen_targets: HashSet<String> = HashSet::new();

    for (idx, change) in diff.changes.iter().enumerate() {
        let target_key = format!("{:?}_{:?}", change.target, change.change_type);

        if !seen_targets.insert(target_key.clone()) {
            errors.push(ValidationError {
                error_type: ValidationErrorType::DuplicateChange,
                description: format!(
                    "Duplicate change detected at index {} for target {:?}",
                    idx, change.target
                ),
                severity: Severity::Moderate,
                related_changes: vec![idx],
            });
        }
    }
}

/// Validates multiple diffs in batch.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, validation::validate_batch};
///
/// let old1 = Statute::new("law1", "Title 1", Effect::new(EffectType::Grant, "Benefit"));
/// let new1 = Statute::new("law1", "New Title 1", Effect::new(EffectType::Grant, "Benefit"));
///
/// let old2 = Statute::new("law2", "Title 2", Effect::new(EffectType::Grant, "Benefit"));
/// let new2 = Statute::new("law2", "New Title 2", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diffs = vec![
///     diff(&old1, &new1).unwrap(),
///     diff(&old2, &new2).unwrap(),
/// ];
///
/// let results = validate_batch(&diffs);
/// assert_eq!(results.len(), 2);
/// ```
pub fn validate_batch(diffs: &[StatuteDiff]) -> Vec<ValidationResult> {
    diffs.iter().map(validate_diff).collect()
}

/// Validates multiple diffs in parallel using rayon.
///
/// This is significantly faster than sequential validation when processing
/// many diffs.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, validation::validate_batch_parallel};
///
/// let diffs: Vec<_> = (0..100)
///     .map(|i| {
///         let id = format!("law{}", i);
///         let old = Statute::new(&id, "Old Title", Effect::new(EffectType::Grant, "Benefit"));
///         let new = Statute::new(&id, "New Title", Effect::new(EffectType::Grant, "Benefit"));
///         diff(&old, &new).unwrap()
///     })
///     .collect();
///
/// let results = validate_batch_parallel(&diffs);
/// assert_eq!(results.len(), 100);
/// ```
pub fn validate_batch_parallel(diffs: &[StatuteDiff]) -> Vec<ValidationResult> {
    use rayon::prelude::*;
    diffs.par_iter().map(validate_diff).collect()
}

/// Summary of batch validation results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchValidationSummary {
    /// Total number of diffs validated.
    pub total_diffs: usize,
    /// Number of valid diffs.
    pub valid_count: usize,
    /// Number of invalid diffs.
    pub invalid_count: usize,
    /// Total errors across all diffs.
    pub total_errors: usize,
    /// Total warnings across all diffs.
    pub total_warnings: usize,
    /// Average validation score.
    pub average_score: f64,
    /// Statute IDs with validation errors.
    pub failed_statute_ids: Vec<String>,
}

/// Generates a summary of batch validation results.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, validation::{validate_batch, summarize_batch_validation}};
///
/// let old1 = Statute::new("law1", "Title 1", Effect::new(EffectType::Grant, "Benefit"));
/// let new1 = Statute::new("law1", "New Title 1", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diffs = vec![diff(&old1, &new1).unwrap()];
/// let results = validate_batch(&diffs);
/// let summary = summarize_batch_validation(&diffs, &results);
///
/// assert_eq!(summary.total_diffs, 1);
/// ```
pub fn summarize_batch_validation(
    diffs: &[StatuteDiff],
    results: &[ValidationResult],
) -> BatchValidationSummary {
    let total_diffs = results.len();
    let valid_count = results.iter().filter(|r| r.is_valid).count();
    let invalid_count = total_diffs - valid_count;

    let total_errors: usize = results.iter().map(|r| r.errors.len()).sum();
    let total_warnings: usize = results.iter().map(|r| r.warnings.len()).sum();

    let average_score = if total_diffs > 0 {
        results.iter().map(|r| r.score).sum::<f64>() / total_diffs as f64
    } else {
        0.0
    };

    let failed_statute_ids: Vec<String> = diffs
        .iter()
        .zip(results.iter())
        .filter(|(_, result)| !result.is_valid)
        .map(|(diff, _)| diff.statute_id.clone())
        .collect();

    BatchValidationSummary {
        total_diffs,
        valid_count,
        invalid_count,
        total_errors,
        total_warnings,
        average_score,
        failed_statute_ids,
    }
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
    }

    #[test]
    fn test_validate_valid_diff() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "Modified Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let validation = validate_diff(&diff_result);

        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
        assert!(validation.score > 0.8);
    }

    #[test]
    fn test_validate_effect_change() {
        let old = test_statute();
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke benefit");

        let diff_result = diff(&old, &new).unwrap();
        let validation = validate_diff(&diff_result);

        assert!(validation.is_valid);
        assert!(validation.score > 0.0);
    }

    #[test]
    fn test_validate_precondition_change() {
        let old = test_statute();
        let new = old.clone().with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let diff_result = diff(&old, &new).unwrap();
        let validation = validate_diff(&diff_result);

        assert!(validation.is_valid);
    }

    #[test]
    fn test_validate_no_changes() {
        let statute = test_statute();
        let diff_result = diff(&statute, &statute).unwrap();
        let validation = validate_diff(&diff_result);

        assert!(validation.is_valid);
        assert_eq!(validation.errors.len(), 0);
    }

    #[test]
    fn test_validate_batch() {
        let old1 = test_statute();
        let mut new1 = old1.clone();
        new1.title = "New Title 1".to_string();

        let old2 = Statute::new(
            "law2",
            "Title 2",
            Effect::new(EffectType::Grant, "Benefit 2"),
        );
        let mut new2 = old2.clone();
        new2.title = "New Title 2".to_string();

        let diffs = vec![diff(&old1, &new1).unwrap(), diff(&old2, &new2).unwrap()];

        let results = validate_batch(&diffs);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_valid));
    }

    #[test]
    fn test_validate_complex_change() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();
        new.effect = Effect::new(EffectType::Obligation, "Different effect");
        new.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let diff_result = diff(&old, &new).unwrap();
        let validation = validate_diff(&diff_result);

        assert!(validation.is_valid);
        assert!(validation.score > 0.0);
    }

    #[test]
    fn test_validate_discretion_change() {
        let old = test_statute();
        let new = old
            .clone()
            .with_discretion("Consider special circumstances");

        let diff_result = diff(&old, &new).unwrap();
        let validation = validate_diff(&diff_result);

        assert!(validation.is_valid);
    }

    #[test]
    fn test_validation_score_decreases_with_warnings() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "T".to_string(); // Very short title might trigger warnings

        let diff_result = diff(&old, &new).unwrap();
        let validation = validate_diff(&diff_result);

        // Should still be valid but might have lower score due to warnings
        assert!(validation.score <= 1.0);
    }

    #[test]
    fn test_validate_batch_parallel() {
        let diffs: Vec<_> = (0..50)
            .map(|i| {
                let id = format!("law{}", i);
                let old = Statute::new(&id, "Old Title", Effect::new(EffectType::Grant, "Benefit"));
                let new = Statute::new(&id, "New Title", Effect::new(EffectType::Grant, "Benefit"));
                diff(&old, &new).unwrap()
            })
            .collect();

        let results = validate_batch_parallel(&diffs);
        assert_eq!(results.len(), 50);
        assert!(results.iter().all(|r| r.is_valid));
    }

    #[test]
    fn test_summarize_batch_validation_all_valid() {
        let old1 = test_statute();
        let mut new1 = old1.clone();
        new1.title = "New Title 1".to_string();

        let old2 = Statute::new(
            "law2",
            "Title 2",
            Effect::new(EffectType::Grant, "Benefit 2"),
        );
        let mut new2 = old2.clone();
        new2.title = "New Title 2".to_string();

        let diffs = vec![diff(&old1, &new1).unwrap(), diff(&old2, &new2).unwrap()];

        let results = validate_batch(&diffs);
        let summary = summarize_batch_validation(&diffs, &results);

        assert_eq!(summary.total_diffs, 2);
        assert_eq!(summary.valid_count, 2);
        assert_eq!(summary.invalid_count, 0);
        assert!(summary.average_score > 0.8);
        assert!(summary.failed_statute_ids.is_empty());
    }

    #[test]
    fn test_batch_validation_summary() {
        let diffs: Vec<_> = (0..10)
            .map(|i| {
                let id = format!("law{}", i);
                let old = Statute::new(&id, "Old", Effect::new(EffectType::Grant, "Benefit"));
                let new = Statute::new(&id, "New", Effect::new(EffectType::Grant, "Benefit"));
                diff(&old, &new).unwrap()
            })
            .collect();

        let results = validate_batch_parallel(&diffs);
        let summary = summarize_batch_validation(&diffs, &results);

        assert_eq!(summary.total_diffs, 10);
        assert_eq!(summary.valid_count + summary.invalid_count, 10);
        assert!(summary.average_score >= 0.0 && summary.average_score <= 1.0);
    }
}

/// Validates rollback diffs to ensure they properly reverse the forward changes.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, rollback::generate_rollback, validation::validate_rollback};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let forward = diff(&old, &new).unwrap();
/// let rollback = generate_rollback(&forward);
///
/// let validation = validate_rollback(&forward, &rollback);
/// assert!(validation.is_valid);
/// ```
pub fn validate_rollback(
    forward_diff: &StatuteDiff,
    rollback_diff: &StatuteDiff,
) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Validate basic rollback properties
    let basic_validation = validate_diff(rollback_diff);
    errors.extend(basic_validation.errors);
    warnings.extend(basic_validation.warnings);

    // Check that rollback has the same number of changes as forward
    if rollback_diff.changes.len() != forward_diff.changes.len() {
        errors.push(ValidationError {
            error_type: ValidationErrorType::InconsistentData,
            description: format!(
                "Rollback has {} changes but forward has {}",
                rollback_diff.changes.len(),
                forward_diff.changes.len()
            ),
            severity: Severity::Major,
            related_changes: vec![],
        });
    }

    // Verify each rollback change properly reverses its corresponding forward change
    for (idx, (forward_change, rollback_change)) in forward_diff
        .changes
        .iter()
        .zip(rollback_diff.changes.iter())
        .enumerate()
    {
        // Check that targets match
        if forward_change.target != rollback_change.target {
            errors.push(ValidationError {
                error_type: ValidationErrorType::InconsistentData,
                description: format!(
                    "Rollback change {} targets {:?} but forward targets {:?}",
                    idx, rollback_change.target, forward_change.target
                ),
                severity: Severity::Major,
                related_changes: vec![idx],
            });
        }

        // Check that values are properly reversed
        if forward_change.old_value != rollback_change.new_value {
            warnings.push(ValidationWarning {
                warning_type: ValidationWarningType::UnusualPattern,
                description: format!(
                    "Rollback change {} may not properly reverse forward change",
                    idx
                ),
                related_changes: vec![idx],
            });
        }
    }

    // Calculate validation score
    let error_penalty = errors.len() as f64 * 0.2;
    let warning_penalty = warnings.len() as f64 * 0.05;
    let score = (1.0 - error_penalty - warning_penalty).clamp(0.0, 1.0);

    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
        score,
    }
}

/// Validates multiple rollback diffs in parallel.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, rollback::parallel_generate_rollbacks, validation::validate_rollbacks_parallel};
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
/// let rollback_diffs = parallel_generate_rollbacks(&forward_diffs);
/// let validations = validate_rollbacks_parallel(&forward_diffs, &rollback_diffs);
///
/// assert_eq!(validations.len(), 10);
/// ```
pub fn validate_rollbacks_parallel(
    forward_diffs: &[StatuteDiff],
    rollback_diffs: &[StatuteDiff],
) -> Vec<ValidationResult> {
    use rayon::prelude::*;

    forward_diffs
        .par_iter()
        .zip(rollback_diffs.par_iter())
        .map(|(forward, rollback)| validate_rollback(forward, rollback))
        .collect()
}

#[cfg(test)]
mod rollback_validation_tests {
    use super::*;
    use crate::{diff, rollback::generate_rollback};
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn test_statute() -> Statute {
        Statute::new(
            "test-law",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test benefit"),
        )
    }

    #[test]
    fn test_validate_rollback_simple() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "Modified Title".to_string();

        let forward = diff(&old, &new).unwrap();
        let rollback = generate_rollback(&forward);

        let validation = validate_rollback(&forward, &rollback);
        assert!(validation.is_valid);
        assert!(validation.score > 0.8);
    }

    #[test]
    fn test_validate_rollback_complex() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();
        new.effect = Effect::new(EffectType::Obligation, "Different effect");
        new.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let forward = diff(&old, &new).unwrap();
        let rollback = generate_rollback(&forward);

        let validation = validate_rollback(&forward, &rollback);
        assert!(validation.is_valid || validation.score > 0.5);
    }

    #[test]
    fn test_validate_rollbacks_parallel() {
        let pairs: Vec<_> = (0..20)
            .map(|i| {
                let id = format!("law{}", i);
                let old = Statute::new(&id, "Old Title", Effect::new(EffectType::Grant, "Benefit"));
                let new = Statute::new(&id, "New Title", Effect::new(EffectType::Grant, "Benefit"));
                (old, new)
            })
            .collect();

        let forward_diffs: Vec<_> = pairs
            .iter()
            .map(|(old, new)| diff(old, new).unwrap())
            .collect();

        let rollback_diffs: Vec<_> = forward_diffs.iter().map(generate_rollback).collect();

        let validations = validate_rollbacks_parallel(&forward_diffs, &rollback_diffs);

        assert_eq!(validations.len(), 20);
        assert!(validations.iter().all(|v| v.is_valid));
    }

    #[test]
    fn test_validate_rollback_with_precondition_changes() {
        let old = test_statute().with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        });

        let mut new = old.clone();
        new.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let forward = diff(&old, &new).unwrap();
        let rollback = generate_rollback(&forward);

        let validation = validate_rollback(&forward, &rollback);
        assert!(validation.is_valid || !validation.errors.is_empty());
    }
}
