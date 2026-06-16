//! Domain-specific language variants (roadmap v0.3.2).
//!
//! Each *legal domain* layers an additive vocabulary on top of the core grammar:
//! a set of recognized keywords/operators, a set of domain-specific
//! condition-builders that lower bespoke syntax (e.g. tax brackets, penalty
//! ranges, emission limits) into ordinary [`ConditionNode`]s, and a validator
//! that checks the domain-specific invariants of a statute.
//!
//! The design is strictly **opt-in** and **additive**:
//!
//! * The base grammar is untouched. Domain keywords such as `BRACKET` or
//!   `MENS_REA` lex as ordinary identifiers; a domain only assigns them meaning
//!   when its [`LegalDomain::parse_condition`] entry point is invoked explicitly.
//! * A statute is *tagged* with a domain via a plain `DEFAULT domain "<name>"`
//!   declaration (already valid base syntax). [`domain_tag`] reads the tag and
//!   the [`DomainRegistry`] applies the matching domain's validation. Statutes
//!   without a tag are never affected.
//!
//! Domain conditions lower to the existing [`ConditionNode`] atoms, so they
//! round-trip through the pretty-printer and interoperate with every other part
//! of the crate (type inference, codegen, formal export, …) for free.
//!
//! The five built-in domains live in their own submodules:
//! [`tax`], [`criminal`], [`environmental`], [`financial`] and [`healthcare`].

use crate::ast::{ConditionNode, ConditionValue, DefaultNode, LegalDocument, StatuteNode, Token};
use crate::{DslError, DslResult};
use std::collections::BTreeMap;

pub mod criminal;
pub mod environmental;
pub mod financial;
pub mod healthcare;
pub mod tax;

#[cfg(test)]
mod tests;

pub use criminal::CriminalLawDomain;
pub use environmental::EnvironmentalDomain;
pub use financial::FinancialServicesDomain;
pub use healthcare::HealthcareDomain;
pub use tax::TaxLawDomain;

/// The conventional `DEFAULT` field used to tag a statute with a domain.
pub const DOMAIN_TAG_FIELD: &str = "domain";

/// Severity of a [`DomainDiagnostic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DomainSeverity {
    /// A definite violation of a domain rule.
    Error,
    /// A likely problem worth flagging.
    Warning,
    /// Informational note.
    Info,
}

impl std::fmt::Display for DomainSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        };
        f.write_str(s)
    }
}

/// A diagnostic produced by a domain validator.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DomainDiagnostic {
    /// The domain that produced the diagnostic.
    pub domain: String,
    /// Severity.
    pub severity: DomainSeverity,
    /// Stable machine-readable code (e.g. `tax.rate-out-of-range`).
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// The statute the diagnostic refers to, when known.
    pub statute_id: Option<String>,
}

impl DomainDiagnostic {
    /// Creates a diagnostic.
    pub fn new(
        domain: impl Into<String>,
        severity: DomainSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            domain: domain.into(),
            severity,
            code: code.into(),
            message: message.into(),
            statute_id: None,
        }
    }

    /// Attaches a statute id.
    pub fn for_statute(mut self, id: impl Into<String>) -> Self {
        self.statute_id = Some(id.into());
        self
    }

    /// Returns true when this diagnostic is an error.
    pub fn is_error(&self) -> bool {
        self.severity == DomainSeverity::Error
    }
}

/// A domain keyword with a short description (used for tooling/documentation).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DomainKeyword {
    /// The keyword spelling (canonical, uppercase by convention).
    pub keyword: String,
    /// What the keyword introduces.
    pub summary: String,
}

impl DomainKeyword {
    /// Creates a keyword descriptor.
    pub fn new(keyword: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            summary: summary.into(),
        }
    }
}

/// A domain operator/modifier word with a short description.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DomainOperator {
    /// The operator spelling.
    pub symbol: String,
    /// What the operator means.
    pub summary: String,
}

impl DomainOperator {
    /// Creates an operator descriptor.
    pub fn new(symbol: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            summary: summary.into(),
        }
    }
}

