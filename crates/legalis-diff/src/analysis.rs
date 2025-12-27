//! Advanced analysis of statute changes.
//!
//! This module provides sophisticated analysis capabilities:
//! - Breaking vs non-breaking change detection
//! - Condition relaxation/tightening tracking
//! - Logical equivalence detection
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_diff::{diff, analysis::analyze_changes};
//!
//! let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
//!     .with_precondition(Condition::Age {
//!         operator: ComparisonOp::GreaterOrEqual,
//!         value: 21,
//!     });
//!
//! let mut new = old.clone();
//! new.preconditions.clear(); // Remove precondition
//!
//! let diff_result = diff(&old, &new).unwrap();
//! let analyses = analyze_changes(&diff_result);
//!
//! // Check if changes relax conditions
//! assert!(analyses.iter().any(|a| a.relaxes_conditions));
//! ```

use crate::{Change, ChangeTarget, ChangeType, StatuteDiff};
use legalis_core::{ComparisonOp, Condition, Statute};

/// Classification of change impact on compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeCompatibility {
    /// Change does not affect behavior (e.g., renaming, formatting).
    NonBreaking,
    /// Change relaxes requirements (backward compatible).
    BackwardCompatible,
    /// Change tightens requirements (forward compatible).
    ForwardCompatible,
    /// Change breaks compatibility in both directions.
    Breaking,
}

/// Analysis result for a change.
#[derive(Debug, Clone)]
pub struct ChangeAnalysis {
    /// The change being analyzed.
    pub change: Change,
    /// Compatibility classification.
    pub compatibility: ChangeCompatibility,
    /// Whether this change relaxes conditions.
    pub relaxes_conditions: bool,
    /// Whether this change tightens conditions.
    pub tightens_conditions: bool,
    /// Explanation of the analysis.
    pub explanation: String,
}

/// Analyzes changes for compatibility and impact.
pub fn analyze_changes(diff: &StatuteDiff) -> Vec<ChangeAnalysis> {
    diff.changes.iter().map(analyze_single_change).collect()
}

/// Analyzes a single change.
pub fn analyze_single_change(change: &Change) -> ChangeAnalysis {
    match &change.target {
        ChangeTarget::Title => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::NonBreaking,
            relaxes_conditions: false,
            tightens_conditions: false,
            explanation: "Title changes are cosmetic and don't affect behavior".to_string(),
        },

        ChangeTarget::Precondition { .. } => analyze_precondition_change(change),

        ChangeTarget::Effect => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::Breaking,
            relaxes_conditions: false,
            tightens_conditions: false,
            explanation: "Effect changes alter the outcome and are breaking".to_string(),
        },

        ChangeTarget::DiscretionLogic => analyze_discretion_change(change),

        ChangeTarget::Metadata { .. } => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::NonBreaking,
            relaxes_conditions: false,
            tightens_conditions: false,
            explanation: "Metadata changes don't affect legal logic".to_string(),
        },
    }
}

fn analyze_precondition_change(change: &Change) -> ChangeAnalysis {
    match change.change_type {
        ChangeType::Added => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::ForwardCompatible,
            relaxes_conditions: false,
            tightens_conditions: true,
            explanation: "Adding preconditions makes eligibility stricter (tightens)".to_string(),
        },

        ChangeType::Removed => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::BackwardCompatible,
            relaxes_conditions: true,
            tightens_conditions: false,
            explanation: "Removing preconditions makes eligibility broader (relaxes)".to_string(),
        },

        ChangeType::Modified => {
            // Try to detect if this is a relaxation or tightening
            // This is a simplified heuristic - real analysis would parse conditions
            let (relaxes, tightens, compat) =
                if let (Some(old), Some(new)) = (&change.old_value, &change.new_value) {
                    detect_condition_direction_change(old, new)
                } else {
                    (false, false, ChangeCompatibility::Breaking)
                };

            ChangeAnalysis {
                change: change.clone(),
                compatibility: compat,
                relaxes_conditions: relaxes,
                tightens_conditions: tightens,
                explanation: if relaxes {
                    "Modified precondition relaxes eligibility requirements".to_string()
                } else if tightens {
                    "Modified precondition tightens eligibility requirements".to_string()
                } else {
                    "Modified precondition changes eligibility logic".to_string()
                },
            }
        }

        ChangeType::Reordered => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::NonBreaking,
            relaxes_conditions: false,
            tightens_conditions: false,
            explanation: "Reordering preconditions doesn't change logic (AND semantics)"
                .to_string(),
        },
    }
}

fn analyze_discretion_change(change: &Change) -> ChangeAnalysis {
    match change.change_type {
        ChangeType::Added => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::Breaking,
            relaxes_conditions: false,
            tightens_conditions: false,
            explanation: "Adding discretion makes outcomes non-deterministic".to_string(),
        },

        ChangeType::Removed => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::Breaking,
            relaxes_conditions: false,
            tightens_conditions: false,
            explanation: "Removing discretion makes outcomes deterministic".to_string(),
        },

        ChangeType::Modified => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::Breaking,
            relaxes_conditions: false,
            tightens_conditions: false,
            explanation: "Changing discretion criteria alters decision process".to_string(),
        },

        ChangeType::Reordered => ChangeAnalysis {
            change: change.clone(),
            compatibility: ChangeCompatibility::NonBreaking,
            relaxes_conditions: false,
            tightens_conditions: false,
            explanation: "Reordering discretion factors is cosmetic".to_string(),
        },
    }
}

/// Attempts to detect whether a condition modification relaxes or tightens.
/// This is a heuristic based on string patterns.
fn detect_condition_direction_change(old: &str, new: &str) -> (bool, bool, ChangeCompatibility) {
    // Simple heuristics:
    // - Numerical threshold changes
    // - Operator changes (>= to >, < to <=, etc.)

    // Check for numerical changes
    if let (Some(old_num), Some(new_num)) = (extract_number(old), extract_number(new)) {
        // For Age conditions (>= or >)
        if old.contains("Age") && old.contains("Greater") {
            if new_num < old_num {
                return (true, false, ChangeCompatibility::BackwardCompatible); // Relaxes
            } else if new_num > old_num {
                return (false, true, ChangeCompatibility::ForwardCompatible); // Tightens
            }
        }
        // For Income conditions (<= or <)
        else if old.contains("Income") && old.contains("Less") {
            if new_num > old_num {
                return (true, false, ChangeCompatibility::BackwardCompatible); // Relaxes
            } else if new_num < old_num {
                return (false, true, ChangeCompatibility::ForwardCompatible); // Tightens
            }
        }
    }

    // Check for operator changes
    if old.contains("GreaterOrEqual") && new.contains("Greater\"")
        || old.contains("LessOrEqual") && new.contains("Less\"")
    {
        return (false, true, ChangeCompatibility::ForwardCompatible); // Tightens
    }

    if old.contains("Greater\"") && new.contains("GreaterOrEqual")
        || old.contains("Less\"") && new.contains("LessOrEqual")
    {
        return (true, false, ChangeCompatibility::BackwardCompatible); // Relaxes
    }

    // Default: breaking change
    (false, false, ChangeCompatibility::Breaking)
}

