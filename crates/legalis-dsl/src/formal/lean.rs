//! Lean 4 export for theorem proving.

use super::{
    CmpOp, DocumentSpec, EffectKind, FieldRef, FormalExporter, Formula, InfixSyntax, Scalar,
    ScalarType, StatuteSpec, render_formula, snake_ident,
};
use crate::DslResult;
use crate::ast::LegalDocument;
use std::collections::HashSet;

/// Exports legal documents to Lean 4.
///
/// Statutes become `def applies_<id> (e : Entity) : Prop`; effects a
/// `List LegalEffect`; proof obligations (when enabled) are `theorem`s closed
/// with `sorry` for the user to complete.
#[derive(Debug, Clone)]
pub struct Lean4Exporter {
    /// Namespace wrapping the generated declarations.
    pub namespace: String,
    /// Emit satisfiability/consistency proof obligations as `sorry` theorems.
    pub emit_obligations: bool,
}

impl Default for Lean4Exporter {
    fn default() -> Self {
        Self {
            namespace: "Legalis".to_string(),
            emit_obligations: true,
        }
    }
}

impl Lean4Exporter {
    /// Creates a new exporter with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the wrapping namespace.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = namespace.into();
        self
    }
}

struct LeanSyntax;

impl LeanSyntax {
    fn op(op: CmpOp) -> &'static str {
        match op {
            CmpOp::Eq => "=",
            CmpOp::Ne => "≠",
            CmpOp::Lt => "<",
            CmpOp::Le => "≤",
            CmpOp::Gt => ">",
            CmpOp::Ge => "≥",
        }
    }

    fn lit(value: &Scalar) -> String {
        match value {
            Scalar::Str(s) => format!("\"{s}\""),
            Scalar::Bool(b) => b.to_string(),
            other => other.as_int().to_string(),
        }
    }
}

impl InfixSyntax for LeanSyntax {
    fn and_op(&self) -> &str {
        "∧"
    }
    fn or_op(&self) -> &str {
        "∨"
    }
    fn not_prefix(&self) -> &str {
        "¬"
    }
    fn truth(&self) -> &str {
        "True"
    }
    fn falsity(&self) -> &str {
        "False"
    }

    fn compare(&self, field: &FieldRef, op: CmpOp, value: &Scalar) -> String {
        let access = format!("e.{}", field.name);
        match field.ty {
            ScalarType::Str | ScalarType::Bool => {
                let lean_op = if op == CmpOp::Ne { "≠" } else { "=" };
                format!("({} {} {})", access, lean_op, Self::lit(value))
            }
            _ => format!("({} {} {})", access, Self::op(op), Self::lit(value)),
        }
    }

    fn bool_field(&self, field: &FieldRef) -> String {
        format!("(e.{} = true)", field.name)
    }

    fn range(
        &self,
        field: &FieldRef,
        lo: &Scalar,
        hi: &Scalar,
        incl_lo: bool,
        incl_hi: bool,
    ) -> String {
        let access = format!("e.{}", field.name);
        let lo_op = if incl_lo { "≤" } else { "<" };
        let hi_op = if incl_hi { "≤" } else { "<" };
        format!(
            "(({} {} {}) ∧ ({} {} {}))",
            Self::lit(lo),
            lo_op,
            access,
            access,
            hi_op,
            Self::lit(hi)
        )
    }

    fn like(&self, field: &FieldRef, pattern: &str) -> String {
        format!("(stringLike e.{} \"{}\")", field.name, pattern)
    }

    fn matches(&self, field: &FieldRef, pattern: &str) -> String {
        format!("(stringMatches e.{} \"{}\")", field.name, pattern)
    }
}

fn lean_sort(ty: ScalarType) -> &'static str {
    match ty {
        ScalarType::Int | ScalarType::Date => "Int",
        ScalarType::Bool => "Bool",
        ScalarType::Str => "String",
    }
}

fn effect_term(effect: &super::EffectSpec) -> String {
    match &effect.kind {
        EffectKind::Grant => format!("LegalEffect.grant \"{}\"", effect.label),
        EffectKind::Revoke => format!("LegalEffect.revoke \"{}\"", effect.label),
        EffectKind::Obligation => format!("LegalEffect.obligation \"{}\"", effect.label),
        EffectKind::Prohibition => format!("LegalEffect.prohibition \"{}\"", effect.label),
        EffectKind::Custom(kind) => {
            format!("LegalEffect.custom \"{}\" \"{}\"", kind, effect.label)
        }
    }
}

