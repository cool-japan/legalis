//! Code generation framework for translating legal statutes to other languages.
//!
//! This module provides a pluggable architecture for generating code in various
//! target languages from the legal DSL AST.

use crate::DslResult;
use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode, TemporalField};
use std::fmt::Write;

/// Helper function to convert TemporalField to a string representation.
fn temporal_field_to_string(field: &TemporalField) -> String {
    match field {
        TemporalField::CurrentDate => "current_date".to_string(),
        TemporalField::DateField(name) => name.clone(),
    }
}

/// Trait for code generators that can translate legal documents.
pub trait CodeGenerator {
    /// Generates code for the entire document.
    fn generate(&self, doc: &LegalDocument) -> DslResult<String>;

    /// Returns the target language name.
    fn target_language(&self) -> &str;

    /// Returns file extension for generated code.
    fn file_extension(&self) -> &str;
}

/// SQL generator for creating database schemas and queries from statutes.
pub struct SqlGenerator {
    /// Use CHECK constraints for conditions
    pub use_check_constraints: bool,
    /// Generate INSERT statements for default values
    pub generate_defaults: bool,
}

impl Default for SqlGenerator {
    fn default() -> Self {
        Self {
            use_check_constraints: true,
            generate_defaults: true,
        }
    }
}

impl SqlGenerator {
    /// Creates a new SQL generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a table name from statute ID.
    fn table_name(&self, id: &str) -> String {
        id.replace('-', "_")
    }

