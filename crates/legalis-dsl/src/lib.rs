//! Legalis-DSL: Domain Specific Language for legal document parsing.
//!
//! This crate provides parsing and AST representation for legal documents,
//! enabling structured representation of statutes and legal rules.
//!
//! ## Grammar
//!
//! ```text
//! STATUTE ::= "STATUTE" ID ":" TITLE "{" BODY "}"
//! BODY ::= (METADATA | WHEN | THEN | DISCRETION)*
//! METADATA ::= EFFECTIVE_DATE | EXPIRY_DATE | JURISDICTION | VERSION
//! EFFECTIVE_DATE ::= ("EFFECTIVE_DATE" | "EFFECTIVE") DATE
//! EXPIRY_DATE ::= ("EXPIRY_DATE" | "EXPIRY" | "EXPIRES") DATE
//! JURISDICTION ::= "JURISDICTION" (STRING | IDENT)
//! VERSION ::= "VERSION" NUMBER
//! DATE ::= YYYY "-" MM "-" DD | STRING
//! WHEN ::= "WHEN" CONDITION
//! CONDITION ::= OR_EXPR
//! OR_EXPR ::= AND_EXPR ("OR" AND_EXPR)*
//! AND_EXPR ::= UNARY_EXPR ("AND" UNARY_EXPR)*
//! UNARY_EXPR ::= "NOT" UNARY_EXPR | "(" CONDITION ")" | PRIMARY_COND
//! PRIMARY_COND ::= AGE_COND | INCOME_COND | "HAS" IDENT | IDENT
//! THEN ::= "THEN" EFFECT
//! DISCRETION ::= "DISCRETION" STRING
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
//!     WHEN AGE >= 18 AND HAS citizen
//!     THEN GRANT "Right to vote"
//!     DISCRETION "Consider residency requirements"
//! }
//! ```

use chrono::NaiveDate;
use legalis_core::{Condition, Effect, EffectType, Statute, TemporalValidity};
use thiserror::Error;

mod ast;
mod parser;
mod printer;

pub use ast::*;
pub use parser::*;
pub use printer::*;

/// Source location for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

/// Errors that can occur during DSL parsing.
#[derive(Debug, Error)]
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
}

/// Result type for DSL operations.
pub type DslResult<T> = Result<T, DslError>;

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
pub struct LegalDslParser;

impl LegalDslParser {
    /// Creates a new parser instance.
    pub fn new() -> Self {
        Self
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

        // Parse imports first
        let mut imports = Vec::new();
        while matches!(iter.peek(), Some(Token::Import)) {
            imports.push(self.parse_import(&mut iter)?);
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

        Ok(ast::LegalDocument { imports, statutes })
    }

    /// Parses an IMPORT statement.
    fn parse_import<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<ast::ImportNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Expect IMPORT
        match iter.next() {
            Some(Token::Import) => {}
            _ => return Err(DslError::parse_error("Expected 'IMPORT' keyword")),
        }

        // Get path
        let path = match iter.next() {
            Some(Token::StringLit(s)) => s.clone(),
            _ => return Err(DslError::parse_error("Expected import path string")),
        };

        // Check for optional AS clause
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

        Ok(ast::ImportNode { path, alias })
    }

