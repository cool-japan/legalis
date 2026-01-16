//! Legalis-DSL: Domain Specific Language for legal document parsing.
//!
//! This crate provides parsing and AST representation for legal documents,
//! enabling structured representation of statutes and legal rules.
//!
//! ## Grammar
//!
//! ```text
//! STATUTE ::= "STATUTE" ID ":" TITLE "{" BODY "}"
//! BODY ::= (METADATA | DEFAULT | WHEN | THEN | DISCRETION | EXCEPTION | AMENDMENT | SUPERSEDES)*
//! METADATA ::= EFFECTIVE_DATE | EXPIRY_DATE | JURISDICTION | VERSION
//! EFFECTIVE_DATE ::= ("EFFECTIVE_DATE" | "EFFECTIVE") DATE
//! EXPIRY_DATE ::= ("EXPIRY_DATE" | "EXPIRY" | "EXPIRES") DATE
//! JURISDICTION ::= "JURISDICTION" (STRING | IDENT)
//! VERSION ::= "VERSION" NUMBER
//! DATE ::= YYYY "-" MM "-" DD | STRING
//! DEFAULT ::= "DEFAULT" IDENT ("=" | ":") VALUE
//! WHEN ::= "WHEN" CONDITION
//! CONDITION ::= OR_EXPR
//! OR_EXPR ::= AND_EXPR ("OR" AND_EXPR)*
//! AND_EXPR ::= UNARY_EXPR ("AND" UNARY_EXPR)*
//! UNARY_EXPR ::= "NOT" UNARY_EXPR | "(" CONDITION ")" | PRIMARY_COND
//! PRIMARY_COND ::= FIELD_COND | "HAS" IDENT | IDENT
//! FIELD_COND ::= FIELD (COMPARISON_OP VALUE | "BETWEEN" VALUE "AND" VALUE | "IN" VALUE_LIST | "LIKE" PATTERN)
//! FIELD ::= "AGE" | "INCOME" | IDENT
//! VALUE_LIST ::= "(" VALUE ("," VALUE)* ")" | VALUE ("," VALUE)*
//! THEN ::= "THEN" EFFECT
//! EFFECT ::= ("GRANT" | "REVOKE" | "OBLIGATION" | "PROHIBITION") STRING
//! DISCRETION ::= "DISCRETION" STRING
//! EXCEPTION ::= "EXCEPTION" ["WHEN" CONDITION] STRING
//! AMENDMENT ::= "AMENDMENT" IDENT ["VERSION" NUMBER] ["EFFECTIVE_DATE" DATE] STRING
//! SUPERSEDES ::= "SUPERSEDES" IDENT ("," IDENT)*
//! ```
//!
//! ## Comments
//!
//! The DSL supports both line comments (`//`) and block comments (`/* */`).
//!
//! ## Example
//!
//! ```text
//! STATUTE adult-voting: "Adult Voting Rights" {
//!     JURISDICTION "US-CA"
//!     VERSION 2
//!     EFFECTIVE_DATE 2024-01-01
//!     EXPIRY_DATE 2030-12-31
//!     DEFAULT status "pending"
//!     WHEN AGE BETWEEN 18 AND 120 AND HAS citizen
//!     THEN GRANT "Right to vote"
//!     EXCEPTION WHEN AGE < 18 AND HAS guardian_consent "Minors with parental consent"
//!     DISCRETION "Consider residency requirements"
//! }
//! ```
//!
//! ## Advanced Features
//!
//! The DSL supports advanced condition operators:
//! - `BETWEEN`: Range checking (e.g., `AGE BETWEEN 18 AND 65`)
//! - `IN`: Set membership (e.g., `AGE IN (18, 21, 25)`)
//! - `LIKE`: Pattern matching (e.g., `INCOME LIKE "consulting%"`)
//! - `DEFAULT`: Default values for attributes (e.g., `DEFAULT status "pending"`)
//! - `EXCEPTION`: Exception clauses (e.g., `EXCEPTION WHEN condition "description"`)
//! - `AMENDMENT`: Version tracking (e.g., `AMENDMENT old-law VERSION 2 "Updated rules"`)
//! - `SUPERSEDES`: Replacing old statutes (e.g., `SUPERSEDES old-law, legacy-law`)

use chrono::NaiveDate;
use legalis_core::{Condition, Effect, EffectType, Statute, TemporalValidity};
use thiserror::Error;

mod ast;
pub mod autofix;
pub mod cache;
pub mod codegen;
pub mod completion;
pub mod compliance;
pub mod consistency;
pub mod dataflow;
pub mod diff;
pub mod docgen;
pub mod error_explainer;
pub mod error_recovery;
pub mod grammar_doc;
pub mod graph;
pub mod heredoc;
pub mod htmlgen;
pub mod import_resolver;
pub mod incremental;
pub mod interpolation;
pub mod lsp;
pub mod macros;
pub mod metadata;
pub mod module_system;
pub mod multilang;
pub mod mutation;
pub mod nl_to_dsl;
pub mod nlgen;
pub mod numeric;
pub mod optimizer;
mod parser;
mod printer;
pub mod profiler;
pub mod query;
pub mod search_index;
pub mod statistics;
pub mod taint;
pub mod templates;
pub mod transform;
pub mod tree_view;
pub mod type_checker;
pub mod validation;
pub mod watch;

#[cfg(test)]
mod tests;

pub use ast::*;
pub use autofix::{AutoFixer, Fix, FixCategory, FixPattern, FixReport};
pub use cache::{CacheKey, CacheStats, CachingParser, ParseCache};
pub use codegen::{
    CSharpGenerator, CodeGenerator, GoGenerator, JavaGenerator, PrologGenerator, PythonGenerator,
    RustGenerator, SqlGenerator, TypeScriptGenerator,
};
pub use completion::{CompletionCategory, CompletionContext, CompletionItem, CompletionProvider};
pub use compliance::{ComplianceMatrix, ComplianceStats};
pub use consistency::{ConsistencyChecker, ConsistencyIssue};
pub use dataflow::{DataFlowAnalyzer, DataFlowIssue, DataFlowState};
pub use diff::{Change, DocumentDiff, StatuteDiff};
pub use docgen::{DocGenerator, LaTeXGenerator, MarkdownGenerator};
pub use error_explainer::{ErrorExplainer, ErrorExplanation, ErrorSeverity};
pub use grammar_doc::{GrammarRule, GrammarSpec, legalis_grammar};
pub use graph::{
    DependencyGraph, GraphFormat, GraphOptions, generate_dot_graph, generate_mermaid_graph,
};
pub use heredoc::{HeredocError, HeredocParser, HeredocResult, HeredocType, parse_heredoc};
pub use htmlgen::{HtmlGenerator, HtmlTheme};
pub use import_resolver::{ImportResolver, detect_circular_imports, validate_import_paths};
pub use incremental::{IncrementalParser, TextEdit};
pub use interpolation::{
    InterpolationError, InterpolationEvaluator, InterpolationParser, Token as InterpolationToken,
    extract_variables, interpolate,
};
pub use metadata::{
    AmendmentAuditTrail, AuditEntry, EntityRelationships, JurisdictionHierarchy, VersionEntry,
    VersionHistory,
};
pub use module_system::{ExportNode, ImportKind, NamespaceNode, Visibility};
pub use multilang::{DslLanguage, KeywordMapping, LanguageExamples, MultiLangTranslator};
pub use mutation::{Mutation, MutationOperator, MutationReport, MutationResult, MutationType};
pub use nl_to_dsl::{
    CommonTemplates, NLPattern, NLTranslator, TranslationResult, TranslatorBuilder,
};
pub use nlgen::{Language, NLConfig, NLGenerator, Verbosity};
pub use numeric::{NumericError, NumericParser, NumericValue, parse_numeric};
pub use parser::*;
pub use printer::*;
pub use profiler::{ParseProfiler, ProfileComparison, ProfileReport, Profiler};
pub use query::{ConditionSearch, StatuteQuery};
pub use search_index::{IndexStats, SearchIndex, SearchResult, StatuteMetadata};
pub use statistics::{
    ComplexityMetrics, DependencyAnalysis, DocumentStatistics, analyze_complexity,
};
pub use taint::{TaintAnalyzer, TaintCategory, TaintConfig, TaintInfo, TaintReport};
pub use templates::{StatuteTemplate, TemplateBuilder, TemplateLibrary};
pub use transform::{
    ConditionTransform, DeduplicateStatutes, DocumentTransform, NormalizeIds, RemoveEmptyStatutes,
    SimplifyConditions, SortByDependencies, StatuteTransform, TransformPipeline,
};
pub use tree_view::TreeFormatter;
pub use type_checker::{Type, TypeChecker, TypeContext, TypeError};
pub use validation::{CompletenessChecker, SemanticValidator, ValidationContext, ValidationError};
pub use watch::{FileWatcher, ValidationResult, WatchConfig};

/// Serializes a LegalDocument AST to JSON string.
pub fn to_json(doc: &ast::LegalDocument) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(doc)
}

/// Deserializes a LegalDocument AST from JSON string.
pub fn from_json(json: &str) -> Result<ast::LegalDocument, serde_json::Error> {
    serde_json::from_str(json)
}

/// Serializes a StatuteNode AST to JSON string.
pub fn statute_to_json(statute: &ast::StatuteNode) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(statute)
}

/// Deserializes a StatuteNode AST from JSON string.
pub fn statute_from_json(json: &str) -> Result<ast::StatuteNode, serde_json::Error> {
    serde_json::from_str(json)
}

/// Serializes a LegalDocument AST to YAML string.
pub fn to_yaml(doc: &ast::LegalDocument) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(doc)
}

