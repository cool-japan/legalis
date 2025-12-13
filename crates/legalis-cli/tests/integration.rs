//! Integration tests for Legalis-RS.
//!
//! These tests verify end-to-end functionality across multiple crates.

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute, TemporalValidity};
use legalis_dsl::LegalDslParser;
use legalis_verifier::{StatuteVerifier, analyze_complexity};
use legalis_viz::DecisionTree;

/// Tests the complete DSL -> Core -> Verifier pipeline.
#[test]
fn test_dsl_to_verification_pipeline() {
    let dsl = r#"
        STATUTE adult-voting-rights: "Adult Voting Rights Act" {
            WHEN AGE >= 18
            THEN GRANT "Right to vote in elections"
        }
    "#;

    // Parse DSL
    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(dsl).expect("Failed to parse statute");

    // Verify
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(std::slice::from_ref(&statute));

    assert!(result.passed, "Verification should pass");
    assert!(result.errors.is_empty(), "Should have no errors");
}

/// Tests the complete DSL -> Core -> Visualization pipeline.
#[test]
fn test_dsl_to_visualization_pipeline() {
    let dsl = r#"
        STATUTE senior-benefits: "Senior Benefits Act" {
            WHEN AGE >= 65
            THEN GRANT "Senior citizen benefits"
            DISCRETION "Consider health status for additional benefits"
        }
    "#;

    // Parse DSL
    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(dsl).expect("Failed to parse statute");

    // Create visualization
    let tree = DecisionTree::from_statute(&statute).expect("Failed to create tree");

    // Verify visualization outputs
    let mermaid = tree.to_mermaid();
    assert!(mermaid.contains("flowchart TD"));
    assert!(mermaid.contains("Senior Benefits Act"));

    let ascii = tree.to_ascii();
    assert!(ascii.contains("Senior Benefits Act"));
    assert!(ascii.contains("🔴")); // Discretion node

    let box_viz = tree.to_box();
    assert!(box_viz.contains("Senior Benefits Act"));
}

