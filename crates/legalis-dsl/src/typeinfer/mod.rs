//! Advanced type system for DSL conditions and effects (v0.2.4).
//!
//! This module adds a genuine Hindley–Milner type-inference engine on top of the
//! existing AST. It is **additive** and intentionally distinct from the
//! lightweight [`crate::type_checker`] (which performs a simple, non-polymorphic
//! compatibility check): here we provide
//!
//! 1. **Hindley–Milner type inference for conditions** — Algorithm W with
//!    unification, the occurs-check and let-generalization
//!    ([`InferenceEngine`], [`infer`](InferenceEngine::infer)).
//! 2. **Algebraic data types** — sum/product declarations with constructors and
//!    exhaustiveness-checked `match` ([`DataDecl`], [`DataEnv`], [`Pattern`]).
//! 3. **Polymorphic condition functions** — parametric polymorphism via
//!    [`TypeScheme`]s and let-generalization (e.g. `id : forall a. a -> a`).
//! 4. **Type classes for condition behaviours** — constraint sets with context
//!    reduction and dictionary/evidence construction ([`ClassEnv`],
//!    [`Pred`], [`Evidence`]).
//! 5. **Row polymorphism for effect parameters** — extensible records with row
//!    variables ([`Row`], [`InferenceEngine::effect_satisfies`]).
//!
//! Conditions are lowered into a typed [`Term`] IR over a shared, row-polymorphic
//! *entity* record, so inferring a statute yields the inferred type of every
//! attribute it references together with an open row tail (the statute applies
//! to any entity that carries *at least* those attributes).
//!
//! ```
//! use legalis_dsl::typeinfer::{InferenceEngine, MonoType};
//! use legalis_dsl::{ConditionNode, ConditionValue};
//!
//! let mut engine = InferenceEngine::with_prelude();
//! let cond = ConditionNode::Comparison {
//!     field: "age".to_string(),
//!     operator: ">=".to_string(),
//!     value: ConditionValue::Number(18),
//! };
//! let typing = engine.infer_condition(&cond).expect("infer");
//! assert_eq!(typing.field_type("age"), Some(&MonoType::int()));
//! assert!(typing.is_open()); // row-polymorphic: more attributes allowed
//! ```

mod adt;
mod classes;
mod error;
mod infer;
mod subst;
mod term;
mod types;
mod unify;

#[cfg(test)]
mod tests;

pub use adt::{Constructor, DataDecl, DataEnv};
pub use classes::{ClassEnv, ClassInfo, Evidence, Instance, match_pred};
pub use error::{InferResult, TypeInferError};
pub use infer::{InferenceEngine, TypeEnv};
pub use subst::Subst;
pub use term::{MatchArm, Pattern, Term};
pub use types::{FreeVars, Kind, Lit, MonoType, Pred, QualType, Row, TypeScheme, VarId, VarSupply};
pub use unify::{unify, unify_rows};

use std::collections::BTreeMap;

use crate::ast::{
    ConditionNode, ConditionValue, EffectNode, LegalDocument, SetExpression, StatuteNode,
    TemporalField,
};

/// The name of the synthetic entity record threaded through a statute's
/// conditions; every attribute reference selects a field from it.
const ENTITY: &str = "$entity";

/// The inferred typing of an entity (the attributes a condition or statute
/// constrains), together with any residual type-class predicates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityTyping {
    /// The inferred (row-polymorphic) entity record.
    pub entity: Row,
    /// Residual class predicates after context reduction.
    pub predicates: Vec<Pred>,
}

impl EntityTyping {
    /// The inferred field types, keyed by attribute name.
    pub fn field_types(&self) -> &BTreeMap<String, MonoType> {
        &self.entity.fields
    }

    /// The inferred type of one attribute, if referenced.
    pub fn field_type(&self, name: &str) -> Option<&MonoType> {
        self.entity.fields.get(name)
    }

    /// `true` when the entity row is open — i.e. the condition/statute is
    /// row-polymorphic and applies to any entity carrying at least these
    /// attributes.
    pub fn is_open(&self) -> bool {
        self.entity.is_open()
    }
}

/// The inferred typings of every statute in a document.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DocumentTyping {
    /// Per-statute entity typings, in document order, keyed by statute id.
    pub statutes: Vec<(String, EntityTyping)>,
}

impl DocumentTyping {
    /// Looks up the typing inferred for a statute id.
    pub fn statute(&self, id: &str) -> Option<&EntityTyping> {
        self.statutes
            .iter()
            .find(|(sid, _)| sid == id)
            .map(|(_, typing)| typing)
    }
}

