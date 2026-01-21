//! Semantic diff: Understanding meaning changes in statutes.
//!
//! This module goes beyond structural differences to understand the
//! semantic meaning and legal implications of statute changes.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_diff::{diff, semantic::analyze_semantic_diff};
//!
//! let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
//!     .with_precondition(Condition::Age {
//!         operator: ComparisonOp::GreaterOrEqual,
//!         value: 21,
//!     });
//!
//! let mut new = old.clone();
//! new.preconditions[0] = Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 18, // Relaxed age requirement
//! };
//!
//! let structural = diff(&old, &new).unwrap();
//! let semantic = analyze_semantic_diff(&structural);
//!
//! // Detects that this is a scope expansion
//! assert!(semantic.semantic_impact.eligibility_broadened);
//! ```

use crate::{Change, ChangeTarget, ChangeType, StatuteDiff};
use legalis_core::{Condition, Effect, Statute};
use serde::{Deserialize, Serialize};

/// Semantic change classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticChangeType {
    /// Changes that preserve the same legal meaning (e.g., rewording).
    MeaningPreserving,
    /// Changes that broaden eligibility or scope.
    ScopeExpansion,
    /// Changes that narrow eligibility or scope.
    ScopeReduction,
    /// Changes that fundamentally alter the legal intent.
    IntentChange,
    /// Changes that clarify ambiguous language without changing meaning.
    Clarification,
    /// Technical or administrative corrections.
    TechnicalCorrection,
}

/// Semantic analysis of a statute diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticDiff {
    /// The original structural diff.
    pub structural_diff: StatuteDiff,
    /// Semantic classification of changes.
    pub semantic_changes: Vec<SemanticChange>,
    /// Overall semantic impact.
    pub semantic_impact: SemanticImpact,
    /// Detected legal patterns.
    pub patterns: Vec<LegalPattern>,
    /// Quantified metrics for condition changes.
    pub relaxation_metrics: RelaxationMetrics,
    /// Effect scope quantification.
    pub scope_metrics: ScopeMetrics,
    /// Breaking change classification.
    pub breaking_classification: BreakingChangeClassification,
}

/// A single semantic change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticChange {
    /// The structural change.
    pub structural_change: Change,
    /// Semantic classification.
    pub semantic_type: SemanticChangeType,
    /// Explanation of the semantic meaning.
    pub explanation: String,
    /// Confidence in the classification (0.0 to 1.0).
    pub confidence: f64,
}

/// Overall semantic impact assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticImpact {
    /// Whether the legal intent fundamentally changed.
    pub intent_changed: bool,
    /// Whether eligibility was broadened.
    pub eligibility_broadened: bool,
    /// Whether eligibility was narrowed.
    pub eligibility_narrowed: bool,
    /// Whether the change is primarily clarifying.
    pub primarily_clarifying: bool,
    /// Aggregate confidence score.
    pub overall_confidence: f64,
}

/// Detected legal patterns in changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalPattern {
    /// Pattern name.
    pub name: String,
    /// Pattern description.
    pub description: String,
    /// Related changes.
    pub related_changes: Vec<usize>,
}

/// Quantified metrics for condition relaxation/tightening.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelaxationMetrics {
    /// Total number of conditions relaxed.
    pub relaxed_count: usize,
    /// Total number of conditions tightened.
    pub tightened_count: usize,
    /// Net relaxation score (-1.0 = fully tightened, +1.0 = fully relaxed).
    pub net_score: f64,
    /// Quantified impact per condition.
    pub per_condition_impact: Vec<ConditionImpact>,
}

/// Impact of a single condition change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionImpact {
    /// Index of the condition.
    pub index: usize,
    /// Relaxation score for this condition (-1.0 to +1.0).
    pub relaxation_score: f64,
    /// Estimated percentage change in eligible population.
    pub estimated_population_change: f64,
    /// Description of the impact.
    pub description: String,
}

/// Quantified metrics for effect scope changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeMetrics {
    /// Whether the effect scope changed.
    pub scope_changed: bool,
    /// Scope change magnitude (0.0 to 1.0).
    pub magnitude: f64,
    /// Direction of scope change.
    pub direction: ScopeDirection,
    /// Description of the scope change.
    pub description: String,
}

