//! Lowering fidelity tests for [`crate::parser::ToCore`].
//!
//! The [`crate::DslPrinter`] emits dedicated syntax for set-membership (`IN`),
//! `LIKE`, and `MATCHES` conditions, and the tokenizer/parser recover the
//! matching [`crate::ast::ConditionNode`] variants. Historically the
//! `ConditionNode -> legalis_core::Condition` lowering collapsed all three into
//! a lossy [`legalis_core::Condition::Custom`]; these tests pin the faithful
//! mapping to [`legalis_core::Condition::SetMembership`] and
//! [`legalis_core::Condition::Pattern`].

use crate::ast::{ConditionNode, ConditionValue};
use crate::parser::ToCore;
use legalis_core::Condition;

/// `IN` with string members lowers to a `SetMembership` (was `Custom`).
#[test]
fn test_in_lowers_to_set_membership() {
    let node = ConditionNode::In {
        field: "category".to_string(),
        values: vec![
            ConditionValue::String("gold".to_string()),
            ConditionValue::String("silver".to_string()),
            ConditionValue::String("bronze".to_string()),
        ],
    };

    match node.to_core().expect("In node must lower") {
        Condition::SetMembership {
            attribute,
            values,
            negated,
        } => {
            assert_eq!(attribute, "category");
            assert_eq!(
                values,
                vec![
                    "gold".to_string(),
                    "silver".to_string(),
                    "bronze".to_string()
                ]
            );
            assert!(!negated);
        }
        other => panic!("expected SetMembership, got {other:?}"),
    }
}

/// `IN` with numeric members lowers to `SetMembership` with stringified values.
#[test]
fn test_in_numeric_lowers_to_set_membership() {
    let node = ConditionNode::In {
        field: "bracket".to_string(),
        values: vec![
            ConditionValue::Number(1),
            ConditionValue::Number(2),
            ConditionValue::Number(3),
        ],
    };

    match node.to_core().expect("In node must lower") {
        Condition::SetMembership {
            attribute, values, ..
        } => {
            assert_eq!(attribute, "bracket");
            assert_eq!(
                values,
                vec!["1".to_string(), "2".to_string(), "3".to_string()]
            );
        }
        other => panic!("expected SetMembership, got {other:?}"),
    }
}

/// `MATCHES` (regex) lowers to a `Pattern` condition (was `Custom`).
#[test]
fn test_matches_lowers_to_pattern() {
    let node = ConditionNode::Matches {
        field: "tax_id".to_string(),
        regex_pattern: "[0-9]{9}".to_string(),
    };

    match node.to_core().expect("Matches node must lower") {
        Condition::Pattern {
            attribute,
            pattern,
            negated,
        } => {
            assert_eq!(attribute, "tax_id");
            assert_eq!(pattern, "[0-9]{9}");
            assert!(!negated);
        }
        other => panic!("expected Pattern, got {other:?}"),
    }
}

/// `LIKE` lowers to a `Pattern` condition (was `Custom`).
#[test]
fn test_like_lowers_to_pattern() {
    let node = ConditionNode::Like {
        field: "name".to_string(),
        pattern: "Smith%".to_string(),
    };

    match node.to_core().expect("Like node must lower") {
        Condition::Pattern {
            attribute,
            pattern,
            negated,
        } => {
            assert_eq!(attribute, "name");
            assert_eq!(pattern, "Smith%");
            assert!(!negated);
        }
        other => panic!("expected Pattern, got {other:?}"),
    }
}

/// Logical composition still lowers recursively, now preserving the leaf
/// `SetMembership`/`Pattern` mappings rather than flattening them to `Custom`.
#[test]
fn test_composed_in_and_matches_lower_faithfully() {
    let node = ConditionNode::And(
        Box::new(ConditionNode::In {
            field: "status".to_string(),
            values: vec![ConditionValue::String("active".to_string())],
        }),
        Box::new(ConditionNode::Matches {
            field: "code".to_string(),
            regex_pattern: "A.*".to_string(),
        }),
    );

    match node.to_core().expect("And node must lower") {
        Condition::And(left, right) => {
            assert!(matches!(*left, Condition::SetMembership { .. }));
            assert!(matches!(*right, Condition::Pattern { .. }));
        }
        other => panic!("expected And, got {other:?}"),
    }
}

