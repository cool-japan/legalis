//! Oracle Integration for Off-Chain Facts
//!
//! This module provides oracle integration for bringing off-chain data onto
//! the blockchain for legal statute evaluation.

use crate::EvaluationContext;
use chrono::Datelike;
use std::collections::HashMap;
use std::fmt;

/// Oracle data source type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum OracleSource {
    /// Chainlink oracle
    Chainlink,
    /// API3 oracle
    Api3,
    /// Band Protocol
    BandProtocol,
    /// Custom HTTP API
    HttpApi,
    /// IPFS
    Ipfs,
    /// Government database
    GovernmentDb,
    /// Internal database
    InternalDb,
}

impl fmt::Display for OracleSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OracleSource::Chainlink => write!(f, "Chainlink"),
            OracleSource::Api3 => write!(f, "API3"),
            OracleSource::BandProtocol => write!(f, "Band Protocol"),
            OracleSource::HttpApi => write!(f, "HTTP API"),
            OracleSource::Ipfs => write!(f, "IPFS"),
            OracleSource::GovernmentDb => write!(f, "Government Database"),
            OracleSource::InternalDb => write!(f, "Internal Database"),
        }
    }
}

/// Oracle data feed with metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OracleFeed {
    /// Feed identifier
    pub id: String,
    /// Data source
    pub source: OracleSource,
    /// Current value
    pub value: OracleValue,
    /// Last update timestamp
    pub last_updated: u64,
    /// Update frequency in seconds
    pub update_frequency: u64,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
}

impl OracleFeed {
    /// Create a new oracle feed
    pub fn new(id: String, source: OracleSource, value: OracleValue) -> Self {
        Self {
            id,
            source,
            value,
            last_updated: current_timestamp(),
            update_frequency: 3600, // Default: 1 hour
            confidence: 1.0,
        }
    }

    /// Update the feed value
    pub fn update(&mut self, value: OracleValue) {
        self.value = value;
        self.last_updated = current_timestamp();
    }

    /// Check if feed is stale
    pub fn is_stale(&self) -> bool {
        let age = current_timestamp().saturating_sub(self.last_updated);
        age > self.update_frequency * 2
    }

    /// Get age of data in seconds
    pub fn age_seconds(&self) -> u64 {
        current_timestamp().saturating_sub(self.last_updated)
    }
}

/// Oracle value types
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OracleValue {
    /// Boolean value
    Bool(bool),
    /// Integer value
    Integer(i64),
    /// Floating point value
    Float(f64),
    /// String value
    String(String),
    /// Bytes value
    Bytes(Vec<u8>),
}

