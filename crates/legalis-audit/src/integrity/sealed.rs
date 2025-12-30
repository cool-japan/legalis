//! Tamper-evident sealed audit logs.
//!
//! This module provides cryptographic sealing of audit logs to create tamper-evident
//! archives. Sealed logs cannot be modified without detection, making them suitable
//! for legal compliance and forensic analysis.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A sealed audit log that is cryptographically protected against tampering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedLog {
    /// Unique identifier for this sealed log
    pub seal_id: String,
    /// Timestamp when the log was sealed
    pub sealed_at: DateTime<Utc>,
    /// The sealed records
    pub records: Vec<AuditRecord>,
    /// Root hash of the Merkle tree for the records
    pub merkle_root: String,
    /// Digital signature of the seal
    pub seal_signature: String,
    /// Public key used for verification
    pub public_key: String,
    /// Metadata about the sealing process
    pub metadata: SealMetadata,
}

/// Metadata about a sealed log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealMetadata {
    /// Authority that sealed the log
    pub sealing_authority: String,
    /// Purpose of the seal
    pub purpose: String,
    /// Legal jurisdiction
    pub jurisdiction: Option<String>,
    /// Retention period in days
    pub retention_days: Option<u32>,
    /// Additional custom metadata
    pub custom: HashMap<String, String>,
}

impl SealMetadata {
    /// Creates new seal metadata.
    pub fn new(sealing_authority: String, purpose: String) -> Self {
        Self {
            sealing_authority,
            purpose,
            jurisdiction: None,
            retention_days: None,
            custom: HashMap::new(),
        }
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: String) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Sets the retention period.
    pub fn with_retention(mut self, retention_days: u32) -> Self {
        self.retention_days = Some(retention_days);
        self
    }

    /// Adds custom metadata.
    pub fn add_custom(mut self, key: String, value: String) -> Self {
        self.custom.insert(key, value);
        self
    }
}

impl SealedLog {
    /// Creates a new sealed log from records.
    pub fn seal(
        records: Vec<AuditRecord>,
        metadata: SealMetadata,
        private_key: &str,
    ) -> AuditResult<Self> {
        if records.is_empty() {
            return Err(AuditError::InvalidRecord(
                "Cannot seal empty record set".to_string(),
            ));
        }

        // Generate seal ID
        let seal_id = uuid::Uuid::new_v4().to_string();

        // Compute Merkle root
        let merkle_root = Self::compute_merkle_root(&records);

        // Create seal signature
        let seal_data = format!("{}{}{}", seal_id, merkle_root, metadata.sealing_authority);
        let seal_signature = Self::sign_data(&seal_data, private_key);

        // Derive public key from private key (simplified)
        let public_key = format!("pub_{}", private_key);

        Ok(Self {
            seal_id,
            sealed_at: Utc::now(),
            records,
            merkle_root,
            seal_signature,
            public_key,
            metadata,
        })
    }

    /// Verifies the integrity of the sealed log.
    pub fn verify(&self) -> AuditResult<bool> {
        // Verify Merkle root
        let computed_root = Self::compute_merkle_root(&self.records);
        if computed_root != self.merkle_root {
            return Ok(false);
        }

        // Verify seal signature
        let seal_data = format!(
            "{}{}{}",
            self.seal_id, self.merkle_root, self.metadata.sealing_authority
        );
        if !Self::verify_signature(&seal_data, &self.seal_signature, &self.public_key) {
            return Ok(false);
        }

        // Verify individual records
        for record in &self.records {
            if !record.verify() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Checks if the seal has expired based on retention policy.
    pub fn is_expired(&self) -> bool {
        if let Some(retention_days) = self.metadata.retention_days {
            let expiry_date = self.sealed_at + chrono::Duration::days(retention_days as i64);
            Utc::now() > expiry_date
        } else {
            false
        }
    }

    /// Gets the age of the seal in days.
    pub fn age_days(&self) -> i64 {
        (Utc::now() - self.sealed_at).num_days()
    }

    /// Exports the sealed log to JSON.
    pub fn to_json(&self) -> AuditResult<String> {
        serde_json::to_string_pretty(self).map_err(AuditError::SerializationError)
    }

    /// Imports a sealed log from JSON.
    pub fn from_json(json: &str) -> AuditResult<Self> {
        serde_json::from_str(json).map_err(AuditError::SerializationError)
    }

    /// Computes the Merkle root for a set of records.
    fn compute_merkle_root(records: &[AuditRecord]) -> String {
        let mut hashes: Vec<String> = records.iter().map(|r| r.record_hash.clone()).collect();

        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    chunk[0].clone()
                };
                next_level.push(Self::hash(&combined));
            }
            hashes = next_level;
        }