/// Deserializes a LegalDocument AST from YAML string.
pub fn from_yaml(yaml: &str) -> Result<ast::LegalDocument, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Serializes a StatuteNode AST to YAML string.
pub fn statute_to_yaml(statute: &ast::StatuteNode) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(statute)
}

/// Deserializes a StatuteNode AST from YAML string.
pub fn statute_from_yaml(yaml: &str) -> Result<ast::StatuteNode, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Serializes a LegalDocument AST to TOML string.
pub fn to_toml(doc: &ast::LegalDocument) -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(doc)
}

/// Deserializes a LegalDocument AST from TOML string.
pub fn from_toml(toml_str: &str) -> Result<ast::LegalDocument, toml::de::Error> {
    toml::from_str(toml_str)
}

/// Serializes a StatuteNode AST to TOML string.
pub fn statute_to_toml(statute: &ast::StatuteNode) -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(statute)
}

/// Deserializes a StatuteNode AST from TOML string.
pub fn statute_from_toml(toml_str: &str) -> Result<ast::StatuteNode, toml::de::Error> {
    toml::from_str(toml_str)
}

/// Source location for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceLocation {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Byte offset from start of input
    pub offset: usize,
}

impl SourceLocation {
    /// Creates a new source location.
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }

    /// Creates a source location from a byte offset by scanning the input.
    pub fn from_offset(offset: usize, input: &str) -> Self {
        let mut line = 1;
        let mut column = 1;
        for (idx, ch) in input.char_indices() {
            if idx >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        Self {
            line,
            column,
            offset,
        }
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Source span representing a range in the source code.
/// Useful for IDE integration and error highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceSpan {
    /// Start location
    pub start: SourceLocation,
    /// End location
    pub end: SourceLocation,
}

impl SourceSpan {
    /// Creates a new source span.
    pub fn new(start: SourceLocation, end: SourceLocation) -> Self {
        Self { start, end }
    }

    /// Creates a span from a single location (zero-width span).
    pub fn from_location(loc: SourceLocation) -> Self {
        Self {
            start: loc,
            end: loc,
        }
    }

    /// Creates a span from byte offsets by scanning the input.
    pub fn from_offsets(start_offset: usize, end_offset: usize, input: &str) -> Self {
        let start = SourceLocation::from_offset(start_offset, input);
        let end = SourceLocation::from_offset(end_offset, input);
        Self { start, end }
    }

    /// Returns the length of the span in bytes.
    pub fn len(&self) -> usize {
        self.end.offset.saturating_sub(self.start.offset)
    }

    /// Returns true if the span is empty (zero-width).
    pub fn is_empty(&self) -> bool {
        self.start.offset == self.end.offset
    }

    /// Extracts the text covered by this span from the input.
    pub fn text<'a>(&self, input: &'a str) -> &'a str {
        let start = self.start.offset;
        let end = self.end.offset.min(input.len());
        &input[start..end]
    }
}

impl std::fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start.line == self.end.line {
            write!(
                f,
                "{}:{}-{}",
                self.start.line, self.start.column, self.end.column
            )
        } else {
            write!(
                f,
                "{}:{} to {}:{}",
                self.start.line, self.start.column, self.end.line, self.end.column
            )
        }
    }
}

/// Warnings that can be emitted during DSL parsing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DslWarning {
    /// Deprecated syntax warning
    DeprecatedSyntax {
        location: SourceLocation,
        old_syntax: String,
        new_syntax: String,
        message: String,
    },
    /// Redundant condition warning
    RedundantCondition {
        location: SourceLocation,
        description: String,
    },
    /// Unused import warning
    UnusedImport {
        location: SourceLocation,
        import_path: String,
    },
}

impl DslWarning {
    /// Returns the source location of this warning.
    pub fn location(&self) -> &SourceLocation {
        match self {
            Self::DeprecatedSyntax { location, .. }
            | Self::RedundantCondition { location, .. }
            | Self::UnusedImport { location, .. } => location,
        }
    }
}

impl std::fmt::Display for DslWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeprecatedSyntax {
                location,
                old_syntax,
                new_syntax,
                message,
            } => write!(
                f,
                "Warning at {}: Deprecated syntax '{}' used. Use '{}' instead. {}",
                location, old_syntax, new_syntax, message
            ),
            Self::RedundantCondition {
                location,
                description,
            } => write!(
                f,
                "Warning at {}: Redundant condition: {}",
                location, description
            ),
            Self::UnusedImport {
                location,
                import_path,
            } => write!(
                f,
                "Warning at {}: Unused import '{}'",
                location, import_path
            ),
        }
    }
}

/// Errors that can occur during DSL parsing.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum DslError {
    #[error("Parse error at {}: {message}", location.map(|l| l.to_string()).unwrap_or_else(|| "unknown".to_string()))]
    ParseError {
        location: Option<SourceLocation>,
        message: String,
    },

    #[error("Invalid condition: {0}")]
    InvalidCondition(String),

    #[error("Invalid effect: {0}")]
    InvalidEffect(String),

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Unclosed comment starting at {}", .0.map(|l| l.to_string()).unwrap_or_else(|| "unknown".to_string()))]
    UnclosedComment(Option<SourceLocation>),

    #[error("Unmatched parenthesis at {}", .0.map(|l| l.to_string()).unwrap_or_else(|| "unknown".to_string()))]
    UnmatchedParen(Option<SourceLocation>),

    #[error("Syntax error at {location}: {message}\nExpected: {expected}\nFound: {found}{}", hint.as_ref().map(|h| format!("\nHint: {}", h)).unwrap_or_default())]
    SyntaxError {
        location: SourceLocation,
        message: String,
        expected: String,
        found: String,
        hint: Option<String>,
    },

    #[error("Undefined reference at {location}: {name}\n{}", hint.as_ref().map(|h| format!("Hint: {}", h)).unwrap_or_default())]
    UndefinedReference {
        location: SourceLocation,
        name: String,
        hint: Option<String>,
    },

    #[error("Syntax error at {span}: {message}{}", hint.as_ref().map(|h| format!("\nHint: {}", h)).unwrap_or_default())]
    SyntaxErrorWithSpan {
        span: SourceSpan,
        message: String,
        hint: Option<String>,
    },
}

impl DslError {
    /// Creates a parse error with location.
    pub fn parse_error_at(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::ParseError {
            location: Some(SourceLocation::new(line, column, 0)),
            message: message.into(),
        }
    }

    /// Creates a parse error without location (for backward compatibility).
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self::ParseError {
            location: None,
            message: message.into(),
        }
    }

    /// Creates a syntax error with context and optional hint.
    pub fn syntax_error(
        location: SourceLocation,
        message: impl Into<String>,
        expected: impl Into<String>,
        found: impl Into<String>,
        hint: Option<String>,
    ) -> Self {
        Self::SyntaxError {
            location,
            message: message.into(),
            expected: expected.into(),
            found: found.into(),
            hint,
        }
    }

    /// Creates an undefined reference error with optional suggestion.
    pub fn undefined_reference(
        location: SourceLocation,
        name: impl Into<String>,
        hint: Option<String>,
    ) -> Self {
        Self::UndefinedReference {
            location,
            name: name.into(),
            hint,
        }
    }

    /// Creates a syntax error with span for IDE integration.
    pub fn syntax_error_with_span(
        span: SourceSpan,
        message: impl Into<String>,
        hint: Option<String>,
    ) -> Self {
        Self::SyntaxErrorWithSpan {
            span,
            message: message.into(),
            hint,
        }
    }

    /// Extracts the span from this error, if available.
    pub fn span(&self) -> Option<SourceSpan> {
        match self {
            Self::SyntaxErrorWithSpan { span, .. } => Some(*span),
            Self::SyntaxError { location, .. } | Self::UndefinedReference { location, .. } => {
                Some(SourceSpan::from_location(*location))
            }
            Self::ParseError {
                location: Some(loc),
                ..
            } => Some(SourceSpan::from_location(*loc)),
            Self::UnclosedComment(Some(loc)) | Self::UnmatchedParen(Some(loc)) => {
                Some(SourceSpan::from_location(*loc))
            }
            _ => None,
        }
    }
}

/// Calculates the Levenshtein distance between two strings.
/// Used for "did you mean?" suggestions.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut prev_row: Vec<usize> = (0..=b_len).collect();
    let mut curr_row = vec![0; b_len + 1];

    for (i, a_char) in a.chars().enumerate() {
        curr_row[0] = i + 1;

        for (j, b_char) in b.chars().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            curr_row[j + 1] = (curr_row[j] + 1)
                .min(prev_row[j + 1] + 1)
                .min(prev_row[j] + cost);
        }

        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[b_len]
}

/// Finds the closest match from a list of valid keywords.
/// Returns None if no close match is found.
pub fn suggest_keyword(input: &str, valid_keywords: &[&str]) -> Option<String> {
    let input_upper = input.to_uppercase();

    let mut best_match: Option<(&str, usize)> = None;

    for &keyword in valid_keywords {
        let distance = levenshtein_distance(&input_upper, keyword);

        // Only suggest if distance is small (typo threshold)
        if distance <= 2 {
            match best_match {
                None => best_match = Some((keyword, distance)),
                Some((_, best_distance)) if distance < best_distance => {
                    best_match = Some((keyword, distance));
                }
                _ => {}
            }
        }
    }

    best_match.map(|(keyword, _)| keyword.to_string())
}

/// Result type for DSL operations.
pub type DslResult<T> = Result<T, DslError>;

/// A partial parse result that contains both parsed content and errors.
/// This is used for error recovery, allowing the parser to continue
/// parsing and collect multiple errors instead of failing at the first one.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseResult<T> {
    /// The partially parsed result (may be incomplete)
    pub result: Option<T>,
    /// Errors encountered during parsing
    pub errors: Vec<DslError>,
}

