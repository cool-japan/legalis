//! Go and Java code generators for the Legalis DSL.

use super::{CodeGenerator, temporal_field_to_string};
use crate::DslResult;
use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use std::fmt::Write;

pub struct GoGenerator {
    /// Package name for generated code
    pub package_name: String,
}

impl Default for GoGenerator {
    fn default() -> Self {
        Self {
            package_name: "statutes".to_string(),
        }
    }
}

impl GoGenerator {
    /// Creates a new Go generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates function name from statute ID (capitalize first letter).
    fn function_name(&self, id: &str) -> String {
        let name = id.replace('-', "_");
        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().chain(chars).collect(),
        }
    }

    /// Generates Go condition expression.
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
            ConditionNode::HasAttribute { key } => Ok(format!("{}.{} != nil", entity_var, key)),
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
                    "contains([]interface{{{{{}}}}}, {}.{})",
                    vals.join(", "),
                    entity_var,
                    field
                ))
            }
            ConditionNode::Like { field, pattern } => Ok(format!(
                "strings.Contains({}.{}, \"{}\")",
                entity_var,
                field,
                pattern.replace('%', "")
            )),
            ConditionNode::And(left, right) => {
                let left_go = self.generate_condition(left, entity_var)?;
                let right_go = self.generate_condition(right, entity_var)?;
                Ok(format!("({} && {})", left_go, right_go))
            }
            ConditionNode::Or(left, right) => {
                let left_go = self.generate_condition(left, entity_var)?;
                let right_go = self.generate_condition(right, entity_var)?;
                Ok(format!("({} || {})", left_go, right_go))
            }
            ConditionNode::Not(inner) => {
                let inner_go = self.generate_condition(inner, entity_var)?;
                Ok(format!("!({})", inner_go))
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
                "regexp.MustCompile(\"{}\").MatchString({}.{})",
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

    /// Formats a condition value for Go.
    fn format_value(&self, value: &ConditionValue) -> DslResult<String> {
        match value {
            ConditionValue::Number(n) => Ok(n.to_string()),
            ConditionValue::String(s) => Ok(format!("\"{}\"", s.replace('"', "\\\""))),
            ConditionValue::Boolean(b) => Ok(b.to_string()),
            ConditionValue::Date(d) => Ok(format!("\"{}\"", d)),
            ConditionValue::SetExpr(_) => Ok("[]interface{}{}".to_string()),
        }
    }

    /// Generates validation function for a statute.
    fn generate_function(&self, statute: &StatuteNode) -> DslResult<String> {
        let mut code = String::new();
        let fn_name = self.function_name(&statute.id);

        writeln!(&mut code, "// {} - {}", fn_name, statute.title)
            .expect("writing to String is infallible");
        writeln!(&mut code, "func {}(entity interface{{}}) bool {{", fn_name)
            .expect("writing to String is infallible");

        if statute.conditions.is_empty() {
            writeln!(&mut code, "\treturn true").expect("writing to String is infallible");
        } else {
            write!(&mut code, "\treturn ").expect("writing to String is infallible");
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

impl CodeGenerator for GoGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut code = String::new();

        writeln!(&mut code, "// Generated Go code from Legal DSL")
            .expect("writing to String is infallible");
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len())
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        writeln!(&mut code, "package {}", self.package_name)
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");
        writeln!(&mut code, "import (").expect("writing to String is infallible");
        writeln!(&mut code, "\t\"regexp\"").expect("writing to String is infallible");
        writeln!(&mut code, "\t\"strings\"").expect("writing to String is infallible");
        writeln!(&mut code, ")").expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        for statute in &doc.statutes {
            let fn_code = self.generate_function(statute)?;
            code.push_str(&fn_code);
        }

        Ok(code)
    }

    fn target_language(&self) -> &str {
        "Go"
    }

    fn file_extension(&self) -> &str {
        "go"
    }
}

/// Java generator for creating validation classes.
pub struct JavaGenerator {
    /// Package name for generated code
    pub package_name: String,
    /// Class name for generated code
    pub class_name: String,
}

impl Default for JavaGenerator {
    fn default() -> Self {
        Self {
            package_name: "com.legal.statutes".to_string(),
            class_name: "StatuteValidator".to_string(),
        }
    }
}

