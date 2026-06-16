//! Formal-specification export for the Legalis DSL (v0.2.9).
//!
//! This module lowers the legal DSL AST into well-formed source for a range of
//! formal-methods tools so that statutes can be reasoned about with proof
//! assistants, model checkers, constraint analysers and SMT solvers:
//!
//! * [`CoqExporter`] — Gallina for the Coq proof assistant.
//! * [`Lean4Exporter`] — Lean 4 theorem prover.
//! * [`TlaExporter`] — TLA+ for model checking.
//! * [`AlloyExporter`] — Alloy for constraint analysis.
//! * [`SmtLibExporter`] — SMT-LIB 2 for the OxiZ SMT solver (and any
//!   SMT-LIB 2.6 compatible engine).
//!
//! All five share a single, well-tested lowering: a [`DocumentSpec`] holding a
//! [`FieldRegistry`] (the typed entity record inferred from every condition) and
//! a list of [`StatuteSpec`]s, each carrying a precondition [`Formula`], any
//! exception carve-outs, `REQUIRES` dependencies and effects. Backends only
//! pretty-print this intermediate representation, so the semantically tricky
//! AST → logic translation lives in exactly one place.
//!
//! The export is purely additive and reuses the existing
//! [`crate::ast`] types without duplication.

use crate::DslResult;
use crate::ast::{
    ConditionNode, ConditionValue, EffectNode, LegalDocument, StatuteNode, TemporalField,
};

mod alloy;
mod coq;
mod lean;
mod smtlib;
#[cfg(test)]
mod tests;
mod tla;

pub use alloy::AlloyExporter;
pub use coq::CoqExporter;
pub use lean::Lean4Exporter;
pub use smtlib::SmtLibExporter;
pub use tla::TlaExporter;

/// Trait implemented by every formal-specification backend.
///
/// Mirrors the shape of [`crate::codegen::CodeGenerator`] but targets formal
/// verification languages rather than general-purpose code.
pub trait FormalExporter {
    /// Exports an entire document to the target formal language.
    fn export(&self, doc: &LegalDocument) -> DslResult<String>;

    /// Human-readable name of the target system (e.g. `"Coq"`).
    fn target(&self) -> &str;

    /// File extension used for the generated artefact (without the dot).
    fn file_extension(&self) -> &str;

    /// Convenience helper that exports a single statute by wrapping it in a
    /// throwaway document.
    fn export_statute(&self, statute: &StatuteNode) -> DslResult<String> {
        let doc = LegalDocument {
            namespace: None,
            imports: Vec::new(),
            exports: Vec::new(),
            statutes: vec![statute.clone()],
        };
        self.export(&doc)
    }
}

// ---------------------------------------------------------------------------
// Scalar types and values
// ---------------------------------------------------------------------------

/// Scalar sort inferred for an entity field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarType {
    /// Integer-valued field.
    Int,
    /// Boolean-valued field.
    Bool,
    /// String-valued field.
    Str,
    /// Date field, normalised to an integer `YYYYMMDD` (an integer sort with
    /// date semantics).
    Date,
}

impl ScalarType {
    /// Returns `true` when the sort is represented as an integer.
    pub fn is_int_sort(self) -> bool {
        matches!(self, ScalarType::Int | ScalarType::Date)
    }

    /// Merge rank used to deterministically resolve a field referenced with
    /// conflicting types across statutes (higher wins).
    fn rank(self) -> u8 {
        match self {
            ScalarType::Bool => 0,
            ScalarType::Int => 1,
            ScalarType::Date => 2,
            ScalarType::Str => 3,
        }
    }
}

/// A literal scalar appearing on the right-hand side of a comparison.
#[derive(Debug, Clone, PartialEq)]
pub enum Scalar {
    /// Integer literal.
    Int(i64),
    /// Boolean literal.
    Bool(bool),
    /// String literal (already cleaned of quotes/backslashes).
    Str(String),
    /// Date literal with its normalised integer form when parseable.
    Date {
        /// Original `YYYY-MM-DD` text.
        raw: String,
        /// Normalised `YYYYMMDD` integer, when the text parses.
        as_int: Option<i64>,
    },
}

