//! Pluggable parser modules (roadmap v0.3.4).
//!
//! A [`ParserPlugin`] is a trait-based extension that the [`ExtensibleParser`]
//! consults when it meets a construct the base grammar does not handle. Each
//! plugin advertises the keywords and operators it owns and offers a
//! [`ParserPlugin::try_parse_condition`] hook that either claims the leading
//! tokens (returning a [`ParsedFragment`]) or declines (`None`), letting the next
//! plugin — or the core parser — take over.
//!
//! [`ExtensibleParser`]: super::ExtensibleParser

use super::operators::OperatorDef;
use super::syntax::KeywordSpec;
use crate::DslResult;
use crate::ast::{ConditionNode, Token};

/// The result of a plugin successfully parsing a fragment.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedFragment {
    /// The condition the plugin produced.
    pub node: ConditionNode,
    /// How many leading tokens the plugin consumed.
    pub consumed: usize,
}

impl ParsedFragment {
    /// Creates a fragment.
    pub fn new(node: ConditionNode, consumed: usize) -> Self {
        Self { node, consumed }
    }
}

/// A pluggable parser module.
pub trait ParserPlugin: Send + Sync {
    /// The plugin's name.
    fn name(&self) -> &str;

    /// The plugin's version string.
    fn version(&self) -> &str {
        "1.0.0"
    }

    /// Keywords this plugin introduces (for tooling/highlighting).
    fn keywords(&self) -> Vec<KeywordSpec> {
        Vec::new()
    }

    /// Operators this plugin introduces.
    fn operators(&self) -> Vec<OperatorDef> {
        Vec::new()
    }

    /// Attempts to parse a condition fragment from the start of `tokens`.
    /// Returns `None` if the plugin does not recognize the leading tokens, or
    /// `Some(result)` (which may itself be an error) when it claims them.
    fn try_parse_condition(&self, tokens: &[Token]) -> Option<DslResult<ParsedFragment>>;
}

/// An ordered collection of parser plugins.
#[derive(Default)]
pub struct PluginRegistry {
    plugins: Vec<Box<dyn ParserPlugin>>,
}

impl PluginRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a plugin (consulted after all previously-registered plugins).
    pub fn register(&mut self, plugin: Box<dyn ParserPlugin>) {
        self.plugins.push(plugin);
    }

    /// Returns the names of all registered plugins, in consultation order.
    pub fn names(&self) -> Vec<String> {
        self.plugins.iter().map(|p| p.name().to_string()).collect()
    }

    /// Returns true if any plugin is registered.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// The number of registered plugins.
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Aggregates the keywords advertised by every plugin.
    pub fn all_keywords(&self) -> Vec<KeywordSpec> {
        self.plugins.iter().flat_map(|p| p.keywords()).collect()
    }

    /// Aggregates the operators advertised by every plugin.
    pub fn all_operators(&self) -> Vec<OperatorDef> {
        self.plugins.iter().flat_map(|p| p.operators()).collect()
    }

    /// Consults each plugin in order, returning the first one that claims the
    /// leading tokens.
    pub fn try_parse_condition(&self, tokens: &[Token]) -> Option<DslResult<ParsedFragment>> {
        for plugin in &self.plugins {
            if let Some(result) = plugin.try_parse_condition(tokens) {
                return Some(result);
            }
        }
        None
    }
}
