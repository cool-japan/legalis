//! C# code generator for the Legalis DSL.

use super::{CodeGenerator, temporal_field_to_string};
use crate::DslResult;
use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use std::fmt::Write;

pub struct CSharpGenerator {
    /// Namespace for generated code
    pub namespace: String,
    /// Class name for generated code
    pub class_name: String,
}

impl Default for CSharpGenerator {
    fn default() -> Self {
        Self {
            namespace: "Legal.Statutes".to_string(),
            class_name: "StatuteValidator".to_string(),
        }
    }
}

impl CSharpGenerator {
    /// Creates a new C# generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates method name from statute ID (PascalCase).
    fn method_name(&self, id: &str) -> String {
        id.split('-')
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            })
            .collect()
    }

    /// Generates C# condition expression.
    fn generate_condition(&self, condition: &ConditionNode, entity_var: &str) -> DslResult<String> {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                Ok(format!(
                    "{}.{} {} {}",
                    entity_var,
                    self.capitalize_first(field),
                    operator,
                    val
                ))
            }
            ConditionNode::HasAttribute { key } => Ok(format!(
                "{}.{} != null",
                entity_var,
                self.capitalize_first(key)
            )),
            ConditionNode::Between { field, min, max } => {
                let min_val = self.format_value(min)?;
                let max_val = self.format_value(max)?;
                let prop = self.capitalize_first(field);
                Ok(format!(
                    "{}.{} >= {} && {}.{} <= {}",
                    entity_var, prop, min_val, entity_var, prop, max_val
                ))
            }
            ConditionNode::In { field, values } => {
                let vals: Result<Vec<_>, _> = values.iter().map(|v| self.format_value(v)).collect();
                let vals = vals?;
                let prop = self.capitalize_first(field);
                Ok(format!(
                    "new[] {{ {} }}.Contains({}.{})",
                    vals.join(", "),
                    entity_var,
                    prop
                ))
            }
            ConditionNode::Like { field, pattern } => {
                let prop = self.capitalize_first(field);
                Ok(format!(
                    "{}.{}.Contains(\"{}\")",
                    entity_var,
                    prop,
                    pattern.replace('%', "")
                ))
            }
            ConditionNode::And(left, right) => {
                let left_cs = self.generate_condition(left, entity_var)?;
                let right_cs = self.generate_condition(right, entity_var)?;
                Ok(format!("({} && {})", left_cs, right_cs))
            }
            ConditionNode::Or(left, right) => {
                let left_cs = self.generate_condition(left, entity_var)?;
                let right_cs = self.generate_condition(right, entity_var)?;
                Ok(format!("({} || {})", left_cs, right_cs))
            }
            ConditionNode::Not(inner) => {
                let inner_cs = self.generate_condition(inner, entity_var)?;
                Ok(format!("!({})", inner_cs))
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
                let prop = self.capitalize_first(field);
                let min_op = if *inclusive_min { ">=" } else { ">" };
                let max_op = if *inclusive_max { "<=" } else { "<" };
                Ok(format!(
                    "{}.{} {} {} && {}.{} {} {}",
                    entity_var, prop, min_op, min_val, entity_var, prop, max_op, max_val
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
                let prop = self.capitalize_first(field);
                let min_op = if *inclusive_min { "<" } else { "<=" };
                let max_op = if *inclusive_max { ">" } else { ">=" };
                Ok(format!(
                    "{}.{} {} {} || {}.{} {} {}",
                    entity_var, prop, min_op, min_val, entity_var, prop, max_op, max_val
                ))
            }
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => {
                let prop = self.capitalize_first(field);
                Ok(format!(
                    "Regex.IsMatch({}.{}, @\"{}\")",
                    entity_var, prop, regex_pattern
                ))
            }
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                let field_str = temporal_field_to_string(field);
                let prop = self.capitalize_first(&field_str);
                Ok(format!("{}.{} {} {}", entity_var, prop, operator, val))
            }
        }
    }

    /// Capitalizes first character of a string.
    fn capitalize_first(&self, s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().chain(chars).collect(),
        }
    }

    /// Formats a condition value for C#.
    fn format_value(&self, value: &ConditionValue) -> DslResult<String> {
        match value {
            ConditionValue::Number(n) => Ok(n.to_string()),
            ConditionValue::String(s) => Ok(format!("\"{}\"", s.replace('"', "\\\""))),
            ConditionValue::Boolean(b) => Ok(if *b { "true" } else { "false" }.to_string()),
            ConditionValue::Date(d) => Ok(format!("\"{}\"", d)),
            ConditionValue::SetExpr(_) => Ok("new List<object>()".to_string()),
        }
    }

    /// Generates validation method for a statute.
    fn generate_method(&self, statute: &StatuteNode) -> DslResult<String> {
        let mut code = String::new();
        let method_name = self.method_name(&statute.id);

        writeln!(&mut code, "    /// <summary>").expect("writing to String is infallible");
        writeln!(&mut code, "    /// {}", statute.title).expect("writing to String is infallible");
        writeln!(&mut code, "    /// </summary>").expect("writing to String is infallible");
        writeln!(
            &mut code,
            "    /// <param name=\"entity\">The entity to validate</param>"
        )
        .expect("writing to String is infallible");
        writeln!(
            &mut code,
            "    /// <returns>Whether the statute applies</returns>"
        )
        .expect("writing to String is infallible");
        writeln!(
            &mut code,
            "    public static bool {}(object entity)",
            method_name
        )
        .expect("writing to String is infallible");
        writeln!(&mut code, "    {{").expect("writing to String is infallible");

        if statute.conditions.is_empty() {
            writeln!(&mut code, "        return true;").expect("writing to String is infallible");
        } else {
            write!(&mut code, "        return ").expect("writing to String is infallible");
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, "entity"))
                .collect();
            let conditions = conditions?;
            writeln!(&mut code, "{};", conditions.join(" && "))
                .expect("writing to String is infallible");
        }

        writeln!(&mut code, "    }}").expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        Ok(code)
    }
}

impl CodeGenerator for CSharpGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut code = String::new();

        writeln!(&mut code, "// Generated C# code from Legal DSL")
            .expect("writing to String is infallible");
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len())
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        writeln!(&mut code, "using System;").expect("writing to String is infallible");
        writeln!(&mut code, "using System.Linq;").expect("writing to String is infallible");
        writeln!(&mut code, "using System.Collections.Generic;")
            .expect("writing to String is infallible");
        writeln!(&mut code, "using System.Text.RegularExpressions;")
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        writeln!(&mut code, "namespace {}", self.namespace)
            .expect("writing to String is infallible");
        writeln!(&mut code, "{{").expect("writing to String is infallible");
        writeln!(&mut code, "    public static class {}", self.class_name)
            .expect("writing to String is infallible");
        writeln!(&mut code, "    {{").expect("writing to String is infallible");

        for statute in &doc.statutes {
            let method_code = self.generate_method(statute)?;
            code.push_str(&method_code);
        }

        writeln!(&mut code, "    }}").expect("writing to String is infallible");
        writeln!(&mut code, "}}").expect("writing to String is infallible");

        Ok(code)
    }

    fn target_language(&self) -> &str {
        "C#"
    }

    fn file_extension(&self) -> &str {
        "cs"
    }
}
