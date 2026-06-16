//! Coq (Gallina) export for proof assistants.

use super::{
    CmpOp, DocumentSpec, EffectKind, FieldRef, FormalExporter, Formula, InfixSyntax, Scalar,
    ScalarType, StatuteSpec, render_formula, snake_ident,
};
use crate::DslResult;
use crate::ast::LegalDocument;
use std::collections::HashSet;

/// Exports legal documents to Coq for interactive theorem proving.
///
/// Each statute becomes a `Definition applies_<id> (e : Entity) : Prop`, its
/// effects a `list LegalEffect`, and (optionally) proof obligations are emitted
/// as `Conjecture`s: precondition satisfiability and pairwise effect
/// consistency.
#[derive(Debug, Clone)]
pub struct CoqExporter {
    /// Emit satisfiability/consistency proof obligations.
    pub emit_obligations: bool,
}

impl Default for CoqExporter {
    fn default() -> Self {
        Self {
            emit_obligations: true,
        }
    }
}

impl CoqExporter {
    /// Creates a new exporter with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Disables emission of proof obligations.
    pub fn without_obligations(mut self) -> Self {
        self.emit_obligations = false;
        self
    }
}

struct CoqSyntax;

impl CoqSyntax {
    fn num_op(op: CmpOp) -> &'static str {
        match op {
            CmpOp::Eq => "=",
            CmpOp::Ne => "<>",
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Gt => ">",
            CmpOp::Ge => ">=",
        }
    }

    fn lit(value: &Scalar) -> String {
        match value {
            Scalar::Str(s) => format!("\"{s}\"%string"),
            Scalar::Bool(b) => b.to_string(),
            other => other.as_int().to_string(),
        }
    }
}

impl InfixSyntax for CoqSyntax {
    fn and_op(&self) -> &str {
        "/\\"
    }
    fn or_op(&self) -> &str {
        "\\/"
    }
    fn not_prefix(&self) -> &str {
        "~"
    }
    fn truth(&self) -> &str {
        "True"
    }
    fn falsity(&self) -> &str {
        "False"
    }

    fn compare(&self, field: &FieldRef, op: CmpOp, value: &Scalar) -> String {
        let access = format!("({} e)", field.name);
        match field.ty {
            ScalarType::Str => {
                let coq_op = if op == CmpOp::Ne { "<>" } else { "=" };
                format!("({} {} {})", access, coq_op, Self::lit(value))
            }
            ScalarType::Bool => {
                let coq_op = if op == CmpOp::Ne { "<>" } else { "=" };
                format!("({} {} {})", access, coq_op, Self::lit(value))
            }
            _ => format!("(({} {} {})%Z)", access, Self::num_op(op), Self::lit(value)),
        }
    }

    fn bool_field(&self, field: &FieldRef) -> String {
        format!("(({} e) = true)", field.name)
    }

    fn range(
        &self,
        field: &FieldRef,
        lo: &Scalar,
        hi: &Scalar,
        incl_lo: bool,
        incl_hi: bool,
    ) -> String {
        let access = format!("({} e)", field.name);
        let lo_op = if incl_lo { "<=" } else { "<" };
        let hi_op = if incl_hi { "<=" } else { "<" };
        format!(
            "((({} {} {})%Z) /\\ (({} {} {})%Z))",
            Self::lit(lo),
            lo_op,
            access,
            access,
            hi_op,
            Self::lit(hi)
        )
    }

    fn like(&self, field: &FieldRef, pattern: &str) -> String {
        format!("(string_like ({} e) \"{}\"%string)", field.name, pattern)
    }

    fn matches(&self, field: &FieldRef, pattern: &str) -> String {
        format!("(string_matches ({} e) \"{}\"%string)", field.name, pattern)
    }
}

fn coq_sort(ty: ScalarType) -> &'static str {
    match ty {
        ScalarType::Int | ScalarType::Date => "Z",
        ScalarType::Bool => "bool",
        ScalarType::Str => "string",
    }
}

fn effect_term(effect: &super::EffectSpec) -> String {
    match &effect.kind {
        EffectKind::Grant => format!("EffGrant \"{}\"%string", effect.label),
        EffectKind::Revoke => format!("EffRevoke \"{}\"%string", effect.label),
        EffectKind::Obligation => format!("EffObligation \"{}\"%string", effect.label),
        EffectKind::Prohibition => format!("EffProhibition \"{}\"%string", effect.label),
        EffectKind::Custom(kind) => {
            format!("EffCustom \"{}\"%string \"{}\"%string", kind, effect.label)
        }
    }
}