/// The aggregate vocabulary exposed by a domain.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DomainVocabulary {
    /// Domain name.
    pub domain: String,
    /// Recognized keywords.
    pub keywords: Vec<DomainKeyword>,
    /// Recognized operators/modifiers.
    pub operators: Vec<DomainOperator>,
}

/// A specialized legal domain: an additive vocabulary plus condition-builders and
/// validation layered on the core grammar.
pub trait LegalDomain: Send + Sync {
    /// The canonical domain name used in `DEFAULT domain "<name>"` tags.
    fn name(&self) -> &str;

    /// A one-line description of the domain.
    fn description(&self) -> &str;

    /// The keywords this domain recognizes.
    fn keywords(&self) -> Vec<DomainKeyword>;

    /// The operators/modifier words this domain recognizes.
    fn operators(&self) -> Vec<DomainOperator>;

    /// Parses a single domain-specific condition expression into a core
    /// [`ConditionNode`]. Returns an error for syntax the domain does not
    /// recognize.
    fn parse_condition(&self, input: &str) -> DslResult<ConditionNode>;

    /// Validates a statute against this domain's invariants, returning any
    /// diagnostics found.
    fn validate_statute(&self, statute: &StatuteNode) -> Vec<DomainDiagnostic>;

    /// The aggregate vocabulary of this domain.
    fn vocabulary(&self) -> DomainVocabulary {
        DomainVocabulary {
            domain: self.name().to_string(),
            keywords: self.keywords(),
            operators: self.operators(),
        }
    }
}

/// A registry of available domains, keyed by name.
#[derive(Default)]
pub struct DomainRegistry {
    domains: BTreeMap<String, Box<dyn LegalDomain>>,
}

