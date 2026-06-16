//! Parser extensibility framework (roadmap v0.3.4).
//!
//! A clean, additive extension API around the core parser. Nothing here changes
//! the base grammar — instead, an [`ExtensibleParser`] layers five independent,
//! opt-in capabilities on top of [`crate::LegalDslParser`]:
//!
//! * [`syntax`] — user-defined keywords and grammar productions.
//! * [`operators`] — domain-specific operators with precedence/associativity and
//!   a precedence-climbing expression parser.
//! * [`literals`] — custom literal forms (money/percent/duration) with parse +
//!   validation.
//! * [`plugin`] — a trait-based [`plugin::ParserPlugin`] interface the parser
//!   consults for unrecognized constructs.
//! * [`compat`] — version-aware backward-compatibility shims that accept older
//!   syntax and normalize it (emitting deprecation diagnostics).
//!
//! The [`ExtensibleParser::parse_condition`] entry point runs the compatibility
//! normaliser, then dispatches to a registered production, then to the plugins,
//! and finally falls back to the core condition grammar — so existing inputs are
//! always parsed exactly as before.

use crate::LegalDslParser;
use crate::ast::{ConditionNode, Token};
use crate::{DslError, DslResult, DslWarning};
use std::cell::RefCell;

pub mod compat;
pub mod literals;
pub mod operators;
pub mod plugin;
pub mod syntax;

#[cfg(test)]
mod tests;

pub use compat::{CompatibilityLayer, DeprecationRule, SyntaxVersion};
pub use literals::{
    CustomLiteral, DurationLiteral, LiteralRegistry, LiteralValue, MoneyLiteral, PercentLiteral,
};
pub use operators::{Associativity, ExprNode, OperatorDef, OperatorFixity, OperatorTable};
pub use plugin::{ParsedFragment, ParserPlugin, PluginRegistry};
pub use syntax::{GrammarProduction, KeywordSpec, ProductionHandler, SyntaxExtensionRegistry};

/// The syntax version this build targets (used as the default compatibility
/// target).
pub const CURRENT_SYNTAX_VERSION: SyntaxVersion = SyntaxVersion::new(0, 3, 0);

/// Tokenizes `input` into bare tokens via the core lexer (comments stripped).
pub(crate) fn tokenize(input: &str) -> DslResult<Vec<Token>> {
    let (spanned, _warnings) = crate::tokenizer::tokenize_input(input)?;
    Ok(spanned.into_iter().map(|s| s.token).collect())
}

/// Returns the uppercase spelling of a leading custom keyword (an identifier),
/// used to dispatch to user-defined productions/plugins.
pub(crate) fn leading_keyword(tokens: &[Token]) -> Option<String> {
    match tokens.first() {
        Some(Token::Ident(s)) => Some(s.to_uppercase()),
        _ => None,
    }
}

/// A parser that consults a stack of opt-in extensions before falling back to the
/// core grammar.
pub struct ExtensibleParser {
    /// Registered operators (for [`ExtensibleParser::parse_expression`]).
    pub operators: OperatorTable,
    /// Registered custom literal forms.
    pub literals: LiteralRegistry,
    /// Registered keywords and grammar productions.
    pub syntax: SyntaxExtensionRegistry,
    /// Registered parser plugins.
    pub plugins: PluginRegistry,
    /// The backward-compatibility normaliser.
    pub compat: CompatibilityLayer,
    base: LegalDslParser,
    last_warnings: RefCell<Vec<DslWarning>>,
}

impl Default for ExtensibleParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtensibleParser {
    /// Creates a parser with the standard operator table, default literal forms
    /// and the built-in compatibility rules at [`CURRENT_SYNTAX_VERSION`].
    pub fn new() -> Self {
        Self {
            operators: OperatorTable::standard(),
            literals: LiteralRegistry::with_defaults(),
            syntax: SyntaxExtensionRegistry::new(),
            plugins: PluginRegistry::new(),
            compat: CompatibilityLayer::with_builtin_rules(CURRENT_SYNTAX_VERSION),
            base: LegalDslParser::new(),
            last_warnings: RefCell::new(Vec::new()),
        }
    }