impl OracleValue {
    /// Convert to boolean if possible
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            OracleValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Convert to integer if possible
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            OracleValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Convert to float if possible
    pub fn as_float(&self) -> Option<f64> {
        match self {
            OracleValue::Float(f) => Some(*f),
            OracleValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Convert to string if possible
    pub fn as_string(&self) -> Option<&str> {
        match self {
            OracleValue::String(s) => Some(s),
            _ => None,
        }
    }
}

/// Oracle registry for managing data feeds
///
/// # Example
///
/// ```
/// use legalis_core::oracle::{OracleRegistry, OracleSource, OracleValue};
///
/// let mut registry = OracleRegistry::new();
///
/// registry.register_feed(
///     "age-verification",
///     OracleSource::GovernmentDb,
///     OracleValue::Integer(25),
/// );
///
/// let age = registry.get_value("age-verification")
///     .and_then(|v| v.as_integer())
///     .unwrap();
/// assert_eq!(age, 25);
/// ```
pub struct OracleRegistry {
    feeds: HashMap<String, OracleFeed>,
}

impl OracleRegistry {
    /// Create a new oracle registry
    pub fn new() -> Self {
        Self {
            feeds: HashMap::new(),
        }
    }

    /// Register a new oracle feed
    pub fn register_feed(
        &mut self,
        id: impl Into<String>,
        source: OracleSource,
        value: OracleValue,
    ) -> String {
        let id = id.into();
        let feed = OracleFeed::new(id.clone(), source, value);
        self.feeds.insert(id.clone(), feed);
        id
    }

    /// Update an existing feed
    pub fn update_feed(&mut self, id: &str, value: OracleValue) -> Result<(), OracleError> {
        let feed = self
            .feeds
            .get_mut(id)
            .ok_or_else(|| OracleError::FeedNotFound(id.to_string()))?;

        feed.update(value);
        Ok(())
    }

    /// Get the current value from a feed
    pub fn get_value(&self, id: &str) -> Option<&OracleValue> {
        self.feeds.get(id).map(|feed| &feed.value)
    }

    /// Get a feed by ID
    pub fn get_feed(&self, id: &str) -> Option<&OracleFeed> {
        self.feeds.get(id)
    }

    /// Remove a feed
    pub fn remove_feed(&mut self, id: &str) -> Option<OracleFeed> {
        self.feeds.remove(id)
    }

    /// Get all feed IDs
    pub fn list_feeds(&self) -> Vec<&str> {
        self.feeds.keys().map(|s| s.as_str()).collect()
    }

    /// Get number of registered feeds
    pub fn feed_count(&self) -> usize {
        self.feeds.len()
    }

    /// Get all stale feeds
    pub fn stale_feeds(&self) -> Vec<&str> {
        self.feeds
            .iter()
            .filter(|(_, feed)| feed.is_stale())
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// Query feeds by source
    pub fn feeds_by_source(&self, source: OracleSource) -> Vec<&OracleFeed> {
        self.feeds
            .values()
            .filter(|feed| feed.source == source)
            .collect()
    }
}

impl Default for OracleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Oracle-backed evaluation context
///
/// # Example
///
/// ```
/// use legalis_core::{Condition, ComparisonOp, EvaluationContext};
/// use legalis_core::oracle::{OracleContext, OracleRegistry, OracleSource, OracleValue};
///
/// let mut registry = OracleRegistry::new();
/// registry.register_feed("entity-123-age", OracleSource::GovernmentDb, OracleValue::Integer(30));
///
/// let context = OracleContext::new("entity-123", registry);
///
/// let age = context.get_age().unwrap();
/// assert_eq!(age, 30);
/// ```
pub struct OracleContext {
    entity_id: String,
    registry: OracleRegistry,
    attributes: HashMap<String, bool>,
}

impl OracleContext {
    /// Create a new oracle-backed context
    pub fn new(entity_id: impl Into<String>, registry: OracleRegistry) -> Self {
        Self {
            entity_id: entity_id.into(),
            registry,
            attributes: HashMap::new(),
        }
    }

    /// Set an attribute
    pub fn set_attribute(&mut self, key: String, value: bool) {
        self.attributes.insert(key, value);
    }

    /// Get entity ID
    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }

    /// Get the registry
    pub fn registry(&self) -> &OracleRegistry {
        &self.registry
    }

    /// Get mutable registry
    pub fn registry_mut(&mut self) -> &mut OracleRegistry {
        &mut self.registry
    }
}

impl EvaluationContext for OracleContext {
    fn get_attribute(&self, key: &str) -> Option<String> {
        // Try local attributes first
        if let Some(value) = self.attributes.get(key) {
            return Some(value.to_string());
        }

        // Try oracle feed
        self.registry
            .get_value(&format!("{}-{}", self.entity_id, key))
            .and_then(|v| v.as_string())
            .map(|s| s.to_string())
    }

    fn get_age(&self) -> Option<u32> {
        self.registry
            .get_value(&format!("{}-age", self.entity_id))
            .and_then(|v| v.as_integer())
            .map(|i| i as u32)
    }

    fn get_income(&self) -> Option<u64> {
        self.registry
            .get_value(&format!("{}-income", self.entity_id))
            .and_then(|v| v.as_integer())
            .map(|i| i as u64)
    }

    fn get_percentage(&self, key: &str) -> Option<u32> {
        self.registry
            .get_value(&format!("{}-{}", self.entity_id, key))
            .and_then(|v| v.as_integer())
            .map(|i| i as u32)
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        let resolve = |name: &str| -> Option<f64> {
            self.registry
                .get_value(&format!("{}-{}", self.entity_id, name))
                .and_then(|v| v.as_float())
        };
        formula_eval::eval(formula, &resolve).ok()
    }

    fn get_current_timestamp(&self) -> Option<i64> {
        Some(current_timestamp() as i64)
    }

    fn get_current_date(&self) -> Option<chrono::NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: crate::RegionType, _region_id: &str) -> bool {
        false // Not implemented for simplicity
    }

    fn check_relationship(
        &self,
        _relationship_type: crate::RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false // Not implemented for simplicity
    }

    fn get_residency_months(&self) -> Option<u32> {
        // Check for directly stored residency_months attribute
        if let Some(months) = self
            .registry
            .get_value(&format!("{}-residency_months", self.entity_id))
            .and_then(|v| v.as_integer())
        {
            return Some(months as u32);
        }
        // Compute from residency_start date if available (ISO 8601 string)
        self.registry
            .get_value(&format!("{}-residency_start", self.entity_id))
            .and_then(|v| {
                v.as_string()
                    .and_then(|s| s.parse::<chrono::NaiveDate>().ok())
            })
            .map(|start| {
                let today = chrono::Utc::now().date_naive();
                let months = (today.year() - start.year()) * 12 + today.month() as i32
                    - start.month() as i32;
                months.max(0) as u32
            })
    }

    fn get_duration(&self, _unit: crate::DurationUnit) -> Option<u32> {
        None // Not implemented for simplicity
    }
}

/// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Pure-Rust recursive-descent arithmetic formula evaluator.
///
/// Supports literals, variables, `+` `-` `*` `/` `()` grouping, and
/// comparison operators `<` `>` `<=` `>=` `==` `!=`.
/// Operator precedence: comparison < add/sub < mul/div < unary < primary.
///
/// # Safety
/// This is a self-contained Rust parser with no dynamic code execution.
/// It operates purely on the input string and a variable-resolver closure.
pub(crate) mod formula_eval {
    #[derive(Debug, Clone, PartialEq)]
    enum Token {
        Number(f64),
        Ident(String),
        Plus,
        Minus,
        Star,
        Slash,
        Lt,
        Gt,
        Le,
        Ge,
        Eq,
        Ne,
        LParen,
        RParen,
        Eof,
    }

