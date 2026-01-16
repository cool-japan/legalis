//! Oracle Integration for Off-Chain Facts
//!
//! This module provides oracle integration for bringing off-chain data onto
//! the blockchain for legal statute evaluation.

use crate::EvaluationContext;
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

    fn get_percentage(&self, _key: &str) -> Option<u32> {
        None // Not implemented for simplicity
    }

    fn evaluate_formula(&self, _formula: &str) -> Option<f64> {
        None // Not implemented for simplicity
    }

    fn get_current_timestamp(&self) -> Option<i64> {
        Some(current_timestamp() as i64)
    }

    fn get_current_date(&self) -> Option<chrono::NaiveDate> {
        None // Not implemented for simplicity
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
        None // Not implemented for simplicity
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
}
