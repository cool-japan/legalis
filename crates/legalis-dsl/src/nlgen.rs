//! Natural Language Generator for Legal DSL
//!
//! This module converts legal DSL statutes into human-readable natural language descriptions.
//! It supports multiple languages and customizable verbosity levels.

use crate::ast::*;
use std::fmt::Write as FmtWrite;

/// Language for natural language output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Japanese,
    German,
    French,
    Chinese,
}

/// Verbosity level for natural language output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Brief summary (1-2 sentences)
    Brief,
    /// Normal detail level
    Normal,
    /// Full detailed explanation
    Detailed,
}

/// Configuration for natural language generation
#[derive(Debug, Clone)]
pub struct NLConfig {
    pub language: Language,
    pub verbosity: Verbosity,
    pub include_metadata: bool,
    pub include_examples: bool,
}

impl Default for NLConfig {
    fn default() -> Self {
        Self {
            language: Language::English,
            verbosity: Verbosity::Normal,
            include_metadata: true,
            include_examples: false,
        }
    }
}

/// Natural Language Generator
pub struct NLGenerator {
    config: NLConfig,
}

impl NLGenerator {
    /// Create a new natural language generator with default configuration
    pub fn new() -> Self {
        Self {
            config: NLConfig::default(),
        }
    }

    /// Create a new natural language generator with custom configuration
    pub fn with_config(config: NLConfig) -> Self {
        Self { config }
    }

    /// Generate natural language description for a legal document
    pub fn generate_document(&self, doc: &LegalDocument) -> String {
        let mut output = String::new();

        // Document header
        if self.config.verbosity != Verbosity::Brief {
            writeln!(output, "# Legal Document").unwrap();
            writeln!(output).unwrap();
        }

        // Namespace
        if let Some(namespace) = &doc.namespace {
            if self.config.include_metadata {
                writeln!(
                    output,
                    "This document belongs to the namespace: {}",
                    namespace.path
                )
                .unwrap();
                writeln!(output).unwrap();
            }
        }

        // Imports
        if !doc.imports.is_empty() && self.config.include_metadata {
            writeln!(output, "## Dependencies").unwrap();
            for import in &doc.imports {
                writeln!(output, "- {}", self.generate_import(import)).unwrap();
            }
            writeln!(output).unwrap();
        }

        // Statutes
        if !doc.statutes.is_empty() {
            if self.config.verbosity != Verbosity::Brief {
                writeln!(output, "## Statutes").unwrap();
                writeln!(output).unwrap();
            }

            for statute in &doc.statutes {
                output.push_str(&self.generate_statute(statute));
                writeln!(output).unwrap();
            }
        }

        // Exports
        if !doc.exports.is_empty() && self.config.include_metadata {
            writeln!(output, "## Public Exports").unwrap();
            for export in &doc.exports {
                writeln!(output, "- {}", self.generate_export(export)).unwrap();
            }
        }

        output
    }

    /// Generate natural language for a single statute
    pub fn generate_statute(&self, statute: &StatuteNode) -> String {
        let mut output = String::new();

        // Title
        match self.config.verbosity {
            Verbosity::Brief => {
                write!(output, "{}: ", statute.title).unwrap();
            }
            _ => {
                writeln!(output, "### {} (ID: {})", statute.title, statute.id).unwrap();
                writeln!(output).unwrap();
            }
        }

        // Visibility
        if self.config.include_metadata
            && statute.visibility != crate::module_system::Visibility::Public
        {
            writeln!(output, "*This statute is private and not exported.*").unwrap();
            writeln!(output).unwrap();
        }

        // Priority
        if let Some(priority) = statute.priority {
            writeln!(output, "**Priority:** {}", priority).unwrap();
            writeln!(output).unwrap();
        }

        // Scope
        if let Some(scope) = &statute.scope {
            writeln!(
                output,
                "**Scope:** Applies to entities of type: {}",
                scope.entity_types.join(", ")
            )
            .unwrap();
            if let Some(desc) = &scope.description {
                writeln!(output, "  {}", desc).unwrap();
            }
            writeln!(output).unwrap();
        }

        // Defaults
        if !statute.defaults.is_empty() && self.config.verbosity != Verbosity::Brief {
            writeln!(output, "**Default Values:**").unwrap();
            for default in &statute.defaults {
                writeln!(
                    output,
                    "- If {} is not specified, it defaults to {}",
                    default.field,
                    self.value_to_string(&default.value)
                )
                .unwrap();
            }
            writeln!(output).unwrap();
        }

        // Main rule
        if !statute.conditions.is_empty() || !statute.effects.is_empty() {
            output.push_str(&self.generate_rule(statute));
        }

        // Exceptions
        if !statute.exceptions.is_empty() {
            output.push_str(&self.generate_exceptions(&statute.exceptions));
        }

        // Discretion
        if let Some(discretion) = &statute.discretion {
            writeln!(output, "**Discretionary Considerations:**").unwrap();
            writeln!(output, "- {}", discretion).unwrap();
            writeln!(output).unwrap();
        }

        // Amendments
        if !statute.amendments.is_empty() && self.config.include_metadata {
            output.push_str(&self.generate_amendments(&statute.amendments));
        }

        // Delegates
        if !statute.delegates.is_empty() && self.config.include_metadata {
            output.push_str(&self.generate_delegates(&statute.delegates));
        }

        // Constraints
        if !statute.constraints.is_empty() && self.config.include_metadata {
            output.push_str(&self.generate_constraints(&statute.constraints));
        }

        // Relationships
        if self.config.include_metadata {
            output.push_str(&self.generate_relationships(statute));
        }

        output
    }