    struct Lexer<'a> {
        bytes: &'a [u8],
        pos: usize,
    }

    impl<'a> Lexer<'a> {
        fn new(input: &'a str) -> Self {
            Self {
                bytes: input.as_bytes(),
                pos: 0,
            }
        }

        fn skip_ws(&mut self) {
            while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
        }

        fn next_token(&mut self) -> Result<Token, String> {
            self.skip_ws();
            if self.pos >= self.bytes.len() {
                return Ok(Token::Eof);
            }
            match self.bytes[self.pos] {
                b'+' => {
                    self.pos += 1;
                    Ok(Token::Plus)
                }
                b'-' => {
                    self.pos += 1;
                    Ok(Token::Minus)
                }
                b'*' => {
                    self.pos += 1;
                    Ok(Token::Star)
                }
                b'/' => {
                    self.pos += 1;
                    Ok(Token::Slash)
                }
                b'(' => {
                    self.pos += 1;
                    Ok(Token::LParen)
                }
                b')' => {
                    self.pos += 1;
                    Ok(Token::RParen)
                }
                b'<' => {
                    if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'=' {
                        self.pos += 2;
                        Ok(Token::Le)
                    } else {
                        self.pos += 1;
                        Ok(Token::Lt)
                    }
                }
                b'>' => {
                    if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'=' {
                        self.pos += 2;
                        Ok(Token::Ge)
                    } else {
                        self.pos += 1;
                        Ok(Token::Gt)
                    }
                }
                b'=' if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'=' => {
                    self.pos += 2;
                    Ok(Token::Eq)
                }
                b'!' if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'=' => {
                    self.pos += 2;
                    Ok(Token::Ne)
                }
                b'0'..=b'9' | b'.' => {
                    let start = self.pos;
                    while self.pos < self.bytes.len()
                        && (self.bytes[self.pos].is_ascii_digit() || self.bytes[self.pos] == b'.')
                    {
                        self.pos += 1;
                    }
                    let s = std::str::from_utf8(&self.bytes[start..self.pos])
                        .map_err(|_| "invalid UTF-8 in number".to_string())?;
                    s.parse::<f64>()
                        .map(Token::Number)
                        .map_err(|_| format!("invalid number: {}", s))
                }
                c if c.is_ascii_alphabetic() || c == b'_' => {
                    let start = self.pos;
                    while self.pos < self.bytes.len()
                        && (self.bytes[self.pos].is_ascii_alphanumeric()
                            || self.bytes[self.pos] == b'_')
                    {
                        self.pos += 1;
                    }
                    let s = std::str::from_utf8(&self.bytes[start..self.pos])
                        .map_err(|_| "invalid UTF-8 in identifier".to_string())?;
                    Ok(Token::Ident(s.to_string()))
                }
                c => Err(format!("unexpected character: {}", c as char)),
            }
        }
    }

