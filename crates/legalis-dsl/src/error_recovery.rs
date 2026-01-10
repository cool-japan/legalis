//! Error recovery mechanisms for robust parsing (v0.1.6).
//!
//! This module provides panic mode recovery, missing delimiter insertion,
//! contextual error messages, and multi-error reporting for the parser.

use crate::ast::Token;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Parser recovery strategy
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// Panic mode: skip tokens until synchronization point
    PanicMode { sync_tokens: Vec<Token> },
    /// Insert missing delimiter
    InsertDelimiter { missing: Token },
    /// Try alternative parse
    Alternative { description: String },
    /// Skip current token and continue
    SkipToken,
}

/// Error with recovery information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoverableError {
    /// The error message
    pub message: String,
    /// Location in source (line, column)
    pub location: Option<(usize, usize)>,
    /// Suggested recovery action
    pub recovery_hint: String,
    /// Whether recovery was attempted
    pub recovered: bool,
    /// Severity level
    pub severity: ErrorSeverity,
}

/// Error severity level
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Error - parsing failed
    Error,
    /// Warning - recovered but may be incorrect
    Warning,
    /// Info - informational message
    Info,
}

impl fmt::Display for RecoverableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}",
            match self.severity {
                ErrorSeverity::Error => "ERROR",
                ErrorSeverity::Warning => "WARNING",
                ErrorSeverity::Info => "INFO",
            },
            self.message
        )?;

        if let Some((line, col)) = self.location {
            write!(f, " at line {}, column {}", line, col)?;
        }

        if !self.recovery_hint.is_empty() {
            write!(f, "\n  Hint: {}", self.recovery_hint)?;
        }

        if self.recovered {
            write!(f, "\n  (Recovered and continued parsing)")?;
        }

        Ok(())
    }
}

/// Multi-error collector for reporting multiple errors per parse
#[derive(Debug, Clone)]
pub struct ErrorCollector {
    /// Collected errors
    errors: Vec<RecoverableError>,
    /// Maximum errors before giving up
    max_errors: usize,
    /// Whether to attempt recovery
    enable_recovery: bool,
}

impl ErrorCollector {
    /// Creates a new error collector
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            max_errors: 10,
            enable_recovery: true,
        }
    }

    /// Creates a collector with custom max errors
    pub fn with_max_errors(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            max_errors,
            enable_recovery: true,
        }
    }

    /// Disables recovery (fail fast mode)
    pub fn disable_recovery(mut self) -> Self {
        self.enable_recovery = false;
        self
    }

    /// Adds an error to the collection
    pub fn add_error(&mut self, error: RecoverableError) {
        self.errors.push(error);
    }

    /// Creates and adds an error
    pub fn report(
        &mut self,
        message: String,
        location: Option<(usize, usize)>,
        severity: ErrorSeverity,
    ) {
        self.add_error(RecoverableError {
            message,
            location,
            recovery_hint: String::new(),
            recovered: false,
            severity,
        });
    }

    /// Creates and adds an error with recovery hint
    pub fn report_with_hint(
        &mut self,
        message: String,
        location: Option<(usize, usize)>,
        hint: String,
        severity: ErrorSeverity,
    ) {
        self.add_error(RecoverableError {
            message,
            location,
            recovery_hint: hint,
            recovered: false,
            severity,
        });
    }

    /// Checks if should continue parsing
    pub fn should_continue(&self) -> bool {
        self.error_count() < self.max_errors
    }

    /// Returns whether recovery is enabled
    pub fn is_recovery_enabled(&self) -> bool {
        self.enable_recovery
    }

    /// Returns the number of errors
    pub fn error_count(&self) -> usize {
        self.errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Error))
            .count()
    }

    /// Returns the number of warnings
    pub fn warning_count(&self) -> usize {
        self.errors
            .iter()
            .filter(|e| matches!(e.severity, ErrorSeverity::Warning))
            .count()
    }

    /// Returns all errors
    pub fn errors(&self) -> &[RecoverableError] {
        &self.errors
    }

    /// Checks if there are any errors
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    /// Formats all errors as a single message
    pub fn format_all(&self) -> String {
        if self.errors.is_empty() {
            return "No errors".to_string();
        }

        let mut result = String::new();
        result.push_str(&format!(
            "Found {} error(s) and {} warning(s):\n\n",
            self.error_count(),
            self.warning_count()
        ));

        for (i, error) in self.errors.iter().enumerate() {
            result.push_str(&format!("{}. {}\n", i + 1, error));
        }

        result
    }

    /// Clears all errors
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}