fn extract_number(s: &str) -> Option<i64> {
    s.split_whitespace().find_map(|word| {
        word.trim_matches(|c: char| !c.is_numeric())
            .parse::<i64>()
            .ok()
    })
}

/// Analyzes condition relaxation/tightening between two conditions.
pub fn compare_conditions(old: &Condition, new: &Condition) -> ConditionComparison {
    match (old, new) {
        (
            Condition::Age {
                operator: old_op,
                value: old_val,
            },
            Condition::Age {
                operator: new_op,
                value: new_val,
            },
        ) => compare_numeric_condition(*old_op, *old_val as i64, *new_op, *new_val as i64, true),

        (
            Condition::Income {
                operator: old_op,
                value: old_val,
            },
            Condition::Income {
                operator: new_op,
                value: new_val,
            },
        ) => compare_numeric_condition(*old_op, *old_val as i64, *new_op, *new_val as i64, false),

        (
            Condition::Geographic {
                region_id: old_id, ..
            },
            Condition::Geographic {
                region_id: new_id, ..
            },
        ) => {
            if old_id == new_id {
                ConditionComparison::Equivalent
            } else {
                ConditionComparison::Different
            }
        }

        _ => ConditionComparison::Different,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionComparison {
    /// Conditions are logically equivalent.
    Equivalent,
    /// New condition is more relaxed (easier to satisfy).
    Relaxed,
    /// New condition is tightened (harder to satisfy).
    Tightened,
    /// Conditions are different in incomparable ways.
    Different,
}

fn compare_numeric_condition(
    old_op: ComparisonOp,
    old_val: i64,
    new_op: ComparisonOp,
    new_val: i64,
    is_lower_bound: bool,
) -> ConditionComparison {
    // For lower bounds (Age >= X): higher value = tighter
    // For upper bounds (Income <= X): lower value = tighter

    if old_op == new_op && old_val == new_val {
        return ConditionComparison::Equivalent;
    }

    // Simplified comparison - only handle same operators
    if old_op == new_op {
        match (old_op, is_lower_bound) {
            (ComparisonOp::GreaterOrEqual, true) | (ComparisonOp::GreaterThan, true) => {
                if new_val < old_val {
                    ConditionComparison::Relaxed
                } else {
                    ConditionComparison::Tightened
                }
            }
            (ComparisonOp::LessOrEqual, false) | (ComparisonOp::LessThan, false) => {
                if new_val > old_val {
                    ConditionComparison::Relaxed
                } else {
                    ConditionComparison::Tightened
                }
            }
            _ => ConditionComparison::Different,
        }
    } else {
        // Operator changed - different
        ConditionComparison::Different
    }
}

/// Identifies breaking changes in a diff.
pub fn identify_breaking_changes(analyses: &[ChangeAnalysis]) -> Vec<&ChangeAnalysis> {
    analyses
        .iter()
        .filter(|a| a.compatibility == ChangeCompatibility::Breaking)
        .collect()
}

/// Identifies backward-compatible changes in a diff.
pub fn identify_backward_compatible_changes(analyses: &[ChangeAnalysis]) -> Vec<&ChangeAnalysis> {
    analyses
        .iter()
        .filter(|a| a.compatibility == ChangeCompatibility::BackwardCompatible)
        .collect()
}

/// Summary of compatibility analysis.
#[derive(Debug)]
pub struct CompatibilitySummary {
    pub total_changes: usize,
    pub breaking_changes: usize,
    pub backward_compatible_changes: usize,
    pub forward_compatible_changes: usize,
    pub non_breaking_changes: usize,
    pub overall_compatibility: ChangeCompatibility,
}

/// Result of logical equivalence analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquivalenceResult {
    /// Changes are logically equivalent.
    Equivalent,
    /// Changes are not equivalent.
    NotEquivalent,
    /// Cannot determine equivalence.
    Unknown,
}

/// Generates a compatibility summary from analyses.
pub fn summarize_compatibility(analyses: &[ChangeAnalysis]) -> CompatibilitySummary {
    let total_changes = analyses.len();
    let breaking_changes = analyses
        .iter()
        .filter(|a| a.compatibility == ChangeCompatibility::Breaking)
        .count();
    let backward_compatible_changes = analyses
        .iter()
        .filter(|a| a.compatibility == ChangeCompatibility::BackwardCompatible)
        .count();
    let forward_compatible_changes = analyses
        .iter()
        .filter(|a| a.compatibility == ChangeCompatibility::ForwardCompatible)
        .count();
    let non_breaking_changes = analyses
        .iter()
        .filter(|a| a.compatibility == ChangeCompatibility::NonBreaking)
        .count();

    let overall_compatibility = if breaking_changes > 0 {
        ChangeCompatibility::Breaking
    } else if forward_compatible_changes > 0 {
        ChangeCompatibility::ForwardCompatible
    } else if backward_compatible_changes > 0 {
        ChangeCompatibility::BackwardCompatible
    } else {
        ChangeCompatibility::NonBreaking
    };

    CompatibilitySummary {
        total_changes,
        breaking_changes,
        backward_compatible_changes,
        forward_compatible_changes,
        non_breaking_changes,
        overall_compatibility,
    }
}

/// Detects logically equivalent conditions despite syntactic differences.
pub fn detect_equivalent_conditions(old: &Condition, new: &Condition) -> EquivalenceResult {
    // Exact match
    if old == new {
        return EquivalenceResult::Equivalent;
    }

    match (old, new) {
        // Age conditions with equivalent bounds
        (
            Condition::Age {
                operator: old_op,
                value: old_val,
            },
            Condition::Age {
                operator: new_op,
                value: new_val,
            },
        ) => {
            // Age >= 18 is equivalent to Age > 17
            if (*old_op == ComparisonOp::GreaterOrEqual && *new_op == ComparisonOp::GreaterThan)
                && *new_val == old_val - 1
            {
                return EquivalenceResult::Equivalent;
            }
            // Age > 17 is equivalent to Age >= 18
            if (*old_op == ComparisonOp::GreaterThan && *new_op == ComparisonOp::GreaterOrEqual)
                && *old_val == new_val - 1
            {
                return EquivalenceResult::Equivalent;
            }
            // Age <= 65 is equivalent to Age < 66
            if (*old_op == ComparisonOp::LessOrEqual && *new_op == ComparisonOp::LessThan)
                && *new_val == old_val + 1
            {
                return EquivalenceResult::Equivalent;
            }
            // Age < 66 is equivalent to Age <= 65
            if (*old_op == ComparisonOp::LessThan && *new_op == ComparisonOp::LessOrEqual)
                && *old_val == new_val + 1
            {
                return EquivalenceResult::Equivalent;
            }
            EquivalenceResult::NotEquivalent
        }

        // Income conditions with equivalent bounds
        (
            Condition::Income {
                operator: old_op,
                value: old_val,
            },
            Condition::Income {
                operator: new_op,
                value: new_val,
            },
        ) => {
            // Income >= 1000 is equivalent to Income > 999
            if (*old_op == ComparisonOp::GreaterOrEqual && *new_op == ComparisonOp::GreaterThan)
                && *new_val == old_val - 1
            {
                return EquivalenceResult::Equivalent;
            }
            // Income > 999 is equivalent to Income >= 1000
            if (*old_op == ComparisonOp::GreaterThan && *new_op == ComparisonOp::GreaterOrEqual)
                && *old_val == new_val - 1
            {
                return EquivalenceResult::Equivalent;
            }
            // Income <= 5000000 is equivalent to Income < 5000001
            if (*old_op == ComparisonOp::LessOrEqual && *new_op == ComparisonOp::LessThan)
                && *new_val == old_val + 1
            {
                return EquivalenceResult::Equivalent;
            }
            // Income < 5000001 is equivalent to Income <= 5000000
            if (*old_op == ComparisonOp::LessThan && *new_op == ComparisonOp::LessOrEqual)
                && *old_val == new_val + 1
            {
                return EquivalenceResult::Equivalent;
            }
            EquivalenceResult::NotEquivalent
        }

        // Different condition types
        _ => EquivalenceResult::NotEquivalent,
    }
}

/// Detects logically equivalent precondition lists.
/// Handles reordering (AND semantics means order doesn't matter).
pub fn detect_equivalent_preconditions(old: &[Condition], new: &[Condition]) -> EquivalenceResult {
    if old.len() != new.len() {
        return EquivalenceResult::NotEquivalent;
    }

    // For each old condition, try to find an equivalent new condition
    for old_cond in old {
        let found = new.iter().any(|new_cond| {
            detect_equivalent_conditions(old_cond, new_cond) == EquivalenceResult::Equivalent
        });
        if !found {
            return EquivalenceResult::NotEquivalent;
        }
    }

    // For each new condition, try to find an equivalent old condition
    for new_cond in new {
        let found = old.iter().any(|old_cond| {
            detect_equivalent_conditions(old_cond, new_cond) == EquivalenceResult::Equivalent
        });
        if !found {
            return EquivalenceResult::NotEquivalent;
        }
    }

    EquivalenceResult::Equivalent
}

/// Detects if two statutes are logically equivalent despite differences.
pub fn detect_equivalent_statutes(old: &Statute, new: &Statute) -> EquivalenceResult {
    // IDs must match
    if old.id != new.id {
        return EquivalenceResult::NotEquivalent;
    }

    // Effects must match exactly (no logical equivalence for effects)
    if old.effect != new.effect {
        return EquivalenceResult::NotEquivalent;
    }

    // Discretion logic must match
    if old.discretion_logic != new.discretion_logic {
        return EquivalenceResult::NotEquivalent;
    }

    // Check preconditions equivalence
    detect_equivalent_preconditions(&old.preconditions, &new.preconditions)
}

/// Filters out logically equivalent changes from a diff.
pub fn filter_equivalent_changes(diff: &StatuteDiff) -> Vec<Change> {
    diff.changes
        .iter()
        .filter(|change| !is_cosmetic_change(change))
        .cloned()
        .collect()
}

/// Determines if a change is purely cosmetic (no logical impact).
fn is_cosmetic_change(change: &Change) -> bool {
    match &change.target {
        ChangeTarget::Title => true, // Title changes are always cosmetic
        ChangeTarget::Metadata { .. } => true, // Metadata is cosmetic
        ChangeTarget::Precondition { .. } => {
            // Could check if the old and new values are logically equivalent
            // For now, assume all precondition changes have logical impact
            false
        }
        ChangeTarget::Effect => false, // Effect changes are always significant
        ChangeTarget::DiscretionLogic => false, // Discretion changes are significant
    }
}

/// Effect scope change analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectScopeChange {
    /// The effect scope has expanded (more people/situations affected).
    Expanded,
    /// The effect scope has narrowed (fewer people/situations affected).
    Narrowed,
    /// The effect scope has changed in incomparable ways.
    Changed,
    /// The effect scope is unchanged.
    Unchanged,
}