impl Scalar {
    /// Best-effort integer projection for integer-sort backends.
    pub fn as_int(&self) -> i64 {
        match self {
            Scalar::Int(n) => *n,
            Scalar::Date { as_int, .. } => as_int.unwrap_or(0),
            Scalar::Bool(b) => i64::from(*b),
            Scalar::Str(_) => 0,
        }
    }

    /// The inferred sort of this scalar.
    pub fn sort(&self) -> ScalarType {
        match self {
            Scalar::Int(_) => ScalarType::Int,
            Scalar::Bool(_) => ScalarType::Bool,
            Scalar::Str(_) => ScalarType::Str,
            Scalar::Date { .. } => ScalarType::Date,
        }
    }
}

/// Comparison operator in a [`Formula`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    /// `=`
    Eq,
    /// `!=`
    Ne,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
}

impl CmpOp {
    /// Parses a DSL operator string into a [`CmpOp`], defaulting to equality.
    pub fn parse(op: &str) -> Self {
        match op {
            "!=" | "<>" => CmpOp::Ne,
            "<" => CmpOp::Lt,
            "<=" => CmpOp::Le,
            ">" => CmpOp::Gt,
            ">=" => CmpOp::Ge,
            _ => CmpOp::Eq,
        }
    }

    /// Returns `true` for the ordering operators (`<`, `<=`, `>`, `>=`).
    pub fn is_ordering(self) -> bool {
        matches!(self, CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge)
    }
}

/// A reference to an entity field with its inferred sort.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldRef {
    /// Sanitised field identifier (valid in every target language).
    pub name: String,
    /// Inferred sort of the field.
    pub ty: ScalarType,
}

/// Logical formula over an entity record — the shared intermediate
/// representation that every backend pretty-prints.
#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    /// A boolean constant (`true`/`false`).
    Const(bool),
    /// A boolean field asserted to hold.
    BoolField(FieldRef),
    /// `field <op> value`.
    Compare {
        /// Left-hand field.
        field: FieldRef,
        /// Comparison operator.
        op: CmpOp,
        /// Right-hand literal.
        value: Scalar,
    },
    /// An inclusive/exclusive range `lo .. hi` over a field.
    Range {
        /// Field constrained by the range.
        field: FieldRef,
        /// Lower bound.
        lo: Scalar,
        /// Upper bound.
        hi: Scalar,
        /// Whether the lower bound is inclusive.
        incl_lo: bool,
        /// Whether the upper bound is inclusive.
        incl_hi: bool,
    },
    /// SQL-style `LIKE` pattern match.
    Like {
        /// Field being matched.
        field: FieldRef,
        /// Pattern (with `%` wildcards).
        pattern: String,
    },
    /// Regular-expression match.
    Matches {
        /// Field being matched.
        field: FieldRef,
        /// Regular expression text.
        pattern: String,
    },
    /// Logical negation.
    Not(Box<Formula>),
    /// Logical conjunction.
    And(Box<Formula>, Box<Formula>),
    /// Logical disjunction.
    Or(Box<Formula>, Box<Formula>),
}

impl Formula {
    /// Folds two formulas with conjunction, eliminating trivial `true`s.
    fn and(self, other: Formula) -> Formula {
        match (self, other) {
            (Formula::Const(true), f) | (f, Formula::Const(true)) => f,
            (a, b) => Formula::And(Box::new(a), Box::new(b)),
        }
    }

