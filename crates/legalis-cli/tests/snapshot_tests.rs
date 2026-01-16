//! Snapshot tests for CLI output formats.
//!
//! These tests capture and compare the output of commands across different runs
//! to detect unexpected changes in output formatting.

use insta::assert_snapshot;
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_dsl::LegalDslParser;

/// Helper to create a test statute.
fn create_simple_statute() -> Statute {
    Statute::new(
        "snapshot-test",
        "Snapshot Test Act",
        Effect::new(EffectType::Grant, "Test right"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_jurisdiction("US")
}

/// Helper to create a complex statute.
fn create_complex_statute() -> Statute {
    Statute::new(
        "complex-snapshot",
        "Complex Snapshot Test Act",
        Effect::new(EffectType::Grant, "Complex right"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterThan,
        value: 30000,
    })
    .with_discretion("Consider individual circumstances")
    .with_jurisdiction("US-CA")
    .with_version(2)
}

#[test]
fn test_statute_json_serialization_snapshot() {
    let statute = create_simple_statute();
    let json = serde_json::to_string_pretty(&statute).unwrap();
    assert_snapshot!(json);
}

#[test]
fn test_statute_yaml_serialization_snapshot() {
    let statute = create_simple_statute();
    let yaml = serde_yaml::to_string(&statute).unwrap();
    assert_snapshot!(yaml);
}

#[test]
fn test_complex_statute_json_snapshot() {
    let statute = create_complex_statute();
    let json = serde_json::to_string_pretty(&statute).unwrap();
    assert_snapshot!(json);
}

#[test]
fn test_complex_statute_yaml_snapshot() {
    let statute = create_complex_statute();
    let yaml = serde_yaml::to_string(&statute).unwrap();
    assert_snapshot!(yaml);
}

#[test]
fn test_dsl_parsing_snapshot() {
    let dsl = r#"
STATUTE test-parsing: "Test Parsing Act" {
    WHEN AGE >= 21 AND INCOME > 25000
    THEN GRANT "Special privilege"
    DISCRETION "May require additional documentation"
}
"#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(dsl).unwrap();
    let json = serde_json::to_string_pretty(&statute).unwrap();
    assert_snapshot!(json);
}

#[test]
fn test_verification_output_snapshot() {
    use legalis_verifier::StatuteVerifier;

    let statute = create_simple_statute();
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&[statute]);

    // Create a simplified representation for snapshot
    let snapshot_data = serde_json::json!({
        "passed": result.passed,
        "error_count": result.errors.len(),
        "warning_count": result.warnings.len(),
    });

    assert_snapshot!(serde_json::to_string_pretty(&snapshot_data).unwrap());
}

#[test]
fn test_complexity_analysis_snapshot() {
    use legalis_verifier::analyze_complexity;

    let statute = create_complex_statute();
    let metrics = analyze_complexity(&statute);

    let snapshot_data = serde_json::json!({
        "complexity_score": metrics.complexity_score,
        "condition_count": metrics.condition_count,
        "logical_operator_count": metrics.logical_operator_count,
        "has_discretion": metrics.has_discretion,
        "condition_depth": metrics.condition_depth,
    });

    assert_snapshot!(serde_json::to_string_pretty(&snapshot_data).unwrap());
}

#[test]
fn test_visualization_mermaid_snapshot() {
    use legalis_viz::DecisionTree;

    let statute = create_simple_statute();
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let mermaid = tree.to_mermaid();

    assert_snapshot!(mermaid);
}

#[test]
fn test_visualization_dot_snapshot() {
    use legalis_viz::DecisionTree;

    let statute = create_simple_statute();
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let dot = tree.to_dot();

    assert_snapshot!(dot);
}

#[test]
fn test_visualization_ascii_snapshot() {
    use legalis_viz::DecisionTree;

    let statute = create_simple_statute();
    let tree = DecisionTree::from_statute(&statute).unwrap();
    let ascii = tree.to_ascii();

    assert_snapshot!(ascii);
}

#[test]
fn test_diff_output_snapshot() {
    let old_statute = Statute::new(
        "diff-snapshot",
        "Original Title",
        Effect::new(EffectType::Grant, "Original right"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    let new_statute = Statute::new(
        "diff-snapshot",
        "Updated Title",
        Effect::new(EffectType::Grant, "Updated right"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 21,
    });

    let diff = legalis_diff::diff(&old_statute, &new_statute).unwrap();
    let json = serde_json::to_string_pretty(&diff).unwrap();

    assert_snapshot!(json);
}

#[test]
fn test_printer_output_snapshot() {
    use legalis_dsl::{DslPrinter, PrinterConfig};

    let statute = create_simple_statute();
    let printer = DslPrinter::with_config(PrinterConfig::default());
    let output = printer.format(&statute);

    assert_snapshot!(output);
}

#[test]
fn test_printer_compact_output_snapshot() {
    use legalis_dsl::{DslPrinter, PrinterConfig};

    let statute = create_complex_statute();
    let printer = DslPrinter::with_config(PrinterConfig::compact());
    let output = printer.format(&statute);

    assert_snapshot!(output);
}

#[test]
fn test_printer_verbose_output_snapshot() {
    use legalis_dsl::{DslPrinter, PrinterConfig};

    let statute = create_complex_statute();
    let printer = DslPrinter::with_config(PrinterConfig::verbose());
    let output = printer.format(&statute);

    assert_snapshot!(output);
}

#[test]
fn test_statute_display_snapshot() {
    let statute = create_complex_statute();
    let display = format!("{}", statute);
    assert_snapshot!(display);
}

#[test]
fn test_condition_display_snapshots() {
    let conditions = [
        Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        },
        Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        },
        Condition::HasAttribute {
            key: "citizen".to_string(),
        },
        Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 21,
            }),
            Box::new(Condition::HasAttribute {
                key: "licensed".to_string(),
            }),
        ),
    ];

    for (i, condition) in conditions.iter().enumerate() {
        let display = format!("{}", condition);
        insta::with_settings!({snapshot_suffix => format!("condition_{}", i)}, {
            assert_snapshot!(display);
        });
    }
}