    /// Parses tokens into an AST StatuteNode.
    fn parse_statute_node(&self, tokens: &[Token]) -> DslResult<ast::StatuteNode> {
        let mut iter = tokens.iter().peekable();

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

        // Parse body
        while let Some(token) = iter.next() {
            match token {
                Token::When => {
                    if let Some(cond) = self.parse_condition_node(&mut iter)? {
                        conditions.push(cond);
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
                Token::RBrace => break,
                _ => {}
            }
        }

        Ok(ast::StatuteNode {
            id,
            title,
            conditions,
            effects,
            discretion,
            exceptions,
            amendments,
            supersedes,
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
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_number(iter)?;
                Ok(Some(ast::ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: op.to_string(),
                    value: ast::ConditionValue::Number(value as i64),
                }))
            }
            Some(Token::Income) => {
                iter.next();
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_number(iter)?;
                Ok(Some(ast::ConditionNode::Comparison {
                    field: "income".to_string(),
                    operator: op.to_string(),
                    value: ast::ConditionValue::Number(value as i64),
                }))
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
                } else {
                    Ok(Some(ast::ConditionNode::HasAttribute { key: name }))
                }
            }
            Some(Token::Then) | Some(Token::RBrace) | Some(Token::Discretion) => Ok(None),
            _ => Ok(None),
        }
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

    fn tokenize(&self, input: &str) -> DslResult<Vec<SpannedToken>> {
        let stripped = self.strip_comments(input)?;
        let mut tokens = Vec::new();
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
                ' ' | '\t' | '\r' => {
                    chars.next();
                    offset += 1;
                    column += 1;
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
                    let mut s = String::new();
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
                    let mut word = String::new();
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
                    let token = match word.to_uppercase().as_str() {
                        "STATUTE" => Token::Statute,
                        "WHEN" => Token::When,
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
                        "AND" => Token::And,
                        "OR" => Token::Or,
                        "NOT" => Token::Not,
                        "HAS" => Token::Has,
                        "EFFECTIVE_DATE" | "EFFECTIVE" => Token::EffectiveDate,
                        "EXPIRY_DATE" | "EXPIRY" | "EXPIRES" => Token::ExpiryDate,
                        "JURISDICTION" => Token::Jurisdiction,
                        "VERSION" => Token::Version,
                        _ => Token::Ident(word),
                    };
                    tokens.push(SpannedToken::new(token, token_start));
                }
                _ if ch.is_numeric() => {
                    let mut num = String::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_simple_statute() {
        let input = r#"
            STATUTE adult-rights: "Adult Rights Act" {
                WHEN AGE >= 18
                THEN GRANT "Full legal capacity"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.id, "adult-rights");
        assert_eq!(statute.title, "Adult Rights Act");
        assert_eq!(statute.preconditions.len(), 1);
    }

    #[test]
    fn test_parse_statute_with_discretion() {
        let input = r#"
            STATUTE subsidy-1: "Housing Subsidy" {
                WHEN INCOME <= 5000000
                THEN GRANT "Housing subsidy eligibility"
                DISCRETION "Consider special family circumstances"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert!(statute.discretion_logic.is_some());
        assert_eq!(
            statute.discretion_logic.unwrap(),
            "Consider special family circumstances"
        );
    }

    #[test]
    fn test_parse_and_condition() {
        let input = r#"
            STATUTE combo: "Combined Requirements" {
                WHEN AGE >= 18 AND INCOME <= 5000000
                THEN GRANT "Eligibility"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.preconditions.len(), 1);
        assert!(matches!(statute.preconditions[0], Condition::And(_, _)));
    }

    #[test]
    fn test_parse_or_condition() {
        let input = r#"
            STATUTE either: "Either Requirement" {
                WHEN AGE >= 65 OR disabled
                THEN GRANT "Pension eligibility"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.preconditions.len(), 1);
        assert!(matches!(statute.preconditions[0], Condition::Or(_, _)));
    }

    #[test]
    fn test_parse_not_condition() {
        let input = r#"
            STATUTE exclude: "Exclusion" {
                WHEN NOT convicted
                THEN GRANT "Voting rights"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.preconditions.len(), 1);
        assert!(matches!(statute.preconditions[0], Condition::Not(_)));
    }

    #[test]
    fn test_parse_nested_conditions() {
        let input = r#"
            STATUTE complex: "Complex Requirements" {
                WHEN (AGE >= 18 AND INCOME <= 5000000) OR disabled
                THEN GRANT "Benefits eligibility"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.preconditions.len(), 1);
        // Should be OR at top level with AND inside left branch
        assert!(matches!(statute.preconditions[0], Condition::Or(_, _)));
    }

    #[test]
    fn test_parse_with_line_comments() {
        let input = r#"
            // This is a comment about the statute
            STATUTE adult-rights: "Adult Rights Act" {
                WHEN AGE >= 18  // Must be adult
                THEN GRANT "Full legal capacity"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.id, "adult-rights");
    }

    #[test]
    fn test_parse_with_block_comments() {
        let input = r#"
            /*
             * Multi-line comment explaining the statute
             * This grants rights to adults
             */
            STATUTE adult-rights: "Adult Rights Act" {
                WHEN AGE >= 18
                THEN GRANT "Full legal capacity"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.id, "adult-rights");
    }

    #[test]
    fn test_unclosed_comment_error() {
        let input = r#"
            /* Unclosed comment
            STATUTE test: "Test" {
                WHEN AGE >= 18
                THEN GRANT "Test"
            }
        "#;

        let parser = LegalDslParser::new();
        let result = parser.parse_statute(input);

        assert!(matches!(result, Err(DslError::UnclosedComment(_))));
    }

    #[test]
    fn test_unclosed_comment_error_location() {
        // The comment starts at offset where /* begins
        // "STATUTE test: \"Test\" {\n" = 22 bytes, then 12 spaces + "/*"
        // So offset = 22 + 12 = 34, which is line 2, column 13
        // But the offset we store is where we found the /*, which increments after consuming chars
        let input = "STATUTE test: \"Test\" {\n            /* unclosed\n}";

        let parser = LegalDslParser::new();
        let result = parser.parse_statute(input);

        match result {
            Err(DslError::UnclosedComment(Some(loc))) => {
                // Line 2 because we're after the first newline
                assert_eq!(loc.line, 2, "Expected line 2");
                // Column depends on exactly how offset is calculated
                assert!(
                    loc.column >= 13 && loc.column <= 15,
                    "Expected column around 13-15, got {}",
                    loc.column
                );
            }
            _ => panic!("Expected UnclosedComment error with location"),
        }
    }

    #[test]
    fn test_source_location_display() {
        let loc = SourceLocation::new(10, 5, 100);
        assert_eq!(format!("{}", loc), "10:5");
    }

    #[test]
    fn test_source_location_from_offset() {
        let input = "line1\nline2\nline3";
        // Offset 0 should be line 1, column 1
        let loc = SourceLocation::from_offset(0, input);
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 1);

        // Offset 6 should be line 2, column 1 (after the newline)
        let loc = SourceLocation::from_offset(6, input);
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 1);

        // Offset 8 should be line 2, column 3
        let loc = SourceLocation::from_offset(8, input);
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 3);
    }

    #[test]
    fn test_precedence_and_before_or() {
        // AND should bind tighter than OR
        // A OR B AND C should parse as A OR (B AND C)
        let input = r#"
            STATUTE prec: "Precedence Test" {
                WHEN AGE >= 65 OR AGE >= 18 AND employed
                THEN GRANT "Something"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.preconditions.len(), 1);
        // Top level should be OR
        match &statute.preconditions[0] {
            Condition::Or(_, right) => {
                // Right side should be AND
                assert!(matches!(right.as_ref(), Condition::And(_, _)));
            }
            _ => panic!("Expected OR at top level"),
        }
    }

    #[test]
    fn test_parse_effective_date() {
        let input = r#"
            STATUTE dated: "Dated Statute" {
                EFFECTIVE_DATE 2024-01-01
                WHEN AGE >= 18
                THEN GRANT "Rights"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert!(statute.temporal_validity.effective_date.is_some());
        let date = statute.temporal_validity.effective_date.unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_parse_expiry_date() {
        let input = r#"
            STATUTE sunset: "Sunset Clause" {
                EXPIRY_DATE 2025-12-31
                WHEN AGE >= 21
                THEN GRANT "Temporary right"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert!(statute.temporal_validity.expiry_date.is_some());
        let date = statute.temporal_validity.expiry_date.unwrap();
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 31);
    }

    #[test]
    fn test_parse_effective_and_expiry() {
        let input = r#"
            STATUTE temporal: "Temporal Statute" {
                EFFECTIVE 2024-06-01
                EXPIRES 2026-05-31
                WHEN AGE >= 18
                THEN GRANT "Time-limited right"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert!(statute.temporal_validity.effective_date.is_some());
        assert!(statute.temporal_validity.expiry_date.is_some());
    }

    #[test]
    fn test_parse_jurisdiction() {
        let input = r#"
            STATUTE jurisdictional: "Regional Statute" {
                JURISDICTION "JP-13"
                WHEN AGE >= 20
                THEN GRANT "Local rights"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.jurisdiction, Some("JP-13".to_string()));
    }

    #[test]
    fn test_parse_version() {
        let input = r#"
            STATUTE versioned: "Versioned Statute" {
                VERSION 3
                WHEN AGE >= 18
                THEN GRANT "Rights v3"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.version, 3);
    }

    #[test]
    fn test_parse_full_metadata() {
        let input = r#"
            STATUTE full-meta: "Full Metadata Statute" {
                JURISDICTION "US-CA"
                VERSION 2
                EFFECTIVE_DATE "2024-01-15"
                EXPIRY_DATE "2029-12-31"
                WHEN AGE >= 21
                THEN GRANT "Full rights"
                DISCRETION "Consider circumstances"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.jurisdiction, Some("US-CA".to_string()));
        assert_eq!(statute.version, 2);
        assert!(statute.temporal_validity.effective_date.is_some());
        assert!(statute.temporal_validity.expiry_date.is_some());
        assert!(statute.discretion_logic.is_some());
    }

    #[test]
    fn test_parse_has_keyword() {
        let input = r#"
            STATUTE citizen: "Citizenship Requirement" {
                WHEN HAS citizen
                THEN GRANT "Voting rights"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.preconditions.len(), 1);
        assert!(matches!(
            &statute.preconditions[0],
            Condition::HasAttribute { key } if key == "citizen"
        ));
    }

    #[test]
    fn test_parse_has_with_string() {
        let input = r#"
            STATUTE status: "Status Requirement" {
                WHEN HAS "active-member"
                THEN GRANT "Member benefits"
            }
        "#;

        let parser = LegalDslParser::new();
        let statute = parser.parse_statute(input).unwrap();

        assert_eq!(statute.preconditions.len(), 1);
        assert!(matches!(
            &statute.preconditions[0],
            Condition::HasAttribute { key } if key == "active-member"
        ));
    }

    #[test]
    fn test_parse_multiple_statutes() {
        let input = r#"
            // First statute
            STATUTE adult-rights: "Adult Rights" {
                WHEN AGE >= 18
                THEN GRANT "Full legal capacity"
            }

            // Second statute
            STATUTE senior-benefits: "Senior Benefits" {
                WHEN AGE >= 65
                THEN GRANT "Pension eligibility"
            }

            /* Third statute with block comment */
            STATUTE minor-protection: "Minor Protection" {
                WHEN AGE < 18
                THEN GRANT "Protected status"
            }
        "#;

        let parser = LegalDslParser::new();
        let statutes = parser.parse_statutes(input).unwrap();

        assert_eq!(statutes.len(), 3);
        assert_eq!(statutes[0].id, "adult-rights");
        assert_eq!(statutes[1].id, "senior-benefits");
        assert_eq!(statutes[2].id, "minor-protection");
    }

    #[test]
    fn test_parse_statutes_with_metadata() {
        let input = r#"
            STATUTE law-1: "Law One" {
                JURISDICTION "US"
                VERSION 2
                WHEN AGE >= 18
                THEN GRANT "Rights"
            }

            STATUTE law-2: "Law Two" {
                JURISDICTION "JP"
                EFFECTIVE_DATE 2024-01-01
                WHEN INCOME <= 5000000
                THEN GRANT "Subsidy"
                DISCRETION "Consider circumstances"
            }
        "#;

        let parser = LegalDslParser::new();
        let statutes = parser.parse_statutes(input).unwrap();

        assert_eq!(statutes.len(), 2);
        assert_eq!(statutes[0].jurisdiction, Some("US".to_string()));
        assert_eq!(statutes[0].version, 2);
        assert_eq!(statutes[1].jurisdiction, Some("JP".to_string()));
        assert!(statutes[1].temporal_validity.effective_date.is_some());
        assert!(statutes[1].discretion_logic.is_some());
    }

    #[test]
    fn test_parse_single_statute_via_parse_statutes() {
        let input = r#"
            STATUTE single: "Single Statute" {
                WHEN AGE >= 21
                THEN GRANT "Something"
            }
        "#;

        let parser = LegalDslParser::new();
        let statutes = parser.parse_statutes(input).unwrap();

        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].id, "single");
    }

    #[test]
    fn test_parse_import_simple() {
        let input = r#"
            IMPORT "base/laws.legalis"

            STATUTE child: "Child Statute" {
                WHEN AGE >= 18
                THEN GRANT "Rights"
            }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.imports.len(), 1);
        assert_eq!(doc.imports[0].path, "base/laws.legalis");
        assert!(doc.imports[0].alias.is_none());
        assert_eq!(doc.statutes.len(), 1);
    }

    #[test]
    fn test_parse_import_with_alias() {
        let input = r#"
            IMPORT "other/laws.legalis" AS other

            STATUTE derived: "Derived Statute" {
                WHEN other.adult_rights
                THEN GRANT "Extended rights"
            }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.imports.len(), 1);
        assert_eq!(doc.imports[0].path, "other/laws.legalis");
        assert_eq!(doc.imports[0].alias, Some("other".to_string()));
        assert_eq!(doc.statutes.len(), 1);

        // Check that the condition references the imported module
        assert_eq!(doc.statutes[0].conditions.len(), 1);
        match &doc.statutes[0].conditions[0] {
            ast::ConditionNode::HasAttribute { key } => {
                assert_eq!(key, "other.adult_rights");
            }
            _ => panic!("Expected HasAttribute condition"),
        }
    }

