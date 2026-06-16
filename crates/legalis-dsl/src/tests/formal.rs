//! Integration tests for the formal-specification export backends.
//!
//! These parse real DSL source through the public parser and then drive every
//! exporter (Coq, Lean 4, TLA+, Alloy, SMT-LIB), checking that the generated
//! artefacts capture preconditions, `REQUIRES` dependencies, exception
//! carve-outs and effect-consistency obligations.

use super::*;

/// A small corpus exercising conjunctions, `HAS`, `BETWEEN`, exceptions and a
/// grant/revoke conflict on the same right.
const CORPUS: &str = r#"
STATUTE base-eligibility: "Base Eligibility" {
    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "Base benefit"
}

STATUTE senior-grant: "Senior Grant" {
    WHEN AGE BETWEEN 65 AND 120
    THEN GRANT "Senior benefit"
    EXCEPTION WHEN HAS institutionalized "Institutional residents excluded"
}

STATUTE senior-revoke: "Senior Revoke" {
    WHEN AGE >= 65 AND HAS fraud_flag
    THEN REVOKE "Senior benefit"
}
"#;

/// A document where a dependent statute `REQUIRES` another, declared in reverse
/// source order so dependency ordering is observable.
const REQUIRES_DOC: &str = r#"
STATUTE dependent: "Dependent" {
    REQUIRES base
    WHEN HAS resident
    THEN GRANT "Dependent benefit"
}

STATUTE base: "Base" {
    WHEN HAS citizen
    THEN GRANT "Base benefit"
}
"#;

fn parse(src: &str) -> LegalDocument {
    LegalDslParser::new()
        .parse_document(src)
        .expect("corpus must parse")
}

#[test]
fn test_export_corpus_all_backends() {
    let doc = parse(CORPUS);

    let coq = CoqExporter::new().export(&doc).expect("coq export");
    assert!(coq.contains("Definition applies_base_eligibility"));
    assert!(coq.contains("Definition applies_senior_grant"));

    let lean = Lean4Exporter::new().export(&doc).expect("lean export");
    assert!(lean.contains("def applies_senior_revoke"));

    let tla = TlaExporter::new().export(&doc).expect("tla export");
    assert!(tla.contains("AppliesBaseEligibility(e) =="));

    let alloy = AlloyExporter::new().export(&doc).expect("alloy export");
    assert!(alloy.contains("pred appliesSeniorGrant[e : Entity]"));

    let smt = SmtLibExporter::new().export(&doc).expect("smt export");
    assert!(smt.contains("(define-fun applies_senior_revoke ((e Entity)) Bool"));
}

#[test]
fn test_coq_requires_reference_and_ordering() {
    let doc = parse(REQUIRES_DOC);
    let coq = CoqExporter::new().export(&doc).expect("coq export");

    // The dependency must be referenced inside the dependent predicate.
    assert!(coq.contains("(applies_base e)"));

    // And it must be defined before the statute that requires it, even though
    // it appears later in the source.
    let base = coq
        .find("Definition applies_base ")
        .expect("base def present");
    let dependent = coq
        .find("Definition applies_dependent ")
        .expect("dependent def present");
    assert!(base < dependent, "required statute must be defined first");
}

#[test]
fn test_smt_between_and_exception() {
    let doc = parse(CORPUS);
    let smt = SmtLibExporter::new().export(&doc).expect("smt export");

    // BETWEEN 65 AND 120 lowers to an inclusive integer range.
    assert!(smt.contains("(<= 65 (age e))"));
    assert!(smt.contains("(<= (age e) 120)"));
    // The exception carve-out becomes a negated conjunct.
    assert!(smt.contains("(not (has_institutionalized e))"));
}

#[test]
fn test_consistency_obligation_present() {
    let doc = parse(CORPUS);

    let coq = CoqExporter::new().export(&doc).expect("coq export");
    assert!(coq.contains("consistent_senior_grant_senior_revoke"));

    let smt = SmtLibExporter::new().export(&doc).expect("smt export");
    assert!(smt.contains("Senior benefit\" (expect unsat)"));

    let alloy = AlloyExporter::new().export(&doc).expect("alloy export");
    assert!(alloy.contains("check consistentSeniorGrantSeniorRevoke"));
}

#[test]
fn test_matches_uninterpreted_predicate() {
    let src = r#"
    STATUTE valid-email: "Valid Email" {
        WHEN email MATCHES "^[a-z]+@[a-z]+$"
        THEN GRANT "Verified contact"
    }
    "#;
    let doc = parse(src);

    let coq = CoqExporter::new().export(&doc).expect("coq export");
    assert!(coq.contains("Parameter string_matches"));
    assert!(coq.contains("string_matches (email e)"));

    let smt = SmtLibExporter::new().export(&doc).expect("smt export");
    assert!(smt.contains("(declare-fun str_matches (String String) Bool)"));
    assert!(smt.contains("(str_matches (email e)"));
}

#[test]
fn test_like_lowers_to_string_ops_in_smt() {
    let src = r#"
    STATUTE consultant: "Consultant" {
        WHEN income_source LIKE "consulting%"
        THEN GRANT "Self-employed status"
    }
    "#;
    let doc = parse(src);
    let smt = SmtLibExporter::new().export(&doc).expect("smt export");
    assert!(smt.contains("(str.prefixof \"consulting\" (income_source e))"));
}

#[test]
fn test_all_backends_idempotent_on_parsed_corpus() {
    let doc = parse(CORPUS);
    let coq = CoqExporter::new();
    let lean = Lean4Exporter::new();
    let tla = TlaExporter::new();
    let alloy = AlloyExporter::new();
    let smt = SmtLibExporter::new();
    assert_eq!(coq.export(&doc).unwrap(), coq.export(&doc).unwrap());
    assert_eq!(lean.export(&doc).unwrap(), lean.export(&doc).unwrap());
    assert_eq!(tla.export(&doc).unwrap(), tla.export(&doc).unwrap());
    assert_eq!(alloy.export(&doc).unwrap(), alloy.export(&doc).unwrap());
    assert_eq!(smt.export(&doc).unwrap(), smt.export(&doc).unwrap());
}

#[test]
fn test_targets_and_extensions() {
    assert_eq!(CoqExporter::new().target(), "Coq");
    assert_eq!(CoqExporter::new().file_extension(), "v");
    assert_eq!(Lean4Exporter::new().target(), "Lean4");
    assert_eq!(Lean4Exporter::new().file_extension(), "lean");
    assert_eq!(TlaExporter::new().target(), "TLA+");
    assert_eq!(TlaExporter::new().file_extension(), "tla");
    assert_eq!(AlloyExporter::new().target(), "Alloy");
    assert_eq!(AlloyExporter::new().file_extension(), "als");
    assert_eq!(SmtLibExporter::new().target(), "SMT-LIB");
    assert_eq!(SmtLibExporter::new().file_extension(), "smt2");
}

#[test]
fn test_custom_configuration() {
    let doc = parse(CORPUS);
    let lean = Lean4Exporter::new()
        .with_namespace("MyLaws")
        .export(&doc)
        .expect("lean export");
    assert!(lean.contains("namespace MyLaws"));
    assert!(lean.contains("end MyLaws"));

    let tla = TlaExporter::new()
        .with_module_name("Rules")
        .export(&doc)
        .expect("tla export");
    assert!(tla.contains("MODULE Rules"));

    let alloy = AlloyExporter::new()
        .with_scope(7)
        .export(&doc)
        .expect("alloy export");
    assert!(alloy.contains("for 7 but 8 Int"));
}
