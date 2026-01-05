//! Semantic Error Explainer for Legalis DSL
//!
//! This module provides human-readable explanations for DSL errors,
//! helping users understand what went wrong and how to fix it.

use crate::{DslError, SourceLocation};

/// Explanation for a DSL error in plain language
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorExplanation {
    /// The original error
    pub error: DslError,
    /// Plain language explanation
    pub explanation: String,
    /// Suggested fixes
    pub suggestions: Vec<String>,
    /// Example of correct usage
    pub example: Option<String>,
    /// Severity level
    pub severity: ErrorSeverity,
}

/// Severity level of an error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Critical error that prevents parsing
    Error,
    /// Warning that may indicate a problem
    Warning,
    /// Informational message
    Info,
}

/// Error explainer that converts DSL errors to plain language
pub struct ErrorExplainer {
    /// Include examples in explanations
    include_examples: bool,
    /// Verbosity level (0-2)
    verbosity: u8,
}

impl ErrorExplainer {
    /// Create a new error explainer with default settings
    pub fn new() -> Self {
        Self {
            include_examples: true,
            verbosity: 1,
        }
    }

    /// Create an error explainer with custom settings
    pub fn with_settings(include_examples: bool, verbosity: u8) -> Self {
        Self {
            include_examples,
            verbosity: verbosity.min(2),
        }
    }

    /// Explain an error in plain language
    pub fn explain(&self, error: &DslError) -> ErrorExplanation {
        match error {
            DslError::ParseError { location, message } => {
                self.explain_parse_error(location, message)
            }
            DslError::InvalidCondition(msg) => self.explain_invalid_condition(msg),
            DslError::InvalidEffect(msg) => self.explain_invalid_effect(msg),
            DslError::UnmatchedParen(_) => self.explain_unmatched_paren(),
            DslError::UnclosedComment(location) => self.explain_unclosed_comment(location),
            DslError::SyntaxError {
                expected,
                found,
                hint,
                ..
            } => self.explain_syntax_error(expected, found, hint.as_deref()),
            DslError::UndefinedReference { name, hint, .. } => {
                self.explain_undefined_reference(name, hint.as_deref())
            }
            _ => ErrorExplanation {
                error: error.clone(),
                explanation: error.to_string(),
                suggestions: vec!["Check the error message for details".to_string()],
                example: None,
                severity: ErrorSeverity::Error,
            },
        }
    }

    fn explain_parse_error(
        &self,
        location: &Option<SourceLocation>,
        message: &str,
    ) -> ErrorExplanation {
        let explanation = if self.verbosity >= 1 {
            format!(
                "The parser encountered an unexpected element while reading your legal document. \
                 At {}, the parser expected a specific syntax element but found something different.",
                location
                    .map(|l| l.to_string())
                    .unwrap_or_else(|| "an unknown location".to_string())
            )
        } else {
            message.to_string()
        };

        let mut suggestions =
            vec!["Check the DSL syntax reference for the correct format".to_string()];

        // Add specific suggestions based on the message
        if message.contains("STATUTE") {
            suggestions.push(
                "Make sure your statute definition starts with 'STATUTE id: \"title\" {'"
                    .to_string(),
            );
        } else if message.contains("identifier") {
            suggestions.push(
                "Statute IDs should contain only letters, numbers, hyphens, and underscores"
                    .to_string(),
            );
        } else if message.contains("':'") {
            suggestions.push(
                "Remember to include a colon (:) between the statute ID and title".to_string(),
            );
        } else if message.contains("'{'") {
            suggestions.push("Statute body must be enclosed in curly braces { }".to_string());
        }

        let example = if self.include_examples {
            Some(
                "STATUTE example-001: \"Example Statute\" {\n    \
                 WHEN age >= 18\n    \
                 THEN GRANT \"voting rights\"\n\
                 }"
                .to_string(),
            )
        } else {
            None
        };

        ErrorExplanation {
            error: DslError::ParseError {
                location: *location,
                message: message.to_string(),
            },
            explanation,
            suggestions,
            example,
            severity: ErrorSeverity::Error,
        }
    }

