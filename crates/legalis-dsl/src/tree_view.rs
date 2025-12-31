//! Tree-view formatter for visualizing statute structure.
//!
//! This module provides utilities to display legal documents and statutes
//! in a hierarchical tree format for better readability and understanding.

use crate::ast::{
    AmendmentNode, ConditionNode, ConditionValue, ExceptionNode, LegalDocument, StatuteNode,
};

/// Tree formatter with customizable styling.
#[derive(Debug, Clone)]
pub struct TreeFormatter {
    /// Indentation string for each level
    indent: String,
    /// Symbol for tree branches
    branch: String,
    /// Symbol for tree continuation
    continuation: String,
    /// Symbol for last item in a branch
    last_branch: String,
    /// Whether to use color output (ANSI codes)
    use_color: bool,
}

impl Default for TreeFormatter {
    fn default() -> Self {
        Self {
            indent: "  ".to_string(),
            branch: "├─ ".to_string(),
            continuation: "│  ".to_string(),
            last_branch: "└─ ".to_string(),
            use_color: false,
        }
    }
}

impl TreeFormatter {
    /// Creates a new tree formatter with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables or disables color output.
    pub fn with_color(mut self, use_color: bool) -> Self {
        self.use_color = use_color;
        self
    }

    /// Sets custom indentation.
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent = indent.into();
        self
    }

    /// Formats a legal document as a tree.
    pub fn format_document(&self, doc: &LegalDocument) -> String {
        let mut output = String::new();

        // Document header
        output.push_str(&self.colorize("📜 Legal Document", Color::Cyan, true));
        output.push('\n');

        // Imports
        if !doc.imports.is_empty() {
            output.push_str(&format!("{}Imports ({})\n", self.branch, doc.imports.len()));
            for (i, import) in doc.imports.iter().enumerate() {
                let is_last = i == doc.imports.len() - 1;
                let prefix = if is_last {
                    &self.last_branch
                } else {
                    &self.branch
                };

                let import_str = if let Some(ref alias) = import.alias {
                    format!("{} as {}", import.path, alias)
                } else {
                    import.path.clone()
                };

                output.push_str(&format!(
                    "{}{}{}\n",
                    self.indent,
                    prefix,
                    self.colorize(&import_str, Color::Green, false)
                ));
            }
        }

        // Statutes
        output.push_str(&format!(
            "{}Statutes ({})\n",
            if doc.imports.is_empty() {
                &self.last_branch
            } else {
                &self.branch
            },
            doc.statutes.len()
        ));

        for (i, statute) in doc.statutes.iter().enumerate() {
            let is_last = i == doc.statutes.len() - 1;
            output.push_str(&self.format_statute_node(statute, &self.indent, is_last));
        }

        output
    }

    /// Formats a single statute node.
    fn format_statute_node(&self, statute: &StatuteNode, prefix: &str, is_last: bool) -> String {
        let mut output = String::new();

        let branch_prefix = if is_last {
            &self.last_branch
        } else {
            &self.branch
        };

        let continuation_prefix = if is_last { "   " } else { &self.continuation };

        // Statute header
        output.push_str(&format!(
            "{}{}{}: \"{}\"\n",
            prefix,
            branch_prefix,
            self.colorize(&statute.id, Color::Yellow, true),
            self.colorize(&statute.title, Color::White, false)
        ));

        let child_prefix = format!("{}{}", prefix, continuation_prefix);

        // Conditions
        if !statute.conditions.is_empty() {
            output.push_str(&format!(
                "{}{}Conditions ({})\n",
                child_prefix,
                self.branch,
                statute.conditions.len()
            ));
            for (i, condition) in statute.conditions.iter().enumerate() {
                let is_last_cond = i == statute.conditions.len() - 1;
                output.push_str(&self.format_condition(
                    condition,
                    &format!("{}{}", child_prefix, self.indent),
                    is_last_cond,
                ));
            }
        }

        // Effects
        if !statute.effects.is_empty() {
            output.push_str(&format!(
                "{}{}Effects ({})\n",
                child_prefix,
                self.branch,
                statute.effects.len()
            ));
            for (i, effect) in statute.effects.iter().enumerate() {
                let is_last_effect = i == statute.effects.len() - 1;
                let effect_prefix = if is_last_effect {
                    &self.last_branch
                } else {
                    &self.branch
                };
                output.push_str(&format!(
                    "{}{}{}{}: {}\n",
                    child_prefix,
                    self.indent,
                    effect_prefix,
                    self.colorize(&effect.effect_type, Color::Magenta, false),
                    effect.description
                ));
            }
        }

        // Requires
        if !statute.requires.is_empty() {
            output.push_str(&format!(
                "{}{}Requires ({}): {}\n",
                child_prefix,
                self.branch,
                statute.requires.len(),
                statute.requires.join(", ")
            ));
        }

        // Supersedes
        if !statute.supersedes.is_empty() {
            output.push_str(&format!(
                "{}{}Supersedes ({}): {}\n",
                child_prefix,
                self.branch,
                statute.supersedes.len(),
                statute.supersedes.join(", ")
            ));
        }

        // Discretion
        if let Some(ref discretion) = statute.discretion {
            output.push_str(&format!(
                "{}{}Discretion: {}\n",
                child_prefix,
                self.branch,
                self.colorize(discretion, Color::Blue, false)
            ));
        }

        // Exceptions
        if !statute.exceptions.is_empty() {
            output.push_str(&format!(
                "{}{}Exceptions ({})\n",
                child_prefix,
                self.branch,
                statute.exceptions.len()
            ));
            for (i, exception) in statute.exceptions.iter().enumerate() {
                let is_last_exc = i == statute.exceptions.len() - 1;
                output.push_str(&self.format_exception(
                    exception,
                    &format!("{}{}", child_prefix, self.indent),
                    is_last_exc,
                ));
            }
        }

        // Amendments
        if !statute.amendments.is_empty() {
            output.push_str(&format!(
                "{}{}Amendments ({})\n",
                child_prefix,
                self.branch,
                statute.amendments.len()
            ));
            for (i, amendment) in statute.amendments.iter().enumerate() {
                let is_last_amend = i == statute.amendments.len() - 1;
                output.push_str(&self.format_amendment(
                    amendment,
                    &format!("{}{}", child_prefix, self.indent),
                    is_last_amend,
                ));
            }
        }

        // Defaults
        if !statute.defaults.is_empty() {
            output.push_str(&format!(
                "{}{}Defaults ({})\n",
                child_prefix,
                if statute.amendments.is_empty() {
                    &self.last_branch
                } else {
                    &self.branch
                },
                statute.defaults.len()
            ));
            for (i, default) in statute.defaults.iter().enumerate() {
                let is_last_def = i == statute.defaults.len() - 1;
                let def_prefix = if is_last_def {
                    &self.last_branch
                } else {
                    &self.branch
                };
                output.push_str(&format!(
                    "{}{}{}{} = {}\n",
                    child_prefix,
                    self.indent,
                    def_prefix,
                    default.field,
                    self.format_value(&default.value)
                ));
            }
        }

        output
    }

    /// Formats a condition node.
    fn format_condition(&self, condition: &ConditionNode, prefix: &str, is_last: bool) -> String {
        let mut output = String::new();

        let branch_prefix = if is_last {
            &self.last_branch
        } else {
            &self.branch
        };

        let continuation_prefix = if is_last { "   " } else { &self.continuation };

        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                output.push_str(&format!(
                    "{}{}{} {} {}\n",
                    prefix,
                    branch_prefix,
                    self.colorize(field, Color::Cyan, false),
                    operator,
                    self.format_value(value)
                ));
            }
            ConditionNode::Between { field, min, max } => {
                output.push_str(&format!(
                    "{}{}{} BETWEEN {} AND {}\n",
                    prefix,
                    branch_prefix,
                    self.colorize(field, Color::Cyan, false),
                    self.format_value(min),
                    self.format_value(max)
                ));
            }
            ConditionNode::In { field, values } => {
                output.push_str(&format!(
                    "{}{}{} IN [{}]\n",
                    prefix,
                    branch_prefix,
                    self.colorize(field, Color::Cyan, false),
                    values
                        .iter()
                        .map(|v| self.format_value(v))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            ConditionNode::HasAttribute { key } => {
                output.push_str(&format!(
                    "{}{}HAS {}\n",
                    prefix,
                    branch_prefix,
                    self.colorize(key, Color::Cyan, false)
                ));
            }
            ConditionNode::And(left, right) => {
                output.push_str(&format!("{}{}AND\n", prefix, branch_prefix));
                let child_prefix = format!("{}{}", prefix, continuation_prefix);
                output.push_str(&self.format_condition(left, &child_prefix, false));
                output.push_str(&self.format_condition(right, &child_prefix, true));
            }
            ConditionNode::Or(left, right) => {
                output.push_str(&format!("{}{}OR\n", prefix, branch_prefix));
                let child_prefix = format!("{}{}", prefix, continuation_prefix);
                output.push_str(&self.format_condition(left, &child_prefix, false));
                output.push_str(&self.format_condition(right, &child_prefix, true));
            }
            ConditionNode::Not(inner) => {
                output.push_str(&format!("{}{}NOT\n", prefix, branch_prefix));
                let child_prefix = format!("{}{}", prefix, continuation_prefix);
                output.push_str(&self.format_condition(inner, &child_prefix, true));
            }
            ConditionNode::Like { field, pattern } => {
                output.push_str(&format!(
                    "{}{}{} LIKE \"{}\"\n",
                    prefix,
                    branch_prefix,
                    self.colorize(field, Color::Cyan, false),
                    pattern
                ));
            }
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => {
                output.push_str(&format!(
                    "{}{}{} MATCHES /{}/\n",
                    prefix,
                    branch_prefix,
                    self.colorize(field, Color::Cyan, false),
                    regex_pattern
                ));
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
                output.push_str(&format!(
                    "{}{}{} IN_RANGE {}{}..{}{}\n",
                    prefix,
                    branch_prefix,
                    self.colorize(field, Color::Cyan, false),
                    min_bracket,
                    self.format_value(min),
                    self.format_value(max),
                    max_bracket
                ));
            }
            ConditionNode::NotInRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => {
                let min_bracket = if *inclusive_min { "[" } else { "(" };
                let max_bracket = if *inclusive_max { "]" } else { ")" };
                output.push_str(&format!(
                    "{}{}{} NOT_IN_RANGE {}{}..{}{}\n",
                    prefix,
                    branch_prefix,
                    self.colorize(field, Color::Cyan, false),
                    min_bracket,
                    self.format_value(min),
                    self.format_value(max),
                    max_bracket
                ));
            }
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                output.push_str(&format!(
                    "{}{}{:?} {} {}\n",
                    prefix,
                    branch_prefix,
                    field,
                    operator,
                    self.format_value(value)
                ));
            }
        }

        output
    }

    /// Formats an exception node.
    fn format_exception(&self, exception: &ExceptionNode, prefix: &str, is_last: bool) -> String {
        let mut output = String::new();

        let branch_prefix = if is_last {
            &self.last_branch
        } else {
            &self.branch
        };

        output.push_str(&format!(
            "{}{}{}\n",
            prefix,
            branch_prefix,
            self.colorize(&exception.description, Color::Red, false)
        ));

        if !exception.conditions.is_empty() {
            let continuation_prefix = if is_last { "   " } else { &self.continuation };
            let child_prefix = format!("{}{}", prefix, continuation_prefix);
            output.push_str(&format!("{}{}Conditions:\n", child_prefix, self.branch));
            for (i, condition) in exception.conditions.iter().enumerate() {
                let is_last_cond = i == exception.conditions.len() - 1;
                output.push_str(&self.format_condition(
                    condition,
                    &format!("{}{}", child_prefix, self.indent),
                    is_last_cond,
                ));
            }
        }

        output
    }

    /// Formats an amendment node.
    fn format_amendment(&self, amendment: &AmendmentNode, prefix: &str, is_last: bool) -> String {
        let branch_prefix = if is_last {
            &self.last_branch
        } else {
            &self.branch
        };

        let mut parts = vec![format!("Target: {}", amendment.target_id)];

        if let Some(version) = amendment.version {
            parts.push(format!("v{}", version));
        }

        if let Some(ref date) = amendment.date {
            parts.push(format!("Date: {}", date));
        }

        format!(
            "{}{}{} - {}\n",
            prefix,
            branch_prefix,
            parts.join(", "),
            self.colorize(&amendment.description, Color::Blue, false)
        )
    }

    /// Formats a condition value.
    fn format_value(&self, value: &ConditionValue) -> String {
        match value {
            ConditionValue::Number(n) => n.to_string(),
            ConditionValue::String(s) => format!("\"{}\"", s),
            ConditionValue::Boolean(b) => b.to_string(),
            ConditionValue::Date(d) => d.clone(),
            ConditionValue::SetExpr(expr) => format!("{:?}", expr),
        }
    }

    /// Applies color to text if color mode is enabled.
    fn colorize(&self, text: &str, color: Color, bold: bool) -> String {
        if !self.use_color {
            return text.to_string();
        }

        let color_code = match color {
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
        };

        if bold {
            format!("\x1b[1;{}m{}\x1b[0m", color_code, text)
        } else {
            format!("\x1b[{}m{}\x1b[0m", color_code, text)
        }
    }
}

/// ANSI color codes.
#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_formatter_basic() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![StatuteNode {
                id: "test".to_string(),
                title: "Test Statute".to_string(),
                conditions: vec![ConditionNode::HasAttribute {
                    key: "citizen".to_string(),
                }],
                effects: vec![],
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
            }],
        };

        let formatter = TreeFormatter::new();
        let output = formatter.format_document(&doc);

        assert!(output.contains("Legal Document"));
        assert!(output.contains("test"));
        assert!(output.contains("Test Statute"));
        assert!(output.contains("citizen"));
    }

    #[test]
    fn test_tree_formatter_with_color() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![StatuteNode {
                id: "test".to_string(),
                title: "Test".to_string(),
                conditions: vec![],
                effects: vec![],
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
            }],
        };

        let formatter = TreeFormatter::new().with_color(true);
        let output = formatter.format_document(&doc);

        // Should contain ANSI escape codes
        assert!(output.contains("\x1b["));
    }
}
