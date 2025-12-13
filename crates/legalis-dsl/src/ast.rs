//! AST (Abstract Syntax Tree) definitions for the legal DSL.

use crate::SourceLocation;

/// Token with source location information.
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    /// The token type and value
    pub token: Token,
    /// Source location
    pub location: SourceLocation,
}

impl SpannedToken {
    /// Creates a new spanned token.
    pub fn new(token: Token, location: SourceLocation) -> Self {
        Self { token, location }
    }
}

/// Token types for the legal DSL lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Statute,
    When,
    Then,
    Discretion,
    Age,
    Income,
    Grant,
    Revoke,
    Obligation,
    Prohibition,
    Import,
    As,
    Exception,
    Amendment,
    Supersedes,

    // Metadata keywords
    EffectiveDate,
    ExpiryDate,
    Jurisdiction,
    Version,
    Has,

    // Logical operators
    And,
    Or,
    Not,

    // Structural
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Dash,
    Dot,
    Comma,

    // Literals
    Ident(String),
    StringLit(String),
    Number(u64),
    Operator(String),
}

/// AST node for an import declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ImportNode {
    /// The path to the imported file.
    pub path: String,
    /// Optional alias for the import (AS clause).
    pub alias: Option<String>,
}

/// AST node for a complete legal document.
#[derive(Debug, Clone)]
pub struct LegalDocument {
    /// Import declarations at the top of the document.
    pub imports: Vec<ImportNode>,
    /// Statute definitions.
    pub statutes: Vec<StatuteNode>,
}

/// AST node for an exception clause.
#[derive(Debug, Clone)]
pub struct ExceptionNode {
    /// Conditions under which the exception applies
    pub conditions: Vec<ConditionNode>,
    /// Description of the exception
    pub description: String,
}

/// AST node for an amendment clause.
#[derive(Debug, Clone)]
pub struct AmendmentNode {
    /// ID of the statute being amended
    pub target_id: String,
    /// Version of the amendment
    pub version: Option<u32>,
    /// Date of the amendment
    pub date: Option<String>,
    /// Description of changes
    pub description: String,
}

/// AST node for a statute definition.
#[derive(Debug, Clone)]
pub struct StatuteNode {
    pub id: String,
    pub title: String,
    pub conditions: Vec<ConditionNode>,
    pub effects: Vec<EffectNode>,
    pub discretion: Option<String>,
    pub exceptions: Vec<ExceptionNode>,
    pub amendments: Vec<AmendmentNode>,
    pub supersedes: Vec<String>,
}

/// AST node for conditions.
#[derive(Debug, Clone)]
pub enum ConditionNode {
    Comparison {
        field: String,
        operator: String,
        value: ConditionValue,
    },
    HasAttribute {
        key: String,
    },
    And(Box<ConditionNode>, Box<ConditionNode>),
    Or(Box<ConditionNode>, Box<ConditionNode>),
    Not(Box<ConditionNode>),
}

/// Values that can appear in conditions.
#[derive(Debug, Clone)]
pub enum ConditionValue {
    Number(i64),
    String(String),
    Boolean(bool),
}

/// AST node for effects.
#[derive(Debug, Clone)]
pub struct EffectNode {
    pub effect_type: String,
    pub description: String,
    pub parameters: Vec<(String, String)>,
}