        hashes.into_iter().next().unwrap_or_default()
    }

    /// Signs data with a private key (simplified implementation).
    fn sign_data(data: &str, private_key: &str) -> String {
        // In production, use proper cryptographic signing
        format!("seal_sig_{}_{}", Self::hash(private_key), Self::hash(data))
    }

    /// Verifies a signature (simplified implementation).
    fn verify_signature(data: &str, signature: &str, public_key: &str) -> bool {
        // Derive expected private key from public key (simplified)
        let private_key = public_key.strip_prefix("pub_").unwrap_or("");
        let expected_sig = Self::sign_data(data, private_key);
        signature == expected_sig
    }

    /// Computes a simple hash.
    fn hash(input: &str) -> String {
        let mut hash: u64 = 0;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        format!("{:x}", hash)
    }
}

/// Manager for creating and verifying sealed logs.
pub struct SealManager {
    private_key: String,
    public_key: String,
    sealed_logs: Vec<SealedLog>,
}

impl SealManager {
    /// Creates a new seal manager with a generated key pair.
    pub fn new() -> Self {
        let private_key = Self::generate_private_key();
        let public_key = format!("pub_{}", private_key);
        Self {
            private_key,
            public_key,
            sealed_logs: Vec::new(),
        }
    }

    /// Creates a seal manager with an existing private key.
    pub fn with_private_key(private_key: String) -> Self {
        let public_key = format!("pub_{}", private_key);
        Self {
            private_key,
            public_key,
            sealed_logs: Vec::new(),
        }
    }

    /// Seals a set of audit records.
    pub fn seal_records(
        &mut self,
        records: Vec<AuditRecord>,
        metadata: SealMetadata,
    ) -> AuditResult<SealedLog> {
        let sealed = SealedLog::seal(records, metadata, &self.private_key)?;
        self.sealed_logs.push(sealed.clone());
        Ok(sealed)
    }

    /// Verifies a sealed log.
    pub fn verify_seal(&self, sealed_log: &SealedLog) -> AuditResult<bool> {
        sealed_log.verify()
    }

    /// Gets all sealed logs managed by this manager.
    pub fn get_all_seals(&self) -> &[SealedLog] {
        &self.sealed_logs
    }

    /// Gets expired seals.
    pub fn get_expired_seals(&self) -> Vec<&SealedLog> {
        self.sealed_logs.iter().filter(|s| s.is_expired()).collect()
    }

    /// Gets the public key for verification.
    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    /// Generates a private key (simplified).
    fn generate_private_key() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        let key: u64 = rng.random();
        format!("priv_{:x}", key)
    }
}

impl Default for SealManager {
    fn default() -> Self {
        Self::new()
    }
}

/// A verification report for a sealed log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealVerificationReport {
    /// The seal ID
    pub seal_id: String,
    /// Whether the seal is valid
    pub is_valid: bool,
    /// Whether the seal has expired
    pub is_expired: bool,
    /// Age of the seal in days
    pub age_days: i64,
    /// Number of records in the seal
    pub record_count: usize,
    /// Verification timestamp
    pub verified_at: DateTime<Utc>,
    /// Error message if verification failed
    pub error: Option<String>,
}

