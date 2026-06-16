//! Tests for the contract / compliance / inline-test grammar extensions.
//!
//! Covers structural parsing of every clause, full `parse -> print -> parse`
//! round-trip stability, the `@test` runner wired into the core evaluator, and
//! located error reporting.

use super::*;

/// A contract exercising every clause kind, plus an inline `@test`.
const FULL: &str = r#"
CONTRACT supply-2025: "Widget Supply Agreement" {
    PARTY buyer: "Acme Corp" ROLE buyer
    PARTY seller: "Beta LLC" ROLE seller
    PARTY agent: "Gamma Agent" ROLE broker
    CLAUSE governing_law FROM governing_law: "Governed by the laws of Japan."
    CLAUSE bespoke: "Bespoke provision."
    OBLIGATION pay BY buyer TO seller: "Pay each invoice" WHEN HAS invoice DUE "2025-12-31"
    OBLIGATION deliver BY seller TO buyer: "Deliver goods" WHEN HAS purchase_order AND HAS payment
    RIGHT terminate OF seller CLAIM: "Terminate on default" WHEN HAS breach CORRELATIVE pay
    PERFORMANCE delivery {
        DESC "Deliver conforming goods"
        WHEN HAS purchase_order
        DUE "2025-06-30"
    }
    COMPLIANCE iso_9001: "Maintain quality management" STANDARD "ISO 9001" WHEN HAS factory
    PENALTY late_fee: "Late payment surcharge" AMOUNT 5 USD PER month FOR pay WHEN HAS overdue
    REPORT quarterly: "Financial statement" EVERY quarterly TO seller DUE "2025-03-31"
    REPORT adhoc: "Ad-hoc report" EVERY biweekly
    INSPECT safety: "On-site safety audit" BY regulator EVERY annually WHEN HAS site
    DEADLINE filing: "2025-04-15" "Annual filing"
    TIMELINE rollout: "Phased rollout" {
        DEADLINE phase1: "2025-03-01" "Pilot"
        DEADLINE phase2: "2025-09-01" "General availability"
    }
}

@test "adult votes" FOR voting {
    GIVEN age = 20, citizen = true
    EXPECT GRANT
}
"#;

#[test]
fn test_parse_full_contract_structure() {
    let parser = LegalDslParser::new();
    let doc = parser
        .parse_contract_document(FULL)
        .expect("FULL must parse");

    assert_eq!(doc.contracts.len(), 1);
    assert_eq!(doc.test_cases.len(), 1);

    let contract = &doc.contracts[0];
    assert_eq!(contract.id, "supply-2025");
    assert_eq!(contract.title, "Widget Supply Agreement");
    assert_eq!(contract.parties.len(), 3);
    assert_eq!(contract.clauses.len(), 2);
    assert_eq!(contract.obligations.len(), 2);
    assert_eq!(contract.rights.len(), 1);
    assert_eq!(contract.performances.len(), 1);
    assert_eq!(contract.compliance.len(), 1);
    assert_eq!(contract.penalties.len(), 1);
    assert_eq!(contract.reports.len(), 2);
    assert_eq!(contract.inspections.len(), 1);
    assert_eq!(contract.deadlines.len(), 1);
    assert_eq!(contract.timelines.len(), 1);
    assert_eq!(contract.timelines[0].deadlines.len(), 2);
}

#[test]
fn test_full_contract_roundtrips() {
    let parser = LegalDslParser::new();
    let doc1 = parser
        .parse_contract_document(FULL)
        .expect("FULL must parse");
    let printed = format_contract_document(&doc1);
    let doc2 = parser
        .parse_contract_document(&printed)
        .unwrap_or_else(|e| panic!("printed contract must re-parse: {e}\n---\n{printed}"));
    assert_eq!(doc1, doc2, "round-trip changed the AST\n---\n{printed}");
}

#[test]
fn test_party_roles_known_and_other() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let contract = &doc.contracts[0];
    assert_eq!(
        contract.party("buyer").unwrap().role,
        Some(PartyRole::Buyer)
    );
    assert_eq!(
        contract.party("seller").unwrap().role,
        Some(PartyRole::Seller)
    );
    // Unknown role keyword is preserved verbatim as `Other`.
    assert_eq!(
        contract.party("agent").unwrap().role,
        Some(PartyRole::Other("broker".to_string()))
    );
}

#[test]
fn test_obligation_relationship_fields() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let contract = &doc.contracts[0];
    let pay = contract.obligation("pay").expect("pay obligation");
    assert_eq!(pay.obligor.as_deref(), Some("buyer"));
    assert_eq!(pay.obligee.as_deref(), Some("seller"));
    assert_eq!(pay.due.as_deref(), Some("2025-12-31"));
    assert_eq!(pay.conditions.len(), 1);

    // `obligations_of` keys off the obligor (the duty-bearer).
    assert_eq!(contract.obligations_of("buyer").len(), 1);
    assert_eq!(contract.obligations_of("seller").len(), 1);
}

