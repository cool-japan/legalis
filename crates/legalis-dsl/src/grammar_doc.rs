//! Grammar documentation generator for the Legalis DSL.
//!
//! This module provides functionality to automatically generate
//! documentation for the DSL grammar in various formats.

use std::fmt::Write as _;

/// Represents a grammar rule with its definition and description.
#[derive(Debug, Clone)]
pub struct GrammarRule {
    /// The name of the non-terminal
    pub name: String,
    /// The production rule (right-hand side)
    pub production: String,
    /// Optional description of what this rule represents
    pub description: Option<String>,
    /// Examples demonstrating this rule
    pub examples: Vec<String>,
}

impl GrammarRule {
    /// Creates a new grammar rule.
    pub fn new(name: impl Into<String>, production: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            production: production.into(),
            description: None,
            examples: Vec::new(),
        }
    }

    /// Adds a description to this rule.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds an example to this rule.
    pub fn with_example(mut self, example: impl Into<String>) -> Self {
        self.examples.push(example.into());
        self
    }
}

/// A complete grammar specification.
#[derive(Debug, Clone)]
pub struct GrammarSpec {
    /// The name of the language/grammar
    pub name: String,
    /// Description of the grammar
    pub description: String,
    /// All grammar rules
    pub rules: Vec<GrammarRule>,
    /// Keywords in the language
    pub keywords: Vec<String>,
    /// Operators in the language
    pub operators: Vec<String>,
}