impl InferenceEngine {
    /// Builds an engine pre-loaded with the standard "condition behaviour"
    /// classes, their instances, the `List` data type, and the polymorphic
    /// condition functions used when lowering DSL conditions.
    pub fn with_prelude() -> Self {
        let mut engine = InferenceEngine::new();
        install_classes(&mut engine);
        install_data(&mut engine);
        install_functions(&mut engine);
        engine
    }

    /// Infers the attribute types referenced by a single condition.
    pub fn infer_condition(&mut self, condition: &ConditionNode) -> InferResult<EntityTyping> {
        self.infer_conditions(std::slice::from_ref(condition))
    }

    /// Infers the attribute types referenced by a slice of conditions, all sharing
    /// one entity record (so a field used in two conditions gets one type).
    pub fn infer_conditions(&mut self, conditions: &[ConditionNode]) -> InferResult<EntityTyping> {
        self.reset();
        let tail = self.fresh_row_var();
        let env = self
            .prelude_env()
            .extended(ENTITY, TypeScheme::mono(MonoType::record(Row::open(tail))));

        for condition in conditions {
            let term = lower_condition(condition);
            let ty = self.infer(&env, &term)?;
            self.unify(&ty, &MonoType::boolean())?;
        }

        self.finish_entity(tail)
    }

    /// Infers the attribute types referenced anywhere in a statute (its main
    /// conditions, exception/scope/delegate conditions and constraints), all over
    /// the same entity record.
    pub fn infer_statute(&mut self, statute: &StatuteNode) -> InferResult<EntityTyping> {
        self.reset();
        let tail = self.fresh_row_var();
        let env = self
            .prelude_env()
            .extended(ENTITY, TypeScheme::mono(MonoType::record(Row::open(tail))));

        for condition in statute_conditions(statute) {
            let term = lower_condition(condition);
            let ty = self.infer(&env, &term)?;
            self.unify(&ty, &MonoType::boolean())?;
        }

        self.finish_entity(tail)
    }

    /// Infers the entity typing of every statute in a document.
    pub fn infer_document(&mut self, doc: &LegalDocument) -> InferResult<DocumentTyping> {
        let mut statutes = Vec::with_capacity(doc.statutes.len());
        for statute in &doc.statutes {
            let typing = self.infer_statute(statute)?;
            statutes.push((statute.id.clone(), typing));
        }
        Ok(DocumentTyping { statutes })
    }

    /// Finalizes an entity inference session: reduces the collected constraints
    /// and resolves the entity row.
    fn finish_entity(&mut self, tail: VarId) -> InferResult<EntityTyping> {
        let predicates = self.take_reduced_constraints()?;
        let entity = self.apply_row(&Row::open(tail));
        Ok(EntityTyping { entity, predicates })
    }

    /// The row of types for an effect's parameters (a closed record), inferring
    /// each parameter's type from the textual value.
    pub fn effect_row(&self, effect: &EffectNode) -> Row {
        effect_parameter_row(effect)
    }

    /// `true` when an effect provides at least the parameters demanded by a
    /// (possibly open) `required` row, with compatible types.
    ///
    /// This is row polymorphism applied to effects: a handler that requires
    /// `{ amount: Int | r }` accepts any effect that carries an `Int` `amount`,
    /// regardless of what else it carries.
    pub fn effect_satisfies(&mut self, effect: &EffectNode, required: &Row) -> bool {
        self.reset();
        let provided = MonoType::record(effect_parameter_row(effect));
        let demanded = MonoType::record(required.clone());
        self.unify(&provided, &demanded).is_ok()
    }
}

/// Collects every condition referenced anywhere in a statute.
fn statute_conditions(statute: &StatuteNode) -> Vec<&ConditionNode> {
    let mut out: Vec<&ConditionNode> = statute.conditions.iter().collect();
    for exception in &statute.exceptions {
        out.extend(exception.conditions.iter());
    }
    for delegate in &statute.delegates {
        out.extend(delegate.conditions.iter());
    }
    if let Some(scope) = &statute.scope {
        out.extend(scope.conditions.iter());
    }
    for constraint in &statute.constraints {
        out.push(&constraint.condition);
    }
    out
}

