//! Multi-party verification with threshold signatures.
//!
//! This module provides threshold signature schemes for multi-party verification
//! of audit trails. Multiple parties can participate in signing and verifying
//! the integrity of the audit trail, with configurable threshold requirements.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A party that can participate in multi-party verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Party {
    /// Unique identifier for the party
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Public key for signature verification (simplified as string)
    pub public_key: String,
}

impl Party {
    /// Creates a new party.
    pub fn new(id: String, name: String, public_key: String) -> Self {
        Self {
            id,
            name,
            public_key,
        }
    }
}

/// A signature from a single party.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartySignature {
    /// The party that created this signature
    pub party_id: String,
    /// The signature data (simplified as hash)
    pub signature: String,
    /// When this signature was created
    pub signed_at: DateTime<Utc>,
}

impl PartySignature {
    /// Creates a new party signature.
    pub fn new(party_id: String, data_hash: &str, _private_key: &str) -> Self {
        // In a real implementation, this would use proper cryptographic signing
        let signature = format!("sig_{}_{}", party_id, data_hash);
        Self {
            party_id,
            signature,
            signed_at: Utc::now(),
        }
    }

    /// Verifies this signature against a party's public key.
    pub fn verify(&self, data_hash: &str, public_key: &str) -> bool {
        // In a real implementation, this would use proper cryptographic verification
        let expected_sig = format!("sig_{}_{}", self.party_id, data_hash);
        self.signature == expected_sig && !public_key.is_empty()
    }
}

/// Threshold signature configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    /// Minimum number of signatures required
    pub threshold: usize,
    /// All parties that can participate
    pub parties: Vec<Party>,
}

impl ThresholdConfig {
    /// Creates a new threshold configuration.
    pub fn new(threshold: usize, parties: Vec<Party>) -> AuditResult<Self> {
        if threshold == 0 {
            return Err(AuditError::InvalidRecord(
                "Threshold must be at least 1".to_string(),
            ));
        }
        if threshold > parties.len() {
            return Err(AuditError::InvalidRecord(
                "Threshold cannot exceed number of parties".to_string(),
            ));
        }
        Ok(Self { threshold, parties })
    }

    /// Gets a party by ID.
    pub fn get_party(&self, party_id: &str) -> Option<&Party> {
        self.parties.iter().find(|p| p.id == party_id)
    }
}

/// A multi-party signature that satisfies a threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiPartySignature {
    /// The hash of the data that was signed
    pub data_hash: String,
    /// Individual signatures from parties
    pub signatures: Vec<PartySignature>,
    /// Timestamp when threshold was met
    pub completed_at: Option<DateTime<Utc>>,
}

impl MultiPartySignature {
    /// Creates a new multi-party signature.
    pub fn new(data_hash: String) -> Self {
        Self {
            data_hash,
            signatures: Vec::new(),
            completed_at: None,
        }
    }

    /// Adds a signature from a party.
    pub fn add_signature(&mut self, signature: PartySignature, threshold: usize) {
        self.signatures.push(signature);
        if self.signatures.len() >= threshold && self.completed_at.is_none() {
            self.completed_at = Some(Utc::now());
        }
    }

    /// Verifies all signatures against the threshold config.
    pub fn verify(&self, config: &ThresholdConfig) -> AuditResult<bool> {
        if self.signatures.len() < config.threshold {
            return Ok(false);
        }

        let mut verified_parties = HashSet::new();

        for sig in &self.signatures {
            let party = config.get_party(&sig.party_id).ok_or_else(|| {
                AuditError::InvalidRecord(format!("Unknown party: {}", sig.party_id))
            })?;

            if !sig.verify(&self.data_hash, &party.public_key) {
                return Ok(false);
            }

            verified_parties.insert(&sig.party_id);
        }

        Ok(verified_parties.len() >= config.threshold)
    }

    /// Gets the list of parties that have signed.
    pub fn signed_parties(&self) -> Vec<String> {
        self.signatures.iter().map(|s| s.party_id.clone()).collect()
    }

