//! Unit tests for the formal-specification export module.

use super::*;
use crate::ast::{
    ConditionNode, ConditionValue, EffectNode, ExceptionNode, LegalDocument, StatuteNode,
};

fn statute(id: &str) -> StatuteNode {
    StatuteNode {
        id: id.to_string(),
        title: format!("{id} statute"),
        ..Default::default()
    }
}

fn doc(statutes: Vec<StatuteNode>) -> LegalDocument {
    LegalDocument {
        namespace: None,
        imports: Vec::new(),
        exports: Vec::new(),
        statutes,
    }
}

fn cmp(field: &str, op: &str, value: ConditionValue) -> ConditionNode {
    ConditionNode::Comparison {
        field: field.to_string(),
        operator: op.to_string(),
        value,
    }
}

fn grant(label: &str) -> EffectNode {
    EffectNode {
        effect_type: "grant".to_string(),
        description: label.to_string(),
        parameters: Vec::new(),
    }
}

/// A statute: AGE >= 18 AND HAS citizen ⇒ GRANT "Right to vote".
fn voting_doc() -> LegalDocument {
    let mut s = statute("voting-rights");
    s.title = "Voting Rights".to_string();
    s.conditions = vec![ConditionNode::And(
        Box::new(cmp("age", ">=", ConditionValue::Number(18))),
        Box::new(ConditionNode::HasAttribute {
            key: "citizen".to_string(),
        }),
    )];
    s.effects = vec![grant("Right to vote")];
    doc(vec![s])
}

#[test]
fn test_date_to_int() {
    assert_eq!(date_to_int("2024-01-15"), Some(20_240_115));
    assert_eq!(date_to_int("2024-12-31"), Some(20_241_231));
    assert_eq!(date_to_int("not-a-date"), None);
    assert_eq!(date_to_int("2024-13-01"), None);
    assert_eq!(date_to_int("2024-01-32"), None);
    assert_eq!(date_to_int("2024-01-15-99"), None);
}

#[test]
fn test_sanitize_identifiers() {
    assert_eq!(sanitize("voting-rights"), "voting_rights");
    assert_eq!(sanitize("tax.income"), "tax_income");
    assert_eq!(sanitize("123abc"), "_123abc");
    assert_eq!(sanitize(""), "field");
    // Non-ASCII collapses to underscores deterministically.
    assert_eq!(sanitize("年齢"), "__");
}

#[test]
fn test_snake_and_camel_ident() {
    assert_eq!(snake_ident("Voting-Rights"), "voting_rights");
    assert_eq!(camel_ident("voting-rights"), "VotingRights");
    assert_eq!(camel_ident("tax_income_2024"), "TaxIncome2024");
    assert_eq!(camel_ident("9lives"), "S9lives");
}

#[test]
fn test_cmpop_parse() {
    assert_eq!(CmpOp::parse("="), CmpOp::Eq);
    assert_eq!(CmpOp::parse("=="), CmpOp::Eq);
    assert_eq!(CmpOp::parse("!="), CmpOp::Ne);
    assert_eq!(CmpOp::parse(">="), CmpOp::Ge);
    assert_eq!(CmpOp::parse("<"), CmpOp::Lt);
    assert!(CmpOp::Ge.is_ordering());
    assert!(!CmpOp::Eq.is_ordering());
}

#[test]
fn test_like_shape() {
    assert!(matches!(like_shape("%abc%"), LikeShape::Contains(s) if s == "abc"));
    assert!(matches!(like_shape("abc%"), LikeShape::Prefix(s) if s == "abc"));
    assert!(matches!(like_shape("%abc"), LikeShape::Suffix(s) if s == "abc"));
    assert!(matches!(like_shape("abc"), LikeShape::Exact(s) if s == "abc"));
}

#[test]
fn test_field_registry_merge() {
    let mut s = statute("s");
    s.conditions = vec![
        cmp("age", ">=", ConditionValue::Number(18)),
        cmp("name", "=", ConditionValue::String("a".to_string())),
        ConditionNode::HasAttribute {
            key: "citizen".to_string(),
        },
    ];
    let spec = DocumentSpec::from_document(&doc(vec![s]));
    assert_eq!(spec.fields.get("age"), ScalarType::Int);
    assert_eq!(spec.fields.get("name"), ScalarType::Str);
    assert_eq!(spec.fields.get("has_citizen"), ScalarType::Bool);
    assert!(spec.fields.has_bool());
    assert_eq!(spec.fields.len(), 3);
}

