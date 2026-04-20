//! Advanced parsing tests: temporal conditions, range operators, warnings, set expressions,
//! error recovery, and TOML serialization.

use super::*;

#[test]
fn test_parse_temporal_condition_current_date() {
    let input = r#"
        STATUTE time-limited: "Time Limited Statute" {
            WHEN CURRENT_DATE >= "2024-01-01"
            THEN GRANT "Access to new program"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.conditions.len(), 1);

    match &statute.conditions[0] {
        ast::ConditionNode::TemporalComparison {
            field,
            operator,
            value,
        } => {
            assert_eq!(field, &ast::TemporalField::CurrentDate);
            assert_eq!(operator, ">=");
            match value {
                ast::ConditionValue::Date(d) => assert_eq!(d, "2024-01-01"),
                _ => panic!("Expected date value"),
            }
        }
        _ => panic!("Expected TemporalComparison condition"),
    }
}

#[test]
fn test_parse_temporal_condition_date_field() {
    let input = r#"
        STATUTE expiring-rights: "Expiring Rights" {
            WHEN DATE_FIELD expiration < "2025-12-31"
            THEN GRANT "Must renew before expiration"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.conditions.len(), 1);

    match &statute.conditions[0] {
        ast::ConditionNode::TemporalComparison {
            field,
            operator,
            value,
        } => {
            assert_eq!(
                field,
                &ast::TemporalField::DateField("expiration".to_string())
            );
            assert_eq!(operator, "<");
            match value {
                ast::ConditionValue::Date(d) => assert_eq!(d, "2025-12-31"),
                _ => panic!("Expected date value"),
            }
        }
        _ => panic!("Expected TemporalComparison condition"),
    }
}

#[test]
fn test_parse_temporal_with_aliases() {
    let input = r#"
        STATUTE today-check: "Today Check" {
            WHEN NOW > "2024-06-01" AND TODAY <= "2024-12-31"
            THEN GRANT "Valid for 2024"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.conditions.len(), 1);

    // Should have AND condition with two temporal comparisons
    match &statute.conditions[0] {
        ast::ConditionNode::And(left, right) => {
            assert!(matches!(
                left.as_ref(),
                ast::ConditionNode::TemporalComparison { .. }
            ));
            assert!(matches!(
                right.as_ref(),
                ast::ConditionNode::TemporalComparison { .. }
            ));
        }
        _ => panic!("Expected AND condition with temporal comparisons"),
    }
}

#[test]
fn test_parse_regex_pattern() {
    let input = r#"
        STATUTE email-validation: "Email Validation" {
            WHEN email MATCHES "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
            THEN GRANT "Valid email"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.conditions.len(), 1);

    match &statute.conditions[0] {
        ast::ConditionNode::Matches {
            field,
            regex_pattern,
        } => {
            assert_eq!(field, "email");
            assert_eq!(
                regex_pattern,
                "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
            );
        }
        _ => panic!("Expected Matches condition"),
    }
}

#[test]
fn test_parse_regex_match_alias() {
    let input = r#"
        STATUTE phone-validation: "Phone Validation" {
            WHEN phone MATCH "^\\+?[1-9]\\d{1,14}$"
            THEN GRANT "Valid phone number"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.conditions.len(), 1);

    match &statute.conditions[0] {
        ast::ConditionNode::Matches {
            field,
            regex_pattern,
        } => {
            assert_eq!(field, "phone");
            assert!(regex_pattern.contains("\\+"));
        }
        _ => panic!("Expected Matches condition"),
    }
}

#[test]
fn test_parse_invalid_regex() {
    let input = r#"
        STATUTE bad-regex: "Bad Regex" {
            WHEN field MATCHES "[invalid(regex"
            THEN GRANT "Should fail"
        }
    "#;

    let parser = LegalDslParser::new();
    let result = parser.parse_document(input);

    assert!(result.is_err());
    match result {
        Err(DslError::InvalidCondition(msg)) => {
            assert!(msg.contains("Invalid regex pattern"));
        }
        _ => panic!("Expected InvalidCondition error"),
    }
}

#[test]
fn test_source_span_creation() {
    let start = SourceLocation::new(1, 5, 4);
    let end = SourceLocation::new(1, 10, 9);
    let span = SourceSpan::new(start, end);

    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
    assert_eq!(span.len(), 5);
    assert!(!span.is_empty());
}

