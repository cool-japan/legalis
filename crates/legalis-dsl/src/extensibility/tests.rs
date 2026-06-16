//! Tests for the parser extensibility framework.

use super::*;
use crate::ast::{ConditionNode, ConditionValue, Token};
use crate::{DslError, DslResult};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Operators.
// ---------------------------------------------------------------------------

fn eval(input: &str, table: &OperatorTable) -> Option<f64> {
    let expr = table.parse(input).expect("parse expression");
    expr.eval(&HashMap::new())
}

#[test]
fn test_operator_precedence() {
    let table = OperatorTable::standard();
    assert_eq!(eval("2 + 3 * 4", &table), Some(14.0));
    assert_eq!(eval("(2 + 3) * 4", &table), Some(20.0));
    assert_eq!(eval("10 - 2 - 3", &table), Some(5.0)); // left assoc
}

#[test]
fn test_operator_right_associativity() {
    let table = OperatorTable::standard();
    // Right-assoc exponent: 2^(3^2) = 2^9 = 512, not (2^3)^2 = 64.
    assert_eq!(eval("2 ^ 3 ^ 2", &table), Some(512.0));
}

#[test]
fn test_prefix_operator() {
    let table = OperatorTable::standard();
    assert_eq!(eval("-3 + 5", &table), Some(2.0));
    assert_eq!(eval("- - 4", &table), Some(4.0));
}

#[test]
fn test_custom_operator_left_assoc() {
    let mut table = OperatorTable::new();
    table.register(OperatorDef::infix("~>", 5, Associativity::Left));
    let expr = table.parse("a ~> b ~> c").expect("parse");
    // Left assoc: (a ~> b) ~> c
    match expr {
        ExprNode::Binary { op, left, right } => {
            assert_eq!(op, "~>");
            assert!(matches!(right.as_ref(), ExprNode::Ident(s) if s == "c"));
            assert!(matches!(left.as_ref(), ExprNode::Binary { .. }));
        }
        other => panic!("expected binary, got {other:?}"),
    }
}

#[test]
fn test_word_operator() {
    let mut table = OperatorTable::new();
    table.register(OperatorDef::infix("AND", 3, Associativity::Left));
    let expr = table.parse("a AND b").expect("parse");
    assert!(matches!(expr, ExprNode::Binary { op, .. } if op == "AND"));
}

#[test]
fn test_non_associative_operator_rejected() {
    let mut table = OperatorTable::new();
    table.register(OperatorDef::infix("<>", 5, Associativity::NonAssoc));
    assert!(table.parse("a <> b").is_ok());
    assert!(table.parse("a <> b <> c").is_err());
}

#[test]
fn test_unknown_operator_errors() {
    let table = OperatorTable::standard();
    // `$` is not registered.
    assert!(table.parse("a $ b").is_err());
}

#[test]
fn test_operator_table_all() {
    let table = OperatorTable::standard();
    let all = table.all();
    assert!(all.iter().any(|d| d.symbol == "^"));
    assert!(all.iter().any(|d| d.symbol == "+"));
}

// ---------------------------------------------------------------------------
// Custom literals.
// ---------------------------------------------------------------------------

#[test]
fn test_money_literal() {
    let registry = LiteralRegistry::with_defaults();
    let (name, value) = registry.try_parse("$1,234.56").expect("money");
    assert_eq!(name, "money");
    assert_eq!(
        value,
        LiteralValue::Money {
            minor_units: 123456,
            currency: "USD".to_string(),
        }
    );
    assert_eq!(value.to_condition_value(), ConditionValue::Number(123456));
}

#[test]
fn test_money_with_currency_code() {
    let registry = LiteralRegistry::with_defaults();
    let (_, value) = registry.try_parse("100 EUR").expect("money");
    assert_eq!(
        value,
        LiteralValue::Money {
            minor_units: 10000,
            currency: "EUR".to_string(),
        }
    );
}

#[test]
fn test_percent_literal() {
    let registry = LiteralRegistry::with_defaults();
    let (name, value) = registry.try_parse("12.5%").expect("percent");
    assert_eq!(name, "percent");
    assert_eq!(value, LiteralValue::Percent(12.5));
}

#[test]
fn test_duration_literal() {
    let registry = LiteralRegistry::with_defaults();
    let (name, value) = registry.try_parse("30d").expect("duration");
    assert_eq!(name, "duration");
    assert_eq!(value, LiteralValue::Duration { seconds: 2_592_000 });
    let (_, months) = registry.try_parse("6mo").expect("duration");
    assert_eq!(
        months,
        LiteralValue::Duration {
            seconds: 15_552_000
        }
    );
}

#[test]
fn test_bare_integer_is_not_money() {
    let registry = LiteralRegistry::with_defaults();
    // No currency, no '%', no unit -> no custom literal matches.
    assert!(registry.try_parse("100").is_none());
}