    /// Generates SQL condition expression.
    fn generate_condition(&self, condition: &ConditionNode) -> DslResult<String> {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                Ok(format!("{} {} {}", field, operator, val))
            }
            ConditionNode::HasAttribute { key } => Ok(format!("{} IS NOT NULL", key)),
            ConditionNode::Between { field, min, max } => {
                let min_val = self.format_value(min)?;
                let max_val = self.format_value(max)?;
                Ok(format!("{} BETWEEN {} AND {}", field, min_val, max_val))
            }
            ConditionNode::In { field, values } => {
                let vals: Result<Vec<_>, _> = values.iter().map(|v| self.format_value(v)).collect();
                let vals = vals?;
                Ok(format!("{} IN ({})", field, vals.join(", ")))
            }
            ConditionNode::Like { field, pattern } => Ok(format!("{} LIKE '{}'", field, pattern)),
            ConditionNode::And(left, right) => {
                let left_sql = self.generate_condition(left)?;
                let right_sql = self.generate_condition(right)?;
                Ok(format!("({} AND {})", left_sql, right_sql))
            }
            ConditionNode::Or(left, right) => {
                let left_sql = self.generate_condition(left)?;
                let right_sql = self.generate_condition(right)?;
                Ok(format!("({} OR {})", left_sql, right_sql))
            }
            ConditionNode::Not(inner) => {
                let inner_sql = self.generate_condition(inner)?;
                Ok(format!("NOT ({})", inner_sql))
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
                    "({} {} {} AND {} {} {})",
                    field, min_op, min_val, field, max_op, max_val
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
                    "({} {} {} OR {} {} {})",
                    field, min_op, min_val, field, max_op, max_val
                ))
            }
            ConditionNode::Matches {
                field,
                regex_pattern: _,
            } => {
                // SQL doesn't have universal regex support, use LIKE as fallback
                Ok(format!("{} LIKE '%'", field))
            }
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                let val = self.format_value(value)?;
                Ok(format!("{:?} {} {}", field, operator, val))
            }
        }
    }

    /// Formats a condition value for SQL.
    fn format_value(&self, value: &ConditionValue) -> DslResult<String> {
        match value {
            ConditionValue::Number(n) => Ok(n.to_string()),
            ConditionValue::String(s) => Ok(format!("'{}'", s.replace('\'', "''"))),
            ConditionValue::Boolean(b) => Ok(if *b {
                "TRUE".to_string()
            } else {
                "FALSE".to_string()
            }),
            ConditionValue::Date(d) => Ok(format!("'{}'", d)),
            ConditionValue::SetExpr(_) => Ok("NULL".to_string()), // Set expressions not directly supported in SQL
        }
    }

    /// Generates CREATE TABLE statement for a statute.
    fn generate_table(&self, statute: &StatuteNode) -> DslResult<String> {
        let mut sql = String::new();
        let table_name = self.table_name(&statute.id);

        writeln!(&mut sql, "-- Statute: {}", statute.title).unwrap();
        writeln!(&mut sql, "CREATE TABLE {} (", table_name).unwrap();
        writeln!(&mut sql, "    id SERIAL PRIMARY KEY,").unwrap();

        // Extract fields from conditions
        let mut fields = std::collections::HashSet::new();
        for condition in &statute.conditions {
            self.extract_fields(condition, &mut fields);
        }

        for field in &fields {
            writeln!(&mut sql, "    {} VARCHAR(255),", field).unwrap();
        }

        // Add effect tracking
        writeln!(&mut sql, "    applied BOOLEAN DEFAULT FALSE,").unwrap();
        writeln!(&mut sql, "    applied_at TIMESTAMP").unwrap();

        // Add CHECK constraints if enabled
        if self.use_check_constraints && !statute.conditions.is_empty() {
            write!(&mut sql, "    CONSTRAINT check_{} CHECK (", table_name).unwrap();
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c))
                .collect();
            let conditions = conditions?;
            write!(&mut sql, "{}", conditions.join(" AND ")).unwrap();
            writeln!(&mut sql, ")").unwrap();
        }

        writeln!(&mut sql, ");").unwrap();
        writeln!(&mut sql).unwrap();

        Ok(sql)
    }

    /// Extracts field names from conditions.
    #[allow(clippy::only_used_in_recursion)]
    fn extract_fields(
        &self,
        condition: &ConditionNode,
        fields: &mut std::collections::HashSet<String>,
    ) {
        match condition {
            ConditionNode::Comparison { field, .. }
            | ConditionNode::Between { field, .. }
            | ConditionNode::In { field, .. }
            | ConditionNode::Like { field, .. }
            | ConditionNode::Matches { field, .. }
            | ConditionNode::InRange { field, .. }
            | ConditionNode::NotInRange { field, .. } => {
                fields.insert(field.clone());
            }
            ConditionNode::HasAttribute { key } => {
                fields.insert(key.clone());
            }
            ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                self.extract_fields(left, fields);
                self.extract_fields(right, fields);
            }
            ConditionNode::Not(inner) => {
                self.extract_fields(inner, fields);
            }
            ConditionNode::TemporalComparison { .. } => {
                // Temporal comparisons might need special handling
            }
        }
    }
}

impl CodeGenerator for SqlGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut sql = String::new();

        writeln!(&mut sql, "-- Generated SQL from Legal DSL").unwrap();
        writeln!(&mut sql, "-- Total statutes: {}", doc.statutes.len()).unwrap();
        writeln!(&mut sql).unwrap();

        for statute in &doc.statutes {
            let table_sql = self.generate_table(statute)?;
            sql.push_str(&table_sql);
        }

        Ok(sql)
    }

    fn target_language(&self) -> &str {
        "SQL"
    }

    fn file_extension(&self) -> &str {
        "sql"
    }
}

/// Python generator for creating Python functions from statutes.
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
            writeln!(&mut py, "def {}(obj: Any) -> bool:", func_name).unwrap();
        } else {
            writeln!(&mut py, "def {}(obj):", func_name).unwrap();
        }

        // Docstring
        if self.generate_docstrings {
            writeln!(&mut py, "    \"\"\"{}\"\"\"", statute.title).unwrap();
        }

        // Generate condition check
        if !statute.conditions.is_empty() {
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, 1))
                .collect();
            let conditions = conditions?;
            writeln!(&mut py, "    return {}", conditions.join(" and ")).unwrap();
        } else {
            writeln!(&mut py, "    return True").unwrap();
        }

        writeln!(&mut py).unwrap();
        Ok(py)
    }
}

