//! Property-based tests for diff operations using proptest.
//!
//! These tests verify fundamental properties that should hold for all diff operations:
//! - Idempotence: diff(x, x) produces no changes
//! - Consistency: same input produces same output
//! - Rollback correctness: rollback reverses forward diff
//! - Change count bounds: reasonable number of changes
//! - Impact assessment consistency: flags match actual changes

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, RegionType, Statute};
use legalis_diff::{ChangeType, Severity, diff, rollback::generate_rollback};
use proptest::prelude::*;

// Strategy for generating Conditions
fn condition_strategy() -> impl Strategy<Value = Condition> {
    prop_oneof![
        (any::<u32>().prop_map(|v| v % 100 + 1)).prop_map(|age| Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: age,
        }),
        (any::<u64>().prop_map(|v| v % 10_000_000)).prop_map(|income| Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: income,
        }),
        ("[a-z]{3,10}").prop_map(|key| Condition::HasAttribute { key }),
        (any::<bool>()).prop_map(|is_country| Condition::Geographic {
            region_type: if is_country {
                RegionType::Country
            } else {
                RegionType::State
            },
            region_id: "JP".to_string(),
        }),
    ]
}

// Strategy for generating Effects
fn effect_strategy() -> impl Strategy<Value = Effect> {
    prop_oneof![
        Just(Effect::new(EffectType::Grant, "Grant benefit")),
        Just(Effect::new(EffectType::Revoke, "Revoke benefit")),
        Just(Effect::new(EffectType::Obligation, "Impose obligation")),
        Just(Effect::new(EffectType::Prohibition, "Prohibit action")),
        Just(Effect::new(EffectType::MonetaryTransfer, "Transfer funds")),
    ]
}

// Strategy for generating Statutes
fn statute_strategy() -> impl Strategy<Value = Statute> {
    (
        "[a-z]{3,10}",
        "[A-Z][a-z ]{5,30}",
        effect_strategy(),
        prop::collection::vec(condition_strategy(), 0..5),
        prop::option::of("[A-Z][a-z ]{10,50}"),
    )
        .prop_map(|(id, title, effect, preconditions, discretion)| {
            let mut statute = Statute::new(&id, &title, effect);
            statute.preconditions = preconditions;
            if let Some(disc) = discretion {
                statute.discretion_logic = Some(disc);
            }
            statute
        })
}

