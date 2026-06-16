//! Document- and statute-level parsing for [`LegalDslParser`].
//!
//! This submodule holds the public entry points (`parse_statute`,
//! `parse_statutes`, `parse_document`, `parse_document_with_recovery`) and the
//! parsing of module-system constructs (imports, namespaces, exports) plus the
//! top-level statute body dispatch (`parse_statute_node`). Split out of the
//! original monolithic `parser_impl.rs` to keep every file under 2000 lines.

use legalis_core::Statute;

use super::LegalDslParser;
use crate::ast::{self, Token};
use crate::{DslError, DslResult, ParseResult};

impl LegalDslParser {
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

                // `token` is the `&Token` returned by `peek` above; clone it and
                // then advance the iterator (the bare `next()` discards the same
                // element we already inspected, so no value is lost).
                let token = token.clone();
                iter.next();
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

                // `token` is the `&Token` returned by `peek` above; clone it and
                // then advance the iterator (the bare `next()` discards the same
                // element we already inspected, so no value is lost).
                let token = token.clone();
                iter.next();
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

                // `token` is the `&Token` returned by `peek` above; clone it and
                // then advance the iterator (the bare `next()` discards the same
                // element we already inspected, so no value is lost).
                let token = token.clone();
                iter.next();
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
                    // Parse comma-separated list of statute IDs that are required.
                    //
                    // The list is terminated by the first token that is not an
                    // identifier/string/comma (e.g. the next clause keyword such
                    // as `WHEN`/`THEN`, or the closing `}`). That terminator must
                    // be left in the stream for the outer body loop to dispatch on,
                    // so we `peek` before deciding to consume — using `iter.next()`
                    // here would swallow (and silently drop) the following clause.
                    loop {
                        match iter.peek() {
                            Some(Token::Ident(id)) => {
                                requires.push(id.clone());
                                iter.next();
                            }
                            Some(Token::StringLit(id)) => {
                                requires.push(id.clone());
                                iter.next();
                            }
                            Some(Token::Comma) => {
                                iter.next();
                            }
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
                    // Parse comma-separated list of statute IDs. As with REQUIRES,
                    // peek before consuming so the clause/keyword following the ID
                    // list (or the closing `}`) is preserved for the outer loop and
                    // not silently dropped.
                    loop {
                        match iter.peek() {
                            Some(Token::Ident(id)) => {
                                supersedes.push(id.clone());
                                iter.next();
                            }
                            Some(Token::StringLit(id)) => {
                                supersedes.push(id.clone());
                                iter.next();
                            }
                            Some(Token::Comma) => {
                                iter.next();
                            }
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
}