    fn generate_rule(&self, statute: &StatuteNode) -> String {
        let mut output = String::new();

        if statute.conditions.is_empty() {
            // Unconditional effects
            writeln!(output, "This statute provides:").unwrap();
            for effect in &statute.effects {
                writeln!(output, "- {}", self.generate_effect(effect)).unwrap();
            }
        } else {
            // Conditional effects
            match self.config.verbosity {
                Verbosity::Brief => {
                    write!(output, "If ").unwrap();
                    output.push_str(&self.generate_conditions(&statute.conditions, true));
                    write!(output, ", then ").unwrap();
                    if statute.effects.len() == 1 {
                        output.push_str(&self.generate_effect(&statute.effects[0]));
                    } else {
                        write!(output, "{} outcomes apply", statute.effects.len()).unwrap();
                    }
                }
                _ => {
                    writeln!(output, "**Conditions:**").unwrap();
                    writeln!(
                        output,
                        "{}",
                        self.generate_conditions(&statute.conditions, false)
                    )
                    .unwrap();
                    writeln!(output).unwrap();

                    if !statute.effects.is_empty() {
                        writeln!(output, "**Then:**").unwrap();
                        for effect in &statute.effects {
                            writeln!(output, "- {}", self.generate_effect(effect)).unwrap();
                        }
                    }
                }
            }
        }

        writeln!(output).unwrap();
        output
    }

