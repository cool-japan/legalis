//! Tests for the Hindley–Milner inference engine and the five advanced
//! type-system features it provides.

use super::*;
use crate::ast::{ConditionNode, ConditionValue, EffectNode, LegalDocument, StatuteNode};

// --------------------------------------------------------------------------
// Unification & occurs-check
// --------------------------------------------------------------------------

#[test]
fn test_unify_var_with_constructor() {
    let mut supply = VarSupply::new();
    let var = MonoType::Var(0);
    let subst = unify(&mut supply, &var, &MonoType::int()).expect("unify");
    assert_eq!(subst.apply_type(&var), MonoType::int());
}

#[test]
fn test_unify_occurs_check_fails() {
    let mut supply = VarSupply::new();
    let var = MonoType::Var(0);
    let recursive = MonoType::list(var.clone());
    let err = unify(&mut supply, &var, &recursive).unwrap_err();
    assert!(matches!(err, TypeInferError::OccursCheck { .. }));
}

#[test]
fn test_unify_function_types() {
    let mut supply = VarSupply::new();
    let lhs = MonoType::func(MonoType::Var(0), MonoType::Var(1));
    let rhs = MonoType::func(MonoType::int(), MonoType::boolean());
    let subst = unify(&mut supply, &lhs, &rhs).expect("unify");
    assert_eq!(subst.apply_type(&MonoType::Var(0)), MonoType::int());
    assert_eq!(subst.apply_type(&MonoType::Var(1)), MonoType::boolean());
}

#[test]
fn test_unify_mismatch_errors() {
    let mut supply = VarSupply::new();
    let err = unify(&mut supply, &MonoType::int(), &MonoType::string()).unwrap_err();
    assert!(matches!(err, TypeInferError::Mismatch { .. }));
}

// --------------------------------------------------------------------------
// Row unification (row polymorphism foundation)
// --------------------------------------------------------------------------

#[test]
fn test_row_open_absorbs_extra_field() {
    let mut supply = VarSupply::new();
    let tail = supply.fresh_row_var();
    let open = Row::open(tail).with("age", MonoType::int());
    let closed = Row::empty()
        .with("age", MonoType::int())
        .with("name", MonoType::string());
    let subst = unify_rows(&mut supply, &open, &closed).expect("row unify");
    let resolved = subst.apply_row(&open);
    assert_eq!(resolved.fields.get("name"), Some(&MonoType::string()));
    assert!(resolved.is_closed());
}

#[test]
fn test_row_closed_missing_label_fails() {
    let mut supply = VarSupply::new();
    let r1 = Row::empty().with("age", MonoType::int());
    let r2 = Row::empty()
        .with("age", MonoType::int())
        .with("name", MonoType::string());
    let err = unify_rows(&mut supply, &r1, &r2).unwrap_err();
    assert!(matches!(err, TypeInferError::MissingLabel { .. }));
}

#[test]
fn test_row_both_open_introduce_fresh_tail() {
    let mut supply = VarSupply::new();
    let t1 = supply.fresh_row_var();
    let t2 = supply.fresh_row_var();
    let row_a = Row::open(t1).with("age", MonoType::int());
    let row_b = Row::open(t2).with("name", MonoType::string());
    let subst = unify_rows(&mut supply, &row_a, &row_b).expect("row unify");
    let resolved = subst.apply_row(&row_a);
    assert_eq!(resolved.fields.get("age"), Some(&MonoType::int()));
    assert_eq!(resolved.fields.get("name"), Some(&MonoType::string()));
    assert!(resolved.is_open());
}

// --------------------------------------------------------------------------
// Let-generalization & polymorphism
// --------------------------------------------------------------------------

#[test]
fn test_let_generalization_identity() {
    let mut engine = InferenceEngine::with_prelude();
    let env = engine.prelude_env().clone();
    let term = Term::abs("x", Term::var("x"));
    let scheme = engine.infer_scheme(&env, &term).expect("infer");
    let expected = TypeScheme::new(
        vec![0],
        Vec::new(),
        QualType::plain(MonoType::func(MonoType::Var(0), MonoType::Var(0))),
    )
    .normalized();
    assert_eq!(scheme, expected);
}