    #[test]
    fn test_parse_multiple_imports() {
        let input = r#"
            IMPORT "core/basic.legalis" AS basic
            IMPORT "extensions/premium.legalis" AS premium

            STATUTE combined: "Combined Features" {
                WHEN basic.eligibility AND premium.subscription
                THEN GRANT "Full access"
            }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.imports.len(), 2);
        assert_eq!(doc.imports[0].path, "core/basic.legalis");
        assert_eq!(doc.imports[0].alias, Some("basic".to_string()));
        assert_eq!(doc.imports[1].path, "extensions/premium.legalis");
        assert_eq!(doc.imports[1].alias, Some("premium".to_string()));
        assert_eq!(doc.statutes.len(), 1);
    }

    #[test]
    fn test_parse_document_no_imports() {
        let input = r#"
            STATUTE standalone: "Standalone Statute" {
                WHEN AGE >= 18
                THEN GRANT "Rights"
            }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert!(doc.imports.is_empty());
        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "standalone");
    }

    #[test]
    fn test_parse_document_multiple_statutes() {
        let input = r#"
            IMPORT "common.legalis" AS common

            STATUTE statute1: "First" {
                WHEN AGE >= 18
                THEN GRANT "First benefit"
            }

            STATUTE statute2: "Second" {
                WHEN common.employed
                THEN GRANT "Second benefit"
            }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.imports.len(), 1);
        assert_eq!(doc.statutes.len(), 2);
        assert_eq!(doc.statutes[0].id, "statute1");
        assert_eq!(doc.statutes[1].id, "statute2");
    }

    #[test]
    fn test_exception_clause() {
        let dsl = r#"
        STATUTE emergency-override: "Emergency Override" {
            WHEN AGE >= 18
            THEN GRANT "Emergency powers"
            EXCEPTION WHEN HAS medical_emergency "Medical emergencies bypass age requirement"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        let statute = &doc.statutes[0];
        assert_eq!(statute.exceptions.len(), 1);
        assert_eq!(statute.exceptions[0].description, "Medical emergencies bypass age requirement");
        assert_eq!(statute.exceptions[0].conditions.len(), 1);
    }

    #[test]
    fn test_amendment_clause() {
        let dsl = r#"
        STATUTE voting-age-update: "Voting Age Update" {
            AMENDMENT voting-rights VERSION 3 EFFECTIVE_DATE 2024-01-15 "Lowered voting age to 16"
            WHEN AGE >= 16
            THEN GRANT "Right to vote"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        let statute = &doc.statutes[0];
        assert_eq!(statute.amendments.len(), 1);
        assert_eq!(statute.amendments[0].target_id, "voting-rights");
        assert_eq!(statute.amendments[0].version, Some(3));
        assert_eq!(statute.amendments[0].date, Some("2024-1-15".to_string()));
        assert_eq!(statute.amendments[0].description, "Lowered voting age to 16");
    }

    #[test]
    fn test_supersedes_clause() {
        let dsl = r#"
        STATUTE new-tax-law: "New Tax Law" {
            SUPERSEDES old-tax-2020, old-tax-2021
            WHEN INCOME >= 50000
            THEN OBLIGATION "Pay income tax"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        let statute = &doc.statutes[0];
        assert_eq!(statute.supersedes.len(), 2);
        assert!(statute.supersedes.contains(&"old-tax-2020".to_string()));
        assert!(statute.supersedes.contains(&"old-tax-2021".to_string()));
    }

    #[test]
    fn test_comprehensive_statute() {
        let dsl = r#"
        STATUTE comprehensive-law: "Comprehensive Law" {
            JURISDICTION "US-CA"
            VERSION 2
            EFFECTIVE_DATE 2024-01-01
            AMENDMENT old-law VERSION 1 "Updated rules"
            SUPERSEDES legacy-law
            WHEN AGE >= 21 AND HAS license
            THEN GRANT "Driving privileges"
            EXCEPTION WHEN HAS medical_condition "Requires medical clearance"
            DISCRETION "Judge may waive requirements for hardship"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        let statute = &doc.statutes[0];
        assert_eq!(statute.id, "comprehensive-law");
        assert_eq!(statute.amendments.len(), 1);
        assert_eq!(statute.supersedes.len(), 1);
        assert_eq!(statute.exceptions.len(), 1);
        assert!(statute.discretion.is_some());
    }

    #[test]
    fn test_multiple_exceptions() {
        let dsl = r#"
        STATUTE age-restricted: "Age Restricted Activity" {
            WHEN AGE >= 18
            THEN GRANT "Access"
            EXCEPTION WHEN HAS guardian_consent "Minors with consent"
            EXCEPTION "Emergency situations"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(dsl).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        let statute = &doc.statutes[0];
        assert_eq!(statute.exceptions.len(), 2);
        assert!(!statute.exceptions[0].conditions.is_empty());
        assert_eq!(statute.exceptions[1].conditions.len(), 0);
    }
}