#[test]
fn test_literal_registry_parse_error() {
    let registry = LiteralRegistry::with_defaults();
    assert!(registry.parse("not a literal!").is_err());
    assert_eq!(registry.names(), vec!["money", "percent", "duration"]);
}

// ---------------------------------------------------------------------------
// User-defined syntax productions.
// ---------------------------------------------------------------------------

fn within_days_handler(tokens: &[Token]) -> DslResult<ConditionNode> {
    match tokens {
        [Token::Ident(_), Token::Number(n)] => Ok(ConditionNode::Comparison {
            field: "days_elapsed".to_string(),
            operator: "<=".to_string(),
            value: ConditionValue::Number(*n as i64),
        }),
        _ => Err(DslError::parse_error("WITHIN_DAYS expects a single number")),
    }
}

#[test]
fn test_syntax_production_registration_and_parse() {
    let mut registry = SyntaxExtensionRegistry::new();
    registry.register_production("WITHIN_DAYS", "deadline window", within_days_handler);

    assert!(registry.is_keyword("within_days"));
    assert!(registry.production_for("WITHIN_DAYS").is_some());

    let cond = registry.parse_condition("WITHIN_DAYS 30").expect("parse");
    assert_eq!(
        cond,
        ConditionNode::Comparison {
            field: "days_elapsed".to_string(),
            operator: "<=".to_string(),
            value: ConditionValue::Number(30),
        }
    );
}

#[test]
fn test_syntax_unregistered_keyword_errors() {
    let registry = SyntaxExtensionRegistry::new();
    assert!(registry.parse_condition("UNKNOWN 5").is_err());
}

#[test]
fn test_keyword_aliases() {
    let mut registry = SyntaxExtensionRegistry::new();
    registry.register_keyword(KeywordSpec::new("EMISSION", "emission").with_alias("EMIT"));
    assert!(registry.is_keyword("EMISSION"));
    assert!(registry.is_keyword("emit"));
    assert!(!registry.is_keyword("nope"));
}

// ---------------------------------------------------------------------------
// Pluggable parser modules.
// ---------------------------------------------------------------------------

struct GeofencePlugin;

impl ParserPlugin for GeofencePlugin {
    fn name(&self) -> &str {
        "geofence"
    }

    fn keywords(&self) -> Vec<KeywordSpec> {
        vec![KeywordSpec::new(
            "GEOFENCE",
            "a geographic region predicate",
        )]
    }

    fn try_parse_condition(&self, tokens: &[Token]) -> Option<DslResult<ParsedFragment>> {
        match tokens.first() {
            Some(Token::Ident(s)) if s.eq_ignore_ascii_case("GEOFENCE") => match tokens.get(1) {
                Some(Token::Ident(region)) => Some(Ok(ParsedFragment::new(
                    ConditionNode::HasAttribute {
                        key: format!("in_region_{}", region.to_lowercase()),
                    },
                    2,
                ))),
                _ => Some(Err(DslError::parse_error("GEOFENCE expects a region"))),
            },
            _ => None,
        }
    }
}

#[test]
fn test_plugin_registry_dispatch() {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(GeofencePlugin));
    assert_eq!(registry.len(), 1);
    assert_eq!(registry.names(), vec!["geofence"]);
    assert!(
        registry
            .all_keywords()
            .iter()
            .any(|k| k.keyword == "GEOFENCE")
    );

    let tokens = tokenize("GEOFENCE downtown").expect("tokenize");
    let result = registry
        .try_parse_condition(&tokens)
        .expect("plugin claims tokens")
        .expect("parse ok");
    assert_eq!(
        result.node,
        ConditionNode::HasAttribute {
            key: "in_region_downtown".to_string(),
        }
    );
    assert_eq!(result.consumed, 2);
}

#[test]
fn test_plugin_declines_unrelated_tokens() {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(GeofencePlugin));
    let tokens = tokenize("AGE >= 18").expect("tokenize");
    assert!(registry.try_parse_condition(&tokens).is_none());
}

// ---------------------------------------------------------------------------
// Backward-compatibility layer.
// ---------------------------------------------------------------------------

#[test]
fn test_syntax_version_ordering_and_parse() {
    assert!(SyntaxVersion::parse("0.2.1").unwrap() < SyntaxVersion::parse("0.3.0").unwrap());
    assert_eq!(
        SyntaxVersion::parse("1.2").unwrap(),
        SyntaxVersion::new(1, 2, 0)
    );
    assert!(SyntaxVersion::parse("not.a.version").is_none());
    assert!(SyntaxVersion::parse("1.2.3.4").is_none());
}