    /// Visits every sub-formula, invoking `f` on each node.
    fn walk(&self, f: &mut impl FnMut(&Formula)) {
        f(self);
        match self {
            Formula::Not(inner) => inner.walk(f),
            Formula::And(a, b) | Formula::Or(a, b) => {
                a.walk(f);
                b.walk(f);
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Effects
// ---------------------------------------------------------------------------

/// The kind of legal effect attached to a statute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectKind {
    /// Grants a right.
    Grant,
    /// Revokes a right.
    Revoke,
    /// Imposes an obligation.
    Obligation,
    /// Imposes a prohibition.
    Prohibition,
    /// Any other, custom effect type.
    Custom(String),
}

impl EffectKind {
    /// Classifies an effect-type string from the AST.
    fn classify(raw: &str) -> Self {
        match raw.to_lowercase().as_str() {
            "grant" => EffectKind::Grant,
            "revoke" => EffectKind::Revoke,
            "obligation" => EffectKind::Obligation,
            "prohibition" => EffectKind::Prohibition,
            _ => EffectKind::Custom(raw.to_string()),
        }
    }

    /// `+1` for rights/obligations created, `-1` for those withdrawn, `0`
    /// otherwise. Used for conflict detection.
    fn polarity(&self) -> i8 {
        match self {
            EffectKind::Grant | EffectKind::Obligation => 1,
            EffectKind::Revoke | EffectKind::Prohibition => -1,
            EffectKind::Custom(_) => 0,
        }
    }

    /// `Some("right")`/`Some("duty")` for the two conflict domains.
    fn domain(&self) -> Option<&'static str> {
        match self {
            EffectKind::Grant | EffectKind::Revoke => Some("right"),
            EffectKind::Obligation | EffectKind::Prohibition => Some("duty"),
            EffectKind::Custom(_) => None,
        }
    }
}

/// A single effect, lowered for export.
#[derive(Debug, Clone, PartialEq)]
pub struct EffectSpec {
    /// Classified effect kind.
    pub kind: EffectKind,
    /// Cleaned descriptive label.
    pub label: String,
}

// ---------------------------------------------------------------------------
// Field registry
// ---------------------------------------------------------------------------

/// The typed set of entity fields referenced anywhere in a document.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FieldRegistry {
    fields: std::collections::BTreeMap<String, ScalarType>,
}

impl FieldRegistry {
    /// Inserts (or merges) a field/type, keeping the higher-ranked sort on a
    /// conflict so the result is deterministic.
    fn add(&mut self, name: String, ty: ScalarType) {
        self.fields
            .entry(name)
            .and_modify(|existing| {
                if ty.rank() > existing.rank() {
                    *existing = ty;
                }
            })
            .or_insert(ty);
    }

    /// Looks up a field's sort, defaulting to [`ScalarType::Int`] for unknown
    /// names (which only occurs for internally consistent inputs when a name was
    /// never registered).
    pub fn get(&self, name: &str) -> ScalarType {
        self.fields.get(name).copied().unwrap_or(ScalarType::Int)
    }

    /// Iterates over fields in deterministic (sorted) order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &ScalarType)> {
        self.fields.iter()
    }

    /// Number of distinct fields.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns `true` when no fields were registered.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns `true` when any field has boolean sort.
    pub fn has_bool(&self) -> bool {
        self.fields.values().any(|t| *t == ScalarType::Bool)
    }

    /// Builds the registry from an entire document.
    fn from_document(doc: &LegalDocument) -> Self {
        let mut reg = FieldRegistry::default();
        for statute in &doc.statutes {
            for cond in &statute.conditions {
                register_condition(cond, &mut reg);
            }
            for exc in &statute.exceptions {
                for cond in &exc.conditions {
                    register_condition(cond, &mut reg);
                }
            }
            if let Some(scope) = &statute.scope {
                for cond in &scope.conditions {
                    register_condition(cond, &mut reg);
                }
            }
            for delegate in &statute.delegates {
                for cond in &delegate.conditions {
                    register_condition(cond, &mut reg);
                }
            }
            for constraint in &statute.constraints {
                register_condition(&constraint.condition, &mut reg);
            }
        }
        reg
    }
}

// ---------------------------------------------------------------------------
// Document / statute specs
// ---------------------------------------------------------------------------

