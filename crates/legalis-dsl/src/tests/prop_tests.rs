//! Property-based tests using the proptest framework.

use super::*;

mod proptest_tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::strategy::ValueTree;

    fn arb_operator() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("=".to_string()),
            Just("==".to_string()),
            Just("!=".to_string()),
            Just(">".to_string()),
            Just("<".to_string()),
            Just(">=".to_string()),
            Just("<=".to_string()),
        ]
    }

    fn arb_field_name() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9_]{0,10}".prop_filter("must not be a keyword", |s| {
            let keywords = [
                "when",
                "then",
                "unless",
                "requires",
                "has",
                "in",
                "and",
                "or",
                "not",
                "between",
                "like",
                "grant",
                "obligation",
                "prohibition",
                "discretion",
                "exception",
                "amendment",
                "default",
                "supersedes",
                "statute",
                "import",
                "as",
                "jurisdiction",
                "version",
            ];
            !keywords.contains(&s.to_lowercase().as_str())
        })
    }

    fn arb_statute_id() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9-]{2,15}".prop_filter("must not be a keyword", |s| {
            let keywords = [
                "when",
                "then",
                "unless",
                "requires",
                "has",
                "in",
                "and",
                "or",
                "not",
                "between",
                "like",
                "grant",
                "obligation",
                "prohibition",
                "discretion",
                "exception",
                "amendment",
                "default",
                "supersedes",
                "statute",
                "import",
                "as",
                "jurisdiction",
                "version",
            ];
            !keywords.contains(&s.to_lowercase().as_str())
        })
    }

    proptest! {
        #[test]
        fn test_parse_simple_comparison(
            field in arb_field_name(),
            operator in arb_operator(),
            value in 0i64..1000
        ) {
            let dsl = format!(
                r#"STATUTE test: "Test" {{
                    WHEN {} {} {}
                    THEN GRANT "benefit"
                }}"#,
                field, operator, value
            );

            let parser = LegalDslParser::new();
            let result = parser.parse_document(&dsl);
            assert!(result.is_ok(), "Failed to parse: {}", dsl);

            let doc = result.unwrap();
            assert_eq!(doc.statutes.len(), 1);
            assert_eq!(doc.statutes[0].conditions.len(), 1);
        }

        #[test]
        fn test_parse_statute_id_format(id in arb_statute_id()) {
            let dsl = format!(
                r#"STATUTE {}: "Test Statute" {{
                    WHEN age >= 18
                    THEN GRANT "benefit"
                }}"#,
                id
            );

            let parser = LegalDslParser::new();
            let result = parser.parse_document(&dsl);
            assert!(result.is_ok(), "Failed to parse statute with id: {}", id);

            let doc = result.unwrap();
            assert_eq!(doc.statutes[0].id, id);
        }

        #[test]
        fn test_numeric_range_validation(min in 0i64..100, max in 0i64..100) {
            let dsl = format!(
                r#"STATUTE test: "Test" {{
                    WHEN age BETWEEN {} AND {}
                    THEN GRANT "benefit"
                }}"#,
                min, max
            );

            let parser = LegalDslParser::new();
            let result = parser.parse_document(&dsl);

            if min < max {
                assert!(result.is_ok(), "Should parse valid range: {} to {}", min, max);
            } else {
                // Invalid range should still parse but fail validation
                if result.is_ok() {
                    let doc = result.unwrap();
                    let validator = crate::validation::SemanticValidator::new();
                    let validation_result = validator.validate_document(&doc);
                    if min >= max {
                        assert!(validation_result.is_err(), "Should fail validation for invalid range: {} to {}", min, max);
                    }
                }
            }
        }

        #[test]
        fn test_printer_roundtrip(
            id in arb_statute_id(),
            title in "[A-Za-z ]{5,30}",
            value in 0i64..100
        ) {
            let dsl = format!(
                r#"STATUTE {}: "{}" {{
                    WHEN age >= {}
                    THEN GRANT "benefit"
                }}"#,
                id, title, value
            );

            let parser = LegalDslParser::new();
            let doc = parser.parse_document(&dsl).unwrap();

            // Print back to DSL
            let printed = crate::printer::format_document(&doc);

            // Parse again
            let doc2 = parser.parse_document(&printed).unwrap();

            // Verify structure is preserved
            assert_eq!(doc.statutes.len(), doc2.statutes.len());
            assert_eq!(doc.statutes[0].id, doc2.statutes[0].id);
            assert_eq!(doc.statutes[0].title, doc2.statutes[0].title);
        }

        #[test]
        fn test_json_serialization_roundtrip(
            id in arb_statute_id(),
            value in 0i64..100
        ) {
            use crate::ast::{StatuteNode, ConditionNode, ConditionValue, EffectNode};

            let statute = StatuteNode {
                id: id.clone(),
                visibility: crate::module_system::Visibility::Private,
            title: "Test Statute".to_string(),
                conditions: vec![ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">=".to_string(),
                    value: ConditionValue::Number(value),
                }],
                effects: vec![EffectNode {
                    effect_type: "GRANT".to_string(),
                    description: "benefit".to_string(),
                    parameters: vec![],
                }],
                discretion: None,
                exceptions: vec![],
                amendments: vec![],
                supersedes: vec![],
                defaults: vec![],
                requires: vec![],
                delegates: vec![],
                scope: None,
                constraints: vec![],
                priority: None,
            };

            let json = serde_json::to_string(&statute).unwrap();
            let statute2: StatuteNode = serde_json::from_str(&json).unwrap();

            assert_eq!(statute.id, statute2.id);
            assert_eq!(statute.conditions.len(), statute2.conditions.len());
        }
    }

    // Test case generators for fuzzing and property-based testing
    pub mod generators {
        use super::*;
        use crate::ast::{ConditionNode, ConditionValue, EffectNode, LegalDocument, StatuteNode};

        /// Generates arbitrary condition values for testing
        #[allow(dead_code)]
        pub fn arb_condition_value() -> impl Strategy<Value = ConditionValue> {
            prop_oneof![
                (0i64..1000000).prop_map(ConditionValue::Number),
                "[a-zA-Z0-9 ]{1,50}".prop_map(ConditionValue::String),
                prop::bool::ANY.prop_map(ConditionValue::Boolean),
                "(20[0-9]{2})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])"
                    .prop_map(ConditionValue::Date),
            ]
        }

        /// Generates arbitrary comparison conditions
        #[allow(dead_code)]
        pub fn arb_comparison_condition() -> impl Strategy<Value = ConditionNode> {
            (arb_field_name(), arb_operator(), arb_condition_value()).prop_map(
                |(field, operator, value)| ConditionNode::Comparison {
                    field,
                    operator,
                    value,
                },
            )
        }

        /// Generates arbitrary simple conditions (no recursion)
        pub fn arb_simple_condition() -> impl Strategy<Value = ConditionNode> {
            prop_oneof![
                // Numeric comparisons
                (arb_field_name(), arb_operator(), 0i64..1000000).prop_map(
                    |(field, operator, value)| ConditionNode::Comparison {
                        field,
                        operator,
                        value: ConditionValue::Number(value),
                    }
                ),
                // Has attribute
                arb_field_name().prop_map(|key| ConditionNode::HasAttribute { key }),
                // Numeric BETWEEN
                (arb_field_name(), 0i64..1000, 1000i64..2000).prop_map(|(field, min, max)| {
                    ConditionNode::Between {
                        field,
                        min: ConditionValue::Number(min),
                        max: ConditionValue::Number(max),
                    }
                }),
            ]
        }

        /// Generates arbitrary effects
        pub fn arb_effect() -> impl Strategy<Value = EffectNode> {
            (
                prop_oneof![
                    Just("grant".to_string()),
                    Just("revoke".to_string()),
                    Just("obligation".to_string()),
                    Just("prohibition".to_string()),
                ],
                "[a-zA-Z0-9 ]{5,50}".prop_map(|s: String| s),
            )
                .prop_map(|(effect_type, description)| EffectNode {
                    effect_type,
                    description,
                    parameters: vec![],
                })
        }

        /// Generates arbitrary statute nodes
        pub fn arb_statute() -> impl Strategy<Value = StatuteNode> {
            (
                arb_statute_id(),
                "[A-Za-z ]{5,30}",
                prop::collection::vec(arb_simple_condition(), 0..5),
                prop::collection::vec(arb_effect(), 1..3),
            )
                .prop_map(|(id, title, conditions, effects)| StatuteNode {
                    id,
                    title,
                    visibility: crate::module_system::Visibility::Private,
                    conditions,
                    effects,
                    discretion: None,
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec![],
                    delegates: vec![],
                    scope: None,
                    constraints: vec![],
                    priority: None,
                })
        }

        /// Generates arbitrary legal documents
        pub fn arb_document() -> impl Strategy<Value = LegalDocument> {
            prop::collection::vec(arb_statute(), 1..5).prop_map(|statutes| LegalDocument {
                namespace: None,
                exports: vec![],
                imports: vec![],
                statutes,
            })
        }

        /// Generates test cases for fuzzing
        pub fn generate_fuzz_cases(count: usize) -> Vec<LegalDocument> {
            let mut runner = proptest::test_runner::TestRunner::default();
            let strategy = arb_document();

            (0..count)
                .filter_map(|_| strategy.new_tree(&mut runner).ok())
                .map(|tree| tree.current())
                .collect()
        }
    }

    #[test]
    fn test_generated_statutes_are_valid() {
        use generators::*;

        let test_cases = generate_fuzz_cases(10);
        let parser = LegalDslParser::new();

        for doc in test_cases {
            // Convert to DSL and parse back
            let dsl = crate::printer::format_document(&doc);
            let parsed = parser.parse_document(&dsl);

            assert!(parsed.is_ok(), "Generated invalid DSL: {}", dsl);
        }
    }

    #[test]
    fn test_condition_generators() {
        use generators::*;

        let mut runner = proptest::test_runner::TestRunner::default();
        let strategy = arb_simple_condition();

        for _ in 0..20 {
            let condition = strategy.new_tree(&mut runner).unwrap().current();

            // Verify condition can be formatted
            match &condition {
                ConditionNode::Comparison {
                    field,
                    operator,
                    value,
                } => {
                    assert!(!field.is_empty());
                    assert!(!operator.is_empty());
                    let _ = value;
                }
                ConditionNode::HasAttribute { key } => {
                    assert!(!key.is_empty());
                }
                ConditionNode::Between { field, min, max } => {
                    assert!(!field.is_empty());
                    let _ = (min, max);
                }
                _ => {}
            }
        }
    }
}