/// Analyzes how the scope of effect application changes.
///
/// This considers both precondition changes and effect magnitude changes.
pub fn analyze_effect_scope_change(old: &Statute, new: &Statute) -> EffectScopeChange {
    // Analyze precondition changes
    let precond_scope = analyze_precondition_scope(&old.preconditions, &new.preconditions);

    // Analyze effect magnitude changes
    let effect_magnitude = analyze_effect_magnitude(&old.effect, &new.effect);

    // Combine analysis
    match (precond_scope, effect_magnitude) {
        (EffectScopeChange::Expanded, EffectScopeChange::Expanded) => EffectScopeChange::Expanded,
        (EffectScopeChange::Narrowed, EffectScopeChange::Narrowed) => EffectScopeChange::Narrowed,
        (EffectScopeChange::Unchanged, change) | (change, EffectScopeChange::Unchanged) => change,
        _ => EffectScopeChange::Changed,
    }
}

fn analyze_precondition_scope(old: &[Condition], new: &[Condition]) -> EffectScopeChange {
    // Fewer preconditions = broader scope
    // More preconditions = narrower scope

    if old.len() < new.len() {
        return EffectScopeChange::Narrowed;
    } else if old.len() > new.len() {
        return EffectScopeChange::Expanded;
    }

    // Check for relaxation/tightening if same number of conditions
    let mut relaxations = 0;
    let mut tightenings = 0;

    for (old_cond, new_cond) in old.iter().zip(new.iter()) {
        match compare_conditions(old_cond, new_cond) {
            ConditionComparison::Relaxed => relaxations += 1,
            ConditionComparison::Tightened => tightenings += 1,
            _ => {}
        }
    }

    if relaxations > tightenings {
        EffectScopeChange::Expanded
    } else if tightenings > relaxations {
        EffectScopeChange::Narrowed
    } else {
        EffectScopeChange::Unchanged
    }
}

fn analyze_effect_magnitude(
    old: &legalis_core::Effect,
    new: &legalis_core::Effect,
) -> EffectScopeChange {
    use legalis_core::EffectType;

    // Check if effect type changed in a way that affects scope
    match (&old.effect_type, &new.effect_type) {
        // Same effect type - check parameters
        (old_type, new_type) if old_type == new_type => {
            // Try to extract numerical values from parameters
            if let (Some(old_val), Some(new_val)) = (
                extract_numeric_value(&old.description),
                extract_numeric_value(&new.description),
            ) {
                if new_val > old_val {
                    return EffectScopeChange::Expanded;
                } else if new_val < old_val {
                    return EffectScopeChange::Narrowed;
                }
            }
            EffectScopeChange::Unchanged
        }
        // Grant to Revoke or vice versa - fundamental change
        (EffectType::Grant, EffectType::Revoke) | (EffectType::Revoke, EffectType::Grant) => {
            EffectScopeChange::Changed
        }
        // Other type changes
        _ => EffectScopeChange::Changed,
    }
}

