//! SMT-LIB 2 export for the OxiZ SMT solver (and any SMT-LIB 2.6 engine).

use super::{
    CmpOp, DocumentSpec, EffectKind, FormalExporter, Formula, LikeShape, Scalar, ScalarType,
    StatuteSpec, like_shape, snake_ident,
};
use crate::DslResult;
use crate::ast::LegalDocument;
use std::collections::HashSet;

/// Exports legal documents to SMT-LIB 2, ready to feed directly to the OxiZ SMT
/// solver.
///
/// Entities are modelled as an algebraic datatype, statutes as
/// `(define-fun applies_<id> ((e Entity)) Bool ...)`, and `LIKE` patterns are
/// translated into native string-theory operations (`str.contains`,
/// `str.prefixof`, `str.suffixof`). Optional `(check-sat)` blocks probe
/// precondition satisfiability and pairwise effect consistency.
#[derive(Debug, Clone)]
pub struct SmtLibExporter {
    /// SMT-LIB logic to declare.
    pub logic: String,
    /// Emit `(check-sat)` proof obligations.
    pub emit_checks: bool,
}

impl Default for SmtLibExporter {
    fn default() -> Self {
        Self {
            logic: "ALL".to_string(),
            emit_checks: true,
        }
    }
}

impl SmtLibExporter {
    /// Creates a new exporter with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the declared SMT-LIB logic.
    pub fn with_logic(mut self, logic: impl Into<String>) -> Self {
        self.logic = logic.into();
        self
    }
}

fn smt_sort(ty: ScalarType) -> &'static str {
    match ty {
        ScalarType::Int | ScalarType::Date => "Int",
        ScalarType::Bool => "Bool",
        ScalarType::Str => "String",
    }
}

fn smt_int(n: i64) -> String {
    if n < 0 {
        format!("(- {})", n.unsigned_abs())
    } else {
        n.to_string()
    }
}

fn smt_lit(value: &Scalar) -> String {
    match value {
        Scalar::Str(s) => format!("\"{s}\""),
        Scalar::Bool(b) => b.to_string(),
        Scalar::Int(n) => smt_int(*n),
        Scalar::Date { as_int, .. } => smt_int(as_int.unwrap_or(0)),
    }
}

fn smt_compare(access: &str, op: CmpOp, value: &Scalar) -> String {
    let lit = smt_lit(value);
    match op {
        CmpOp::Eq => format!("(= {access} {lit})"),
        CmpOp::Ne => format!("(not (= {access} {lit}))"),
        CmpOp::Lt => format!("(< {access} {lit})"),
        CmpOp::Le => format!("(<= {access} {lit})"),
        CmpOp::Gt => format!("(> {access} {lit})"),
        CmpOp::Ge => format!("(>= {access} {lit})"),
    }
}

fn render(formula: &Formula) -> String {
    match formula {
        Formula::Const(b) => b.to_string(),
        Formula::BoolField(field) => format!("({} e)", field.name),
        Formula::Compare { field, op, value } => {
            smt_compare(&format!("({} e)", field.name), *op, value)
        }
        Formula::Range {
            field,
            lo,
            hi,
            incl_lo,
            incl_hi,
        } => {
            let access = format!("({} e)", field.name);
            let lo_op = if *incl_lo { "<=" } else { "<" };
            let hi_op = if *incl_hi { "<=" } else { "<" };
            format!(
                "(and ({} {} {}) ({} {} {}))",
                lo_op,
                smt_lit(lo),
                access,
                hi_op,
                access,
                smt_lit(hi)
            )
        }
        Formula::Like { field, pattern } => {
            let access = format!("({} e)", field.name);
            match like_shape(pattern) {
                LikeShape::Contains(inner) => format!("(str.contains {access} \"{inner}\")"),
                LikeShape::Prefix(inner) => format!("(str.prefixof \"{inner}\" {access})"),
                LikeShape::Suffix(inner) => format!("(str.suffixof \"{inner}\" {access})"),
                LikeShape::Exact(inner) => format!("(= {access} \"{inner}\")"),
            }
        }
        Formula::Matches { field, pattern } => {
            format!("(str_matches ({} e) \"{}\")", field.name, pattern)
        }
        Formula::Not(inner) => format!("(not {})", render(inner)),
        Formula::And(a, b) => format!("(and {} {})", render(a), render(b)),
        Formula::Or(a, b) => format!("(or {} {})", render(a), render(b)),
    }
}