#[test]
fn test_complexity_report_snapshot() {
    let statutes = vec![create_simple_statute(), create_complex_statute()];

    let report = legalis_verifier::complexity_report(&statutes);
    assert_snapshot!(report);
}

// Note: These tests are commented out because the APIs are not available yet
// Uncomment when the APIs are implemented

// #[test]
// fn test_lod_turtle_output_snapshot() {
//     use legalis_lod::{LegalOntologyMapper, RdfFormat};
//
//     let statute = create_simple_statute();
//     let mapper = LegalOntologyMapper::new("https://example.org/legalis/");
//     let rdf = mapper.statute_to_rdf(&statute, RdfFormat::Turtle).unwrap();
//
//     assert_snapshot!(rdf);
// }
//
// #[test]
// fn test_solidity_export_snapshot() {
//     use legalis_chain::SolidityGenerator;
//
//     let statute = create_simple_statute();
//     let generator = SolidityGenerator::new();
//     let solidity = generator.generate(&statute).unwrap();
//
//     assert_snapshot!(solidity);
// }

#[test]
fn test_table_format_output_snapshot() {
    use comfy_table::Table;
    use comfy_table::modifiers::UTF8_ROUND_CORNERS;
    use comfy_table::presets::UTF8_FULL;

    let statutes = vec![create_simple_statute(), create_complex_statute()];

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["ID", "Title", "Jurisdiction", "Version"]);

    for statute in statutes {
        table.add_row(vec![
            statute.id.clone(),
            statute.title.clone(),
            statute
                .jurisdiction
                .clone()
                .unwrap_or_else(|| "-".to_string()),
            statute.version.to_string(),
        ]);
    }

    assert_snapshot!(table.to_string());
}