/// A statute lowered into export-ready form.
#[derive(Debug, Clone, PartialEq)]
pub struct StatuteSpec {
    /// Original statute identifier (unsanitised).
    pub raw_id: String,
    /// Statute title.
    pub title: String,
    /// Conjoined precondition.
    pub precond: Formula,
    /// Exception carve-outs (each a formula that, when true, disables the rule).
    pub exceptions: Vec<Formula>,
    /// Identifiers this statute depends on (`REQUIRES`).
    pub requires: Vec<String>,
    /// Effects produced when the statute applies.
    pub effects: Vec<EffectSpec>,
}

/// A full document lowered into export-ready form, shared by all backends.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentSpec {
    /// Inferred entity record.
    pub fields: FieldRegistry,
    /// Lowered statutes.
    pub statutes: Vec<StatuteSpec>,
}

impl DocumentSpec {
    /// Lowers a [`LegalDocument`] into a [`DocumentSpec`].
    pub fn from_document(doc: &LegalDocument) -> Self {
        let fields = FieldRegistry::from_document(doc);
        let statutes = doc
            .statutes
            .iter()
            .map(|s| lower_statute(s, &fields))
            .collect();
        DocumentSpec { fields, statutes }
    }

    /// Returns statute indices in dependency order (`REQUIRES` targets first),
    /// falling back to source order on cycles.
    pub fn ordered_indices(&self) -> Vec<usize> {
        ordered_indices(&self.statutes)
    }

    /// Returns `true` when any lowered formula uses a `LIKE` pattern.
    pub fn uses_like(&self) -> bool {
        self.any_formula(|f| matches!(f, Formula::Like { .. }))
    }

    /// Returns `true` when any lowered formula uses a regex match.
    pub fn uses_matches(&self) -> bool {
        self.any_formula(|f| matches!(f, Formula::Matches { .. }))
    }

    fn any_formula(&self, pred: impl Fn(&Formula) -> bool) -> bool {
        let mut found = false;
        let mut visit = |f: &Formula| {
            if pred(f) {
                found = true;
            }
        };
        for s in &self.statutes {
            s.precond.walk(&mut visit);
            for exc in &s.exceptions {
                exc.walk(&mut visit);
            }
        }
        found
    }

    /// Detects pairs of statutes whose effects conflict (same label, same
    /// domain, opposite polarity), returning `(i, j, label)` triples.
    pub fn conflicting_pairs(&self) -> Vec<(usize, usize, String)> {
        let mut out = Vec::new();
        for i in 0..self.statutes.len() {
            for j in (i + 1)..self.statutes.len() {
                if let Some(label) = conflict_label(&self.statutes[i], &self.statutes[j]) {
                    out.push((i, j, label));
                }
            }
        }
        out
    }
}

fn conflict_label(a: &StatuteSpec, b: &StatuteSpec) -> Option<String> {
    for ea in &a.effects {
        for eb in &b.effects {
            if ea.label == eb.label
                && ea.kind.domain().is_some()
                && ea.kind.domain() == eb.kind.domain()
                && ea.kind.polarity() != 0
                && ea.kind.polarity() == -eb.kind.polarity()
            {
                return Some(ea.label.clone());
            }
        }
    }
    None
}

fn lower_statute(statute: &StatuteNode, reg: &FieldRegistry) -> StatuteSpec {
    let precond = statute
        .conditions
        .iter()
        .map(|c| lower_condition(c, reg))
        .fold(Formula::Const(true), Formula::and);

    let exceptions = statute
        .exceptions
        .iter()
        .filter(|exc| !exc.conditions.is_empty())
        .map(|exc| {
            exc.conditions
                .iter()
                .map(|c| lower_condition(c, reg))
                .fold(Formula::Const(true), Formula::and)
        })
        .collect();

    let effects = statute.effects.iter().map(lower_effect).collect::<Vec<_>>();

    StatuteSpec {
        raw_id: statute.id.clone(),
        title: statute.title.clone(),
        precond,
        exceptions,
        requires: statute.requires.clone(),
        effects,
    }
}

fn lower_effect(effect: &EffectNode) -> EffectSpec {
    EffectSpec {
        kind: EffectKind::classify(&effect.effect_type),
        label: clean_label(&effect.description),
    }
}