impl GrammarSpec {
    /// Creates a new grammar specification.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            rules: Vec::new(),
            keywords: Vec::new(),
            operators: Vec::new(),
        }
    }

    /// Adds a grammar rule.
    pub fn add_rule(&mut self, rule: GrammarRule) {
        self.rules.push(rule);
    }

    /// Adds a keyword.
    pub fn add_keyword(&mut self, keyword: impl Into<String>) {
        self.keywords.push(keyword.into());
    }

    /// Adds an operator.
    pub fn add_operator(&mut self, operator: impl Into<String>) {
        self.operators.push(operator.into());
    }

    /// Generates Markdown documentation for the grammar.
    pub fn to_markdown(&self) -> String {
        let mut doc = String::new();

        writeln!(doc, "# {} Grammar Reference", self.name).unwrap();
        writeln!(doc).unwrap();
        writeln!(doc, "{}", self.description).unwrap();
        writeln!(doc).unwrap();

        // Grammar Rules
        writeln!(doc, "## Grammar Rules").unwrap();
        writeln!(doc).unwrap();
        writeln!(doc, "```ebnf").unwrap();
        for rule in &self.rules {
            writeln!(doc, "{} ::= {}", rule.name, rule.production).unwrap();
        }
        writeln!(doc, "```").unwrap();
        writeln!(doc).unwrap();

        // Detailed Rule Descriptions
        writeln!(doc, "## Rule Descriptions").unwrap();
        writeln!(doc).unwrap();
        for rule in &self.rules {
            writeln!(doc, "### `{}`", rule.name).unwrap();
            writeln!(doc).unwrap();
            if let Some(description) = &rule.description {
                writeln!(doc, "{}", description).unwrap();
                writeln!(doc).unwrap();
            }
            writeln!(doc, "**Production:** `{}`", rule.production).unwrap();
            writeln!(doc).unwrap();

            if !rule.examples.is_empty() {
                writeln!(doc, "**Examples:**").unwrap();
                writeln!(doc).unwrap();
                for example in &rule.examples {
                    writeln!(doc, "```").unwrap();
                    writeln!(doc, "{}", example).unwrap();
                    writeln!(doc, "```").unwrap();
                    writeln!(doc).unwrap();
                }
            }
        }

        // Keywords
        if !self.keywords.is_empty() {
            writeln!(doc, "## Keywords").unwrap();
            writeln!(doc).unwrap();
            for keyword in &self.keywords {
                writeln!(doc, "- `{}`", keyword).unwrap();
            }
            writeln!(doc).unwrap();
        }

        // Operators
        if !self.operators.is_empty() {
            writeln!(doc, "## Operators").unwrap();
            writeln!(doc).unwrap();
            for operator in &self.operators {
                writeln!(doc, "- `{}`", operator).unwrap();
            }
            writeln!(doc).unwrap();
        }

        doc
    }

    /// Generates HTML documentation for the grammar.
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        writeln!(html, "<!DOCTYPE html>").unwrap();
        writeln!(html, "<html>").unwrap();
        writeln!(html, "<head>").unwrap();
        writeln!(html, "  <meta charset=\"UTF-8\">").unwrap();
        writeln!(html, "  <title>{} Grammar Reference</title>", self.name).unwrap();
        writeln!(html, "  <style>").unwrap();
        writeln!(html, "    body {{ font-family: Arial, sans-serif; max-width: 900px; margin: 0 auto; padding: 20px; }}").unwrap();
        writeln!(html, "    h1 {{ color: #2c3e50; }}").unwrap();
        writeln!(
            html,
            "    h2 {{ color: #34495e; border-bottom: 2px solid #ecf0f1; padding-bottom: 10px; }}"
        )
        .unwrap();
        writeln!(html, "    h3 {{ color: #7f8c8d; }}").unwrap();
        writeln!(
            html,
            "    code {{ background-color: #f8f9fa; padding: 2px 6px; border-radius: 3px; }}"
        )
        .unwrap();
        writeln!(html, "    pre {{ background-color: #f8f9fa; padding: 15px; border-radius: 5px; overflow-x: auto; }}").unwrap();
        writeln!(html, "    .rule {{ margin-bottom: 30px; }}").unwrap();
        writeln!(html, "    ul {{ line-height: 1.8; }}").unwrap();
        writeln!(html, "  </style>").unwrap();
        writeln!(html, "</head>").unwrap();
        writeln!(html, "<body>").unwrap();

        writeln!(html, "  <h1>{} Grammar Reference</h1>", self.name).unwrap();
        writeln!(html, "  <p>{}</p>", self.description).unwrap();

        // Grammar Rules
        writeln!(html, "  <h2>Grammar Rules</h2>").unwrap();
        writeln!(html, "  <pre><code>").unwrap();
        for rule in &self.rules {
            writeln!(html, "{} ::= {}", rule.name, rule.production).unwrap();
        }
        writeln!(html, "  </code></pre>").unwrap();

        // Detailed Rule Descriptions
        writeln!(html, "  <h2>Rule Descriptions</h2>").unwrap();
        for rule in &self.rules {
            writeln!(html, "  <div class=\"rule\">").unwrap();
            writeln!(html, "    <h3>{}</h3>", rule.name).unwrap();
            if let Some(description) = &rule.description {
                writeln!(html, "    <p>{}</p>", description).unwrap();
            }
            writeln!(
                html,
                "    <p><strong>Production:</strong> <code>{}</code></p>",
                rule.production
            )
            .unwrap();

            if !rule.examples.is_empty() {
                writeln!(html, "    <p><strong>Examples:</strong></p>").unwrap();
                for example in &rule.examples {
                    writeln!(html, "    <pre><code>{}</code></pre>", example).unwrap();
                }
            }
            writeln!(html, "  </div>").unwrap();
        }

        // Keywords
        if !self.keywords.is_empty() {
            writeln!(html, "  <h2>Keywords</h2>").unwrap();
            writeln!(html, "  <ul>").unwrap();
            for keyword in &self.keywords {
                writeln!(html, "    <li><code>{}</code></li>", keyword).unwrap();
            }
            writeln!(html, "  </ul>").unwrap();
        }

        // Operators
        if !self.operators.is_empty() {
            writeln!(html, "  <h2>Operators</h2>").unwrap();
            writeln!(html, "  <ul>").unwrap();
            for operator in &self.operators {
                writeln!(html, "    <li><code>{}</code></li>", operator).unwrap();
            }
            writeln!(html, "  </ul>").unwrap();
        }

        writeln!(html, "</body>").unwrap();
        writeln!(html, "</html>").unwrap();

        html
    }

    /// Generates plain text documentation for the grammar.
    pub fn to_text(&self) -> String {
        let mut text = String::new();

        writeln!(text, "{} GRAMMAR REFERENCE", self.name.to_uppercase()).unwrap();
        writeln!(text, "{}", "=".repeat(self.name.len() + 18)).unwrap();
        writeln!(text).unwrap();
        writeln!(text, "{}", self.description).unwrap();
        writeln!(text).unwrap();

        writeln!(text, "GRAMMAR RULES").unwrap();
        writeln!(text, "-------------").unwrap();
        for rule in &self.rules {
            writeln!(text, "{} ::= {}", rule.name, rule.production).unwrap();
        }
        writeln!(text).unwrap();

        writeln!(text, "RULE DESCRIPTIONS").unwrap();
        writeln!(text, "-----------------").unwrap();
        for rule in &self.rules {
            writeln!(text, "{}:", rule.name).unwrap();
            if let Some(description) = &rule.description {
                writeln!(text, "  {}", description).unwrap();
            }
            writeln!(text, "  Production: {}", rule.production).unwrap();
            if !rule.examples.is_empty() {
                writeln!(text, "  Examples:").unwrap();
                for example in &rule.examples {
                    for line in example.lines() {
                        writeln!(text, "    {}", line).unwrap();
                    }
                }
            }
            writeln!(text).unwrap();
        }

        if !self.keywords.is_empty() {
            writeln!(text, "KEYWORDS").unwrap();
            writeln!(text, "--------").unwrap();
            for keyword in &self.keywords {
                writeln!(text, "- {}", keyword).unwrap();
            }
            writeln!(text).unwrap();
        }

        if !self.operators.is_empty() {
            writeln!(text, "OPERATORS").unwrap();
            writeln!(text, "---------").unwrap();
            for operator in &self.operators {
                writeln!(text, "- {}", operator).unwrap();
            }
            writeln!(text).unwrap();
        }

        text
    }
}

