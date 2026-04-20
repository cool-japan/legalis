//! TypeScript and Rust code generators for the Legalis DSL.

use super::{CodeGenerator, temporal_field_to_string};
use crate::DslResult;
use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use std::fmt::Write;

pub struct TypeScriptGenerator {
    /// Generate TypeScript (true) or JavaScript (false)
    pub use_typescript: bool,
    /// Generate ES6 modules
    pub use_es6_modules: bool,
}

impl Default for TypeScriptGenerator {
    fn default() -> Self {
        Self {
            use_typescript: true,
            use_es6_modules: true,
        }
    }
}

impl TypeScriptGenerator {
    /// Creates a new TypeScript generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates function name from statute ID.
    fn function_name(&self, id: &str) -> String {
        id.replace('-', "_")
    }

    /// Generates TypeScript/JavaScript condition expression.
    fn generate_condition(&self, condition: &ConditionNode, entity_var: &str) -> DslResult<String> {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                Ok(format!("{}.{} {} {}", entity_var, field, operator, val))
            }
            ConditionNode::HasAttribute { key } => Ok(format!(
                "{}.{} !== undefined && {}.{} !== null",
                entity_var, key, entity_var, key
            )),
            ConditionNode::Between { field, min, max } => {
                let min_val = self.format_value(min)?;
                let max_val = self.format_value(max)?;
                Ok(format!(
                    "{}.{} >= {} && {}.{} <= {}",
                    entity_var, field, min_val, entity_var, field, max_val
                ))
            }
            ConditionNode::In { field, values } => {
                let vals: Result<Vec<_>, _> = values.iter().map(|v| self.format_value(v)).collect();
                let vals = vals?;
                Ok(format!(
                    "[{}].includes({}.{})",
                    vals.join(", "),
                    entity_var,
                    field
                ))
            }
            ConditionNode::Like { field, pattern } => Ok(format!(
                "/{}/i.test({}.{})",
                pattern.replace('%', ".*"),
                entity_var,
                field
            )),
            ConditionNode::And(left, right) => {
                let left_js = self.generate_condition(left, entity_var)?;
                let right_js = self.generate_condition(right, entity_var)?;
                Ok(format!("({} && {})", left_js, right_js))
            }
            ConditionNode::Or(left, right) => {
                let left_js = self.generate_condition(left, entity_var)?;
                let right_js = self.generate_condition(right, entity_var)?;
                Ok(format!("({} || {})", left_js, right_js))
            }
            ConditionNode::Not(inner) => {
                let inner_js = self.generate_condition(inner, entity_var)?;
                Ok(format!("!({})", inner_js))
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
                let max_op = if *inclusive_max { "<=" } else { "<" };
                Ok(format!(
                    "{}.{} {} {} && {}.{} {} {}",
                    entity_var, field, min_op, min_val, entity_var, field, max_op, max_val
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
                let min_op = if *inclusive_min { "<" } else { "<=" };
                let max_op = if *inclusive_max { ">" } else { ">=" };
                Ok(format!(
                    "{}.{} {} {} || {}.{} {} {}",
                    entity_var, field, min_op, min_val, entity_var, field, max_op, max_val
                ))
            }
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => Ok(format!(
                "/{}/i.test({}.{})",
                regex_pattern, entity_var, field
            )),
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                let field_str = temporal_field_to_string(field);
                Ok(format!(
                    "new Date({}.{}) {} new Date({})",
                    entity_var, field_str, operator, val
                ))
            }
        }
    }

    /// Formats a condition value for TypeScript/JavaScript.
    fn format_value(&self, value: &ConditionValue) -> DslResult<String> {
        match value {
            ConditionValue::Number(n) => Ok(n.to_string()),
            ConditionValue::String(s) => Ok(format!("\"{}\"", s.replace('"', "\\\""))),
            ConditionValue::Boolean(b) => Ok(b.to_string()),
            ConditionValue::Date(d) => Ok(format!("\"{}\"", d)),
            ConditionValue::SetExpr(_) => Ok("[]".to_string()),
        }
    }

    /// Generates validation function for a statute.
    fn generate_function(&self, statute: &StatuteNode) -> DslResult<String> {
        let mut code = String::new();
        let fn_name = self.function_name(&statute.id);

        // Add JSDoc comment
        writeln!(&mut code, "/**").expect("writing to String is infallible");
        writeln!(&mut code, " * {}", statute.title).expect("writing to String is infallible");
        if !statute.conditions.is_empty() {
            writeln!(
                &mut code,
                " * @param {{any}} entity - The entity to validate"
            )
            .expect("writing to String is infallible");
            writeln!(
                &mut code,
                " * @returns {{boolean}} - Whether the statute applies"
            )
            .expect("writing to String is infallible");
        }
        writeln!(&mut code, " */").expect("writing to String is infallible");

        // Function signature
        if self.use_typescript {
            write!(
                &mut code,
                "export function {}(entity: any): boolean ",
                fn_name
            )
            .expect("writing to String is infallible");
        } else {
            write!(&mut code, "export function {}(entity) ", fn_name)
                .expect("writing to String is infallible");
        }

        writeln!(&mut code, "{{").expect("writing to String is infallible");

        if statute.conditions.is_empty() {
            writeln!(&mut code, "  return true;").expect("writing to String is infallible");
        } else {
            write!(&mut code, "  return ").expect("writing to String is infallible");
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, "entity"))
                .collect();
            let conditions = conditions?;
            writeln!(&mut code, "{};", conditions.join(" && "))
                .expect("writing to String is infallible");
        }

        writeln!(&mut code, "}}").expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        Ok(code)
    }
}