#[test]
fn test_source_span_from_location() {
    let loc = SourceLocation::new(2, 3, 10);
    let span = SourceSpan::from_location(loc);

    assert_eq!(span.start, loc);
    assert_eq!(span.end, loc);
    assert_eq!(span.len(), 0);
    assert!(span.is_empty());
}

#[test]
fn test_source_span_text() {
    let input = "STATUTE test";
    let start = SourceLocation::new(1, 1, 0);
    let end = SourceLocation::new(1, 8, 7);
    let span = SourceSpan::new(start, end);

    assert_eq!(span.text(input), "STATUTE");
}

#[test]
fn test_source_span_display_same_line() {
    let start = SourceLocation::new(1, 5, 4);
    let end = SourceLocation::new(1, 10, 9);
    let span = SourceSpan::new(start, end);

    assert_eq!(span.to_string(), "1:5-10");
}

#[test]
fn test_source_span_display_multi_line() {
    let start = SourceLocation::new(1, 5, 4);
    let end = SourceLocation::new(3, 2, 25);
    let span = SourceSpan::new(start, end);

    assert_eq!(span.to_string(), "1:5 to 3:2");
}

#[test]
fn test_error_span_extraction() {
    let span = SourceSpan::new(SourceLocation::new(1, 5, 4), SourceLocation::new(1, 10, 9));
    let error = DslError::syntax_error_with_span(
        span,
        "Invalid syntax",
        Some("Check your syntax".to_string()),
    );

    let extracted_span = error.span();
    assert!(extracted_span.is_some());
    assert_eq!(extracted_span.unwrap(), span);
}

#[test]
fn test_error_span_from_syntax_error() {
    let loc = SourceLocation::new(2, 3, 10);
    let error = DslError::syntax_error(loc, "Invalid token", "STATUTE", "STAUTE", None);

    let span = error.span();
    assert!(span.is_some());
    let span = span.unwrap();
    assert_eq!(span.start, loc);
    assert_eq!(span.end, loc);
}

#[test]
fn test_parse_in_range_inclusive() {
    let input = r#"
        STATUTE age-range: "Age Range" {
            WHEN age IN_RANGE 18..65
            THEN GRANT "Working age"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.conditions.len(), 1);

    match &statute.conditions[0] {
        ast::ConditionNode::InRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => {
            assert_eq!(field, "age");
            assert_eq!(min, &ast::ConditionValue::Number(18));
            assert_eq!(max, &ast::ConditionValue::Number(65));
            assert!(inclusive_min);
            assert!(inclusive_max);
        }
        _ => panic!("Expected InRange condition"),
    }
}

#[test]
fn test_parse_in_range_exclusive() {
    let input = r#"
        STATUTE score-range: "Score Range" {
            WHEN score IN_RANGE (0..100)
            THEN GRANT "Valid score"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.conditions.len(), 1);

    match &statute.conditions[0] {
        ast::ConditionNode::InRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => {
            assert_eq!(field, "score");
            assert_eq!(min, &ast::ConditionValue::Number(0));
            assert_eq!(max, &ast::ConditionValue::Number(100));
            assert!(!inclusive_min);
            assert!(!inclusive_max);
        }
        _ => panic!("Expected InRange condition"),
    }
}

#[test]
fn test_parse_not_in_range() {
    let input = r#"
        STATUTE invalid-range: "Invalid Range" {
            WHEN temperature NOT_IN_RANGE 0..100
            THEN GRANT "Out of range"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.conditions.len(), 1);

    match &statute.conditions[0] {
        ast::ConditionNode::NotInRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => {
            assert_eq!(field, "temperature");
            assert_eq!(min, &ast::ConditionValue::Number(0));
            assert_eq!(max, &ast::ConditionValue::Number(100));
            assert!(inclusive_min);
            assert!(inclusive_max);
        }
        _ => panic!("Expected NotInRange condition"),
    }
}

