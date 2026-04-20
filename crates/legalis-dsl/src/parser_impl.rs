//! Core parser implementation for the Legalis DSL.
//!
//! This module contains the `LegalDslParser` struct and all its parsing methods.
//! It is kept separate from `lib.rs` to keep individual files under 2000 lines.

use chrono::NaiveDate;
use legalis_core::{Condition, Effect, EffectType, Statute, TemporalValidity};

use crate::ast::{self, SpannedToken, Token};
use crate::{DslError, DslResult, DslWarning, ParseResult};

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

                let token = iter
                    .next()
                    .expect("invariant: peek succeeded so next is Some")
                    .clone();
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

                let token = iter
                    .next()
                    .expect("invariant: peek succeeded so next is Some")
                    .clone();
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

                let token = iter
                    .next()
                    .expect("invariant: peek succeeded so next is Some")
                    .clone();
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
        let mut result = left.expect("invariant: left is Some (checked is_none above)");

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
        let mut result = left.expect("invariant: left is Some (checked is_none above)");

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

    /// Tokenizes the input DSL text.
    /// Delegates to the standalone tokenizer and emits any collected warnings.
    pub fn tokenize(&self, input: &str) -> DslResult<Vec<SpannedToken>> {
        let (tokens, warnings) = crate::tokenizer::tokenize_input(input)?;
        for warning in warnings {
            self.emit_warning(warning);
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
        let mut result = left.expect("invariant: left is Some (checked is_none above)");

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
        let mut result = left.expect("invariant: left is Some (checked is_none above)");

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