#[test]
fn test_let_polymorphism_two_instantiations() {
    let mut engine = InferenceEngine::with_prelude();
    let env = engine.prelude_env().clone();
    // let id = \x. x in and (id true) (eq (id 1) 1)
    let id = Term::abs("x", Term::var("x"));
    let body = Term::apply_many(
        Term::var("and"),
        [
            Term::app(Term::var("id"), Term::Lit(Lit::Bool(true))),
            Term::apply_many(
                Term::var("eq"),
                [
                    Term::app(Term::var("id"), Term::Lit(Lit::Int(1))),
                    Term::Lit(Lit::Int(1)),
                ],
            ),
        ],
    );
    let term = Term::let_in("id", id, body);
    let scheme = engine.infer_scheme(&env, &term).expect("infer");
    assert_eq!(scheme.qual.ty, MonoType::boolean());
    assert!(scheme.qual.preds.is_empty());
}

#[test]
fn test_lambda_parameter_is_monomorphic() {
    let mut engine = InferenceEngine::with_prelude();
    let env = engine.prelude_env().clone();
    // \f. and (f true) (f 1)  -- f cannot be both Bool->_ and Int->_
    let term = Term::abs(
        "f",
        Term::apply_many(
            Term::var("and"),
            [
                Term::app(Term::var("f"), Term::Lit(Lit::Bool(true))),
                Term::app(Term::var("f"), Term::Lit(Lit::Int(1))),
            ],
        ),
    );
    assert!(engine.infer_scheme(&env, &term).is_err());
}

#[test]
fn test_scheme_normalization_alpha_equivalent() {
    let s1 = TypeScheme::new(
        vec![5],
        Vec::new(),
        QualType::plain(MonoType::func(MonoType::Var(5), MonoType::Var(5))),
    );
    let s2 = TypeScheme::new(
        vec![9],
        Vec::new(),
        QualType::plain(MonoType::func(MonoType::Var(9), MonoType::Var(9))),
    );
    assert_eq!(s1.normalized(), s2.normalized());
}

// --------------------------------------------------------------------------
// Algebraic data types & exhaustiveness
// --------------------------------------------------------------------------

fn declare_maybe(engine: &mut InferenceEngine) {
    engine.data_mut().declare(DataDecl::new(
        "Maybe",
        vec![0],
        vec![
            Constructor::nullary("Nothing"),
            Constructor::new("Just", vec![MonoType::Var(0)]),
        ],
    ));
}

fn declare_status(engine: &mut InferenceEngine) {
    engine.data_mut().declare(DataDecl::enumeration(
        "Status",
        [
            "Active".to_string(),
            "Inactive".to_string(),
            "Pending".to_string(),
        ],
    ));
}

#[test]
fn test_adt_constructor_application() {
    let mut engine = InferenceEngine::with_prelude();
    declare_maybe(&mut engine);
    let env = engine.prelude_env().clone();
    let term = Term::Construct("Just".to_string(), vec![Term::Lit(Lit::Int(5))]);
    let scheme = engine.infer_scheme(&env, &term).expect("infer");
    assert_eq!(
        scheme.qual.ty,
        MonoType::app("Maybe", vec![MonoType::int()])
    );
}

#[test]
fn test_adt_polymorphic_nullary_constructor() {
    let mut engine = InferenceEngine::with_prelude();
    declare_maybe(&mut engine);
    let env = engine.prelude_env().clone();
    let term = Term::Construct("Nothing".to_string(), Vec::new());
    let scheme = engine.infer_scheme(&env, &term).expect("infer");
    let expected = TypeScheme::new(
        vec![0],
        Vec::new(),
        QualType::plain(MonoType::app("Maybe", vec![MonoType::Var(0)])),
    )
    .normalized();
    assert_eq!(scheme, expected);
}