#[test]
fn test_document_spec_lowering() {
    let mut s = statute("dep");
    s.conditions = vec![
        cmp("age", ">=", ConditionValue::Number(18)),
        ConditionNode::HasAttribute {
            key: "citizen".to_string(),
        },
    ];
    s.requires = vec!["base".to_string()];
    s.exceptions = vec![
        ExceptionNode {
            conditions: vec![ConditionNode::HasAttribute {
                key: "waiver".to_string(),
            }],
            description: "waived".to_string(),
        },
        // Exception with no conditions is filtered out (would be vacuously true).
        ExceptionNode {
            conditions: Vec::new(),
            description: "blanket".to_string(),
        },
    ];
    let spec = DocumentSpec::from_document(&doc(vec![s]));
    let lowered = &spec.statutes[0];
    assert!(matches!(lowered.precond, Formula::And(_, _)));
    assert_eq!(lowered.exceptions.len(), 1);
    assert_eq!(lowered.requires, vec!["base".to_string()]);
}

#[test]
fn test_uses_like_and_matches() {
    let mut s = statute("s");
    s.conditions = vec![ConditionNode::Like {
        field: "income".to_string(),
        pattern: "salary%".to_string(),
    }];
    let spec = DocumentSpec::from_document(&doc(vec![s]));
    assert!(spec.uses_like());
    assert!(!spec.uses_matches());

    let mut s2 = statute("s2");
    s2.conditions = vec![ConditionNode::Matches {
        field: "email".to_string(),
        regex_pattern: ".*@.*".to_string(),
    }];
    let spec2 = DocumentSpec::from_document(&doc(vec![s2]));
    assert!(spec2.uses_matches());
    assert!(!spec2.uses_like());
}

#[test]
fn test_conflicting_pairs() {
    let mut granter = statute("granter");
    granter.effects = vec![grant("citizenship")];
    let mut revoker = statute("revoker");
    revoker.effects = vec![EffectNode {
        effect_type: "revoke".to_string(),
        description: "citizenship".to_string(),
        parameters: Vec::new(),
    }];
    let spec = DocumentSpec::from_document(&doc(vec![granter, revoker]));
    let pairs = spec.conflicting_pairs();
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0], (0, 1, "citizenship".to_string()));
}

#[test]
fn test_ordered_indices_respects_requires() {
    // Source order is [dependent, dependency]; ordering must place the
    // dependency first.
    let mut dependent = statute("dependent");
    dependent.requires = vec!["dependency".to_string()];
    let dependency = statute("dependency");
    let spec = DocumentSpec::from_document(&doc(vec![dependent, dependency]));
    let order = spec.ordered_indices();
    let pos_dep = order.iter().position(|&i| i == 1).unwrap();
    let pos_dependent = order.iter().position(|&i| i == 0).unwrap();
    assert!(pos_dep < pos_dependent);
}

#[test]
fn test_coq_export_contains() {
    let out = CoqExporter::new().export(&voting_doc()).unwrap();
    assert!(out.contains("Record Entity"));
    assert!(out.contains("Definition applies_voting_rights"));
    assert!(out.contains("(age e)"));
    assert!(out.contains(">= 18"));
    assert!(out.contains("has_citizen"));
    assert!(out.contains("EffGrant \"Right to vote\"%string"));
    assert!(out.contains("Conjecture voting_rights_satisfiable"));
    assert_eq!(CoqExporter::new().target(), "Coq");
    assert_eq!(CoqExporter::new().file_extension(), "v");
}

#[test]
fn test_lean_export_contains() {
    let out = Lean4Exporter::new().export(&voting_doc()).unwrap();
    assert!(out.contains("namespace Legalis"));
    assert!(out.contains("structure Entity where"));
    assert!(out.contains("def applies_voting_rights (e : Entity) : Prop"));
    assert!(out.contains("e.age ≥ 18"));
    assert!(out.contains("e.has_citizen = true"));
    assert!(out.contains("LegalEffect.grant \"Right to vote\""));
    assert!(out.contains("theorem voting_rights_satisfiable"));
    assert!(out.contains("end Legalis"));
}

