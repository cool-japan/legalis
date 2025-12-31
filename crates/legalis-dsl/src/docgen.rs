//! Documentation generation from legal document AST.
//!
//! This module provides utilities to generate human-readable documentation
//! in various formats (Markdown, HTML, etc.) from legal documents.

use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use std::fmt::Write;

/// Documentation generator trait.
pub trait DocGenerator {
    /// Generates documentation for a legal document.
    fn generate(&self, doc: &LegalDocument) -> String;

    /// Returns the output format name.
    fn format_name(&self) -> &str;
}

/// Markdown documentation generator.
pub struct MarkdownGenerator {
    /// Include table of contents
    pub include_toc: bool,
    /// Include cross-references
    pub include_cross_refs: bool,
    /// Maximum heading level
    pub max_heading_level: usize,
}

impl Default for MarkdownGenerator {
    fn default() -> Self {
        Self {
            include_toc: true,
            include_cross_refs: true,
            max_heading_level: 3,
        }
    }
}

impl MarkdownGenerator {
    /// Creates a new Markdown generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a table of contents.
    fn generate_toc(&self, doc: &LegalDocument) -> String {
        let mut toc = String::new();

        writeln!(&mut toc, "## Table of Contents\n").unwrap();

        for (i, statute) in doc.statutes.iter().enumerate() {
            writeln!(
                &mut toc,
                "{}. [{}](#statute-{})",
                i + 1,
                statute.title,
                statute.id
            )
            .unwrap();
        }

        writeln!(&mut toc).unwrap();
        toc
    }

    /// Generates documentation for a statute.
    fn generate_statute(&self, statute: &StatuteNode) -> String {
        let mut doc = String::new();

        // Heading
        writeln!(
            &mut doc,
            "### {} {{#statute-{}}}\n",
            statute.title, statute.id
        )
        .unwrap();

        // ID badge
        writeln!(&mut doc, "**ID:** `{}`\n", statute.id).unwrap();

        // Dependencies
        if !statute.requires.is_empty() {
            writeln!(&mut doc, "**Requires:**").unwrap();
            for req in &statute.requires {
                writeln!(&mut doc, "- [`{}`](#statute-{})", req, req).unwrap();
            }
            writeln!(&mut doc).unwrap();
        }

        // Supersedes
        if !statute.supersedes.is_empty() {
            writeln!(&mut doc, "**Supersedes:**").unwrap();
            for sup in &statute.supersedes {
                writeln!(&mut doc, "- [`{}`](#statute-{})", sup, sup).unwrap();
            }
            writeln!(&mut doc).unwrap();
        }

        // Conditions
        if !statute.conditions.is_empty() {
            writeln!(&mut doc, "#### Conditions\n").unwrap();
            for (i, condition) in statute.conditions.iter().enumerate() {
                writeln!(&mut doc, "{}. {}", i + 1, self.format_condition(condition)).unwrap();
            }
            writeln!(&mut doc).unwrap();
        }

        // Effects
        if !statute.effects.is_empty() {
            writeln!(&mut doc, "#### Effects\n").unwrap();
            for effect in &statute.effects {
                writeln!(
                    &mut doc,
                    "- **{}**: {}",
                    effect.effect_type, effect.description
                )
                .unwrap();
            }
            writeln!(&mut doc).unwrap();
        }

        // Discretion
        if let Some(discretion) = &statute.discretion {
            writeln!(&mut doc, "#### Discretion\n").unwrap();
            writeln!(&mut doc, "{}\n", discretion).unwrap();
        }

        // Exceptions
        if !statute.exceptions.is_empty() {
            writeln!(&mut doc, "#### Exceptions\n").unwrap();
            for (i, exception) in statute.exceptions.iter().enumerate() {
                writeln!(&mut doc, "{}. {}", i + 1, exception.description).unwrap();
                if !exception.conditions.is_empty() {
                    writeln!(&mut doc, "   - When:").unwrap();
                    for cond in &exception.conditions {
                        writeln!(&mut doc, "     - {}", self.format_condition(cond)).unwrap();
                    }
                }
            }
            writeln!(&mut doc).unwrap();
        }

        // Amendments
        if !statute.amendments.is_empty() {
            writeln!(&mut doc, "#### Amendment History\n").unwrap();
            for amendment in &statute.amendments {
                write!(&mut doc, "- **{}**", amendment.target_id).unwrap();
                if let Some(version) = amendment.version {
                    write!(&mut doc, " (v{})", version).unwrap();
                }
                if let Some(date) = &amendment.date {
                    write!(&mut doc, " [{}]", date).unwrap();
                }
                writeln!(&mut doc, ": {}", amendment.description).unwrap();
            }
            writeln!(&mut doc).unwrap();
        }

        // Defaults
        if !statute.defaults.is_empty() {
            writeln!(&mut doc, "#### Default Values\n").unwrap();
            writeln!(&mut doc, "| Field | Value |").unwrap();
            writeln!(&mut doc, "|-------|-------|").unwrap();
            for default in &statute.defaults {
                writeln!(
                    &mut doc,
                    "| `{}` | {} |",
                    default.field,
                    self.format_value(&default.value)
                )
                .unwrap();
            }
            writeln!(&mut doc).unwrap();
        }

        writeln!(&mut doc, "---\n").unwrap();
        doc
    }

