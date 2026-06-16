//! Parsing of statute clause nodes (effects, exceptions, defaults, delegates,
//! scope, constraints, amendments) producing the corresponding `crate::ast`
//! `*Node` values.
//!
//! These methods are `pub(crate)` because they are dispatched from
//! `parse_statute_node` in the sibling [`super::document`] module. Split out of
//! the original `parser_impl.rs` to keep every file under 2000 lines.

use super::LegalDslParser;
use crate::ast::{self, Token};
use crate::{DslError, DslResult};

impl LegalDslParser {
    /// Parses an effect into an AST EffectNode.
    pub(crate) fn parse_effect_node<'a, I>(
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
    pub(crate) fn parse_exception_node<'a, I>(
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
    pub(crate) fn parse_default_node<'a, I>(
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
    pub(crate) fn parse_delegate_node<'a, I>(
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
    pub(crate) fn parse_scope_node<'a, I>(
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
    pub(crate) fn parse_constraint_node<'a, I>(
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
    pub(crate) fn parse_amendment_node<'a, I>(
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
}