// ---------------------------------------------------------------------------
// Condition lowering
// ---------------------------------------------------------------------------

fn register_condition(cond: &ConditionNode, reg: &mut FieldRegistry) {
    match cond {
        ConditionNode::Comparison { field, value, .. } => {
            if let Some(scalar) = value_to_scalar(value) {
                reg.add(sanitize(field), scalar.sort());
            }
        }
        ConditionNode::HasAttribute { key } => {
            reg.add(has_field(key), ScalarType::Bool);
        }
        ConditionNode::Between { field, min, .. }
        | ConditionNode::InRange { field, min, .. }
        | ConditionNode::NotInRange { field, min, .. } => {
            let ty = value_to_scalar(min).map_or(ScalarType::Int, |s| s.sort());
            reg.add(sanitize(field), ty);
        }
        ConditionNode::In { field, values } => {
            let ty = values
                .iter()
                .find_map(value_to_scalar)
                .map_or(ScalarType::Int, |s| s.sort());
            reg.add(sanitize(field), ty);
        }
        ConditionNode::Like { field, .. } | ConditionNode::Matches { field, .. } => {
            reg.add(sanitize(field), ScalarType::Str);
        }
        ConditionNode::TemporalComparison { field, .. } => {
            reg.add(temporal_name(field), ScalarType::Date);
        }
        ConditionNode::And(a, b) | ConditionNode::Or(a, b) => {
            register_condition(a, reg);
            register_condition(b, reg);
        }
        ConditionNode::Not(inner) => register_condition(inner, reg),
    }
}

fn lower_condition(cond: &ConditionNode, reg: &FieldRegistry) -> Formula {
    match cond {
        ConditionNode::Comparison {
            field,
            operator,
            value,
        } => {
            let name = sanitize(field);
            let ty = reg.get(&name);
            match value_to_scalar(value) {
                Some(scalar) => {
                    let op = CmpOp::parse(operator);
                    // Ordering a non-numeric value is meaningless in our model;
                    // treat it as vacuously true rather than emitting nonsense.
                    if op.is_ordering()
                        && matches!(scalar.sort(), ScalarType::Str | ScalarType::Bool)
                    {
                        Formula::Const(true)
                    } else {
                        Formula::Compare {
                            field: FieldRef { name, ty },
                            op,
                            value: scalar,
                        }
                    }
                }
                None => Formula::Const(true),
            }
        }
        ConditionNode::HasAttribute { key } => Formula::BoolField(FieldRef {
            name: has_field(key),
            ty: ScalarType::Bool,
        }),
        ConditionNode::Between { field, min, max } => lower_range(field, min, max, true, true, reg),
        ConditionNode::InRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => lower_range(field, min, max, *inclusive_min, *inclusive_max, reg),
        ConditionNode::NotInRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => Formula::Not(Box::new(lower_range(
            field,
            min,
            max,
            *inclusive_min,
            *inclusive_max,
            reg,
        ))),
        ConditionNode::In { field, values } => {
            let name = sanitize(field);
            let ty = reg.get(&name);
            let scalars: Vec<Scalar> = values.iter().filter_map(value_to_scalar).collect();
            if scalars.is_empty() {
                Formula::Const(false)
            } else {
                scalars
                    .into_iter()
                    .map(|scalar| Formula::Compare {
                        field: FieldRef {
                            name: name.clone(),
                            ty,
                        },
                        op: CmpOp::Eq,
                        value: scalar,
                    })
                    .reduce(|acc, item| Formula::Or(Box::new(acc), Box::new(item)))
                    .unwrap_or(Formula::Const(false))
            }
        }
        ConditionNode::Like { field, pattern } => Formula::Like {
            field: FieldRef {
                name: sanitize(field),
                ty: ScalarType::Str,
            },
            pattern: clean_label(pattern),
        },
        ConditionNode::Matches {
            field,
            regex_pattern,
        } => Formula::Matches {
            field: FieldRef {
                name: sanitize(field),
                ty: ScalarType::Str,
            },
            pattern: clean_label(regex_pattern),
        },
        ConditionNode::TemporalComparison {
            field,
            operator,
            value,
        } => {
            let name = temporal_name(field);
            let scalar = value_to_scalar(value).unwrap_or(Scalar::Int(0));
            Formula::Compare {
                field: FieldRef {
                    name,
                    ty: ScalarType::Date,
                },
                op: CmpOp::parse(operator),
                value: scalar,
            }
        }
        ConditionNode::And(a, b) => Formula::And(
            Box::new(lower_condition(a, reg)),
            Box::new(lower_condition(b, reg)),
        ),
        ConditionNode::Or(a, b) => Formula::Or(
            Box::new(lower_condition(a, reg)),
            Box::new(lower_condition(b, reg)),
        ),
        ConditionNode::Not(inner) => Formula::Not(Box::new(lower_condition(inner, reg))),
    }
}