    fn explain_invalid_condition(&self, message: &str) -> ErrorExplanation {
        let explanation = if self.verbosity >= 1 {
            "A condition in your statute is not formatted correctly. \
             Conditions define when a statute applies and must follow specific patterns."
                .to_string()
        } else {
            message.to_string()
        };

        let mut suggestions = Vec::new();

        if message.contains("operator") {
            suggestions.push("Valid comparison operators are: =, !=, <, <=, >, >=".to_string());
            suggestions.push("For ranges, use: BETWEEN min AND max".to_string());
            suggestions.push("For set membership, use: IN (value1, value2, ...)".to_string());
        } else if message.contains("BETWEEN") {
            suggestions.push("BETWEEN requires AND keyword: field BETWEEN 18 AND 65".to_string());
        } else if message.contains("value") {
            suggestions.push("Values can be numbers (18), strings (\"active\"), booleans (true/false), or dates (2024-01-01)".to_string());
        } else if message.contains("LIKE") || message.contains("pattern") {
            suggestions
                .push("LIKE patterns use SQL-style wildcards: field LIKE \"prefix%\"".to_string());
        } else if message.contains("MATCHES") || message.contains("regex") {
            suggestions.push(
                "MATCHES uses regular expressions: field MATCHES \"^[A-Z]{2}[0-9]{4}$\""
                    .to_string(),
            );
            suggestions.push("Make sure your regex pattern is valid".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("Common condition formats:".to_string());
            suggestions.push("  - Simple comparison: age >= 18".to_string());
            suggestions.push("  - Range check: age BETWEEN 18 AND 65".to_string());
            suggestions.push("  - Set membership: status IN (\"active\", \"pending\")".to_string());
            suggestions.push("  - Attribute check: HAS citizenship".to_string());
        }

        let example = if self.include_examples {
            Some(
                "WHEN age >= 18 AND income > 50000\n\
                 WHEN status IN (\"active\", \"verified\")\n\
                 WHEN age BETWEEN 21 AND 65\n\
                 WHEN HAS citizenship"
                    .to_string(),
            )
        } else {
            None
        };

        ErrorExplanation {
            error: DslError::InvalidCondition(message.to_string()),
            explanation,
            suggestions,
            example,
            severity: ErrorSeverity::Error,
        }
    }

    fn explain_invalid_effect(&self, message: &str) -> ErrorExplanation {
        let explanation = if self.verbosity >= 1 {
            "An effect clause in your statute is not properly formatted. \
             Effects describe what happens when the statute's conditions are met."
                .to_string()
        } else {
            message.to_string()
        };

        let suggestions = vec![
            "Effect types must be one of: GRANT, REVOKE, OBLIGATION, PROHIBITION".to_string(),
            "Effects require a description in quotes: GRANT \"voting rights\"".to_string(),
            "You can add multiple effects to a single statute".to_string(),
        ];

        let example = if self.include_examples {
            Some(
                "THEN GRANT \"right to vote\"\n\
                 THEN OBLIGATION \"must file annual report\"\n\
                 THEN PROHIBITION \"cannot operate heavy machinery\"\n\
                 THEN REVOKE \"previous benefit\""
                    .to_string(),
            )
        } else {
            None
        };

        ErrorExplanation {
            error: DslError::InvalidEffect(message.to_string()),
            explanation,
            suggestions,
            example,
            severity: ErrorSeverity::Error,
        }
    }

    fn explain_unmatched_paren(&self) -> ErrorExplanation {
        let explanation = if self.verbosity >= 1 {
            "Your statute has mismatched parentheses. \
             Every opening parenthesis '(' must have a corresponding closing parenthesis ')'. \
             Parentheses are used to group conditions together."
                .to_string()
        } else {
            "Unmatched parenthesis in condition".to_string()
        };

        let suggestions = vec![
            "Count your opening '(' and closing ')' parentheses to make sure they match"
                .to_string(),
            "Use parentheses to group complex conditions: (age >= 18 AND age <= 65)".to_string(),
            "Nested parentheses are allowed: ((age >= 18) AND (income > 0))".to_string(),
        ];

        let example = if self.include_examples {
            Some(
                "WHEN (age >= 18 AND citizenship) OR (age >= 16 AND HAS guardian_consent)\n\
                 WHEN ((income > 50000) OR (HAS scholarship)) AND (age < 30)"
                    .to_string(),
            )
        } else {
            None
        };

        ErrorExplanation {
            error: DslError::UnmatchedParen(None),
            explanation,
            suggestions,
            example,
            severity: ErrorSeverity::Error,
        }
    }

    fn explain_unclosed_comment(&self, location: &Option<SourceLocation>) -> ErrorExplanation {
        let explanation = if self.verbosity >= 1 {
            format!(
                "A multi-line comment starting at {} was never closed. \
                 Multi-line comments must begin with /* and end with */",
                location
                    .map(|l| l.to_string())
                    .unwrap_or_else(|| "an unknown location".to_string())
            )
        } else {
            "Unclosed comment".to_string()
        };

        let suggestions = vec![
            "Add */ at the end of your multi-line comment".to_string(),
            "Single-line comments start with // and don't need closing".to_string(),
            "Multi-line comments: /* comment text */".to_string(),
        ];

        let example = if self.include_examples {
            Some(
                "// This is a single-line comment\n\
                 \n\
                 /* This is a multi-line comment\n   \
                 that can span multiple lines */\n\
                 \n\
                 STATUTE example: \"Example\" { }"
                    .to_string(),
            )
        } else {
            None
        };

        ErrorExplanation {
            error: DslError::UnclosedComment(*location),
            explanation,
            suggestions,
            example,
            severity: ErrorSeverity::Error,
        }
    }

    fn explain_syntax_error(
        &self,
        expected: &str,
        found: &str,
        hint: Option<&str>,
    ) -> ErrorExplanation {
        let explanation = if self.verbosity >= 1 {
            format!(
                "The parser expected to find {} but instead found {}. \
                 This usually means there's a missing or misplaced keyword or punctuation.",
                expected, found
            )
        } else {
            format!("Expected {} but found {}", expected, found)
        };

        let mut suggestions = vec![format!(
            "Make sure you have {} in the right place",
            expected
        )];

        if let Some(h) = hint {
            suggestions.insert(0, h.to_string());
        }

        // Add context-specific suggestions
        if expected.contains("keyword") {
            suggestions
                .push("Keywords are case-sensitive (use uppercase: WHEN, THEN, etc.)".to_string());
        }

        ErrorExplanation {
            error: DslError::SyntaxError {
                location: SourceLocation::default(),
                message: explanation.clone(),
                expected: expected.to_string(),
                found: found.to_string(),
                hint: hint.map(|s| s.to_string()),
            },
            explanation,
            suggestions,
            example: None,
            severity: ErrorSeverity::Error,
        }
    }

    fn explain_undefined_reference(&self, id: &str, hint: Option<&str>) -> ErrorExplanation {
        let explanation = if self.verbosity >= 1 {
            format!(
                "Your statute references '{}' but this statute ID doesn't exist in the document. \
                 This could be in a REQUIRES or SUPERSEDES clause.",
                id
            )
        } else {
            format!("Undefined statute reference: {}", id)
        };

        let mut suggestions = vec![
            format!("Check if '{}' is the correct statute ID", id),
            "Make sure the referenced statute is defined in the same document or imported"
                .to_string(),
            "Statute IDs are case-sensitive".to_string(),
        ];

        if let Some(h) = hint {
            suggestions.insert(0, format!("Did you mean '{}'?", h));
        }

        let example = if self.include_examples {
            Some(
                "// Define the statute first\n\
                 STATUTE base-001: \"Base Statute\" { }\n\
                 \n\
                 // Then reference it\n\
                 STATUTE derived-001: \"Derived Statute\" {\n    \
                 REQUIRES base-001\n\
                 }"
                .to_string(),
            )
        } else {
            None
        };

        ErrorExplanation {
            error: DslError::UndefinedReference {
                location: SourceLocation::default(),
                name: id.to_string(),
                hint: hint.map(|s| s.to_string()),
            },
            explanation,
            suggestions,
            example,
            severity: ErrorSeverity::Error,
        }
    }

    /// Format an error explanation as a user-friendly message
    pub fn format_explanation(&self, explanation: &ErrorExplanation) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&format!("Error: {}\n\n", explanation.error));

