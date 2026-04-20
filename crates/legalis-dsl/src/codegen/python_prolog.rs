//! Python and Prolog code generators for the Legalis DSL.

use super::CodeGenerator;
use crate::DslResult;
use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use std::fmt::Write;

pub struct PythonGenerator {
    /// Use type hints
    pub use_type_hints: bool,
    /// Generate docstrings
    pub generate_docstrings: bool,
}

impl Default for PythonGenerator {
    fn default() -> Self {
        Self {
            use_type_hints: true,
            generate_docstrings: true,
        }
    }
}

impl PythonGenerator {
    /// Creates a new Python generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a Python function name from statute ID.
    fn function_name(&self, id: &str) -> String {
        id.replace('-', "_").to_lowercase()
    }

    /// Generates Python condition expression.
    #[allow(clippy::only_used_in_recursion)]
    fn generate_condition(&self, condition: &ConditionNode, indent: usize) -> DslResult<String> {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                let py_op = match operator.as_str() {
                    "=" | "==" => "==",
                    op => op,
                };
                let val = self.format_value(value)?;
                Ok(format!("{}.{} {} {}", "obj", field, py_op, val))
            }
            ConditionNode::HasAttribute { key } => Ok(format!(
                "hasattr(obj, '{}') and obj.{} is not None",
                key, key
            )),
            ConditionNode::Between { field, min, max } => {
                let min_val = self.format_value(min)?;
                let max_val = self.format_value(max)?;
                Ok(format!("{} <= obj.{} <= {}", min_val, field, max_val))
            }
            ConditionNode::In { field, values } => {
                let vals: Result<Vec<_>, _> = values.iter().map(|v| self.format_value(v)).collect();
                let vals = vals?;
                Ok(format!("obj.{} in [{}]", field, vals.join(", ")))
            }
            ConditionNode::And(left, right) => {
                let left_py = self.generate_condition(left, indent)?;
                let right_py = self.generate_condition(right, indent)?;
                Ok(format!("({} and {})", left_py, right_py))
            }
            ConditionNode::Or(left, right) => {
                let left_py = self.generate_condition(left, indent)?;
                let right_py = self.generate_condition(right, indent)?;
                Ok(format!("({} or {})", left_py, right_py))
            }
            ConditionNode::Not(inner) => {
                let inner_py = self.generate_condition(inner, indent)?;
                Ok(format!("not ({})", inner_py))
            }
            _ => Ok("True".to_string()),
        }
    }

    /// Formats a condition value for Python.
    fn format_value(&self, value: &ConditionValue) -> DslResult<String> {
        match value {
            ConditionValue::Number(n) => Ok(n.to_string()),
            ConditionValue::String(s) => Ok(format!("'{}'", s.replace('\'', "\\'"))),
            ConditionValue::Boolean(b) => Ok(if *b {
                "True".to_string()
            } else {
                "False".to_string()
            }),
            ConditionValue::Date(d) => Ok(format!("'{}'", d)),
            ConditionValue::SetExpr(_) => Ok("None".to_string()), // Set expressions not directly supported
        }
    }

    /// Generates a Python function for a statute.
    fn generate_function(&self, statute: &StatuteNode) -> DslResult<String> {
        let mut py = String::new();
        let func_name = self.function_name(&statute.id);

        // Function signature
        if self.use_type_hints {
            writeln!(&mut py, "def {}(obj: Any) -> bool:", func_name)
                .expect("writing to String is infallible");
        } else {
            writeln!(&mut py, "def {}(obj):", func_name).expect("writing to String is infallible");
        }

        // Docstring
        if self.generate_docstrings {
            writeln!(&mut py, "    \"\"\"{}\"\"\"", statute.title)
                .expect("writing to String is infallible");
        }

        // Generate condition check
        if !statute.conditions.is_empty() {
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, 1))
                .collect();
            let conditions = conditions?;
            writeln!(&mut py, "    return {}", conditions.join(" and "))
                .expect("writing to String is infallible");
        } else {
            writeln!(&mut py, "    return True").expect("writing to String is infallible");
        }

        writeln!(&mut py).expect("writing to String is infallible");
        Ok(py)
    }
}

