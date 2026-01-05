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

    /// Property: Diff should handle extreme values gracefully
    #[test]
    fn prop_extreme_values_handled(
        title_len in 0usize..1000,
        precond_count in 0usize..100
    ) {
        let title = "A".repeat(title_len);
        let effect = Effect::new(EffectType::Grant, "Test");

        let mut statute1 = Statute::new("test", &title, effect.clone());
        let statute2 = Statute::new("test", &title, effect);

        // Add many preconditions
        for i in 0..precond_count {
            statute1.preconditions.push(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: i as u32,
            });
        }

        let result = diff(&statute1, &statute2);
        prop_assert!(result.is_ok(), "Should handle extreme values without panic");
    }

    /// Property: Diff should be commutative in terms of detecting changes
    #[test]
    fn prop_diff_detects_inverse_changes(old in statute_strategy(), new in statute_strategy()) {
        if old.id != new.id {
            return Ok(());
        }

        let forward = diff(&old, &new).expect("Forward diff should succeed");
        let backward = diff(&new, &old).expect("Backward diff should succeed");

        // Same number of changes in both directions
        prop_assert_eq!(forward.changes.len(), backward.changes.len(),
            "Forward and backward should have same change count");
    }

    /// Property: Multiple consecutive diffs should maintain consistency
    #[test]
    fn prop_consecutive_diffs_consistent(
        s1 in statute_strategy(),
        s2 in statute_strategy(),
        s3 in statute_strategy()
    ) {
        if s1.id != s2.id || s2.id != s3.id {
            return Ok(());
        }

        let diff12 = diff(&s1, &s2).expect("Diff 1->2 should succeed");
        let diff23 = diff(&s2, &s3).expect("Diff 2->3 should succeed");
        let diff13 = diff(&s1, &s3).expect("Diff 1->3 should succeed");

        // If no changes between 1 and 2, and no changes between 2 and 3,
        // there should be no changes between 1 and 3
        if diff12.changes.is_empty() && diff23.changes.is_empty() {
            prop_assert!(diff13.changes.is_empty(),
                "Transitive no-change property should hold");
        }
    }

    /// Property: Severity should never decrease when adding changes
    #[test]
    fn prop_severity_monotonic(statute in statute_strategy()) {
        let base_diff = diff(&statute, &statute).expect("Self-diff should succeed");
        prop_assert_eq!(base_diff.impact.severity, Severity::None);

        // Make a change
        let mut modified = statute.clone();
        modified.title = format!("{}_modified", modified.title);

        let modified_diff = diff(&statute, &modified).expect("Diff should succeed");
        prop_assert!(modified_diff.impact.severity > Severity::None,
            "Any change should increase severity");
    }
}

#[cfg(test)]
mod fuzz_style_tests {
    use super::*;
    use legalis_diff::{diff_effect_only, diff_preconditions_only};

    proptest! {
        /// Fuzz test: Random mutations should not panic
        #[test]
        fn fuzz_random_statute_mutations(statute in statute_strategy(), seed in any::<u64>()) {
            use std::hash::{BuildHasher, RandomState};

            let mut modified = statute.clone();

            // Apply random mutations based on seed
            let mutation_type = RandomState::new().hash_one(seed) % 5;

            match mutation_type {
                0 => modified.title.push_str("_mutated"),
                1 => {
                    if !modified.preconditions.is_empty() {
                        modified.preconditions.clear();
                    }
                },
                2 => modified.effect = Effect::new(EffectType::Revoke, "Mutated"),
                3 => modified.discretion_logic = Some("Mutated logic".to_string()),
                _ => modified.id.push('x'),
            }

            // Should not panic, even with ID mismatch
            let _ = diff(&statute, &modified);
        }

        /// Fuzz test: Partial diff functions should not panic
        #[test]
        fn fuzz_partial_diffs(old in statute_strategy(), new in statute_strategy()) {
            if old.id != new.id {
                return Ok(());
            }

            // These should never panic
            let _ = diff_preconditions_only(&old, &new);
            let _ = diff_effect_only(&old, &new);
        }

        /// Fuzz test: Large statute sequences
        #[test]
        fn fuzz_large_sequences(count in 1usize..50) {
            let effect = Effect::new(EffectType::Grant, "Test");
            let statutes: Vec<_> = (0..count)
                .map(|i| Statute::new("test", format!("Title {}", i), effect.clone()))
                .collect();

            let result = legalis_diff::diff_sequence(&statutes);
            prop_assert!(result.is_ok(), "Large sequences should not panic");
        }

        /// Fuzz test: Deeply nested condition combinations
        #[test]
        fn fuzz_complex_preconditions(cond_count in 0usize..50) {
            let effect = Effect::new(EffectType::Grant, "Test");
            let mut statute1 = Statute::new("test", "Title", effect.clone());
            let mut statute2 = Statute::new("test", "Title", effect);

            // Add many different types of conditions
            for i in 0..cond_count {
                let cond = match i % 4 {
                    0 => Condition::Age {
                        operator: ComparisonOp::GreaterOrEqual,
                        value: i as u32,
                    },
                    1 => Condition::Income {
                        operator: ComparisonOp::LessOrEqual,
                        value: i as u64 * 1000,
                    },
                    2 => Condition::HasAttribute {
                        key: format!("attr_{}", i),
                    },
                    _ => Condition::Geographic {
                        region_type: RegionType::Country,
                        region_id: "US".to_string(),
                    },
                };

                if i % 2 == 0 {
                    statute1.preconditions.push(cond);
                } else {
                    statute2.preconditions.push(cond);
                }
            }

            let result = diff(&statute1, &statute2);
            prop_assert!(result.is_ok(), "Complex preconditions should not panic");
        }

        /// Fuzz test: Unicode and special characters in strings
        #[test]
        fn fuzz_unicode_strings(
            title in "\\PC{0,100}",
            discretion in prop::option::of("\\PC{0,200}")
        ) {
            let effect = Effect::new(EffectType::Grant, "Test");
            let statute1 = Statute::new("test", &title, effect.clone());
            let mut statute2 = Statute::new("test", &title, effect);
            statute2.discretion_logic = discretion;

            let result = diff(&statute1, &statute2);
            prop_assert!(result.is_ok(), "Unicode strings should not panic");
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