    /// Formats a condition for display.
    fn format_condition(&self, condition: &ConditionNode) -> String {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                format!("`{}` {} {}", field, operator, self.format_value(value))
            }
            ConditionNode::HasAttribute { key } => {
                format!("Has attribute `{}`", key)
            }
            ConditionNode::Between { field, min, max } => {
                format!(
                    "`{}` BETWEEN {} AND {}",
                    field,
                    self.format_value(min),
                    self.format_value(max)
                )
            }
            ConditionNode::In { field, values } => {
                let vals: Vec<_> = values.iter().map(|v| self.format_value(v)).collect();
                format!("`{}` IN ({})", field, vals.join(", "))
            }
            ConditionNode::Like { field, pattern } => {
                format!("`{}` LIKE \"{}\"", field, pattern)
            }
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => {
                format!("`{}` MATCHES `{}`", field, regex_pattern)
            }
            ConditionNode::InRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => {
                let min_bracket = if *inclusive_min { "[" } else { "(" };
                let max_bracket = if *inclusive_max { "]" } else { ")" };
                format!(
                    "`{}` IN RANGE {}{}, {}{}",
                    field,
                    min_bracket,
                    self.format_value(min),
                    self.format_value(max),
                    max_bracket
                )
            }
            ConditionNode::NotInRange {
                field, min, max, ..
            } => {
                format!(
                    "`{}` NOT IN RANGE ({}, {})",
                    field,
                    self.format_value(min),
                    self.format_value(max)
                )
            }
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                format!("{:?} {} {}", field, operator, self.format_value(value))
            }
            ConditionNode::And(left, right) => {
                format!(
                    "({} AND {})",
                    self.format_condition(left),
                    self.format_condition(right)
                )
            }
            ConditionNode::Or(left, right) => {
                format!(
                    "({} OR {})",
                    self.format_condition(left),
                    self.format_condition(right)
                )
            }
            ConditionNode::Not(inner) => {
                format!("NOT ({})", self.format_condition(inner))
            }
        }
    }

    /// Formats a value for display.
    fn format_value(&self, value: &ConditionValue) -> String {
        match value {
            ConditionValue::Number(n) => n.to_string(),
            ConditionValue::String(s) => format!("\"{}\"", s),
            ConditionValue::Boolean(b) => b.to_string(),
            ConditionValue::Date(d) => format!("**{}**", d),
            ConditionValue::SetExpr(_) => "{...}".to_string(),
        }
    }

    /// Generates cross-reference section.
    fn generate_cross_refs(&self, doc: &LegalDocument) -> String {
        let mut refs = String::new();

        writeln!(&mut refs, "## Cross-References\n").unwrap();

        // Build dependency map
        let mut depends_on: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        let mut depended_by: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for statute in &doc.statutes {
            for req in &statute.requires {
                depends_on
                    .entry(statute.id.clone())
                    .or_default()
                    .push(req.clone());
                depended_by
                    .entry(req.clone())
                    .or_default()
                    .push(statute.id.clone());
            }
        }

        if !depends_on.is_empty() {
            writeln!(&mut refs, "### Dependency Graph\n").unwrap();
            writeln!(&mut refs, "```mermaid").unwrap();
            writeln!(&mut refs, "graph TD").unwrap();
            for (statute, deps) in &depends_on {
                for dep in deps {
                    writeln!(
                        &mut refs,
                        "    {}[{}] --> {}[{}]",
                        statute.replace('-', "_"),
                        statute,
                        dep.replace('-', "_"),
                        dep
                    )
                    .unwrap();
                }
            }
            writeln!(&mut refs, "```\n").unwrap();
        }

        refs
    }
}