impl CodeGenerator for PythonGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut py = String::new();

        writeln!(&mut py, "# Generated Python from Legal DSL")
            .expect("writing to String is infallible");
        writeln!(&mut py, "# Total statutes: {}", doc.statutes.len())
            .expect("writing to String is infallible");
        writeln!(&mut py, "from typing import Any").expect("writing to String is infallible");
        writeln!(&mut py).expect("writing to String is infallible");

        for statute in &doc.statutes {
            let func_py = self.generate_function(statute)?;
            py.push_str(&func_py);
        }

        Ok(py)
    }

    fn target_language(&self) -> &str {
        "Python"
    }

    fn file_extension(&self) -> &str {
        "py"
    }
}

/// Prolog generator for creating logic predicates from statutes.
pub struct PrologGenerator {
    /// Generate module declarations
    pub generate_module: bool,
    /// Use dynamic predicates
    pub use_dynamic: bool,
}

impl Default for PrologGenerator {
    fn default() -> Self {
        Self {
            generate_module: true,
            use_dynamic: false,
        }
    }
}

impl PrologGenerator {
    /// Creates a new Prolog generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a Prolog predicate name from statute ID.
    fn predicate_name(&self, id: &str) -> String {
        id.replace('-', "_").to_lowercase()
    }