    struct Parser<'a> {
        lexer: Lexer<'a>,
        current: Token,
        resolve: &'a dyn Fn(&str) -> Option<f64>,
    }

    impl<'a> Parser<'a> {
        fn new(input: &'a str, resolve: &'a dyn Fn(&str) -> Option<f64>) -> Result<Self, String> {
            let mut lexer = Lexer::new(input);
            let current = lexer.next_token()?;
            Ok(Self {
                lexer,
                current,
                resolve,
            })
        }

        fn advance(&mut self) -> Result<(), String> {
            self.current = self.lexer.next_token()?;
            Ok(())
        }

        fn parse_expr(&mut self) -> Result<f64, String> {
            self.parse_comparison()
        }

        fn parse_comparison(&mut self) -> Result<f64, String> {
            let mut left = self.parse_additive()?;
            loop {
                let tok = self.current.clone();
                match tok {
                    Token::Lt => {
                        self.advance()?;
                        let r = self.parse_additive()?;
                        left = if left < r { 1.0 } else { 0.0 };
                    }
                    Token::Gt => {
                        self.advance()?;
                        let r = self.parse_additive()?;
                        left = if left > r { 1.0 } else { 0.0 };
                    }
                    Token::Le => {
                        self.advance()?;
                        let r = self.parse_additive()?;
                        left = if left <= r { 1.0 } else { 0.0 };
                    }
                    Token::Ge => {
                        self.advance()?;
                        let r = self.parse_additive()?;
                        left = if left >= r { 1.0 } else { 0.0 };
                    }
                    Token::Eq => {
                        self.advance()?;
                        let r = self.parse_additive()?;
                        left = if (left - r).abs() < f64::EPSILON {
                            1.0
                        } else {
                            0.0
                        };
                    }
                    Token::Ne => {
                        self.advance()?;
                        let r = self.parse_additive()?;
                        left = if (left - r).abs() >= f64::EPSILON {
                            1.0
                        } else {
                            0.0
                        };
                    }
                    _ => break,
                }
            }
            Ok(left)
        }

        fn parse_additive(&mut self) -> Result<f64, String> {
            let mut left = self.parse_multiplicative()?;
            loop {
                let tok = self.current.clone();
                match tok {
                    Token::Plus => {
                        self.advance()?;
                        left += self.parse_multiplicative()?;
                    }
                    Token::Minus => {
                        self.advance()?;
                        left -= self.parse_multiplicative()?;
                    }
                    _ => break,
                }
            }
            Ok(left)
        }

        fn parse_multiplicative(&mut self) -> Result<f64, String> {
            let mut left = self.parse_unary()?;
            loop {
                let tok = self.current.clone();
                match tok {
                    Token::Star => {
                        self.advance()?;
                        left *= self.parse_unary()?;
                    }
                    Token::Slash => {
                        self.advance()?;
                        let right = self.parse_unary()?;
                        if right == 0.0 {
                            return Err("division by zero".to_string());
                        }
                        left /= right;
                    }
                    _ => break,
                }
            }
            Ok(left)
        }

        fn parse_unary(&mut self) -> Result<f64, String> {
            let tok = self.current.clone();
            if let Token::Minus = tok {
                self.advance()?;
                return Ok(-self.parse_unary()?);
            }
            self.parse_primary()
        }

        fn parse_primary(&mut self) -> Result<f64, String> {
            let tok = self.current.clone();
            match tok {
                Token::Number(n) => {
                    self.advance()?;
                    Ok(n)
                }
                Token::Ident(name) => {
                    self.advance()?;
                    (self.resolve)(&name).ok_or_else(|| format!("unknown variable: {}", name))
                }
                Token::LParen => {
                    self.advance()?;
                    let val = self.parse_expr()?;
                    let is_rparen = matches!(self.current, Token::RParen);
                    if is_rparen {
                        self.advance()?;
                        Ok(val)
                    } else {
                        Err("expected closing parenthesis".to_string())
                    }
                }
                Token::Eof => Err("unexpected end of formula".to_string()),
                _ => Err("unexpected token in expression".to_string()),
            }
        }
    }

    /// Evaluates an arithmetic formula string with a variable resolver closure.
    ///
    /// Returns `Ok(f64)` on success, or `Err(reason)` on failure.
    /// The `resolve_var` closure maps variable names to their numeric values.
    ///
    /// This is a safe, pure-Rust recursive-descent parser — no dynamic
    /// code execution or external crate dependencies.
    pub(crate) fn eval(
        formula: &str,
        resolve_var: &dyn Fn(&str) -> Option<f64>,
    ) -> Result<f64, String> {
        let mut parser = Parser::new(formula, resolve_var)?;
        let result = parser.parse_expr()?;
        if !matches!(parser.current, Token::Eof) {
            return Err("unexpected tokens after expression".to_string());
        }
        Ok(result)
    }
}