/// Direction of scope change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScopeDirection {
    /// No change in scope.
    Unchanged,
    /// Scope expanded.
    Expanded,
    /// Scope contracted.
    Contracted,
    /// Mixed (some expanded, some contracted).
    Mixed,
}

/// Breaking change classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChangeClassification {
    /// Whether this is a breaking change.
    pub is_breaking: bool,
    /// Breaking change level.
    pub level: BreakingLevel,
    /// Specific breaking change types detected.
    pub breaking_types: Vec<BreakingType>,
    /// Backward compatibility score (0.0 to 1.0, where 1.0 is fully compatible).
    pub compatibility_score: f64,
    /// Migration difficulty (0.0 to 1.0, where 1.0 is extremely difficult).
    pub migration_difficulty: f64,
}

/// Breaking change severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BreakingLevel {
    /// No breaking changes.
    None,
    /// Minor breaking changes (edge cases).
    Minor,
    /// Moderate breaking changes (some users affected).
    Moderate,
    /// Major breaking changes (many users affected).
    Major,
    /// Critical breaking changes (all users affected).
    Critical,
}

/// Types of breaking changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakingType {
    /// Effect type changed (e.g., Grant to Revoke).
    EffectTypeChange,
    /// New mandatory preconditions added.
    NewMandatoryConditions,
    /// Existing conditions tightened.
    TightenedConditions,
    /// Discretion requirements changed.
    DiscretionChange,
    /// Outcome fundamentally altered.
    OutcomeChange,
    /// Legal intent changed.
    IntentChange,
}

/// Performs semantic analysis on a statute diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, semantic::analyze_semantic_diff};
///
/// let old = Statute::new(
///     "benefit",
///     "Old Title",
///     Effect::new(EffectType::Grant, "Benefit"),
/// ).with_precondition(Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 18,
/// });
///
/// let mut new = old.clone();
/// new.preconditions.clear(); // Removing condition = scope expansion
///
/// let structural_diff = diff(&old, &new).unwrap();
/// let semantic = analyze_semantic_diff(&structural_diff);
///
/// assert!(semantic.semantic_impact.eligibility_broadened);
/// ```
pub fn analyze_semantic_diff(diff: &StatuteDiff) -> SemanticDiff {
    let mut semantic_changes = Vec::new();
    let mut patterns = Vec::new();

    // Analyze each change semantically
    for change in &diff.changes {
        semantic_changes.push(analyze_semantic_change(change));
    }

    // Detect patterns
    patterns.extend(detect_eligibility_patterns(&semantic_changes));
    patterns.extend(detect_clarification_patterns(&semantic_changes));
    patterns.extend(detect_scope_patterns(&semantic_changes));

    // Compute overall impact
    let semantic_impact = compute_semantic_impact(&semantic_changes);

    // Compute relaxation metrics
    let relaxation_metrics = compute_relaxation_metrics(&diff.changes);

    // Compute scope metrics
    let scope_metrics = compute_scope_metrics(&diff.changes, &semantic_changes);

    // Classify breaking changes
    let breaking_classification = classify_breaking_changes(diff, &semantic_changes);

    SemanticDiff {
        structural_diff: diff.clone(),
        semantic_changes,
        semantic_impact,
        patterns,
        relaxation_metrics,
        scope_metrics,
        breaking_classification,
    }
}

