//! Integration tests for the Test DSL extension (v0.2.7): `@mock`, `@property`,
//! `@coverage`, and `@snapshot` directives plus `USING` in `@test`.
//!
//! Covers structural parsing, `parse -> print -> parse` round-trip stability of
//! the full [`TestSpecDocument`], the property engine (exhaustive + sampled +
//! shrinking), mock fixture resolution, coverage measurement, snapshot
//! signatures, and located error reporting.

use super::*;

/// The canonical voting statute reused across the suite.
const VOTING: &str = r#"
STATUTE voting: "Voting Rights" {
    WHEN AGE >= 18
    THEN GRANT "Right to vote"
}
"#;

#[test]
fn test_parse_mock_entities() {
    let parser = LegalDslParser::new();
    let doc = parser
        .parse_test_spec_document("@mock adult { age = 30, citizen = true }")
        .expect("mock parses");
    assert_eq!(doc.mocks.len(), 1);
    let mock = doc.mock("adult").expect("adult mock present");
    assert_eq!(mock.bindings.len(), 2);
    assert_eq!(mock.bindings[0].key, "age");
    assert_eq!(mock.bindings[0].value, TestValue::Number(30));
    assert_eq!(mock.bindings[1].value, TestValue::Boolean(true));
}

#[test]
fn test_test_uses_mock_with_given_override() {
    // The mock makes the case pass; an explicit GIVEN can override a mock value.
    let src = format!(
        "{VOTING}\n@mock adult {{ age = 40 }}\n\
         @test \"adult\" FOR voting {{ USING adult EXPECT GRANT }}\n\
         @test \"override\" FOR voting {{ USING adult GIVEN age = 10 EXPECT NOT SATISFIED }}\n"
    );
    let parser = LegalDslParser::new();
    let report = parser.run_test_spec(&src).expect("spec runs");
    assert!(report.tests.all_passed(), "{:?}", report.tests.results);
    assert_eq!(report.tests.total(), 2);
}

#[test]
fn test_property_holds_exhaustively() {
    let src = format!(
        "{VOTING}\n@property \"adults eligible\" FOR voting {{\n\
         FORALL age IN 18 TO 120\n EXPECT SATISFIED\n}}\n"
    );
    let parser = LegalDslParser::new();
    let report = parser.run_test_spec(&src).expect("spec runs");
    assert!(
        report.properties.all_passed(),
        "{:?}",
        report.properties.results
    );
    let result = &report.properties.results[0];
    assert!(result.exhaustive);
    assert_eq!(result.checked_cases, 103); // 18..=120 inclusive
}

#[test]
fn test_property_reports_shrunk_counterexample() {
    let src = format!(
        "{VOTING}\n@property \"everyone eligible\" FOR voting {{\n\
         FORALL age IN 0 TO 120\n EXPECT SATISFIED\n}}\n"
    );
    let parser = LegalDslParser::new();
    let report = parser.run_test_spec(&src).expect("spec runs");
    assert_eq!(report.properties.failed(), 1);
    let counter = report.properties.results[0]
        .counterexample
        .as_ref()
        .expect("counterexample present");
    assert_eq!(counter, &vec![("age".to_string(), TestValue::Number(0))]);
}

#[test]
fn test_property_value_list_domain() {
    let parser = LegalDslParser::new();
    // All sampled ages are adults → passes.
    let pass = format!(
        "{VOTING}\n@property \"listed\" FOR voting {{ FORALL age IN ( 20, 30, 40 ) EXPECT SATISFIED }}\n"
    );
    let report = parser.run_test_spec(&pass).expect("runs");
    assert!(report.properties.all_passed());

    // A minor in the list breaks the property.
    let fail = format!(
        "{VOTING}\n@property \"listed\" FOR voting {{ FORALL age IN ( 10, 20 ) EXPECT SATISFIED }}\n"
    );
    let report = parser.run_test_spec(&fail).expect("runs");
    assert_eq!(report.properties.failed(), 1);
}

