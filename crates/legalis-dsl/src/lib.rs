//! Legalis-DSL: Domain Specific Language for legal document parsing.
//!
//! This crate provides parsing and AST representation for legal documents,
//! enabling structured representation of statutes and legal rules.

use legalis_core::{Condition, Effect, EffectType, Statute};
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

    fn tokenize(&self, input: &str) -> DslResult<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        #[allow(unused_variables, unused_assignments)]
        let mut position = 0;

        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                    position += 1;
                }
                '{' => {
                    tokens.push(Token::LBrace);
                    chars.next();
                    position += 1;
                }
                '}' => {
                    tokens.push(Token::RBrace);
                    chars.next();
                    position += 1;
                }
                ':' => {
                    tokens.push(Token::Colon);
                    chars.next();
                    position += 1;
                }
                '"' => {
                    chars.next();
                    position += 1;
                    let mut s = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '"' {
                            chars.next();
                            position += 1;
                            break;
                        }
                        s.push(c);
                        chars.next();
                        position += 1;
                    }
                    tokens.push(Token::StringLit(s));
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let mut word = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' || c == '-' {
                            word.push(c);
                            chars.next();
                            position += 1;
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
                            position += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(num.parse().unwrap_or(0)));
                }
                '>' | '<' | '=' | '!' => {
                    let mut op = String::new();
                    op.push(ch);
                    chars.next();
                    position += 1;
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            op.push(next);
                            chars.next();
                            position += 1;
                        }
                    }
                    tokens.push(Token::Operator(op));
                }
                _ => {
                    chars.next();
                    position += 1;
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
                Token::RBrace => break,
                _ => {}
            }
        }

        let effect = effect.unwrap_or_else(|| Effect::new(EffectType::Custom, "No effect specified"));

        let mut statute = Statute::new(id, title, effect);
        statute.preconditions = conditions;
        statute.discretion_logic = discretion;

        Ok(statute)
    }

    fn parse_condition<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::Age) => {
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_number(iter)?;
                Ok(Some(Condition::Age {
                    operator: op,
                    value: value as u32,
                }))
            }
            Some(Token::Income) => {
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_number(iter)?;
                Ok(Some(Condition::Income {
                    operator: op,
                    value,
                }))
            }
            Some(Token::Ident(key)) => Ok(Some(Condition::HasAttribute { key: key.clone() })),
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
                _ => Err(DslError::InvalidCondition(format!("Unknown operator: {op}"))),
            },
            _ => Err(DslError::InvalidCondition("Expected comparison operator".to_string())),
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