/// `AttributeEquals` survives a full `format_statute` → `parse_statute`
/// round-trip — the primary parser now reads the printer's `"key" = "value"`.
#[test]
fn test_attribute_equals_statute_roundtrip() {
    use legalis_core::{Effect, EffectType, Statute};

    let statute = Statute::new(
        "attr-eq",
        "Attribute Equals Round-trip",
        Effect::new(EffectType::Grant, "Eligible"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "residency".to_string(),
        value: "permanent".to_string(),
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [Condition::AttributeEquals { key, value }] => {
            assert_eq!(key, "residency");
            assert_eq!(value, "permanent");
        }
        other => panic!("expected a single AttributeEquals, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `Pattern` survives a full `format_statute` → `parse_statute` round-trip —
/// the primary parser now reads the printer's `attr MATCHES "regex"`.
#[test]
fn test_pattern_statute_roundtrip() {
    use legalis_core::{Effect, EffectType, Statute};

    let statute = Statute::new(
        "pattern-rt",
        "Pattern Round-trip",
        Effect::new(EffectType::Grant, "Valid"),
    )
    .with_precondition(Condition::Pattern {
        attribute: "tax_code".to_string(),
        pattern: "[A-Z]{2}[0-9]+".to_string(),
        negated: false,
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [
            Condition::Pattern {
                attribute,
                pattern,
                negated,
            },
        ] => {
            assert_eq!(attribute, "tax_code");
            assert_eq!(pattern, "[A-Z]{2}[0-9]+");
            assert!(!negated);
        }
        other => panic!("expected a single Pattern, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `Duration` survives a full `format_statute` → `parse_statute` round-trip —
/// the primary parser now reads the printer's `DURATION op N unit` (the most
/// common condition kind across the jurisdiction statute adapters).
#[test]
fn test_duration_statute_roundtrip() {
    use legalis_core::{ComparisonOp, DurationUnit, Effect, EffectType, Statute};

    let statute = Statute::new(
        "duration-rt",
        "Duration Round-trip",
        Effect::new(EffectType::Obligation, "Probation period applies"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 2,
        unit: DurationUnit::Months,
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [
            Condition::Duration {
                operator,
                value,
                unit,
            },
        ] => {
            assert_eq!(*operator, ComparisonOp::GreaterOrEqual);
            assert_eq!(*value, 2);
            assert_eq!(*unit, DurationUnit::Months);
        }
        other => panic!("expected a single Duration, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `ResidencyDuration` survives a full round-trip — the primary parser now reads
/// the printer's `RESIDENCY op N months` form.
#[test]
fn test_residency_duration_statute_roundtrip() {
    use legalis_core::{ComparisonOp, Effect, EffectType, Statute};

    let statute = Statute::new(
        "residency-rt",
        "Residency Round-trip",
        Effect::new(EffectType::Grant, "Naturalisation eligibility"),
    )
    .with_precondition(Condition::ResidencyDuration {
        operator: ComparisonOp::GreaterOrEqual,
        months: 60,
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [Condition::ResidencyDuration { operator, months }] => {
            assert_eq!(*operator, ComparisonOp::GreaterOrEqual);
            assert_eq!(*months, 60);
        }
        other => panic!("expected a single ResidencyDuration, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `SetMembership` survives a full round-trip — the primary parser now reads
/// the printer's `attr IN {a, b, c}` form.
#[test]
fn test_set_membership_statute_roundtrip() {
    use legalis_core::{Effect, EffectType, Statute};

    let statute = Statute::new(
        "set-rt",
        "Set Membership Round-trip",
        Effect::new(EffectType::Grant, "Eligible"),
    )
    .with_precondition(Condition::SetMembership {
        attribute: "tier".to_string(),
        values: vec!["gold".to_string(), "silver".to_string()],
        negated: false,
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [
            Condition::SetMembership {
                attribute,
                values,
                negated,
            },
        ] => {
            assert_eq!(attribute, "tier");
            assert_eq!(values, &vec!["gold".to_string(), "silver".to_string()]);
            assert!(!negated);
        }
        other => panic!("expected a single SetMembership, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `Geographic` survives a full round-trip — the printer prints the fieldless
/// `RegionType` variant via `{:?}` (`REGION State "CA"`), which parses back.
#[test]
fn test_geographic_statute_roundtrip() {
    use legalis_core::{Effect, EffectType, RegionType, Statute};

    let statute = Statute::new(
        "geo-rt",
        "Geographic Round-trip",
        Effect::new(EffectType::Grant, "Regional benefit"),
    )
    .with_precondition(Condition::Geographic {
        region_type: RegionType::State,
        region_id: "CA".to_string(),
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [
            Condition::Geographic {
                region_type,
                region_id,
            },
        ] => {
            assert_eq!(*region_type, RegionType::State);
            assert_eq!(region_id, "CA");
        }
        other => panic!("expected a single Geographic, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `EntityRelationship` survives a full round-trip — printer's
/// `RELATIONSHIP ParentChild "child-1"`.
#[test]
fn test_entity_relationship_statute_roundtrip() {
    use legalis_core::{Effect, EffectType, RelationshipType, Statute};

    let statute = Statute::new(
        "rel-rt",
        "Relationship Round-trip",
        Effect::new(EffectType::Grant, "Dependent benefit"),
    )
    .with_precondition(Condition::EntityRelationship {
        relationship_type: RelationshipType::ParentChild,
        target_entity_id: Some("child-1".to_string()),
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [
            Condition::EntityRelationship {
                relationship_type,
                target_entity_id,
            },
        ] => {
            assert_eq!(*relationship_type, RelationshipType::ParentChild);
            assert_eq!(target_entity_id.as_deref(), Some("child-1"));
        }
        other => panic!("expected a single EntityRelationship, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `Percentage` survives a full round-trip — the printer's
/// `PERCENTAGE op N% (context)` (the `%` glyph is dropped by the tokenizer).
#[test]
fn test_percentage_statute_roundtrip() {
    use legalis_core::{ComparisonOp, Effect, EffectType, Statute};

    let statute = Statute::new(
        "pct-rt",
        "Percentage Round-trip",
        Effect::new(EffectType::Prohibition, "Market dominance threshold"),
    )
    .with_precondition(Condition::Percentage {
        operator: ComparisonOp::GreaterOrEqual,
        value: 50,
        context: "market_share".to_string(),
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [
            Condition::Percentage {
                operator,
                value,
                context,
            },
        ] => {
            assert_eq!(*operator, ComparisonOp::GreaterOrEqual);
            assert_eq!(*value, 50);
            assert_eq!(context, "market_share");
        }
        other => panic!("expected a single Percentage, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `Custom` survives a full round-trip — the printer's `CUSTOM "description"`.
#[test]
fn test_custom_statute_roundtrip() {
    use legalis_core::{Effect, EffectType, Statute};

    let statute = Statute::new(
        "custom-rt",
        "Custom Round-trip",
        Effect::new(EffectType::Obligation, "Special rule"),
    )
    .with_precondition(Condition::Custom {
        description: "court discretion applies".to_string(),
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [Condition::Custom { description }] => {
            assert_eq!(description, "court discretion applies");
        }
        other => panic!("expected a single Custom, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `Calculation` survives a full round-trip — the lexer now tokenizes the `f64`
/// value (`CALC "income * 0.2" > 0.05`) instead of splitting it on the decimal.
#[test]
fn test_calculation_statute_roundtrip() {
    use legalis_core::{ComparisonOp, Effect, EffectType, Statute};

    let statute = Statute::new(
        "calc-rt",
        "Calculation Round-trip",
        Effect::new(EffectType::Obligation, "Tax owed"),
    )
    .with_precondition(Condition::Calculation {
        formula: "income * 0.2".to_string(),
        operator: ComparisonOp::GreaterThan,
        value: 0.05,
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [
            Condition::Calculation {
                formula,
                operator,
                value,
            },
        ] => {
            assert_eq!(formula, "income * 0.2");
            assert_eq!(*operator, ComparisonOp::GreaterThan);
            assert!((*value - 0.05).abs() < 1e-9, "value was {value}");
        }
        other => panic!("expected a single Calculation, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// Float literals with leading fractional zeros survive lexing exactly
/// (regression guard for the integer-only-lexer bug, e.g. `0.05` ≠ `0.5`).
#[test]
fn test_calculation_leading_zero_fraction() {
    use legalis_core::{ComparisonOp, Effect, EffectType, Statute};

    let statute = Statute::new(
        "calc-zero",
        "Calc Leading Zero",
        Effect::new(EffectType::Obligation, "Levy"),
    )
    .with_precondition(Condition::Calculation {
        formula: "rate".to_string(),
        operator: ComparisonOp::GreaterOrEqual,
        value: 0.05,
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [Condition::Calculation { value, .. }] => {
            assert!(
                (*value - 0.05).abs() < 1e-9,
                "0.05 must not become 0.5; got {value}"
            );
        }
        other => panic!("expected a single Calculation, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// `DateRange` survives a full round-trip — the printer's `DATE start TO end`
/// (dates reconstruct numerically, so leading zeros in the text are irrelevant).
#[test]
fn test_date_range_statute_roundtrip() {
    use chrono::NaiveDate;
    use legalis_core::{Effect, EffectType, Statute};

    let statute = Statute::new(
        "date-rt",
        "Date Range Round-trip",
        Effect::new(EffectType::Grant, "Validity window"),
    )
    .with_precondition(Condition::DateRange {
        start: NaiveDate::from_ymd_opt(2020, 1, 1),
        end: NaiveDate::from_ymd_opt(2025, 12, 31),
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [Condition::DateRange { start, end }] => {
            assert_eq!(*start, NaiveDate::from_ymd_opt(2020, 1, 1));
            assert_eq!(*end, NaiveDate::from_ymd_opt(2025, 12, 31));
        }
        other => panic!("expected a single DateRange, got {other:?}\nDSL:\n{dsl}"),
    }
}

/// Negated `SetMembership` round-trips — the printer's `attr NOT IN {..}` form.
#[test]
fn test_negated_set_membership_roundtrip() {
    use legalis_core::{Effect, EffectType, Statute};

    let statute = Statute::new(
        "neg-set-rt",
        "Negated Set Membership",
        Effect::new(EffectType::Prohibition, "Excluded categories"),
    )
    .with_precondition(Condition::SetMembership {
        attribute: "status".to_string(),
        values: vec!["banned".to_string(), "revoked".to_string()],
        negated: true,
    });

    let dsl = crate::format_statute(&statute);
    let parser = crate::LegalDslParser::new();
    let parsed = parser
        .parse_statute(&dsl)
        .unwrap_or_else(|e| panic!("rendered DSL must parse: {e}\n---\n{dsl}"));

    match parsed.preconditions() {
        [
            Condition::SetMembership {
                attribute,
                values,
                negated,
            },
        ] => {
            assert_eq!(attribute, "status");
            assert_eq!(values, &vec!["banned".to_string(), "revoked".to_string()]);
            assert!(negated, "negation must survive the round-trip");
        }
        other => panic!("expected a single negated SetMembership, got {other:?}\nDSL:\n{dsl}"),
    }
}