/// Lowers a [`ConditionNode`] into a boolean-typed [`Term`] over the entity
/// record. Field references become row-polymorphic record selections, so the
/// inferred entity row grows one field per referenced attribute.
fn lower_condition(condition: &ConditionNode) -> Term {
    match condition {
        ConditionNode::Comparison {
            field,
            operator,
            value,
        } => {
            let func = if is_ordering_operator(operator) {
                "cmp"
            } else {
                "eq"
            };
            Term::apply_many(Term::var(func), [field_access(field), value_term(value)])
        }
        ConditionNode::HasAttribute { key } => Term::app(Term::var("present"), field_access(key)),
        ConditionNode::Between { field, min, max } => Term::apply_many(
            Term::var("between"),
            [field_access(field), value_term(min), value_term(max)],
        ),
        ConditionNode::In { field, values } => Term::apply_many(
            Term::var("member"),
            [field_access(field), list_term(values)],
        ),
        ConditionNode::Like { field, pattern } => Term::apply_many(
            Term::var("like"),
            [field_access(field), Term::Lit(Lit::Str(pattern.clone()))],
        ),
        ConditionNode::Matches {
            field,
            regex_pattern,
        } => Term::apply_many(
            Term::var("like"),
            [
                field_access(field),
                Term::Lit(Lit::Str(regex_pattern.clone())),
            ],
        ),
        ConditionNode::InRange {
            field, min, max, ..
        } => Term::apply_many(
            Term::var("between"),
            [field_access(field), value_term(min), value_term(max)],
        ),
        ConditionNode::NotInRange {
            field, min, max, ..
        } => Term::app(
            Term::var("not"),
            Term::apply_many(
                Term::var("between"),
                [field_access(field), value_term(min), value_term(max)],
            ),
        ),
        ConditionNode::TemporalComparison {
            field,
            operator,
            value,
        } => {
            let left = match field {
                TemporalField::CurrentDate => Term::Lit(Lit::Date("CURRENT_DATE".to_string())),
                TemporalField::DateField(name) => field_access(name),
            };
            let func = if is_ordering_operator(operator) {
                "cmp"
            } else {
                "eq"
            };
            Term::apply_many(Term::var(func), [left, value_term(value)])
        }
        ConditionNode::And(left, right) => Term::apply_many(
            Term::var("and"),
            [lower_condition(left), lower_condition(right)],
        ),
        ConditionNode::Or(left, right) => Term::apply_many(
            Term::var("or"),
            [lower_condition(left), lower_condition(right)],
        ),
        ConditionNode::Not(inner) => Term::app(Term::var("not"), lower_condition(inner)),
    }
}

/// `true` for the order-comparison operators (which demand `Ord`).
fn is_ordering_operator(operator: &str) -> bool {
    matches!(operator, "<" | ">" | "<=" | ">=")
}

/// Builds an entity field selection term.
fn field_access(field: &str) -> Term {
    Term::select(field, Term::var(ENTITY))
}

/// Lowers a condition value to a literal (or list) term.
fn value_term(value: &ConditionValue) -> Term {
    match value {
        ConditionValue::Number(n) => Term::Lit(Lit::Int(*n)),
        ConditionValue::String(s) => Term::Lit(Lit::Str(s.clone())),
        ConditionValue::Boolean(b) => Term::Lit(Lit::Bool(*b)),
        ConditionValue::Date(d) => Term::Lit(Lit::Date(d.clone())),
        ConditionValue::SetExpr(expr) => list_term(&flatten_set(expr)),
    }
}

/// Builds a `List` term (`Cons v0 (Cons v1 ... Nil)`) from condition values, so
/// all elements are unified to a single element type.
fn list_term(values: &[ConditionValue]) -> Term {
    let mut term = Term::Construct("Nil".to_string(), Vec::new());
    for value in values.iter().rev() {
        term = Term::Construct("Cons".to_string(), vec![value_term(value), term]);
    }
    term
}

/// Flattens a set expression to the values it mentions (type inference only
/// cares about the element types, not the set algebra).
fn flatten_set(expr: &SetExpression) -> Vec<ConditionValue> {
    match expr {
        SetExpression::Values(values) => values.clone(),
        SetExpression::Union(a, b)
        | SetExpression::Intersect(a, b)
        | SetExpression::Difference(a, b) => {
            let mut out = flatten_set(a);
            out.extend(flatten_set(b));
            out
        }
    }
}

/// Infers the type of one effect parameter value from its textual form.
fn parameter_type(value: &str) -> MonoType {
    let trimmed = value.trim();
    if trimmed.parse::<i64>().is_ok() {
        return MonoType::int();
    }
    if trimmed.eq_ignore_ascii_case("true") || trimmed.eq_ignore_ascii_case("false") {
        return MonoType::boolean();
    }
    if is_iso_date(trimmed) {
        return MonoType::date();
    }
    if is_decimal(trimmed) {
        return MonoType::decimal();
    }
    MonoType::string()
}

/// Builds the closed row of an effect's parameters.
fn effect_parameter_row(effect: &EffectNode) -> Row {
    let mut fields = BTreeMap::new();
    for (key, value) in &effect.parameters {
        fields.insert(key.clone(), parameter_type(value));
    }
    Row::new(fields, None)
}

/// Recognises an `YYYY-MM-DD` date.
fn is_iso_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    text.char_indices().all(|(idx, ch)| match idx {
        4 | 7 => ch == '-',
        _ => ch.is_ascii_digit(),
    })
}