impl Default for ErrorCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Panic mode recovery helper
pub struct PanicModeRecovery {
    /// Synchronization tokens (where to stop skipping)
    sync_tokens: Vec<Token>,
}

impl PanicModeRecovery {
    /// Creates a new panic mode recovery with default sync tokens
    pub fn new() -> Self {
        Self {
            sync_tokens: vec![Token::Statute, Token::When, Token::Then, Token::RBrace],
        }
    }

    /// Creates recovery with custom sync tokens
    pub fn with_sync_tokens(sync_tokens: Vec<Token>) -> Self {
        Self { sync_tokens }
    }

    /// Recovers by skipping tokens until a sync point
    pub fn recover<I>(&self, tokens: &mut std::iter::Peekable<I>) -> bool
    where
        I: Iterator<Item = Token>,
    {
        let mut skipped = 0;
        while let Some(token) = tokens.peek() {
            if self.is_sync_token(token) {
                return true;
            }
            tokens.next();
            skipped += 1;
            if skipped > 100 {
                // Prevent infinite loops
                return false;
            }
        }
        false
    }

    /// Checks if a token is a synchronization point
    fn is_sync_token(&self, token: &Token) -> bool {
        self.sync_tokens
            .iter()
            .any(|t| std::mem::discriminant(t) == std::mem::discriminant(token))
    }
}

impl Default for PanicModeRecovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Missing delimiter detector and inserter
pub struct DelimiterInserter;

impl DelimiterInserter {
    /// Detects missing closing delimiter
    pub fn detect_missing_close(open: &Token) -> Option<Token> {
        match open {
            Token::LParen => Some(Token::RParen),
            Token::LBrace => Some(Token::RBrace),
            _ => None,
        }
    }

    /// Detects missing opening delimiter
    pub fn detect_missing_open(close: &Token) -> Option<Token> {
        match close {
            Token::RParen => Some(Token::LParen),
            Token::RBrace => Some(Token::LBrace),
            _ => None,
        }
    }

    /// Suggests delimiter insertion
    pub fn suggest_insertion(expected: &Token, found: &Token) -> Option<String> {
        // If we expected a closing delimiter but didn't find it
        if Self::is_closing_delimiter(expected) && !Self::is_closing_delimiter(found) {
            return Some(format!("Insert missing '{}'", Self::token_symbol(expected)));
        }

        // If we found a closing delimiter but expected something else
        if Self::is_closing_delimiter(found) {
            if let Some(open) = Self::detect_missing_open(found) {
                return Some(format!(
                    "Missing opening '{}' for closing '{}'",
                    Self::token_symbol(&open),
                    Self::token_symbol(found)
                ));
            }
        }

        None
    }

    /// Checks if token is a closing delimiter
    fn is_closing_delimiter(token: &Token) -> bool {
        matches!(token, Token::RParen | Token::RBrace)
    }

    /// Gets symbol representation of a token
    fn token_symbol(token: &Token) -> &'static str {
        match token {
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Comma => ",",
            _ => "token",
        }
    }
}

/// Contextual error message generator
pub struct ContextualErrorGenerator;

impl ContextualErrorGenerator {
    /// Generates a contextual error message based on parser state
    pub fn generate_message(
        expected: &[&str],
        found: Option<&Token>,
        context: ParserContext,
    ) -> String {
        let found_str = if let Some(token) = found {
            format!("{:?}", token)
        } else {
            "end of input".to_string()
        };

        let context_str = match context {
            ParserContext::StatuteDefinition => "while parsing statute definition",
            ParserContext::ConditionBlock => "while parsing condition block",
            ParserContext::EffectBlock => "while parsing effect block",
            ParserContext::MetadataSection => "while parsing metadata section",
            ParserContext::ImportStatement => "while parsing import statement",
            ParserContext::NamespaceDeclaration => "while parsing namespace declaration",
            ParserContext::MacroDefinition => "while parsing macro definition",
        };

        if expected.len() == 1 {
            format!(
                "Expected {} but found {} {}",
                expected[0], found_str, context_str
            )
        } else {
            format!(
                "Expected one of [{}] but found {} {}",
                expected.join(", "),
                found_str,
                context_str
            )
        }
    }

    /// Suggests correction for a typo
    pub fn suggest_correction(word: &str, candidates: &[&str]) -> Option<String> {
        let mut best_match = None;
        let mut best_distance = usize::MAX;

        for candidate in candidates {
            let distance = levenshtein_distance(word, candidate);
            if distance < best_distance && distance <= 2 {
                best_distance = distance;
                best_match = Some(*candidate);
            }
        }

        best_match.map(|s| format!("Did you mean '{}'?", s))
    }
}

