use legalis_core::Statute;

use super::super::types::{
    CoverageInfo, CrossReferenceErrorType, ImpactAssessment, RiskLevel, SequenceConstraint,
    StatuteVerifier, VerificationBudget,
};
use super::super::types_3::{
    CrossReferenceError, CtlFormula, JurisdictionalRuleSet, PrincipleCheck, PrincipleCheckResult,
    SimilarityScore, TerminologyInconsistency,
};
use super::super::types_4::{
    AmbiguousTerm, CombinationMode, CompositePrinciple, Deadline, ImpactLevel, IncrementalState,
    LtlFormula, PrincipleDefinition, PrincipleRegistry, TemporalState, TransitionSystem,
};
use super::super::types_5::{ComplexityLevel, VerificationError, VerificationResult};

use super::super::*;
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, TemporalValidity};

#[test]
fn test_verifier_pass() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&[statute]);
    assert!(result.passed);
    assert!(result.errors.is_empty());
}
#[test]
fn test_verifier_discretion_warning() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    )
    .with_discretion("Consider special circumstances");
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&[statute]);
    assert!(result.passed);
    assert!(!result.warnings.is_empty());
}
#[test]
fn test_verify_integrity() {
    let statutes = vec![Statute::new(
        "test-1",
        "Test",
        Effect::new(EffectType::Grant, "Test"),
    )];
    let result = verify_integrity(&statutes).unwrap();
    assert!(result.passed);
}
#[test]
fn test_complexity_simple() {
    let statute = Statute::new(
        "simple-1",
        "Simple Statute",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let metrics = analyze_complexity(&statute);
    assert_eq!(metrics.condition_count, 1);
    assert_eq!(metrics.condition_depth, 1);
    assert_eq!(metrics.logical_operator_count, 0);
    assert_eq!(metrics.complexity_level, ComplexityLevel::Simple);
}
#[test]
fn test_complexity_with_and() {
    let statute = Statute::new(
        "and-1",
        "AND Statute",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        }),
    ));
    let metrics = analyze_complexity(&statute);
    assert_eq!(metrics.condition_count, 1);
    assert_eq!(metrics.condition_depth, 2);
    assert_eq!(metrics.logical_operator_count, 1);
    assert_eq!(metrics.condition_type_count, 2);
}
#[test]
fn test_complexity_nested() {
    let statute = Statute::new(
        "nested-1",
        "Nested Statute",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Or(
        Box::new(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        )),
        Box::new(Condition::HasAttribute {
            key: "disabled".to_string(),
        }),
    ))
    .with_discretion("Consider special circumstances");
    let metrics = analyze_complexity(&statute);
    assert_eq!(metrics.condition_depth, 3);
    assert_eq!(metrics.logical_operator_count, 2);
    assert!(metrics.has_discretion);
    assert!(metrics.complexity_score > 25);
}
#[test]
fn test_complexity_report() {
    let statutes = vec![
        Statute::new("s1", "Simple", Effect::new(EffectType::Grant, "Test")),
        Statute::new(
            "s2",
            "With Condition",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        }),
    ];
    let report = complexity_report(&statutes);
    assert!(report.contains("# Statute Complexity Report"));
    assert!(report.contains("s1"));
    assert!(report.contains("s2"));
    assert!(report.contains("## Summary"));
}
#[test]
fn test_complexity_with_calculation() {
    let statute = Statute::new(
        "calc-test",
        "Calculation Test",
        Effect::new(EffectType::Grant, "Tax benefit"),
    )
    .with_precondition(Condition::Calculation {
        formula: "income * 0.2".to_string(),
        operator: ComparisonOp::GreaterThan,
        value: 1000.0,
    });
    let metrics = analyze_complexity(&statute);
    assert_eq!(metrics.condition_count, 1);
    assert_eq!(metrics.condition_depth, 1);
    assert_eq!(metrics.condition_type_count, 1);
    assert_eq!(metrics.logical_operator_count, 0);
}
#[test]
fn test_complexity_with_mixed_calculation() {
    let statute = Statute::new(
        "mixed-test",
        "Mixed Calculation Test",
        Effect::new(EffectType::Grant, "Complex benefit"),
    )
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(Condition::Calculation {
            formula: "net_worth / annual_income".to_string(),
            operator: ComparisonOp::LessThan,
            value: 5.0,
        }),
    ));
    let metrics = analyze_complexity(&statute);
    assert_eq!(metrics.condition_count, 1);
    assert_eq!(metrics.condition_depth, 2);
    assert_eq!(metrics.condition_type_count, 2);
    assert_eq!(metrics.logical_operator_count, 1);
}
#[test]
fn test_json_export() {
    let result = VerificationResult::pass()
        .with_warning("Test warning")
        .with_suggestion("Test suggestion");
    let json = result.to_json().unwrap();
    assert!(json.contains("passed"));
    assert!(json.contains("Test warning"));
    assert!(json.contains("Test suggestion"));
}
#[test]
fn test_json_roundtrip() {
    let original = VerificationResult::fail(vec![VerificationError::CircularReference {
        message: "Test cycle".to_string(),
    }])
    .with_warning("Test warning");
    let json = original.to_json().unwrap();
    let restored = VerificationResult::from_json(&json).unwrap();
    assert_eq!(original.passed, restored.passed);
    assert_eq!(original.errors.len(), restored.errors.len());
    assert_eq!(original.warnings.len(), restored.warnings.len());
}
#[test]
fn test_html_report_generation() {
    let result = VerificationResult::fail(vec![VerificationError::DeadStatute {
        statute_id: "test-1".to_string(),
    }])
    .with_warning("Test warning")
    .with_suggestion("Test suggestion");
    let html = generate_html_report(&result, "Test Report");
    assert!(html.contains("<html"));
    assert!(html.contains("Test Report"));
    assert!(html.contains("test-1"));
    assert!(html.contains("Test warning"));
    assert!(html.contains("Test suggestion"));
    assert!(html.contains("Verification Failed"));
}
#[test]
fn test_sarif_report_generation() {
    let result = VerificationResult::fail(vec![VerificationError::LogicalContradiction {
        message: "Test contradiction".to_string(),
    }])
    .with_warning("Test warning");
    let sarif = generate_sarif_report(&result, "legalis-verifier", "0.2.0").unwrap();
    assert!(sarif.contains("2.1.0"));
    assert!(sarif.contains("legalis-verifier"));
    assert!(sarif.contains("logical-contradiction"));
    assert!(sarif.contains("Test contradiction"));
}
#[test]
fn test_circular_reference_detection() {
    let statute1 = Statute::new(
        "statute-a",
        "Statute A",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Custom {
        description: "statute:statute-b".to_string(),
    });
    let statute2 = Statute::new(
        "statute-b",
        "Statute B",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Custom {
        description: "statute:statute-a".to_string(),
    });
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&[statute1, statute2]);
    assert!(!result.passed);
    assert!(!result.errors.is_empty());
    assert!(
        result
            .errors
            .iter()
            .any(|e| matches!(e, VerificationError::CircularReference { .. }))
    );
}
#[test]
fn test_no_circular_reference() {
    let statute1 = Statute::new(
        "statute-a",
        "Statute A",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let statute2 = Statute::new(
        "statute-b",
        "Statute B",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::LessThan,
        value: 50000,
    });
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&[statute1, statute2]);
    assert!(
        result
            .errors
            .iter()
            .all(|e| !matches!(e, VerificationError::CircularReference { .. }))
    );
}
#[test]
fn test_coverage_analysis() {
    let statutes = vec![
        Statute::new(
            "test-1",
            "Test Statute 1",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Statute::new(
            "test-2",
            "Test Statute 2",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        }),
    ];
    let coverage = analyze_coverage(&statutes);
    assert_eq!(coverage.total_conditions, 2);
    assert!(coverage.coverage_percentage >= 0.0);
    assert!(coverage.coverage_percentage <= 100.0);
}
#[test]
fn test_coverage_report() {
    let statutes = vec![
        Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
    ];
    let coverage = analyze_coverage(&statutes);
    let report = coverage.report();
    assert!(report.contains("Condition Coverage Report"));
    assert!(report.contains("Total Conditions:"));
    assert!(report.contains("Coverage:"));
}
#[test]
fn test_coverage_info_new() {
    let coverage = CoverageInfo::new();
    assert_eq!(coverage.total_conditions, 0);
    assert_eq!(coverage.coverage_percentage, 0.0);
    assert!(coverage.covered_conditions.is_empty());
    assert!(coverage.uncovered_conditions.is_empty());
}
#[test]
fn test_coverage_compute_percentage() {
    let mut coverage = CoverageInfo::new();
    coverage.total_conditions = 10;
    coverage
        .covered_conditions
        .insert("test".to_string(), vec![0, 1, 2, 3, 4]);
    coverage.compute_percentage();
    assert_eq!(coverage.coverage_percentage, 50.0);
}
#[test]
fn test_semantic_similarity_identical() {
    let statute1 = Statute::new(
        "test-1",
        "Tax Credit",
        Effect::new(EffectType::Grant, "Grant tax credit"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let statute2 = Statute::new(
        "test-2",
        "Tax Credit",
        Effect::new(EffectType::Grant, "Grant tax credit"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let similarity = semantic_similarity(&statute1, &statute2);
    assert!(similarity.is_high());
    assert!(similarity.0 > 0.8);
}
#[test]
fn test_semantic_similarity_different() {
    let statute1 = Statute::new(
        "test-1",
        "Tax Credit",
        Effect::new(EffectType::Grant, "Grant tax credit"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let statute2 = Statute::new(
        "test-2",
        "Parking Fine",
        Effect::new(EffectType::Obligation, "Pay fine"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::LessThan,
        value: 30000,
    });
    let similarity = semantic_similarity(&statute1, &statute2);
    assert!(similarity.is_low());
    assert!(similarity.0 < 0.5);
}
#[test]
fn test_find_similar_statutes() {
    let statutes = vec![
        Statute::new(
            "test-1",
            "Tax Credit A",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Statute::new(
            "test-2",
            "Tax Credit B",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        }),
        Statute::new(
            "test-3",
            "Parking Fine",
            Effect::new(EffectType::Obligation, "Pay fine"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 30000,
        }),
    ];
    let similar = find_similar_statutes(&statutes, 0.7);
    assert!(!similar.is_empty());
}
#[test]
fn test_string_similarity() {
    assert_eq!(string_similarity("hello", "hello"), 1.0);
    assert_eq!(string_similarity("", ""), 1.0);
    assert_eq!(string_similarity("hello", ""), 0.0);
    assert!(string_similarity("hello", "hallo") > 0.5);
    assert!(string_similarity("hello", "world") < 0.5);
}
#[test]
fn test_similarity_score() {
    let score = SimilarityScore::new(0.85);
    assert!(score.is_high());
    assert!(!score.is_moderate());
    assert!(!score.is_low());
    let score = SimilarityScore::new(0.6);
    assert!(!score.is_high());
    assert!(score.is_moderate());
    assert!(!score.is_low());
    let score = SimilarityScore::new(0.3);
    assert!(!score.is_high());
    assert!(!score.is_moderate());
    assert!(score.is_low());
}
#[test]
fn test_find_ambiguous_terms() {
    let statutes = vec![
        Statute::new(
            "test-1",
            "Tax benefit for persons",
            Effect::new(EffectType::Grant, "Grant to eligible person"),
        ),
        Statute::new(
            "test-2",
            "Child support",
            Effect::new(EffectType::Obligation, "Pay support"),
        ),
    ];
    let ambiguous = find_ambiguous_terms(&statutes);
    assert!(!ambiguous.is_empty());
    let person_term = ambiguous.iter().find(|t| t.term == "person");
    assert!(person_term.is_some());
    let child_term = ambiguous.iter().find(|t| t.term == "child");
    assert!(child_term.is_some());
}
#[test]
fn test_term_disambiguation_report() {
    let statutes = vec![Statute::new(
        "test-1",
        "Income tax benefit",
        Effect::new(EffectType::Grant, "Grant tax benefit"),
    )];
    let report = term_disambiguation_report(&statutes);
    assert!(report.contains("Term Disambiguation Report"));
    assert!(report.contains("income") || report.contains("tax") || report.contains("benefit"));
}
#[test]
fn test_ambiguous_term_builder() {
    let term = AmbiguousTerm::new("test")
        .with_context("context1")
        .with_statute_id("statute1")
        .with_suggestion("suggestion1");
    assert_eq!(term.term, "test");
    assert_eq!(term.contexts.len(), 1);
    assert_eq!(term.statute_ids.len(), 1);
    assert_eq!(term.suggestions.len(), 1);
}
#[test]
fn test_validate_cross_references_valid() {
    let statute1 = Statute::new(
        "statute-a",
        "Statute A",
        Effect::new(EffectType::Grant, "Test"),
    );
    let statute2 = Statute::new(
        "statute-b",
        "Statute B",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Custom {
        description: "statute:statute-a".to_string(),
    });
    let errors = validate_cross_references(&[statute1, statute2]);
    assert!(errors.is_empty());
}
#[test]
fn test_validate_cross_references_invalid() {
    let statute = Statute::new(
        "statute-a",
        "Statute A",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Custom {
        description: "statute:non-existent".to_string(),
    });
    let errors = validate_cross_references(&[statute]);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].error_type, CrossReferenceErrorType::NotFound);
    assert_eq!(errors[0].referenced_statute_id, "non-existent");
}
#[test]
fn test_cross_reference_report() {
    let statute = Statute::new(
        "statute-a",
        "Statute A",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Custom {
        description: "statute:missing".to_string(),
    });
    let report = cross_reference_report(&[statute]);
    assert!(report.contains("Cross-Reference Validation Report"));
    assert!(report.contains("missing"));
}
#[test]
fn test_cross_reference_error_display() {
    let error = CrossReferenceError {
        source_statute_id: "statute-a".to_string(),
        referenced_statute_id: "statute-b".to_string(),
        error_type: CrossReferenceErrorType::NotFound,
    };
    let display = format!("{}", error);
    assert!(display.contains("statute-a"));
    assert!(display.contains("statute-b"));
    assert!(display.contains("non-existent"));
}
#[test]
fn test_terminology_consistency() {
    let statutes = vec![
        Statute::new(
            "test-1",
            "Minor support benefit",
            Effect::new(EffectType::Grant, "Grant benefit to child"),
        ),
        Statute::new(
            "test-2",
            "Juvenile assistance",
            Effect::new(EffectType::Grant, "Grant assistance to juvenile"),
        ),
    ];
    let inconsistencies = check_terminology_consistency(&statutes);
    assert!(!inconsistencies.is_empty());
}
#[test]
fn test_terminology_consistency_report() {
    let statutes = vec![
        Statute::new(
            "test-1",
            "Income benefit",
            Effect::new(EffectType::Grant, "Grant benefit"),
        ),
        Statute::new(
            "test-2",
            "Earnings benefit",
            Effect::new(EffectType::Grant, "Grant benefit"),
        ),
    ];
    let report = terminology_consistency_report(&statutes);
    assert!(report.contains("Terminology Consistency Report"));
}
#[test]
fn test_terminology_inconsistency_builder() {
    let inconsistency = TerminologyInconsistency::new("canonical")
        .with_variation("var1")
        .with_variation("var2")
        .with_statute_id("statute1");
    assert_eq!(inconsistency.canonical_term, "canonical");
    assert_eq!(inconsistency.variations.len(), 2);
    assert_eq!(inconsistency.statute_ids.len(), 1);
}
#[test]
fn test_incremental_state() {
    let mut state = IncrementalState::new();
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    assert!(state.has_changed(&statute));
    let result = VerificationResult::pass();
    state.update(&statute, result.clone());
    assert!(!state.has_changed(&statute));
    let modified_statute = Statute::new(
        "test-1",
        "Modified Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    assert!(state.has_changed(&modified_statute));
}
#[test]
fn test_verify_incremental() {
    let verifier = StatuteVerifier::new();
    let mut state = IncrementalState::new();
    let statute1 = Statute::new("test-1", "Test 1", Effect::new(EffectType::Grant, "Test"));
    let statute2 = Statute::new("test-2", "Test 2", Effect::new(EffectType::Grant, "Test"));
    let result1 = verifier.verify_incremental(&[statute1.clone(), statute2.clone()], &mut state);
    assert!(result1.passed);
    let result2 = verifier.verify_incremental(&[statute1.clone(), statute2.clone()], &mut state);
    assert!(result2.passed);
    let modified_statute1 = Statute::new(
        "test-1",
        "Modified Test 1",
        Effect::new(EffectType::Grant, "Test"),
    );
    let result3 = verifier.verify_incremental(&[modified_statute1, statute2], &mut state);
    assert!(result3.passed);
}
#[test]
fn test_verification_budget() {
    let budget = VerificationBudget::with_max_statutes(5);
    assert!(!budget.statute_limit_reached(4));
    assert!(budget.statute_limit_reached(5));
    let budget = VerificationBudget::with_max_checks(10);
    assert!(!budget.check_limit_reached(9));
    assert!(budget.check_limit_reached(10));
    let budget = VerificationBudget::unlimited();
    assert!(!budget.statute_limit_reached(1000));
    assert!(!budget.check_limit_reached(1000));
}
#[test]
fn test_verify_with_budget() {
    let verifier = StatuteVerifier::new();
    let statutes = vec![
        Statute::new("test-1", "Test 1", Effect::new(EffectType::Grant, "Test")),
        Statute::new("test-2", "Test 2", Effect::new(EffectType::Grant, "Test")),
        Statute::new("test-3", "Test 3", Effect::new(EffectType::Grant, "Test")),
    ];
    let budget = VerificationBudget::unlimited();
    let (result, verified, _checks, exceeded) = verifier.verify_with_budget(&statutes, budget);
    assert!(result.passed);
    assert_eq!(verified, 3);
    assert!(!exceeded);
    let budget = VerificationBudget::with_max_statutes(1);
    let (result, verified, _checks, exceeded) = verifier.verify_with_budget(&statutes, budget);
    assert!(result.passed);
    assert_eq!(verified, 1);
    assert!(exceeded);
    let budget = VerificationBudget::with_max_checks(5);
    let (_result, verified, _checks, exceeded) = verifier.verify_with_budget(&statutes, budget);
    assert!(verified < 3);
    assert!(exceeded);
}
#[test]
fn test_equality_check() {
    let statute = Statute::new(
        "test-1",
        "Senior benefit",
        Effect::new(EffectType::Grant, "Grant benefit"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 70,
    });
    let result = check_equality(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
}
#[test]
fn test_due_process_check() {
    let statute = Statute::new(
        "test-1",
        "License revocation",
        Effect::new(EffectType::Revoke, "Revoke license"),
    );
    let result = check_due_process(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
    let statute_with_discretion = Statute::new(
        "test-2",
        "License revocation with review",
        Effect::new(EffectType::Revoke, "Revoke license"),
    )
    .with_discretion("Review individual circumstances");
    let result2 = check_due_process(&statute_with_discretion);
    assert!(result2.passed);
}
#[test]
fn test_privacy_impact_check() {
    let statute = Statute::new(
        "test-1",
        "Medical benefit",
        Effect::new(EffectType::Grant, "Grant benefit"),
    )
    .with_precondition(Condition::HasAttribute {
        key: "medical_history".to_string(),
    });
    let result = check_privacy_impact(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
    assert!(!result.suggestions.is_empty());
}
#[test]
fn test_proportionality_check() {
    let statute = Statute::new(
        "test-1",
        "Prohibition",
        Effect::new(EffectType::Prohibition, "Prohibit action"),
    );
    let result = check_proportionality(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
    let mut complex_statute = Statute::new(
        "test-2",
        "Complex grant",
        Effect::new(EffectType::Grant, "Grant benefit"),
    );
    for i in 0..6 {
        complex_statute = complex_statute.with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18 + i,
        });
    }
    let result2 = check_proportionality(&complex_statute);
    assert!(!result2.passed);
}
#[test]
fn test_principle_check_result() {
    let result = PrincipleCheckResult::pass();
    assert!(result.passed);
    assert!(result.issues.is_empty());
    let result = PrincipleCheckResult::fail(vec!["Issue 1".to_string()]).with_suggestion("Fix it");
    assert!(!result.passed);
    assert_eq!(result.issues.len(), 1);
    assert_eq!(result.suggestions.len(), 1);
}
#[test]
fn test_accessibility_check() {
    let statute = Statute::new(
        "test-1",
        "Physical test requirement",
        Effect::new(EffectType::Grant, "Grant benefit"),
    )
    .with_precondition(Condition::HasAttribute {
        key: "physical_fitness".to_string(),
    });
    let result = check_accessibility(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
    assert!(!result.suggestions.is_empty());
    let statute2 = Statute::new(
        "test-2",
        "Online registration",
        Effect::new(EffectType::Obligation, "Register online"),
    )
    .with_precondition(Condition::HasAttribute {
        key: "internet_access".to_string(),
    });
    let result2 = check_accessibility(&statute2);
    assert!(!result2.passed);
    assert!(result2.issues.iter().any(|i| i.contains("internet")));
}
#[test]
fn test_impact_assessment() {
    let statute = Statute::new(
        "test-1",
        "Senior benefit",
        Effect::new(EffectType::Grant, "Grant senior benefit"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 65,
    });
    let assessment = assess_impact(&statute);
    assert!(assessment.affected_groups.contains(&"Seniors".to_string()));
    assert!(!assessment.positive_impacts.is_empty());
    let statute2 = Statute::new(
        "test-2",
        "License revocation",
        Effect::new(EffectType::Revoke, "Revoke license"),
    );
    let assessment2 = assess_impact(&statute2);
    assert!(!assessment2.negative_impacts.is_empty());
    assert!(assessment2.overall_risk >= RiskLevel::High);
}
#[test]
fn test_assess_multiple_impacts() {
    let statutes = vec![
        Statute::new(
            "test-1",
            "Tax benefit",
            Effect::new(EffectType::Grant, "Grant tax benefit"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        }),
        Statute::new(
            "test-2",
            "License requirement",
            Effect::new(EffectType::Obligation, "Obtain license"),
        ),
    ];
    let report = assess_multiple_impacts(&statutes);
    assert!(report.contains("Comprehensive Impact Assessment"));
    assert!(report.contains("Overall Summary"));
}
#[test]
fn test_impact_levels() {
    assert_eq!(format!("{}", ImpactLevel::Low), "Low");
    assert_eq!(format!("{}", ImpactLevel::Medium), "Medium");
    assert_eq!(format!("{}", ImpactLevel::High), "High");
    assert_eq!(format!("{}", RiskLevel::Low), "Low");
    assert_eq!(format!("{}", RiskLevel::Critical), "Critical");
}
#[test]
fn test_impact_assessment_report() {
    let mut assessment = ImpactAssessment::new();
    assessment.affected_groups.push("Test group".to_string());
    assessment
        .positive_impacts
        .push("Positive impact".to_string());
    assessment.overall_risk = RiskLevel::Medium;
    let report = assessment.report();
    assert!(report.contains("Impact Assessment Report"));
    assert!(report.contains("Test group"));
    assert!(report.contains("Medium"));
}
#[test]
fn test_ltl_atom() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1").with_proposition("p");
    system.add_state(s1);
    system.add_initial_state("s1");
    let formula = LtlFormula::atom("p");
    assert!(verify_ltl(&system, &formula));
    let formula2 = LtlFormula::atom("q");
    assert!(!verify_ltl(&system, &formula2));
}
#[test]
fn test_ltl_next() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1");
    let s2 = TemporalState::new("s2").with_proposition("p");
    system.add_state(s1);
    system.add_state(s2);
    system.add_transition("s1", "s2");
    system.add_initial_state("s1");
    let formula = LtlFormula::next(LtlFormula::atom("p"));
    assert!(verify_ltl(&system, &formula));
}
#[test]
fn test_ltl_always() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1").with_proposition("p");
    let s2 = TemporalState::new("s2").with_proposition("p");
    system.add_state(s1);
    system.add_state(s2);
    system.add_transition("s1", "s2");
    system.add_initial_state("s1");
    let formula = LtlFormula::always(LtlFormula::atom("p"));
    assert!(verify_ltl(&system, &formula));
}
#[test]
fn test_ltl_eventually() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1");
    let s2 = TemporalState::new("s2");
    let s3 = TemporalState::new("s3").with_proposition("p");
    system.add_state(s1);
    system.add_state(s2);
    system.add_state(s3);
    system.add_transition("s1", "s2");
    system.add_transition("s2", "s3");
    system.add_initial_state("s1");
    let formula = LtlFormula::eventually(LtlFormula::atom("p"));
    assert!(verify_ltl(&system, &formula));
}
#[test]
fn test_ltl_display() {
    let formula = LtlFormula::always(LtlFormula::atom("p"));
    assert_eq!(format!("{}", formula), "G(p)");
    let formula2 = LtlFormula::eventually(LtlFormula::atom("q"));
    assert_eq!(format!("{}", formula2), "F(q)");
}
#[test]
fn test_ctl_exists_next() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1");
    let s2 = TemporalState::new("s2").with_proposition("p");
    let s3 = TemporalState::new("s3");
    system.add_state(s1);
    system.add_state(s2);
    system.add_state(s3);
    system.add_transition("s1", "s2");
    system.add_transition("s1", "s3");
    system.add_initial_state("s1");
    let formula = CtlFormula::exists_next(CtlFormula::atom("p"));
    assert!(verify_ctl(&system, &formula));
}
#[test]
fn test_ctl_all_next() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1");
    let s2 = TemporalState::new("s2").with_proposition("p");
    let s3 = TemporalState::new("s3").with_proposition("p");
    system.add_state(s1);
    system.add_state(s2);
    system.add_state(s3);
    system.add_transition("s1", "s2");
    system.add_transition("s1", "s3");
    system.add_initial_state("s1");
    let formula = CtlFormula::all_next(CtlFormula::atom("p"));
    assert!(verify_ctl(&system, &formula));
}
#[test]
fn test_ctl_display() {
    let formula = CtlFormula::exists_eventually(CtlFormula::atom("p"));
    assert_eq!(format!("{}", formula), "EF(p)");
    let formula2 = CtlFormula::all_always(CtlFormula::atom("q"));
    assert_eq!(format!("{}", formula2), "AG(q)");
}
#[test]
fn test_deadline_verification_pass() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1");
    let s2 = TemporalState::new("s2");
    let s3 = TemporalState::new("s3").with_proposition("completed");
    system.add_state(s1);
    system.add_state(s2);
    system.add_state(s3);
    system.add_transition("s1", "s2");
    system.add_transition("s2", "s3");
    system.add_initial_state("s1");
    let deadline = Deadline::new("d1", "completed", 5);
    let result = verify_deadlines(&system, &[deadline]);
    assert!(result.passed);
    assert!(result.violations.is_empty());
}
#[test]
fn test_deadline_verification_fail() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1");
    let s2 = TemporalState::new("s2");
    let s3 = TemporalState::new("s3").with_proposition("completed");
    system.add_state(s1);
    system.add_state(s2);
    system.add_state(s3);
    system.add_transition("s1", "s2");
    system.add_transition("s2", "s3");
    system.add_initial_state("s1");
    let deadline = Deadline::new("d1", "completed", 1);
    let result = verify_deadlines(&system, &[deadline]);
    assert!(!result.passed);
    assert!(!result.violations.is_empty());
}
#[test]
fn test_sequence_verification_pass() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1").with_proposition("start");
    let s2 = TemporalState::new("s2").with_proposition("middle");
    let s3 = TemporalState::new("s3").with_proposition("end");
    system.add_state(s1);
    system.add_state(s2);
    system.add_state(s3);
    system.add_transition("s1", "s2");
    system.add_transition("s2", "s3");
    system.add_initial_state("s1");
    let constraint = SequenceConstraint::new(
        "seq1",
        vec!["start".to_string(), "middle".to_string(), "end".to_string()],
    );
    let result = verify_sequences(&system, &[constraint]);
    assert!(result.passed);
}
#[test]
fn test_sequence_verification_fail() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1").with_proposition("start");
    let s2 = TemporalState::new("s2").with_proposition("end");
    system.add_state(s1);
    system.add_state(s2);
    system.add_transition("s1", "s2");
    system.add_initial_state("s1");
    let constraint = SequenceConstraint::new(
        "seq1",
        vec!["start".to_string(), "middle".to_string(), "end".to_string()],
    );
    let result = verify_sequences(&system, &[constraint]);
    assert!(!result.passed);
}
#[test]
fn test_temporal_state_creation() {
    let state = TemporalState::new("s1")
        .with_proposition("p")
        .with_proposition("q");
    assert_eq!(state.id, "s1");
    assert!(state.satisfies("p"));
    assert!(state.satisfies("q"));
    assert!(!state.satisfies("r"));
}
#[test]
fn test_transition_system_creation() {
    let mut system = TransitionSystem::new();
    let s1 = TemporalState::new("s1").with_proposition("p");
    let s2 = TemporalState::new("s2").with_proposition("q");
    system.add_state(s1);
    system.add_state(s2);
    system.add_transition("s1", "s2");
    system.add_initial_state("s1");
    assert_eq!(system.states.len(), 2);
    assert!(system.initial_states.contains("s1"));
    assert_eq!(system.successors("s1").len(), 1);
}
#[test]
fn test_principle_definition_creation() {
    let principle = PrincipleDefinition::new("test", "Test Principle", "A test")
        .with_priority(10)
        .with_jurisdiction("US")
        .with_check(PrincipleCheck::NoDiscrimination);
    assert_eq!(principle.id, "test");
    assert_eq!(principle.priority, 10);
    assert_eq!(principle.jurisdiction, Some("US".to_string()));
    assert_eq!(principle.checks.len(), 1);
}
#[test]
fn test_composite_principle_creation() {
    let composite = CompositePrinciple::new("comp1", "Composite")
        .with_component("p1")
        .with_component("p2")
        .with_mode(CombinationMode::All);
    assert_eq!(composite.id, "comp1");
    assert_eq!(composite.components.len(), 2);
    assert_eq!(composite.combination_mode, CombinationMode::All);
}
#[test]
fn test_jurisdictional_rule_set() {
    let principle = PrincipleDefinition::new("p1", "Principle 1", "Test").with_priority(10);
    let rule_set = JurisdictionalRuleSet::new("US", "United States").with_principle(principle);
    assert_eq!(rule_set.jurisdiction, "US");
    assert_eq!(rule_set.principles.len(), 1);
}
#[test]
fn test_principle_registry() {
    let mut registry = PrincipleRegistry::new();
    let principle = PrincipleDefinition::new("p1", "Test", "Description")
        .with_check(PrincipleCheck::NoDiscrimination);
    let rule_set = JurisdictionalRuleSet::new("US", "United States").with_principle(principle);
    registry.add_jurisdiction(rule_set);
    assert!(registry.get_jurisdiction("US").is_some());
    assert!(registry.get_jurisdiction("UK").is_none());
}
#[test]
fn test_verify_for_jurisdiction() {
    let mut registry = PrincipleRegistry::new();
    let principle = PrincipleDefinition::new("equality", "Equality", "Equal treatment")
        .with_priority(10)
        .with_check(PrincipleCheck::NoDiscrimination);
    let rule_set = JurisdictionalRuleSet::new("US", "United States").with_principle(principle);
    registry.add_jurisdiction(rule_set);
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    );
    let result = registry.verify_for_jurisdiction(&statute, "US");
    let _ = result.passed;
}
#[test]
fn test_retroactivity_check_pass() {
    use chrono::{NaiveDate, Utc};
    let statute = Statute::new(
        "test-1",
        "Traffic prohibition",
        Effect::new(EffectType::Prohibition, "Prohibit parking"),
    )
    .with_temporal_validity(
        TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2026, 6, 1).unwrap())
            .with_enacted_at(Utc::now()),
    );
    let result = check_retroactivity(&statute);
    assert!(result.passed);
    assert!(result.issues.is_empty());
}
