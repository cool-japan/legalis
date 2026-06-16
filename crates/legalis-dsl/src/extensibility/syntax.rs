//! User-defined syntax extensions (roadmap v0.3.4).
//!
//! Lets callers register custom **keywords** and **grammar productions** — a
//! production maps a leading trigger keyword to a handler that consumes the
//! token stream and yields a core [`ConditionNode`]. Because custom keywords lex
//! as ordinary identifiers, registering a production gives them meaning without
//! changing the base grammar; unregistered input is unaffected.

use crate::ast::{ConditionNode, Token};
use crate::{DslError, DslResult};
use std::collections::BTreeMap;
use std::sync::Arc;

/// A registered keyword, with optional aliases and a description.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KeywordSpec {
    /// Canonical keyword spelling (uppercase by convention).
    pub keyword: String,
    /// Alternative spellings accepted for the keyword.
    pub aliases: Vec<String>,
    /// Description for tooling/documentation.
    pub description: String,
}

impl KeywordSpec {
    /// Creates a keyword spec.
    pub fn new(keyword: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            aliases: Vec::new(),
            description: description.into(),
        }
    }

    /// Adds an alias.
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Returns true if `word` (case-insensitive) is this keyword or an alias.
    pub fn matches(&self, word: &str) -> bool {
        self.keyword.eq_ignore_ascii_case(word)
            || self.aliases.iter().any(|a| a.eq_ignore_ascii_case(word))
    }
}

/// The handler signature for a grammar production: it receives the full token
/// slice of the construct (including the trigger keyword) and produces a
/// condition node.
pub type ProductionHandler = Arc<dyn Fn(&[Token]) -> DslResult<ConditionNode> + Send + Sync>;

/// A user-defined grammar production.
#[derive(Clone)]
pub struct GrammarProduction {
    /// The leading keyword that triggers this production (uppercase).
    pub trigger: String,
    /// Description for tooling.
    pub description: String,
    /// The parse handler.
    pub handler: ProductionHandler,
}

impl std::fmt::Debug for GrammarProduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GrammarProduction")
            .field("trigger", &self.trigger)
            .field("description", &self.description)
            .finish_non_exhaustive()
    }
}

/// A registry of custom keywords and grammar productions.
#[derive(Default, Clone)]
pub struct SyntaxExtensionRegistry {
    keywords: BTreeMap<String, KeywordSpec>,
    productions: BTreeMap<String, GrammarProduction>,
}

impl SyntaxExtensionRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a keyword.
    pub fn register_keyword(&mut self, spec: KeywordSpec) {
        self.keywords.insert(spec.keyword.to_uppercase(), spec);
    }

    /// Registers a grammar production triggered by `trigger`. The trigger is also
    /// registered as a keyword.
    pub fn register_production<F>(
        &mut self,
        trigger: impl Into<String>,
        description: impl Into<String>,
        handler: F,
    ) where
        F: Fn(&[Token]) -> DslResult<ConditionNode> + Send + Sync + 'static,
    {
        let trigger = trigger.into().to_uppercase();
        let description = description.into();
        self.register_keyword(KeywordSpec::new(trigger.clone(), description.clone()));
        self.productions.insert(
            trigger.clone(),
            GrammarProduction {
                trigger,
                description,
                handler: Arc::new(handler),
            },
        );
    }

    /// Returns true if `word` is a registered keyword or alias.
    pub fn is_keyword(&self, word: &str) -> bool {
        let upper = word.to_uppercase();
        self.keywords.contains_key(&upper) || self.keywords.values().any(|spec| spec.matches(word))
    }

    /// Returns the production triggered by `trigger`, if any.
    pub fn production_for(&self, trigger: &str) -> Option<&GrammarProduction> {
        self.productions.get(&trigger.to_uppercase())
    }

    /// Returns all registered keyword specs (sorted by keyword).
    pub fn keywords(&self) -> Vec<KeywordSpec> {
        self.keywords.values().cloned().collect()
    }

    /// Returns the triggers of all registered productions.
    pub fn production_triggers(&self) -> Vec<String> {
        self.productions.keys().cloned().collect()
    }

    /// Tokenizes `input` and dispatches to the matching production, returning the
    /// produced condition. Errors if the leading token is not a registered
    /// production trigger.
    pub fn parse_condition(&self, input: &str) -> DslResult<ConditionNode> {
        let tokens = super::tokenize(input)?;
        let trigger = super::leading_keyword(&tokens)
            .ok_or_else(|| DslError::parse_error("Expected a leading keyword"))?;
        match self.production_for(&trigger) {
            Some(prod) => (prod.handler)(&tokens),
            None => Err(DslError::parse_error(format!(
                "No registered production for keyword '{trigger}'"
            ))),
        }
    }
}