#[test]
fn test_adt_constructor_arity_error() {
    let mut engine = InferenceEngine::with_prelude();
    declare_maybe(&mut engine);
    let env = engine.prelude_env().clone();
    let term = Term::Construct("Just".to_string(), Vec::new());
    let err = engine.infer_scheme(&env, &term).unwrap_err();
    assert!(matches!(err, TypeInferError::ConstructorArity { .. }));
}

#[test]
fn test_match_exhaustive_enum() {
    let mut engine = InferenceEngine::with_prelude();
    declare_status(&mut engine);
    let env = engine.prelude_env().clone();
    let term = Term::Match(
        Box::new(Term::Construct("Active".to_string(), Vec::new())),
        vec![
            MatchArm::new(Pattern::nullary("Active"), Term::Lit(Lit::Int(1))),
            MatchArm::new(Pattern::nullary("Inactive"), Term::Lit(Lit::Int(2))),
            MatchArm::new(Pattern::nullary("Pending"), Term::Lit(Lit::Int(3))),
        ],
    );
    let scheme = engine.infer_scheme(&env, &term).expect("exhaustive");
    assert_eq!(scheme.qual.ty, MonoType::int());
}

#[test]
fn test_match_non_exhaustive_detected() {
    let mut engine = InferenceEngine::with_prelude();
    declare_status(&mut engine);
    let env = engine.prelude_env().clone();
    let term = Term::Match(
        Box::new(Term::Construct("Active".to_string(), Vec::new())),
        vec![
            MatchArm::new(Pattern::nullary("Active"), Term::Lit(Lit::Int(1))),
            MatchArm::new(Pattern::nullary("Inactive"), Term::Lit(Lit::Int(2))),
        ],
    );
    let err = engine.infer_scheme(&env, &term).unwrap_err();
    match err {
        TypeInferError::NonExhaustiveMatch { missing } => {
            assert!(missing.contains(&"Pending".to_string()));
        }
        other => panic!("expected NonExhaustiveMatch, got {other}"),
    }
}

#[test]
fn test_match_wildcard_is_exhaustive() {
    let mut engine = InferenceEngine::with_prelude();
    declare_status(&mut engine);
    let env = engine.prelude_env().clone();
    let term = Term::Match(
        Box::new(Term::Construct("Active".to_string(), Vec::new())),
        vec![
            MatchArm::new(Pattern::nullary("Active"), Term::Lit(Lit::Int(1))),
            MatchArm::new(Pattern::Wildcard, Term::Lit(Lit::Int(0))),
        ],
    );
    let scheme = engine.infer_scheme(&env, &term).expect("wildcard exhausts");
    assert_eq!(scheme.qual.ty, MonoType::int());
}

#[test]
fn test_match_nested_constructor_exhaustive() {
    let mut engine = InferenceEngine::with_prelude();
    declare_maybe(&mut engine);
    declare_status(&mut engine);
    let env = engine.prelude_env().clone();
    // match (Just Active) { Just Active -> 1; Just _ -> 2; Nothing -> 3 }
    let scrut = Term::Construct(
        "Just".to_string(),
        vec![Term::Construct("Active".to_string(), Vec::new())],
    );
    let term = Term::Match(
        Box::new(scrut),
        vec![
            MatchArm::new(
                Pattern::constructor("Just", vec![Pattern::nullary("Active")]),
                Term::Lit(Lit::Int(1)),
            ),
            MatchArm::new(
                Pattern::constructor("Just", vec![Pattern::Wildcard]),
                Term::Lit(Lit::Int(2)),
            ),
            MatchArm::new(Pattern::nullary("Nothing"), Term::Lit(Lit::Int(3))),
        ],
    );
    let scheme = engine.infer_scheme(&env, &term).expect("nested exhaustive");
    assert_eq!(scheme.qual.ty, MonoType::int());
}

// --------------------------------------------------------------------------
// Type classes: constraint solving & dictionaries
// --------------------------------------------------------------------------

#[test]
fn test_class_discharge_ground_instance() {
    let engine = InferenceEngine::with_prelude();
    assert!(
        engine
            .classes()
            .entails(&[], &Pred::new("Ord", MonoType::int()))
    );
}

