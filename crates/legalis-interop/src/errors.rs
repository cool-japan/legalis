//! Enhanced error types with context information.
//!
//! This module provides rich error types that include line/column information,
//! suggestions for fixes, and detailed context about what went wrong.

use crate::{InteropError, LegalFormat};
use std::fmt;

/// Enhanced error with source location and context.
#[derive(Debug, Clone)]
pub struct ContextualError {
    /// The error message
    pub message: String,
    /// Source format being parsed/converted
    pub format: Option<LegalFormat>,
    /// Line number where error occurred (1-indexed)
    pub line: Option<usize>,
    /// Column number where error occurred (1-indexed)
    pub column: Option<usize>,
    /// Snippet of source code around the error
    pub snippet: Option<String>,
    /// Suggested fix or hint
    pub suggestion: Option<String>,
    /// Additional context information
    pub context: Vec<String>,
}

impl ContextualError {
    /// Creates a new contextual error from an InteropError.
    pub fn from_error(error: InteropError) -> Self {
        Self {
            message: error.to_string(),
            format: None,
            line: None,
            column: None,
            snippet: None,
            suggestion: None,
            context: Vec::new(),
        }
    }

    /// Creates a new contextual error from a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            format: None,
            line: None,
            column: None,
            snippet: None,
            suggestion: None,
            context: Vec::new(),
        }
    }

    /// Sets the format being processed.
    pub fn with_format(mut self, format: LegalFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Sets the line number where the error occurred.
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// Sets the column number where the error occurred.
    pub fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    /// Sets a source code snippet showing where the error occurred.
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }

    /// Adds a suggestion for how to fix the error.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Adds additional context information.
    pub fn add_context(mut self, info: impl Into<String>) -> Self {
        self.context.push(info.into());
        self
    }

    /// Returns a formatted error message with all context.
    pub fn detailed_message(&self) -> String {
        let mut parts = Vec::new();

        // Main error message
        parts.push(format!("Error: {}", self.message));

        // Format and location
        if let Some(format) = self.format {
            let mut location = format!("Format: {:?}", format);
            if let Some(line) = self.line {
                location.push_str(&format!(" at line {}", line));
                if let Some(col) = self.column {
                    location.push_str(&format!(", column {}", col));
                }
            }
            parts.push(location);
        }

        // Source snippet
        if let Some(snippet) = &self.snippet {
            parts.push(String::new());
            parts.push("Source:".to_string());
            parts.push(snippet.clone());
        }

        // Suggestion
        if let Some(suggestion) = &self.suggestion {
            parts.push(String::new());
            parts.push(format!("Suggestion: {}", suggestion));
        }

        // Additional context
        if !self.context.is_empty() {
            parts.push(String::new());
            parts.push("Context:".to_string());
            for ctx in &self.context {
                parts.push(format!("  - {}", ctx));
            }
        }

        parts.join("\n")
    }
}

impl fmt::Display for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detailed_message())
    }
}

impl std::error::Error for ContextualError {}

/// Helper to extract line and column information from source text.
pub struct SourceLocation {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Byte offset in source
    pub offset: usize,
}

impl SourceLocation {
    /// Finds the location of a byte offset in source text.
    pub fn from_offset(source: &str, offset: usize) -> Self {
        let mut line = 1;
        let mut column = 1;
        let mut current_offset = 0;

        for ch in source.chars() {
            if current_offset >= offset {
                break;
            }

            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }

            current_offset += ch.len_utf8();
        }

        Self {
            line,
            column,
            offset,
        }
    }

    /// Extracts a snippet of source code around this location.
    pub fn extract_snippet(&self, source: &str, context_lines: usize) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let line_idx = self.line.saturating_sub(1);

        let start_line = line_idx.saturating_sub(context_lines);
        let end_line = (line_idx + context_lines + 1).min(lines.len());

        let mut snippet = String::new();

        for (idx, line) in lines[start_line..end_line].iter().enumerate() {
            let current_line = start_line + idx + 1;
            let marker = if current_line == self.line { ">" } else { " " };

            snippet.push_str(&format!("{:4} {} | {}\n", current_line, marker, line));

            // Add column indicator for the error line
            if current_line == self.line {
                let spaces = " ".repeat(self.column + 7); // Account for line number formatting
                snippet.push_str(&format!("{}^\n", spaces));
            }
        }

        snippet
    }
}

/// Error suggestions based on common mistakes.
pub struct ErrorSuggester;

impl ErrorSuggester {
    /// Suggests fixes for common Catala errors.
    pub fn suggest_catala_fix(error_msg: &str, source: &str) -> Option<String> {
        if error_msg.contains("scope") && !source.contains("declaration scope") {
            return Some("Did you forget 'declaration scope' before the scope name?".to_string());
        }

        if error_msg.contains("definition") && !source.contains("definition") {
            return Some(
                "Use 'definition <name> equals <expression>' to define values".to_string(),
            );
        }

        None
    }