impl DomainRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a domain (replacing any existing one with the same name).
    pub fn register(&mut self, domain: Box<dyn LegalDomain>) {
        self.domains.insert(domain.name().to_string(), domain);
    }

    /// Looks up a domain by name.
    pub fn get(&self, name: &str) -> Option<&dyn LegalDomain> {
        self.domains.get(name).map(|b| b.as_ref())
    }

    /// Returns the names of all registered domains (sorted).
    pub fn names(&self) -> Vec<String> {
        self.domains.keys().cloned().collect()
    }

    /// Returns true if a domain with the given name is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.domains.contains_key(name)
    }

    /// Parses a condition using the named domain's grammar.
    pub fn parse_condition(&self, domain: &str, input: &str) -> DslResult<ConditionNode> {
        match self.get(domain) {
            Some(d) => d.parse_condition(input),
            None => Err(DslError::parse_error(format!(
                "Unknown legal domain: '{domain}'"
            ))),
        }
    }

    /// Validates a single statute against its tagged domain (if any).
    pub fn validate_statute(&self, statute: &StatuteNode) -> Vec<DomainDiagnostic> {
        match domain_tag(statute).and_then(|tag| self.get(&tag)) {
            Some(domain) => domain
                .validate_statute(statute)
                .into_iter()
                .map(|d| d.for_statute(&statute.id))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Validates every statute in a document against its tagged domain.
    pub fn validate_document(&self, doc: &LegalDocument) -> Vec<DomainDiagnostic> {
        doc.statutes
            .iter()
            .flat_map(|s| self.validate_statute(s))
            .collect()
    }
}

/// Builds a registry pre-populated with the five built-in domains.
pub fn builtin_registry() -> DomainRegistry {
    let mut registry = DomainRegistry::new();
    registry.register(Box::new(TaxLawDomain));
    registry.register(Box::new(CriminalLawDomain));
    registry.register(Box::new(EnvironmentalDomain));
    registry.register(Box::new(FinancialServicesDomain));
    registry.register(Box::new(HealthcareDomain));
    registry
}

/// Returns the domain a statute is tagged with, if any. The tag is a
/// `DEFAULT domain "<name>"` declaration; the returned name is lowercased.
pub fn domain_tag(statute: &StatuteNode) -> Option<String> {
    statute
        .defaults
        .iter()
        .find(|d| d.field.eq_ignore_ascii_case(DOMAIN_TAG_FIELD))
        .and_then(|d| match &d.value {
            ConditionValue::String(s) => Some(s.to_lowercase()),
            _ => None,
        })
}

/// Returns true if a statute is tagged with the given domain (case-insensitive).
pub fn is_tagged_with(statute: &StatuteNode, domain: &str) -> bool {
    domain_tag(statute).is_some_and(|t| t.eq_ignore_ascii_case(domain))
}

/// Returns a clone of `statute` tagged with `domain` via a `DEFAULT domain`
/// declaration. If a domain tag already exists it is updated in place. Purely
/// additive — the input is not mutated.
pub fn tag_statute(statute: &StatuteNode, domain: &str) -> StatuteNode {
    let mut out = statute.clone();
    let value = ConditionValue::String(domain.to_string());
    if let Some(existing) = out
        .defaults
        .iter_mut()
        .find(|d| d.field.eq_ignore_ascii_case(DOMAIN_TAG_FIELD))
    {
        existing.value = value;
    } else {
        out.defaults.push(DefaultNode {
            field: DOMAIN_TAG_FIELD.to_string(),
            value,
        });
    }
    out
}

// ---------------------------------------------------------------------------
// Shared parsing helpers used by the individual domain grammars.
// ---------------------------------------------------------------------------

/// Tokenizes a domain condition snippet into bare tokens (comments stripped).
pub(crate) fn domain_tokens(input: &str) -> DslResult<Vec<Token>> {
    let (spanned, _warnings) = crate::tokenizer::tokenize_input(input)?;
    Ok(spanned.into_iter().map(|s| s.token).collect())
}

/// Returns the uppercase "word" for keyword-like tokens, so domain grammars can
/// match keywords uniformly regardless of whether the lexer promoted them to a
/// dedicated token (e.g. `FROM`, `IN`, `AGE`, `INCOME`).
pub(crate) fn token_word(token: &Token) -> Option<String> {
    let word = match token {
        Token::Ident(s) => return Some(s.to_uppercase()),
        Token::StringLit(s) => return Some(s.to_uppercase()),
        Token::From => "FROM",
        Token::In => "IN",
        Token::And => "AND",
        Token::Or => "OR",
        Token::Not => "NOT",
        Token::Has => "HAS",
        Token::Between => "BETWEEN",
        Token::Age => "AGE",
        Token::Income => "INCOME",
        Token::Report => "REPORT",
        Token::Default => "DEFAULT",
        Token::Version => "VERSION",
        _ => return None,
    };
    Some(word.to_string())
}

/// Returns a usable field/identifier name for a token (handles keyword tokens
/// that double as field names, like `AGE`/`INCOME`).
pub(crate) fn token_field_name(token: &Token) -> Option<String> {
    match token {
        Token::Ident(s) => Some(s.clone()),
        Token::StringLit(s) => Some(s.clone()),
        Token::Age => Some("age".to_string()),
        Token::Income => Some("income".to_string()),
        _ => None,
    }
}

/// Extracts an `f64` from a numeric [`ConditionValue`] (or a parseable string),
/// used by validators to range-check rates, ratios and amounts.
pub(crate) fn value_as_f64(value: &ConditionValue) -> Option<f64> {
    match value {
        ConditionValue::Number(n) => Some(*n as f64),
        ConditionValue::String(s) => s.parse::<f64>().ok(),
        _ => None,
    }
}

/// Encodes a percentage/decimal as a [`ConditionValue`]: an integer when whole,
/// otherwise a decimal string (so fractional values survive print/parse).
pub(crate) fn percent_value(value: f64) -> ConditionValue {
    if value.fract() == 0.0 && value.abs() < i64::MAX as f64 {
        ConditionValue::Number(value as i64)
    } else {
        ConditionValue::String(format!("{value}"))
    }
}

/// A minimal forward-only cursor over a token slice, shared by the domain
/// grammars. All consuming methods return [`DslError`] on mismatch.
pub(crate) struct TokenCursor<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> TokenCursor<'a> {
    /// Creates a cursor over `tokens`.
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Returns true when all tokens have been consumed.
    pub(crate) fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Peeks at the next token without consuming it.
    pub(crate) fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }

    /// Consumes and returns the next token.
    pub(crate) fn advance(&mut self) -> Option<&'a Token> {
        let t = self.tokens.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    /// Returns the uppercase keyword of the next token, if it is keyword-like.
    pub(crate) fn peek_word(&self) -> Option<String> {
        self.peek().and_then(token_word)
    }

    /// Consumes the next token, requiring it to be the keyword `word`
    /// (case-insensitive).
    pub(crate) fn expect_keyword(&mut self, word: &str) -> DslResult<()> {
        match self.peek_word() {
            Some(w) if w.eq_ignore_ascii_case(word) => {
                self.advance();
                Ok(())
            }
            other => Err(DslError::parse_error(format!(
                "Expected '{word}', found {}",
                other.unwrap_or_else(|| "end of input".to_string())
            ))),
        }
    }

    /// Consumes an optional keyword, returning whether it was present.
    pub(crate) fn eat_keyword(&mut self, word: &str) -> bool {
        if matches!(self.peek_word(), Some(w) if w.eq_ignore_ascii_case(word)) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consumes a field/identifier name.
    pub(crate) fn expect_field(&mut self) -> DslResult<String> {
        match self.advance().and_then(token_field_name) {
            Some(name) => Ok(name),
            None => Err(DslError::parse_error("Expected a field name")),
        }
    }

    /// Consumes a numeric literal (integer or float) as `f64`.
    pub(crate) fn expect_number(&mut self) -> DslResult<f64> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(*n as f64),
            Some(Token::Float(f)) => Ok(*f),
            _ => Err(DslError::parse_error("Expected a numeric value")),
        }
    }

    /// Consumes a string or identifier as a string value.
    pub(crate) fn expect_string(&mut self) -> DslResult<String> {
        match self.advance() {
            Some(Token::StringLit(s)) | Some(Token::Ident(s)) => Ok(s.clone()),
            _ => Err(DslError::parse_error("Expected a string value")),
        }
    }

    /// Consumes a comparison operator, defaulting to `==` when the next token is
    /// not an operator (so `RATE 10` means `RATE == 10`).
    pub(crate) fn expect_comparison_op(&mut self) -> String {
        if let Some(Token::Operator(op)) = self.peek() {
            let op = op.clone();
            self.advance();
            normalize_op(&op)
        } else {
            "==".to_string()
        }
    }

    /// Requires the cursor to be at end of input (no trailing tokens).
    pub(crate) fn expect_eof(&self) -> DslResult<()> {
        if self.is_eof() {
            Ok(())
        } else {
            Err(DslError::parse_error("Unexpected trailing tokens"))
        }
    }
}