    /// Creates a parser with no extensions registered (empty operator table, no
    /// literals, no compatibility rules at the given target version).
    pub fn empty(target: SyntaxVersion) -> Self {
        Self {
            operators: OperatorTable::new(),
            literals: LiteralRegistry::new(),
            syntax: SyntaxExtensionRegistry::new(),
            plugins: PluginRegistry::new(),
            compat: CompatibilityLayer::new(target),
            base: LegalDslParser::new(),
            last_warnings: RefCell::new(Vec::new()),
        }
    }

    /// Registers an operator (builder style).
    pub fn with_operator(mut self, def: OperatorDef) -> Self {
        self.operators.register(def);
        self
    }

    /// Registers a custom literal form (builder style).
    pub fn with_literal(mut self, literal: Box<dyn CustomLiteral>) -> Self {
        self.literals.register(literal);
        self
    }

    /// Registers a grammar production (builder style).
    pub fn with_production<F>(
        mut self,
        trigger: impl Into<String>,
        description: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: Fn(&[Token]) -> DslResult<ConditionNode> + Send + Sync + 'static,
    {
        self.syntax
            .register_production(trigger, description, handler);
        self
    }

    /// Registers a parser plugin (builder style).
    pub fn with_plugin(mut self, plugin: Box<dyn ParserPlugin>) -> Self {
        self.plugins.register(plugin);
        self
    }

    /// Adds a compatibility rule (builder style).
    pub fn with_compat_rule(mut self, rule: DeprecationRule) -> Self {
        self.compat.add_rule(rule);
        self
    }

    /// Replaces the compatibility layer with one targeting `version` (built-in
    /// rules included).
    pub fn with_target_version(mut self, version: SyntaxVersion) -> Self {
        self.compat = CompatibilityLayer::with_builtin_rules(version);
        self
    }

    /// Returns the deprecation warnings emitted by the most recent
    /// [`parse_condition`](Self::parse_condition) call.
    pub fn warnings(&self) -> Vec<DslWarning> {
        self.last_warnings.borrow().clone()
    }

    /// Parses a condition, consulting (in order): the compatibility normaliser,
    /// user productions, registered plugins, and finally the core grammar.
    pub fn parse_condition(&self, input: &str) -> DslResult<ConditionNode> {
        let (normalized, warnings) = self.compat.normalize(input)?;
        *self.last_warnings.borrow_mut() = warnings;

        let tokens = tokenize(&normalized)?;
        if tokens.is_empty() {
            return Err(DslError::parse_error("Empty condition"));
        }

        // 1. User-defined productions, dispatched by leading custom keyword.
        if let Some(trigger) = leading_keyword(&tokens)
            && let Some(production) = self.syntax.production_for(&trigger)
        {
            return (production.handler)(&tokens);
        }

        // 2. Plugins, consulted in registration order.
        if let Some(result) = self.plugins.try_parse_condition(&tokens) {
            return result.map(|fragment| fragment.node);
        }

        // 3. Fall back to the core condition grammar.
        let mut iter = tokens.iter().peekable();
        self.base
            .parse_condition_node(&mut iter)?
            .ok_or_else(|| DslError::parse_error("Could not parse condition"))
    }

    /// Parses an arithmetic/operator expression using the registered operator
    /// table.
    pub fn parse_expression(&self, input: &str) -> DslResult<ExprNode> {
        self.operators.parse(input)
    }

    /// Attempts to parse `lexeme` as one of the registered custom literals.
    pub fn try_literal(&self, lexeme: &str) -> Option<(String, LiteralValue)> {
        self.literals.try_parse(lexeme)
    }

    /// Aggregates every keyword known to the extension layer (productions +
    /// plugins).
    pub fn known_keywords(&self) -> Vec<KeywordSpec> {
        let mut keywords = self.syntax.keywords();
        keywords.extend(self.plugins.all_keywords());
        keywords
    }
}