#[test]
fn test_class_no_instance_is_rejected() {
    let engine = InferenceEngine::with_prelude();
    let subst = Subst::new();
    // Bool has Eq but not Ord.
    let err = engine
        .classes()
        .reduce(&subst, &[Pred::new("Ord", MonoType::boolean())])
        .unwrap_err();
    assert!(matches!(err, TypeInferError::NoInstance { .. }));
}

#[test]
fn test_class_instance_with_context() {
    let engine = InferenceEngine::with_prelude();
    // Eq (List Int) holds via `Eq a => Eq (List a)` plus `Eq Int`.
    assert!(
        engine
            .classes()
            .entails(&[], &Pred::new("Eq", MonoType::list(MonoType::int())))
    );
}

#[test]
fn test_class_superclass_entailment() {
    let engine = InferenceEngine::with_prelude();
    let given = vec![Pred::new("Ord", MonoType::Var(0))];
    // Ord a entails its superclass Eq a.
    assert!(
        engine
            .classes()
            .entails(&given, &Pred::new("Eq", MonoType::Var(0)))
    );
}

#[test]
fn test_class_evidence_dictionary() {
    let engine = InferenceEngine::with_prelude();
    let goal = Pred::new("Eq", MonoType::list(MonoType::int()));
    let evidence = engine.classes().resolve(&[], &goal).expect("evidence");
    match evidence {
        Evidence::Instance { head, args } => {
            assert_eq!(head, goal);
            assert_eq!(args.len(), 1);
            assert_eq!(args[0].predicate(), &Pred::new("Eq", MonoType::int()));
        }
        other => panic!("expected instance evidence, got {other:?}"),
    }
}

#[test]
fn test_class_match_pred_is_one_way() {
    let head = Pred::new("Eq", MonoType::list(MonoType::Var(0)));
    let goal = Pred::new("Eq", MonoType::list(MonoType::int()));
    let subst = match_pred(&head, &goal).expect("match");
    assert_eq!(subst.apply_type(&MonoType::Var(0)), MonoType::int());
    // Matching the (more specific) goal against the (more general) head fails.
    assert!(match_pred(&goal, &head).is_none());
}

#[test]
fn test_ambiguous_constraint_rejected() {
    let mut engine = InferenceEngine::with_prelude();
    // forall a. Ord a => Bool -- the variable appears only in the constraint.
    engine.prelude_mut().insert(
        "ambiguous",
        TypeScheme::new(
            vec![0],
            Vec::new(),
            QualType::new(
                vec![Pred::new("Ord", MonoType::Var(0))],
                MonoType::boolean(),
            ),
        ),
    );
    let env = engine.prelude_env().clone();
    let err = engine
        .infer_scheme(&env, &Term::var("ambiguous"))
        .unwrap_err();
    assert!(matches!(err, TypeInferError::AmbiguousConstraint { .. }));
}

// --------------------------------------------------------------------------
// Row polymorphism: records & effect parameters
// --------------------------------------------------------------------------

#[test]
fn test_record_select_is_row_polymorphic() {
    let mut engine = InferenceEngine::with_prelude();
    let env = engine.prelude_env().clone();
    // \r. r.age  :  forall a r0. { age: a | r0 } -> a
    let term = Term::abs("r", Term::select("age", Term::var("r")));
    let scheme = engine.infer_scheme(&env, &term).expect("infer");
    assert_eq!(scheme.type_vars.len(), 1);
    assert_eq!(scheme.row_vars.len(), 1);
    match &scheme.qual.ty {
        MonoType::Fun(from, to) => match from.as_ref() {
            MonoType::Record(row) => {
                assert!(row.is_open());
                assert_eq!(row.fields.get("age"), Some(to.as_ref()));
            }
            other => panic!("expected record domain, got {other}"),
        },
        other => panic!("expected function, got {other}"),
    }
}