/// Normalizes a raw operator lexeme to its canonical comparison spelling.
fn normalize_op(op: &str) -> String {
    match op {
        "=" => "==".to_string(),
        other => other.to_string(),
    }
}

/// Recursively collects every atomic (non-connective) sub-condition, used by the
/// domain validators to inspect the leaves of a statute's guards.
pub(crate) fn collect_atoms(cond: &ConditionNode, out: &mut Vec<ConditionNode>) {
    match cond {
        ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
            collect_atoms(left, out);
            collect_atoms(right, out);
        }
        ConditionNode::Not(inner) => collect_atoms(inner, out),
        atom => out.push(atom.clone()),
    }
}

/// Collects every atom across all of a statute's condition trees (preconditions,
/// exception carve-outs, delegate/scope guards and constraints).
pub(crate) fn statute_atoms(statute: &StatuteNode) -> Vec<ConditionNode> {
    let mut out = Vec::new();
    for cond in &statute.conditions {
        collect_atoms(cond, &mut out);
    }
    for ex in &statute.exceptions {
        for cond in &ex.conditions {
            collect_atoms(cond, &mut out);
        }
    }
    for d in &statute.delegates {
        for cond in &d.conditions {
            collect_atoms(cond, &mut out);
        }
    }
    if let Some(scope) = &statute.scope {
        for cond in &scope.conditions {
            collect_atoms(cond, &mut out);
        }
    }
    for c in &statute.constraints {
        collect_atoms(&c.condition, &mut out);
    }
    out
}