/// Returns the complete Legalis DSL grammar specification.
pub fn legalis_grammar() -> GrammarSpec {
    let mut spec = GrammarSpec::new(
        "Legalis DSL",
        "A domain-specific language for representing legal statutes and rules.",
    );

    // Add keywords
    for keyword in &[
        "STATUTE",
        "WHEN",
        "THEN",
        "UNLESS",
        "REQUIRES",
        "DISCRETION",
        "EXCEPTION",
        "AMENDMENT",
        "SUPERSEDES",
        "DEFAULT",
        "IMPORT",
        "AS",
        "JURISDICTION",
        "VERSION",
        "EFFECTIVE_DATE",
        "EXPIRY_DATE",
        "EFFECTIVE",
        "EXPIRY",
        "EXPIRES",
        "AGE",
        "INCOME",
        "CURRENT_DATE",
        "DATE_FIELD",
        "HAS",
        "AND",
        "OR",
        "NOT",
        "BETWEEN",
        "IN",
        "LIKE",
        "MATCHES",
        "IN_RANGE",
        "NOT_IN_RANGE",
        "GRANT",
        "REVOKE",
        "OBLIGATION",
        "PROHIBITION",
        "UNION",
        "INTERSECT",
        "DIFFERENCE",
    ] {
        spec.add_keyword(*keyword);
    }

    // Add operators
    for operator in &[
        ">=", "<=", ">", "<", "==", "=", "!=", ":", ",", "{", "}", "(", ")", ".", "..",
    ] {
        spec.add_operator(*operator);
    }

    // Add grammar rules with descriptions and examples
    spec.add_rule(
        GrammarRule::new("STATUTE", "\"STATUTE\" ID \":\" TITLE \"{\" BODY \"}\"")
            .with_description("Defines a legal statute with an identifier, title, and body containing conditions and effects.")
            .with_example(r#"STATUTE adult-rights: "Adult Rights Act" {
    WHEN AGE >= 18
    THEN GRANT "Full legal capacity"
}"#)
    );

    spec.add_rule(
        GrammarRule::new("BODY", "(METADATA | DEFAULT | WHEN | UNLESS | REQUIRES | THEN | DISCRETION | EXCEPTION | AMENDMENT | SUPERSEDES)*")
            .with_description("The body of a statute contains metadata, conditions, effects, and other clauses.")
    );

    spec.add_rule(
        GrammarRule::new(
            "METADATA",
            "EFFECTIVE_DATE | EXPIRY_DATE | JURISDICTION | VERSION",
        )
        .with_description("Metadata about the statute such as dates, jurisdiction, and version."),
    );

    spec.add_rule(
        GrammarRule::new("WHEN", "\"WHEN\" CONDITION")
            .with_description("Specifies a condition that must be met for the statute to apply.")
            .with_example("WHEN AGE >= 18 AND HAS citizenship"),
    );

    spec.add_rule(
        GrammarRule::new("UNLESS", "\"UNLESS\" CONDITION")
            .with_description("Specifies a negative condition (equivalent to WHEN NOT condition).")
            .with_example("UNLESS AGE < 18"),
    );

    spec.add_rule(
        GrammarRule::new("REQUIRES", "\"REQUIRES\" IDENT (\",\" IDENT)*")
            .with_description("Lists other statutes that must be satisfied as prerequisites.")
            .with_example("REQUIRES citizenship-law, residence-requirement"),
    );

    spec.add_rule(GrammarRule::new("CONDITION", "OR_EXPR").with_description(
        "A boolean condition expression supporting AND, OR, NOT, and comparisons.",
    ));

    spec.add_rule(
        GrammarRule::new("OR_EXPR", "AND_EXPR (\"OR\" AND_EXPR)*")
            .with_description("Logical OR expression with left-to-right evaluation."),
    );

    spec.add_rule(
        GrammarRule::new("AND_EXPR", "UNARY_EXPR (\"AND\" UNARY_EXPR)*")
            .with_description("Logical AND expression with higher precedence than OR."),
    );

    spec.add_rule(
        GrammarRule::new(
            "UNARY_EXPR",
            "\"NOT\" UNARY_EXPR | \"(\" CONDITION \")\" | PRIMARY_COND",
        )
        .with_description("Unary negation or parenthesized condition."),
    );

    spec.add_rule(
        GrammarRule::new("THEN", "\"THEN\" EFFECT")
            .with_description("Specifies the legal effect when conditions are met.")
            .with_example(r#"THEN GRANT "Right to vote""#),
    );

    spec.add_rule(
        GrammarRule::new(
            "EFFECT",
            "(\"GRANT\" | \"REVOKE\" | \"OBLIGATION\" | \"PROHIBITION\") STRING",
        )
        .with_description(
            "A legal effect that grants, revokes, obligates, or prohibits something.",
        ),
    );

    spec.add_rule(
        GrammarRule::new("DISCRETION", "\"DISCRETION\" STRING")
            .with_description("Optional discretionary logic that may be considered.")
            .with_example(r#"DISCRETION "Consider special circumstances""#),
    );

    spec.add_rule(
        GrammarRule::new("EXCEPTION", "\"EXCEPTION\" [\"WHEN\" CONDITION] STRING")
            .with_description("Defines an exception to the statute with optional conditions.")
            .with_example(
                r#"EXCEPTION WHEN AGE < 18 AND HAS guardian_consent "Minors with consent""#,
            ),
    );

    spec.add_rule(
        GrammarRule::new("DEFAULT", "\"DEFAULT\" IDENT (\"=\" | \":\") VALUE")
            .with_description("Specifies a default value for an attribute.")
            .with_example(r#"DEFAULT status "pending""#),
    );

    spec.add_rule(
        GrammarRule::new("IMPORT", "\"IMPORT\" STRING [\"AS\" IDENT]")
            .with_description("Imports definitions from another statute file with optional alias.")
            .with_example(r#"IMPORT "common-definitions.dsl" AS common"#),
    );

    spec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_rule_creation() {
        let rule = GrammarRule::new("TEST", "\"test\" ID")
            .with_description("A test rule")
            .with_example("test example");

        assert_eq!(rule.name, "TEST");
        assert_eq!(rule.production, "\"test\" ID");
        assert_eq!(rule.description, Some("A test rule".to_string()));
        assert_eq!(rule.examples.len(), 1);
    }

    #[test]
    fn test_grammar_spec_markdown() {
        let mut spec = GrammarSpec::new("Test Language", "A test language");
        spec.add_rule(GrammarRule::new("RULE", "\"rule\" ID"));
        spec.add_keyword("KEYWORD");
        spec.add_operator("=");

        let markdown = spec.to_markdown();
        assert!(markdown.contains("# Test Language Grammar Reference"));
        assert!(markdown.contains("RULE ::= \"rule\" ID"));
        assert!(markdown.contains("KEYWORD"));
    }

    #[test]
    fn test_grammar_spec_html() {
        let mut spec = GrammarSpec::new("Test", "Test language");
        spec.add_rule(GrammarRule::new("RULE", "ID"));

        let html = spec.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Grammar Reference"));
    }

    #[test]
    fn test_grammar_spec_text() {
        let mut spec = GrammarSpec::new("Test", "Test language");
        spec.add_rule(GrammarRule::new("RULE", "ID"));

        let text = spec.to_text();
        assert!(text.contains("TEST GRAMMAR REFERENCE"));
        assert!(text.contains("RULE ::= ID"));
    }

    #[test]
    fn test_legalis_grammar() {
        let spec = legalis_grammar();
        assert_eq!(spec.name, "Legalis DSL");
        assert!(!spec.rules.is_empty());
        assert!(!spec.keywords.is_empty());
        assert!(!spec.operators.is_empty());

        // Check that specific important rules exist
        assert!(spec.rules.iter().any(|r| r.name == "STATUTE"));
        assert!(spec.rules.iter().any(|r| r.name == "WHEN"));
        assert!(spec.rules.iter().any(|r| r.name == "CONDITION"));
    }

    #[test]
    fn test_legalis_grammar_markdown() {
        let spec = legalis_grammar();
        let markdown = spec.to_markdown();

        assert!(markdown.contains("# Legalis DSL Grammar Reference"));
        assert!(markdown.contains("STATUTE"));
        assert!(markdown.contains("WHEN"));
        assert!(markdown.contains("## Keywords"));
    }
}