/// Parser context for contextual error messages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserContext {
    /// Parsing a statute definition
    StatuteDefinition,
    /// Parsing a condition block
    ConditionBlock,
    /// Parsing an effect block
    EffectBlock,
    /// Parsing metadata section
    MetadataSection,
    /// Parsing import statement
    ImportStatement,
    /// Parsing namespace declaration
    NamespaceDeclaration,
    /// Parsing macro definition
    MacroDefinition,
}

/// Calculates Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
        row[0] = i;
    }
    for (j, cell) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
        *cell = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_collector_new() {
        let collector = ErrorCollector::new();
        assert_eq!(collector.error_count(), 0);
        assert!(!collector.has_errors());
    }

    #[test]
    fn test_error_collector_add() {
        let mut collector = ErrorCollector::new();
        collector.report("Test error".to_string(), Some((1, 1)), ErrorSeverity::Error);
        assert_eq!(collector.error_count(), 1);
        assert!(collector.has_errors());
    }

    #[test]
    fn test_error_collector_max_errors() {
        let mut collector = ErrorCollector::with_max_errors(3);
        collector.report("Error 1".to_string(), None, ErrorSeverity::Error);
        assert!(collector.should_continue());
        collector.report("Error 2".to_string(), None, ErrorSeverity::Error);
        assert!(collector.should_continue());
        collector.report("Error 3".to_string(), None, ErrorSeverity::Error);
        assert!(!collector.should_continue());
    }

    #[test]
    fn test_error_severity_counting() {
        let mut collector = ErrorCollector::new();
        collector.report("Error".to_string(), None, ErrorSeverity::Error);
        collector.report("Warning".to_string(), None, ErrorSeverity::Warning);
        collector.report("Info".to_string(), None, ErrorSeverity::Info);

        assert_eq!(collector.error_count(), 1);
        assert_eq!(collector.warning_count(), 1);
    }

    #[test]
    fn test_panic_mode_recovery() {
        let recovery = PanicModeRecovery::new();
        assert!(!recovery.sync_tokens.is_empty());
    }

    #[test]
    fn test_delimiter_detection() {
        assert_eq!(
            DelimiterInserter::detect_missing_close(&Token::LParen),
            Some(Token::RParen)
        );
        assert_eq!(
            DelimiterInserter::detect_missing_close(&Token::LBrace),
            Some(Token::RBrace)
        );
        assert_eq!(
            DelimiterInserter::detect_missing_open(&Token::RParen),
            Some(Token::LParen)
        );
    }

    #[test]
    fn test_delimiter_suggestion() {
        let suggestion = DelimiterInserter::suggest_insertion(&Token::RParen, &Token::Statute);
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("Insert missing"));
    }

    #[test]
    fn test_contextual_error_message() {
        let msg = ContextualErrorGenerator::generate_message(
            &["STATUTE"],
            Some(&Token::When),
            ParserContext::StatuteDefinition,
        );
        assert!(msg.contains("Expected"));
        assert!(msg.contains("while parsing"));
    }

    #[test]
    fn test_typo_correction() {
        let candidates = &["STATUTE", "WHEN", "THEN"];
        let suggestion = ContextualErrorGenerator::suggest_correction("STATUT", candidates);
        assert_eq!(suggestion, Some("Did you mean 'STATUTE'?".to_string()));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("cat", "cat"), 0);
        assert_eq!(levenshtein_distance("cat", "bat"), 1);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn test_error_formatting() {
        let error = RecoverableError {
            message: "Test error".to_string(),
            location: Some((10, 5)),
            recovery_hint: "Try adding a semicolon".to_string(),
            recovered: true,
            severity: ErrorSeverity::Error,
        };

        let formatted = error.to_string();
        assert!(formatted.contains("ERROR"));
        assert!(formatted.contains("line 10"));
        assert!(formatted.contains("Hint"));
        assert!(formatted.contains("Recovered"));
    }

    #[test]
    fn test_recovery_disabled() {
        let collector = ErrorCollector::new().disable_recovery();
        assert!(!collector.is_recovery_enabled());
    }

    #[test]
    fn test_clear_errors() {
        let mut collector = ErrorCollector::new();
        collector.report("Error".to_string(), None, ErrorSeverity::Error);
        assert!(collector.has_errors());
        collector.clear();
        assert!(!collector.has_errors());
    }
}