#[test]
fn test_right_correlativity_resolves() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let contract = &doc.contracts[0];
    let right = &contract.rights[0];
    assert_eq!(right.id, "terminate");
    assert_eq!(right.holder.as_deref(), Some("seller"));
    assert_eq!(right.kind, Some(RightKind::Claim));
    assert_eq!(right.correlative_obligation.as_deref(), Some("pay"));

    // The right's correlative obligation must resolve to a real obligation.
    let correlative = contract
        .correlative_obligation(right)
        .expect("correlative obligation resolves");
    assert_eq!(correlative.id, "pay");
}

#[test]
fn test_performance_block_fields() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let performance = &doc.contracts[0].performances[0];
    assert_eq!(performance.id, "delivery");
    assert_eq!(
        performance.description.as_deref(),
        Some("Deliver conforming goods")
    );
    assert_eq!(performance.conditions.len(), 1);
    assert_eq!(performance.due.as_deref(), Some("2025-06-30"));
}

#[test]
fn test_compliance_clause_fields() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let requirement = &doc.contracts[0].compliance[0];
    assert_eq!(requirement.id, "iso_9001");
    assert_eq!(requirement.standard.as_deref(), Some("ISO 9001"));
    assert_eq!(requirement.conditions.len(), 1);
}

#[test]
fn test_penalty_structure_fields() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let penalty = &doc.contracts[0].penalties[0];
    assert_eq!(penalty.id, "late_fee");
    assert_eq!(penalty.amount, Some(5));
    assert_eq!(penalty.currency.as_deref(), Some("USD"));
    assert_eq!(penalty.per_unit.as_deref(), Some("month"));
    assert_eq!(penalty.for_obligation.as_deref(), Some("pay"));
    assert_eq!(penalty.conditions.len(), 1);
}

#[test]
fn test_report_frequencies_known_and_custom() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let reports = &doc.contracts[0].reports;
    assert_eq!(reports[0].frequency, Some(ReportFrequency::Quarterly));
    assert_eq!(reports[0].recipient.as_deref(), Some("seller"));
    assert_eq!(reports[0].due.as_deref(), Some("2025-03-31"));
    // An unrecognised cadence keyword becomes `Custom`.
    assert_eq!(
        reports[1].frequency,
        Some(ReportFrequency::Custom("biweekly".to_string()))
    );
}

#[test]
fn test_inspection_and_audit_alias() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let inspection = &doc.contracts[0].inspections[0];
    assert_eq!(inspection.authority.as_deref(), Some("regulator"));
    assert_eq!(inspection.frequency, Some(ReportFrequency::Annually));
    assert_eq!(inspection.conditions.len(), 1);

    // `AUDIT` is an alias for `INSPECT`.
    let audit_src = r#"CONTRACT c: "C" {
        AUDIT yearly: "Yearly financial audit" BY auditor EVERY annually
    }"#;
    let doc2 = parser
        .parse_contract_document(audit_src)
        .expect("audit parse");
    assert_eq!(doc2.contracts[0].inspections.len(), 1);
    assert_eq!(doc2.contracts[0].inspections[0].id, "yearly");
}

#[test]
fn test_deadline_and_timeline() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let contract = &doc.contracts[0];
    assert_eq!(contract.deadlines[0].id, "filing");
    assert_eq!(contract.deadlines[0].date, "2025-04-15");
    assert_eq!(
        contract.deadlines[0].description.as_deref(),
        Some("Annual filing")
    );

    let timeline = &contract.timelines[0];
    assert_eq!(timeline.id, "rollout");
    assert_eq!(timeline.description.as_deref(), Some("Phased rollout"));
    assert_eq!(timeline.deadlines[1].id, "phase2");
    assert_eq!(timeline.deadlines[1].date, "2025-09-01");
}

#[test]
fn test_clause_from_template_matches_library() {
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(FULL).expect("parse");
    let clause = doc.contracts[0]
        .clauses
        .iter()
        .find(|c| c.id == "governing_law")
        .expect("governing_law clause");
    assert_eq!(clause.from_template.as_deref(), Some("governing_law"));

    let template = common_clause_template("governing_law").expect("template exists");
    let instantiated = template.instantiate("governing_law");
    assert_eq!(instantiated.from_template.as_deref(), Some("governing_law"));
    assert!(!instantiated.text.is_empty());
}

const WITH_STATUTE: &str = r#"
STATUTE voting: "Voting Rights" {
    WHEN AGE >= 18
    THEN GRANT "Right to vote"
}

@test "adult is eligible" FOR voting {
    GIVEN age = 20
    EXPECT GRANT
}

@test "minor is not" FOR voting {
    GIVEN age = 16
    EXPECT NOT SATISFIED
}

@test "wrong effect" FOR voting {
    GIVEN age = 20
    EXPECT REVOKE
}
"#;

#[test]
fn test_run_embedded_tests_against_statute() {
    let parser = LegalDslParser::new();
    let report = parser
        .run_embedded_tests(WITH_STATUTE)
        .expect("embedded tests run");
    assert_eq!(report.total(), 3);
    assert_eq!(report.passed(), 2);
    assert_eq!(report.failed(), 1);

    let failing = report
        .results
        .iter()
        .find(|r| !r.passed)
        .expect("one failing case");
    assert_eq!(failing.name, "wrong effect");
    assert!(failing.message.contains("expected effect REVOKE"));
}