#[test]
fn test_record_extend_then_select() {
    let mut engine = InferenceEngine::with_prelude();
    let env = engine.prelude_env().clone();
    let base = Term::Record(vec![(
        "name".to_string(),
        Term::Lit(Lit::Str("x".to_string())),
    )]);
    let extended = Term::RecordExtend(
        "age".to_string(),
        Box::new(Term::Lit(Lit::Int(30))),
        Box::new(base),
    );
    let select = Term::RecordSelect("age".to_string(), Box::new(extended));
    let scheme = engine.infer_scheme(&env, &select).expect("infer");
    assert_eq!(scheme.qual.ty, MonoType::int());
}

#[test]
fn test_effect_satisfies_row_polymorphism() {
    let mut engine = InferenceEngine::with_prelude();
    let effect = EffectNode {
        effect_type: "monetary".to_string(),
        description: "pay".to_string(),
        parameters: vec![
            ("amount".to_string(), "500".to_string()),
            ("currency".to_string(), "USD".to_string()),
        ],
    };
    // Open requirement "{ amount: Int | r }" -> satisfied (provides at least).
    let required_open = Row::open(0).with("amount", MonoType::int());
    assert!(engine.effect_satisfies(&effect, &required_open));
    // Closed requirement "{ amount: Int }" -> not satisfied (extra `currency`).
    let required_closed = Row::empty().with("amount", MonoType::int());
    assert!(!engine.effect_satisfies(&effect, &required_closed));
    // Wrong field type -> not satisfied.
    let required_bad = Row::open(0).with("amount", MonoType::string());
    assert!(!engine.effect_satisfies(&effect, &required_bad));
    // Demanding an absent field -> not satisfied.
    let required_missing = Row::open(0).with("nonexistent", MonoType::int());
    assert!(!engine.effect_satisfies(&effect, &required_missing));
}

#[test]
fn test_effect_row_infers_parameter_types() {
    let engine = InferenceEngine::with_prelude();
    let effect = EffectNode {
        effect_type: "status".to_string(),
        description: "adjust".to_string(),
        parameters: vec![
            ("count".to_string(), "3".to_string()),
            ("rate".to_string(), "0.05".to_string()),
            ("active".to_string(), "true".to_string()),
            ("effective".to_string(), "2025-01-01".to_string()),
            ("label".to_string(), "primary".to_string()),
        ],
    };
    let row = engine.effect_row(&effect);
    assert_eq!(row.fields.get("count"), Some(&MonoType::int()));
    assert_eq!(row.fields.get("rate"), Some(&MonoType::decimal()));
    assert_eq!(row.fields.get("active"), Some(&MonoType::boolean()));
    assert_eq!(row.fields.get("effective"), Some(&MonoType::date()));
    assert_eq!(row.fields.get("label"), Some(&MonoType::string()));
    assert!(row.is_closed());
}

// --------------------------------------------------------------------------
// Condition / statute / document inference (AST integration)
// --------------------------------------------------------------------------

#[test]
fn test_infer_condition_comparison_field_type() {
    let mut engine = InferenceEngine::with_prelude();
    let cond = ConditionNode::Comparison {
        field: "age".to_string(),
        operator: ">=".to_string(),
        value: ConditionValue::Number(18),
    };
    let typing = engine.infer_condition(&cond).expect("infer");
    assert_eq!(typing.field_type("age"), Some(&MonoType::int()));
    assert!(typing.is_open());
}

#[test]
fn test_infer_condition_like_forces_string() {
    let mut engine = InferenceEngine::with_prelude();
    let cond = ConditionNode::Like {
        field: "name".to_string(),
        pattern: "A%".to_string(),
    };
    let typing = engine.infer_condition(&cond).expect("infer");
    assert_eq!(typing.field_type("name"), Some(&MonoType::string()));
}

#[test]
fn test_infer_condition_set_membership() {
    let mut engine = InferenceEngine::with_prelude();
    let cond = ConditionNode::In {
        field: "code".to_string(),
        values: vec![ConditionValue::Number(1), ConditionValue::Number(2)],
    };
    let typing = engine.infer_condition(&cond).expect("infer");
    assert_eq!(typing.field_type("code"), Some(&MonoType::int()));
}

