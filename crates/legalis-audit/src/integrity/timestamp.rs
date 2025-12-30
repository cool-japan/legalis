//! RFC 3161 Timestamping Authority integration.
//!
//! This module provides integration with Time Stamping Authorities (TSA)
//! to provide cryptographic proof that audit records existed at a specific time.

use crate::AuditRecord;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A timestamp token from a Time Stamping Authority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampToken {
    /// Unique identifier for this token
    pub id: Uuid,
    /// Record ID being timestamped
    pub record_id: Uuid,
    /// TSA identifier
    pub tsa_id: String,
    /// TSA name or URL
    pub tsa_name: String,
    /// Time when the token was issued
    pub timestamp: DateTime<Utc>,
    /// Hash algorithm used (e.g., "SHA-256")
    pub hash_algorithm: String,
    /// Hash of the record being timestamped
    pub record_hash: String,
    /// The timestamp token (base64 encoded)
    pub token: String,
    /// Serial number from TSA
    pub serial_number: Option<String>,
    /// Policy OID used by TSA
    pub policy_oid: Option<String>,
    /// Accuracy of the timestamp in microseconds
    pub accuracy_microseconds: Option<i64>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl TimestampToken {
    /// Creates a new timestamp token.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        record_id: Uuid,
        tsa_id: String,
        tsa_name: String,
        hash_algorithm: String,
        record_hash: String,
        token: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            record_id,
            tsa_id,
            tsa_name,
            timestamp: Utc::now(),
            hash_algorithm,
            record_hash,
            token,
            serial_number: None,
            policy_oid: None,
            accuracy_microseconds: None,
            metadata: HashMap::new(),
        }
    }

    /// Sets the serial number.
    pub fn with_serial_number(mut self, serial: String) -> Self {
        self.serial_number = Some(serial);
        self
    }

    /// Sets the policy OID.
    pub fn with_policy_oid(mut self, oid: String) -> Self {
        self.policy_oid = Some(oid);
        self
    }

    /// Sets the timestamp accuracy.
    pub fn with_accuracy(mut self, microseconds: i64) -> Self {
        self.accuracy_microseconds = Some(microseconds);
        self
    }

    /// Adds metadata to the token.
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Verifies this timestamp token against a record.
    pub fn verify(&self, record: &AuditRecord) -> bool {
        if record.id != self.record_id {
            return false;
        }

        // In a real implementation, this would verify the cryptographic signature
        // and parse the ASN.1 structure of the timestamp token
        self.record_hash == record.record_hash && !self.token.is_empty()
    }
}

/// Registry of timestamp tokens for audit records.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimestampRegistry {
    /// Tokens indexed by record ID
    tokens: HashMap<Uuid, Vec<TimestampToken>>,
}

impl TimestampRegistry {
    /// Creates a new timestamp registry.
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    /// Adds a timestamp token to the registry.
    pub fn add_token(&mut self, token: TimestampToken) {
        self.tokens.entry(token.record_id).or_default().push(token);
    }