fn extract_numeric_value(text: &str) -> Option<f64> {
    // Simple extraction - find first number
    // Split by whitespace and non-numeric characters, find first parseable number
    text.split(|c: char| !c.is_numeric() && c != '.')
        .find_map(|s| {
            if !s.is_empty() {
                s.parse::<f64>().ok()
            } else {
                None
            }
        })
}

/// Result of effect scope analysis.
#[derive(Debug, Clone)]
pub struct EffectScopeAnalysis {
    /// Overall scope change
    pub scope_change: EffectScopeChange,
    /// Estimated impact on population (percentage)
    pub population_impact: Option<f64>,
    /// Explanation
    pub explanation: String,
}

/// Cross-statute impact analysis.
///
/// Analyzes how changes to one statute might affect other related statutes.
#[derive(Debug, Clone)]
pub struct CrossStatuteImpact {
    /// The statute being changed
    pub source_statute_id: String,
    /// Potentially affected statutes
    pub affected_statutes: Vec<AffectedStatute>,
    /// Overall impact level
    pub impact_level: CrossStatuteImpactLevel,
}

/// A statute potentially affected by changes to another.
#[derive(Debug, Clone)]
pub struct AffectedStatute {
    /// ID of the affected statute
    pub statute_id: String,
    /// Type of relationship
    pub relationship: StatuteRelationship,
    /// Reason for potential impact
    pub impact_reason: String,
    /// Recommended action
    pub recommended_action: String,
}

/// Types of relationships between statutes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatuteRelationship {
    /// One statute references another (e.g., "as defined in...")
    References,
    /// Statutes have overlapping conditions
    OverlappingConditions,
    /// Statutes have related effects
    RelatedEffects,
    /// One statute is a special case of another
    SpecialCase,
    /// Statutes are mutually exclusive
    MutuallyExclusive,
    /// Part of the same legislative package
    SamePackage,
}

/// Overall impact level of cross-statute changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CrossStatuteImpactLevel {
    /// No cross-statute impact detected
    None,
    /// Minor impact (informational)
    Low,
    /// Moderate impact (review recommended)
    Medium,
    /// High impact (coordination required)
    High,
    /// Critical impact (simultaneous amendment needed)
    Critical,
}

/// Analyzes cross-statute impact by comparing condition overlap.
pub fn analyze_cross_statute_impact(
    changed_statute: &Statute,
    related_statutes: &[Statute],
) -> CrossStatuteImpact {
    let mut affected_statutes = Vec::new();

    for statute in related_statutes {
        if statute.id == changed_statute.id {
            continue; // Skip self
        }

        // Check for various types of relationships
        let mut relationships = Vec::new();

        // Check for overlapping conditions
        if has_overlapping_conditions(&changed_statute.preconditions, &statute.preconditions) {
            relationships.push((
                StatuteRelationship::OverlappingConditions,
                "Statutes have overlapping eligibility criteria".to_string(),
                "Review for potential double-coverage or gaps".to_string(),
            ));
        }

        // Check for related effects
        if has_related_effects(&changed_statute.effect, &statute.effect) {
            relationships.push((
                StatuteRelationship::RelatedEffects,
                "Statutes produce similar or related effects".to_string(),
                "Ensure consistency in benefit/obligation levels".to_string(),
            ));
        }

        // Check for mutual exclusivity
        if might_be_mutually_exclusive(changed_statute, statute) {
            relationships.push((
                StatuteRelationship::MutuallyExclusive,
                "Statutes may be mutually exclusive".to_string(),
                "Verify eligibility rules prevent overlap".to_string(),
            ));
        }

        // Add affected statutes
        for (relationship, reason, action) in relationships {
            affected_statutes.push(AffectedStatute {
                statute_id: statute.id.clone(),
                relationship,
                impact_reason: reason,
                recommended_action: action,
            });
        }
    }

    // Determine overall impact level
    let impact_level = if affected_statutes.is_empty() {
        CrossStatuteImpactLevel::None
    } else if affected_statutes.len() == 1 {
        CrossStatuteImpactLevel::Low
    } else if affected_statutes.len() <= 3 {
        CrossStatuteImpactLevel::Medium
    } else if affected_statutes.iter().any(|a| {
        matches!(
            a.relationship,
            StatuteRelationship::MutuallyExclusive | StatuteRelationship::References
        )
    }) {
        CrossStatuteImpactLevel::High
    } else {
        CrossStatuteImpactLevel::Medium
    };

    CrossStatuteImpact {
        source_statute_id: changed_statute.id.clone(),
        affected_statutes,
        impact_level,
    }
}

fn has_overlapping_conditions(conds1: &[Condition], conds2: &[Condition]) -> bool {
    // Simple heuristic: check if any conditions are similar
    for c1 in conds1 {
        for c2 in conds2 {
            if conditions_overlap(c1, c2) {
                return true;
            }
        }
    }
    false
}

fn conditions_overlap(c1: &Condition, c2: &Condition) -> bool {
    match (c1, c2) {
        (Condition::Age { .. }, Condition::Age { .. }) => true,
        (Condition::Income { .. }, Condition::Income { .. }) => true,
        (
            Condition::Geographic { region_id: r1, .. },
            Condition::Geographic { region_id: r2, .. },
        ) => r1 == r2,
        _ => false,
    }
}

fn has_related_effects(eff1: &legalis_core::Effect, eff2: &legalis_core::Effect) -> bool {
    // Effects are related if they're the same type
    eff1.effect_type == eff2.effect_type
}

fn might_be_mutually_exclusive(stat1: &Statute, stat2: &Statute) -> bool {
    use legalis_core::EffectType;

    // Heuristic: Grant and Revoke of similar things might be mutually exclusive
    matches!(
        (&stat1.effect.effect_type, &stat2.effect.effect_type),
        (EffectType::Grant, EffectType::Revoke) | (EffectType::Revoke, EffectType::Grant)
    )
}

/// Generates a report of cross-statute impact.
pub fn generate_cross_statute_report(impact: &CrossStatuteImpact) -> String {
    let mut report = format!(
        "Cross-Statute Impact Analysis for '{}'\n\n",
        impact.source_statute_id
    );

    report.push_str(&format!("Impact Level: {:?}\n\n", impact.impact_level));

    if impact.affected_statutes.is_empty() {
        report.push_str("No related statutes identified.\n");
        return report;
    }

    report.push_str(&format!(
        "Potentially Affected Statutes: {}\n\n",
        impact.affected_statutes.len()
    ));

    for (i, affected) in impact.affected_statutes.iter().enumerate() {
        report.push_str(&format!(
            "{}. {} (Relationship: {:?})\n",
            i + 1,
            affected.statute_id,
            affected.relationship
        ));
        report.push_str(&format!("   Reason: {}\n", affected.impact_reason));
        report.push_str(&format!("   Action: {}\n\n", affected.recommended_action));
    }

    report
}