/// Tests complex nested conditions through the full pipeline.
#[test]
fn test_complex_nested_conditions_pipeline() {
    let dsl = r#"
        STATUTE working-adult-license: "Working Adult Driver License" {
            WHEN AGE >= 18 AND INCOME > 0
            THEN GRANT "Full driver license"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(dsl).expect("Failed to parse statute");

    // Verify the structure
    assert_eq!(statute.preconditions.len(), 1);

    // The precondition should be an AND condition
    match &statute.preconditions[0] {
        Condition::And(left, right) => {
            assert!(matches!(left.as_ref(), Condition::Age { .. }));
            assert!(matches!(right.as_ref(), Condition::Income { .. }));
        }
        _ => panic!("Expected AND condition"),
    }

    // Analyze complexity - the metrics count logical operators
    let metrics = analyze_complexity(&statute);
    // At least one logical operator (AND)
    assert!(
        metrics.logical_operator_count >= 1,
        "Should have at least 1 operator"
    );
}

/// Tests DSL with comments through the pipeline.
#[test]
fn test_dsl_with_comments_pipeline() {
    let dsl = r#"
        // This is a test statute
        STATUTE comment-test: "Comment Test Act" {
            /* Multi-line
               comment here */
            WHEN AGE >= 21  // Drinking age
            THEN GRANT "Can purchase alcohol"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(dsl)
        .expect("Failed to parse statute with comments");

    assert_eq!(statute.id, "comment-test");
    assert_eq!(statute.title, "Comment Test Act");

    // Verify
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&[statute]);
    assert!(result.passed);
}

/// Tests OR conditions through the full pipeline.
#[test]
fn test_or_conditions_pipeline() {
    let dsl = r#"
        STATUTE flexible-eligibility: "Flexible Eligibility Act" {
            WHEN AGE >= 65 OR INCOME < 20000
            THEN GRANT "Social assistance eligibility"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(dsl).expect("Failed to parse statute");

    match &statute.preconditions[0] {
        Condition::Or(left, right) => {
            assert!(matches!(left.as_ref(), Condition::Age { .. }));
            assert!(matches!(right.as_ref(), Condition::Income { .. }));
        }
        _ => panic!("Expected OR condition"),
    }
}

/// Tests NOT conditions through the pipeline.
#[test]
fn test_not_conditions_pipeline() {
    let dsl = r#"
        STATUTE not-minor: "Non-Minor Rights Act" {
            WHEN NOT AGE < 18
            THEN GRANT "Adult privileges"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(dsl).expect("Failed to parse statute");

    match &statute.preconditions[0] {
        Condition::Not(inner) => {
            assert!(matches!(inner.as_ref(), Condition::Age { .. }));
        }
        _ => panic!("Expected NOT condition"),
    }
}

/// Tests operator precedence (AND binds tighter than OR).
#[test]
fn test_operator_precedence_pipeline() {
    let dsl = r#"
        STATUTE precedence-test: "Precedence Test Act" {
            WHEN AGE >= 18 OR AGE < 65 AND INCOME > 50000
            THEN GRANT "Complex eligibility"
        }
    "#;

    // This should parse as: AGE >= 18 OR (AGE < 65 AND INCOME > 50000)
    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(dsl).expect("Failed to parse statute");

    match &statute.preconditions[0] {
        Condition::Or(left, right) => {
            // Left should be simple AGE condition
            assert!(matches!(left.as_ref(), Condition::Age { .. }));
            // Right should be AND condition
            assert!(matches!(right.as_ref(), Condition::And(_, _)));
        }
        _ => panic!("Expected OR at top level (AND should bind tighter)"),
    }
}

/// Tests multiple statutes verification.
#[test]
fn test_multiple_statutes_verification() {
    let parser = LegalDslParser::new();

    let statute1 = parser
        .parse_statute(
            r#"
            STATUTE statute-one: "First Act" {
                WHEN AGE >= 18
                THEN GRANT "Right one"
            }
        "#,
        )
        .unwrap();

    let statute2 = parser
        .parse_statute(
            r#"
            STATUTE statute-two: "Second Act" {
                WHEN AGE >= 21
                THEN GRANT "Right two"
            }
        "#,
        )
        .unwrap();

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&[statute1, statute2]);

    assert!(result.passed);
    assert!(result.errors.is_empty());
}

/// Tests complexity analysis across multiple statutes.
#[test]
fn test_complexity_analysis_integration() {
    let parser = LegalDslParser::new();

    // Simple statute
    let simple = parser
        .parse_statute(
            r#"
            STATUTE simple: "Simple Act" {
                WHEN AGE >= 18
                THEN GRANT "Basic right"
            }
        "#,
        )
        .unwrap();

    // Complex statute - using AND/OR without parentheses (parser supports this)
    let complex = parser
        .parse_statute(
            r#"
            STATUTE complex: "Complex Act" {
                WHEN AGE >= 18 AND INCOME > 30000 OR HAS citizen
                THEN GRANT "Complex right"
                DISCRETION "Consider individual circumstances"
            }
        "#,
        )
        .unwrap();

    let simple_metrics = analyze_complexity(&simple);
    let complex_metrics = analyze_complexity(&complex);

    assert!(
        complex_metrics.complexity_score > simple_metrics.complexity_score,
        "Complex statute should have higher complexity score"
    );
    assert!(complex_metrics.has_discretion);
    assert!(!simple_metrics.has_discretion);
}

/// Tests the full report generation.
#[test]
fn test_complexity_report_generation() {
    let parser = LegalDslParser::new();

    let statutes: Vec<_> = ["statute-a", "statute-b", "statute-c"]
        .iter()
        .enumerate()
        .map(|(i, id)| {
            parser
                .parse_statute(&format!(
                    r#"
                    STATUTE {}: "Act {}" {{
                        WHEN AGE >= {}
                        THEN GRANT "Right {}"
                    }}
                "#,
                    id,
                    i + 1,
                    18 + i,
                    i + 1
                ))
                .unwrap()
        })
        .collect();

    let report = legalis_verifier::complexity_report(&statutes);

    assert!(report.contains("Complexity Report"));
    assert!(report.contains("statute-a"));
    assert!(report.contains("statute-b"));
    assert!(report.contains("statute-c"));
}

/// Tests core Statute builder pattern.
#[test]
fn test_statute_builder_integration() {
    let statute = Statute::new(
        "builder-test",
        "Builder Test Act",
        Effect::new(EffectType::Grant, "Test right"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_precondition(Condition::HasAttribute {
        key: "citizen".to_string(),
    })
    .with_discretion("Consider special cases")
    .with_version(2)
    .with_jurisdiction("US-CA");

    assert_eq!(statute.preconditions.len(), 2);
    assert!(statute.discretion_logic.is_some());
    assert_eq!(statute.version, 2);
    assert_eq!(statute.jurisdiction, Some("US-CA".to_string()));

    // Verify it passes verification
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&[statute]);
    // Note: discretion generates a warning, not an error
    assert!(result.passed);
}

/// Tests temporal validity integration.
#[test]
fn test_temporal_validity_integration() {
    use chrono::{NaiveDate, Utc};

    let statute = Statute::new(
        "temporal-test",
        "Temporal Test Act",
        Effect::new(EffectType::Grant, "Time-bound right"),
    )
    .with_temporal_validity(TemporalValidity {
        effective_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        expiry_date: Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
        enacted_at: Some(Utc::now()),
        amended_at: None,
    });

    // temporal_validity is not Option, it's directly TemporalValidity
    assert!(statute.temporal_validity.effective_date.is_some());
    assert!(statute.temporal_validity.expiry_date.is_some());
}

/// Tests visualization of complex statutes.
#[test]
fn test_visualization_complex_statute() {
    let parser = LegalDslParser::new();

    let statute = parser
        .parse_statute(
            r#"
            STATUTE complex-viz: "Complex Visualization Test" {
                WHEN AGE >= 18 AND HAS citizen
                THEN GRANT "Full rights"
                DISCRETION "Review individual cases"
            }
        "#,
        )
        .unwrap();

    let tree = DecisionTree::from_statute(&statute).unwrap();

    // Test all visualization formats
    let dot = tree.to_dot();
    assert!(!dot.is_empty());

    let mermaid = tree.to_mermaid();
    assert!(mermaid.contains("flowchart TD"));
    assert!(mermaid.contains("Complex Visualization Test"));

    let ascii = tree.to_ascii();
    assert!(ascii.contains("Complex Visualization Test"));

    let box_viz = tree.to_box();
    assert!(box_viz.contains("Complex Visualization Test"));

    // Check node counts
    assert!(tree.node_count() > 0);
    assert!(tree.discretionary_count() > 0);
}

/// Tests diff functionality between two statutes.
#[test]
fn test_diff_integration() {
    use legalis_diff::ChangeTarget;

    let old_statute = Statute::new(
        "diff-test",
        "Original Title",
        Effect::new(EffectType::Grant, "Original right"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    let new_statute = Statute::new(
        "diff-test",
        "Updated Title",
        Effect::new(EffectType::Grant, "Updated right"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 21,
    });

    let diff = legalis_diff::diff(&old_statute, &new_statute).expect("Diff should succeed");

    assert_eq!(diff.statute_id, "diff-test");
    assert!(!diff.changes.is_empty());

    // Check for title change - target is an enum
    let has_title_change = diff
        .changes
        .iter()
        .any(|c| matches!(c.target, ChangeTarget::Title));
    assert!(has_title_change, "Should detect title change");
}

/// Tests error handling for invalid DSL.
#[test]
fn test_invalid_dsl_error_handling() {
    let parser = LegalDslParser::new();

    // Missing STATUTE keyword
    let result = parser.parse_statute(r#"invalid: "No statute" { }"#);
    assert!(result.is_err(), "Should fail without STATUTE keyword");

    // Unclosed comment
    let result = parser.parse_statute(
        r#"
        STATUTE test: "Test" {
            /* unclosed comment
            WHEN AGE >= 18
            THEN GRANT "Right"
        }
    "#,
    );
    assert!(result.is_err(), "Should fail with unclosed comment");

    // Invalid condition (missing operator)
    let result = parser.parse_statute(
        r#"
        STATUTE test: "Test" {
            WHEN AGE 18
            THEN GRANT "Right"
        }
    "#,
    );
    assert!(result.is_err(), "Should fail with invalid condition");
}

/// Tests Display trait implementations.
#[test]
fn test_display_implementations() {
    let statute = Statute::new(
        "display-test",
        "Display Test Act",
        Effect::new(EffectType::Grant, "Test right"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    let display = format!("{}", statute);
    assert!(display.contains("display-test"));
    assert!(display.contains("Display Test Act"));

    let condition = Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    };
    let cond_display = format!("{}", condition);
    assert!(cond_display.contains("18"));
}