impl CodeGenerator for PythonGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut py = String::new();

        writeln!(&mut py, "# Generated Python from Legal DSL").unwrap();
        writeln!(&mut py, "# Total statutes: {}", doc.statutes.len()).unwrap();
        writeln!(&mut py, "from typing import Any").unwrap();
        writeln!(&mut py).unwrap();

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
        writeln!(&mut pl, "% {}", statute.title).unwrap();

        // Generate required predicates if any
        if !statute.requires.is_empty() {
            writeln!(&mut pl, "% Requires: {}", statute.requires.join(", ")).unwrap();
        }

        // Predicate head
        write!(&mut pl, "{}(Entity) :- ", pred_name).unwrap();

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
            writeln!(&mut pl, "{}", body).unwrap();
        } else {
            writeln!(&mut pl, "true").unwrap();
        }

        // Add required statute checks
        for req in &statute.requires {
            let req_pred = self.predicate_name(req);
            writeln!(&mut pl, "    , {}(Entity)", req_pred).unwrap();
        }

        writeln!(&mut pl, ".").unwrap();
        writeln!(&mut pl).unwrap();

        // Generate effect predicates
        for (idx, effect) in statute.effects.iter().enumerate() {
            writeln!(
                &mut pl,
                "% Effect {}: {} - {}",
                idx + 1,
                effect.effect_type,
                effect.description
            )
            .unwrap();
            writeln!(
                &mut pl,
                "{}_effect_{}(Entity, '{}') :- {}(Entity).",
                pred_name,
                idx + 1,
                effect.description,
                pred_name
            )
            .unwrap();
            writeln!(&mut pl).unwrap();
        }

        Ok(pl)
    }
}

impl CodeGenerator for PrologGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut pl = String::new();

        writeln!(&mut pl, "% Generated Prolog from Legal DSL").unwrap();
        writeln!(&mut pl, "% Total statutes: {}", doc.statutes.len()).unwrap();
        writeln!(&mut pl).unwrap();

        if self.generate_module {
            writeln!(&mut pl, ":- module(legal_statutes, []).").unwrap();
            writeln!(&mut pl).unwrap();
        }

        if self.use_dynamic {
            for statute in &doc.statutes {
                let pred_name = self.predicate_name(&statute.id);
                writeln!(&mut pl, ":- dynamic {}/1.", pred_name).unwrap();
            }
            writeln!(&mut pl).unwrap();
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

/// TypeScript/JavaScript generator for creating type-safe validation functions.
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
        writeln!(&mut code, "/**").unwrap();
        writeln!(&mut code, " * {}", statute.title).unwrap();
        if !statute.conditions.is_empty() {
            writeln!(
                &mut code,
                " * @param {{any}} entity - The entity to validate"
            )
            .unwrap();
            writeln!(
                &mut code,
                " * @returns {{boolean}} - Whether the statute applies"
            )
            .unwrap();
        }
        writeln!(&mut code, " */").unwrap();

        // Function signature
        if self.use_typescript {
            write!(
                &mut code,
                "export function {}(entity: any): boolean ",
                fn_name
            )
            .unwrap();
        } else {
            write!(&mut code, "export function {}(entity) ", fn_name).unwrap();
        }

        writeln!(&mut code, "{{").unwrap();

        if statute.conditions.is_empty() {
            writeln!(&mut code, "  return true;").unwrap();
        } else {
            write!(&mut code, "  return ").unwrap();
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, "entity"))
                .collect();
            let conditions = conditions?;
            writeln!(&mut code, "{};", conditions.join(" && ")).unwrap();
        }

        writeln!(&mut code, "}}").unwrap();
        writeln!(&mut code).unwrap();

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
        .unwrap();
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len()).unwrap();
        writeln!(&mut code).unwrap();

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

        writeln!(&mut code, "/// {}", statute.title).unwrap();
        writeln!(&mut code, "#[allow(dead_code)]").unwrap();
        write!(&mut code, "pub fn {}<T>(entity: &T) -> bool ", fn_name).unwrap();
        writeln!(&mut code, "{{").unwrap();

        if statute.conditions.is_empty() {
            writeln!(&mut code, "    true").unwrap();
        } else {
            write!(&mut code, "    ").unwrap();
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, "entity"))
                .collect();
            let conditions = conditions?;
            writeln!(&mut code, "{}", conditions.join(" && ")).unwrap();
        }

        writeln!(&mut code, "}}").unwrap();
        writeln!(&mut code).unwrap();

        Ok(code)
    }
}