/// Analyzes the semantic meaning of a single change.
fn analyze_semantic_change(change: &Change) -> SemanticChange {
    match &change.target {
        ChangeTarget::Title => {
            // Title changes are usually clarifications or technical corrections
            let is_minor = change.old_value.as_ref().zip(change.new_value.as_ref())
                .map(|(old, new)| {
                    // Check if it's just case or punctuation changes
                    old.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>() ==
                    new.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>()
                })
                .unwrap_or(false);

            if is_minor {
                SemanticChange {
                    structural_change: change.clone(),
                    semantic_type: SemanticChangeType::TechnicalCorrection,
                    explanation: "Minor formatting or punctuation change to title".to_string(),
                    confidence: 0.95,
                }
            } else {
                SemanticChange {
                    structural_change: change.clone(),
                    semantic_type: SemanticChangeType::Clarification,
                    explanation: "Title updated to better reflect statute's purpose".to_string(),
                    confidence: 0.7,
                }
            }
        }

        ChangeTarget::Precondition { .. } => analyze_precondition_semantic(change),

        ChangeTarget::Effect => {
            // Effect changes are fundamental intent changes
            let changed_type = change.old_value.as_ref().zip(change.new_value.as_ref())
                .map(|(old, new)| old.contains("Grant") != new.contains("Grant") ||
                                  old.contains("Revoke") != new.contains("Revoke"))
                .unwrap_or(true);

            if changed_type {
                SemanticChange {
                    structural_change: change.clone(),
                    semantic_type: SemanticChangeType::IntentChange,
                    explanation: "Effect type changed, fundamentally altering the statute's purpose".to_string(),
                    confidence: 0.98,
                }
            } else {
                SemanticChange {
                    structural_change: change.clone(),
                    semantic_type: SemanticChangeType::Clarification,
                    explanation: "Effect description updated without changing outcome type".to_string(),
                    confidence: 0.8,
                }
            }
        }

        ChangeTarget::DiscretionLogic => {
            match change.change_type {
                ChangeType::Added => SemanticChange {
                    structural_change: change.clone(),
                    semantic_type: SemanticChangeType::IntentChange,
                    explanation: "Added human discretion requirement, changing from deterministic to judgment-based".to_string(),
                    confidence: 0.95,
                },
                ChangeType::Removed => SemanticChange {
                    structural_change: change.clone(),
                    semantic_type: SemanticChangeType::IntentChange,
                    explanation: "Removed human discretion, making the statute fully deterministic".to_string(),
                    confidence: 0.95,
                },
                ChangeType::Modified => SemanticChange {
                    structural_change: change.clone(),
                    semantic_type: SemanticChangeType::Clarification,
                    explanation: "Discretion criteria updated, adjusting judgment guidelines".to_string(),
                    confidence: 0.75,
                },
                ChangeType::Reordered => SemanticChange {
                    structural_change: change.clone(),
                    semantic_type: SemanticChangeType::TechnicalCorrection,
                    explanation: "Discretion logic reordered without semantic change".to_string(),
                    confidence: 0.6,
                },
            }
        }

        ChangeTarget::Metadata { .. } => SemanticChange {
            structural_change: change.clone(),
            semantic_type: SemanticChangeType::TechnicalCorrection,
            explanation: "Metadata update with no legal impact".to_string(),
            confidence: 0.99,
        },
    }
}

/// Analyzes semantic meaning of precondition changes.
fn analyze_precondition_semantic(change: &Change) -> SemanticChange {
    match change.change_type {
        ChangeType::Added => SemanticChange {
            structural_change: change.clone(),
            semantic_type: SemanticChangeType::ScopeReduction,
            explanation: "New precondition added, reducing eligible population".to_string(),
            confidence: 0.9,
        },

        ChangeType::Removed => SemanticChange {
            structural_change: change.clone(),
            semantic_type: SemanticChangeType::ScopeExpansion,
            explanation: "Precondition removed, expanding eligible population".to_string(),
            confidence: 0.9,
        },

        ChangeType::Modified => {
            // Try to detect if this is a threshold change
            if let (Some(old), Some(new)) = (&change.old_value, &change.new_value) {
                if let Some((old_val, new_val, is_relaxed)) = detect_threshold_change(old, new) {
                    if is_relaxed {
                        SemanticChange {
                            structural_change: change.clone(),
                            semantic_type: SemanticChangeType::ScopeExpansion,
                            explanation: format!(
                                "Threshold relaxed from {} to {}, broadening eligibility",
                                old_val, new_val
                            ),
                            confidence: 0.85,
                        }
                    } else {
                        SemanticChange {
                            structural_change: change.clone(),
                            semantic_type: SemanticChangeType::ScopeReduction,
                            explanation: format!(
                                "Threshold tightened from {} to {}, narrowing eligibility",
                                old_val, new_val
                            ),
                            confidence: 0.85,
                        }
                    }
                } else {
                    SemanticChange {
                        structural_change: change.clone(),
                        semantic_type: SemanticChangeType::Clarification,
                        explanation: "Precondition modified, impact depends on specific changes"
                            .to_string(),
                        confidence: 0.5,
                    }
                }
            } else {
                SemanticChange {
                    structural_change: change.clone(),
                    semantic_type: SemanticChangeType::Clarification,
                    explanation: "Precondition changed".to_string(),
                    confidence: 0.4,
                }
            }
        }

        ChangeType::Reordered => SemanticChange {
            structural_change: change.clone(),
            semantic_type: SemanticChangeType::MeaningPreserving,
            explanation: "Precondition order changed without affecting logic".to_string(),
            confidence: 0.8,
        },
    }
}