fn effect_comment(effect: &super::EffectSpec) -> String {
    let kind = match &effect.kind {
        EffectKind::Grant => "grant".to_string(),
        EffectKind::Revoke => "revoke".to_string(),
        EffectKind::Obligation => "obligation".to_string(),
        EffectKind::Prohibition => "prohibition".to_string(),
        EffectKind::Custom(k) => k.to_lowercase(),
    };
    format!("; effect: {} \"{}\"", kind, effect.label)
}

fn applies_body(spec: &StatuteSpec, known: &HashSet<&str>) -> String {
    let mut conjuncts = Vec::new();
    if !matches!(spec.precond, Formula::Const(true)) {
        conjuncts.push(render(&spec.precond));
    }
    for req in &spec.requires {
        if known.contains(req.as_str()) {
            conjuncts.push(format!("(applies_{} e)", snake_ident(req)));
        }
    }
    for exc in &spec.exceptions {
        conjuncts.push(format!("(not {})", render(exc)));
    }
    match conjuncts.len() {
        0 => "true".to_string(),
        1 => conjuncts.remove(0),
        _ => format!("(and {})", conjuncts.join(" ")),
    }
}

impl FormalExporter for SmtLibExporter {
    fn export(&self, doc: &LegalDocument) -> DslResult<String> {
        let spec = DocumentSpec::from_document(doc);
        let known: HashSet<&str> = spec.statutes.iter().map(|s| s.raw_id.as_str()).collect();
        let mut out = String::new();

        out.push_str("; Generated by legalis-dsl formal export (SMT-LIB 2 / OxiZ)\n");
        out.push_str(&format!("(set-logic {})\n", self.logic));
        out.push_str("(set-info :smt-lib-version 2.6)\n\n");

        if spec.uses_matches() {
            out.push_str("; uninterpreted regular-expression match predicate\n");
            out.push_str("(declare-fun str_matches (String String) Bool)\n\n");
        }

        // Entity datatype.
        out.push_str("(declare-datatypes ((Entity 0))\n  (((mkEntity");
        if spec.fields.is_empty() {
            out.push_str(" (placeholder Bool)");
        } else {
            for (name, ty) in spec.fields.iter() {
                out.push_str(&format!(" ({} {})", name, smt_sort(*ty)));
            }
        }
        out.push_str("))))\n\n");

        for &idx in &spec.ordered_indices() {
            let statute = &spec.statutes[idx];
            let id = snake_ident(&statute.raw_id);
            out.push_str(&format!("; Statute: {}\n", statute.title));
            out.push_str(&format!(
                "(define-fun applies_{} ((e Entity)) Bool\n  {})\n",
                id,
                applies_body(statute, &known)
            ));
            for effect in &statute.effects {
                out.push_str(&effect_comment(effect));
                out.push('\n');
            }
            out.push('\n');
        }

        if self.emit_checks {
            out.push_str("; --- proof obligations ---\n");
            for statute in &spec.statutes {
                let id = snake_ident(&statute.raw_id);
                out.push_str(&format!("; satisfiability of {id} (expect sat)\n"));
                out.push_str("(push 1)\n");
                out.push_str("(declare-const e Entity)\n");
                out.push_str(&format!("(assert (applies_{id} e))\n"));
                out.push_str("(check-sat)\n");
                out.push_str("(pop 1)\n");
            }
            for (i, j, label) in spec.conflicting_pairs() {
                let a = snake_ident(&spec.statutes[i].raw_id);
                let b = snake_ident(&spec.statutes[j].raw_id);
                out.push_str(&format!(
                    "; consistency for conflicting effect \"{label}\" (expect unsat)\n"
                ));
                out.push_str("(push 1)\n");
                out.push_str("(declare-const e Entity)\n");
                out.push_str(&format!("(assert (applies_{a} e))\n"));
                out.push_str(&format!("(assert (applies_{b} e))\n"));
                out.push_str("(check-sat)\n");
                out.push_str("(pop 1)\n");
            }
        }

        Ok(out)
    }

    fn target(&self) -> &str {
        "SMT-LIB"
    }

    fn file_extension(&self) -> &str {
        "smt2"
    }
}