proptest! {
    /// Property: Diffing identical statutes should produce no changes
    #[test]
    fn prop_diff_identical_produces_no_changes(statute in statute_strategy()) {
        let result = diff(&statute, &statute).expect("Diff should succeed");
        prop_assert_eq!(result.changes.len(), 0, "Identical statutes should have no changes");
        prop_assert_eq!(result.impact.severity, Severity::None, "Severity should be None");
        prop_assert!(!result.impact.affects_eligibility, "Should not affect eligibility");
        prop_assert!(!result.impact.affects_outcome, "Should not affect outcome");
        prop_assert!(!result.impact.discretion_changed, "Discretion should not change");
    }

    /// Property: Diff should be deterministic (same input = same output)
    #[test]
    fn prop_diff_is_deterministic(old in statute_strategy(), new in statute_strategy()) {
        // Only test statutes with same ID
        if old.id != new.id {
            return Ok(());
        }

        let result1 = diff(&old, &new).expect("First diff should succeed");
        let result2 = diff(&old, &new).expect("Second diff should succeed");

        prop_assert_eq!(result1.changes.len(), result2.changes.len(), "Change count should be same");
        prop_assert_eq!(result1.impact.severity, result2.impact.severity, "Severity should be same");
        prop_assert_eq!(result1.impact.affects_eligibility, result2.impact.affects_eligibility);
        prop_assert_eq!(result1.impact.affects_outcome, result2.impact.affects_outcome);
    }

    /// Property: Rollback should reverse all changes
    #[test]
    fn prop_rollback_reverses_changes(old in statute_strategy(), new in statute_strategy()) {
        if old.id != new.id {
            return Ok(());
        }

        let forward = diff(&old, &new).expect("Forward diff should succeed");
        let rollback = generate_rollback(&forward);

        // Rollback should have same number of changes
        prop_assert_eq!(forward.changes.len(), rollback.changes.len(),
            "Rollback should have same number of changes");

        // Each rollback change should reverse the forward change
        for (fwd, rbk) in forward.changes.iter().zip(rollback.changes.iter()) {
            prop_assert_eq!(fwd.change_type, rbk.change_type, "Change types should match");
            prop_assert_eq!(&fwd.target, &rbk.target, "Targets should match");
            // Values should be swapped
            prop_assert_eq!(&fwd.old_value, &rbk.new_value, "Old/new should be swapped");
            prop_assert_eq!(&fwd.new_value, &rbk.old_value, "New/old should be swapped");
        }
    }

    /// Property: Title change should be detected correctly
    #[test]
    fn prop_title_change_detected(mut statute in statute_strategy(), new_title in "[A-Z][a-z ]{5,30}") {
        let old = statute.clone();
        statute.title = new_title.clone();

        if old.title == statute.title {
            return Ok(());
        }

        let result = diff(&old, &statute).expect("Diff should succeed");

        let has_title_change = result.changes.iter().any(|c| {
            matches!(c.target, legalis_diff::ChangeTarget::Title)
        });

        prop_assert!(has_title_change, "Title change should be detected");
    }

    /// Property: Effect change should set affects_outcome flag
    #[test]
    fn prop_effect_change_affects_outcome(statute in statute_strategy(), new_effect in effect_strategy()) {
        let old = statute.clone();
        let mut new = statute;
        new.effect = new_effect;

        if old.effect == new.effect {
            return Ok(());
        }

        let result = diff(&old, &new).expect("Diff should succeed");

        prop_assert!(result.impact.affects_outcome, "Effect change should affect outcome");
        prop_assert!(result.impact.severity >= Severity::Major, "Should be at least Major severity");
    }

    /// Property: Adding preconditions should set affects_eligibility flag
    #[test]
    fn prop_adding_precondition_affects_eligibility(
        statute in statute_strategy(),
        new_cond in condition_strategy()
    ) {
        let old = statute.clone();
        let mut new = statute;
        new.preconditions.push(new_cond);

        let result = diff(&old, &new).expect("Diff should succeed");

        prop_assert!(result.impact.affects_eligibility, "Adding precondition should affect eligibility");
        prop_assert!(result.impact.severity >= Severity::Major, "Should be at least Major severity");

        let has_added = result.changes.iter().any(|c| c.change_type == ChangeType::Added);
        prop_assert!(has_added, "Should have Added change type");
    }

    /// Property: Removing preconditions should set affects_eligibility flag
    #[test]
    fn prop_removing_precondition_affects_eligibility(statute in statute_strategy()) {
        if statute.preconditions.is_empty() {
            return Ok(());
        }

        let mut new = statute.clone();
        new.preconditions.pop();

        let result = diff(&statute, &new).expect("Diff should succeed");

        prop_assert!(result.impact.affects_eligibility, "Removing precondition should affect eligibility");
        prop_assert!(result.impact.severity >= Severity::Major, "Should be at least Major severity");

        let has_removed = result.changes.iter().any(|c| c.change_type == ChangeType::Removed);
        prop_assert!(has_removed, "Should have Removed change type");
    }

    /// Property: Change count should be bounded and reasonable
    #[test]
    fn prop_change_count_is_reasonable(old in statute_strategy(), new in statute_strategy()) {
        if old.id != new.id {
            return Ok(());
        }

        let result = diff(&old, &new).expect("Diff should succeed");

        // Maximum possible changes:
        // 1 title + max preconditions from both + 1 effect + 1 discretion
        let max_preconditions = old.preconditions.len().max(new.preconditions.len());
        let max_changes = 1 + max_preconditions + 1 + 1;

        prop_assert!(
            result.changes.len() <= max_changes * 2, // Allow some margin
            "Change count {} should be reasonable (max ~{})",
            result.changes.len(),
            max_changes
        );
    }

    /// Property: Discretion changes should set discretion_changed flag
    #[test]
    fn prop_discretion_change_sets_flag(
        statute in statute_strategy(),
        new_discretion in prop::option::of("[A-Z][a-z ]{10,50}")
    ) {
        let old = statute.clone();
        let mut new = statute;
        new.discretion_logic = new_discretion;

        if old.discretion_logic == new.discretion_logic {
            return Ok(());
        }

        let result = diff(&old, &new).expect("Diff should succeed");

        prop_assert!(result.impact.discretion_changed, "Discretion change should set flag");
    }

    /// Property: Severity ordering should be consistent
    #[test]
    fn prop_severity_is_ordered(old in statute_strategy(), new in statute_strategy()) {
        if old.id != new.id {
            return Ok(());
        }

        let result = diff(&old, &new).expect("Diff should succeed");

        // If affects_outcome is true, severity should be at least Major
        if result.impact.affects_outcome {
            prop_assert!(
                result.impact.severity >= Severity::Major,
                "Outcome changes should be Major or higher"
            );
        }

        // If affects_eligibility is true, severity should be at least Moderate
        if result.impact.affects_eligibility {
            prop_assert!(
                result.impact.severity >= Severity::Moderate,
                "Eligibility changes should be Moderate or higher"
            );
        }

        // If no changes, severity should be None
        if result.changes.is_empty() {
            prop_assert_eq!(result.impact.severity, Severity::None, "No changes = None severity");
        }
    }
}

#[cfg(test)]
mod sequential_property_tests {
    use super::*;
    use legalis_diff::diff_sequence;

    proptest! {
        /// Property: Sequence diff should produce n-1 diffs for n versions
        #[test]
        fn prop_sequence_diff_count(statutes in prop::collection::vec(statute_strategy(), 1..10)) {
            // Make all IDs the same
            let id = statutes[0].id.clone();
            let statutes: Vec<_> = statutes.into_iter()
                .map(|mut s| { s.id = id.clone(); s })
                .collect();

            let result = diff_sequence(&statutes).expect("Sequence diff should succeed");

            if statutes.len() < 2 {
                prop_assert_eq!(result.len(), 0, "Less than 2 versions should produce no diffs");
            } else {
                prop_assert_eq!(
                    result.len(),
                    statutes.len() - 1,
                    "Should produce n-1 diffs for n versions"
                );
            }
        }
    }
}