    /// Suggests fixes for common L4 errors.
    pub fn suggest_l4_fix(error_msg: &str, source: &str) -> Option<String> {
        if error_msg.contains("RULE") && !source.contains("RULE") {
            return Some("L4 rules must start with the RULE keyword".to_string());
        }

        if error_msg.contains("deontic") || error_msg.contains("modality") {
            return Some(
                "L4 requires a deontic modality (MUST, MAY, SHANT) after THEN".to_string(),
            );
        }

        None
    }

    /// Suggests fixes for common Stipula errors.
    pub fn suggest_stipula_fix(error_msg: &str, source: &str) -> Option<String> {
        if error_msg.contains("agreement") && !source.contains("agreement") {
            return Some(
                "Stipula contracts must start with 'agreement <name>(parties)'".to_string(),
            );
        }

        if error_msg.contains("party") || error_msg.contains("parties") {
            return Some(
                "Check that all parties are declared in the agreement signature".to_string(),
            );
        }

        None
    }

    /// Suggests general formatting fixes.
    pub fn suggest_format_fix(format: LegalFormat, error_msg: &str) -> Option<String> {
        match format {
            LegalFormat::Catala => {
                if error_msg.contains("syntax") {
                    return Some(
                        "Catala uses indentation-sensitive syntax. Check your spacing.".to_string(),
                    );
                }
            }
            LegalFormat::L4 => {
                if error_msg.contains("keyword") {
                    return Some(
                        "L4 keywords are case-sensitive: RULE, WHEN, THEN, MUST, MAY, SHANT"
                            .to_string(),
                    );
                }
            }
            LegalFormat::Stipula => {
                if error_msg.contains("syntax") {
                    return Some(
                        "Stipula uses Java-like syntax with curly braces for blocks".to_string(),
                    );
                }
            }
            _ => {}
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contextual_error_creation() {
        let error = InteropError::ParseError("Test error".to_string());
        let ctx_error = ContextualError::from_error(error)
            .with_format(LegalFormat::Catala)
            .with_line(10)
            .with_column(5)
            .with_suggestion("Try adding 'declaration scope'");

        let msg = ctx_error.detailed_message();
        assert!(msg.contains("Test error"));
        assert!(msg.contains("line 10"));
        assert!(msg.contains("column 5"));
        assert!(msg.contains("Suggestion:"));
    }

    #[test]
    fn test_source_location_from_offset() {
        let source = "line 1\nline 2\nline 3";
        let loc = SourceLocation::from_offset(source, 7); // First char of "line 2"

        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 1);
    }

    #[test]
    fn test_source_location_snippet() {
        let source = "line 1\nline 2\nline 3\nline 4\nline 5";
        let loc = SourceLocation::from_offset(source, 14); // "line 3"

        let snippet = loc.extract_snippet(source, 1);

        assert!(snippet.contains("line 2"));
        assert!(snippet.contains("line 3"));
        assert!(snippet.contains("line 4"));
        assert!(snippet.contains(">")); // Marker for error line
    }

    #[test]
    fn test_error_suggester_catala() {
        let source = "scope Test:";
        let suggestion = ErrorSuggester::suggest_catala_fix("missing scope", source);

        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("declaration scope"));
    }

    #[test]
    fn test_error_suggester_l4() {
        let source = "WHEN age >= 18 THEN Person vote";
        let suggestion = ErrorSuggester::suggest_l4_fix("missing modality", source);

        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("MUST"));
    }

    #[test]
    fn test_error_suggester_stipula() {
        let source = "contract Test { }";
        let suggestion = ErrorSuggester::suggest_stipula_fix("missing agreement", source);

        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("agreement"));
    }

    #[test]
    fn test_contextual_error_with_context() {
        let error = InteropError::ConversionError("Failed to convert".to_string());
        let ctx_error = ContextualError::from_error(error)
            .add_context("Converting from Catala to L4")
            .add_context("Scope 'VotingRights' has unsupported features");

        let msg = ctx_error.detailed_message();
        assert!(msg.contains("Context:"));
        assert!(msg.contains("Converting from Catala to L4"));
        assert!(msg.contains("Scope 'VotingRights'"));
    }

    #[test]
    fn test_source_location_first_line() {
        let source = "test";
        let loc = SourceLocation::from_offset(source, 0);

        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 1);
    }

    #[test]
    fn test_source_location_multiline_offset() {
        let source = "abc\ndefgh\nijklm";
        let loc = SourceLocation::from_offset(source, 10); // 'i' in "ijklm" (first char of line 3)

        assert_eq!(loc.line, 3);
        assert_eq!(loc.column, 1);
    }

    #[test]
    fn test_error_suggester_format_specific() {
        let suggestion =
            ErrorSuggester::suggest_format_fix(LegalFormat::L4, "Unknown keyword 'must'");

        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("case-sensitive"));
    }

    #[test]
    fn test_contextual_error_display() {
        let error = InteropError::ParseError("Syntax error".to_string());
        let ctx_error = ContextualError::from_error(error)
            .with_format(LegalFormat::L4)
            .with_line(5);

        let display = format!("{}", ctx_error);
        assert!(display.contains("Syntax error"));
        assert!(display.contains("L4"));
    }
}