impl CodeGenerator for RustGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut code = String::new();

        writeln!(&mut code, "// Generated Rust code from Legal DSL").unwrap();
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len()).unwrap();
        writeln!(&mut code).unwrap();

        if self.use_serde {
            writeln!(&mut code, "use serde::{{Serialize, Deserialize}};").unwrap();
        }
        writeln!(&mut code, "use regex::Regex;").unwrap();
        writeln!(&mut code).unwrap();

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

/// Go generator for creating validation functions.
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

        writeln!(&mut code, "// {} - {}", fn_name, statute.title).unwrap();
        writeln!(&mut code, "func {}(entity interface{{}}) bool {{", fn_name).unwrap();

        if statute.conditions.is_empty() {
            writeln!(&mut code, "\treturn true").unwrap();
        } else {
            write!(&mut code, "\treturn ").unwrap();
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, "entity"))
                .collect();
            let conditions = conditions?;
            writeln!(&mut code, "{}", conditions.join(" && ")).unwrap();
        }

        writeln!(&mut code, "}}").unwrap();
        writeln!(&mut code).unwrap();

        Ok(code)
    }
}

impl CodeGenerator for GoGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut code = String::new();

        writeln!(&mut code, "// Generated Go code from Legal DSL").unwrap();
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len()).unwrap();
        writeln!(&mut code).unwrap();

        writeln!(&mut code, "package {}", self.package_name).unwrap();
        writeln!(&mut code).unwrap();
        writeln!(&mut code, "import (").unwrap();
        writeln!(&mut code, "\t\"regexp\"").unwrap();
        writeln!(&mut code, "\t\"strings\"").unwrap();
        writeln!(&mut code, ")").unwrap();
        writeln!(&mut code).unwrap();

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

        writeln!(&mut code, "    /**").unwrap();
        writeln!(&mut code, "     * {}", statute.title).unwrap();
        writeln!(&mut code, "     * @param entity The entity to validate").unwrap();
        writeln!(&mut code, "     * @return Whether the statute applies").unwrap();
        writeln!(&mut code, "     */").unwrap();
        writeln!(
            &mut code,
            "    public static boolean {}(Object entity) {{",
            method_name
        )
        .unwrap();

        if statute.conditions.is_empty() {
            writeln!(&mut code, "        return true;").unwrap();
        } else {
            write!(&mut code, "        return ").unwrap();
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, "entity"))
                .collect();
            let conditions = conditions?;
            writeln!(&mut code, "{};", conditions.join(" && ")).unwrap();
        }

        writeln!(&mut code, "    }}").unwrap();
        writeln!(&mut code).unwrap();

        Ok(code)
    }
}

impl CodeGenerator for JavaGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut code = String::new();

        writeln!(&mut code, "// Generated Java code from Legal DSL").unwrap();
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len()).unwrap();
        writeln!(&mut code).unwrap();

        writeln!(&mut code, "package {};", self.package_name).unwrap();
        writeln!(&mut code).unwrap();
        writeln!(&mut code, "import java.util.Arrays;").unwrap();
        writeln!(&mut code, "import java.util.ArrayList;").unwrap();
        writeln!(&mut code, "import java.util.regex.Pattern;").unwrap();
        writeln!(&mut code).unwrap();

        writeln!(&mut code, "public class {} {{", self.class_name).unwrap();
        writeln!(&mut code).unwrap();

        for statute in &doc.statutes {
            let method_code = self.generate_method(statute)?;
            code.push_str(&method_code);
        }

        writeln!(&mut code, "}}").unwrap();

        Ok(code)
    }

    fn target_language(&self) -> &str {
        "Java"
    }

    fn file_extension(&self) -> &str {
        "java"
    }
}