impl<T> ParseResult<T> {
    /// Creates a successful parse result with no errors.
    pub fn ok(value: T) -> Self {
        Self {
            result: Some(value),
            errors: Vec::new(),
        }
    }

    /// Creates a parse result with errors and optionally a partial result.
    pub fn with_errors(result: Option<T>, errors: Vec<DslError>) -> Self {
        Self { result, errors }
    }

    /// Creates a parse result with a single error.
    pub fn err(error: DslError) -> Self {
        Self {
            result: None,
            errors: vec![error],
        }
    }

    /// Returns true if there are no errors.
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns true if there are errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Converts to a Result, returning the first error if any exist.
    pub fn into_result(self) -> DslResult<T> {
        if let Some(err) = self.errors.into_iter().next() {
            Err(err)
        } else if let Some(result) = self.result {
            Ok(result)
        } else {
            Err(DslError::parse_error("No result and no errors"))
        }
    }
}

/// A simple DSL parser for legal rules.
///
/// Grammar (simplified):
/// ```text
/// STATUTE ::= "STATUTE" ID ":" TITLE "{" BODY "}"
/// BODY ::= (WHEN | THEN | DISCRETION)*
/// WHEN ::= "WHEN" CONDITION
/// THEN ::= "THEN" EFFECT
/// DISCRETION ::= "DISCRETION" STRING
/// ```
#[derive(Debug, Default)]
pub struct LegalDslParser {
    /// Collected warnings during parsing
    warnings: std::cell::RefCell<Vec<DslWarning>>,
}

impl LegalDslParser {
    /// Creates a new parser instance.
    pub fn new() -> Self {
        Self {
            warnings: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// Returns the collected warnings from the last parse operation.
    pub fn warnings(&self) -> Vec<DslWarning> {
        self.warnings.borrow().clone()
    }

    /// Clears all collected warnings.
    pub fn clear_warnings(&self) {
        self.warnings.borrow_mut().clear();
    }

    /// Emits a warning.
    fn emit_warning(&self, warning: DslWarning) {
        self.warnings.borrow_mut().push(warning);
    }

    /// Parses a statute from DSL text.
    pub fn parse_statute(&self, input: &str) -> DslResult<Statute> {
        let spanned_tokens = self.tokenize(input)?;
        let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
        self.parse_tokens(&tokens)
    }

    /// Parses multiple statutes from a DSL text.
    /// The text can contain multiple STATUTE blocks.
    pub fn parse_statutes(&self, input: &str) -> DslResult<Vec<Statute>> {
        let spanned_tokens = self.tokenize(input)?;
        let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
        let mut statutes = Vec::new();
        let mut iter = tokens.iter().peekable();

        while iter.peek().is_some() {
            // Skip until we find a STATUTE keyword
            while let Some(token) = iter.peek() {
                if matches!(token, Token::Statute) {
                    break;
                }
                iter.next();
            }

            if iter.peek().is_none() {
                break;
            }

            // Collect tokens for this statute until the next STATUTE or end
            let mut statute_tokens = Vec::new();
            let mut brace_depth = 0;
            let mut started = false;

            while let Some(&token) = iter.peek() {
                if started && brace_depth == 0 && matches!(token, Token::Statute) {
                    break;
                }

                let token = iter.next().unwrap().clone();
                match &token {
                    Token::LBrace => {
                        started = true;
                        brace_depth += 1;
                    }
                    Token::RBrace => {
                        brace_depth -= 1;
                    }
                    _ => {}
                }
                statute_tokens.push(token);

                if started && brace_depth == 0 {
                    break;
                }
            }

            if !statute_tokens.is_empty() {
                let statute = self.parse_tokens(&statute_tokens)?;
                statutes.push(statute);
            }
        }

        if statutes.is_empty() {
            return Err(DslError::parse_error("No statutes found in input"));
        }

        Ok(statutes)
    }

    /// Parses a complete legal document with imports and statutes.
    /// Returns a LegalDocument AST containing both imports and statute nodes.
    pub fn parse_document(&self, input: &str) -> DslResult<ast::LegalDocument> {
        let spanned_tokens = self.tokenize(input)?;
        let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
        let mut iter = tokens.iter().peekable();

        // Parse namespace declaration (optional)
        let namespace = if matches!(iter.peek(), Some(Token::Namespace)) {
            Some(self.parse_namespace(&mut iter)?)
        } else {
            None
        };

        // Parse imports
        let mut imports = Vec::new();
        while matches!(iter.peek(), Some(Token::Import)) {
            imports.push(self.parse_import(&mut iter)?);
        }

        // Parse exports (optional)
        let mut exports = Vec::new();
        while matches!(iter.peek(), Some(Token::Export)) {
            exports.push(self.parse_export(&mut iter)?);
        }

        // Parse statutes
        let mut statutes = Vec::new();
        while iter.peek().is_some() {
            // Skip until we find a STATUTE keyword
            while let Some(token) = iter.peek() {
                if matches!(token, Token::Statute) {
                    break;
                }
                iter.next();
            }

            if iter.peek().is_none() {
                break;
            }

            // Collect tokens for this statute
            let mut statute_tokens = Vec::new();
            let mut brace_depth = 0;
            let mut started = false;

            while let Some(&token) = iter.peek() {
                if started && brace_depth == 0 && matches!(token, Token::Statute) {
                    break;
                }

                let token = iter.next().unwrap().clone();
                match &token {
                    Token::LBrace => {
                        started = true;
                        brace_depth += 1;
                    }
                    Token::RBrace => {
                        brace_depth -= 1;
                    }
                    _ => {}
                }
                statute_tokens.push(token);

                if started && brace_depth == 0 {
                    break;
                }
            }

            if !statute_tokens.is_empty() {
                let statute_node = self.parse_statute_node(&statute_tokens)?;
                statutes.push(statute_node);
            }
        }

        Ok(ast::LegalDocument {
            namespace,
            imports,
            exports,
            statutes,
        })
    }

    /// Parses a complete legal document with error recovery.
    /// Unlike `parse_document`, this method continues parsing even after
    /// encountering errors, collecting all errors and returning a partial AST.
    /// This is useful for IDE integration where you want to show multiple
    /// errors at once and provide syntax highlighting for valid parts.
    pub fn parse_document_with_recovery(&self, input: &str) -> ParseResult<ast::LegalDocument> {
        let spanned_tokens = match self.tokenize(input) {
            Ok(tokens) => tokens,
            Err(e) => return ParseResult::err(e),
        };
        let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
        let mut iter = tokens.iter().peekable();
        let mut errors = Vec::new();

        // Parse imports first
        let mut imports = Vec::new();
        while matches!(iter.peek(), Some(Token::Import)) {
            match self.parse_import(&mut iter) {
                Ok(import) => imports.push(import),
                Err(e) => {
                    errors.push(e);
                    // Try to recover by skipping to the next IMPORT or STATUTE
                    self.skip_to_sync_point(&mut iter);
                }
            }
        }

        // Parse statutes with error recovery
        let mut statutes = Vec::new();
        while iter.peek().is_some() {
            // Skip until we find a STATUTE keyword
            while let Some(token) = iter.peek() {
                if matches!(token, Token::Statute) {
                    break;
                }
                iter.next();
            }

            if iter.peek().is_none() {
                break;
            }

            // Collect tokens for this statute
            let mut statute_tokens = Vec::new();
            let mut brace_depth = 0;
            let mut started = false;

            while let Some(&token) = iter.peek() {
                if started && brace_depth == 0 && matches!(token, Token::Statute) {
                    break;
                }

                let token = iter.next().unwrap().clone();
                match &token {
                    Token::LBrace => {
                        started = true;
                        brace_depth += 1;
                    }
                    Token::RBrace => {
                        brace_depth -= 1;
                    }
                    _ => {}
                }
                statute_tokens.push(token);

                if started && brace_depth == 0 {
                    break;
                }
            }

            if !statute_tokens.is_empty() {
                match self.parse_statute_node(&statute_tokens) {
                    Ok(statute_node) => statutes.push(statute_node),
                    Err(e) => {
                        errors.push(e);
                        // Continue to next statute
                    }
                }
            }
        }

        let doc = ast::LegalDocument {
            namespace: None,
            imports,
            exports: vec![],
            statutes,
        };

        if errors.is_empty() {
            ParseResult::ok(doc)
        } else {
            ParseResult::with_errors(Some(doc), errors)
        }
    }

    /// Skips tokens until reaching a synchronization point.
    /// Synchronization points are: IMPORT, STATUTE, or EOF.
    fn skip_to_sync_point<'a, I>(&self, iter: &mut std::iter::Peekable<I>)
    where
        I: Iterator<Item = &'a Token>,
    {
        while let Some(token) = iter.peek() {
            if matches!(token, Token::Import | Token::Statute) {
                break;
            }
            iter.next();
        }
    }

    /// Parses an IMPORT statement.
    /// Supports:
    /// - Simple: IMPORT "path" [AS alias]
    /// - Wildcard: IMPORT path.*
    /// - Selective: IMPORT { item1, item2 } FROM path
    fn parse_import<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<ast::ImportNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Expect IMPORT
        match iter.next() {
            Some(Token::Import) => {}
            _ => return Err(DslError::parse_error("Expected 'IMPORT' keyword")),
        }

        // Check for selective import: IMPORT { ... }
        if matches!(iter.peek(), Some(Token::LBrace)) {
            iter.next(); // consume {

            let mut items = Vec::new();
            loop {
                match iter.next() {
                    Some(Token::Ident(id)) => items.push(id.clone()),
                    Some(Token::RBrace) => break,
                    Some(Token::Comma) => continue,
                    _ => {
                        return Err(DslError::parse_error(
                            "Expected identifier or '}' in import list",
                        ));
                    }
                }
            }

            // Expect FROM keyword
            match iter.next() {
                Some(Token::From) => {}
                _ => return Err(DslError::parse_error("Expected 'FROM' after import list")),
            }

            // Get path
            let path = match iter.next() {
                Some(Token::StringLit(s)) => s.clone(),
                Some(Token::Ident(s)) => s.clone(),
                _ => return Err(DslError::parse_error("Expected module path after 'FROM'")),
            };

            return Ok(ast::ImportNode {
                path,
                alias: None,
                kind: crate::module_system::ImportKind::Selective(items),
            });
        }

        // Check for wildcard or simple import
        let first_token = iter.next();
        let (path_part, is_ident) = match first_token {
            Some(Token::StringLit(s)) => (s.clone(), false),
            Some(Token::Ident(s)) => (s.clone(), true),
            _ => return Err(DslError::parse_error("Expected import path")),
        };

        // Check for wildcard import: path.*
        if is_ident && matches!(iter.peek(), Some(Token::Dot)) {
            iter.next(); // consume .
            // Not a wildcard, continue as simple import
            if let Some(Token::Star) = iter.peek() {
                iter.next(); // consume *
                return Ok(ast::ImportNode {
                    path: path_part,
                    alias: None,
                    kind: crate::module_system::ImportKind::Wildcard,
                });
            }
        }

        // Simple import - check for optional AS clause
        let alias = if matches!(iter.peek(), Some(Token::As)) {
            iter.next(); // consume AS
            match iter.next() {
                Some(Token::Ident(s)) => Some(s.clone()),
                _ => {
                    return Err(DslError::parse_error(
                        "Expected alias identifier after 'AS'",
                    ));
                }
            }
        } else {
            None
        };

        Ok(ast::ImportNode {
            path: path_part,
            alias,
            kind: crate::module_system::ImportKind::Simple,
        })
    }

    /// Parses a NAMESPACE declaration.
    fn parse_namespace<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<crate::module_system::NamespaceNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Expect NAMESPACE
        match iter.next() {
            Some(Token::Namespace) => {}
            _ => return Err(DslError::parse_error("Expected 'NAMESPACE' keyword")),
        }

        // Get namespace path (either Ident or String)
        let path = match iter.next() {
            Some(Token::Ident(s)) => s.clone(),
            Some(Token::StringLit(s)) => s.clone(),
            _ => {
                return Err(DslError::parse_error(
                    "Expected namespace path (identifier or string)",
                ));
            }
        };

        Ok(crate::module_system::NamespaceNode { path })
    }