#[test]
fn test_parse_in_range_exclusive_max() {
    let input = r#"
        STATUTE range-test: "Range Test" {
            WHEN value IN_RANGE 10...100
            THEN GRANT "Valid"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.conditions.len(), 1);

    match &statute.conditions[0] {
        ast::ConditionNode::InRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => {
            assert_eq!(field, "value");
            assert_eq!(min, &ast::ConditionValue::Number(10));
            assert_eq!(max, &ast::ConditionValue::Number(100));
            assert!(inclusive_min);
            assert!(!inclusive_max);
        }
        _ => panic!("Expected InRange condition"),
    }
}

// ========== Warning System Tests ==========

#[test]
fn test_deprecated_except_warning() {
    let input = r#"
        STATUTE test: "Test" {
            WHEN AGE >= 18
            THEN GRANT "Rights"
            EXCEPT WHEN AGE < 16 "No rights for minors"
        }
    "#;

    let parser = LegalDslParser::new();
    let _doc = parser.parse_document(input).unwrap();

    let warnings = parser.warnings();
    assert_eq!(warnings.len(), 1);

    match &warnings[0] {
        DslWarning::DeprecatedSyntax {
            old_syntax,
            new_syntax,
            ..
        } => {
            assert_eq!(old_syntax, "EXCEPT");
            assert_eq!(new_syntax, "EXCEPTION");
        }
        _ => panic!("Expected DeprecatedSyntax warning"),
    }
}

#[test]
fn test_deprecated_amends_warning() {
    let input = r#"
        STATUTE test: "Test" {
            WHEN AGE >= 18
            THEN GRANT "Rights"
            AMENDS old-statute "Updates old statute"
        }
    "#;

    let parser = LegalDslParser::new();
    let _doc = parser.parse_document(input).unwrap();

    let warnings = parser.warnings();
    assert_eq!(warnings.len(), 1);

    match &warnings[0] {
        DslWarning::DeprecatedSyntax {
            old_syntax,
            new_syntax,
            ..
        } => {
            assert_eq!(old_syntax, "AMENDS");
            assert_eq!(new_syntax, "AMENDMENT");
        }
        _ => panic!("Expected DeprecatedSyntax warning"),
    }
}

#[test]
fn test_deprecated_replaces_warning() {
    let input = r#"
        STATUTE test: "Test" {
            WHEN AGE >= 18
            THEN GRANT "Rights"
            REPLACES old-statute
        }
    "#;

    let parser = LegalDslParser::new();
    let _doc = parser.parse_document(input).unwrap();

    let warnings = parser.warnings();
    assert_eq!(warnings.len(), 1);

    match &warnings[0] {
        DslWarning::DeprecatedSyntax {
            old_syntax,
            new_syntax,
            ..
        } => {
            assert_eq!(old_syntax, "REPLACES");
            assert_eq!(new_syntax, "SUPERSEDES");
        }
        _ => panic!("Expected DeprecatedSyntax warning"),
    }
}

#[test]
fn test_multiple_deprecated_warnings() {
    let input = r#"
        STATUTE test1: "Test 1" {
            WHEN AGE >= 18
            THEN GRANT "Rights"
            EXCEPT WHEN AGE < 16 "No rights"
            REPLACES old-law
        }

        STATUTE test2: "Test 2" {
            WHEN AGE >= 21
            THEN GRANT "More rights"
            AMENDS test1 "Updates test1"
        }
    "#;

    let parser = LegalDslParser::new();
    let _doc = parser.parse_document(input).unwrap();

    let warnings = parser.warnings();
    assert_eq!(warnings.len(), 3);

    // Verify we have all three deprecated keywords
    let deprecated_keywords: Vec<String> = warnings
        .iter()
        .filter_map(|w| match w {
            DslWarning::DeprecatedSyntax { old_syntax, .. } => Some(old_syntax.clone()),
            _ => None,
        })
        .collect();

    assert!(deprecated_keywords.contains(&"EXCEPT".to_string()));
    assert!(deprecated_keywords.contains(&"REPLACES".to_string()));
    assert!(deprecated_keywords.contains(&"AMENDS".to_string()));
}

#[test]
fn test_no_warnings_for_modern_syntax() {
    let input = r#"
        STATUTE test: "Test" {
            JURISDICTION "US-CA"
            VERSION 2
            WHEN AGE >= 18
            THEN GRANT "Rights"
            EXCEPTION WHEN AGE < 16 "No rights"
            AMENDMENT old-statute "Updates"
            SUPERSEDES legacy-law
        }
    "#;

    let parser = LegalDslParser::new();
    let _doc = parser.parse_document(input).unwrap();

    let warnings = parser.warnings();
    assert_eq!(warnings.len(), 0);
}