fn lower_range(
    field: &str,
    min: &ConditionValue,
    max: &ConditionValue,
    incl_lo: bool,
    incl_hi: bool,
    reg: &FieldRegistry,
) -> Formula {
    let name = sanitize(field);
    let ty = reg.get(&name);
    let lo = value_to_scalar(min).unwrap_or(Scalar::Int(0));
    let hi = value_to_scalar(max).unwrap_or(Scalar::Int(0));
    Formula::Range {
        field: FieldRef { name, ty },
        lo,
        hi,
        incl_lo,
        incl_hi,
    }
}

fn value_to_scalar(value: &ConditionValue) -> Option<Scalar> {
    match value {
        ConditionValue::Number(n) => Some(Scalar::Int(*n)),
        ConditionValue::String(s) => Some(Scalar::Str(clean_label(s))),
        ConditionValue::Boolean(b) => Some(Scalar::Bool(*b)),
        ConditionValue::Date(d) => Some(Scalar::Date {
            raw: d.clone(),
            as_int: date_to_int(d),
        }),
        ConditionValue::SetExpr(_) => None,
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Name used for the boolean "presence" field backing a `HAS` attribute.
fn has_field(key: &str) -> String {
    format!("has_{}", sanitize(key))
}

fn temporal_name(field: &TemporalField) -> String {
    match field {
        TemporalField::CurrentDate => "current_date".to_string(),
        TemporalField::DateField(name) => sanitize(name),
    }
}

/// Parses a `YYYY-MM-DD` string into a `YYYYMMDD` integer.
pub fn date_to_int(raw: &str) -> Option<i64> {
    let mut parts = raw.split('-');
    let y: i64 = parts.next()?.parse().ok()?;
    let m: i64 = parts.next()?.parse().ok()?;
    let d: i64 = parts.next()?.parse().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    Some(y * 10_000 + m * 100 + d)
}

/// Sanitises arbitrary DSL text into a snake-case-friendly ASCII identifier.
pub fn sanitize(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push_str("field");
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

/// Produces a `snake_case` identifier from arbitrary text.
pub fn snake_ident(raw: &str) -> String {
    sanitize(raw).to_lowercase()
}

/// Produces a `CamelCase` identifier from arbitrary text.
pub fn camel_ident(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut capitalize = true;
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            if capitalize {
                out.push(ch.to_ascii_uppercase());
                capitalize = false;
            } else {
                out.push(ch);
            }
        } else {
            capitalize = true;
        }
    }
    if out.is_empty() {
        out.push_str("Stmt");
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, 'S');
    }
    out
}

/// Strips characters that would break string literals in the target languages.
fn clean_label(raw: &str) -> String {
    raw.chars()
        .map(|c| match c {
            '"' => '\'',
            '\\' => '/',
            '\n' | '\r' | '\t' => ' ',
            other => other,
        })
        .collect()
}

/// Extracts the literal portion of a `LIKE` pattern and how the `%` wildcards
/// are positioned, for backends with real string support (SMT-LIB).
pub(crate) enum LikeShape {
    /// `%inner%` — substring containment.
    Contains(String),
    /// `inner%` — prefix.
    Prefix(String),
    /// `%inner` — suffix.
    Suffix(String),
    /// `inner` — exact equality.
    Exact(String),
}