    fn generate_conditions(&self, conditions: &[ConditionNode], inline: bool) -> String {
        if conditions.is_empty() {
            return "no conditions".to_string();
        }

        if conditions.len() == 1 {
            return self.generate_condition(&conditions[0], inline);
        }

        let parts: Vec<String> = conditions
            .iter()
            .map(|c| self.generate_condition(c, inline))
            .collect();

        if inline {
            parts.join(" and ")
        } else {
            parts
                .iter()
                .enumerate()
                .map(|(i, p)| format!("{}. {}", i + 1, p))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }

    fn generate_condition(&self, condition: &ConditionNode, _inline: bool) -> String {
        match condition {
            ConditionNode::And(left, right) => {
                format!(
                    "{} and {}",
                    self.generate_condition(left, _inline),
                    self.generate_condition(right, _inline)
                )
            }
            ConditionNode::Or(left, right) => {
                format!(
                    "({} or {})",
                    self.generate_condition(left, _inline),
                    self.generate_condition(right, _inline)
                )
            }
            ConditionNode::Not(inner) => {
                format!("not ({})", self.generate_condition(inner, _inline))
            }
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => self.generate_comparison(field, operator, value),
            ConditionNode::Between { field, min, max } => {
                format!(
                    "the {} is between {} and {}",
                    field,
                    self.value_to_string(min),
                    self.value_to_string(max)
                )
            }
            ConditionNode::In { field, values } => {
                let value_list = values
                    .iter()
                    .map(|v| self.value_to_string(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("the {} is one of: {}", field, value_list)
            }
            ConditionNode::Like { field, pattern } => {
                format!("the {} matches the pattern '{}'", field, pattern)
            }
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => {
                format!(
                    "the {} matches the regular expression /{}/",
                    field, regex_pattern
                )
            }
            ConditionNode::HasAttribute { key } => {
                format!("has the attribute '{}'", key)
            }
            ConditionNode::InRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => {
                let min_op = if *inclusive_min { ">=" } else { ">" };
                let max_op = if *inclusive_max { "<=" } else { "<" };
                format!(
                    "the {} is {} {} and {} {}",
                    field,
                    min_op,
                    self.value_to_string(min),
                    max_op,
                    self.value_to_string(max)
                )
            }
            ConditionNode::NotInRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => {
                let min_op = if *inclusive_min { ">=" } else { ">" };
                let max_op = if *inclusive_max { "<=" } else { "<" };
                format!(
                    "the {} is NOT {} {} and {} {}",
                    field,
                    min_op,
                    self.value_to_string(min),
                    max_op,
                    self.value_to_string(max)
                )
            }
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                let field_name = match field {
                    TemporalField::CurrentDate => "current date",
                    TemporalField::DateField(name) => name,
                };
                format!(
                    "the {} {} {}",
                    field_name,
                    operator,
                    self.value_to_string(value)
                )
            }
        }
    }

    fn generate_comparison(&self, field: &str, op: &str, value: &ConditionValue) -> String {
        let op_word = match op {
            "=" | "==" => "equals",
            "!=" | "<>" => "does not equal",
            "<" => "is less than",
            "<=" => "is less than or equal to",
            ">" => "is greater than",
            ">=" => "is greater than or equal to",
            _ => op,
        };

        format!("the {} {} {}", field, op_word, self.value_to_string(value))
    }

    fn generate_effect(&self, effect: &EffectNode) -> String {
        let action = match effect.effect_type.as_str() {
            "GRANT" => "grants",
            "REVOKE" => "revokes",
            "OBLIGATION" => "requires",
            "PROHIBITION" => "prohibits",
            _ => &effect.effect_type,
        };

        let mut desc = format!("{} {}", action, effect.description);

        if !effect.parameters.is_empty() {
            let params: Vec<String> = effect
                .parameters
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            desc.push_str(&format!(" ({})", params.join(", ")));
        }

        desc
    }

    fn generate_exceptions(&self, exceptions: &[ExceptionNode]) -> String {
        let mut output = String::new();

        writeln!(output, "**Exceptions:**").unwrap();
        for (i, exception) in exceptions.iter().enumerate() {
            write!(output, "{}. ", i + 1).unwrap();
            if !exception.conditions.is_empty() {
                write!(
                    output,
                    "When {}, ",
                    self.generate_conditions(&exception.conditions, true)
                )
                .unwrap();
            }
            writeln!(output, "{}", exception.description).unwrap();
        }
        writeln!(output).unwrap();

        output
    }

    fn generate_amendments(&self, amendments: &[AmendmentNode]) -> String {
        let mut output = String::new();

        writeln!(output, "**Amendment History:**").unwrap();
        for amendment in amendments {
            write!(output, "- Amends statute '{}'", amendment.target_id).unwrap();
            if let Some(version) = amendment.version {
                write!(output, " (version {})", version).unwrap();
            }
            if let Some(date) = &amendment.date {
                write!(output, " effective {}", date).unwrap();
            }
            writeln!(output, ": {}", amendment.description).unwrap();
        }
        writeln!(output).unwrap();

        output
    }

    fn generate_delegates(&self, delegates: &[DelegateNode]) -> String {
        let mut output = String::new();

        writeln!(output, "**Delegation:**").unwrap();
        for delegate in delegates {
            write!(output, "- Delegates to statute '{}'", delegate.target_id).unwrap();
            if !delegate.conditions.is_empty() {
                write!(
                    output,
                    " when {}",
                    self.generate_conditions(&delegate.conditions, true)
                )
                .unwrap();
            }
            writeln!(output, ": {}", delegate.description).unwrap();
        }
        writeln!(output).unwrap();

        output
    }

    fn generate_constraints(&self, constraints: &[ConstraintNode]) -> String {
        let mut output = String::new();

        writeln!(output, "**Constraints:**").unwrap();
        for constraint in constraints {
            write!(output, "- {}: ", constraint.name).unwrap();
            write!(
                output,
                "{}",
                self.generate_condition(&constraint.condition, true)
            )
            .unwrap();
            if let Some(desc) = &constraint.description {
                write!(output, " ({})", desc).unwrap();
            }
            writeln!(output).unwrap();
        }
        writeln!(output).unwrap();

        output
    }

    fn generate_relationships(&self, statute: &StatuteNode) -> String {
        let mut output = String::new();
        let mut has_relationships = false;

        if !statute.requires.is_empty() {
            writeln!(output, "**Requires:** {}", statute.requires.join(", ")).unwrap();
            has_relationships = true;
        }

        if !statute.supersedes.is_empty() {
            writeln!(output, "**Supersedes:** {}", statute.supersedes.join(", ")).unwrap();
            has_relationships = true;
        }

        if has_relationships {
            writeln!(output).unwrap();
        }

        output
    }

    fn generate_import(&self, import: &ImportNode) -> String {
        match &import.kind {
            crate::module_system::ImportKind::Simple => format!("Imports from '{}'", import.path),
            crate::module_system::ImportKind::Wildcard => {
                format!("Imports all items from '{}'", import.path)
            }
            crate::module_system::ImportKind::Selective(items) => {
                format!("Imports {} from '{}'", items.join(", "), import.path)
            }
        }
    }

    fn generate_export(&self, export: &crate::module_system::ExportNode) -> String {
        if export.items.is_empty() {
            "Exports all public items".to_string()
        } else {
            let mut result = format!("Exports: {}", export.items.join(", "));
            if let Some(from) = &export.from {
                result.push_str(&format!(" (from {})", from));
            }
            result
        }
    }

    fn value_to_string(&self, value: &ConditionValue) -> String {
        match value {
            ConditionValue::Number(n) => n.to_string(),
            ConditionValue::String(s) => format!("\"{}\"", s),
            ConditionValue::Boolean(b) => b.to_string(),
            ConditionValue::Date(d) => d.clone(),
            ConditionValue::SetExpr(set) => self.set_expr_to_string(set),
        }
    }

    fn set_expr_to_string(&self, expr: &SetExpression) -> String {
        match expr {
            SetExpression::Values(values) => {
                let items: Vec<String> = values.iter().map(|v| self.value_to_string(v)).collect();
                format!("[{}]", items.join(", "))
            }
            SetExpression::Union(left, right) => {
                format!(
                    "({} ∪ {})",
                    self.set_expr_to_string(left),
                    self.set_expr_to_string(right)
                )
            }
            SetExpression::Intersect(left, right) => {
                format!(
                    "({} ∩ {})",
                    self.set_expr_to_string(left),
                    self.set_expr_to_string(right)
                )
            }
            SetExpression::Difference(left, right) => {
                format!(
                    "({} \\ {})",
                    self.set_expr_to_string(left),
                    self.set_expr_to_string(right)
                )
            }
        }
    }
}

impl Default for NLGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_statute_generation() {
        let statute = StatuteNode {
            id: "test-001".to_string(),
            title: "Test Statute".to_string(),
            visibility: crate::module_system::Visibility::Public,
            conditions: vec![ConditionNode::Comparison {
                field: "age".to_string(),
                operator: ">=".to_string(),
                value: ConditionValue::Number(18),
            }],
            effects: vec![EffectNode {
                effect_type: "GRANT".to_string(),
                description: "voting rights".to_string(),
                parameters: vec![],
            }],
            defaults: vec![],
            exceptions: vec![],
            discretion: None,
            amendments: vec![],
            requires: vec![],
            supersedes: vec![],
            delegates: vec![],
            priority: None,
            scope: None,
            constraints: vec![],
        };

        let generator = NLGenerator::new();
        let output = generator.generate_statute(&statute);

        assert!(output.contains("Test Statute"));
        assert!(output.contains("age is greater than or equal to 18"));
        assert!(output.contains("grants voting rights"));
    }

    #[test]
    fn test_brief_verbosity() {
        let statute = StatuteNode {
            id: "test-002".to_string(),
            title: "Brief Test".to_string(),
            visibility: crate::module_system::Visibility::Public,
            conditions: vec![ConditionNode::HasAttribute {
                key: "citizenship".to_string(),
            }],
            effects: vec![EffectNode {
                effect_type: "GRANT".to_string(),
                description: "benefits".to_string(),
                parameters: vec![],
            }],
            defaults: vec![],
            exceptions: vec![],
            discretion: None,
            amendments: vec![],
            requires: vec![],
            supersedes: vec![],
            delegates: vec![],
            priority: None,
            scope: None,
            constraints: vec![],
        };

        let config = NLConfig {
            verbosity: Verbosity::Brief,
            ..Default::default()
        };
        let generator = NLGenerator::with_config(config);
        let output = generator.generate_statute(&statute);

        assert!(output.contains("If"));
        assert!(output.contains("then"));
    }

    #[test]
    fn test_complex_conditions() {
        let statute = StatuteNode {
            id: "test-003".to_string(),
            title: "Complex Conditions".to_string(),
            visibility: crate::module_system::Visibility::Public,
            conditions: vec![ConditionNode::And(
                Box::new(ConditionNode::Between {
                    field: "age".to_string(),
                    min: ConditionValue::Number(18),
                    max: ConditionValue::Number(65),
                }),
                Box::new(ConditionNode::HasAttribute {
                    key: "citizenship".to_string(),
                }),
            )],
            effects: vec![EffectNode {
                effect_type: "GRANT".to_string(),
                description: "benefits".to_string(),
                parameters: vec![],
            }],
            defaults: vec![],
            exceptions: vec![],
            discretion: None,
            amendments: vec![],
            requires: vec![],
            supersedes: vec![],
            delegates: vec![],
            priority: None,
            scope: None,
            constraints: vec![],
        };

        let generator = NLGenerator::new();
        let output = generator.generate_statute(&statute);

        assert!(output.contains("between"));
        assert!(output.contains("and"));
        assert!(output.contains("has the attribute"));
    }
}