impl JavaGenerator {
    /// Creates a new Java generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates method name from statute ID (camelCase).
    fn method_name(&self, id: &str) -> String {
        let parts: Vec<&str> = id.split('-').collect();
        if parts.is_empty() {
            return String::new();
        }

        let mut result = parts[0].to_string();
        for part in &parts[1..] {
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                result.push_str(&first.to_uppercase().chain(chars).collect::<String>());
            }
        }
        result
    }

    /// Generates Java condition expression.
    fn generate_condition(&self, condition: &ConditionNode, entity_var: &str) -> DslResult<String> {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                let getter = format!("get{}()", self.capitalize_first(field));
                Ok(format!("{}.{} {} {}", entity_var, getter, operator, val))
            }
            ConditionNode::HasAttribute { key } => {
                let getter = format!("get{}()", self.capitalize_first(key));
                Ok(format!("{}.{} != null", entity_var, getter))
            }
            ConditionNode::Between { field, min, max } => {
                let min_val = self.format_value(min)?;
                let max_val = self.format_value(max)?;
                let getter = format!("get{}()", self.capitalize_first(field));
                Ok(format!(
                    "{}.{} >= {} && {}.{} <= {}",
                    entity_var, getter, min_val, entity_var, getter, max_val
                ))
            }
            ConditionNode::In { field, values } => {
                let vals: Result<Vec<_>, _> = values.iter().map(|v| self.format_value(v)).collect();
                let vals = vals?;
                let getter = format!("get{}()", self.capitalize_first(field));
                Ok(format!(
                    "Arrays.asList({}).contains({}.{})",
                    vals.join(", "),
                    entity_var,
                    getter
                ))
            }
            ConditionNode::Like { field, pattern } => {
                let getter = format!("get{}()", self.capitalize_first(field));
                Ok(format!(
                    "{}.{}.contains(\"{}\")",
                    entity_var,
                    getter,
                    pattern.replace('%', "")
                ))
            }
            ConditionNode::And(left, right) => {
                let left_java = self.generate_condition(left, entity_var)?;
                let right_java = self.generate_condition(right, entity_var)?;
                Ok(format!("({} && {})", left_java, right_java))
            }
            ConditionNode::Or(left, right) => {
                let left_java = self.generate_condition(left, entity_var)?;
                let right_java = self.generate_condition(right, entity_var)?;
                Ok(format!("({} || {})", left_java, right_java))
            }
            ConditionNode::Not(inner) => {
                let inner_java = self.generate_condition(inner, entity_var)?;
                Ok(format!("!({})", inner_java))
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
                let getter = format!("get{}()", self.capitalize_first(field));
                let min_op = if *inclusive_min { ">=" } else { ">" };
                let max_op = if *inclusive_max { "<=" } else { "<" };
                Ok(format!(
                    "{}.{} {} {} && {}.{} {} {}",
                    entity_var, getter, min_op, min_val, entity_var, getter, max_op, max_val
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
                let getter = format!("get{}()", self.capitalize_first(field));
                let min_op = if *inclusive_min { "<" } else { "<=" };
                let max_op = if *inclusive_max { ">" } else { ">=" };
                Ok(format!(
                    "{}.{} {} {} || {}.{} {} {}",
                    entity_var, getter, min_op, min_val, entity_var, getter, max_op, max_val
                ))
            }
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => {
                let getter = format!("get{}()", self.capitalize_first(field));
                Ok(format!(
                    "Pattern.compile(\"{}\").matcher({}.{}).matches()",
                    regex_pattern, entity_var, getter
                ))
            }
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                let field_str = temporal_field_to_string(field);
                let getter = format!("get{}()", self.capitalize_first(&field_str));
                Ok(format!("{}.{} {} {}", entity_var, getter, operator, val))
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

    /// Formats a condition value for Java.
    fn format_value(&self, value: &ConditionValue) -> DslResult<String> {
        match value {
            ConditionValue::Number(n) => Ok(n.to_string()),
            ConditionValue::String(s) => Ok(format!("\"{}\"", s.replace('"', "\\\""))),
            ConditionValue::Boolean(b) => Ok(b.to_string()),
            ConditionValue::Date(d) => Ok(format!("\"{}\"", d)),
            ConditionValue::SetExpr(_) => Ok("new ArrayList<>()".to_string()),
        }
    }

    /// Generates validation method for a statute.
    fn generate_method(&self, statute: &StatuteNode) -> DslResult<String> {
        let mut code = String::new();
        let method_name = self.method_name(&statute.id);

        writeln!(&mut code, "    /**").expect("writing to String is infallible");
        writeln!(&mut code, "     * {}", statute.title).expect("writing to String is infallible");
        writeln!(&mut code, "     * @param entity The entity to validate")
            .expect("writing to String is infallible");
        writeln!(&mut code, "     * @return Whether the statute applies")
            .expect("writing to String is infallible");
        writeln!(&mut code, "     */").expect("writing to String is infallible");
        writeln!(
            &mut code,
            "    public static boolean {}(Object entity) {{",
            method_name
        )
        .expect("writing to String is infallible");

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

impl CodeGenerator for JavaGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut code = String::new();

        writeln!(&mut code, "// Generated Java code from Legal DSL")
            .expect("writing to String is infallible");
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len())
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        writeln!(&mut code, "package {};", self.package_name)
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");
        writeln!(&mut code, "import java.util.Arrays;").expect("writing to String is infallible");
        writeln!(&mut code, "import java.util.ArrayList;")
            .expect("writing to String is infallible");
        writeln!(&mut code, "import java.util.regex.Pattern;")
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        writeln!(&mut code, "public class {} {{", self.class_name)
            .expect("writing to String is infallible");
        writeln!(&mut code).expect("writing to String is infallible");

        for statute in &doc.statutes {
            let method_code = self.generate_method(statute)?;
            code.push_str(&method_code);
        }

        writeln!(&mut code, "}}").expect("writing to String is infallible");

        Ok(code)
    }

    fn target_language(&self) -> &str {
        "Java"
    }

    fn file_extension(&self) -> &str {
        "java"
    }
}