/// Change impact score (0-100 scale).
///
/// This provides a quantitative measure of how impactful a change is.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ImpactScore {
    /// Overall impact score (0-100).
    pub overall: u8,
    /// Eligibility impact (0-100).
    pub eligibility: u8,
    /// Outcome impact (0-100).
    pub outcome: u8,
    /// Process impact (0-100).
    pub process: u8,
    /// Population impact (0-100).
    pub population: u8,
}

impl ImpactScore {
    /// Creates a new impact score with all values set to zero.
    pub fn new() -> Self {
        Self {
            overall: 0,
            eligibility: 0,
            outcome: 0,
            process: 0,
            population: 0,
        }
    }
}

impl Default for ImpactScore {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculates the impact score for a diff (0-100 scale).
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, analysis::calculate_impact_score};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 65,
///     });
///
/// let mut new = old.clone();
/// new.preconditions[0] = Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 60, // Lowered age requirement - significant impact
/// };
///
/// let diff_result = diff(&old, &new).unwrap();
/// let score = calculate_impact_score(&diff_result);
///
/// assert!(score.eligibility > 20); // Significant eligibility impact
/// ```
pub fn calculate_impact_score(diff: &crate::StatuteDiff) -> ImpactScore {
    let mut eligibility = 0u8;
    let mut outcome = 0u8;
    let mut process = 0u8;
    let mut population = 0u8;

    // Analyze changes
    for change in &diff.changes {
        match &change.target {
            crate::ChangeTarget::Precondition { .. } => match change.change_type {
                crate::ChangeType::Added => {
                    eligibility = eligibility.saturating_add(30);
                    population = population.saturating_add(20);
                }
                crate::ChangeType::Removed => {
                    eligibility = eligibility.saturating_add(40);
                    population = population.saturating_add(30);
                }
                crate::ChangeType::Modified => {
                    eligibility = eligibility.saturating_add(25);
                    population = population.saturating_add(15);
                }
                crate::ChangeType::Reordered => {
                    eligibility = eligibility.saturating_add(5);
                }
            },
            crate::ChangeTarget::Effect => {
                outcome = outcome.saturating_add(80);
                population = population.saturating_add(50);
            }
            crate::ChangeTarget::DiscretionLogic => {
                process = process.saturating_add(60);
                outcome = outcome.saturating_add(20);
            }
            crate::ChangeTarget::Title => {
                process = process.saturating_add(5);
            }
            crate::ChangeTarget::Metadata { .. } => {
                process = process.saturating_add(2);
            }
        }
    }

    // Cap at 100
    eligibility = eligibility.min(100);
    outcome = outcome.min(100);
    process = process.min(100);
    population = population.min(100);

    // Calculate overall score (weighted average)
    let overall = ((eligibility as u16 * 30
        + outcome as u16 * 40
        + process as u16 * 15
        + population as u16 * 15)
        / 100) as u8;

    ImpactScore {
        overall,
        eligibility,
        outcome,
        process,
        population,
    }
}

/// Stakeholder type affected by statute changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StakeholderType {
    /// Individual citizens/residents.
    Citizens,
    /// Businesses and corporations.
    Businesses,
    /// Government agencies.
    GovernmentAgencies,
    /// Legal professionals.
    LegalProfessionals,
    /// Social service providers.
    ServiceProviders,
    /// Advocacy groups.
    AdvocacyGroups,
}

/// Impact on a specific stakeholder group.
#[derive(Debug, Clone)]
pub struct StakeholderImpact {
    /// Type of stakeholder.
    pub stakeholder_type: StakeholderType,
    /// Impact level (0-100).
    pub impact_level: u8,
    /// Description of the impact.
    pub description: String,
    /// Estimated number affected.
    pub estimated_affected: Option<u64>,
    /// Recommended actions for this stakeholder.
    pub recommended_actions: Vec<String>,
}

/// Complete stakeholder impact analysis.
#[derive(Debug, Clone)]
pub struct StakeholderAnalysis {
    /// Impacts by stakeholder type.
    pub impacts: Vec<StakeholderImpact>,
    /// Overall stakeholder impact summary.
    pub summary: String,
}

/// Analyzes the impact on different stakeholders.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, analysis::analyze_stakeholder_impact};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.effect = Effect::new(EffectType::Revoke, "Revoke benefit");
///
/// let diff_result = diff(&old, &new).unwrap();
/// let stakeholder_analysis = analyze_stakeholder_impact(&diff_result);
///
/// assert!(!stakeholder_analysis.impacts.is_empty());
/// ```
pub fn analyze_stakeholder_impact(diff: &crate::StatuteDiff) -> StakeholderAnalysis {
    let mut impacts = Vec::new();

    // Analyze citizen impact
    if diff.impact.affects_eligibility || diff.impact.affects_outcome {
        let impact_level = if diff.impact.affects_outcome { 80 } else { 50 };
        impacts.push(StakeholderImpact {
            stakeholder_type: StakeholderType::Citizens,
            impact_level,
            description: "Eligibility or benefits directly affected".to_string(),
            estimated_affected: None,
            recommended_actions: vec![
                "Review eligibility criteria".to_string(),
                "Update application processes".to_string(),
            ],
        });
    }

    // Analyze business impact
    if diff.changes.iter().any(|c| {
        matches!(
            c.target,
            crate::ChangeTarget::Precondition { .. } | crate::ChangeTarget::Effect
        )
    }) {
        impacts.push(StakeholderImpact {
            stakeholder_type: StakeholderType::Businesses,
            impact_level: 40,
            description: "Compliance requirements may change".to_string(),
            estimated_affected: None,
            recommended_actions: vec![
                "Review compliance procedures".to_string(),
                "Update internal policies".to_string(),
            ],
        });
    }

    // Analyze government agency impact
    if diff.impact.discretion_changed {
        impacts.push(StakeholderImpact {
            stakeholder_type: StakeholderType::GovernmentAgencies,
            impact_level: 70,
            description: "Administrative procedures affected".to_string(),
            estimated_affected: None,
            recommended_actions: vec![
                "Train staff on new procedures".to_string(),
                "Update decision-making guidelines".to_string(),
            ],
        });
    }

    // Analyze legal professional impact
    if !diff.changes.is_empty() {
        impacts.push(StakeholderImpact {
            stakeholder_type: StakeholderType::LegalProfessionals,
            impact_level: 30,
            description: "Legal interpretation may require update".to_string(),
            estimated_affected: None,
            recommended_actions: vec![
                "Review case precedents".to_string(),
                "Update legal guidance".to_string(),
            ],
        });
    }

    let summary = format!(
        "Analysis identified impact on {} stakeholder group(s)",
        impacts.len()
    );

    StakeholderAnalysis { impacts, summary }
}