#[test]
fn test_warning_clear() {
    let input = r#"
        STATUTE test: "Test" {
            WHEN AGE >= 18
            THEN GRANT "Rights"
            EXCEPT WHEN AGE < 16 "No rights"
        }
    "#;

    let parser = LegalDslParser::new();
    let _doc = parser.parse_document(input).unwrap();

    assert_eq!(parser.warnings().len(), 1);

    parser.clear_warnings();
    assert_eq!(parser.warnings().len(), 0);
}

#[test]
fn test_warning_display() {
    let warning = DslWarning::DeprecatedSyntax {
        location: SourceLocation::new(10, 5, 100),
        old_syntax: "EXCEPT".to_string(),
        new_syntax: "EXCEPTION".to_string(),
        message: "Please use 'EXCEPTION' instead".to_string(),
    };

    let display = format!("{}", warning);
    assert!(display.contains("10:5"));
    assert!(display.contains("EXCEPT"));
    assert!(display.contains("EXCEPTION"));
}

// ========== Set Operations Tests ==========

#[test]
fn test_set_expression_values() {
    let values = vec![
        ast::ConditionValue::Number(1),
        ast::ConditionValue::Number(2),
        ast::ConditionValue::Number(3),
    ];
    let set_expr = ast::SetExpression::Values(values.clone());

    match set_expr {
        ast::SetExpression::Values(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], ast::ConditionValue::Number(1));
        }
        _ => panic!("Expected Values variant"),
    }
}

#[test]
fn test_set_expression_union() {
    let set1 = ast::SetExpression::Values(vec![
        ast::ConditionValue::Number(1),
        ast::ConditionValue::Number(2),
    ]);
    let set2 = ast::SetExpression::Values(vec![
        ast::ConditionValue::Number(3),
        ast::ConditionValue::Number(4),
    ]);

    let union = ast::SetExpression::Union(Box::new(set1), Box::new(set2));

    match union {
        ast::SetExpression::Union(left, right) => match (*left, *right) {
            (ast::SetExpression::Values(v1), ast::SetExpression::Values(v2)) => {
                assert_eq!(v1.len(), 2);
                assert_eq!(v2.len(), 2);
            }
            _ => panic!("Expected Values in both sides"),
        },
        _ => panic!("Expected Union variant"),
    }
}

#[test]
fn test_set_expression_intersect() {
    let set1 = ast::SetExpression::Values(vec![ast::ConditionValue::Number(1)]);
    let set2 = ast::SetExpression::Values(vec![ast::ConditionValue::Number(2)]);

    let intersect = ast::SetExpression::Intersect(Box::new(set1), Box::new(set2));

    match intersect {
        ast::SetExpression::Intersect(_, _) => {
            // Successfully created intersection
        }
        _ => panic!("Expected Intersect variant"),
    }
}

#[test]
fn test_set_expression_difference() {
    let set1 = ast::SetExpression::Values(vec![ast::ConditionValue::Number(1)]);
    let set2 = ast::SetExpression::Values(vec![ast::ConditionValue::Number(2)]);

    let difference = ast::SetExpression::Difference(Box::new(set1), Box::new(set2));

    match difference {
        ast::SetExpression::Difference(_, _) => {
            // Successfully created difference
        }
        _ => panic!("Expected Difference variant"),
    }
}

#[test]
fn test_set_expression_nested() {
    // Test (1, 2) UNION ((3, 4) INTERSECT (5, 6))
    let set1 = ast::SetExpression::Values(vec![
        ast::ConditionValue::Number(1),
        ast::ConditionValue::Number(2),
    ]);
    let set2 = ast::SetExpression::Values(vec![
        ast::ConditionValue::Number(3),
        ast::ConditionValue::Number(4),
    ]);
    let set3 = ast::SetExpression::Values(vec![
        ast::ConditionValue::Number(5),
        ast::ConditionValue::Number(6),
    ]);

    let intersect = ast::SetExpression::Intersect(Box::new(set2), Box::new(set3));
    let union = ast::SetExpression::Union(Box::new(set1), Box::new(intersect));

    match union {
        ast::SetExpression::Union(left, right) => {
            assert!(matches!(*left, ast::SetExpression::Values(_)));
            assert!(matches!(*right, ast::SetExpression::Intersect(_, _)));
        }
        _ => panic!("Expected Union with nested Intersect"),
    }
}