/// Detects if a change represents a threshold modification.
fn detect_threshold_change(old: &str, new: &str) -> Option<(String, String, bool)> {
    // Look for numeric values in the strings
    let old_num = extract_number(old);
    let new_num = extract_number(new);

    if let (Some(old_val), Some(new_val)) = (old_num, new_num) {
        // Determine if this is a relaxation or tightening
        // For age: lower age = relaxation (GreaterOrEqual)
        // For income: higher income = relaxation (LessOrEqual)
        let is_upper_bound = old.contains("LessOrEqual") || old.contains("Less");
        let is_relaxed = if is_upper_bound {
            new_val > old_val // Higher upper bound = relaxed
        } else {
            new_val < old_val // Lower lower bound = relaxed
        };

        Some((old_val.to_string(), new_val.to_string(), is_relaxed))
    } else {
        None
    }
}

/// Extracts a numeric value from a string.
fn extract_number(s: &str) -> Option<i64> {
    s.split_whitespace()
        .find_map(|word| word.parse::<i64>().ok())
}

/// Detects eligibility expansion/reduction patterns.
fn detect_eligibility_patterns(changes: &[SemanticChange]) -> Vec<LegalPattern> {
    let mut patterns = Vec::new();

    // Check if multiple changes expand eligibility
    let expansion_indices: Vec<usize> = changes
        .iter()
        .enumerate()
        .filter(|(_, c)| c.semantic_type == SemanticChangeType::ScopeExpansion)
        .map(|(i, _)| i)
        .collect();

    if expansion_indices.len() > 1 {
        patterns.push(LegalPattern {
            name: "Coordinated Eligibility Expansion".to_string(),
            description: "Multiple changes work together to broaden who qualifies".to_string(),
            related_changes: expansion_indices,
        });
    }

    // Check if multiple changes reduce eligibility
    let reduction_indices: Vec<usize> = changes
        .iter()
        .enumerate()
        .filter(|(_, c)| c.semantic_type == SemanticChangeType::ScopeReduction)
        .map(|(i, _)| i)
        .collect();

    if reduction_indices.len() > 1 {
        patterns.push(LegalPattern {
            name: "Coordinated Eligibility Restriction".to_string(),
            description: "Multiple changes work together to narrow who qualifies".to_string(),
            related_changes: reduction_indices,
        });
    }

    patterns
}

/// Detects clarification patterns.
fn detect_clarification_patterns(changes: &[SemanticChange]) -> Vec<LegalPattern> {
    let mut patterns = Vec::new();

    let clarification_indices: Vec<usize> = changes
        .iter()
        .enumerate()
        .filter(|(_, c)| c.semantic_type == SemanticChangeType::Clarification)
        .map(|(i, _)| i)
        .collect();

    if clarification_indices.len() >= changes.len() / 2 && !clarification_indices.is_empty() {
        patterns.push(LegalPattern {
            name: "Clarification Amendment".to_string(),
            description: "Changes primarily clarify existing intent without altering legal effect"
                .to_string(),
            related_changes: clarification_indices,
        });
    }

    patterns
}

/// Detects scope change patterns.
fn detect_scope_patterns(changes: &[SemanticChange]) -> Vec<LegalPattern> {
    let mut patterns = Vec::new();

    let has_expansion = changes
        .iter()
        .any(|c| c.semantic_type == SemanticChangeType::ScopeExpansion);
    let has_reduction = changes
        .iter()
        .any(|c| c.semantic_type == SemanticChangeType::ScopeReduction);

    if has_expansion && has_reduction {
        let related: Vec<usize> = changes
            .iter()
            .enumerate()
            .filter(|(_, c)| {
                c.semantic_type == SemanticChangeType::ScopeExpansion
                    || c.semantic_type == SemanticChangeType::ScopeReduction
            })
            .map(|(i, _)| i)
            .collect();

        patterns.push(LegalPattern {
            name: "Mixed Scope Adjustment".to_string(),
            description: "Changes both expand and restrict eligibility, likely targeting different populations".to_string(),
            related_changes: related,
        });
    }

    patterns
}