/// Regulatory compliance impact level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplianceImpactLevel {
    /// No compliance impact.
    None,
    /// Minor compliance adjustments needed.
    Minor,
    /// Moderate compliance changes required.
    Moderate,
    /// Major compliance overhaul needed.
    Major,
    /// Critical compliance risk.
    Critical,
}

/// Regulatory compliance area.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComplianceArea {
    /// Data protection and privacy.
    DataProtection,
    /// Financial regulations.
    Financial,
    /// Labor and employment.
    Labor,
    /// Environmental regulations.
    Environmental,
    /// Health and safety.
    HealthSafety,
    /// Consumer protection.
    ConsumerProtection,
    /// General administrative compliance.
    Administrative,
}

/// Impact on regulatory compliance.
#[derive(Debug, Clone)]
pub struct ComplianceImpact {
    /// Affected compliance area.
    pub area: ComplianceArea,
    /// Impact level.
    pub impact_level: ComplianceImpactLevel,
    /// Description of compliance requirements.
    pub requirements: Vec<String>,
    /// Deadline for compliance (if applicable).
    pub deadline_days: Option<u32>,
}

/// Complete regulatory compliance analysis.
#[derive(Debug, Clone)]
pub struct RegulatoryComplianceAnalysis {
    /// Overall compliance impact level.
    pub overall_impact: ComplianceImpactLevel,
    /// Specific compliance impacts.
    pub impacts: Vec<ComplianceImpact>,
    /// Compliance summary.
    pub summary: String,
}

/// Analyzes regulatory compliance impact.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, analysis::analyze_regulatory_compliance};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.effect = Effect::new(EffectType::Obligation, "New obligation");
///
/// let diff_result = diff(&old, &new).unwrap();
/// let compliance = analyze_regulatory_compliance(&diff_result);
///
/// assert!(!compliance.impacts.is_empty());
/// ```
pub fn analyze_regulatory_compliance(diff: &crate::StatuteDiff) -> RegulatoryComplianceAnalysis {
    let mut impacts = Vec::new();
    let mut max_impact = ComplianceImpactLevel::None;

    // Check for administrative compliance impact
    if !diff.changes.is_empty() {
        let impact_level = if diff.impact.severity >= crate::Severity::Major {
            ComplianceImpactLevel::Major
        } else if diff.impact.severity >= crate::Severity::Moderate {
            ComplianceImpactLevel::Moderate
        } else {
            ComplianceImpactLevel::Minor
        };

        max_impact = max_impact.max(impact_level);

        impacts.push(ComplianceImpact {
            area: ComplianceArea::Administrative,
            impact_level,
            requirements: vec![
                "Update internal documentation".to_string(),
                "Notify affected parties".to_string(),
                "Review and update procedures".to_string(),
            ],
            deadline_days: Some(90),
        });
    }

    // Check for data protection impact
    if diff.impact.affects_eligibility {
        max_impact = max_impact.max(ComplianceImpactLevel::Moderate);
        impacts.push(ComplianceImpact {
            area: ComplianceArea::DataProtection,
            impact_level: ComplianceImpactLevel::Moderate,
            requirements: vec![
                "Review data collection requirements".to_string(),
                "Update privacy policies".to_string(),
            ],
            deadline_days: Some(60),
        });
    }

    let summary = format!(
        "Regulatory compliance analysis identified {} area(s) requiring attention",
        impacts.len()
    );

    RegulatoryComplianceAnalysis {
        overall_impact: max_impact,
        impacts,
        summary,
    }
}

/// Backward compatibility score (0-100, where 100 is fully compatible).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BackwardCompatibilityScore {
    /// Overall compatibility score (0-100).
    pub overall: u8,
    /// Data compatibility (0-100).
    pub data: u8,
    /// API compatibility (0-100).
    pub api: u8,
    /// Behavioral compatibility (0-100).
    pub behavioral: u8,
}

/// Calculates backward compatibility score.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, analysis::calculate_backward_compatibility};
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.title = "New Title".to_string(); // Non-breaking change
///
/// let diff_result = diff(&old, &new).unwrap();
/// let compat = calculate_backward_compatibility(&diff_result);
///
/// assert!(compat.overall >= 90); // High compatibility for title-only change
/// ```
pub fn calculate_backward_compatibility(diff: &crate::StatuteDiff) -> BackwardCompatibilityScore {
    let analyses = analyze_changes(diff);
    let summary = summarize_compatibility(&analyses);

    // Calculate scores based on compatibility analysis
    let breaking_ratio = summary.breaking_changes as f64 / summary.total_changes.max(1) as f64;
    let backward_ratio =
        summary.backward_compatible_changes as f64 / summary.total_changes.max(1) as f64;
    let non_breaking_ratio =
        summary.non_breaking_changes as f64 / summary.total_changes.max(1) as f64;

    // Overall score (inverse of breaking changes)
    let overall = ((1.0 - breaking_ratio) * 100.0) as u8;

    // Data compatibility (based on precondition changes)
    let data = if diff.impact.affects_eligibility {
        if backward_ratio > 0.5 {
            80 // Relaxed conditions maintain data compat
        } else {
            40 // Tightened conditions reduce data compat
        }
    } else {
        100
    };

    // API compatibility (based on effect changes)
    let api = if diff.impact.affects_outcome {
        20 // Effect changes break API
    } else {
        100
    };

    // Behavioral compatibility
    let behavioral = if diff.impact.discretion_changed {
        50 // Discretion changes affect behavior
    } else if non_breaking_ratio > 0.8 {
        100
    } else {
        70
    };

    BackwardCompatibilityScore {
        overall,
        data,
        api,
        behavioral,
    }
}

/// Migration complexity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MigrationComplexity {
    /// No migration needed.
    None,
    /// Trivial migration (documentation update only).
    Trivial,
    /// Simple migration (minor code changes).
    Simple,
    /// Moderate migration (significant changes required).
    Moderate,
    /// Complex migration (major refactoring required).
    Complex,
    /// Very complex migration (complete redesign may be needed).
    VeryComplex,
}

/// Migration effort estimation.
#[derive(Debug, Clone)]
pub struct MigrationEffort {
    /// Complexity level.
    pub complexity: MigrationComplexity,
    /// Estimated effort in person-hours.
    pub estimated_hours: f64,
    /// Migration steps required.
    pub migration_steps: Vec<String>,
    /// Risks associated with migration.
    pub risks: Vec<String>,
    /// Recommended migration strategy.
    pub strategy: String,
}

