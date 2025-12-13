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

pub use ast::*;
pub use parser::*;

/// Errors that can occur during DSL parsing.
#[derive(Debug, Error)]
pub enum DslError {
    #[error("Parse error at position {position}: {message}")]
    ParseError { position: usize, message: String },

    #[error("Invalid condition: {0}")]
    InvalidCondition(String),

    #[error("Invalid effect: {0}")]
    InvalidEffect(String),

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Unclosed comment at position {0}")]
    UnclosedComment(usize),

    #[error("Unmatched parenthesis at position {0}")]
    UnmatchedParen(usize),
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
        let tokens = self.tokenize(input)?;
        self.parse_tokens(&tokens)
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
                            return Err(DslError::UnclosedComment(comment_start));
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

    fn tokenize(&self, input: &str) -> DslResult<Vec<Token>> {
        let input = self.strip_comments(input)?;
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        let mut _position = 0;

        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                    _position += 1;
                }
                '(' => {
                    tokens.push(Token::LParen);
                    chars.next();
                    _position += 1;
                }
                ')' => {
                    tokens.push(Token::RParen);
                    chars.next();
                    _position += 1;
                }
                '{' => {
                    tokens.push(Token::LBrace);
                    chars.next();
                    _position += 1;
                }
                '}' => {
                    tokens.push(Token::RBrace);
                    chars.next();
                    _position += 1;
                }
                ':' => {
                    tokens.push(Token::Colon);
                    chars.next();
                    _position += 1;
                }
                '"' => {
                    chars.next();
                    _position += 1;
                    let mut s = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '"' {
                            chars.next();
                            _position += 1;
                            break;
                        }
                        s.push(c);
                        chars.next();
                        _position += 1;
                    }
                    tokens.push(Token::StringLit(s));
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let mut word = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' || c == '-' {
                            word.push(c);
                            chars.next();
                            _position += 1;
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
                    tokens.push(token);
                }
                _ if ch.is_numeric() => {
                    let mut num = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_numeric() {
                            num.push(c);
                            chars.next();
                            _position += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(num.parse().unwrap_or(0)));
                }
                '-' => {
                    tokens.push(Token::Dash);
                    chars.next();
                    _position += 1;
                }
                '>' | '<' | '=' | '!' => {
                    let mut op = String::new();
                    op.push(ch);
                    chars.next();
                    _position += 1;
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            op.push(next);
                            chars.next();
                            _position += 1;
                        }
                    }
                    tokens.push(Token::Operator(op));
                }
                _ => {
                    chars.next();
                    _position += 1;
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
                return Err(DslError::ParseError {
                    position: 0,
                    message: "Expected 'STATUTE' keyword".to_string(),
                });
            }
        }

        // Get ID
        let id = match iter.next() {
            Some(Token::Ident(s)) => s.clone(),
            _ => {
                return Err(DslError::ParseError {
                    position: 0,
                    message: "Expected statute identifier".to_string(),
                });
            }
        };

        // Expect colon
        match iter.next() {
            Some(Token::Colon) => {}
            _ => {
                return Err(DslError::ParseError {
                    position: 0,
                    message: "Expected ':'".to_string(),
                });
            }
        }

        // Get title
        let title = match iter.next() {
            Some(Token::StringLit(s)) => s.clone(),
            Some(Token::Ident(s)) => s.clone(),
            _ => {
                return Err(DslError::ParseError {
                    position: 0,
                    message: "Expected statute title".to_string(),
                });
            }
        };

        // Expect LBrace
        match iter.next() {
            Some(Token::LBrace) => {}
            _ => {
                return Err(DslError::ParseError {
                    position: 0,
                    message: "Expected '{'".to_string(),
                });
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
                    _ => return Err(DslError::UnmatchedParen(0)),
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
}
