//! Core parser implementation for the Legalis DSL.
//!
//! This module defines the [`LegalDslParser`] struct and its parsing methods.
//! The implementation is split across cohesive submodules (by grammar area) so
//! that no single file exceeds 2000 lines, while every method remains an
//! inherent method of [`LegalDslParser`]:
//!
//! * [`document`] — public entry points and document/statute-node parsing
//!   (imports, namespaces, exports, statute bodies).
//! * [`conditions`] — the [`crate::ast::ConditionNode`] recursive-descent grammar.
//! * [`clauses`] — statute clause nodes (effects, exceptions, defaults,
//!   delegates, scope, constraints, amendments).
//! * [`statute`] — the legacy [`legalis_core::Statute`]-producing parse path and
//!   the shared scalar/value helpers.
//!
//! The struct itself, the warning-collection plumbing, and the tokenizer entry
//! point live here in `mod.rs`.

use crate::ast::SpannedToken;
use crate::{DslResult, DslWarning};

mod clauses;
mod conditions;
mod document;
mod statute;

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

    /// Tokenizes the input DSL text.
    /// Delegates to the standalone tokenizer and emits any collected warnings.
    pub fn tokenize(&self, input: &str) -> DslResult<Vec<SpannedToken>> {
        let (tokens, warnings) = crate::tokenizer::tokenize_input(input)?;
        for warning in warnings {
            self.emit_warning(warning);
        }
        Ok(tokens)
    }
}