impl DocGenerator for MarkdownGenerator {
    fn generate(&self, doc: &LegalDocument) -> String {
        let mut output = String::new();

        // Title
        writeln!(&mut output, "# Legal Document\n").unwrap();

        // Metadata
        if !doc.imports.is_empty() {
            writeln!(&mut output, "## Imports\n").unwrap();
            for import in &doc.imports {
                if let Some(alias) = &import.alias {
                    writeln!(&mut output, "- `{}` as `{}`", import.path, alias).unwrap();
                } else {
                    writeln!(&mut output, "- `{}`", import.path).unwrap();
                }
            }
            writeln!(&mut output).unwrap();
        }

        // Table of contents
        if self.include_toc {
            output.push_str(&self.generate_toc(doc));
        }

        // Statutes
        writeln!(&mut output, "## Statutes\n").unwrap();

        for statute in &doc.statutes {
            output.push_str(&self.generate_statute(statute));
        }

        // Cross-references
        if self.include_cross_refs {
            output.push_str(&self.generate_cross_refs(doc));
        }

        // Footer
        writeln!(&mut output, "---\n").unwrap();
        writeln!(
            &mut output,
            "*Generated by legalis-dsl documentation generator*"
        )
        .unwrap();

        output
    }

    fn format_name(&self) -> &str {
        "Markdown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{EffectNode, ImportNode};

    fn sample_document() -> LegalDocument {
        LegalDocument {
            imports: vec![ImportNode {
                path: "common/definitions.legalis".to_string(),
                alias: Some("defs".to_string()),
            }],
            statutes: vec![StatuteNode {
                id: "voting-rights".to_string(),
                title: "Voting Rights".to_string(),
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
                    description: "Right to vote in elections".to_string(),
                    parameters: vec![],
                }],
                discretion: Some("Consider residency requirements".to_string()),
                exceptions: vec![],
                amendments: vec![],
                supersedes: vec![],
                defaults: vec![],
                requires: vec![],
                delegates: vec![],
                scope: None,
                constraints: vec![],
                priority: None,
            }],
        }
    }

    #[test]
    fn test_markdown_generation() {
        let doc = sample_document();
        let generator = MarkdownGenerator::new();
        let markdown = generator.generate(&doc);

        assert!(markdown.contains("# Legal Document"));
        assert!(markdown.contains("## Table of Contents"));
        assert!(markdown.contains("Voting Rights"));
        assert!(markdown.contains("`age` >= 18"));
        assert!(markdown.contains("Has attribute `citizen`"));
        assert!(markdown.contains("Right to vote"));
    }

    #[test]
    fn test_format_condition() {
        let generator = MarkdownGenerator::new();

        let condition = ConditionNode::Between {
            field: "age".to_string(),
            min: ConditionValue::Number(18),
            max: ConditionValue::Number(65),
        };

        let formatted = generator.format_condition(&condition);
        assert!(formatted.contains("age"));
        assert!(formatted.contains("BETWEEN"));
        assert!(formatted.contains("18"));
        assert!(formatted.contains("65"));
    }
}