/// Computes overall semantic impact.
fn compute_semantic_impact(changes: &[SemanticChange]) -> SemanticImpact {
    let intent_changed = changes
        .iter()
        .any(|c| c.semantic_type == SemanticChangeType::IntentChange);

    let eligibility_broadened = changes
        .iter()
        .any(|c| c.semantic_type == SemanticChangeType::ScopeExpansion);

    let eligibility_narrowed = changes
        .iter()
        .any(|c| c.semantic_type == SemanticChangeType::ScopeReduction);

    let clarifying_count = changes
        .iter()
        .filter(|c| c.semantic_type == SemanticChangeType::Clarification)
        .count();

    let primarily_clarifying = if !changes.is_empty() {
        clarifying_count as f64 / changes.len() as f64 > 0.5
    } else {
        false
    };

    let overall_confidence = if !changes.is_empty() {
        changes.iter().map(|c| c.confidence).sum::<f64>() / changes.len() as f64
    } else {
        1.0
    };

    SemanticImpact {
        intent_changed,
        eligibility_broadened,
        eligibility_narrowed,
        primarily_clarifying,
        overall_confidence,
    }
}

/// Checks if two statutes are semantically equivalent.
///
/// Two statutes are considered semantically equivalent if they have the same
/// legal effect despite potential differences in wording or titles.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::semantic::are_semantically_equivalent;
///
/// let statute1 = Statute::new("law", "Title A", Effect::new(EffectType::Grant, "Benefit"));
/// let mut statute2 = statute1.clone();
/// statute2.title = "Title B".to_string(); // Different title, same semantics
///
/// assert!(are_semantically_equivalent(&statute1, &statute2));
/// ```
pub fn are_semantically_equivalent(old: &Statute, new: &Statute) -> bool {
    // Check if statutes have the same ID
    if old.id != new.id {
        return false;
    }

    // Check if effects are semantically equivalent
    if !effects_equivalent(&old.effect, &new.effect) {
        return false;
    }

    // Check if preconditions are semantically equivalent
    if !preconditions_equivalent(&old.preconditions, &new.preconditions) {
        return false;
    }

    // Check discretion
    if old.discretion_logic != new.discretion_logic {
        return false;
    }

    true
}

/// Checks if two effects are semantically equivalent.
fn effects_equivalent(old: &Effect, new: &Effect) -> bool {
    // Effects are equivalent if they have the same type
    // (description differences are considered semantic preserving)
    old.effect_type == new.effect_type
}

/// Checks if two sets of preconditions are semantically equivalent.
fn preconditions_equivalent(old: &[Condition], new: &[Condition]) -> bool {
    // For now, we do structural equivalence
    // Future: could detect logically equivalent reformulations
    if old.len() != new.len() {
        return false;
    }

    // Check if all conditions match (order independent)
    for old_cond in old {
        if !new.contains(old_cond) {
            return false;
        }
    }

    true
}