#[test]
fn test_compat_normalizes_deprecated_keyword() {
    let layer = CompatibilityLayer::with_builtin_rules(SyntaxVersion::new(0, 3, 0));
    let (out, warnings) = layer
        .normalize("HAS active EXCEPT \"keep EXCEPT in string\"")
        .expect("normalize");
    assert!(out.contains("EXCEPTION"));
    // The keyword inside the string literal is preserved.
    assert!(out.contains("\"keep EXCEPT in string\""));
    assert_eq!(warnings.len(), 1);
    match &warnings[0] {
        crate::DslWarning::DeprecatedSyntax {
            old_syntax,
            new_syntax,
            ..
        } => {
            assert_eq!(old_syntax, "EXCEPT");
            assert_eq!(new_syntax, "EXCEPTION");
        }
        other => panic!("expected deprecation warning, got {other:?}"),
    }
}

#[test]
fn test_compat_comment_is_untouched() {
    let layer = CompatibilityLayer::with_builtin_rules(SyntaxVersion::new(0, 3, 0));
    let (out, warnings) = layer
        .normalize("// EXCEPT in a comment\nHAS x")
        .expect("normalize");
    assert!(out.contains("// EXCEPT in a comment"));
    assert!(warnings.is_empty());
}

#[test]
fn test_compat_removed_syntax_is_error() {
    let mut layer = CompatibilityLayer::new(SyntaxVersion::new(0, 3, 0));
    layer.add_rule(
        DeprecationRule::new("LEGACY", "MODERN", SyntaxVersion::new(0, 1, 0))
            .removed_in(SyntaxVersion::new(0, 2, 0)),
    );
    assert!(layer.normalize("LEGACY foo").is_err());
}

#[test]
fn test_compat_rule_not_yet_active() {
    // A rule deprecated in a future version is not applied at an earlier target.
    let mut layer = CompatibilityLayer::new(SyntaxVersion::new(0, 1, 0));
    layer.add_rule(DeprecationRule::new(
        "FUTURE",
        "NEW",
        SyntaxVersion::new(0, 5, 0),
    ));
    let (out, warnings) = layer.normalize("FUTURE thing").expect("normalize");
    assert_eq!(out, "FUTURE thing");
    assert!(warnings.is_empty());
}

// ---------------------------------------------------------------------------
// ExtensibleParser integration.
// ---------------------------------------------------------------------------

#[test]
fn test_extensible_parser_falls_back_to_core() {
    let parser = ExtensibleParser::new();
    let cond = parser.parse_condition("AGE >= 18").expect("parse");
    // Must equal the core parser's result exactly (backward compatible).
    let base = crate::LegalDslParser::new();
    let tokens: Vec<Token> = base
        .tokenize("AGE >= 18")
        .unwrap()
        .into_iter()
        .map(|s| s.token)
        .collect();
    let mut iter = tokens.iter().peekable();
    let expected = base.parse_condition_node(&mut iter).unwrap().unwrap();
    assert_eq!(cond, expected);
}

#[test]
fn test_extensible_parser_uses_production() {
    let parser =
        ExtensibleParser::new().with_production("WITHIN_DAYS", "window", within_days_handler);
    let cond = parser.parse_condition("WITHIN_DAYS 14").expect("parse");
    assert_eq!(
        cond,
        ConditionNode::Comparison {
            field: "days_elapsed".to_string(),
            operator: "<=".to_string(),
            value: ConditionValue::Number(14),
        }
    );
}

#[test]
fn test_extensible_parser_uses_plugin() {
    let parser = ExtensibleParser::new().with_plugin(Box::new(GeofencePlugin));
    let cond = parser.parse_condition("GEOFENCE harbor").expect("parse");
    assert_eq!(
        cond,
        ConditionNode::HasAttribute {
            key: "in_region_harbor".to_string(),
        }
    );
    // And it still parses core conditions (plugin declines).
    assert!(parser.parse_condition("HAS citizen").is_ok());
}

#[test]
fn test_extensible_parser_applies_compat() {
    let parser = ExtensibleParser::new().with_compat_rule(DeprecationRule::new(
        "OLDAGE",
        "AGE",
        SyntaxVersion::new(0, 1, 0),
    ));
    let cond = parser.parse_condition("OLDAGE >= 21").expect("parse");
    assert_eq!(
        cond,
        ConditionNode::Comparison {
            field: "age".to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(21),
        }
    );
    assert_eq!(parser.warnings().len(), 1);
}

#[test]
fn test_extensible_parser_expression_and_literal() {
    let parser = ExtensibleParser::new();
    let expr = parser.parse_expression("1 + 2 * 3").expect("expr");
    assert_eq!(expr.eval(&HashMap::new()), Some(7.0));
    assert!(parser.try_literal("$5.00").is_some());
    assert!(parser.try_literal("bare").is_none());
}

#[test]
fn test_known_keywords_aggregation() {
    let parser = ExtensibleParser::new()
        .with_production("FOO", "foo", within_days_handler)
        .with_plugin(Box::new(GeofencePlugin));
    let keywords = parser.known_keywords();
    assert!(keywords.iter().any(|k| k.keyword == "FOO"));
    assert!(keywords.iter().any(|k| k.keyword == "GEOFENCE"));
}
