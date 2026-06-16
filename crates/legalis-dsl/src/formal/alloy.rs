//! Alloy export for constraint analysis.

use super::{
    CmpOp, DocumentSpec, EffectKind, FieldRef, FormalExporter, Formula, InfixSyntax, Scalar,
    ScalarType, StatuteSpec, camel_ident, render_formula,
};
use crate::DslResult;
use crate::ast::LegalDocument;
use std::collections::HashSet;

/// Exports legal documents to an Alloy model for constraint analysis.
///
/// Fields become a `sig Entity`, statutes become `pred applies<Id>[e : Entity]`,
/// and (when enabled) `run`/`check` commands let the Alloy Analyzer search for
/// applicable entities and counterexamples to effect consistency.
#[derive(Debug, Clone)]
pub struct AlloyExporter {
    /// Generated Alloy module name.
    pub module_name: String,
    /// Default analysis scope.
    pub scope: u32,
    /// Emit `run`/`check` analysis commands.
    pub emit_commands: bool,
}

impl Default for AlloyExporter {
    fn default() -> Self {
        Self {
            module_name: "legalis".to_string(),
            scope: 4,
            emit_commands: true,
        }
    }
}

impl AlloyExporter {
    /// Creates a new exporter with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the analysis scope.
    pub fn with_scope(mut self, scope: u32) -> Self {
        self.scope = scope;
        self
    }
}

struct AlloySyntax;

impl AlloySyntax {
    fn op(op: CmpOp) -> &'static str {
        match op {
            CmpOp::Eq => "=",
            CmpOp::Ne => "!=",
            CmpOp::Lt => "<",
            CmpOp::Le => "=<",
            CmpOp::Gt => ">",
            CmpOp::Ge => ">=",
        }
    }

    fn lit(value: &Scalar) -> String {
        match value {
            Scalar::Str(s) => format!("\"{s}\""),
            Scalar::Bool(b) => {
                if *b {
                    "True".to_string()
                } else {
                    "False".to_string()
                }
            }
            other => other.as_int().to_string(),
        }
    }
}

impl InfixSyntax for AlloySyntax {
    fn and_op(&self) -> &str {
        "and"
    }
    fn or_op(&self) -> &str {
        "or"
    }
    fn not_prefix(&self) -> &str {
        "not "
    }
    fn truth(&self) -> &str {
        "no none"
    }
    fn falsity(&self) -> &str {
        "some none"
    }

    fn compare(&self, field: &FieldRef, op: CmpOp, value: &Scalar) -> String {
        let access = format!("e.{}", field.name);
        match field.ty {
            ScalarType::Str | ScalarType::Bool => {
                let alloy_op = if op == CmpOp::Ne { "!=" } else { "=" };
                format!("({} {} {})", access, alloy_op, Self::lit(value))
            }
            _ => format!("({} {} {})", access, Self::op(op), Self::lit(value)),
        }
    }

    fn bool_field(&self, field: &FieldRef) -> String {
        format!("(e.{} = True)", field.name)
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
        let lo_op = if incl_lo { "=<" } else { "<" };
        let hi_op = if incl_hi { "=<" } else { "<" };
        format!(
            "(({} {} {}) and ({} {} {}))",
            Self::lit(lo),
            lo_op,
            access,
            access,
            hi_op,
            Self::lit(hi)
        )
    }

    fn like(&self, field: &FieldRef, pattern: &str) -> String {
        format!(
            "(e -> \"{}:{}\" in Abstract.satisfiesLike)",
            field.name, pattern
        )
    }

    fn matches(&self, field: &FieldRef, pattern: &str) -> String {
        format!(
            "(e -> \"{}:{}\" in Abstract.satisfiesMatch)",
            field.name, pattern
        )
    }
}

fn alloy_sort(ty: ScalarType) -> &'static str {
    match ty {
        ScalarType::Int | ScalarType::Date => "Int",
        ScalarType::Bool => "Bool",
        ScalarType::Str => "String",
    }
}

fn pred_name(raw_id: &str) -> String {
    format!("applies{}", camel_ident(raw_id))
}

