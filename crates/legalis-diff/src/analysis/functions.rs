//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Change, ChangeTarget, ChangeType, StatuteDiff};
use legalis_core::{ComparisonOp, Condition, Statute};

use super::types::{
    AffectedStatute, BackwardCompatibilityScore, ChangeAnalysis, ChangeCompatibility,
    CompatibilitySummary, ComplianceArea, ComplianceImpact, ComplianceImpactLevel,
    ConditionComparison, CrossStatuteImpact, CrossStatuteImpactLevel, EffectScopeChange,
    EquivalenceResult, ImpactScore, RegulatoryComplianceAnalysis, StakeholderAnalysis,
    StakeholderImpact, StakeholderType, StatuteRelationship,
};

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
    if let (Some(old_num), Some(new_num)) = (extract_number(old), extract_number(new)) {
        if old.contains("Age") && old.contains("Greater") {
            if new_num < old_num {
                return (true, false, ChangeCompatibility::BackwardCompatible);
            } else if new_num > old_num {
                return (false, true, ChangeCompatibility::ForwardCompatible);
            }
        } else if old.contains("Income") && old.contains("Less") {
            if new_num > old_num {
                return (true, false, ChangeCompatibility::BackwardCompatible);
            } else if new_num < old_num {
                return (false, true, ChangeCompatibility::ForwardCompatible);
            }
        }
    }
    if old.contains("GreaterOrEqual") && new.contains("Greater\"")
        || old.contains("LessOrEqual") && new.contains("Less\"")
    {
        return (false, true, ChangeCompatibility::ForwardCompatible);
    }
    if old.contains("Greater\"") && new.contains("GreaterOrEqual")
        || old.contains("Less\"") && new.contains("LessOrEqual")
    {
        return (true, false, ChangeCompatibility::BackwardCompatible);
    }
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
fn compare_numeric_condition(
    old_op: ComparisonOp,
    old_val: i64,
    new_op: ComparisonOp,
    new_val: i64,
    is_lower_bound: bool,
) -> ConditionComparison {
    if old_op == new_op && old_val == new_val {
        return ConditionComparison::Equivalent;
    }
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
    if old == new {
        return EquivalenceResult::Equivalent;
    }
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
        ) => {
            if (*old_op == ComparisonOp::GreaterOrEqual && *new_op == ComparisonOp::GreaterThan)
                && *new_val == old_val - 1
            {
                return EquivalenceResult::Equivalent;
            }
            if (*old_op == ComparisonOp::GreaterThan && *new_op == ComparisonOp::GreaterOrEqual)
                && *old_val == new_val - 1
            {
                return EquivalenceResult::Equivalent;
            }
            if (*old_op == ComparisonOp::LessOrEqual && *new_op == ComparisonOp::LessThan)
                && *new_val == old_val + 1
            {
                return EquivalenceResult::Equivalent;
            }
            if (*old_op == ComparisonOp::LessThan && *new_op == ComparisonOp::LessOrEqual)
                && *old_val == new_val + 1
            {
                return EquivalenceResult::Equivalent;
            }
            EquivalenceResult::NotEquivalent
        }
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
            if (*old_op == ComparisonOp::GreaterOrEqual && *new_op == ComparisonOp::GreaterThan)
                && *new_val == old_val - 1
            {
                return EquivalenceResult::Equivalent;
            }
            if (*old_op == ComparisonOp::GreaterThan && *new_op == ComparisonOp::GreaterOrEqual)
                && *old_val == new_val - 1
            {
                return EquivalenceResult::Equivalent;
            }
            if (*old_op == ComparisonOp::LessOrEqual && *new_op == ComparisonOp::LessThan)
                && *new_val == old_val + 1
            {
                return EquivalenceResult::Equivalent;
            }
            if (*old_op == ComparisonOp::LessThan && *new_op == ComparisonOp::LessOrEqual)
                && *old_val == new_val + 1
            {
                return EquivalenceResult::Equivalent;
            }
            EquivalenceResult::NotEquivalent
        }
        _ => EquivalenceResult::NotEquivalent,
    }
}
/// Detects logically equivalent precondition lists.
/// Handles reordering (AND semantics means order doesn't matter).
pub fn detect_equivalent_preconditions(old: &[Condition], new: &[Condition]) -> EquivalenceResult {
    if old.len() != new.len() {
        return EquivalenceResult::NotEquivalent;
    }
    for old_cond in old {
        let found = new.iter().any(|new_cond| {
            detect_equivalent_conditions(old_cond, new_cond) == EquivalenceResult::Equivalent
        });
        if !found {
            return EquivalenceResult::NotEquivalent;
        }
    }
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
    if old.id != new.id {
        return EquivalenceResult::NotEquivalent;
    }
    if old.effect != new.effect {
        return EquivalenceResult::NotEquivalent;
    }
    if old.discretion_logic != new.discretion_logic {
        return EquivalenceResult::NotEquivalent;
    }
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
        ChangeTarget::Title => true,
        ChangeTarget::Metadata { .. } => true,
        ChangeTarget::Precondition { .. } => false,
        ChangeTarget::Effect => false,
        ChangeTarget::DiscretionLogic => false,
    }
}
/// Analyzes how the scope of effect application changes.
///
/// This considers both precondition changes and effect magnitude changes.
pub fn analyze_effect_scope_change(old: &Statute, new: &Statute) -> EffectScopeChange {
    let precond_scope = analyze_precondition_scope(&old.preconditions, &new.preconditions);
    let effect_magnitude = analyze_effect_magnitude(&old.effect, &new.effect);
    match (precond_scope, effect_magnitude) {
        (EffectScopeChange::Expanded, EffectScopeChange::Expanded) => EffectScopeChange::Expanded,
        (EffectScopeChange::Narrowed, EffectScopeChange::Narrowed) => EffectScopeChange::Narrowed,
        (EffectScopeChange::Unchanged, change) | (change, EffectScopeChange::Unchanged) => change,
        _ => EffectScopeChange::Changed,
    }
}
fn analyze_precondition_scope(old: &[Condition], new: &[Condition]) -> EffectScopeChange {
    if old.len() < new.len() {
        return EffectScopeChange::Narrowed;
    } else if old.len() > new.len() {
        return EffectScopeChange::Expanded;
    }
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
    match (&old.effect_type, &new.effect_type) {
        (old_type, new_type) if old_type == new_type => {
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
        (EffectType::Grant, EffectType::Revoke) | (EffectType::Revoke, EffectType::Grant) => {
            EffectScopeChange::Changed
        }
        _ => EffectScopeChange::Changed,
    }
}
pub(super) fn extract_numeric_value(text: &str) -> Option<f64> {
    text.split(|c: char| !c.is_numeric() && c != '.')
        .find_map(|s| {
            if !s.is_empty() {
                s.parse::<f64>().ok()
            } else {
                None
            }
        })
}
/// Analyzes cross-statute impact by comparing condition overlap.
pub fn analyze_cross_statute_impact(
    changed_statute: &Statute,
    related_statutes: &[Statute],
) -> CrossStatuteImpact {
    let mut affected_statutes = Vec::new();
    for statute in related_statutes {
        if statute.id == changed_statute.id {
            continue;
        }
        let mut relationships = Vec::new();
        if has_overlapping_conditions(&changed_statute.preconditions, &statute.preconditions) {
            relationships.push((
                StatuteRelationship::OverlappingConditions,
                "Statutes have overlapping eligibility criteria".to_string(),
                "Review for potential double-coverage or gaps".to_string(),
            ));
        }
        if has_related_effects(&changed_statute.effect, &statute.effect) {
            relationships.push((
                StatuteRelationship::RelatedEffects,
                "Statutes produce similar or related effects".to_string(),
                "Ensure consistency in benefit/obligation levels".to_string(),
            ));
        }
        if might_be_mutually_exclusive(changed_statute, statute) {
            relationships.push((
                StatuteRelationship::MutuallyExclusive,
                "Statutes may be mutually exclusive".to_string(),
                "Verify eligibility rules prevent overlap".to_string(),
            ));
        }
        for (relationship, reason, action) in relationships {
            affected_statutes.push(AffectedStatute {
                statute_id: statute.id.clone(),
                relationship,
                impact_reason: reason,
                recommended_action: action,
            });
        }
    }
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
    for c1 in conds1 {
        for c2 in conds2 {
            if conditions_overlap(c1, c2) {
                return true;
            }
        }
    }
    false
}
pub(super) fn conditions_overlap(c1: &Condition, c2: &Condition) -> bool {
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
    eff1.effect_type == eff2.effect_type
}
fn might_be_mutually_exclusive(stat1: &Statute, stat2: &Statute) -> bool {
    use legalis_core::EffectType;
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
    eligibility = eligibility.min(100);
    outcome = outcome.min(100);
    process = process.min(100);
    population = population.min(100);
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
    let breaking_ratio = summary.breaking_changes as f64 / summary.total_changes.max(1) as f64;
    let backward_ratio =
        summary.backward_compatible_changes as f64 / summary.total_changes.max(1) as f64;
    let non_breaking_ratio =
        summary.non_breaking_changes as f64 / summary.total_changes.max(1) as f64;
    let overall = ((1.0 - breaking_ratio) * 100.0) as u8;
    let data = if diff.impact.affects_eligibility {
        if backward_ratio > 0.5 { 80 } else { 40 }
    } else {
        100
    };
    let api = if diff.impact.affects_outcome { 20 } else { 100 };
    let behavioral = if diff.impact.discretion_changed {
        50
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