/// C# generator for creating validation classes.
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

        writeln!(&mut code, "    /// <summary>").unwrap();
        writeln!(&mut code, "    /// {}", statute.title).unwrap();
        writeln!(&mut code, "    /// </summary>").unwrap();
        writeln!(
            &mut code,
            "    /// <param name=\"entity\">The entity to validate</param>"
        )
        .unwrap();
        writeln!(
            &mut code,
            "    /// <returns>Whether the statute applies</returns>"
        )
        .unwrap();
        writeln!(
            &mut code,
            "    public static bool {}(object entity)",
            method_name
        )
        .unwrap();
        writeln!(&mut code, "    {{").unwrap();

        if statute.conditions.is_empty() {
            writeln!(&mut code, "        return true;").unwrap();
        } else {
            write!(&mut code, "        return ").unwrap();
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c, "entity"))
                .collect();
            let conditions = conditions?;
            writeln!(&mut code, "{};", conditions.join(" && ")).unwrap();
        }

        writeln!(&mut code, "    }}").unwrap();
        writeln!(&mut code).unwrap();

        Ok(code)
    }
}

impl CodeGenerator for CSharpGenerator {
    fn generate(&self, doc: &LegalDocument) -> DslResult<String> {
        let mut code = String::new();

        writeln!(&mut code, "// Generated C# code from Legal DSL").unwrap();
        writeln!(&mut code, "// Total statutes: {}", doc.statutes.len()).unwrap();
        writeln!(&mut code).unwrap();

        writeln!(&mut code, "using System;").unwrap();
        writeln!(&mut code, "using System.Linq;").unwrap();
        writeln!(&mut code, "using System.Collections.Generic;").unwrap();
        writeln!(&mut code, "using System.Text.RegularExpressions;").unwrap();
        writeln!(&mut code).unwrap();

        writeln!(&mut code, "namespace {}", self.namespace).unwrap();
        writeln!(&mut code, "{{").unwrap();
        writeln!(&mut code, "    public static class {}", self.class_name).unwrap();
        writeln!(&mut code, "    {{").unwrap();

        for statute in &doc.statutes {
            let method_code = self.generate_method(statute)?;
            code.push_str(&method_code);
        }

        writeln!(&mut code, "    }}").unwrap();
        writeln!(&mut code, "}}").unwrap();

        Ok(code)
    }

    fn target_language(&self) -> &str {
        "C#"
    }

    fn file_extension(&self) -> &str {
        "cs"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ConditionValue, EffectNode};