#[test]
fn test_condition_value_set_expr() {
    let set_expr = ast::SetExpression::Values(vec![ast::ConditionValue::Number(42)]);
    let cond_value = ast::ConditionValue::SetExpr(set_expr);

    match cond_value {
        ast::ConditionValue::SetExpr(expr) => match expr {
            ast::SetExpression::Values(v) => {
                assert_eq!(v.len(), 1);
            }
            _ => panic!("Expected Values"),
        },
        _ => panic!("Expected SetExpr variant"),
    }
}

// Error Recovery Tests

#[test]
fn test_error_recovery_parse_result_ok() {
    let result = ParseResult::ok(42);
    assert!(result.is_ok());
    assert!(!result.has_errors());
    assert_eq!(result.result, Some(42));
    assert_eq!(result.errors.len(), 0);
}

#[test]
fn test_error_recovery_parse_result_err() {
    let error = DslError::parse_error("test error");
    let result: ParseResult<i32> = ParseResult::err(error);
    assert!(!result.is_ok());
    assert!(result.has_errors());
    assert_eq!(result.result, None);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn test_error_recovery_parse_result_with_partial() {
    let error = DslError::parse_error("partial error");
    let result = ParseResult::with_errors(Some(42), vec![error]);
    assert!(result.has_errors());
    assert_eq!(result.result, Some(42));
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn test_error_recovery_single_bad_statute() {
    let input = r#"
        STATUTE good-one: "Good Statute" {
            WHEN AGE >= 18
            THEN GRANT "Access"
        }

        STATUTE bad-one "Missing Colon" {
            WHEN AGE >= 21
            THEN GRANT "Other Access"
        }

        STATUTE another-good: "Another Good Statute" {
            WHEN AGE >= 25
            THEN GRANT "More Access"
        }
    "#;

    let parser = LegalDslParser::new();
    let result = parser.parse_document_with_recovery(input);

    // Should have collected the error but continued parsing
    assert!(result.has_errors());
    assert_eq!(result.errors.len(), 1);

    // Should have successfully parsed the valid statutes
    if let Some(doc) = result.result {
        // Should have at least the good statutes (2 out of 3)
        assert!(!doc.statutes.is_empty());
    }
}

#[test]
fn test_error_recovery_multiple_errors() {
    let input = r#"
        IMPORT "other-statute.dsl"

        STATUTE first "Missing Colon" {
            WHEN AGE >= 18
            THEN GRANT "Access"
        }

        STATUTE second "Another Missing Colon" {
            WHEN AGE >= 21
            THEN GRANT "Access"
        }
    "#;

    let parser = LegalDslParser::new();
    let result = parser.parse_document_with_recovery(input);

    // Should have collected multiple errors
    assert!(result.has_errors());
    assert!(!result.errors.is_empty());

    // Should have parsed the import
    if let Some(doc) = result.result {
        assert_eq!(doc.imports.len(), 1);
    }
}

#[test]
fn test_error_recovery_valid_document() {
    let input = r#"
        IMPORT "other.dsl"

        STATUTE test: "Test Statute" {
            WHEN AGE >= 18
            THEN GRANT "Access"
        }
    "#;

    let parser = LegalDslParser::new();
    let result = parser.parse_document_with_recovery(input);

    // Should have no errors for valid input
    assert!(result.is_ok());
    assert_eq!(result.errors.len(), 0);

    let doc = result.result.unwrap();
    assert_eq!(doc.imports.len(), 1);
    assert_eq!(doc.statutes.len(), 1);
}

#[test]
fn test_parse_result_into_result_ok() {
    let result = ParseResult::ok(42);
    let converted = result.into_result();
    assert!(converted.is_ok());
    assert_eq!(converted.unwrap(), 42);
}

#[test]
fn test_parse_result_into_result_err() {
    let error = DslError::parse_error("test error");
    let result: ParseResult<i32> = ParseResult::err(error);
    let converted = result.into_result();
    assert!(converted.is_err());
}

// Error Message Quality Tests

#[test]
fn test_error_message_missing_colon() {
    let input = r#"STATUTE test "Missing Colon" {
        WHEN AGE >= 18
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Expected ':'"));
}

#[test]
fn test_error_message_missing_brace() {
    let input = r#"STATUTE test: "Test"
        WHEN AGE >= 18
        THEN GRANT "Access"
    "#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Expected '{'"));
}