    /// Gets all tokens for a specific record.
    pub fn get_tokens(&self, record_id: Uuid) -> Vec<&TimestampToken> {
        self.tokens
            .get(&record_id)
            .map(|tokens| tokens.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all tokens from a specific TSA.
    pub fn get_tsa_tokens(&self, tsa_id: &str) -> Vec<&TimestampToken> {
        self.tokens
            .values()
            .flatten()
            .filter(|token| token.tsa_id == tsa_id)
            .collect()
    }

    /// Verifies all tokens for a specific record.
    pub fn verify_record(&self, record: &AuditRecord) -> bool {
        let tokens = self.get_tokens(record.id);
        if tokens.is_empty() {
            return true; // No tokens to verify
        }

        tokens.iter().all(|token| token.verify(record))
    }

    /// Gets the total number of tokens.
    pub fn token_count(&self) -> usize {
        self.tokens.values().map(|v| v.len()).sum()
    }

    /// Gets the number of unique TSAs.
    pub fn tsa_count(&self) -> usize {
        let mut tsas: Vec<&str> = self
            .tokens
            .values()
            .flatten()
            .map(|token| token.tsa_id.as_str())
            .collect();
        tsas.sort_unstable();
        tsas.dedup();
        tsas.len()
    }

    /// Gets the earliest timestamp for a record.
    pub fn earliest_timestamp(&self, record_id: Uuid) -> Option<DateTime<Utc>> {
        self.tokens
            .get(&record_id)?
            .iter()
            .map(|token| token.timestamp)
            .min()
    }
}

/// Time Stamping Authority (TSA) configuration.
#[derive(Debug, Clone)]
pub struct TsaConfig {
    /// TSA identifier
    pub tsa_id: String,
    /// TSA name
    pub tsa_name: String,
    /// TSA URL
    pub tsa_url: String,
    /// Hash algorithm to use
    pub hash_algorithm: String,
    /// Policy OID (if required by TSA)
    pub policy_oid: Option<String>,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for TsaConfig {
    fn default() -> Self {
        Self {
            tsa_id: "default-tsa".to_string(),
            tsa_name: "Default TSA".to_string(),
            tsa_url: "https://tsa.example.com".to_string(),
            hash_algorithm: "SHA-256".to_string(),
            policy_oid: None,
            timeout_seconds: 30,
        }
    }
}

impl TsaConfig {
    /// Creates a new TSA configuration.
    pub fn new(tsa_id: String, tsa_name: String, tsa_url: String) -> Self {
        Self {
            tsa_id,
            tsa_name,
            tsa_url,
            ..Default::default()
        }
    }

    /// Sets the hash algorithm.
    pub fn with_hash_algorithm(mut self, algorithm: String) -> Self {
        self.hash_algorithm = algorithm;
        self
    }

    /// Sets the policy OID.
    pub fn with_policy_oid(mut self, oid: String) -> Self {
        self.policy_oid = Some(oid);
        self
    }

    /// Sets the timeout.
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
}

/// Simple timestamp token generator for testing (not cryptographically secure).
pub struct SimpleTimestampAuthority {
    config: TsaConfig,
}

impl SimpleTimestampAuthority {
    /// Creates a new simple TSA.
    pub fn new(config: TsaConfig) -> Self {
        Self { config }
    }

    /// Generates a timestamp token for an audit record.
    pub fn timestamp(&self, record: &AuditRecord) -> TimestampToken {
        use base64::Engine;

        // Simple token: base64 encoded hash + timestamp
        let data = format!(
            "{}:{}:{}",
            record.record_hash,
            Utc::now().timestamp(),
            self.config.tsa_id
        );
        let token = base64::engine::general_purpose::STANDARD.encode(&data);

        TimestampToken::new(
            record.id,
            self.config.tsa_id.clone(),
            self.config.tsa_name.clone(),
            self.config.hash_algorithm.clone(),
            record.record_hash.clone(),
            token,
        )
        .with_serial_number(Uuid::new_v4().to_string())
        .with_policy_oid(self.config.policy_oid.clone().unwrap_or_default())
        .with_accuracy(1000) // 1ms accuracy
    }

    /// Batch timestamps multiple records.
    pub fn timestamp_batch(&self, records: &[AuditRecord]) -> Vec<TimestampToken> {
        records.iter().map(|r| self.timestamp(r)).collect()
    }
}

/// Timestamp verification result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimestampVerification {
    /// Timestamp is valid
    Valid,
    /// Timestamp is invalid
    Invalid(String),
    /// No timestamp available
    NoTimestamp,
}

impl TimestampVerification {
    /// Checks if the verification succeeded.
    pub fn is_valid(&self) -> bool {
        matches!(self, TimestampVerification::Valid)
    }

    /// Checks if there's no timestamp.
    pub fn is_missing(&self) -> bool {
        matches!(self, TimestampVerification::NoTimestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn create_test_record() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-123".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_timestamp_token_creation() {
        let record = create_test_record();
        let token = TimestampToken::new(
            record.id,
            "tsa-1".to_string(),
            "Test TSA".to_string(),
            "SHA-256".to_string(),
            record.record_hash.clone(),
            "token-data".to_string(),
        );

        assert_eq!(token.record_id, record.id);
        assert_eq!(token.tsa_id, "tsa-1");
        assert_eq!(token.hash_algorithm, "SHA-256");
    }

    #[test]
    fn test_timestamp_registry() {
        let record = create_test_record();
        let mut registry = TimestampRegistry::new();

        let token1 = TimestampToken::new(
            record.id,
            "tsa-1".to_string(),
            "TSA 1".to_string(),
            "SHA-256".to_string(),
            record.record_hash.clone(),
            "token1".to_string(),
        );

        let token2 = TimestampToken::new(
            record.id,
            "tsa-2".to_string(),
            "TSA 2".to_string(),
            "SHA-256".to_string(),
            record.record_hash.clone(),
            "token2".to_string(),
        );

        registry.add_token(token1);
        registry.add_token(token2);

        assert_eq!(registry.token_count(), 2);
        assert_eq!(registry.tsa_count(), 2);
        assert_eq!(registry.get_tokens(record.id).len(), 2);
    }

    #[test]
    fn test_timestamp_verification() {
        let record = create_test_record();
        let token = TimestampToken::new(
            record.id,
            "tsa-1".to_string(),
            "Test TSA".to_string(),
            "SHA-256".to_string(),
            record.record_hash.clone(),
            "token-data".to_string(),
        );

        assert!(token.verify(&record));
    }

    #[test]
    fn test_simple_tsa() {
        let record = create_test_record();
        let config = TsaConfig::new(
            "test-tsa".to_string(),
            "Test TSA".to_string(),
            "https://tsa.test.com".to_string(),
        );
        let tsa = SimpleTimestampAuthority::new(config);

        let token = tsa.timestamp(&record);
        assert_eq!(token.record_id, record.id);
        assert_eq!(token.tsa_id, "test-tsa");
        assert!(!token.token.is_empty());
        assert!(token.verify(&record));
    }

    #[test]
    fn test_batch_timestamping() {
        let records = vec![create_test_record(), create_test_record()];
        let config = TsaConfig::default();
        let tsa = SimpleTimestampAuthority::new(config);

        let tokens = tsa.timestamp_batch(&records);
        assert_eq!(tokens.len(), 2);
        assert!(tokens[0].verify(&records[0]));
        assert!(tokens[1].verify(&records[1]));
    }

    #[test]
    fn test_earliest_timestamp() {
        let record = create_test_record();
        let mut registry = TimestampRegistry::new();

        let token = TimestampToken::new(
            record.id,
            "tsa-1".to_string(),
            "TSA 1".to_string(),
            "SHA-256".to_string(),
            record.record_hash.clone(),
            "token1".to_string(),
        );

        registry.add_token(token.clone());

        let earliest = registry.earliest_timestamp(record.id);
        assert!(earliest.is_some());
        assert_eq!(earliest.unwrap(), token.timestamp);
    }

    #[test]
    fn test_tsa_config() {
        let config = TsaConfig::new(
            "tsa-1".to_string(),
            "Test TSA".to_string(),
            "https://tsa.test.com".to_string(),
        )
        .with_hash_algorithm("SHA-512".to_string())
        .with_policy_oid("1.2.3.4".to_string())
        .with_timeout(60);

        assert_eq!(config.hash_algorithm, "SHA-512");
        assert_eq!(config.policy_oid, Some("1.2.3.4".to_string()));
        assert_eq!(config.timeout_seconds, 60);
    }

    #[test]
    fn test_timestamp_verification_enum() {
        assert!(TimestampVerification::Valid.is_valid());
        assert!(!TimestampVerification::NoTimestamp.is_valid());
        assert!(TimestampVerification::NoTimestamp.is_missing());
        assert!(!TimestampVerification::Invalid("error".to_string()).is_valid());
    }
}