    fn sample_statute() -> StatuteNode {
        StatuteNode {
            id: "voting-rights".to_string(),
            visibility: crate::module_system::Visibility::Private,
            title: "Voting Rights Statute".to_string(),
            conditions: vec![
                ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">=".to_string(),
                    value: ConditionValue::Number(18),
                },
                ConditionNode::HasAttribute {
                    key: "citizen".to_string(),
                },
            ],
            effects: vec![EffectNode {
                effect_type: "grant".to_string(),
                description: "Right to vote".to_string(),
                parameters: vec![],
            }],
            discretion: None,
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires: vec![],
            delegates: vec![],
            scope: None,
            constraints: vec![],
            priority: None,
        }
    }

    #[test]
    fn test_sql_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = SqlGenerator::new();
        let sql = generator.generate(&doc).unwrap();

        assert!(sql.contains("CREATE TABLE voting_rights"));
        assert!(sql.contains("age"));
        assert!(sql.contains("citizen"));
    }

    #[test]
    fn test_python_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PythonGenerator::new();
        let py = generator.generate(&doc).unwrap();

        assert!(py.contains("def voting_rights"));
        assert!(py.contains("obj.age >= 18"));
        assert!(py.contains("hasattr(obj, 'citizen')"));
    }

    #[test]
    fn test_sql_generator_metadata() {
        let generator = SqlGenerator::new();
        assert_eq!(generator.target_language(), "SQL");
        assert_eq!(generator.file_extension(), "sql");
    }

    #[test]
    fn test_python_generator_metadata() {
        let generator = PythonGenerator::new();
        assert_eq!(generator.target_language(), "Python");
        assert_eq!(generator.file_extension(), "py");
    }

    #[test]
    fn test_prolog_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PrologGenerator::new();
        let pl = generator.generate(&doc).unwrap();

        assert!(pl.contains("voting_rights(Entity)"));
        assert!(pl.contains("Entity_age >= 18"));
        assert!(pl.contains("nonvar(Entity_citizen)"));
    }

    #[test]
    fn test_prolog_generator_metadata() {
        let generator = PrologGenerator::new();
        assert_eq!(generator.target_language(), "Prolog");
        assert_eq!(generator.file_extension(), "pl");
    }

    #[test]
    fn test_prolog_module_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PrologGenerator {
            generate_module: true,
            use_dynamic: true,
        };
        let pl = generator.generate(&doc).unwrap();

        assert!(pl.contains(":- module(legal_statutes, [])"));
        assert!(pl.contains(":- dynamic voting_rights/1"));
    }

    #[test]
    fn test_prolog_effect_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PrologGenerator::new();
        let pl = generator.generate(&doc).unwrap();

        assert!(pl.contains("voting_rights_effect_1"));
        assert!(pl.contains("Right to vote"));
    }

    #[test]
    fn test_sql_roundtrip_validation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = SqlGenerator::new();
        let sql = generator.generate(&doc).unwrap();

        // Verify SQL contains expected keywords
        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("PRIMARY KEY"));
        assert!(sql.contains("CHECK"));

        // Verify no syntax errors in basic structure
        assert!(!sql.contains(";;")); // No double semicolons
        assert!(sql.matches('(').count() == sql.matches(')').count()); // Balanced parentheses
    }

    #[test]
    fn test_python_roundtrip_validation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PythonGenerator::new();
        let py = generator.generate(&doc).unwrap();

        // Verify Python contains expected structures
        assert!(py.contains("def "));
        assert!(py.contains("return "));
        assert!(py.contains("from typing import Any"));

        // Verify basic Python syntax
        assert!(py.matches("def ").count() == py.matches("return ").count());
    }

    #[test]
    fn test_prolog_roundtrip_validation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PrologGenerator::new();
        let pl = generator.generate(&doc).unwrap();

        // Verify Prolog contains expected structures
        assert!(pl.contains("(Entity) :- "));
        assert!(pl.ends_with("\n") || pl.ends_with("."));

        // Verify balanced predicates (all :- have corresponding .)
        assert!(pl.matches(":-").count() <= pl.matches('.').count());
    }

    #[test]
    fn test_complex_document_all_generators() {
        let complex_statute = StatuteNode {
            id: "complex-law".to_string(),
            visibility: crate::module_system::Visibility::Private,
            title: "Complex Law Test".to_string(),
            conditions: vec![ConditionNode::And(
                Box::new(ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">=".to_string(),
                    value: ConditionValue::Number(18),
                }),
                Box::new(ConditionNode::In {
                    field: "status".to_string(),
                    values: vec![
                        ConditionValue::String("citizen".to_string()),
                        ConditionValue::String("resident".to_string()),
                    ],
                }),
            )],
            effects: vec![
                EffectNode {
                    effect_type: "GRANT".to_string(),
                    description: "Voting rights".to_string(),
                    parameters: vec![],
                },
                EffectNode {
                    effect_type: "OBLIGATION".to_string(),
                    description: "Register to vote".to_string(),
                    parameters: vec![],
                },
            ],
            discretion: None,
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires: vec![],
            delegates: vec![],
            scope: None,
            constraints: vec![],
            priority: None,
        };

        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![complex_statute],
        };

        // Test all generators can handle complex documents
        let sql_gen = SqlGenerator::new();
        let sql = sql_gen.generate(&doc).unwrap();
        assert!(sql.len() > 100);

        let py_gen = PythonGenerator::new();
        let py = py_gen.generate(&doc).unwrap();
        assert!(py.len() > 100);

        let pl_gen = PrologGenerator::new();
        let pl = pl_gen.generate(&doc).unwrap();
        assert!(pl.len() > 100);

        // Test new generators
        let ts_gen = TypeScriptGenerator::new();
        let ts = ts_gen.generate(&doc).unwrap();
        assert!(ts.len() > 100);

        let rust_gen = RustGenerator::new();
        let rust = rust_gen.generate(&doc).unwrap();
        assert!(rust.len() > 100);

        let go_gen = GoGenerator::new();
        let go = go_gen.generate(&doc).unwrap();
        assert!(go.len() > 100);

        let java_gen = JavaGenerator::new();
        let java = java_gen.generate(&doc).unwrap();
        assert!(java.len() > 100);

        let cs_gen = CSharpGenerator::new();
        let cs = cs_gen.generate(&doc).unwrap();
        assert!(cs.len() > 100);
    }

    #[test]
    fn test_typescript_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = TypeScriptGenerator::new();
        let ts = generator.generate(&doc).unwrap();

        assert!(ts.contains("export function"));
        assert!(ts.contains("voting_rights"));
        assert!(ts.contains("entity"));
        assert!(ts.contains(": boolean"));
    }

    #[test]
    fn test_rust_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = RustGenerator::new();
        let rs = generator.generate(&doc).unwrap();

        assert!(rs.contains("pub fn voting_rights"));
        assert!(rs.contains("-> bool"));
        assert!(rs.contains("use regex::Regex"));
    }

    #[test]
    fn test_go_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = GoGenerator::new();
        let go = generator.generate(&doc).unwrap();

        assert!(go.contains("package statutes"));
        assert!(go.contains("func Voting_rights"));
        assert!(go.contains("bool"));
        assert!(go.contains("import ("));
    }

    #[test]
    fn test_java_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = JavaGenerator::new();
        let java = generator.generate(&doc).unwrap();

        assert!(java.contains("public class StatuteValidator"));
        assert!(java.contains("public static boolean votingRights"));
        assert!(java.contains("package com.legal.statutes"));
        assert!(java.contains("import java.util"));
    }

    #[test]
    fn test_typescript_javascript_mode() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let mut generator = TypeScriptGenerator::new();
        generator.use_typescript = false;

        let js = generator.generate(&doc).unwrap();
        assert!(js.contains("export function"));
        assert!(!js.contains(": boolean"));
        assert_eq!(generator.file_extension(), "js");
        assert_eq!(generator.target_language(), "JavaScript");
    }

    #[test]
    fn test_csharp_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = CSharpGenerator::new();
        let cs = generator.generate(&doc).unwrap();

        assert!(cs.contains("namespace Legal.Statutes"));
        assert!(cs.contains("public static class StatuteValidator"));
        assert!(cs.contains("public static bool VotingRights"));
        assert!(cs.contains("using System"));
    }

    #[test]
    fn test_all_generators_file_extensions() {
        assert_eq!(SqlGenerator::new().file_extension(), "sql");
        assert_eq!(PythonGenerator::new().file_extension(), "py");
        assert_eq!(PrologGenerator::new().file_extension(), "pl");
        assert_eq!(TypeScriptGenerator::new().file_extension(), "ts");
        assert_eq!(RustGenerator::new().file_extension(), "rs");
        assert_eq!(GoGenerator::new().file_extension(), "go");
        assert_eq!(JavaGenerator::new().file_extension(), "java");
        assert_eq!(CSharpGenerator::new().file_extension(), "cs");
    }

    #[test]
    fn test_all_generators_target_languages() {
        assert_eq!(SqlGenerator::new().target_language(), "SQL");
        assert_eq!(PythonGenerator::new().target_language(), "Python");
        assert_eq!(PrologGenerator::new().target_language(), "Prolog");
        assert_eq!(TypeScriptGenerator::new().target_language(), "TypeScript");
        assert_eq!(RustGenerator::new().target_language(), "Rust");
        assert_eq!(GoGenerator::new().target_language(), "Go");
        assert_eq!(JavaGenerator::new().target_language(), "Java");
        assert_eq!(CSharpGenerator::new().target_language(), "C#");
    }
}