#[test]
fn test_statutes_and_contracts_coexist() {
    // Statute parsing ignores CONTRACT/@test, and contract parsing ignores
    // STATUTE, so both can share a single source file.
    let mixed = format!("{WITH_STATUTE}\n{FULL}");
    let parser = LegalDslParser::new();

    let statutes = parser.parse_statutes(&mixed).expect("statutes parse");
    assert_eq!(statutes.len(), 1);
    assert_eq!(statutes[0].id, "voting");

    let doc = parser
        .parse_contract_document(&mixed)
        .expect("contracts parse");
    assert_eq!(doc.contracts.len(), 1);
    assert_eq!(doc.contracts[0].id, "supply-2025");
    // Three test cases from WITH_STATUTE plus one from FULL.
    assert_eq!(doc.test_cases.len(), 4);
}

#[test]
fn test_test_case_bindings_and_expectation() {
    let parser = LegalDslParser::new();
    let cases = parser.parse_test_cases(FULL).expect("parse tests");
    assert_eq!(cases.len(), 1);
    let case = &cases[0];
    assert_eq!(case.name, "adult votes");
    assert_eq!(case.target_statute, "voting");
    assert_eq!(case.bindings.len(), 2);
    assert_eq!(case.bindings[0].key, "age");
    assert_eq!(case.bindings[0].value, TestValue::Number(20));
    assert_eq!(case.bindings[1].value, TestValue::Boolean(true));
    assert_eq!(
        case.expectation,
        TestExpectation::Effect(ExpectedEffect::Grant)
    );
}

#[test]
fn test_test_case_roundtrips() {
    let parser = LegalDslParser::new();
    let doc1 = parser
        .parse_contract_document(WITH_STATUTE)
        .expect("parse with statute");
    // Only the @test cases survive a contract-document round-trip (statutes are
    // owned by the statute grammar), so compare the test-case vectors.
    let printed = format_contract_document(&doc1);
    let doc2 = parser
        .parse_contract_document(&printed)
        .expect("re-parse printed tests");
    assert_eq!(doc1.test_cases, doc2.test_cases);
}

#[test]
fn test_run_test_cases_directly() {
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    let statute = Statute::new(
        "subsidy",
        "Housing Subsidy",
        Effect::new(EffectType::Obligation, "Pay subsidy"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::LessOrEqual,
        value: 30000,
    });

    let cases = vec![
        TestCaseNode {
            name: "low income".to_string(),
            target_statute: "subsidy".to_string(),
            uses: Vec::new(),
            bindings: vec![TestBinding {
                key: "income".to_string(),
                value: TestValue::Number(20000),
            }],
            expectation: TestExpectation::Effect(ExpectedEffect::Obligation),
        },
        TestCaseNode {
            name: "high income".to_string(),
            target_statute: "subsidy".to_string(),
            uses: Vec::new(),
            bindings: vec![TestBinding {
                key: "income".to_string(),
                value: TestValue::Number(90000),
            }],
            expectation: TestExpectation::Unsatisfied,
        },
    ];

    let report = run_test_cases(&[statute], &cases);
    assert!(report.all_passed(), "{:?}", report.results);
}

#[test]
fn test_error_reports_line_and_column() {
    let src = "CONTRACT c1: \"t\" {\n    PARTY p1 \"no colon\"\n}";
    let parser = LegalDslParser::new();
    let err = parser
        .parse_contract_document(src)
        .expect_err("missing ':' must error");
    assert!(err.to_string().contains("expected ':'"), "{err}");

    match err {
        DslError::ParseError {
            location: Some(loc),
            ..
        } => assert_eq!(loc.line, 2, "error should point at the PARTY line"),
        other => panic!("expected a located ParseError, got {other:?}"),
    }
}

#[test]
fn test_unknown_directive_errors() {
    let parser = LegalDslParser::new();
    let err = parser
        .parse_contract_document("@foo \"x\" FOR s { EXPECT SATISFIED }")
        .expect_err("@foo must error");
    assert!(err.to_string().contains("expected 'test'"), "{err}");
}

#[test]
fn test_missing_expect_errors() {
    let parser = LegalDslParser::new();
    let err = parser
        .parse_contract_document("@test \"x\" FOR s { GIVEN age = 1 }")
        .expect_err("missing EXPECT must error");
    assert!(err.to_string().contains("EXPECT"), "{err}");
}

#[test]
fn test_validate_detects_dangling_party_reference() {
    let src = r#"CONTRACT c: "C" {
        PARTY buyer: "Acme" ROLE buyer
        OBLIGATION pay BY buyer TO ghost: "Pay"
    }"#;
    let parser = LegalDslParser::new();
    let doc = parser.parse_contract_document(src).expect("parse");
    let problems = doc.contracts[0].validate();
    assert!(
        problems
            .iter()
            .any(|p| p.contains("undefined party 'ghost'")),
        "{problems:?}"
    );
}