impl CodeGenerator for TypeScriptGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut code = String::new();

        writeln!(
            &mut code,
            "// Generated TypeScript/JavaScript from Legal DSL"
        )
        .expect("writing to String is infallible");
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len())
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        for statute in &doc.statutes {
            let fn_code = self.generate_function(statute)?;
            code.push_str(&fn_code);
        }

        Ok(code)
    }

    fn target_language(&self) -> &str {
        if self.use_typescript {
            "TypeScript"
        } else {
            "JavaScript"
        }
    }

    fn file_extension(&self) -> &str {
        if self.use_typescript { "ts" } else { "js" }
    }
}

/// Rust generator for creating type-safe validation functions.
pub struct RustGenerator {
    /// Use serde for JSON serialization
    pub use_serde: bool,
}

impl Default for RustGenerator {
    fn default() -> Self {
        Self { use_serde: true }
    }
}

impl RustGenerator {
    /// Creates a new Rust generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates function name from statute ID.
    fn function_name(&self, id: &str) -> String {
        id.replace('-', "_")
    }

    /// Generates Rust condition expression.
    fn generate_condition(&self, condition: &ConditionNode, entity_var: &str) -> DslResult<String> {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                Ok(format!("{}.{} {} {}", entity_var, field, operator, val))
            }
            ConditionNode::HasAttribute { key } => Ok(format!("{}.{}.is_some()", entity_var, key)),
            ConditionNode::Between { field, min, max } => {
                let min_val = self.format_value(min)?;
                let max_val = self.format_value(max)?;
                Ok(format!(
                    "{}.{} >= {} && {}.{} <= {}",
                    entity_var, field, min_val, entity_var, field, max_val
                ))
            }
            ConditionNode::In { field, values } => {
                let vals: Result<Vec<_>, _> = values.iter().map(|v| self.format_value(v)).collect();
                let vals = vals?;
                Ok(format!(
                    "[{}].contains(&{}.{})",
                    vals.join(", "),
                    entity_var,
                    field
                ))
            }
            ConditionNode::Like { field, pattern } => Ok(format!(
                "{}.{}.contains(\"{}\")",
                entity_var,
                field,
                pattern.replace('%', "")
            )),
            ConditionNode::And(left, right) => {
                let left_rs = self.generate_condition(left, entity_var)?;
                let right_rs = self.generate_condition(right, entity_var)?;
                Ok(format!("({} && {})", left_rs, right_rs))
            }
            ConditionNode::Or(left, right) => {
                let left_rs = self.generate_condition(left, entity_var)?;
                let right_rs = self.generate_condition(right, entity_var)?;
                Ok(format!("({} || {})", left_rs, right_rs))
            }
            ConditionNode::Not(inner) => {
                let inner_rs = self.generate_condition(inner, entity_var)?;
                Ok(format!("!({})", inner_rs))
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
                let max_op = if *inclusive_max { "<=" } else { "<" };
                Ok(format!(
                    "{}.{} {} {} && {}.{} {} {}",
                    entity_var, field, min_op, min_val, entity_var, field, max_op, max_val
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
                let min_op = if *inclusive_min { "<" } else { "<=" };
                let max_op = if *inclusive_max { ">" } else { ">=" };
                Ok(format!(
                    "{}.{} {} {} || {}.{} {} {}",
                    entity_var, field, min_op, min_val, entity_var, field, max_op, max_val
                ))
            }
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => Ok(format!(
                "Regex::new(r\"{}\").unwrap().is_match(&{}.{})",
                regex_pattern, entity_var, field
            )),
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                let field_str = temporal_field_to_string(field);
                Ok(format!("{}.{} {} {}", entity_var, field_str, operator, val))
            }
        }
    }

    /// Formats a condition value for Rust.
    fn format_value(&self, value: &ConditionValue) -> DslResult<String> {
        match value {
            ConditionValue::Number(n) => Ok(n.to_string()),
            ConditionValue::String(s) => Ok(format!("\"{}\"", s.replace('"', "\\\""))),
            ConditionValue::Boolean(b) => Ok(b.to_string()),
            ConditionValue::Date(d) => Ok(format!("\"{}\"", d)),
            ConditionValue::SetExpr(_) => Ok("vec![]".to_string()),
        }
    }

    /// Generates validation function for a statute.
    fn generate_function(&self, statute: &StatuteNode) -> DslResult<String> {
        let mut code = String::new();
        let fn_name = self.function_name(&statute.id);

        writeln!(&mut code, "/// {}", statute.title).expect("writing to String is infallible");
        writeln!(&mut code, "#[allow(dead_code)]").expect("writing to String is infallible");
        write!(&mut code, "pub fn {}<T>(entity: &T) -> bool ", fn_name)
            .expect("writing to String is infallible");
        writeln!(&mut code, "{{").expect("writing to String is infallible");

        if statute.conditions.is_empty() {
            writeln!(&mut code, "    true").expect("writing to String is infallible");
        } else {
            write!(&mut code, "    ").expect("writing to String is infallible");
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, "entity"))
                .collect();
            let conditions = conditions?;
            writeln!(&mut code, "{}", conditions.join(" && "))
                .expect("writing to String is infallible");
        }

        writeln!(&mut code, "}}").expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        Ok(code)
    }
}

impl CodeGenerator for RustGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut code = String::new();

        writeln!(&mut code, "// Generated Rust code from Legal DSL")
            .expect("writing to String is infallible");
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len())
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        if self.use_serde {
            writeln!(&mut code, "use serde::{{Serialize, Deserialize}};")
                .expect("writing to String is infallible");
        }
        writeln!(&mut code, "use regex::Regex;").expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        for statute in &doc.statutes {
            let fn_code = self.generate_function(statute)?;
            code.push_str(&fn_code);
        }

        Ok(code)
    }

    fn target_language(&self) -> &str {
        "Rust"
    }

    fn file_extension(&self) -> &str {
        "rs"
    }
}