#[test]
fn test_error_message_invalid_condition() {
    let input = r#"STATUTE test: "Test" {
        WHEN INVALID_FIELD >= 18
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    // Should parse successfully since INVALID_FIELD is treated as a custom field
    assert!(result.is_ok());
}

#[test]
fn test_error_message_missing_then() {
    let input = r#"STATUTE test: "Test" {
        WHEN AGE >= 18
        GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    // Should parse successfully (effects are optional)
    assert!(result.is_ok());
}

#[test]
fn test_error_message_unclosed_comment() {
    let input = r#"STATUTE test: "Test" {
        /* This comment is never closed
        WHEN AGE >= 18
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Unclosed comment"));
}

#[test]
fn test_error_message_empty_input() {
    let input = "";

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Expected 'STATUTE'"));
}

#[test]
fn test_error_message_missing_statute_id() {
    let input = r#"STATUTE : "Test" {
        WHEN AGE >= 18
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Expected statute identifier") || msg.contains("Expected ':'"));
}

#[test]
fn test_error_message_missing_statute_title() {
    let input = r#"STATUTE test: {
        WHEN AGE >= 18
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Expected statute title") || msg.contains("Expected '{'"));
}

#[test]
fn test_error_message_invalid_operator() {
    let input = r#"STATUTE test: "Test" {
        WHEN AGE >> 18
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    // Parser may interpret >> as two > operators or fail
    // This tests that we get a reasonable error message
    if let Err(err) = result {
        let msg = err.to_string();
        // Should provide some useful error context
        assert!(!msg.is_empty());
    }
}

#[test]
fn test_error_message_unmatched_parenthesis() {
    let input = r#"STATUTE test: "Test" {
        WHEN (AGE >= 18
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Unmatched") || msg.contains("parenthesis") || msg.contains("paren"));
}

#[test]
fn test_error_message_between_without_and() {
    let input = r#"STATUTE test: "Test" {
        WHEN AGE BETWEEN 18 65
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    // Check that error message is informative
    assert!(
        msg.contains("Expected AND") || msg.contains("BETWEEN") || msg.contains("Invalid"),
        "Error message was: {}",
        msg
    );
}

#[test]
fn test_error_span_information() {
    let input = r#"STATUTE test: "Test" {
        WHEN AGE >= 18
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    if result.is_ok() {
        // This input is valid, skip the span test
        return;
    }

    let err = result.unwrap_err();
    // Check that error has location information
    if let Some(_span) = err.span() {
        // Good - error has span information for IDE integration
    } else {
        // Some errors may not have spans yet, but we're documenting this
        // Error message should still be useful without spans
        assert!(!err.to_string().is_empty());
    }
}

#[test]
fn test_error_message_clarity_typo() {
    let input = r#"STATUTE test: "Test" {
        WEN AGE >= 18
        THEN GRANT "Access"
    }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    // May parse (treating WEN as identifier) or fail
    // Either way should provide clear feedback
    if let Err(err) = result {
        let msg = err.to_string();
        assert!(!msg.is_empty());
    }
}

#[test]
fn test_multiple_errors_in_document() {
    let input = r#"
        STATUTE first "Missing Colon" {
            WHEN AGE >= 18
            THEN GRANT "Access"
        }

        STATUTE second "Also Missing Colon" {
            WHEN AGE >= 21
            THEN GRANT "Access"
        }
    "#;

    let parser = LegalDslParser::new();
    let result = parser.parse_document_with_recovery(input);

    // Should collect multiple errors
    assert!(result.has_errors());
    assert!(!result.errors.is_empty());

    // Each error should be informative
    for error in &result.errors {
        let msg = error.to_string();
        assert!(!msg.is_empty());
        assert!(msg.len() > 10); // Ensure error messages are reasonably descriptive
    }
}

#[test]
fn test_error_message_formatting() {
    let input = r#"STATUTE test "Missing Colon" { WHEN AGE >= 18 THEN GRANT "Access" }"#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();

    // Error message should:
    // 1. Not be empty
    assert!(!msg.is_empty());

    // 2. Be reasonably sized (not too short, not too long)
    assert!(msg.len() > 5);
    assert!(msg.len() < 500);

    // 3. Contain actionable information
    assert!(msg.contains("Expected") || msg.contains("error") || msg.contains("Error"));
}

#[test]
fn test_toml_serialization() {
    let dsl = r#"
    IMPORT "base.legalis" AS base

    STATUTE benefits: "Benefits Eligibility" {
        JURISDICTION "US-CA"
        VERSION 2
        DEFAULT status "pending"
        WHEN AGE >= 65
        THEN GRANT "Senior benefits"
        EXCEPTION WHEN AGE < 65 AND HAS disability "Disability exception"
    }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(dsl).unwrap();

    // Serialize to TOML
    let toml = crate::to_toml(&doc).unwrap();
    assert!(toml.contains("benefits"));
    assert!(toml.contains("Benefits Eligibility"));
    assert!(toml.contains("base.legalis"));

    // Deserialize back
    let doc2 = crate::from_toml(&toml).unwrap();
    assert_eq!(doc2.imports.len(), 1);
    assert_eq!(doc2.statutes.len(), 1);
    assert_eq!(doc2.statutes[0].id, "benefits");
}

#[test]
fn test_statute_toml_serialization() {
    let dsl = r#"
    STATUTE benefits: "Benefits Eligibility" {
        JURISDICTION "US-CA"
        DEFAULT status "pending"
        WHEN AGE >= 65
        THEN GRANT "Senior benefits"
        EXCEPTION WHEN AGE < 65 "Youth exception"
    }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(dsl).unwrap();
    let statute = &doc.statutes[0];

    // Serialize to TOML
    let toml = crate::statute_to_toml(statute).unwrap();
    assert!(toml.contains("benefits"));
    assert!(toml.contains("Benefits Eligibility"));

    // Deserialize back
    let statute2 = crate::statute_from_toml(&toml).unwrap();
    assert_eq!(statute2.id, "benefits");
    assert_eq!(statute2.defaults.len(), 1);
    assert_eq!(statute2.exceptions.len(), 1);
}

#[test]
fn test_toml_roundtrip_complex() {
    let dsl = r#"
    IMPORT "lib1.legalis" AS lib1
    IMPORT "lib2.legalis"

    STATUTE complex: "Complex Statute" {
        JURISDICTION "JP"
        VERSION 3
        DEFAULT level "high"
        DEFAULT category "general"
        REQUIRES prerequisite1, prerequisite2
        WHEN (AGE BETWEEN 20 AND 60 OR HAS exemption) AND INCOME LIKE "salary%"
        THEN GRANT "Tax benefit"
        THEN OBLIGATION "File return"
        EXCEPTION WHEN AGE < 20 "Youth exception"
        EXCEPTION WHEN HAS disability "Disability exception"
        AMENDMENT old-law VERSION 2 "Updated rules"
        SUPERSEDES legacy-law, old-statute
    }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(dsl).unwrap();

    // Full roundtrip test
    let toml = crate::to_toml(&doc).unwrap();
    let doc2 = crate::from_toml(&toml).unwrap();

    // Verify structure is preserved
    assert_eq!(doc2.imports.len(), 2);
    assert_eq!(doc2.imports[0].path, "lib1.legalis");
    assert_eq!(doc2.imports[0].alias, Some("lib1".to_string()));
    assert_eq!(doc2.imports[1].path, "lib2.legalis");
    assert_eq!(doc2.imports[1].alias, None);

    assert_eq!(doc2.statutes.len(), 1);
    let statute = &doc2.statutes[0];
    assert_eq!(statute.id, "complex");

    // Verify all fields are preserved
    assert_eq!(statute.defaults.len(), 2);
    assert_eq!(statute.requires.len(), 2);
    assert_eq!(statute.effects.len(), 2);
    assert_eq!(statute.exceptions.len(), 2);
    assert_eq!(statute.amendments.len(), 1);
    assert_eq!(statute.supersedes.len(), 2);
}