    /// Checks if the threshold has been met.
    pub fn is_complete(&self, threshold: usize) -> bool {
        self.signatures.len() >= threshold
    }
}

/// Manager for multi-party verification of audit trails.
pub struct MultiPartyVerifier {
    config: ThresholdConfig,
    signatures: HashMap<String, MultiPartySignature>,
}

impl MultiPartyVerifier {
    /// Creates a new multi-party verifier.
    pub fn new(config: ThresholdConfig) -> Self {
        Self {
            config,
            signatures: HashMap::new(),
        }
    }

    /// Computes the hash for a set of records.
    fn compute_records_hash(records: &[AuditRecord]) -> String {
        let mut hash: u64 = 0;
        for record in records {
            for byte in record.record_hash.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
            }
        }
        format!("{:x}", hash)
    }

    /// Signs a set of audit records as a specific party.
    pub fn sign_records(
        &mut self,
        records: &[AuditRecord],
        party_id: &str,
        private_key: &str,
    ) -> AuditResult<()> {
        // Verify party exists
        self.config
            .get_party(party_id)
            .ok_or_else(|| AuditError::InvalidRecord(format!("Unknown party: {}", party_id)))?;

        let data_hash = Self::compute_records_hash(records);
        let signature = PartySignature::new(party_id.to_string(), &data_hash, private_key);

        let multi_sig = self
            .signatures
            .entry(data_hash.clone())
            .or_insert_with(|| MultiPartySignature::new(data_hash));

        multi_sig.add_signature(signature, self.config.threshold);

        Ok(())
    }

    /// Verifies that a set of records has sufficient signatures.
    pub fn verify_records(&self, records: &[AuditRecord]) -> AuditResult<bool> {
        let data_hash = Self::compute_records_hash(records);

        let multi_sig = self
            .signatures
            .get(&data_hash)
            .ok_or_else(|| AuditError::InvalidRecord("No signatures found".to_string()))?;

        multi_sig.verify(&self.config)
    }

    /// Gets the signature status for a set of records.
    pub fn get_signature_status(&self, records: &[AuditRecord]) -> Option<SignatureStatus> {
        let data_hash = Self::compute_records_hash(records);
        self.signatures.get(&data_hash).map(|sig| SignatureStatus {
            threshold: self.config.threshold,
            current_signatures: sig.signatures.len(),
            signed_parties: sig.signed_parties(),
            is_complete: sig.is_complete(self.config.threshold),
            completed_at: sig.completed_at,
        })
    }

    /// Gets all signatures.
    pub fn all_signatures(&self) -> &HashMap<String, MultiPartySignature> {
        &self.signatures
    }

    /// Exports signatures to JSON.
    pub fn export_signatures(&self) -> AuditResult<String> {
        serde_json::to_string_pretty(&self.signatures).map_err(AuditError::SerializationError)
    }

    /// Imports signatures from JSON.
    pub fn import_signatures(&mut self, json: &str) -> AuditResult<()> {
        let signatures: HashMap<String, MultiPartySignature> =
            serde_json::from_str(json).map_err(AuditError::SerializationError)?;
        self.signatures = signatures;
        Ok(())
    }
}

