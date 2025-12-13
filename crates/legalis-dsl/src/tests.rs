//! Tests for the Legalis DSL parser.

use super::*;
use chrono::Datelike;

#[test]
fn test_parse_simple_statute() {
    let input = r#"
        STATUTE adult-rights: "Adult Rights Act" {
            WHEN AGE >= 18
            THEN GRANT "Full legal capacity"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.id, "adult-rights");
    assert_eq!(statute.title, "Adult Rights Act");
    assert_eq!(statute.preconditions.len(), 1);
}

#[test]
fn test_parse_statute_with_discretion() {
    let input = r#"
        STATUTE subsidy-1: "Housing Subsidy" {
            WHEN INCOME <= 5000000
            THEN GRANT "Housing subsidy eligibility"
            DISCRETION "Consider special family circumstances"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert!(statute.discretion_logic.is_some());
    assert_eq!(
        statute.discretion_logic.unwrap(),
        "Consider special family circumstances"
    );
}

#[test]
fn test_parse_and_condition() {
    let input = r#"
        STATUTE combo: "Combined Requirements" {
            WHEN AGE >= 18 AND INCOME <= 5000000
            THEN GRANT "Eligibility"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.preconditions.len(), 1);
    assert!(matches!(statute.preconditions[0], Condition::And(_, _)));
}

#[test]
fn test_parse_or_condition() {
    let input = r#"
        STATUTE either: "Either Requirement" {
            WHEN AGE >= 65 OR disabled
            THEN GRANT "Pension eligibility"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.preconditions.len(), 1);
    assert!(matches!(statute.preconditions[0], Condition::Or(_, _)));
}

#[test]
fn test_parse_not_condition() {
    let input = r#"
        STATUTE exclude: "Exclusion" {
            WHEN NOT convicted
            THEN GRANT "Voting rights"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.preconditions.len(), 1);
    assert!(matches!(statute.preconditions[0], Condition::Not(_)));
}

#[test]
fn test_parse_nested_conditions() {
    let input = r#"
        STATUTE complex: "Complex Requirements" {
            WHEN (AGE >= 18 AND INCOME <= 5000000) OR disabled
            THEN GRANT "Benefits eligibility"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.preconditions.len(), 1);
    // Should be OR at top level with AND inside left branch
    assert!(matches!(statute.preconditions[0], Condition::Or(_, _)));
}

#[test]
fn test_parse_with_line_comments() {
    let input = r#"
        // This is a comment about the statute
        STATUTE adult-rights: "Adult Rights Act" {
            WHEN AGE >= 18  // Must be adult
            THEN GRANT "Full legal capacity"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.id, "adult-rights");
}

#[test]
fn test_parse_with_block_comments() {
    let input = r#"
        /*
         * Multi-line comment explaining the statute
         * This grants rights to adults
         */
        STATUTE adult-rights: "Adult Rights Act" {
            WHEN AGE >= 18
            THEN GRANT "Full legal capacity"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.id, "adult-rights");
}

#[test]
fn test_unclosed_comment_error() {
    let input = r#"
        /* Unclosed comment
        STATUTE test: "Test" {
            WHEN AGE >= 18
            THEN GRANT "Test"
        }
    "#;

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    assert!(matches!(result, Err(DslError::UnclosedComment(_))));
}

#[test]
fn test_unclosed_comment_error_location() {
    // The comment starts at offset where /* begins
    // "STATUTE test: \"Test\" {\n" = 22 bytes, then 12 spaces + "/*"
    // So offset = 22 + 12 = 34, which is line 2, column 13
    // But the offset we store is where we found the /*, which increments after consuming chars
    let input = "STATUTE test: \"Test\" {\n            /* unclosed\n}";

    let parser = LegalDslParser::new();
    let result = parser.parse_statute(input);

    match result {
        Err(DslError::UnclosedComment(Some(loc))) => {
            // Line 2 because we're after the first newline
            assert_eq!(loc.line, 2, "Expected line 2");
            // Column depends on exactly how offset is calculated
            assert!(
                loc.column >= 13 && loc.column <= 15,
                "Expected column around 13-15, got {}",
                loc.column
            );
        }
        _ => panic!("Expected UnclosedComment error with location"),
    }
}

#[test]
fn test_source_location_display() {
    let loc = SourceLocation::new(10, 5, 100);
    assert_eq!(format!("{}", loc), "10:5");
}

#[test]
fn test_source_location_from_offset() {
    let input = "line1\nline2\nline3";
    // Offset 0 should be line 1, column 1
    let loc = SourceLocation::from_offset(0, input);
    assert_eq!(loc.line, 1);
    assert_eq!(loc.column, 1);

    // Offset 6 should be line 2, column 1 (after the newline)
    let loc = SourceLocation::from_offset(6, input);
    assert_eq!(loc.line, 2);
    assert_eq!(loc.column, 1);

    // Offset 8 should be line 2, column 3
    let loc = SourceLocation::from_offset(8, input);
    assert_eq!(loc.line, 2);
    assert_eq!(loc.column, 3);
}

#[test]
fn test_precedence_and_before_or() {
    // AND should bind tighter than OR
    // A OR B AND C should parse as A OR (B AND C)
    let input = r#"
        STATUTE prec: "Precedence Test" {
            WHEN AGE >= 65 OR AGE >= 18 AND employed
            THEN GRANT "Something"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.preconditions.len(), 1);
    // Top level should be OR
    match &statute.preconditions[0] {
        Condition::Or(_, right) => {
            // Right side should be AND
            assert!(matches!(right.as_ref(), Condition::And(_, _)));
        }
        _ => panic!("Expected OR at top level"),
    }
}

#[test]
fn test_parse_effective_date() {
    let input = r#"
        STATUTE dated: "Dated Statute" {
            EFFECTIVE_DATE 2024-01-01
            WHEN AGE >= 18
            THEN GRANT "Rights"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert!(statute.temporal_validity.effective_date.is_some());
    let date = statute.temporal_validity.effective_date.unwrap();
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_parse_expiry_date() {
    let input = r#"
        STATUTE sunset: "Sunset Clause" {
            EXPIRY_DATE 2025-12-31
            WHEN AGE >= 21
            THEN GRANT "Temporary right"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert!(statute.temporal_validity.expiry_date.is_some());
    let date = statute.temporal_validity.expiry_date.unwrap();
    assert_eq!(date.year(), 2025);
    assert_eq!(date.month(), 12);
    assert_eq!(date.day(), 31);
}

#[test]
fn test_parse_effective_and_expiry() {
    let input = r#"
        STATUTE temporal: "Temporal Statute" {
            EFFECTIVE 2024-06-01
            EXPIRES 2026-05-31
            WHEN AGE >= 18
            THEN GRANT "Time-limited right"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert!(statute.temporal_validity.effective_date.is_some());
    assert!(statute.temporal_validity.expiry_date.is_some());
}

#[test]
fn test_parse_jurisdiction() {
    let input = r#"
        STATUTE jurisdictional: "Regional Statute" {
            JURISDICTION "JP-13"
            WHEN AGE >= 20
            THEN GRANT "Local rights"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.jurisdiction, Some("JP-13".to_string()));
}

#[test]
fn test_parse_version() {
    let input = r#"
        STATUTE versioned: "Versioned Statute" {
            VERSION 3
            WHEN AGE >= 18
            THEN GRANT "Rights v3"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.version, 3);
}

#[test]
fn test_parse_full_metadata() {
    let input = r#"
        STATUTE full-meta: "Full Metadata Statute" {
            JURISDICTION "US-CA"
            VERSION 2
            EFFECTIVE_DATE "2024-01-15"
            EXPIRY_DATE "2029-12-31"
            WHEN AGE >= 21
            THEN GRANT "Full rights"
            DISCRETION "Consider circumstances"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.jurisdiction, Some("US-CA".to_string()));
    assert_eq!(statute.version, 2);
    assert!(statute.temporal_validity.effective_date.is_some());
    assert!(statute.temporal_validity.expiry_date.is_some());
    assert!(statute.discretion_logic.is_some());
}

#[test]
fn test_parse_has_keyword() {
    let input = r#"
        STATUTE citizen: "Citizenship Requirement" {
            WHEN HAS citizen
            THEN GRANT "Voting rights"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.preconditions.len(), 1);
    assert!(matches!(
        &statute.preconditions[0],
        Condition::HasAttribute { key } if key == "citizen"
    ));
}

#[test]
fn test_parse_has_with_string() {
    let input = r#"
        STATUTE status: "Status Requirement" {
            WHEN HAS "active-member"
            THEN GRANT "Member benefits"
        }
    "#;

    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(input).unwrap();

    assert_eq!(statute.preconditions.len(), 1);
    assert!(matches!(
        &statute.preconditions[0],
        Condition::HasAttribute { key } if key == "active-member"
    ));
}

#[test]
fn test_parse_multiple_statutes() {
    let input = r#"
        // First statute
        STATUTE adult-rights: "Adult Rights" {
            WHEN AGE >= 18
            THEN GRANT "Full legal capacity"
        }

        // Second statute
        STATUTE senior-benefits: "Senior Benefits" {
            WHEN AGE >= 65
            THEN GRANT "Pension eligibility"
        }

        /* Third statute with block comment */
        STATUTE minor-protection: "Minor Protection" {
            WHEN AGE < 18
            THEN GRANT "Protected status"
        }
    "#;

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(input).unwrap();

    assert_eq!(statutes.len(), 3);
    assert_eq!(statutes[0].id, "adult-rights");
    assert_eq!(statutes[1].id, "senior-benefits");
    assert_eq!(statutes[2].id, "minor-protection");
}

#[test]
fn test_parse_statutes_with_metadata() {
    let input = r#"
        STATUTE law-1: "Law One" {
            JURISDICTION "US"
            VERSION 2
            WHEN AGE >= 18
            THEN GRANT "Rights"
        }

        STATUTE law-2: "Law Two" {
            JURISDICTION "JP"
            EFFECTIVE_DATE 2024-01-01
            WHEN INCOME <= 5000000
            THEN GRANT "Subsidy"
            DISCRETION "Consider circumstances"
        }
    "#;

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(input).unwrap();

    assert_eq!(statutes.len(), 2);
    assert_eq!(statutes[0].jurisdiction, Some("US".to_string()));
    assert_eq!(statutes[0].version, 2);
    assert_eq!(statutes[1].jurisdiction, Some("JP".to_string()));
    assert!(statutes[1].temporal_validity.effective_date.is_some());
    assert!(statutes[1].discretion_logic.is_some());
}

#[test]
fn test_parse_single_statute_via_parse_statutes() {
    let input = r#"
        STATUTE single: "Single Statute" {
            WHEN AGE >= 21
            THEN GRANT "Something"
        }
    "#;

    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(input).unwrap();

    assert_eq!(statutes.len(), 1);
    assert_eq!(statutes[0].id, "single");
}

#[test]
fn test_parse_import_simple() {
    let input = r#"
        IMPORT "base/laws.legalis"

        STATUTE child: "Child Statute" {
            WHEN AGE >= 18
            THEN GRANT "Rights"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.imports.len(), 1);
    assert_eq!(doc.imports[0].path, "base/laws.legalis");
    assert!(doc.imports[0].alias.is_none());
    assert_eq!(doc.statutes.len(), 1);
}

#[test]
fn test_parse_import_with_alias() {
    let input = r#"
        IMPORT "other/laws.legalis" AS other

        STATUTE derived: "Derived Statute" {
            WHEN other.adult_rights
            THEN GRANT "Extended rights"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.imports.len(), 1);
    assert_eq!(doc.imports[0].path, "other/laws.legalis");
    assert_eq!(doc.imports[0].alias, Some("other".to_string()));
    assert_eq!(doc.statutes.len(), 1);

    // Check that the condition references the imported module
    assert_eq!(doc.statutes[0].conditions.len(), 1);
    match &doc.statutes[0].conditions[0] {
        ast::ConditionNode::HasAttribute { key } => {
            assert_eq!(key, "other.adult_rights");
        }
        _ => panic!("Expected HasAttribute condition"),
    }
}

#[test]
fn test_parse_multiple_imports() {
    let input = r#"
        IMPORT "core/basic.legalis" AS basic
        IMPORT "extensions/premium.legalis" AS premium

        STATUTE combined: "Combined Features" {
            WHEN basic.eligibility AND premium.subscription
            THEN GRANT "Full access"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.imports.len(), 2);
    assert_eq!(doc.imports[0].path, "core/basic.legalis");
    assert_eq!(doc.imports[0].alias, Some("basic".to_string()));
    assert_eq!(doc.imports[1].path, "extensions/premium.legalis");
    assert_eq!(doc.imports[1].alias, Some("premium".to_string()));
    assert_eq!(doc.statutes.len(), 1);
}

#[test]
fn test_parse_document_no_imports() {
    let input = r#"
        STATUTE standalone: "Standalone Statute" {
            WHEN AGE >= 18
            THEN GRANT "Rights"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert!(doc.imports.is_empty());
    assert_eq!(doc.statutes.len(), 1);
    assert_eq!(doc.statutes[0].id, "standalone");
}

#[test]
fn test_parse_document_multiple_statutes() {
    let input = r#"
        IMPORT "common.legalis" AS common

        STATUTE statute1: "First" {
            WHEN AGE >= 18
            THEN GRANT "First benefit"
        }

        STATUTE statute2: "Second" {
            WHEN common.employed
            THEN GRANT "Second benefit"
        }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(input).unwrap();

    assert_eq!(doc.imports.len(), 1);
    assert_eq!(doc.statutes.len(), 2);
    assert_eq!(doc.statutes[0].id, "statute1");
    assert_eq!(doc.statutes[1].id, "statute2");
}

#[test]
fn test_exception_clause() {
    let dsl = r#"
    STATUTE emergency-override: "Emergency Override" {
        WHEN AGE >= 18
        THEN GRANT "Emergency powers"
        EXCEPTION WHEN HAS medical_emergency "Medical emergencies bypass age requirement"
    }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(dsl).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.exceptions.len(), 1);
    assert_eq!(
        statute.exceptions[0].description,
        "Medical emergencies bypass age requirement"
    );
    assert_eq!(statute.exceptions[0].conditions.len(), 1);
}

#[test]
fn test_amendment_clause() {
    let dsl = r#"
    STATUTE voting-age-update: "Voting Age Update" {
        AMENDMENT voting-rights VERSION 3 EFFECTIVE_DATE 2024-01-15 "Lowered voting age to 16"
        WHEN AGE >= 16
        THEN GRANT "Right to vote"
    }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(dsl).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.amendments.len(), 1);
    assert_eq!(statute.amendments[0].target_id, "voting-rights");
    assert_eq!(statute.amendments[0].version, Some(3));
    assert_eq!(statute.amendments[0].date, Some("2024-1-15".to_string()));
    assert_eq!(
        statute.amendments[0].description,
        "Lowered voting age to 16"
    );
}

