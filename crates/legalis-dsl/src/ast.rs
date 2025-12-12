//! AST (Abstract Syntax Tree) definitions for the legal DSL.

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

    // Structural
    LBrace,
    RBrace,
    Colon,

    // Literals
    Ident(String),
    StringLit(String),
    Number(u64),
    Operator(String),
}

/// AST node for a complete legal document.
#[derive(Debug, Clone)]
pub struct LegalDocument {
    pub statutes: Vec<StatuteNode>,
}

/// AST node for a statute definition.
#[derive(Debug, Clone)]
pub struct StatuteNode {
    pub id: String,
    pub title: String,
    pub conditions: Vec<ConditionNode>,
    pub effects: Vec<EffectNode>,
    pub discretion: Option<String>,
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