    /// Generates Prolog condition expression.
    #[allow(clippy::only_used_in_recursion)]
    fn generate_condition(&self, condition: &ConditionNode, var: &str) -> DslResult<String> {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                let pl_op = match operator.as_str() {
                    "=" | "==" => "=",
                    "!=" => "\\=",
                    ">=" => ">=",
                    "<=" => "=<", // Prolog uses =< instead of <=
                    ">" => ">",
                    "<" => "<",
                    _ => "=",
                };
                let val = self.format_value(value)?;
                Ok(format!("{}_{} {} {}", var, field, pl_op, val))
            }
            ConditionNode::HasAttribute { key } => Ok(format!(
                "nonvar({}_{}) , {}_{} \\= null",
                var, key, var, key
            )),
            ConditionNode::Between { field, min, max } => {
                let min_val = self.format_value(min)?;
                let max_val = self.format_value(max)?;
                Ok(format!(
                    "{}_{} >= {} , {}_{} =< {}",
                    var, field, min_val, var, field, max_val
                ))
            }
            ConditionNode::In { field, values } => {
                let vals: Result<Vec<_>, _> = values.iter().map(|v| self.format_value(v)).collect();
                let vals = vals?;
                Ok(format!("member({}_{}, [{}])", var, field, vals.join(", ")))
            }
            ConditionNode::Like { field, pattern } => {
                // Prolog doesn't have built-in LIKE, use atom_string and sub_string
                Ok(format!(
                    "atom_string({}_{}, Str), sub_string(Str, _, _, _, \"{}\")",
                    var, field, pattern
                ))
            }
            ConditionNode::And(left, right) => {
                let left_pl = self.generate_condition(left, var)?;
                let right_pl = self.generate_condition(right, var)?;
                Ok(format!("({} , {})", left_pl, right_pl))
            }
            ConditionNode::Or(left, right) => {
                let left_pl = self.generate_condition(left, var)?;
                let right_pl = self.generate_condition(right, var)?;
                Ok(format!("({} ; {})", left_pl, right_pl))
            }
            ConditionNode::Not(inner) => {
                let inner_pl = self.generate_condition(inner, var)?;
                Ok(format!("\\+ ({})", inner_pl))
            }
            ConditionNode::InRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => {
                let min_val = self.format_value(min)?;
                let max_val = self.format_value(max)?;
                let min_op = if *inclusive_min { ">=" } else { ">" };
                let max_op = if *inclusive_max { "=<" } else { "<" };
                Ok(format!(
                    "{}_{} {} {} , {}_{} {} {}",
                    var, field, min_op, min_val, var, field, max_op, max_val
                ))
            }
            ConditionNode::NotInRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => {
                let min_val = self.format_value(min)?;
                let max_val = self.format_value(max)?;
                let min_op = if *inclusive_min { "<" } else { "=<" };
                let max_op = if *inclusive_max { ">" } else { ">=" };
                Ok(format!(
                    "({}_{} {} {} ; {}_{} {} {})",
                    var, field, min_op, min_val, var, field, max_op, max_val
                ))
            }
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => {
                // Prolog regex support varies, use simple pattern matching
                Ok(format!(
                    "atom_string({}_{}, Str), re_match(\"{}\"^^_, Str)",
                    var, field, regex_pattern
                ))
            }
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                let pl_op = match operator.as_str() {
                    "=" | "==" => "=",
                    "!=" => "\\=",
                    ">=" => ">=",
                    "<=" => "=<",
                    ">" => ">",
                    "<" => "<",
                    _ => "=",
                };
                Ok(format!("{:?}_{} {} {}", field, var, pl_op, val))
            }
        }
    }

    /// Formats a condition value for Prolog.
    fn format_value(&self, value: &ConditionValue) -> DslResult<String> {
        match value {
            ConditionValue::Number(n) => Ok(n.to_string()),
            ConditionValue::String(s) => Ok(format!("'{}'", s.replace('\'', "\\'"))),
            ConditionValue::Boolean(b) => Ok(if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }),
            ConditionValue::Date(d) => Ok(format!("'{}'", d)),
            ConditionValue::SetExpr(_) => Ok("[]".to_string()), // Empty list for unsupported set expressions
        }
    }

    /// Generates a Prolog predicate for a statute.
    fn generate_predicate(&self, statute: &StatuteNode) -> DslResult<String> {
        let mut pl = String::new();
        let pred_name = self.predicate_name(&statute.id);

        // Comment with statute title
        writeln!(&mut pl, "% {}", statute.title).expect("writing to String is infallible");

        // Generate required predicates if any
        if !statute.requires.is_empty() {
            writeln!(&mut pl, "% Requires: {}", statute.requires.join(", "))
                .expect("writing to String is infallible");
        }

        // Predicate head
        write!(&mut pl, "{}(Entity) :- ", pred_name).expect("writing to String is infallible");

        // Generate condition body
        if !statute.conditions.is_empty() {
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, "Entity"))
                .collect();
            let conditions = conditions?;

            // Join conditions with comma (AND)
            let body = conditions.join(" , ");
            writeln!(&mut pl, "{}", body).expect("writing to String is infallible");
        } else {
            writeln!(&mut pl, "true").expect("writing to String is infallible");
        }

        // Add required statute checks
        for req in &statute.requires {
            let req_pred = self.predicate_name(req);
            writeln!(&mut pl, "    , {}(Entity)", req_pred)
                .expect("writing to String is infallible");
        }

        writeln!(&mut pl, ".").expect("writing to String is infallible");
        writeln!(&mut pl).expect("writing to String is infallible");

        // Generate effect predicates
        for (idx, effect) in statute.effects.iter().enumerate() {
            writeln!(
                &mut pl,
                "% Effect {}: {} - {}",
                idx + 1,
                effect.effect_type,
                effect.description
            )
            .expect("writing to String is infallible");
            writeln!(
                &mut pl,
                "{}_effect_{}(Entity, '{}') :- {}(Entity).",
                pred_name,
                idx + 1,
                effect.description,
                pred_name
            )
            .expect("writing to String is infallible");
            writeln!(&mut pl).expect("writing to String is infallible");
        }

        Ok(pl)
    }
}

impl CodeGenerator for PrologGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut pl = String::new();

        writeln!(&mut pl, "% Generated Prolog from Legal DSL")
            .expect("writing to String is infallible");
        writeln!(&mut pl, "% Total statutes: {}", doc.statutes.len())
            .expect("writing to String is infallible");
        writeln!(&mut pl).expect("writing to String is infallible");

        if self.generate_module {
            writeln!(&mut pl, ":- module(legal_statutes, []).")
                .expect("writing to String is infallible");
            writeln!(&mut pl).expect("writing to String is infallible");
        }

        if self.use_dynamic {
            for statute in &doc.statutes {
                let pred_name = self.predicate_name(&statute.id);
                writeln!(&mut pl, ":- dynamic {}/1.", pred_name)
                    .expect("writing to String is infallible");
            }
            writeln!(&mut pl).expect("writing to String is infallible");
        }

        for statute in &doc.statutes {
            let pred_pl = self.generate_predicate(statute)?;
            pl.push_str(&pred_pl);
        }

        Ok(pl)
    }

    fn target_language(&self) -> &str {
        "Prolog"
    }

    fn file_extension(&self) -> &str {
        "pl"
    }
}
