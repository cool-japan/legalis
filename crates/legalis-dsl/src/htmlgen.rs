//! HTML documentation generator for legal documents.
//!
//! This module generates formatted HTML documentation from parsed legal documents,
//! suitable for web viewing and publishing.

#[cfg(test)]
use crate::ast::EffectNode;
use crate::ast::{ConditionNode, LegalDocument, StatuteNode};
use std::fmt::Write as FmtWrite;

/// HTML documentation generator.
pub struct HtmlGenerator {
    /// Include CSS styling inline
    pub include_css: bool,
    /// Include table of contents
    pub include_toc: bool,
    /// CSS theme (light or dark)
    pub theme: HtmlTheme,
}

/// HTML theme options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlTheme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
}

impl Default for HtmlGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl HtmlGenerator {
    /// Creates a new HTML generator with default settings.
    pub fn new() -> Self {
        Self {
            include_css: true,
            include_toc: true,
            theme: HtmlTheme::Light,
        }
    }

    /// Sets whether to include CSS.
    pub fn with_css(mut self, include: bool) -> Self {
        self.include_css = include;
        self
    }

    /// Sets whether to include table of contents.
    pub fn with_toc(mut self, include: bool) -> Self {
        self.include_toc = include;
        self
    }

    /// Sets the HTML theme.
    pub fn with_theme(mut self, theme: HtmlTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates HTML documentation from a legal document.
    pub fn generate(&self, doc: &LegalDocument) -> String {
        let mut html = String::new();

        // HTML header
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("    <title>Legal Document</title>\n");

        if self.include_css {
            html.push_str("    <style>\n");
            html.push_str(&self.generate_css());
            html.push_str("    </style>\n");
        }

        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("    <div class=\"container\">\n");
        html.push_str("        <header>\n");
        html.push_str("            <h1>Legal Document</h1>\n");
        html.push_str("        </header>\n");

        // Table of contents
        if self.include_toc && !doc.statutes.is_empty() {
            html.push_str(&self.generate_toc(&doc.statutes));
        }

        // Main content
        html.push_str("        <main>\n");
        for statute in &doc.statutes {
            html.push_str(&self.generate_statute_html(statute));
        }
        html.push_str("        </main>\n");

        html.push_str("    </div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>");

        html
    }

    /// Generates CSS styles.
    fn generate_css(&self) -> String {
        match self.theme {
            HtmlTheme::Light => r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background-color: #f5f5f5;
            margin: 0;
            padding: 20px;
        }
        .container {
            max-width: 900px;
            margin: 0 auto;
            background: white;
            padding: 30px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            border-radius: 8px;
        }
        header h1 {
            color: #2c3e50;
            border-bottom: 3px solid #3498db;
            padding-bottom: 10px;
        }
        .toc {
            background: #f8f9fa;
            padding: 20px;
            border-radius: 5px;
            margin: 20px 0;
        }
        .toc h2 {
            margin-top: 0;
            color: #2c3e50;
        }
        .toc ul {
            list-style: none;
            padding-left: 0;
        }
        .toc li {
            margin: 8px 0;
        }
        .toc a {
            color: #3498db;
            text-decoration: none;
        }
        .toc a:hover {
            text-decoration: underline;
        }
        .statute {
            margin: 30px 0;
            padding: 20px;
            border: 1px solid #ddd;
            border-radius: 5px;
            background: #fafafa;
        }
        .statute-id {
            color: #7f8c8d;
            font-size: 0.9em;
            font-family: monospace;
        }
        .statute-title {
            color: #2c3e50;
            margin: 10px 0;
        }
        .section {
            margin: 15px 0;
            padding: 15px;
            background: white;
            border-left: 3px solid #3498db;
        }
        .section-title {
            font-weight: bold;
            color: #34495e;
            margin-bottom: 10px;
        }
        .condition, .effect {
            margin: 8px 0;
            padding: 8px 12px;
            border-radius: 3px;
        }
        .condition {
            background: #e3f2fd;
            border-left: 3px solid #2196f3;
        }
        .effect {
            background: #e8f5e9;
            border-left: 3px solid #4caf50;
        }
        .metadata {
            background: #fff3e0;
            padding: 10px;
            border-radius: 3px;
            margin: 10px 0;
            font-size: 0.9em;
        }
        .badge {
            display: inline-block;
            padding: 3px 8px;
            border-radius: 3px;
            font-size: 0.85em;
            font-weight: bold;
            margin-right: 5px;
        }
        .badge-requires {
            background: #ffeb3b;
            color: #333;
        }
        .badge-supersedes {
            background: #ff9800;
            color: white;
        }
"#.to_string(),
            HtmlTheme::Dark => r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            color: #e0e0e0;
            background-color: #1a1a1a;
            margin: 0;
            padding: 20px;
        }
        .container {
            max-width: 900px;
            margin: 0 auto;
            background: #2d2d2d;
            padding: 30px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.3);
            border-radius: 8px;
        }
        header h1 {
            color: #64b5f6;
            border-bottom: 3px solid #42a5f5;
            padding-bottom: 10px;
        }
        .statute {
            margin: 30px 0;
            padding: 20px;
            border: 1px solid #444;
            border-radius: 5px;
            background: #252525;
        }
        .condition {
            background: #1e3a5f;
            border-left: 3px solid #42a5f5;
        }
        .effect {
            background: #1e4d2b;
            border-left: 3px solid #66bb6a;
        }
"#.to_string(),
        }
    }

    /// Generates table of contents.
    fn generate_toc(&self, statutes: &[StatuteNode]) -> String {
        let mut toc = String::from("        <nav class=\"toc\">\n");
        toc.push_str("            <h2>Table of Contents</h2>\n");
        toc.push_str("            <ul>\n");

        for statute in statutes {
            let _ = writeln!(
                &mut toc,
                "                <li><a href=\"#{}\">{}</a></li>",
                self.escape_html(&statute.id),
                self.escape_html(&statute.title)
            );
        }

        toc.push_str("            </ul>\n");
        toc.push_str("        </nav>\n");
        toc
    }

    /// Generates HTML for a single statute.
    fn generate_statute_html(&self, statute: &StatuteNode) -> String {
        let mut html = String::new();

        let _ = writeln!(
            &mut html,
            "            <article class=\"statute\" id=\"{}\">",
            self.escape_html(&statute.id)
        );
        let _ = writeln!(
            &mut html,
            "                <div class=\"statute-id\">{}</div>",
            self.escape_html(&statute.id)
        );
        let _ = writeln!(
            &mut html,
            "                <h2 class=\"statute-title\">{}</h2>",
            self.escape_html(&statute.title)
        );

        // Metadata
        if !statute.requires.is_empty() || !statute.supersedes.is_empty() {
            html.push_str("                <div class=\"metadata\">\n");
            if !statute.requires.is_empty() {
                html.push_str(
                    "                    <span class=\"badge badge-requires\">Requires</span>\n",
                );
                html.push_str("                    ");
                html.push_str(
                    &statute
                        .requires
                        .iter()
                        .map(|r| self.escape_html(r))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                html.push_str("<br>\n");
            }
            if !statute.supersedes.is_empty() {
                html.push_str("                    <span class=\"badge badge-supersedes\">Supersedes</span>\n");
                html.push_str("                    ");
                html.push_str(
                    &statute
                        .supersedes
                        .iter()
                        .map(|s| self.escape_html(s))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                html.push('\n');
            }
            html.push_str("                </div>\n");
        }

        // Conditions
        if !statute.conditions.is_empty() {
            html.push_str("                <div class=\"section\">\n");
            html.push_str("                    <div class=\"section-title\">Conditions</div>\n");
            for condition in &statute.conditions {
                let _ = writeln!(
                    &mut html,
                    "                    <div class=\"condition\">{}</div>",
                    self.format_condition(condition)
                );
            }
            html.push_str("                </div>\n");
        }

        // Effects
        if !statute.effects.is_empty() {
            html.push_str("                <div class=\"section\">\n");
            html.push_str("                    <div class=\"section-title\">Effects</div>\n");
            for effect in &statute.effects {
                let _ = writeln!(
                    &mut html,
                    "                    <div class=\"effect\">{}: {}</div>",
                    self.escape_html(&effect.effect_type),
                    self.escape_html(&effect.description)
                );
            }
            html.push_str("                </div>\n");
        }

        // Discretion
        if let Some(discretion) = &statute.discretion {
            html.push_str("                <div class=\"section\">\n");
            html.push_str("                    <div class=\"section-title\">Discretion</div>\n");
            let _ = writeln!(
                &mut html,
                "                    <div>{}</div>",
                self.escape_html(discretion)
            );
            html.push_str("                </div>\n");
        }

        html.push_str("            </article>\n");
        html
    }

    /// Formats a condition node for HTML display.
    fn format_condition(&self, condition: &ConditionNode) -> String {
        match condition {
            ConditionNode::HasAttribute { key } => {
                format!("HAS {}", self.escape_html(key))
            }
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                format!(
                    "{} {} {:?}",
                    self.escape_html(field),
                    self.escape_html(operator),
                    value
                )
            }
            ConditionNode::And(left, right) => {
                format!(
                    "({}) AND ({})",
                    self.format_condition(left),
                    self.format_condition(right)
                )
            }
            ConditionNode::Or(left, right) => {
                format!(
                    "({}) OR ({})",
                    self.format_condition(left),
                    self.format_condition(right)
                )
            }
            ConditionNode::Not(inner) => {
                format!("NOT ({})", self.format_condition(inner))
            }
            _ => format!("{:?}", condition),
        }
    }

    /// Escapes HTML special characters.
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_statute() -> StatuteNode {
        StatuteNode {
            id: "test-1".to_string(),
            title: "Test Statute".to_string(),
            conditions: vec![ConditionNode::HasAttribute {
                key: "citizen".to_string(),
            }],
            effects: vec![EffectNode {
                effect_type: "GRANT".to_string(),
                description: "benefit".to_string(),
                parameters: vec![],
            }],
            discretion: Some("Case by case".to_string()),
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires: vec!["prereq-1".to_string()],
            delegates: vec![],
            scope: None,
            constraints: vec![],
            priority: None,
        }
    }

    #[test]
    fn test_html_generation() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![create_test_statute()],
        };

        let generator = HtmlGenerator::new();
        let html = generator.generate(&doc);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Statute"));
        assert!(html.contains("test-1"));
        assert!(html.contains("Conditions"));
        assert!(html.contains("Effects"));
    }

    #[test]
    fn test_escape_html() {
        let generator = HtmlGenerator::new();
        assert_eq!(generator.escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(generator.escape_html("a & b"), "a &amp; b");
    }

    #[test]
    fn test_dark_theme() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![create_test_statute()],
        };

        let generator = HtmlGenerator::new().with_theme(HtmlTheme::Dark);
        let html = generator.generate(&doc);

        assert!(html.contains("background-color: #1a1a1a"));
    }

    #[test]
    fn test_without_toc() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![create_test_statute()],
        };

        let generator = HtmlGenerator::new().with_toc(false);
        let html = generator.generate(&doc);

        assert!(!html.contains("Table of Contents"));
    }
}