/// Computes relaxation/tightening metrics for condition changes.
fn compute_relaxation_metrics(changes: &[Change]) -> RelaxationMetrics {
    let mut relaxed_count = 0;
    let mut tightened_count = 0;
    let mut per_condition_impact = Vec::new();

    for change in changes.iter() {
        if let ChangeTarget::Precondition { index } = &change.target {
            let impact = match change.change_type {
                ChangeType::Added => {
                    tightened_count += 1;
                    ConditionImpact {
                        index: *index,
                        relaxation_score: -0.5, // Adding conditions tightens
                        estimated_population_change: -10.0, // Rough estimate
                        description: "New condition added, reducing eligible population"
                            .to_string(),
                    }
                }
                ChangeType::Removed => {
                    relaxed_count += 1;
                    ConditionImpact {
                        index: *index,
                        relaxation_score: 0.5, // Removing conditions relaxes
                        estimated_population_change: 15.0, // Rough estimate
                        description: "Condition removed, expanding eligible population".to_string(),
                    }
                }
                ChangeType::Modified => {
                    // Analyze the modification to determine if it's relaxation or tightening
                    if let (Some(old), Some(new)) = (&change.old_value, &change.new_value) {
                        if let Some((old_val, new_val, is_relaxed)) =
                            detect_threshold_change(old, new)
                        {
                            if is_relaxed {
                                relaxed_count += 1;
                                let pop_change = estimate_population_change(&old_val, &new_val);
                                ConditionImpact {
                                    index: *index,
                                    relaxation_score: 0.3,
                                    estimated_population_change: pop_change,
                                    description: format!(
                                        "Threshold relaxed from {} to {}, increasing eligibility",
                                        old_val, new_val
                                    ),
                                }
                            } else {
                                tightened_count += 1;
                                let pop_change = estimate_population_change(&old_val, &new_val);
                                ConditionImpact {
                                    index: *index,
                                    relaxation_score: -0.3,
                                    estimated_population_change: -pop_change,
                                    description: format!(
                                        "Threshold tightened from {} to {}, decreasing eligibility",
                                        old_val, new_val
                                    ),
                                }
                            }
                        } else {
                            ConditionImpact {
                                index: *index,
                                relaxation_score: 0.0,
                                estimated_population_change: 0.0,
                                description: "Condition modified, impact unclear".to_string(),
                            }
                        }
                    } else {
                        ConditionImpact {
                            index: *index,
                            relaxation_score: 0.0,
                            estimated_population_change: 0.0,
                            description: "Condition modified".to_string(),
                        }
                    }
                }
                ChangeType::Reordered => ConditionImpact {
                    index: *index,
                    relaxation_score: 0.0,
                    estimated_population_change: 0.0,
                    description: "Condition reordered, no impact on eligibility".to_string(),
                },
            };
            per_condition_impact.push(impact);
        }
    }

    // Calculate net score
    let total_changes = relaxed_count + tightened_count;
    let net_score = if total_changes > 0 {
        (relaxed_count as f64 - tightened_count as f64) / total_changes as f64
    } else {
        0.0
    };

    RelaxationMetrics {
        relaxed_count,
        tightened_count,
        net_score,
        per_condition_impact,
    }
}

/// Estimates the percentage change in eligible population.
fn estimate_population_change(old_val: &str, new_val: &str) -> f64 {
    // Simple heuristic based on the magnitude of change
    if let (Ok(old_num), Ok(new_num)) = (old_val.parse::<f64>(), new_val.parse::<f64>()) {
        if old_num != 0.0 {
            ((new_num - old_num) / old_num).abs() * 10.0 // Rough estimate
        } else {
            5.0
        }
    } else {
        5.0 // Default estimate
    }
}

/// Computes scope metrics for effect changes.
fn compute_scope_metrics(changes: &[Change], semantic_changes: &[SemanticChange]) -> ScopeMetrics {
    let mut scope_changed = false;
    let mut expansion_count = 0;
    let mut contraction_count = 0;
    let mut total_magnitude = 0.0;
    let mut descriptions = Vec::new();

    for (change, semantic) in changes.iter().zip(semantic_changes.iter()) {
        match semantic.semantic_type {
            SemanticChangeType::ScopeExpansion => {
                scope_changed = true;
                expansion_count += 1;
                total_magnitude += 0.5;
                descriptions.push("Scope expanded".to_string());
            }
            SemanticChangeType::ScopeReduction => {
                scope_changed = true;
                contraction_count += 1;
                total_magnitude += 0.5;
                descriptions.push("Scope contracted".to_string());
            }
            SemanticChangeType::IntentChange => {
                if matches!(change.target, ChangeTarget::Effect) {
                    scope_changed = true;
                    total_magnitude += 1.0;
                    descriptions.push("Effect fundamentally changed".to_string());
                }
            }
            _ => {}
        }
    }

    let direction = if expansion_count > 0 && contraction_count > 0 {
        ScopeDirection::Mixed
    } else if expansion_count > 0 {
        ScopeDirection::Expanded
    } else if contraction_count > 0 {
        ScopeDirection::Contracted
    } else {
        ScopeDirection::Unchanged
    };

    let magnitude = (total_magnitude / (changes.len() as f64 + 1.0)).min(1.0);

    let description = if descriptions.is_empty() {
        "No scope changes detected".to_string()
    } else {
        descriptions.join("; ")
    };

    ScopeMetrics {
        scope_changed,
        magnitude,
        direction,
        description,
    }
}