    /// Parses an EXPORT declaration.
    fn parse_export<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<crate::module_system::ExportNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Expect EXPORT
        match iter.next() {
            Some(Token::Export) => {}
            _ => return Err(DslError::parse_error("Expected 'EXPORT' keyword")),
        }

        let mut items = Vec::new();

        // Check for wildcard export (EXPORT *)
        if matches!(iter.peek(), Some(Token::Star)) {
            iter.next(); // consume *
            items.push("*".to_string());
            return Ok(crate::module_system::ExportNode { items, from: None });
        }

        // Check for selective export (EXPORT { item1, item2 })
        if matches!(iter.peek(), Some(Token::LBrace)) {
            iter.next(); // consume {

            loop {
                match iter.next() {
                    Some(Token::Ident(id)) => items.push(id.clone()),
                    Some(Token::RBrace) => break,
                    Some(Token::Comma) => continue,
                    _ => {
                        return Err(DslError::parse_error(
                            "Expected identifier or '}' in export list",
                        ));
                    }
                }
            }
        } else {
            // Single item export
            match iter.next() {
                Some(Token::Ident(id)) => items.push(id.clone()),
                _ => return Err(DslError::parse_error("Expected identifier to export")),
            }
        }

        // Check for optional FROM clause (re-export)
        let from = if matches!(iter.peek(), Some(Token::From)) {
            iter.next(); // consume FROM
            match iter.next() {
                Some(Token::StringLit(s)) => Some(s.clone()),
                Some(Token::Ident(s)) => Some(s.clone()),
                _ => return Err(DslError::parse_error("Expected module path after 'FROM'")),
            }
        } else {
            None
        };