#[test]
fn test_property_samples_large_domain() {
    let src = format!(
        "{VOTING}\n@property \"big\" FOR voting {{\n\
         FORALL age IN 18 TO 100000000\n EXPECT SATISFIED\n CASES 50\n}}\n"
    );
    let parser = LegalDslParser::new();
    let report = parser.run_test_spec(&src).expect("spec runs");
    let result = &report.properties.results[0];
    assert!(!result.exhaustive, "domain is too large to enumerate");
    assert_eq!(result.checked_cases, 50);
    assert!(result.passed);
}

#[test]
fn test_property_unknown_mock_reported() {
    let src = format!(
        "{VOTING}\n@property \"p\" FOR voting {{ FORALL age IN 18 TO 20 USING ghost EXPECT SATISFIED }}\n"
    );
    let parser = LegalDslParser::new();
    let report = parser.run_test_spec(&src).expect("spec runs");
    assert_eq!(report.properties.failed(), 1);
    assert!(
        report.properties.results[0]
            .message
            .contains("unknown mock entity 'ghost'")
    );
}

#[test]
fn test_coverage_passes_and_fails() {
    let pass = format!(
        "{VOTING}\n\
         @test \"adult\" FOR voting {{ GIVEN age = 40 EXPECT SATISFIED }}\n\
         @test \"minor\" FOR voting {{ GIVEN age = 10 EXPECT NOT SATISFIED }}\n\
         @coverage REQUIRE statutes >= 100%\n\
         @coverage REQUIRE outcomes >= 50%\n"
    );
    let parser = LegalDslParser::new();
    let report = parser.run_test_spec(&pass).expect("runs");
    assert!(
        report.coverage.all_passed(),
        "{:?}",
        report.coverage.results
    );

    // Only a satisfied case → the false branch is never covered.
    let fail = format!(
        "{VOTING}\n\
         @test \"adult\" FOR voting {{ GIVEN age = 40 EXPECT SATISFIED }}\n\
         @coverage REQUIRE outcomes >= 100%\n"
    );
    let report = parser.run_test_spec(&fail).expect("runs");
    assert_eq!(report.coverage.failed(), 1);
    assert!(
        report.coverage.results[0]
            .message
            .contains("outcomes coverage")
    );
}

#[test]
fn test_snapshot_match_and_mismatch() {
    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(VOTING).expect("statute parses");
    let signature = statute_signature(&statutes[0]);

    let matching = format!("{VOTING}\n@snapshot \"baseline\" FOR voting EXPECT \"{signature}\"\n");
    let report = parser.run_test_spec(&matching).expect("runs");
    assert!(
        report.snapshots.all_passed(),
        "{:?}",
        report.snapshots.results
    );

    let mismatch =
        format!("{VOTING}\n@snapshot \"stale\" FOR voting EXPECT \"GRANT#0000000000000000\"\n");
    let report = parser.run_test_spec(&mismatch).expect("runs");
    assert_eq!(report.snapshots.failed(), 1);
    assert!(
        report.snapshots.results[0]
            .message
            .contains("snapshot mismatch")
    );
}

#[test]
fn test_snapshot_record_mode_passes() {
    let src = format!("{VOTING}\n@snapshot \"bless\" FOR voting RECORD\n");
    let parser = LegalDslParser::new();
    let report = parser.run_test_spec(&src).expect("runs");
    assert!(report.snapshots.all_passed());
    assert!(report.snapshots.results[0].expected.is_none());
    assert!(report.snapshots.results[0].actual.starts_with("GRANT#"));
}

#[test]
fn test_full_spec_roundtrips() {
    let src = r#"
@mock adult {
    age = 30
    citizen = true
}

@test "adult votes" FOR voting {
    USING adult
    GIVEN region = "east"
    EXPECT GRANT
}

@property "adults eligible" FOR voting {
    FORALL age IN 18 TO 120
    FORALL tier IN ( "gold", "silver" )
    GIVEN citizen = true
    USING adult
    EXPECT SATISFIED
    CASES 64
}

@coverage REQUIRE statutes >= 100%
@coverage REQUIRE outcomes >= 50% FOR voting

@snapshot "baseline" FOR voting EXPECT "GRANT#0123456789abcdef"
@snapshot "blessed" FOR voting RECORD
"#;
    let parser = LegalDslParser::new();
    let doc1 = parser.parse_test_spec_document(src).expect("spec parses");
    assert_eq!(doc1.mocks.len(), 1);
    assert_eq!(doc1.tests.len(), 1);
    assert_eq!(doc1.properties.len(), 1);
    assert_eq!(doc1.coverage.len(), 2);
    assert_eq!(doc1.snapshots.len(), 2);

    let printed = format_test_spec_document(&doc1);
    let doc2 = parser
        .parse_test_spec_document(&printed)
        .unwrap_or_else(|e| panic!("printed spec must re-parse: {e}\n---\n{printed}"));
    assert_eq!(doc1, doc2, "round-trip changed the AST\n---\n{printed}");
}