pub(crate) fn like_shape(pattern: &str) -> LikeShape {
    let starts = pattern.starts_with('%');
    let ends = pattern.ends_with('%');
    let inner = pattern.trim_matches('%').to_string();
    match (starts, ends) {
        (true, true) => LikeShape::Contains(inner),
        (false, true) => LikeShape::Prefix(inner),
        (true, false) => LikeShape::Suffix(inner),
        (false, false) => LikeShape::Exact(inner),
    }
}

/// Dependency-ordering of statutes by their `REQUIRES` edges (Kahn-style DFS).
fn ordered_indices(specs: &[StatuteSpec]) -> Vec<usize> {
    use std::collections::HashMap;
    let index: HashMap<&str, usize> = specs
        .iter()
        .enumerate()
        .map(|(i, s)| (s.raw_id.as_str(), i))
        .collect();
    let mut state = vec![0u8; specs.len()];
    let mut order = Vec::with_capacity(specs.len());
    for i in 0..specs.len() {
        visit_dependency(i, specs, &index, &mut state, &mut order);
    }
    order
}

fn visit_dependency(
    i: usize,
    specs: &[StatuteSpec],
    index: &std::collections::HashMap<&str, usize>,
    state: &mut [u8],
    order: &mut Vec<usize>,
) {
    if state[i] != 0 {
        return;
    }
    state[i] = 1;
    for req in &specs[i].requires {
        if let Some(&j) = index.get(req.as_str()) {
            visit_dependency(j, specs, index, state, order);
        }
    }
    state[i] = 2;
    order.push(i);
}

// ---------------------------------------------------------------------------
// Infix rendering (shared by Coq, Lean, TLA+ and Alloy)
// ---------------------------------------------------------------------------

/// Per-backend syntax used by [`render_formula`] to pretty-print the infix
/// formula tree. SMT-LIB uses prefix notation and does not implement this.
pub(crate) trait InfixSyntax {
    fn and_op(&self) -> &str;
    fn or_op(&self) -> &str;
    fn not_prefix(&self) -> &str;
    fn truth(&self) -> &str;
    fn falsity(&self) -> &str;
    fn compare(&self, field: &FieldRef, op: CmpOp, value: &Scalar) -> String;
    fn bool_field(&self, field: &FieldRef) -> String;
    fn range(
        &self,
        field: &FieldRef,
        lo: &Scalar,
        hi: &Scalar,
        incl_lo: bool,
        incl_hi: bool,
    ) -> String;
    fn like(&self, field: &FieldRef, pattern: &str) -> String;
    fn matches(&self, field: &FieldRef, pattern: &str) -> String;
}

/// Recursively renders a [`Formula`] using an [`InfixSyntax`] dialect.
pub(crate) fn render_formula<S: InfixSyntax>(syntax: &S, formula: &Formula) -> String {
    match formula {
        Formula::Const(true) => syntax.truth().to_string(),
        Formula::Const(false) => syntax.falsity().to_string(),
        Formula::BoolField(field) => syntax.bool_field(field),
        Formula::Compare { field, op, value } => syntax.compare(field, *op, value),
        Formula::Range {
            field,
            lo,
            hi,
            incl_lo,
            incl_hi,
        } => syntax.range(field, lo, hi, *incl_lo, *incl_hi),
        Formula::Like { field, pattern } => syntax.like(field, pattern),
        Formula::Matches { field, pattern } => syntax.matches(field, pattern),
        Formula::Not(inner) => {
            format!("{}({})", syntax.not_prefix(), render_formula(syntax, inner))
        }
        Formula::And(a, b) => format!(
            "({} {} {})",
            render_formula(syntax, a),
            syntax.and_op(),
            render_formula(syntax, b)
        ),
        Formula::Or(a, b) => format!(
            "({} {} {})",
            render_formula(syntax, a),
            syntax.or_op(),
            render_formula(syntax, b)
        ),
    }
}