#[test]
fn test_tla_export_contains() {
    let out = TlaExporter::new().export(&voting_doc()).unwrap();
    assert!(out.contains("MODULE LegalisSpec"));
    assert!(out.contains("Entity == ["));
    assert!(out.contains("AppliesVotingRights(e) =="));
    assert!(out.contains("e.age >= 18"));
    assert!(out.contains("e.has_citizen = TRUE"));
    assert!(out.contains("THEOREM VotingRightsSat"));
    assert!(out.trim_end().ends_with('='));
}

#[test]
fn test_alloy_export_contains() {
    let out = AlloyExporter::new().export(&voting_doc()).unwrap();
    assert!(out.contains("module legalis"));
    assert!(out.contains("enum Bool { True, False }"));
    assert!(out.contains("sig Entity {"));
    assert!(out.contains("pred appliesVotingRights[e : Entity]"));
    assert!(out.contains("e.age >= 18"));
    assert!(out.contains("e.has_citizen = True"));
    assert!(out.contains("run appliesVotingRights for 4 but 8 Int"));
}

#[test]
fn test_smtlib_export_contains() {
    let out = SmtLibExporter::new().export(&voting_doc()).unwrap();
    assert!(out.contains("(set-logic ALL)"));
    assert!(out.contains("(declare-datatypes ((Entity 0))"));
    assert!(out.contains("(define-fun applies_voting_rights ((e Entity)) Bool"));
    assert!(out.contains("(>= (age e) 18)"));
    assert!(out.contains("(has_citizen e)"));
    assert!(out.contains("(check-sat)"));
    assert_eq!(SmtLibExporter::new().file_extension(), "smt2");
}

#[test]
fn test_smtlib_like_uses_string_ops() {
    let mut s = statute("consultant");
    s.conditions = vec![ConditionNode::Like {
        field: "income_source".to_string(),
        pattern: "consulting%".to_string(),
    }];
    s.effects = vec![grant("Self-employed status")];
    let out = SmtLibExporter::new().export(&doc(vec![s])).unwrap();
    assert!(out.contains("(str.prefixof \"consulting\" (income_source e))"));
}

#[test]
fn test_all_backends_idempotent() {
    let document = voting_doc();
    let coq = CoqExporter::new();
    let lean = Lean4Exporter::new();
    let tla = TlaExporter::new();
    let alloy = AlloyExporter::new();
    let smt = SmtLibExporter::new();
    assert_eq!(
        coq.export(&document).unwrap(),
        coq.export(&document).unwrap()
    );
    assert_eq!(
        lean.export(&document).unwrap(),
        lean.export(&document).unwrap()
    );
    assert_eq!(
        tla.export(&document).unwrap(),
        tla.export(&document).unwrap()
    );
    assert_eq!(
        alloy.export(&document).unwrap(),
        alloy.export(&document).unwrap()
    );
    assert_eq!(
        smt.export(&document).unwrap(),
        smt.export(&document).unwrap()
    );
}

#[test]
fn test_export_statute_convenience() {
    let document = voting_doc();
    let statute = &document.statutes[0];
    let via_doc = SmtLibExporter::new().export(&document).unwrap();
    let via_statute = SmtLibExporter::new().export_statute(statute).unwrap();
    assert_eq!(via_doc, via_statute);
}

#[test]
fn test_obligations_toggle() {
    let document = voting_doc();
    let with = CoqExporter::new().export(&document).unwrap();
    let without = CoqExporter::new()
        .without_obligations()
        .export(&document)
        .unwrap();
    assert!(with.contains("Conjecture"));
    assert!(!without.contains("Conjecture"));
}

#[test]
fn test_empty_document_is_wellformed() {
    let empty = doc(Vec::new());
    // No fields ⇒ each backend emits a placeholder record rather than failing.
    assert!(
        CoqExporter::new()
            .export(&empty)
            .unwrap()
            .contains("Entity")
    );
    assert!(
        Lean4Exporter::new()
            .export(&empty)
            .unwrap()
            .contains("placeholder")
    );
    assert!(
        SmtLibExporter::new()
            .export(&empty)
            .unwrap()
            .contains("placeholder")
    );
}