#[test]
fn test_run_test_spec_end_to_end() {
    let parser = LegalDslParser::new();
    let statutes = parser.parse_statutes(VOTING).expect("statute parses");
    let signature = statute_signature(&statutes[0]);
    let src = format!(
        "{VOTING}\n@mock adult {{ age = 40 }}\n\
         @test \"adult\" FOR voting {{ USING adult EXPECT GRANT }}\n\
         @test \"minor\" FOR voting {{ GIVEN age = 10 EXPECT NOT SATISFIED }}\n\
         @property \"eligible\" FOR voting {{ FORALL age IN 18 TO 60 EXPECT SATISFIED }}\n\
         @coverage REQUIRE statutes >= 100%\n\
         @coverage REQUIRE outcomes >= 50%\n\
         @snapshot \"baseline\" FOR voting EXPECT \"{signature}\"\n"
    );
    let report = parser.run_test_spec(&src).expect("spec runs");
    assert!(report.all_passed(), "failures: {}", report.total_failures());
    assert_eq!(report.tests.total(), 2);
    assert_eq!(report.properties.total(), 1);
    assert_eq!(report.coverage.total(), 2);
    assert_eq!(report.snapshots.total(), 1);
}

#[test]
fn test_contract_document_ignores_spec_directives() {
    // parse_contract_document keeps only @test, but must still skip the other
    // directives without error.
    let src = r#"
@mock m { age = 1 }
@test "t" FOR s { EXPECT SATISFIED }
@property "p" FOR s { FORALL age IN 1 TO 2 EXPECT SATISFIED }
@coverage REQUIRE statutes >= 50%
@snapshot "sn" FOR s RECORD
"#;
    let parser = LegalDslParser::new();
    let doc = parser
        .parse_contract_document(src)
        .expect("contract doc parses");
    assert!(doc.contracts.is_empty());
    assert_eq!(doc.test_cases.len(), 1);
    assert_eq!(doc.test_cases[0].name, "t");
}

#[test]
fn test_property_requires_forall() {
    let parser = LegalDslParser::new();
    let err = parser
        .parse_test_spec_document("@property \"p\" FOR voting { EXPECT SATISFIED }")
        .expect_err("property without FORALL must error");
    assert!(err.to_string().contains("FORALL"), "{err}");
}

#[test]
fn test_coverage_unknown_metric_errors() {
    let parser = LegalDslParser::new();
    let err = parser
        .parse_test_spec_document("@coverage REQUIRE bogus >= 50%")
        .expect_err("unknown metric must error");
    assert!(err.to_string().contains("unknown coverage metric"), "{err}");
}

#[test]
fn test_spec_error_reports_line_and_column() {
    let src = "@property \"p\" FOR voting {\n    FORALL age IN 18 120\n}";
    let parser = LegalDslParser::new();
    let err = parser
        .parse_test_spec_document(src)
        .expect_err("missing TO must error");
    assert!(err.to_string().contains("expected TO"), "{err}");
    match err {
        DslError::ParseError {
            location: Some(loc),
            ..
        } => assert_eq!(loc.line, 2, "error should point at the FORALL line"),
        other => panic!("expected a located ParseError, got {other:?}"),
    }
}

#[test]
fn test_unknown_directive_still_errors() {
    let parser = LegalDslParser::new();
    let err = parser
        .parse_test_spec_document("@bogus \"x\" FOR s { }")
        .expect_err("@bogus must error");
    assert!(err.to_string().contains("expected 'test'"), "{err}");
}