/// Recognises a decimal number with a fractional part (`12.5`, `-0.05`).
fn is_decimal(text: &str) -> bool {
    let body = text.strip_prefix('-').unwrap_or(text);
    match body.split_once('.') {
        Some((int_part, frac_part)) => {
            !int_part.is_empty()
                && !frac_part.is_empty()
                && int_part.bytes().all(|b| b.is_ascii_digit())
                && frac_part.bytes().all(|b| b.is_ascii_digit())
        }
        None => false,
    }
}

/// Registers the condition-behaviour classes and their instances.
fn install_classes(engine: &mut InferenceEngine) {
    let classes = engine.classes_mut();
    classes.declare_class("Eq", Vec::new());
    classes.declare_class("Ord", vec!["Eq".to_string()]);
    classes.declare_class("Numeric", vec!["Eq".to_string()]);
    classes.declare_class("Matchable", Vec::new());

    for ty in [
        MonoType::int(),
        MonoType::boolean(),
        MonoType::string(),
        MonoType::date(),
        MonoType::decimal(),
    ] {
        classes.add_instance(Instance::ground("Eq", ty));
    }
    for ty in [
        MonoType::int(),
        MonoType::string(),
        MonoType::date(),
        MonoType::decimal(),
    ] {
        classes.add_instance(Instance::ground("Ord", ty));
    }
    classes.add_instance(Instance::ground("Numeric", MonoType::int()));
    classes.add_instance(Instance::ground("Numeric", MonoType::decimal()));
    classes.add_instance(Instance::ground("Matchable", MonoType::string()));

    // Eq (List a) requires Eq a — a context-bearing instance.
    let elem = MonoType::Var(0);
    classes.add_instance(Instance::new(
        vec![Pred::new("Eq", elem.clone())],
        Pred::new("Eq", MonoType::list(elem)),
    ));
}

/// Registers the built-in `List` data type used to lower set membership.
fn install_data(engine: &mut InferenceEngine) {
    let elem = 0;
    let list_decl = DataDecl::new(
        "List",
        vec![elem],
        vec![
            Constructor::nullary("Nil"),
            Constructor::new(
                "Cons",
                vec![MonoType::Var(elem), MonoType::list(MonoType::Var(elem))],
            ),
        ],
    );
    engine.data_mut().declare(list_decl);
}

/// Registers the polymorphic, class-constrained condition functions that DSL
/// conditions lower into.
fn install_functions(engine: &mut InferenceEngine) {
    let prelude = engine.prelude_mut();
    let boolean = MonoType::boolean;

    // Boolean combinators (monomorphic).
    prelude.insert(
        "and",
        TypeScheme::mono(MonoType::arrow(vec![boolean(), boolean()], boolean())),
    );
    prelude.insert(
        "or",
        TypeScheme::mono(MonoType::arrow(vec![boolean(), boolean()], boolean())),
    );
    prelude.insert(
        "not",
        TypeScheme::mono(MonoType::func(boolean(), boolean())),
    );

    // forall a. Eq a => a -> a -> Bool
    prelude.insert(
        "eq",
        poly1("Eq", |a| MonoType::arrow(vec![a.clone(), a], boolean())),
    );
    // forall a. Ord a => a -> a -> Bool
    prelude.insert(
        "cmp",
        poly1("Ord", |a| MonoType::arrow(vec![a.clone(), a], boolean())),
    );
    // forall a. Ord a => a -> a -> a -> Bool
    prelude.insert(
        "between",
        poly1("Ord", |a| {
            MonoType::arrow(vec![a.clone(), a.clone(), a], boolean())
        }),
    );
    // forall a. Eq a => a -> List a -> Bool
    prelude.insert(
        "member",
        poly1("Eq", |a| {
            MonoType::arrow(vec![a.clone(), MonoType::list(a)], boolean())
        }),
    );
    // forall a. Matchable a => a -> a -> Bool
    prelude.insert(
        "like",
        poly1("Matchable", |a| {
            MonoType::arrow(vec![a.clone(), a], boolean())
        }),
    );
    // forall a. Numeric a => a -> a -> a
    prelude.insert(
        "add",
        poly1("Numeric", |a| {
            MonoType::arrow(vec![a.clone(), a.clone()], a)
        }),
    );
    // forall a. a -> Bool  (attribute presence)
    prelude.insert(
        "present",
        TypeScheme::new(
            vec![0],
            Vec::new(),
            QualType::plain(MonoType::func(MonoType::Var(0), boolean())),
        ),
    );
}

/// Builds a one-variable scheme `forall a. Class a => body(a)`.
fn poly1(class: &str, body: impl Fn(MonoType) -> MonoType) -> TypeScheme {
    let a = MonoType::Var(0);
    TypeScheme::new(
        vec![0],
        Vec::new(),
        QualType::new(vec![Pred::new(class, a.clone())], body(a)),
    )
}
