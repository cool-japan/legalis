//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{calculate_backward_compatibility, calculate_impact_score};
use super::types::{MigrationComplexity, MigrationEffort};

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
    let base_hours = match complexity {
        MigrationComplexity::None => 0.0,
        MigrationComplexity::Trivial => 2.0,
        MigrationComplexity::Simple => 8.0,
        MigrationComplexity::Moderate => 40.0,
        MigrationComplexity::Complex => 120.0,
        MigrationComplexity::VeryComplex => 320.0,
    };
    let estimated_hours = base_hours * (1.0 + diff.changes.len() as f64 * 0.1);
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
        MigrationComplexity::Complex => "Multi-phase migration with extensive testing".to_string(),
        MigrationComplexity::VeryComplex => "Complete redesign with gradual transition".to_string(),
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
    use super::super::functions::{
        analyze_cross_statute_impact, analyze_effect_scope_change, analyze_single_change,
        compare_conditions, conditions_overlap, detect_equivalent_conditions,
        detect_equivalent_preconditions, extract_numeric_value, generate_cross_statute_report,
    };
    use super::super::types::{
        ChangeCompatibility, ConditionComparison, CrossStatuteImpactLevel, EffectScopeChange,
        EquivalenceResult, StatuteRelationship,
    };
    use crate::{Change, ChangeTarget, ChangeType};
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