/// Oracle request for fetching data
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OracleRequest {
    /// Request ID
    pub id: String,
    /// Data source
    pub source: OracleSource,
    /// Query parameters
    pub params: HashMap<String, String>,
    /// Callback contract address (if on-chain)
    pub callback_address: Option<String>,
    /// Request timestamp
    pub timestamp: u64,
}

impl OracleRequest {
    /// Create a new oracle request
    pub fn new(id: String, source: OracleSource) -> Self {
        Self {
            id,
            source,
            params: HashMap::new(),
            callback_address: None,
            timestamp: current_timestamp(),
        }
    }

    /// Add a parameter to the request
    pub fn with_param(mut self, key: String, value: String) -> Self {
        self.params.insert(key, value);
        self
    }

    /// Set callback address
    pub fn with_callback(mut self, address: String) -> Self {
        self.callback_address = Some(address);
        self
    }
}

/// Oracle response with data
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OracleResponse {
    /// Request ID this responds to
    pub request_id: String,
    /// Response value
    pub value: OracleValue,
    /// Confidence score
    pub confidence: f64,
    /// Response timestamp
    pub timestamp: u64,
}

impl OracleResponse {
    /// Create a new oracle response
    pub fn new(request_id: String, value: OracleValue, confidence: f64) -> Self {
        Self {
            request_id,
            value,
            confidence,
            timestamp: current_timestamp(),
        }
    }
}

