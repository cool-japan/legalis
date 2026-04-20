//! Snapshot-based tests using the insta framework.

use super::*;

mod snapshot_tests {
    use super::*;
    use insta::{assert_json_snapshot, assert_yaml_snapshot};

    #[test]
    fn test_snapshot_simple_statute() {
        let dsl = r#"
        STATUTE voting-rights: "Voting Rights Act" {
            WHEN age >= 18 AND HAS citizenship
            THEN GRANT "right to vote"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_json_snapshot!(doc);
    }

    #[test]
    fn test_snapshot_complex_statute() {
        let dsl = r#"
        STATUTE tax-credit: "Tax Credit Eligibility" {
            JURISDICTION "US"
            VERSION 2
            REQUIRES base-income, residency
            WHEN income BETWEEN 20000 AND 100000
            WHEN age >= 25 AND age <= 65
            WHEN HAS dependents
            THEN GRANT "tax credit"
            THEN OBLIGATION "file tax return"
            EXCEPTION WHEN income > 90000 "High income exception"
            DEFAULT category "standard"
            SUPERSEDES old-tax-credit
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_yaml_snapshot!(doc);
    }

    #[test]
    fn test_snapshot_multiple_statutes() {
        let dsl = r#"
        STATUTE statute1: "First Statute" {
            WHEN age >= 18
            THEN GRANT "benefit1"
        }

        STATUTE statute2: "Second Statute" {
            REQUIRES statute1
            WHEN income < 50000
            THEN GRANT "benefit2"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_json_snapshot!(doc);
    }

    #[test]
    fn test_snapshot_with_imports() {
        let dsl = r#"
        IMPORT "common/definitions.legalis"
        IMPORT "lib/utils.legalis" AS utils

        STATUTE test: "Test Statute" {
            WHEN age >= 21
            THEN GRANT "adult rights"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_yaml_snapshot!(doc);
    }

    #[test]
    fn test_snapshot_complex_conditions() {
        let dsl = r#"
        STATUTE complex: "Complex Conditions" {
            WHEN (age >= 18 AND age <= 65) OR HAS disability
            WHEN income IN [20000, 30000, 40000]
            WHEN name LIKE "Smith%"
            WHEN NOT (status = "invalid")
            THEN GRANT "eligibility"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_json_snapshot!(doc);
    }
}