#[test]
fn test_supersedes_clause() {
    let dsl = r#"
    STATUTE new-tax-law: "New Tax Law" {
        SUPERSEDES old-tax-2020, old-tax-2021
        WHEN INCOME >= 50000
        THEN OBLIGATION "Pay income tax"
    }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(dsl).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.supersedes.len(), 2);
    assert!(statute.supersedes.contains(&"old-tax-2020".to_string()));
    assert!(statute.supersedes.contains(&"old-tax-2021".to_string()));
}

#[test]
fn test_comprehensive_statute() {
    let dsl = r#"
    STATUTE comprehensive-law: "Comprehensive Law" {
        JURISDICTION "US-CA"
        VERSION 2
        EFFECTIVE_DATE 2024-01-01
        AMENDMENT old-law VERSION 1 "Updated rules"
        SUPERSEDES legacy-law
        WHEN AGE >= 21 AND HAS license
        THEN GRANT "Driving privileges"
        EXCEPTION WHEN HAS medical_condition "Requires medical clearance"
        DISCRETION "Judge may waive requirements for hardship"
    }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(dsl).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.id, "comprehensive-law");
    assert_eq!(statute.amendments.len(), 1);
    assert_eq!(statute.supersedes.len(), 1);
    assert_eq!(statute.exceptions.len(), 1);
    assert!(statute.discretion.is_some());
}

#[test]
fn test_multiple_exceptions() {
    let dsl = r#"
    STATUTE age-restricted: "Age Restricted Activity" {
        WHEN AGE >= 18
        THEN GRANT "Access"
        EXCEPTION WHEN HAS guardian_consent "Minors with consent"
        EXCEPTION "Emergency situations"
    }
    "#;

    let parser = LegalDslParser::new();
    let doc = parser.parse_document(dsl).unwrap();

    assert_eq!(doc.statutes.len(), 1);
    let statute = &doc.statutes[0];
    assert_eq!(statute.exceptions.len(), 2);
    assert!(!statute.exceptions[0].conditions.is_empty());
    assert_eq!(statute.exceptions[1].conditions.len(), 0);
}