fn effect_comment(effect: &super::EffectSpec) -> String {
    let kind = match &effect.kind {
        EffectKind::Grant => "grant".to_string(),
        EffectKind::Revoke => "revoke".to_string(),
        EffectKind::Obligation => "obligation".to_string(),
        EffectKind::Prohibition => "prohibition".to_string(),
        EffectKind::Custom(k) => k.to_lowercase(),
    };
    format!("  // effect: {} \"{}\"", kind, effect.label)
}

fn applies_body(spec: &StatuteSpec, known: &HashSet<&str>) -> String {
    let syntax = AlloySyntax;
    let mut conjuncts = Vec::new();
    if !matches!(spec.precond, Formula::Const(true)) {
        conjuncts.push(render_formula(&syntax, &spec.precond));
    }
    for req in &spec.requires {
        if known.contains(req.as_str()) {
            conjuncts.push(format!("{}[e]", pred_name(req)));
        }
    }
    for exc in &spec.exceptions {
        conjuncts.push(format!("not ({})", render_formula(&syntax, exc)));
    }
    if conjuncts.is_empty() {
        "no none".to_string()
    } else {
        conjuncts.join(" and ")
    }
}

impl FormalExporter for AlloyExporter {
    fn export(&self, doc: &LegalDocument) -> DslResult<String> {
        let spec = DocumentSpec::from_document(doc);
        let known: HashSet<&str> = spec.statutes.iter().map(|s| s.raw_id.as_str()).collect();
        let scope = self.scope;
        let mut out = String::new();

        out.push_str("// Generated by legalis-dsl formal export (Alloy 6)\n");
        out.push_str(&format!("module {}\n\n", self.module_name));

        if spec.fields.has_bool() {
            out.push_str("enum Bool { True, False }\n\n");
        }
        if spec.uses_like() || spec.uses_matches() {
            out.push_str("one sig Abstract {\n");
            out.push_str("  satisfiesLike : Entity -> String,\n");
            out.push_str("  satisfiesMatch : Entity -> String\n");
            out.push_str("}\n\n");
        }

        out.push_str("sig Entity {\n");
        if spec.fields.is_empty() {
            out.push_str("  placeholder : one Bool\n");
        } else {
            let parts: Vec<String> = spec
                .fields
                .iter()
                .map(|(name, ty)| format!("  {} : one {}", name, alloy_sort(*ty)))
                .collect();
            out.push_str(&parts.join(",\n"));
            out.push('\n');
        }
        out.push_str("}\n\n");

        for &idx in &spec.ordered_indices() {
            let statute = &spec.statutes[idx];
            out.push_str(&format!("// Statute: {}\n", statute.title));
            out.push_str(&format!(
                "pred {}[e : Entity] {{\n  {}\n}}\n",
                pred_name(&statute.raw_id),
                applies_body(statute, &known)
            ));
            for effect in &statute.effects {
                out.push_str(&effect_comment(effect));
                out.push('\n');
            }
            out.push('\n');
        }

        if self.emit_commands {
            out.push_str("// Analysis commands\n");
            for statute in &spec.statutes {
                out.push_str(&format!(
                    "run {} for {} but 8 Int\n",
                    pred_name(&statute.raw_id),
                    scope
                ));
            }
            for (i, j, label) in spec.conflicting_pairs() {
                let a = pred_name(&spec.statutes[i].raw_id);
                let b = pred_name(&spec.statutes[j].raw_id);
                let assertion = format!(
                    "consistent{}{}",
                    camel_ident(&spec.statutes[i].raw_id),
                    camel_ident(&spec.statutes[j].raw_id)
                );
                out.push_str(&format!(
                    "// Consistency for conflicting effect \"{label}\"\n"
                ));
                out.push_str(&format!(
                    "assert {assertion} {{ no e : Entity | {a}[e] and {b}[e] }}\n"
                ));
                out.push_str(&format!("check {assertion} for {scope} but 8 Int\n"));
            }
        }

        Ok(out)
    }

    fn target(&self) -> &str {
        "Alloy"
    }

    fn file_extension(&self) -> &str {
        "als"
    }
}