/// Oracle errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum OracleError {
    #[error("Feed not found: {0}")]
    FeedNotFound(String),

    #[error("Invalid value type: expected {expected}, got {actual}")]
    InvalidValueType { expected: String, actual: String },

    #[error("Stale data: {0}")]
    StaleData(String),

    #[error("Oracle unavailable: {0}")]
    OracleUnavailable(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_value_conversions() {
        let bool_val = OracleValue::Bool(true);
        assert_eq!(bool_val.as_bool(), Some(true));
        assert_eq!(bool_val.as_integer(), None);

        let int_val = OracleValue::Integer(42);
        assert_eq!(int_val.as_integer(), Some(42));
        assert_eq!(int_val.as_float(), Some(42.0));
    }

    #[test]
    fn test_oracle_registry() {
        let mut registry = OracleRegistry::new();

        registry.register_feed(
            "test-feed",
            OracleSource::Chainlink,
            OracleValue::Integer(100),
        );

        assert_eq!(registry.feed_count(), 1);

        let value = registry.get_value("test-feed").unwrap();
        assert_eq!(value.as_integer(), Some(100));
    }

    #[test]
    fn test_feed_update() {
        let mut registry = OracleRegistry::new();

        registry.register_feed("price-feed", OracleSource::Api3, OracleValue::Float(1.5));

        registry
            .update_feed("price-feed", OracleValue::Float(2.0))
            .unwrap();

        let value = registry.get_value("price-feed").unwrap();
        assert_eq!(value.as_float(), Some(2.0));
    }

    #[test]
    fn test_oracle_context() {
        let mut registry = OracleRegistry::new();
        registry.register_feed(
            "entity-123-age",
            OracleSource::GovernmentDb,
            OracleValue::Integer(30),
        );

        let context = OracleContext::new("entity-123", registry);

        assert_eq!(context.get_age(), Some(30));
    }

    #[test]
    fn test_stale_detection() {
        let mut feed = OracleFeed::new(
            "test".to_string(),
            OracleSource::HttpApi,
            OracleValue::Integer(1),
        );

        feed.update_frequency = 60; // 1 minute
        feed.last_updated = current_timestamp() - 200; // 3+ minutes ago

        assert!(feed.is_stale());
    }

    #[test]
    fn test_feeds_by_source() {
        let mut registry = OracleRegistry::new();

        registry.register_feed("feed1", OracleSource::Chainlink, OracleValue::Integer(1));
        registry.register_feed("feed2", OracleSource::Api3, OracleValue::Integer(2));
        registry.register_feed("feed3", OracleSource::Chainlink, OracleValue::Integer(3));

        let chainlink_feeds = registry.feeds_by_source(OracleSource::Chainlink);
        assert_eq!(chainlink_feeds.len(), 2);
    }

    #[test]
    fn test_oracle_request() {
        let request = OracleRequest::new("req-1".to_string(), OracleSource::HttpApi)
            .with_param("entity_id".to_string(), "123".to_string())
            .with_callback("0xabc".to_string());

        assert_eq!(request.params.get("entity_id").unwrap(), "123");
        assert_eq!(request.callback_address.unwrap(), "0xabc");
    }

    #[test]
    fn test_oracle_response() {
        let response = OracleResponse::new("req-1".to_string(), OracleValue::Bool(true), 0.95);

        assert_eq!(response.request_id, "req-1");
        assert_eq!(response.confidence, 0.95);
    }

    #[test]
    fn test_source_display() {
        assert_eq!(OracleSource::Chainlink.to_string(), "Chainlink");
        assert_eq!(
            OracleSource::GovernmentDb.to_string(),
            "Government Database"
        );
    }

    #[test]
    fn test_get_percentage_found() {
        let mut registry = OracleRegistry::new();
        registry.register_feed(
            "entity-1-ownership",
            OracleSource::InternalDb,
            OracleValue::Integer(30),
        );
        let context = OracleContext::new("entity-1", registry);
        assert_eq!(context.get_percentage("ownership"), Some(30));
    }

    #[test]
    fn test_get_percentage_not_found() {
        let registry = OracleRegistry::new();
        let context = OracleContext::new("entity-1", registry);
        assert_eq!(context.get_percentage("ownership"), None);
    }

    #[test]
    fn test_get_current_date_returns_something() {
        let registry = OracleRegistry::new();
        let context = OracleContext::new("entity-1", registry);
        let date = context.get_current_date();
        assert!(date.is_some());
        // Year must be reasonable (2020 or later)
        assert!(date.unwrap().year() >= 2020);
    }

    #[test]
    fn test_get_residency_months_from_stored_value() {
        let mut registry = OracleRegistry::new();
        registry.register_feed(
            "entity-1-residency_months",
            OracleSource::GovernmentDb,
            OracleValue::Integer(24),
        );
        let context = OracleContext::new("entity-1", registry);
        assert_eq!(context.get_residency_months(), Some(24));
    }

    #[test]
    fn test_evaluate_formula_basic_arithmetic() {
        let registry = OracleRegistry::new();
        let context = OracleContext::new("entity-1", registry);
        assert_eq!(context.evaluate_formula("2 + 3 * 4"), Some(14.0));
    }

    #[test]
    fn test_evaluate_formula_precedence_with_parens() {
        let registry = OracleRegistry::new();
        let context = OracleContext::new("entity-1", registry);
        assert_eq!(context.evaluate_formula("(2 + 3) * 4"), Some(20.0));
    }

    #[test]
    fn test_evaluate_formula_division_by_zero() {
        // Test the underlying evaluator directly for error detail
        let resolver = |_: &str| -> Option<f64> { None };
        let result = super::formula_eval::eval("5 / 0", &resolver);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().contains("division by zero"),
            "Expected 'division by zero' error"
        );
    }

    #[test]
    fn test_evaluate_formula_unknown_variable() {
        let resolver = |_: &str| -> Option<f64> { None };
        let result = super::formula_eval::eval("x + 1", &resolver);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().contains("unknown variable"),
            "Expected 'unknown variable' error"
        );
    }
}