/// Classifies breaking changes.
fn classify_breaking_changes(
    diff: &StatuteDiff,
    semantic_changes: &[SemanticChange],
) -> BreakingChangeClassification {
    let mut breaking_types = Vec::new();
    let mut severity_score = 0.0;

    // Check for effect type changes
    for change in &diff.changes {
        if matches!(change.target, ChangeTarget::Effect)
            && change.change_type == ChangeType::Modified
            && let (Some(old), Some(new)) = (&change.old_value, &change.new_value)
        {
            if old.contains("Grant") != new.contains("Grant")
                || old.contains("Revoke") != new.contains("Revoke")
            {
                breaking_types.push(BreakingType::EffectTypeChange);
                severity_score += 1.0;
            } else {
                breaking_types.push(BreakingType::OutcomeChange);
                severity_score += 0.6;
            }
        }
    }

    // Check for new mandatory conditions
    let new_conditions = diff
        .changes
        .iter()
        .filter(|c| {
            matches!(c.target, ChangeTarget::Precondition { .. })
                && c.change_type == ChangeType::Added
        })
        .count();

    if new_conditions > 0 {
        breaking_types.push(BreakingType::NewMandatoryConditions);
        severity_score += 0.7 * new_conditions as f64;
    }

    // Check for tightened conditions
    let tightened = semantic_changes
        .iter()
        .filter(|sc| {
            matches!(
                sc.structural_change.target,
                ChangeTarget::Precondition { .. }
            ) && sc.structural_change.change_type == ChangeType::Modified
                && sc.explanation.contains("tightened")
        })
        .count();

    if tightened > 0 {
        breaking_types.push(BreakingType::TightenedConditions);
        severity_score += 0.5 * tightened as f64;
    }

    // Check for discretion changes
    if diff.impact.discretion_changed {
        breaking_types.push(BreakingType::DiscretionChange);
        severity_score += 0.8;
    }

    // Check for intent changes
    let has_intent_change = semantic_changes
        .iter()
        .any(|sc| sc.semantic_type == SemanticChangeType::IntentChange);

    if has_intent_change {
        breaking_types.push(BreakingType::IntentChange);
        severity_score += 0.9;
    }

    // Determine breaking level
    let level = if severity_score >= 2.0 {
        BreakingLevel::Critical
    } else if severity_score >= 1.5 {
        BreakingLevel::Major
    } else if severity_score >= 0.8 {
        BreakingLevel::Moderate
    } else if severity_score >= 0.3 {
        BreakingLevel::Minor
    } else {
        BreakingLevel::None
    };

    let is_breaking = !breaking_types.is_empty();

    // Calculate compatibility score (inverse of severity)
    let compatibility_score = (1.0 - (severity_score / 3.0).min(1.0)).max(0.0);

    // Calculate migration difficulty
    let migration_difficulty = if breaking_types.contains(&BreakingType::EffectTypeChange) {
        0.9
    } else if breaking_types.len() >= 3 {
        0.7
    } else if breaking_types.len() >= 2 {
        0.5
    } else if breaking_types.len() == 1 {
        0.3
    } else {
        0.0
    };

    BreakingChangeClassification {
        is_breaking,
        level,
        breaking_types,
        compatibility_score,
        migration_difficulty,
    }
}

