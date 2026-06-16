//! TLA+ export for model checking.

use super::{
    CmpOp, DocumentSpec, EffectKind, FieldRef, FormalExporter, Formula, InfixSyntax, Scalar,
    ScalarType, StatuteSpec, camel_ident, render_formula,
};
use crate::DslResult;
use crate::ast::LegalDocument;
use std::collections::HashSet;

/// Exports legal documents to a TLA+ module for model checking.
///
/// The entity record becomes a TLA+ record set `Entity`, each statute an
/// operator `Applies<Id>(e)`, and (when enabled) satisfiability/consistency
/// `THEOREM`s are emitted over `Entity`.
#[derive(Debug, Clone)]
pub struct TlaExporter {
    /// Name of the generated TLA+ module.
    pub module_name: String,
    /// Emit satisfiability/consistency theorems.
    pub emit_theorems: bool,
}

impl Default for TlaExporter {
    fn default() -> Self {
        Self {
            module_name: "LegalisSpec".to_string(),
            emit_theorems: true,
        }
    }
}

impl TlaExporter {
    /// Creates a new exporter with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the generated module name.
    pub fn with_module_name(mut self, name: impl Into<String>) -> Self {
        self.module_name = name.into();
        self
    }
}

struct TlaSyntax;

impl TlaSyntax {
    fn op(op: CmpOp) -> &'static str {
        match op {
            CmpOp::Eq => "=",
            CmpOp::Ne => "#",
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Gt => ">",
            CmpOp::Ge => ">=",
        }
    }

    fn lit(value: &Scalar) -> String {
        match value {
            Scalar::Str(s) => format!("\"{s}\""),
            Scalar::Bool(b) => {
                if *b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
            other => other.as_int().to_string(),
        }
    }
}

impl InfixSyntax for TlaSyntax {
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
        "TRUE"
    }
    fn falsity(&self) -> &str {
        "FALSE"
    }

    fn compare(&self, field: &FieldRef, op: CmpOp, value: &Scalar) -> String {
        let access = format!("e.{}", field.name);
        match field.ty {
            ScalarType::Str | ScalarType::Bool => {
                let tla_op = if op == CmpOp::Ne { "#" } else { "=" };
                format!("({} {} {})", access, tla_op, Self::lit(value))
            }
            _ => format!("({} {} {})", access, Self::op(op), Self::lit(value)),
        }
    }

    fn bool_field(&self, field: &FieldRef) -> String {
        format!("(e.{} = TRUE)", field.name)
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
        let lo_op = if incl_lo { "<=" } else { "<" };
        let hi_op = if incl_hi { "<=" } else { "<" };
        format!(
            "(({} {} {}) /\\ ({} {} {}))",
            Self::lit(lo),
            lo_op,
            access,
            access,
            hi_op,
            Self::lit(hi)
        )
    }

    fn like(&self, field: &FieldRef, pattern: &str) -> String {
        format!("StringLike(e.{}, \"{}\")", field.name, pattern)
    }

    fn matches(&self, field: &FieldRef, pattern: &str) -> String {
        format!("StringMatches(e.{}, \"{}\")", field.name, pattern)
    }
}

fn tla_sort(ty: ScalarType) -> &'static str {
    match ty {
        ScalarType::Int | ScalarType::Date => "Int",
        ScalarType::Bool => "BOOLEAN",
        ScalarType::Str => "STRING",
    }
}

fn effect_string(effect: &super::EffectSpec) -> String {
    let kind = match &effect.kind {
        EffectKind::Grant => "grant".to_string(),
        EffectKind::Revoke => "revoke".to_string(),
        EffectKind::Obligation => "obligation".to_string(),
        EffectKind::Prohibition => "prohibition".to_string(),
        EffectKind::Custom(k) => k.to_lowercase(),
    };
    format!("\"{}: {}\"", kind, effect.label)
}

fn applies_body(spec: &StatuteSpec, known: &HashSet<&str>) -> String {
    let syntax = TlaSyntax;
    let mut conjuncts = Vec::new();
    if !matches!(spec.precond, Formula::Const(true)) {
        conjuncts.push(render_formula(&syntax, &spec.precond));
    }
    for req in &spec.requires {
        if known.contains(req.as_str()) {
            conjuncts.push(format!("Applies{}(e)", camel_ident(req)));
        }
    }
    for exc in &spec.exceptions {
        conjuncts.push(format!("(~({}))", render_formula(&syntax, exc)));
    }
    if conjuncts.is_empty() {
        "TRUE".to_string()
    } else {
        conjuncts.join(" /\\ ")
    }
}

impl FormalExporter for TlaExporter {
    fn export(&self, doc: &LegalDocument) -> DslResult<String> {
        let spec = DocumentSpec::from_document(doc);
        let known: HashSet<&str> = spec.statutes.iter().map(|s| s.raw_id.as_str()).collect();
        let mut out = String::new();

        let dashes = "----------------------------";
        out.push_str(&format!(
            "{} MODULE {} {}\n",
            dashes, self.module_name, dashes
        ));
        out.push_str("EXTENDS Integers, Sequences, FiniteSets, TLC\n\n");

        if spec.uses_like() {
            out.push_str("CONSTANT StringLike(_, _)\n");
        }
        if spec.uses_matches() {
            out.push_str("CONSTANT StringMatches(_, _)\n");
        }
        if spec.uses_like() || spec.uses_matches() {
            out.push('\n');
        }

        // Entity record set.
        if spec.fields.is_empty() {
            out.push_str("Entity == [ placeholder : BOOLEAN ]\n\n");
        } else {
            let parts: Vec<String> = spec
                .fields
                .iter()
                .map(|(name, ty)| format!("{} : {}", name, tla_sort(*ty)))
                .collect();
            out.push_str(&format!("Entity == [ {} ]\n\n", parts.join(", ")));
        }

        for &idx in &spec.ordered_indices() {
            let statute = &spec.statutes[idx];
            let name = camel_ident(&statute.raw_id);
            out.push_str(&format!("(* Statute: {} *)\n", statute.title));
            out.push_str(&format!(
                "Applies{}(e) ==\n    {}\n",
                name,
                applies_body(statute, &known)
            ));
            let effects: Vec<String> = statute.effects.iter().map(effect_string).collect();
            out.push_str(&format!(
                "Effects{} == << {} >>\n\n",
                name,
                effects.join(", ")
            ));
        }

        if self.emit_theorems {
            out.push_str("(* Proof obligations *)\n");
            for statute in &spec.statutes {
                let name = camel_ident(&statute.raw_id);
                out.push_str(&format!(
                    "THEOREM {name}Sat == \\E e \\in Entity : Applies{name}(e)\n"
                ));
            }
            for (i, j, label) in spec.conflicting_pairs() {
                let a = camel_ident(&spec.statutes[i].raw_id);
                let b = camel_ident(&spec.statutes[j].raw_id);
                out.push_str(&format!(
                    "(* Consistency for conflicting effect \"{label}\" *)\n"
                ));
                out.push_str(&format!(
                    "THEOREM Consistent{a}{b} == ~ (\\E e \\in Entity : Applies{a}(e) /\\ Applies{b}(e))\n"
                ));
            }
            out.push('\n');
        }

        out.push_str(
            "=============================================================================\n",
        );
        Ok(out)
    }

    fn target(&self) -> &str {
        "TLA+"
    }

    fn file_extension(&self) -> &str {
        "tla"
    }
}
