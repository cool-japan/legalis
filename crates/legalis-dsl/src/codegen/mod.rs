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

        writeln!(&mut sql, "-- Statute: {}", statute.title)
            .expect("writing to String is infallible");
        writeln!(&mut sql, "CREATE TABLE {} (", table_name)
            .expect("writing to String is infallible");
        writeln!(&mut sql, "    id SERIAL PRIMARY KEY,").expect("writing to String is infallible");

        // Extract fields from conditions
        let mut fields = std::collections::HashSet::new();
        for condition in &statute.conditions {
            self.extract_fields(condition, &mut fields);
        }

        for field in &fields {
            writeln!(&mut sql, "    {} VARCHAR(255),", field)
                .expect("writing to String is infallible");
        }

        // Add effect tracking
        writeln!(&mut sql, "    applied BOOLEAN DEFAULT FALSE,")
            .expect("writing to String is infallible");
        writeln!(&mut sql, "    applied_at TIMESTAMP").expect("writing to String is infallible");

        // Add CHECK constraints if enabled
        if self.use_check_constraints && !statute.conditions.is_empty() {
            write!(&mut sql, "    CONSTRAINT check_{} CHECK (", table_name)
                .expect("writing to String is infallible");
            let conditions: Result<Vec<_>, _> = statute
                .conditions
                .iter()
                .map(|c| self.generate_condition(c))
                .collect();
            let conditions = conditions?;
            write!(&mut sql, "{}", conditions.join(" AND "))
                .expect("writing to String is infallible");
            writeln!(&mut sql, ")").expect("writing to String is infallible");
        }

        writeln!(&mut sql, ");").expect("writing to String is infallible");
        writeln!(&mut sql).expect("writing to String is infallible");

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

        writeln!(&mut sql, "-- Generated SQL from Legal DSL")
            .expect("writing to String is infallible");
        writeln!(&mut sql, "-- Total statutes: {}", doc.statutes.len())
            .expect("writing to String is infallible");
        writeln!(&mut sql).expect("writing to String is infallible");

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

// Submodules containing the generator implementations.
mod csharp;
mod go_java;
mod python_prolog;
#[cfg(test)]
mod tests;
mod typescript_rust;

pub use csharp::CSharpGenerator;
pub use go_java::{GoGenerator, JavaGenerator};
pub use python_prolog::{PrologGenerator, PythonGenerator};
pub use typescript_rust::{RustGenerator, TypeScriptGenerator};