        Ok(crate::module_system::ExportNode { items, from })
    }

    /// Parses tokens into an AST StatuteNode.
    /// Supports optional visibility modifier: PUBLIC STATUTE ... or PRIVATE STATUTE ...
    fn parse_statute_node(&self, tokens: &[Token]) -> DslResult<ast::StatuteNode> {
        let mut iter = tokens.iter().peekable();

        // Check for optional visibility modifier
        let visibility = match iter.peek() {
            Some(Token::Public) => {
                iter.next(); // consume PUBLIC
                crate::module_system::Visibility::Public
            }
            Some(Token::Private) => {
                iter.next(); // consume PRIVATE
                crate::module_system::Visibility::Private
            }
            _ => crate::module_system::Visibility::Private, // Default to private
        };

        // Expect STATUTE
        match iter.next() {
            Some(Token::Statute) => {}
            _ => return Err(DslError::parse_error("Expected 'STATUTE' keyword")),
        }

        // Get ID
        let id = match iter.next() {
            Some(Token::Ident(s)) => s.clone(),
            _ => return Err(DslError::parse_error("Expected statute identifier")),
        };

        // Expect colon
        match iter.next() {
            Some(Token::Colon) => {}
            _ => return Err(DslError::parse_error("Expected ':'")),
        }

        // Get title
        let title = match iter.next() {
            Some(Token::StringLit(s)) => s.clone(),
            Some(Token::Ident(s)) => s.clone(),
            _ => return Err(DslError::parse_error("Expected statute title")),
        };

        // Expect LBrace
        match iter.next() {
            Some(Token::LBrace) => {}
            _ => return Err(DslError::parse_error("Expected '{'")),
        }

        let mut conditions = Vec::new();
        let mut effects = Vec::new();
        let mut discretion = None;
        let mut exceptions = Vec::new();
        let mut amendments = Vec::new();
        let mut supersedes = Vec::new();
        let mut defaults = Vec::new();
        let mut requires = Vec::new();
        let mut delegates = Vec::new();
        let mut scope = None;
        let mut constraints = Vec::new();
        let mut priority = None;

        // Parse body
        while let Some(token) = iter.next() {
            match token {
                Token::When => {
                    if let Some(cond) = self.parse_condition_node(&mut iter)? {
                        conditions.push(cond);
                    }
                }
                Token::Unless => {
                    // UNLESS is equivalent to WHEN NOT
                    if let Some(cond) = self.parse_condition_node(&mut iter)? {
                        conditions.push(ast::ConditionNode::Not(Box::new(cond)));
                    }
                }
                Token::Requires => {
                    // Parse comma-separated list of statute IDs that are required
                    loop {
                        match iter.next() {
                            Some(Token::Ident(id)) => requires.push(id.clone()),
                            Some(Token::StringLit(id)) => requires.push(id.clone()),
                            Some(Token::Comma) => continue,
                            _ => break,
                        }
                    }
                }
                Token::Then => {
                    let effect = self.parse_effect_node(&mut iter)?;
                    effects.push(effect);
                }
                Token::Discretion => {
                    if let Some(Token::StringLit(s)) = iter.next() {
                        discretion = Some(s.clone());
                    }
                }
                Token::Exception => {
                    let exception = self.parse_exception_node(&mut iter)?;
                    exceptions.push(exception);
                }
                Token::Amendment => {
                    let amendment = self.parse_amendment_node(&mut iter)?;
                    amendments.push(amendment);
                }
                Token::Supersedes => {
                    // Parse comma-separated list of statute IDs
                    loop {
                        match iter.next() {
                            Some(Token::Ident(id)) => supersedes.push(id.clone()),
                            Some(Token::StringLit(id)) => supersedes.push(id.clone()),
                            Some(Token::Comma) => continue,
                            _ => break,
                        }
                    }
                }
                Token::Default => {
                    let default = self.parse_default_node(&mut iter)?;
                    defaults.push(default);
                }
                Token::Priority => {
                    // Parse priority number
                    match iter.next() {
                        Some(Token::Number(n)) => priority = Some(*n as u32),
                        _ => return Err(DslError::parse_error("Expected number after PRIORITY")),
                    }
                }
                Token::Delegate => {
                    let delegate = self.parse_delegate_node(&mut iter)?;
                    delegates.push(delegate);
                }
                Token::Scope => {
                    scope = Some(self.parse_scope_node(&mut iter)?);
                }
                Token::Constraint => {
                    let constraint = self.parse_constraint_node(&mut iter)?;
                    constraints.push(constraint);
                }
                Token::RBrace => break,
                _ => {}
            }
        }

        Ok(ast::StatuteNode {
            id,
            title,
            visibility,
            conditions,
            effects,
            discretion,
            exceptions,
            amendments,
            supersedes,
            defaults,
            requires,
            delegates,
            scope,
            constraints,
            priority,
        })
    }

    /// Parses a condition into an AST ConditionNode.
    fn parse_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        self.parse_or_condition_node(iter)
    }

    fn parse_or_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let left = self.parse_and_condition_node(iter)?;
        if left.is_none() {
            return Ok(None);
        }
        let mut result = left.unwrap();

        while matches!(iter.peek(), Some(Token::Or)) {
            iter.next();
            let right = self.parse_and_condition_node(iter)?;
            if let Some(right_cond) = right {
                result = ast::ConditionNode::Or(Box::new(result), Box::new(right_cond));
            }
        }

        Ok(Some(result))
    }

    fn parse_and_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let left = self.parse_unary_condition_node(iter)?;
        if left.is_none() {
            return Ok(None);
        }
        let mut result = left.unwrap();

        while matches!(iter.peek(), Some(Token::And)) {
            iter.next();
            let right = self.parse_unary_condition_node(iter)?;
            if let Some(right_cond) = right {
                result = ast::ConditionNode::And(Box::new(result), Box::new(right_cond));
            }
        }

        Ok(Some(result))
    }

    fn parse_unary_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek() {
            Some(Token::Not) => {
                iter.next();
                let inner = self.parse_unary_condition_node(iter)?;
                Ok(inner.map(|c| ast::ConditionNode::Not(Box::new(c))))
            }
            Some(Token::LParen) => {
                iter.next();
                let inner = self.parse_or_condition_node(iter)?;
                match iter.peek() {
                    Some(Token::RParen) => {
                        iter.next();
                    }
                    _ => return Err(DslError::UnmatchedParen(None)),
                }
                Ok(inner)
            }
            _ => self.parse_primary_condition_node(iter),
        }
    }

    fn parse_primary_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek().cloned() {
            Some(Token::Age) => {
                iter.next();
                self.parse_field_condition(iter, "age")
            }
            Some(Token::Income) => {
                iter.next();
                self.parse_field_condition(iter, "income")
            }
            Some(Token::CurrentDate) => {
                iter.next();
                self.parse_temporal_condition(iter, ast::TemporalField::CurrentDate)
            }
            Some(Token::DateField) => {
                iter.next();
                // Expect field name
                let field_name = match iter.next() {
                    Some(Token::Ident(s)) => s.clone(),
                    Some(Token::StringLit(s)) => s.clone(),
                    _ => {
                        return Err(DslError::InvalidCondition(
                            "Expected field name after DATE_FIELD".to_string(),
                        ));
                    }
                };
                self.parse_temporal_condition(iter, ast::TemporalField::DateField(field_name))
            }
            Some(Token::Has) => {
                iter.next();
                if let Some(Token::Ident(key)) = iter.peek() {
                    let key = key.clone();
                    iter.next();
                    Ok(Some(ast::ConditionNode::HasAttribute { key }))
                } else if let Some(Token::StringLit(key)) = iter.peek() {
                    let key = key.clone();
                    iter.next();
                    Ok(Some(ast::ConditionNode::HasAttribute { key }))
                } else {
                    Err(DslError::InvalidCondition(
                        "Expected attribute key after HAS".to_string(),
                    ))
                }
            }
            Some(Token::Ident(name)) => {
                let name = name.clone();
                iter.next();
                // Check for qualified reference (alias.statute_id)
                if matches!(iter.peek(), Some(Token::Dot)) {
                    iter.next(); // consume dot
                    if let Some(Token::Ident(member)) = iter.next() {
                        // This is a qualified reference like "other.adult_rights"
                        Ok(Some(ast::ConditionNode::HasAttribute {
                            key: format!("{}.{}", name, member),
                        }))
                    } else {
                        Err(DslError::parse_error("Expected identifier after '.'"))
                    }
                } else if matches!(
                    iter.peek(),
                    Some(Token::Operator(_))
                        | Some(Token::Between)
                        | Some(Token::In)
                        | Some(Token::Like)
                        | Some(Token::Matches)
                        | Some(Token::InRange)
                        | Some(Token::NotInRange)
                ) {
                    // This is a field condition (e.g., "email MATCHES pattern")
                    self.parse_field_condition(iter, &name)
                } else {
                    Ok(Some(ast::ConditionNode::HasAttribute { key: name }))
                }
            }
            Some(Token::Then) | Some(Token::RBrace) | Some(Token::Discretion) => Ok(None),
            _ => Ok(None),
        }
    }

    /// Parses temporal field conditions (date comparisons).
    fn parse_temporal_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
        field: ast::TemporalField,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let op = self.parse_comparison_op(iter)?;
        let value = self.parse_condition_value(iter)?;
        Ok(Some(ast::ConditionNode::TemporalComparison {
            field,
            operator: op.to_string(),
            value,
        }))
    }

    /// Parses numeric range conditions with inclusive/exclusive bounds.
    /// Syntax: IN_RANGE min..max or IN_RANGE (min..max) or IN_RANGE [min..max]
    fn parse_range_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
        field: &str,
        negated: bool,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Check for opening bracket/paren to determine inclusivity
        let mut inclusive_min = true;
        let mut inclusive_max = true;

        // Look for optional opening bracket
        if matches!(iter.peek(), Some(Token::LParen)) {
            iter.next();
            inclusive_min = false;
        }

        // Parse min value
        let min = self.parse_condition_value(iter)?;

        // Expect .. or ...
        match iter.peek() {
            Some(Token::Dot) => {
                iter.next(); // first dot
                if matches!(iter.peek(), Some(Token::Dot)) {
                    iter.next(); // second dot
                    // Check for third dot (exclusive max)
                    if matches!(iter.peek(), Some(Token::Dot)) {
                        iter.next();
                        inclusive_max = false;
                    }
                } else {
                    return Err(DslError::InvalidCondition(
                        "Expected '..' or '...' in range".to_string(),
                    ));
                }
            }
            _ => {
                return Err(DslError::InvalidCondition(
                    "Expected '..' in range expression".to_string(),
                ));
            }
        }

        // Parse max value
        let max = self.parse_condition_value(iter)?;

        // Look for closing bracket/paren
        if matches!(iter.peek(), Some(Token::RParen)) {
            iter.next();
            if !inclusive_min {
                inclusive_max = false; // (min..max) - both exclusive
            }
        }

        if negated {
            Ok(Some(ast::ConditionNode::NotInRange {
                field: field.to_string(),
                min,
                max,
                inclusive_min,
                inclusive_max,
            }))
        } else {
            Ok(Some(ast::ConditionNode::InRange {
                field: field.to_string(),
                min,
                max,
                inclusive_min,
                inclusive_max,
            }))
        }
    }

    /// Parses field conditions including BETWEEN, IN, LIKE, and comparison operators.
    fn parse_field_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
        field: &str,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek() {
            Some(Token::Between) => {
                iter.next(); // consume BETWEEN
                let min = self.parse_condition_value(iter)?;
                // Expect AND
                if !matches!(iter.next(), Some(Token::And)) {
                    return Err(DslError::InvalidCondition(
                        "Expected AND in BETWEEN expression".to_string(),
                    ));
                }
                let max = self.parse_condition_value(iter)?;
                Ok(Some(ast::ConditionNode::Between {
                    field: field.to_string(),
                    min,
                    max,
                }))
            }
            Some(Token::In) => {
                iter.next(); // consume IN
                // Expect opening paren or bracket
                let _has_paren = if matches!(iter.peek(), Some(Token::LParen)) {
                    iter.next();
                    true
                } else {
                    false
                };

                let mut values = Vec::new();
                loop {
                    if matches!(iter.peek(), Some(Token::RParen) | Some(Token::Comma)) {
                        if matches!(iter.peek(), Some(Token::RParen)) {
                            iter.next(); // consume closing paren
                            break;
                        }
                        if matches!(iter.peek(), Some(Token::Comma)) {
                            iter.next(); // consume comma
                            continue;
                        }
                    }

                    if matches!(
                        iter.peek(),
                        Some(Token::Then)
                            | Some(Token::And)
                            | Some(Token::Or)
                            | Some(Token::RBrace)
                    ) {
                        break;
                    }

                    let value = self.parse_condition_value(iter)?;
                    values.push(value);

                    if matches!(iter.peek(), Some(Token::Comma)) {
                        iter.next(); // consume comma
                    } else if matches!(iter.peek(), Some(Token::RParen)) {
                        iter.next(); // consume closing paren
                        break;
                    } else {
                        break;
                    }
                }

                Ok(Some(ast::ConditionNode::In {
                    field: field.to_string(),
                    values,
                }))
            }
            Some(Token::Like) => {
                iter.next(); // consume LIKE
                let pattern = match iter.next() {
                    Some(Token::StringLit(s)) => s.clone(),
                    Some(Token::Ident(s)) => s.clone(),
                    _ => {
                        return Err(DslError::InvalidCondition(
                            "Expected pattern after LIKE".to_string(),
                        ));
                    }
                };
                Ok(Some(ast::ConditionNode::Like {
                    field: field.to_string(),
                    pattern,
                }))
            }
            Some(Token::Matches) => {
                iter.next(); // consume MATCHES
                let regex_pattern = match iter.next() {
                    Some(Token::StringLit(s)) => s.clone(),
                    Some(Token::Ident(s)) => s.clone(),
                    _ => {
                        return Err(DslError::InvalidCondition(
                            "Expected regex pattern after MATCHES".to_string(),
                        ));
                    }
                };
                // Validate regex pattern
                if let Err(e) = regex::Regex::new(&regex_pattern) {
                    return Err(DslError::InvalidCondition(format!(
                        "Invalid regex pattern: {}",
                        e
                    )));
                }
                Ok(Some(ast::ConditionNode::Matches {
                    field: field.to_string(),
                    regex_pattern,
                }))
            }
            Some(Token::InRange) => {
                iter.next(); // consume IN_RANGE
                self.parse_range_condition(iter, field, false)
            }
            Some(Token::NotInRange) => {
                iter.next(); // consume NOT_IN_RANGE
                self.parse_range_condition(iter, field, true)
            }
            Some(Token::Operator(_)) => {
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_condition_value(iter)?;
                Ok(Some(ast::ConditionNode::Comparison {
                    field: field.to_string(),
                    operator: op.to_string(),
                    value,
                }))
            }
            _ => Err(DslError::InvalidCondition(format!(
                "Expected comparison operator, BETWEEN, IN, or LIKE after {}",
                field
            ))),
        }
    }

    /// Parses a condition value (number, string, date, or boolean).
    fn parse_condition_value<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::ConditionValue>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek() {
            Some(Token::Number(n)) => {
                let val = ast::ConditionValue::Number(*n as i64);
                iter.next();
                Ok(val)
            }
            Some(Token::StringLit(s)) => {
                let s = s.clone();
                iter.next();
                // Check if it looks like a date (YYYY-MM-DD)
                if s.contains('-')
                    && s.split('-').count() == 3
                    && s.split('-').all(|part| part.parse::<u32>().is_ok())
                {
                    Ok(ast::ConditionValue::Date(s))
                } else {
                    Ok(ast::ConditionValue::String(s))
                }
            }
            Some(Token::Ident(s)) => {
                let s_upper = s.to_uppercase();
                let val = if s_upper == "TRUE" {
                    ast::ConditionValue::Boolean(true)
                } else if s_upper == "FALSE" {
                    ast::ConditionValue::Boolean(false)
                } else {
                    ast::ConditionValue::String(s.clone())
                };
                iter.next();
                Ok(val)
            }
            _ => Err(DslError::InvalidCondition(
                "Expected value (number, string, date, or boolean)".to_string(),
            )),
        }
    }

    /// Parses a set expression for set operations.
    /// Supports UNION, INTERSECT, and DIFFERENCE operations.
    /// Example: (1, 2, 3) UNION (4, 5, 6)
    #[allow(dead_code)]
    fn parse_set_expression<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::SetExpression>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Parse the initial set (values in parentheses or a single value list)
        let left = self.parse_simple_set(iter)?;

        // Check for set operations
        match iter.peek() {
            Some(Token::Union) => {
                iter.next(); // consume UNION
                let right = self.parse_set_expression(iter)?;
                Ok(ast::SetExpression::Union(Box::new(left), Box::new(right)))
            }
            Some(Token::Intersect) => {
                iter.next(); // consume INTERSECT
                let right = self.parse_set_expression(iter)?;
                Ok(ast::SetExpression::Intersect(
                    Box::new(left),
                    Box::new(right),
                ))
            }
            Some(Token::Difference) => {
                iter.next(); // consume DIFFERENCE
                let right = self.parse_set_expression(iter)?;
                Ok(ast::SetExpression::Difference(
                    Box::new(left),
                    Box::new(right),
                ))
            }
            _ => Ok(left),
        }
    }

    /// Parses a simple set of values (without operations).
    #[allow(dead_code)]
    fn parse_simple_set<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::SetExpression>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut values = Vec::new();

        // Expect opening paren (optional if already consumed)
        if matches!(iter.peek(), Some(Token::LParen)) {
            iter.next(); // consume opening paren
        }

        // Parse values until we hit a closing paren or set operator
        loop {
            if matches!(
                iter.peek(),
                Some(Token::RParen)
                    | Some(Token::Union)
                    | Some(Token::Intersect)
                    | Some(Token::Difference)
            ) {
                break;
            }

            // Skip commas
            if matches!(iter.peek(), Some(Token::Comma)) {
                iter.next();
                continue;
            }

            // Stop at logical operators or statement terminators
            if matches!(
                iter.peek(),
                Some(Token::And) | Some(Token::Or) | Some(Token::Then) | Some(Token::RBrace)
            ) {
                break;
            }

            let value = self.parse_condition_value(iter)?;
            values.push(value);
        }

        // Consume closing paren if present
        if matches!(iter.peek(), Some(Token::RParen)) {
            iter.next();
        }

        Ok(ast::SetExpression::Values(values))
    }

    /// Parses an effect into an AST EffectNode.
    fn parse_effect_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::EffectNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        let effect_type = match iter.next() {
            Some(Token::Grant) => "grant".to_string(),
            Some(Token::Revoke) => "revoke".to_string(),
            Some(Token::Obligation) => "obligation".to_string(),
            Some(Token::Prohibition) => "prohibition".to_string(),
            Some(Token::Ident(s)) => s.clone(),
            _ => return Err(DslError::InvalidEffect("Expected effect type".to_string())),
        };

        let description = match iter.next() {
            Some(Token::StringLit(s)) => s.clone(),
            Some(Token::Ident(s)) => s.clone(),
            _ => String::new(),
        };

        Ok(ast::EffectNode {
            effect_type,
            description,
            parameters: Vec::new(),
        })
    }

    /// Parses an exception clause.
    fn parse_exception_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::ExceptionNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Parse optional conditions
        let mut conditions = Vec::new();
        if matches!(iter.peek(), Some(Token::When)) {
            iter.next(); // consume WHEN
            if let Some(cond) = self.parse_condition_node(iter)? {
                conditions.push(cond);
            }
        }

        // Get description
        let description = match iter.next() {
            Some(Token::StringLit(s)) => s.clone(),
            Some(Token::Ident(s)) => s.clone(),
            _ => String::new(),
        };

        Ok(ast::ExceptionNode {
            conditions,
            description,
        })
    }

    /// Parses a default value declaration.
    fn parse_default_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::DefaultNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Get field name
        let field = match iter.next() {
            Some(Token::Ident(f)) => f.clone(),
            Some(Token::StringLit(f)) => f.clone(),
            _ => return Err(DslError::parse_error("Expected field name after DEFAULT")),
        };

        // Expect = or :
        match iter.peek() {
            Some(Token::Operator(op)) if op == "=" => {
                iter.next();
            }
            Some(Token::Colon) => {
                iter.next();
            }
            _ => {}
        }

        // Get value
        let value = match iter.peek() {
            Some(Token::Number(n)) => {
                let val = ast::ConditionValue::Number(*n as i64);
                iter.next();
                val
            }
            Some(Token::StringLit(s)) => {
                let val = ast::ConditionValue::String(s.clone());
                iter.next();
                val
            }
            Some(Token::Ident(s)) => {
                let s_upper = s.to_uppercase();
                let val = if s_upper == "TRUE" {
                    ast::ConditionValue::Boolean(true)
                } else if s_upper == "FALSE" {
                    ast::ConditionValue::Boolean(false)
                } else {
                    ast::ConditionValue::String(s.clone())
                };
                iter.next();
                val
            }
            _ => return Err(DslError::parse_error("Expected default value")),
        };

        Ok(ast::DefaultNode { field, value })
    }

    /// Parses a delegate clause.
    fn parse_delegate_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::DelegateNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Get target statute ID
        let target_id = match iter.next() {
            Some(Token::Ident(id)) => id.clone(),
            Some(Token::StringLit(id)) => id.clone(),
            _ => return Err(DslError::parse_error("Expected statute ID after DELEGATE")),
        };

        // Parse optional conditions
        let mut conditions = Vec::new();
        if matches!(iter.peek(), Some(Token::When)) {
            iter.next(); // consume WHEN
            if let Some(cond) = self.parse_condition_node(iter)? {
                conditions.push(cond);
            }
        }

        // Get description
        let description = match iter.next() {
            Some(Token::StringLit(s)) => s.clone(),
            Some(Token::Ident(s)) => s.clone(),
            _ => String::new(),
        };

        Ok(ast::DelegateNode {
            target_id,
            conditions,
            description,
        })
    }

    /// Parses a scope clause.
    fn parse_scope_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::ScopeNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Parse entity types (comma-separated list)
        let mut entity_types = Vec::new();
        loop {
            match iter.peek() {
                Some(Token::Ident(id)) => {
                    entity_types.push(id.clone());
                    iter.next();
                }
                Some(Token::StringLit(s)) => {
                    entity_types.push(s.clone());
                    iter.next();
                }
                Some(Token::Comma) => {
                    iter.next();
                    continue;
                }
                _ => break,
            }
        }

        // Parse optional conditions
        let mut conditions = Vec::new();
        if matches!(iter.peek(), Some(Token::When)) {
            iter.next(); // consume WHEN
            if let Some(cond) = self.parse_condition_node(iter)? {
                conditions.push(cond);
            }
        }

        // Get optional description
        let description = match iter.peek() {
            Some(Token::StringLit(s)) => {
                let desc = s.clone();
                iter.next();
                Some(desc)
            }
            _ => None,
        };

        Ok(ast::ScopeNode {
            entity_types,
            conditions,
            description,
        })
    }

    /// Parses a constraint clause.
    fn parse_constraint_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::ConstraintNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Get constraint name
        let name = match iter.next() {
            Some(Token::Ident(n)) => n.clone(),
            Some(Token::StringLit(n)) => n.clone(),
            _ => {
                return Err(DslError::parse_error(
                    "Expected constraint name after CONSTRAINT",
                ));
            }
        };

        // Expect colon
        if !matches!(iter.peek(), Some(Token::Colon)) {
            return Err(DslError::parse_error("Expected ':' after constraint name"));
        }
        iter.next();

        // Parse condition
        let condition = match self.parse_condition_node(iter)? {
            Some(cond) => cond,
            None => return Err(DslError::parse_error("Expected condition for constraint")),
        };

        // Get optional description
        let description = match iter.peek() {
            Some(Token::StringLit(s)) => {
                let desc = s.clone();
                iter.next();
                Some(desc)
            }
            _ => None,
        };

        Ok(ast::ConstraintNode {
            name,
            condition,
            description,
        })
    }

    /// Parses an amendment clause.
    fn parse_amendment_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::AmendmentNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Get target statute ID
        let target_id = match iter.next() {
            Some(Token::Ident(id)) => id.clone(),
            Some(Token::StringLit(id)) => id.clone(),
            _ => return Err(DslError::parse_error("Expected statute ID after AMENDMENT")),
        };

        let mut version = None;
        let mut date = None;
        let mut description = String::new();

        // Parse optional metadata and description
        loop {
            match iter.peek() {
                Some(Token::Version) => {
                    iter.next();
                    if let Some(Token::Number(v)) = iter.next() {
                        version = Some(*v as u32);
                    }
                }
                Some(Token::EffectiveDate) => {
                    iter.next();
                    // Parse date (could be YYYY-MM-DD or string)
                    let mut date_parts = Vec::new();
                    let mut found_string = false;
                    while let Some(token) = iter.peek() {
                        match token {
                            Token::Number(n) => {
                                date_parts.push(n.to_string());
                                iter.next();
                            }
                            Token::Dash => {
                                date_parts.push("-".to_string());
                                iter.next();
                            }
                            Token::StringLit(_) => {
                                // This might be the description, not the date
                                if date_parts.is_empty() {
                                    // No date parts yet, treat as string date
                                    if let Some(Token::StringLit(s)) = iter.next() {
                                        date = Some(s.clone());
                                        found_string = true;
                                    }
                                }
                                break;
                            }
                            _ => break,
                        }
                    }
                    if !found_string && !date_parts.is_empty() {
                        date = Some(date_parts.join(""));
                    }
                }
                Some(Token::StringLit(_)) => {
                    if let Some(Token::StringLit(s)) = iter.next() {
                        description = s.clone();
                    }
                    break;
                }
                _ => break,
            }
        }

        Ok(ast::AmendmentNode {
            target_id,
            version,
            date,
            description,
        })
    }

    /// Removes comments from input (both // and /* */).
    fn strip_comments(&self, input: &str) -> DslResult<String> {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        let mut position = 0;

        while let Some(ch) = chars.next() {
            position += 1;
            if ch == '/' {
                if let Some(&next) = chars.peek() {
                    if next == '/' {
                        // Line comment: skip until newline
                        chars.next();
                        while let Some(&c) = chars.peek() {
                            if c == '\n' {
                                break;
                            }
                            chars.next();
                        }
                        result.push('\n');
                        continue;
                    } else if next == '*' {
                        // Block comment: skip until */
                        chars.next();
                        let comment_start = position;
                        let mut found_end = false;
                        while let Some(c) = chars.next() {
                            if c == '*' {
                                if let Some(&next_c) = chars.peek() {
                                    if next_c == '/' {
                                        chars.next();
                                        found_end = true;
                                        break;
                                    }
                                }
                            }
                        }
                        if !found_end {
                            return Err(DslError::UnclosedComment(Some(
                                SourceLocation::from_offset(comment_start, input),
                            )));
                        }
                        result.push(' ');
                        continue;
                    }
                }
            }
            result.push(ch);
        }

        Ok(result)
    }

    pub fn tokenize(&self, input: &str) -> DslResult<Vec<SpannedToken>> {
        let stripped = self.strip_comments(input)?;
        // Pre-allocate capacity: estimate ~10 tokens per 100 bytes
        let estimated_tokens = (stripped.len() / 10).max(16);
        let mut tokens = Vec::with_capacity(estimated_tokens);
        let mut chars = stripped.chars().peekable();
        let mut offset = 0;
        let mut line = 1;
        let mut column = 1;

        while let Some(&ch) = chars.peek() {
            let token_start = SourceLocation::new(line, column, offset);
            match ch {
                '\n' => {
                    chars.next();
                    offset += 1;
                    line += 1;
                    column = 1;
                }
                // Optimize: skip multiple whitespace characters at once
                ' ' | '\t' | '\r' => {
                    chars.next();
                    offset += 1;
                    column += 1;
                    // Fast-path: skip additional whitespace
                    while let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            ' ' | '\t' | '\r' => {
                                chars.next();
                                offset += 1;
                                column += 1;
                            }
                            _ => break,
                        }
                    }
                }
                '(' => {
                    tokens.push(SpannedToken::new(Token::LParen, token_start));
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                ')' => {
                    tokens.push(SpannedToken::new(Token::RParen, token_start));
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                '{' => {
                    tokens.push(SpannedToken::new(Token::LBrace, token_start));
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                '}' => {
                    tokens.push(SpannedToken::new(Token::RBrace, token_start));
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                ':' => {
                    tokens.push(SpannedToken::new(Token::Colon, token_start));
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                ',' => {
                    tokens.push(SpannedToken::new(Token::Comma, token_start));
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                '"' => {
                    chars.next();
                    offset += 1;
                    column += 1;
                    // Pre-allocate for typical string length
                    let mut s = String::with_capacity(32);
                    while let Some(&c) = chars.peek() {
                        if c == '"' {
                            chars.next();
                            offset += 1;
                            column += 1;
                            break;
                        }
                        if c == '\n' {
                            line += 1;
                            column = 1;
                        } else {
                            column += 1;
                        }
                        s.push(c);
                        chars.next();
                        offset += 1;
                    }
                    tokens.push(SpannedToken::new(Token::StringLit(s), token_start));
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    // Pre-allocate for typical keyword/identifier length
                    let mut word = String::with_capacity(16);
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' || c == '-' {
                            word.push(c);
                            chars.next();
                            offset += 1;
                            column += 1;
                        } else {
                            break;
                        }
                    }
                    let upper = word.to_uppercase();

                    // Check for deprecated syntax and emit warnings
                    match upper.as_str() {
                        "EXCEPT" => {
                            self.emit_warning(DslWarning::DeprecatedSyntax {
                                location: token_start,
                                old_syntax: "EXCEPT".to_string(),
                                new_syntax: "EXCEPTION".to_string(),
                                message: "Please use 'EXCEPTION' instead of 'EXCEPT'".to_string(),
                            });
                        }
                        "AMENDS" => {
                            self.emit_warning(DslWarning::DeprecatedSyntax {
                                location: token_start,
                                old_syntax: "AMENDS".to_string(),
                                new_syntax: "AMENDMENT".to_string(),
                                message: "Please use 'AMENDMENT' instead of 'AMENDS'".to_string(),
                            });
                        }
                        "REPLACES" => {
                            self.emit_warning(DslWarning::DeprecatedSyntax {
                                location: token_start,
                                old_syntax: "REPLACES".to_string(),
                                new_syntax: "SUPERSEDES".to_string(),
                                message: "Please use 'SUPERSEDES' instead of 'REPLACES'"
                                    .to_string(),
                            });
                        }
                        _ => {}
                    }

                    let token = match upper.as_str() {
                        "STATUTE" => Token::Statute,
                        "WHEN" => Token::When,
                        "UNLESS" => Token::Unless,
                        "REQUIRES" => Token::Requires,
                        "THEN" => Token::Then,
                        "DISCRETION" => Token::Discretion,
                        "AGE" => Token::Age,
                        "INCOME" => Token::Income,
                        "GRANT" => Token::Grant,
                        "REVOKE" => Token::Revoke,
                        "OBLIGATION" => Token::Obligation,
                        "PROHIBITION" => Token::Prohibition,
                        "IMPORT" => Token::Import,
                        "AS" => Token::As,
                        "EXCEPTION" | "EXCEPT" => Token::Exception,
                        "AMENDMENT" | "AMENDS" => Token::Amendment,
                        "SUPERSEDES" | "REPLACES" => Token::Supersedes,
                        "DELEGATE" | "DELEGATES" => Token::Delegate,
                        "PRIORITY" => Token::Priority,
                        "SCOPE" => Token::Scope,
                        "CONSTRAINT" | "CONSTRAINTS" | "INVARIANT" => Token::Constraint,
                        // Module system keywords (v0.1.4)
                        "NAMESPACE" => Token::Namespace,
                        "FROM" => Token::From,
                        "PUBLIC" => Token::Public,
                        "PRIVATE" => Token::Private,
                        "EXPORT" => Token::Export,
                        "AND" => Token::And,
                        "OR" => Token::Or,
                        "NOT" => Token::Not,
                        "HAS" => Token::Has,
                        "BETWEEN" => Token::Between,
                        "IN" => Token::In,
                        "LIKE" => Token::Like,
                        "MATCHES" | "MATCH" | "REGEX" => Token::Matches,
                        "IN_RANGE" | "INRANGE" => Token::InRange,
                        "NOT_IN_RANGE" | "NOTINRANGE" => Token::NotInRange,
                        "DEFAULT" => Token::Default,
                        "UNION" => Token::Union,
                        "INTERSECT" | "INTERSECTION" => Token::Intersect,
                        "DIFFERENCE" | "SETMINUS" => Token::Difference,
                        "EFFECTIVE_DATE" | "EFFECTIVE" => Token::EffectiveDate,
                        "EXPIRY_DATE" | "EXPIRY" | "EXPIRES" => Token::ExpiryDate,
                        "JURISDICTION" => Token::Jurisdiction,
                        "VERSION" => Token::Version,
                        "CURRENT_DATE" | "CURRENTDATE" | "NOW" | "TODAY" => Token::CurrentDate,
                        "DATE_FIELD" | "DATEFIELD" => Token::DateField,
                        _ => Token::Ident(word),
                    };
                    tokens.push(SpannedToken::new(token, token_start));
                }
                _ if ch.is_numeric() => {
                    // Pre-allocate for typical number length
                    let mut num = String::with_capacity(8);
                    while let Some(&c) = chars.peek() {
                        if c.is_numeric() {
                            num.push(c);
                            chars.next();
                            offset += 1;
                            column += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push(SpannedToken::new(
                        Token::Number(num.parse().unwrap_or(0)),
                        token_start,
                    ));
                }
                '-' => {
                    tokens.push(SpannedToken::new(Token::Dash, token_start));
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                '.' => {
                    tokens.push(SpannedToken::new(Token::Dot, token_start));
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                '>' | '<' | '=' | '!' => {
                    let mut op = String::new();
                    op.push(ch);
                    chars.next();
                    offset += 1;
                    column += 1;
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            op.push(next);
                            chars.next();
                            offset += 1;
                            column += 1;
                        }
                    }
                    tokens.push(SpannedToken::new(Token::Operator(op), token_start));
                }
                '*' => {
                    tokens.push(SpannedToken::new(Token::Star, token_start));
                    chars.next();
                    offset += 1;
                    column += 1;
                }
                _ => {
                    chars.next();
                    offset += 1;
                    column += 1;
                }
            }
        }

        Ok(tokens)
    }

    fn parse_tokens(&self, tokens: &[Token]) -> DslResult<Statute> {
        let mut iter = tokens.iter().peekable();

        // Expect STATUTE
        match iter.next() {
            Some(Token::Statute) => {}
            _ => {
                return Err(DslError::parse_error("Expected 'STATUTE' keyword"));
            }
        }

        // Get ID
        let id = match iter.next() {
            Some(Token::Ident(s)) => s.clone(),
            _ => {
                return Err(DslError::parse_error("Expected statute identifier"));
            }
        };

        // Expect colon
        match iter.next() {
            Some(Token::Colon) => {}
            _ => {
                return Err(DslError::parse_error("Expected ':'"));
            }
        }

        // Get title
        let title = match iter.next() {
            Some(Token::StringLit(s)) => s.clone(),
            Some(Token::Ident(s)) => s.clone(),
            _ => {
                return Err(DslError::parse_error("Expected statute title"));
            }
        };

        // Expect LBrace
        match iter.next() {
            Some(Token::LBrace) => {}
            _ => {
                return Err(DslError::parse_error("Expected '{'"));
            }
        }

        let mut conditions = Vec::new();
        let mut effect = None;
        let mut discretion = None;
        let mut effective_date = None;
        let mut expiry_date = None;
        let mut jurisdiction = None;
        let mut version = None;

        // Parse body
        while let Some(token) = iter.next() {
            match token {
                Token::When => {
                    if let Some(cond) = self.parse_condition(&mut iter)? {
                        conditions.push(cond);
                    }
                }
                Token::Then => {
                    effect = Some(self.parse_effect(&mut iter)?);
                }
                Token::Discretion => {
                    if let Some(Token::StringLit(s)) = iter.next() {
                        discretion = Some(s.clone());
                    }
                }
                Token::EffectiveDate => {
                    effective_date = self.parse_date(&mut iter);
                }
                Token::ExpiryDate => {
                    expiry_date = self.parse_date(&mut iter);
                }
                Token::Jurisdiction => {
                    if let Some(Token::StringLit(s)) = iter.next() {
                        jurisdiction = Some(s.clone());
                    } else if let Some(Token::Ident(s)) = iter.peek() {
                        jurisdiction = Some(s.clone());
                        iter.next();
                    }
                }
                Token::Version => {
                    if let Some(Token::Number(n)) = iter.next() {
                        version = Some(*n as u32);
                    }
                }
                Token::RBrace => break,
                _ => {}
            }
        }

        let effect =
            effect.unwrap_or_else(|| Effect::new(EffectType::Custom, "No effect specified"));

        let mut statute = Statute::new(id, title, effect);
        statute.preconditions = conditions;
        statute.discretion_logic = discretion;

        // Set temporal validity if any dates were specified
        if effective_date.is_some() || expiry_date.is_some() {
            statute.temporal_validity = TemporalValidity {
                effective_date,
                expiry_date,
                enacted_at: None,
                amended_at: None,
            };
        }

        // Set jurisdiction and version if specified
        if let Some(jur) = jurisdiction {
            statute.jurisdiction = Some(jur);
        }
        if let Some(ver) = version {
            statute.version = ver;
        }

        Ok(statute)
    }

    /// Parses a condition expression (handles OR at lowest precedence).
    fn parse_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        self.parse_or_condition(iter)
    }

    /// Parses OR expressions (lowest precedence).
    fn parse_or_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let left = self.parse_and_condition(iter)?;
        if left.is_none() {
            return Ok(None);
        }
        let mut result = left.unwrap();

        while matches!(iter.peek(), Some(Token::Or)) {
            iter.next(); // consume OR
            let right = self.parse_and_condition(iter)?;
            if let Some(right_cond) = right {
                result = Condition::Or(Box::new(result), Box::new(right_cond));
            }
        }

        Ok(Some(result))
    }

    /// Parses AND expressions (higher precedence than OR).
    fn parse_and_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let left = self.parse_unary_condition(iter)?;
        if left.is_none() {
            return Ok(None);
        }
        let mut result = left.unwrap();

        while matches!(iter.peek(), Some(Token::And)) {
            iter.next(); // consume AND
            let right = self.parse_unary_condition(iter)?;
            if let Some(right_cond) = right {
                result = Condition::And(Box::new(result), Box::new(right_cond));
            }
        }

        Ok(Some(result))
    }

    /// Parses unary expressions (NOT) and primary conditions.
    fn parse_unary_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek() {
            Some(Token::Not) => {
                iter.next(); // consume NOT
                let inner = self.parse_unary_condition(iter)?;
                Ok(inner.map(|c| Condition::Not(Box::new(c))))
            }
            Some(Token::LParen) => {
                iter.next(); // consume (
                let inner = self.parse_or_condition(iter)?;
                // Expect closing paren
                match iter.peek() {
                    Some(Token::RParen) => {
                        iter.next(); // consume )
                    }
                    _ => return Err(DslError::UnmatchedParen(None)),
                }
                Ok(inner)
            }
            _ => self.parse_primary_condition(iter),
        }
    }

    /// Parses primary (atomic) conditions.
    fn parse_primary_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek().cloned() {
            Some(Token::Age) => {
                iter.next();
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_number(iter)?;
                Ok(Some(Condition::Age {
                    operator: op,
                    value: value as u32,
                }))
            }
            Some(Token::Income) => {
                iter.next();
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_number(iter)?;
                Ok(Some(Condition::Income {
                    operator: op,
                    value,
                }))
            }
            Some(Token::Has) => {
                iter.next();
                // Expect an identifier after HAS
                if let Some(Token::Ident(key)) = iter.peek() {
                    let key = key.clone();
                    iter.next();
                    Ok(Some(Condition::HasAttribute { key }))
                } else if let Some(Token::StringLit(key)) = iter.peek() {
                    let key = key.clone();
                    iter.next();
                    Ok(Some(Condition::HasAttribute { key }))
                } else {
                    Ok(None)
                }
            }
            Some(Token::Ident(key)) => {
                iter.next();
                Ok(Some(Condition::HasAttribute { key: key.clone() }))
            }
            _ => Ok(None),
        }
    }

    fn parse_comparison_op<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<legalis_core::ComparisonOp>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::Operator(op)) => match op.as_str() {
                ">=" => Ok(legalis_core::ComparisonOp::GreaterOrEqual),
                "<=" => Ok(legalis_core::ComparisonOp::LessOrEqual),
                ">" => Ok(legalis_core::ComparisonOp::GreaterThan),
                "<" => Ok(legalis_core::ComparisonOp::LessThan),
                "==" | "=" => Ok(legalis_core::ComparisonOp::Equal),
                "!=" => Ok(legalis_core::ComparisonOp::NotEqual),
                _ => Err(DslError::InvalidCondition(format!(
                    "Unknown operator: {op}"
                ))),
            },
            _ => Err(DslError::InvalidCondition(
                "Expected comparison operator".to_string(),
            )),
        }
    }

    fn parse_number<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<u64>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::Number(n)) => Ok(*n),
            _ => Err(DslError::InvalidCondition("Expected number".to_string())),
        }
    }

    fn parse_effect<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<Effect>
    where
        I: Iterator<Item = &'a Token>,
    {
        let effect_type = match iter.next() {
            Some(Token::Grant) => EffectType::Grant,
            Some(Token::Revoke) => EffectType::Revoke,
            Some(Token::Obligation) => EffectType::Obligation,
            Some(Token::Prohibition) => EffectType::Prohibition,
            Some(Token::Ident(_)) => EffectType::Custom,
            _ => EffectType::Custom,
        };

        let description = match iter.peek() {
            Some(Token::StringLit(s)) => {
                let s = s.clone();
                iter.next();
                s
            }
            _ => "No description".to_string(),
        };

        Ok(Effect::new(effect_type, description))
    }

    /// Parses a date in YYYY-MM-DD format.
    fn parse_date<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> Option<NaiveDate>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Try to parse date as YYYY-MM-DD (Number-Dash-Number-Dash-Number)
        // or as a quoted string "YYYY-MM-DD"
        match iter.peek() {
            Some(Token::StringLit(s)) => {
                let date_str = s.clone();
                iter.next();
                NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()
            }
            Some(Token::Number(year)) => {
                let year = *year as i32;
                iter.next();

                // Expect dash
                if !matches!(iter.next(), Some(Token::Dash)) {
                    return None;
                }

                // Month
                let month = match iter.next() {
                    Some(Token::Number(m)) => *m as u32,
                    _ => return None,
                };

                // Expect dash
                if !matches!(iter.next(), Some(Token::Dash)) {
                    return None;
                }

                // Day
                let day = match iter.next() {
                    Some(Token::Number(d)) => *d as u32,
                    _ => return None,
                };

                NaiveDate::from_ymd_opt(year, month, day)
            }
            _ => None,
        }
    }
}