#[test]
fn test_infer_condition_between_is_int() {
    let mut engine = InferenceEngine::with_prelude();
    let cond = ConditionNode::Between {
        field: "age".to_string(),
        min: ConditionValue::Number(18),
        max: ConditionValue::Number(65),
    };
    let typing = engine.infer_condition(&cond).expect("infer");
    assert_eq!(typing.field_type("age"), Some(&MonoType::int()));
}

#[test]
fn test_infer_condition_temporal_date_field() {
    let mut engine = InferenceEngine::with_prelude();
    let cond = ConditionNode::TemporalComparison {
        field: crate::ast::TemporalField::DateField("expiry".to_string()),
        operator: ">".to_string(),
        value: ConditionValue::Date("2025-01-01".to_string()),
    };
    let typing = engine.infer_condition(&cond).expect("infer");
    assert_eq!(typing.field_type("expiry"), Some(&MonoType::date()));
}

#[test]
fn test_infer_conditions_type_conflict() {
    let mut engine = InferenceEngine::with_prelude();
    let conditions = [
        ConditionNode::Comparison {
            field: "age".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::String("adult".to_string()),
        },
        ConditionNode::Comparison {
            field: "age".to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(18),
        },
    ];
    let err = engine.infer_conditions(&conditions).unwrap_err();
    assert!(matches!(err, TypeInferError::Mismatch { .. }));
}

#[test]
fn test_infer_statute_shares_entity_across_conditions() {
    let mut engine = InferenceEngine::with_prelude();
    let statute = StatuteNode {
        id: "eligibility".to_string(),
        conditions: vec![
            ConditionNode::And(
                Box::new(ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">=".to_string(),
                    value: ConditionValue::Number(18),
                }),
                Box::new(ConditionNode::Comparison {
                    field: "income".to_string(),
                    operator: ">".to_string(),
                    value: ConditionValue::Number(30000),
                }),
            ),
            ConditionNode::HasAttribute {
                key: "citizen".to_string(),
            },
        ],
        ..Default::default()
    };
    let typing = engine.infer_statute(&statute).expect("infer");
    assert_eq!(typing.field_type("age"), Some(&MonoType::int()));
    assert_eq!(typing.field_type("income"), Some(&MonoType::int()));
    assert!(typing.field_types().contains_key("citizen"));
    assert!(typing.is_open());
}

#[test]
fn test_infer_document_multiple_statutes() {
    let mut engine = InferenceEngine::with_prelude();
    let make = |id: &str, field: &str| StatuteNode {
        id: id.to_string(),
        conditions: vec![ConditionNode::Comparison {
            field: field.to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(1),
        }],
        ..Default::default()
    };
    let doc = LegalDocument {
        namespace: None,
        imports: Vec::new(),
        exports: Vec::new(),
        statutes: vec![make("s1", "age"), make("s2", "score")],
    };
    let typing = engine.infer_document(&doc).expect("infer");
    assert_eq!(typing.statutes.len(), 2);
    assert_eq!(
        typing.statute("s1").and_then(|t| t.field_type("age")),
        Some(&MonoType::int())
    );
    assert_eq!(
        typing.statute("s2").and_then(|t| t.field_type("score")),
        Some(&MonoType::int())
    );
}

#[test]
fn test_numeric_polymorphic_function() {
    let mut engine = InferenceEngine::with_prelude();
    let env = engine.prelude_env().clone();
    // add 1 2 : Int  (Numeric Int discharged)
    let term = Term::apply_many(
        Term::var("add"),
        [Term::Lit(Lit::Int(1)), Term::Lit(Lit::Int(2))],
    );
    let scheme = engine.infer_scheme(&env, &term).expect("infer");
    assert_eq!(scheme.qual.ty, MonoType::int());
    assert!(scheme.qual.preds.is_empty());
    // add true false : no Numeric Bool instance -> rejected.
    let bad = Term::apply_many(
        Term::var("add"),
        [Term::Lit(Lit::Bool(true)), Term::Lit(Lit::Bool(false))],
    );
    assert!(engine.infer_scheme(&env, &bad).is_err());
}