impl SealVerificationReport {
    /// Creates a verification report for a sealed log.
    pub fn verify(sealed_log: &SealedLog) -> Self {
        let is_valid = sealed_log.verify().unwrap_or(false);
        Self {
            seal_id: sealed_log.seal_id.clone(),
            is_valid,
            is_expired: sealed_log.is_expired(),
            age_days: sealed_log.age_days(),
            record_count: sealed_log.records.len(),
            verified_at: Utc::now(),
            error: if is_valid {
                None
            } else {
                Some("Seal verification failed".to_string())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;
    use uuid::Uuid;

    fn create_test_record() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_seal_metadata() {
        let metadata = SealMetadata::new("TestAuthority".to_string(), "Compliance".to_string())
            .with_jurisdiction("US".to_string())
            .with_retention(365)
            .add_custom("case_id".to_string(), "12345".to_string());

        assert_eq!(metadata.sealing_authority, "TestAuthority");
        assert_eq!(metadata.jurisdiction, Some("US".to_string()));
        assert_eq!(metadata.retention_days, Some(365));
        assert_eq!(metadata.custom.get("case_id"), Some(&"12345".to_string()));
    }

    #[test]
    fn test_sealed_log_creation() {
        let records = vec![create_test_record(), create_test_record()];
        let metadata = SealMetadata::new("TestAuth".to_string(), "Test".to_string());
        let sealed = SealedLog::seal(records, metadata, "test_private_key").unwrap();

        assert_eq!(sealed.records.len(), 2);
        assert!(!sealed.merkle_root.is_empty());
        assert!(!sealed.seal_signature.is_empty());
    }

    #[test]
    fn test_sealed_log_verification() {
        let records = vec![create_test_record(), create_test_record()];
        let metadata = SealMetadata::new("TestAuth".to_string(), "Test".to_string());
        let sealed = SealedLog::seal(records, metadata, "test_private_key").unwrap();

        assert!(sealed.verify().unwrap());
    }

    #[test]
    fn test_seal_empty_records() {
        let metadata = SealMetadata::new("TestAuth".to_string(), "Test".to_string());
        let result = SealedLog::seal(Vec::new(), metadata, "test_private_key");
        assert!(result.is_err());
    }

    #[test]
    fn test_seal_expiration() {
        let records = vec![create_test_record()];
        let metadata =
            SealMetadata::new("TestAuth".to_string(), "Test".to_string()).with_retention(1);
        let sealed = SealedLog::seal(records, metadata, "test_key").unwrap();

        // Fresh seal should not be expired
        assert!(!sealed.is_expired());
    }

    #[test]
    fn test_seal_age() {
        let records = vec![create_test_record()];
        let metadata = SealMetadata::new("TestAuth".to_string(), "Test".to_string());
        let sealed = SealedLog::seal(records, metadata, "test_key").unwrap();

        // Fresh seal should be 0 days old
        assert_eq!(sealed.age_days(), 0);
    }

    #[test]
    fn test_seal_json_roundtrip() {
        let records = vec![create_test_record()];
        let metadata = SealMetadata::new("TestAuth".to_string(), "Test".to_string());
        let sealed = SealedLog::seal(records, metadata, "test_key").unwrap();

        let json = sealed.to_json().unwrap();
        let restored = SealedLog::from_json(&json).unwrap();

        assert_eq!(sealed.seal_id, restored.seal_id);
        assert_eq!(sealed.merkle_root, restored.merkle_root);
        assert!(restored.verify().unwrap());
    }

    #[test]
    fn test_seal_manager() {
        let mut manager = SealManager::new();

        let records = vec![create_test_record(), create_test_record()];
        let metadata = SealMetadata::new("TestAuth".to_string(), "Test".to_string());

        let sealed = manager.seal_records(records, metadata).unwrap();
        assert!(manager.verify_seal(&sealed).unwrap());
        assert_eq!(manager.get_all_seals().len(), 1);
    }

    #[test]
    fn test_seal_manager_with_key() {
        let manager = SealManager::with_private_key("my_private_key".to_string());
        assert_eq!(manager.public_key(), "pub_my_private_key");
    }

    #[test]
    fn test_seal_verification_report() {
        let records = vec![create_test_record()];
        let metadata = SealMetadata::new("TestAuth".to_string(), "Test".to_string());
        let sealed = SealedLog::seal(records, metadata, "test_key").unwrap();

        let report = SealVerificationReport::verify(&sealed);
        assert!(report.is_valid);
        assert!(!report.is_expired);
        assert_eq!(report.record_count, 1);
        assert!(report.error.is_none());
    }

    #[test]
    fn test_multiple_seals() {
        let mut manager = SealManager::new();

        for i in 0..3 {
            let records = vec![create_test_record()];
            let metadata = SealMetadata::new(format!("Authority{}", i), format!("Purpose{}", i));
            manager.seal_records(records, metadata).unwrap();
        }

        assert_eq!(manager.get_all_seals().len(), 3);

        // Verify all seals
        for seal in manager.get_all_seals() {
            assert!(manager.verify_seal(seal).unwrap());
        }
    }
}
