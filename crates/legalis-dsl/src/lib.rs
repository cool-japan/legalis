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

use thiserror::Error;

// Re-export core types needed by tests and downstream users.
pub use legalis_core::{Condition, Effect, EffectType, Statute, TemporalValidity};

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

mod parser_impl;
pub use parser_impl::LegalDslParser;
mod tokenizer;