        // Explanation
        output.push_str("What went wrong:\n");
        output.push_str(&format!("  {}\n\n", explanation.explanation));

        // Suggestions
        if !explanation.suggestions.is_empty() {
            output.push_str("How to fix it:\n");
            for suggestion in &explanation.suggestions {
                output.push_str(&format!("  • {}\n", suggestion));
            }
            output.push('\n');
        }

        // Example
        if let Some(example) = &explanation.example {
            output.push_str("Example of correct usage:\n");
            for line in example.lines() {
                output.push_str(&format!("  {}\n", line));
            }
        }

        output
    }
}

impl Default for ErrorExplainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explain_parse_error() {
        let explainer = ErrorExplainer::new();
        let error = DslError::parse_error("Expected 'STATUTE' keyword");
        let explanation = explainer.explain(&error);

        assert!(!explanation.explanation.is_empty());
        assert!(!explanation.suggestions.is_empty());
        assert!(explanation.example.is_some());
        assert_eq!(explanation.severity, ErrorSeverity::Error);
    }

    #[test]
    fn test_explain_invalid_condition() {
        let explainer = ErrorExplainer::new();
        let error = DslError::InvalidCondition("Expected operator".to_string());
        let explanation = explainer.explain(&error);

        assert!(explanation.explanation.contains("condition"));
        assert!(!explanation.suggestions.is_empty());
    }

    #[test]
    fn test_explain_unmatched_paren() {
        let explainer = ErrorExplainer::new();
        let error = DslError::UnmatchedParen(None);
        let explanation = explainer.explain(&error);

        assert!(explanation.explanation.contains("parenthes"));
        assert!(explanation.suggestions.iter().any(|s| s.contains("match")));
    }

    #[test]
    fn test_format_explanation() {
        let explainer = ErrorExplainer::new();
        let error = DslError::InvalidCondition("Test error".to_string());
        let explanation = explainer.explain(&error);
        let formatted = explainer.format_explanation(&explanation);

        assert!(formatted.contains("What went wrong"));
        assert!(formatted.contains("How to fix it"));
    }

    #[test]
    fn test_verbosity_levels() {
        let brief = ErrorExplainer::with_settings(false, 0);
        let verbose = ErrorExplainer::with_settings(true, 2);

        let error = DslError::parse_error("Test");

        let brief_exp = brief.explain(&error);
        let verbose_exp = verbose.explain(&error);

        assert!(verbose_exp.explanation.len() >= brief_exp.explanation.len());
    }

    #[test]
    fn test_undefined_reference_with_hint() {
        let explainer = ErrorExplainer::new();
        let error = DslError::UndefinedReference {
            location: SourceLocation::default(),
            name: "statue-001".to_string(),
            hint: Some("statute-001".to_string()),
        };
        let explanation = explainer.explain(&error);

        assert!(
            explanation
                .suggestions
                .iter()
                .any(|s| s.contains("Did you mean"))
        );
    }
}