fn applies_body(spec: &StatuteSpec, known: &HashSet<&str>) -> String {
    let syntax = CoqSyntax;
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
        conjuncts.push(format!("(~({}))", render_formula(&syntax, exc)));
    }
    if conjuncts.is_empty() {
        "True".to_string()
    } else {
        conjuncts.join(" /\\ ")
    }
}

impl FormalExporter for CoqExporter {
    fn export(&self, doc: &LegalDocument) -> DslResult<String> {
        let spec = DocumentSpec::from_document(doc);
        let known: HashSet<&str> = spec.statutes.iter().map(|s| s.raw_id.as_str()).collect();
        let mut out = String::new();

        out.push_str("(* Generated by legalis-dsl formal export (Coq) *)\n");
        out.push_str("Require Import Coq.ZArith.ZArith.\n");
        out.push_str("Require Import Coq.Strings.String.\n");
        out.push_str("Require Import Coq.Lists.List.\n");
        out.push_str("Import ListNotations.\n");
        out.push_str("Open Scope Z_scope.\n\n");

        if spec.uses_like() {
            out.push_str("Parameter string_like : string -> string -> Prop.\n");
        }
        if spec.uses_matches() {
            out.push_str("Parameter string_matches : string -> string -> Prop.\n");
        }
        if spec.uses_like() || spec.uses_matches() {
            out.push('\n');
        }

        // Entity record.
        out.push_str("Record Entity : Type := mkEntity {\n");
        let field_lines: Vec<String> = spec
            .fields
            .iter()
            .map(|(name, ty)| format!("  {} : {}", name, coq_sort(*ty)))
            .collect();
        if field_lines.is_empty() {
            out.push_str("  _placeholder : bool\n");
        } else {
            out.push_str(&field_lines.join(";\n"));
            out.push('\n');
        }
        out.push_str("}.\n\n");

        // Effect datatype.
        out.push_str("Inductive LegalEffect : Type :=\n");
        out.push_str("  | EffGrant : string -> LegalEffect\n");
        out.push_str("  | EffRevoke : string -> LegalEffect\n");
        out.push_str("  | EffObligation : string -> LegalEffect\n");
        out.push_str("  | EffProhibition : string -> LegalEffect\n");
        out.push_str("  | EffCustom : string -> string -> LegalEffect.\n\n");

        for &idx in &spec.ordered_indices() {
            let statute = &spec.statutes[idx];
            let id = snake_ident(&statute.raw_id);
            out.push_str(&format!("(* Statute: {} *)\n", statute.title));
            out.push_str(&format!(
                "Definition applies_{} (e : Entity) : Prop :=\n  {}.\n",
                id,
                applies_body(statute, &known)
            ));

            let effects: Vec<String> = statute.effects.iter().map(effect_term).collect();
            if effects.is_empty() {
                out.push_str(&format!(
                    "Definition effects_{id} : list LegalEffect := nil.\n\n"
                ));
            } else {
                out.push_str(&format!(
                    "Definition effects_{} : list LegalEffect := [{}].\n\n",
                    id,
                    effects.join("; ")
                ));
            }
        }

        if self.emit_obligations {
            out.push_str("(* Proof obligations *)\n");
            for statute in &spec.statutes {
                let id = snake_ident(&statute.raw_id);
                out.push_str(&format!(
                    "Conjecture {id}_satisfiable : exists e : Entity, applies_{id} e.\n"
                ));
            }
            for (i, j, label) in spec.conflicting_pairs() {
                let a = snake_ident(&spec.statutes[i].raw_id);
                let b = snake_ident(&spec.statutes[j].raw_id);
                out.push_str(&format!(
                    "(* Consistency for conflicting effect \"{label}\" *)\n"
                ));
                out.push_str(&format!(
                    "Conjecture consistent_{a}_{b} : ~ (exists e : Entity, applies_{a} e /\\ applies_{b} e).\n"
                ));
            }
        }

        Ok(out)
    }

    fn target(&self) -> &str {
        "Coq"
    }

    fn file_extension(&self) -> &str {
        "v"
    }
}