/// Status of a multi-party signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureStatus {
    /// Required threshold
    pub threshold: usize,
    /// Current number of signatures
    pub current_signatures: usize,
    /// Parties that have signed
    pub signed_parties: Vec<String>,
    /// Whether threshold is met
    pub is_complete: bool,
    /// When threshold was completed
    pub completed_at: Option<DateTime<Utc>>,
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
    fn test_party_creation() {
        let party = Party::new(
            "party1".to_string(),
            "Party One".to_string(),
            "pubkey1".to_string(),
        );
        assert_eq!(party.id, "party1");
        assert_eq!(party.name, "Party One");
    }

    #[test]
    fn test_threshold_config() {
        let parties = vec![
            Party::new("p1".to_string(), "Party 1".to_string(), "key1".to_string()),
            Party::new("p2".to_string(), "Party 2".to_string(), "key2".to_string()),
            Party::new("p3".to_string(), "Party 3".to_string(), "key3".to_string()),
        ];

        let config = ThresholdConfig::new(2, parties).unwrap();
        assert_eq!(config.threshold, 2);
        assert_eq!(config.parties.len(), 3);
    }

    #[test]
    fn test_threshold_config_invalid() {
        let parties = vec![Party::new(
            "p1".to_string(),
            "Party 1".to_string(),
            "key1".to_string(),
        )];

        // Threshold exceeds parties
        assert!(ThresholdConfig::new(2, parties.clone()).is_err());

        // Zero threshold
        assert!(ThresholdConfig::new(0, parties).is_err());
    }

    #[test]
    fn test_party_signature() {
        let sig = PartySignature::new("party1".to_string(), "hash123", "privkey1");
        assert_eq!(sig.party_id, "party1");
        assert!(sig.verify("hash123", "pubkey1"));
        assert!(!sig.verify("hash456", "pubkey1"));
    }

    #[test]
    fn test_multi_party_signature() {
        let mut multi_sig = MultiPartySignature::new("hash123".to_string());
        assert!(!multi_sig.is_complete(2));

        let sig1 = PartySignature::new("p1".to_string(), "hash123", "key1");
        multi_sig.add_signature(sig1, 2);
        assert!(!multi_sig.is_complete(2));

        let sig2 = PartySignature::new("p2".to_string(), "hash123", "key2");
        multi_sig.add_signature(sig2, 2);
        assert!(multi_sig.is_complete(2));
        assert!(multi_sig.completed_at.is_some());
    }

    #[test]
    fn test_multi_party_verifier() {
        let parties = vec![
            Party::new("p1".to_string(), "Party 1".to_string(), "key1".to_string()),
            Party::new("p2".to_string(), "Party 2".to_string(), "key2".to_string()),
            Party::new("p3".to_string(), "Party 3".to_string(), "key3".to_string()),
        ];

        let config = ThresholdConfig::new(2, parties).unwrap();
        let mut verifier = MultiPartyVerifier::new(config);

        let records = vec![create_test_record()];

        // First signature
        verifier.sign_records(&records, "p1", "privkey1").unwrap();
        let status = verifier.get_signature_status(&records).unwrap();
        assert_eq!(status.current_signatures, 1);
        assert!(!status.is_complete);

        // Second signature - threshold met
        verifier.sign_records(&records, "p2", "privkey2").unwrap();
        let status = verifier.get_signature_status(&records).unwrap();
        assert_eq!(status.current_signatures, 2);
        assert!(status.is_complete);

        // Verify
        assert!(verifier.verify_records(&records).unwrap());
    }

    #[test]
    fn test_signature_export_import() {
        let parties = vec![
            Party::new("p1".to_string(), "Party 1".to_string(), "key1".to_string()),
            Party::new("p2".to_string(), "Party 2".to_string(), "key2".to_string()),
        ];

        let config = ThresholdConfig::new(2, parties).unwrap();
        let mut verifier = MultiPartyVerifier::new(config.clone());

        let records = vec![create_test_record()];
        verifier.sign_records(&records, "p1", "privkey1").unwrap();
        verifier.sign_records(&records, "p2", "privkey2").unwrap();

        // Export
        let exported = verifier.export_signatures().unwrap();

        // Import into new verifier
        let mut new_verifier = MultiPartyVerifier::new(config);
        new_verifier.import_signatures(&exported).unwrap();

        // Verify imported signatures work
        assert!(new_verifier.verify_records(&records).unwrap());
    }

    #[test]
    fn test_unknown_party_signature() {
        let parties = vec![Party::new(
            "p1".to_string(),
            "Party 1".to_string(),
            "key1".to_string(),
        )];

        let config = ThresholdConfig::new(1, parties).unwrap();
        let mut verifier = MultiPartyVerifier::new(config);

        let records = vec![create_test_record()];

        // Try to sign as unknown party
        assert!(
            verifier
                .sign_records(&records, "unknown", "privkey")
                .is_err()
        );
    }
}