fn applies_body(spec: &StatuteSpec, known: &HashSet<&str>) -> String {
    let syntax = LeanSyntax;
    let mut conjuncts = Vec::new();
    if !matches!(spec.precond, Formula::Const(true)) {
        conjuncts.push(render_formula(&syntax, &spec.precond));
    }
    for req in &spec.requires {
        if known.contains(req.as_str()) {
            conjuncts.push(format!("(applies_{} e)", snake_ident(req)));
        }
    }
    for exc in &spec.exceptions {
        conjuncts.push(format!("(¬({}))", render_formula(&syntax, exc)));
    }
    if conjuncts.is_empty() {
        "True".to_string()
    } else {
        conjuncts.join(" ∧ ")
    }
}

impl FormalExporter for Lean4Exporter {
    fn export(&self, doc: &LegalDocument) -> DslResult<String> {
        let spec = DocumentSpec::from_document(doc);
        let known: HashSet<&str> = spec.statutes.iter().map(|s| s.raw_id.as_str()).collect();
        let mut out = String::new();

        out.push_str("-- Generated by legalis-dsl formal export (Lean 4)\n");
        out.push_str(&format!("namespace {}\n\n", self.namespace));

        if spec.uses_like() {
            out.push_str("axiom stringLike : String → String → Prop\n");
        }
        if spec.uses_matches() {
            out.push_str("axiom stringMatches : String → String → Prop\n");
        }
        if spec.uses_like() || spec.uses_matches() {
            out.push('\n');
        }

        out.push_str("structure Entity where\n");
        if spec.fields.is_empty() {
            out.push_str("  placeholder : Bool\n");
        } else {
            for (name, ty) in spec.fields.iter() {
                out.push_str(&format!("  {} : {}\n", name, lean_sort(*ty)));
            }
        }
        out.push('\n');

        out.push_str("inductive LegalEffect where\n");
        out.push_str("  | grant (label : String)\n");
        out.push_str("  | revoke (label : String)\n");
        out.push_str("  | obligation (label : String)\n");
        out.push_str("  | prohibition (label : String)\n");
        out.push_str("  | custom (kind : String) (label : String)\n\n");

        for &idx in &spec.ordered_indices() {
            let statute = &spec.statutes[idx];
            let id = snake_ident(&statute.raw_id);
            out.push_str(&format!("-- Statute: {}\n", statute.title));
            out.push_str(&format!(
                "def applies_{} (e : Entity) : Prop :=\n  {}\n",
                id,
                applies_body(statute, &known)
            ));
            let effects: Vec<String> = statute.effects.iter().map(effect_term).collect();
            if effects.is_empty() {
                out.push_str(&format!(
                    "def effects_{id} : List LegalEffect := ([] : List LegalEffect)\n\n"
                ));
            } else {
                out.push_str(&format!(
                    "def effects_{} : List LegalEffect := [{}]\n\n",
                    id,
                    effects.join(", ")
                ));
            }
        }

        if self.emit_obligations {
            out.push_str("-- Proof obligations (complete the `sorry` placeholders)\n");
            for statute in &spec.statutes {
                let id = snake_ident(&statute.raw_id);
                out.push_str(&format!(
                    "theorem {id}_satisfiable : ∃ e : Entity, applies_{id} e := by\n  sorry\n"
                ));
            }
            for (i, j, label) in spec.conflicting_pairs() {
                let a = snake_ident(&spec.statutes[i].raw_id);
                let b = snake_ident(&spec.statutes[j].raw_id);
                out.push_str(&format!(
                    "-- Consistency for conflicting effect \"{label}\"\n"
                ));
                out.push_str(&format!(
                    "theorem consistent_{a}_{b} : ¬ (∃ e : Entity, applies_{a} e ∧ applies_{b} e) := by\n  sorry\n"
                ));
            }
        }

        out.push_str(&format!("\nend {}\n", self.namespace));
        Ok(out)
    }

    fn target(&self) -> &str {
        "Lean4"
    }

    fn file_extension(&self) -> &str {
        "lean"
    }
}