/// Estimates migration effort for a diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, analysis::estimate_migration_effort};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.effect = Effect::new(EffectType::Revoke, "Revoke"); // Major change
///
/// let diff_result = diff(&old, &new).unwrap();
/// let effort = estimate_migration_effort(&diff_result);
///
/// assert!(effort.estimated_hours > 0.0);
/// ```
pub fn estimate_migration_effort(diff: &crate::StatuteDiff) -> MigrationEffort {
    let impact_score = calculate_impact_score(diff);
    let compat_score = calculate_backward_compatibility(diff);

    // Determine complexity based on impact and compatibility
    let complexity = if diff.changes.is_empty() {
        MigrationComplexity::None
    } else if impact_score.overall < 20 && compat_score.overall > 90 {
        MigrationComplexity::Trivial
    } else if impact_score.overall < 40 && compat_score.overall > 70 {
        MigrationComplexity::Simple
    } else if impact_score.overall < 60 {
        MigrationComplexity::Moderate
    } else if impact_score.overall < 80 {
        MigrationComplexity::Complex
    } else {
        MigrationComplexity::VeryComplex
    };

    // Estimate hours based on complexity and number of changes
    let base_hours = match complexity {
        MigrationComplexity::None => 0.0,
        MigrationComplexity::Trivial => 2.0,
        MigrationComplexity::Simple => 8.0,
        MigrationComplexity::Moderate => 40.0,
        MigrationComplexity::Complex => 120.0,
        MigrationComplexity::VeryComplex => 320.0,
    };

    let estimated_hours = base_hours * (1.0 + diff.changes.len() as f64 * 0.1);

    // Generate migration steps
    let mut migration_steps = Vec::new();
    let mut risks = Vec::new();

    if diff.impact.affects_eligibility {
        migration_steps.push("Update eligibility verification logic".to_string());
        migration_steps.push("Migrate existing eligibility records".to_string());
        risks.push("Existing users may lose eligibility".to_string());
    }

    if diff.impact.affects_outcome {
        migration_steps.push("Update outcome processing logic".to_string());
        migration_steps.push("Test all outcome scenarios".to_string());
        risks.push("Outcomes may differ for existing cases".to_string());
    }

    if diff.impact.discretion_changed {
        migration_steps.push("Train staff on new discretion guidelines".to_string());
        migration_steps.push("Update decision-making workflows".to_string());
        risks.push("Inconsistent decisions during transition".to_string());
    }

    if !diff.changes.is_empty() {
        migration_steps.push("Update documentation and training materials".to_string());
        migration_steps.push("Communicate changes to stakeholders".to_string());
    }

    let strategy = match complexity {
        MigrationComplexity::None => "No migration required".to_string(),
        MigrationComplexity::Trivial => "Update documentation only".to_string(),
        MigrationComplexity::Simple => "Phased rollout with monitoring".to_string(),
        MigrationComplexity::Moderate => {
            "Staged migration with parallel operation period".to_string()
        }
        MigrationComplexity::Complex => {
            "Multi-phase migration with extensive testing".to_string()
        }
        MigrationComplexity::VeryComplex => {
            "Complete redesign with gradual transition".to_string()
        }
    };

    MigrationEffort {
        complexity,
        estimated_hours,
        migration_steps,
        risks,
        strategy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition};

    #[test]
    fn test_age_condition_relaxation() {
        let old = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 20,
        };
        let new = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let result = compare_conditions(&old, &new);
        assert_eq!(result, ConditionComparison::Relaxed);
    }

    #[test]
    fn test_age_condition_tightening() {
        let old = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let new = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        };
        let result = compare_conditions(&old, &new);
        assert_eq!(result, ConditionComparison::Tightened);
    }

    #[test]
    fn test_income_condition_relaxation() {
        let old = Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 3000000,
        };
        let new = Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        };
        let result = compare_conditions(&old, &new);
        assert_eq!(result, ConditionComparison::Relaxed);
    }

    #[test]
    fn test_income_condition_tightening() {
        let old = Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        };
        let new = Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 3000000,
        };
        let result = compare_conditions(&old, &new);
        assert_eq!(result, ConditionComparison::Tightened);
    }

    #[test]
    fn test_equivalent_conditions() {
        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let cond2 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let result = compare_conditions(&cond1, &cond2);
        assert_eq!(result, ConditionComparison::Equivalent);
    }

    #[test]
    fn test_title_change_non_breaking() {
        let change = Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Title,
            description: "Title changed".to_string(),
            old_value: Some("Old".to_string()),
            new_value: Some("New".to_string()),
        };
        let analysis = analyze_single_change(&change);
        assert_eq!(analysis.compatibility, ChangeCompatibility::NonBreaking);
    }

    #[test]
    fn test_effect_change_breaking() {
        let change = Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Effect,
            description: "Effect changed".to_string(),
            old_value: Some("Grant".to_string()),
            new_value: Some("Deny".to_string()),
        };
        let analysis = analyze_single_change(&change);
        assert_eq!(analysis.compatibility, ChangeCompatibility::Breaking);
    }

    #[test]
    fn test_precondition_added_tightens() {
        let change = Change {
            change_type: ChangeType::Added,
            target: ChangeTarget::Precondition { index: 0 },
            description: "Added precondition".to_string(),
            old_value: None,
            new_value: Some("Age >= 18".to_string()),
        };
        let analysis = analyze_single_change(&change);
        assert!(analysis.tightens_conditions);
        assert!(!analysis.relaxes_conditions);
    }

    #[test]
    fn test_precondition_removed_relaxes() {
        let change = Change {
            change_type: ChangeType::Removed,
            target: ChangeTarget::Precondition { index: 0 },
            description: "Removed precondition".to_string(),
            old_value: Some("Age >= 18".to_string()),
            new_value: None,
        };
        let analysis = analyze_single_change(&change);
        assert!(analysis.relaxes_conditions);
        assert!(!analysis.tightens_conditions);
    }

    // Equivalence detection tests
    #[test]
    fn test_age_ge_18_equivalent_to_gt_17() {
        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let cond2 = Condition::Age {
            operator: ComparisonOp::GreaterThan,
            value: 17,
        };
        assert_eq!(
            detect_equivalent_conditions(&cond1, &cond2),
            EquivalenceResult::Equivalent
        );
        assert_eq!(
            detect_equivalent_conditions(&cond2, &cond1),
            EquivalenceResult::Equivalent
        );
    }

    #[test]
    fn test_age_le_65_equivalent_to_lt_66() {
        let cond1 = Condition::Age {
            operator: ComparisonOp::LessOrEqual,
            value: 65,
        };
        let cond2 = Condition::Age {
            operator: ComparisonOp::LessThan,
            value: 66,
        };
        assert_eq!(
            detect_equivalent_conditions(&cond1, &cond2),
            EquivalenceResult::Equivalent
        );
    }

    #[test]
    fn test_income_ge_1000_equivalent_to_gt_999() {
        let cond1 = Condition::Income {
            operator: ComparisonOp::GreaterOrEqual,
            value: 1000,
        };
        let cond2 = Condition::Income {
            operator: ComparisonOp::GreaterThan,
            value: 999,
        };
        assert_eq!(
            detect_equivalent_conditions(&cond1, &cond2),
            EquivalenceResult::Equivalent
        );
    }

    #[test]
    fn test_exact_match_is_equivalent() {
        let cond = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        assert_eq!(
            detect_equivalent_conditions(&cond, &cond),
            EquivalenceResult::Equivalent
        );
    }

    #[test]
    fn test_different_values_not_equivalent() {
        let cond1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let cond2 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 20,
        };
        assert_eq!(
            detect_equivalent_conditions(&cond1, &cond2),
            EquivalenceResult::NotEquivalent
        );
    }

    #[test]
    fn test_preconditions_reordered_are_equivalent() {
        let old = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
            Condition::Income {
                operator: ComparisonOp::LessOrEqual,
                value: 5000000,
            },
        ];
        let new = vec![
            Condition::Income {
                operator: ComparisonOp::LessOrEqual,
                value: 5000000,
            },
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ];
        assert_eq!(
            detect_equivalent_preconditions(&old, &new),
            EquivalenceResult::Equivalent
        );
    }

    #[test]
    fn test_preconditions_different_length_not_equivalent() {
        let old = vec![Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }];
        let new = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
            Condition::Income {
                operator: ComparisonOp::LessOrEqual,
                value: 5000000,
            },
        ];
        assert_eq!(
            detect_equivalent_preconditions(&old, &new),
            EquivalenceResult::NotEquivalent
        );
    }

    #[test]
    fn test_effect_scope_expanded_by_removing_precondition() {
        use legalis_core::{Effect, EffectType, Statute};

        let old = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let new = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let scope_change = analyze_effect_scope_change(&old, &new);
        assert_eq!(scope_change, EffectScopeChange::Expanded);
    }

    #[test]
    fn test_effect_scope_narrowed_by_adding_precondition() {
        use legalis_core::{Effect, EffectType, Statute};

        let old = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let new = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let scope_change = analyze_effect_scope_change(&old, &new);
        assert_eq!(scope_change, EffectScopeChange::Narrowed);
    }

    #[test]
    fn test_effect_scope_expanded_by_relaxing_condition() {
        use legalis_core::{Effect, EffectType, Statute};

        let old = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let new = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let scope_change = analyze_effect_scope_change(&old, &new);
        assert_eq!(scope_change, EffectScopeChange::Expanded);
    }

    #[test]
    fn test_effect_scope_narrowed_by_tightening_condition() {
        use legalis_core::{Effect, EffectType, Statute};

        let old = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let new = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 3000000,
        });

        let scope_change = analyze_effect_scope_change(&old, &new);
        assert_eq!(scope_change, EffectScopeChange::Narrowed);
    }

    #[test]
    fn test_effect_scope_expanded_by_increased_benefit() {
        use legalis_core::{Effect, EffectType, Statute};

        let old = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Monthly subsidy of 50000 yen"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let new = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Monthly subsidy of 60000 yen"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let scope_change = analyze_effect_scope_change(&old, &new);
        assert_eq!(scope_change, EffectScopeChange::Expanded);
    }

    #[test]
    fn test_extract_numeric_value() {
        assert_eq!(
            extract_numeric_value("Monthly subsidy of 50000 yen"),
            Some(50000.0)
        );
        assert_eq!(
            extract_numeric_value("Grant 1500.50 dollars"),
            Some(1500.50)
        );
        assert_eq!(extract_numeric_value("No numbers here"), None);
        assert_eq!(extract_numeric_value(""), None);
    }

    #[test]
    fn test_cross_statute_impact_no_overlap() {
        use legalis_core::{Effect, EffectType, Statute};

        let changed = Statute::new(
            "statute-a",
            "Statute A",
            Effect::new(EffectType::Grant, "Grant A"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let related = vec![
            Statute::new(
                "statute-b",
                "Statute B",
                Effect::new(EffectType::Obligation, "Obligation B"),
            )
            .with_precondition(Condition::Income {
                operator: ComparisonOp::LessOrEqual,
                value: 5000000,
            }),
        ];

        let impact = analyze_cross_statute_impact(&changed, &related);
        assert_eq!(impact.impact_level, CrossStatuteImpactLevel::None);
        assert!(impact.affected_statutes.is_empty());
    }

    #[test]
    fn test_cross_statute_impact_overlapping_conditions() {
        use legalis_core::{Effect, EffectType, Statute};

        let changed = Statute::new(
            "statute-a",
            "Statute A",
            Effect::new(EffectType::Grant, "Grant housing subsidy"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 3000000,
        });

        let related = vec![
            Statute::new(
                "statute-b",
                "Statute B",
                Effect::new(EffectType::Grant, "Grant rental assistance"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 20,
            })
            .with_precondition(Condition::Income {
                operator: ComparisonOp::LessOrEqual,
                value: 4000000,
            }),
        ];

        let impact = analyze_cross_statute_impact(&changed, &related);
        assert!(impact.impact_level > CrossStatuteImpactLevel::None);
        assert!(!impact.affected_statutes.is_empty());

        let affected = &impact.affected_statutes[0];
        assert!(matches!(
            affected.relationship,
            StatuteRelationship::OverlappingConditions | StatuteRelationship::RelatedEffects
        ));
    }

    #[test]
    fn test_cross_statute_impact_mutually_exclusive() {
        use legalis_core::{Effect, EffectType, Statute};

        let changed = Statute::new(
            "statute-grant",
            "Grant License",
            Effect::new(EffectType::Grant, "Grant driving license"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let related = vec![
            Statute::new(
                "statute-revoke",
                "Revoke License",
                Effect::new(EffectType::Revoke, "Revoke driving license"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 16,
            }),
        ];

        let impact = analyze_cross_statute_impact(&changed, &related);
        assert!(!impact.affected_statutes.is_empty());

        let has_mutual_exclusive = impact
            .affected_statutes
            .iter()
            .any(|a| matches!(a.relationship, StatuteRelationship::MutuallyExclusive));
        assert!(has_mutual_exclusive);
    }

    #[test]
    fn test_generate_cross_statute_report() {
        use legalis_core::{Effect, EffectType, Statute};

        let changed = Statute::new(
            "statute-a",
            "Statute A",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let related = vec![
            Statute::new(
                "statute-b",
                "Statute B",
                Effect::new(EffectType::Grant, "Grant similar benefit"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 20,
            }),
        ];

        let impact = analyze_cross_statute_impact(&changed, &related);
        let report = generate_cross_statute_report(&impact);

        assert!(report.contains("statute-a"));
        assert!(report.contains("Impact Level"));
    }

    #[test]
    fn test_conditions_overlap() {
        let age1 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let age2 = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 20,
        };
        let income = Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        };

        assert!(conditions_overlap(&age1, &age2));
        assert!(!conditions_overlap(&age1, &income));
    }
}