/// Detects intent-preserving refactoring.
///
/// Returns true if changes are purely refactoring without changing legal intent.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, semantic::is_intent_preserving_refactoring};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "Improved Title".to_string(); // Just rewording
///
/// let diff_result = diff(&old, &new).unwrap();
/// assert!(is_intent_preserving_refactoring(&diff_result));
/// ```
pub fn is_intent_preserving_refactoring(diff: &StatuteDiff) -> bool {
    let semantic = analyze_semantic_diff(diff);

    // Check if all changes are meaning-preserving or technical corrections
    let all_preserving = semantic.semantic_changes.iter().all(|sc| {
        matches!(
            sc.semantic_type,
            SemanticChangeType::MeaningPreserving
                | SemanticChangeType::TechnicalCorrection
                | SemanticChangeType::Clarification
        )
    });

    // Check that there are no breaking changes
    let no_breaking = !semantic.breaking_classification.is_breaking;

    // Check that intent hasn't changed
    let intent_preserved = !semantic.semantic_impact.intent_changed;

    all_preserving && no_breaking && intent_preserved
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, EffectType};

    fn test_statute() -> Statute {
        Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_title_change_semantic() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "Updated Test Statute".to_string();

        let structural_diff = diff(&old, &new).unwrap();
        let semantic_diff = analyze_semantic_diff(&structural_diff);

        assert_eq!(semantic_diff.semantic_changes.len(), 1);
        assert!(matches!(
            semantic_diff.semantic_changes[0].semantic_type,
            SemanticChangeType::Clarification | SemanticChangeType::TechnicalCorrection
        ));
    }

    #[test]
    fn test_scope_expansion() {
        let old = test_statute();
        let mut new = old.clone();
        // Remove the precondition means scope expansion
        new.preconditions.clear();

        let structural_diff = diff(&old, &new).unwrap();
        let semantic_diff = analyze_semantic_diff(&structural_diff);

        assert!(semantic_diff.semantic_impact.eligibility_broadened);
    }

    #[test]
    fn test_scope_reduction() {
        let old = test_statute();
        let new = old.clone().with_precondition(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let structural_diff = diff(&old, &new).unwrap();
        let semantic_diff = analyze_semantic_diff(&structural_diff);

        assert!(semantic_diff.semantic_impact.eligibility_narrowed);
    }

    #[test]
    fn test_intent_change() {
        let old = test_statute();
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke benefit");

        let structural_diff = diff(&old, &new).unwrap();
        let semantic_diff = analyze_semantic_diff(&structural_diff);

        assert!(semantic_diff.semantic_impact.intent_changed);
    }

    #[test]
    fn test_semantically_equivalent() {
        let statute1 = test_statute();
        let mut statute2 = statute1.clone();
        statute2.title = "Different Title".to_string();

        // Different titles but same legal logic ARE semantically equivalent
        // (titles are just labels, not legal substance)
        assert!(are_semantically_equivalent(&statute1, &statute2));

        // But different preconditions are NOT equivalent
        let mut statute3 = statute1.clone();
        statute3.preconditions.clear();
        assert!(!are_semantically_equivalent(&statute1, &statute3));
    }

    #[test]
    fn test_pattern_detection() {
        let old = test_statute();
        let mut new = old.clone();
        new.preconditions.clear(); // Remove all preconditions = scope expansion
        new.title = "New Title".to_string();

        let structural_diff = diff(&old, &new).unwrap();
        let semantic_diff = analyze_semantic_diff(&structural_diff);

        // Should detect some patterns
        assert!(!semantic_diff.patterns.is_empty() || !semantic_diff.semantic_changes.is_empty());
    }

    #[test]
    fn test_threshold_detection() {
        let old_str = "Age: GreaterOrEqual 18";
        let new_str = "Age: GreaterOrEqual 21";

        let result = detect_threshold_change(old_str, new_str);
        assert!(result.is_some());

        let (old_val, new_val, is_relaxed) = result.unwrap();
        assert_eq!(old_val, "18");
        assert_eq!(new_val, "21");
        assert!(!is_relaxed); // Higher age threshold is tighter
    }

    #[test]
    fn test_confidence_scores() {
        let old = test_statute();
        let mut new = old.clone();
        new.title = "New Title".to_string();

        let structural_diff = diff(&old, &new).unwrap();
        let semantic_diff = analyze_semantic_diff(&structural_diff);

        // All semantic changes should have confidence scores
        for change in &semantic_diff.semantic_changes {
            assert!(change.confidence > 0.0 && change.confidence <= 1.0);
        }

        assert!(semantic_diff.semantic_impact.overall_confidence > 0.0);
        assert!(semantic_diff.semantic_impact.overall_confidence <= 1.0);
    }
}
