//! Witness signatures for external notarization.
//!
//! This module provides functionality for external parties to sign audit records,
//! providing independent verification and tamper evidence.

use crate::AuditRecord;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A witness signature on an audit record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessSignature {
    /// Unique identifier for this signature
    pub id: Uuid,
    /// Record ID being witnessed
    pub record_id: Uuid,
    /// Witness identifier
    pub witness_id: String,
    /// Witness name or organization
    pub witness_name: String,
    /// Timestamp when signature was created
    pub timestamp: DateTime<Utc>,
    /// Signature algorithm (e.g., "ed25519", "rsa2048", "ecdsa")
    pub algorithm: String,
    /// The signature bytes (base64 encoded)
    pub signature: String,
    /// Public key of the witness (base64 encoded)
    pub public_key: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl WitnessSignature {
    /// Creates a new witness signature.
    pub fn new(
        record_id: Uuid,
        witness_id: String,
        witness_name: String,
        algorithm: String,
        signature: String,
        public_key: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            record_id,
            witness_id,
            witness_name,
            timestamp: Utc::now(),
            algorithm,
            signature,
            public_key,
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata to the signature.
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Verifies this signature against a record.
    pub fn verify(&self, record: &AuditRecord) -> bool {
        if record.id != self.record_id {
            return false;
        }

        // In a real implementation, this would use cryptographic verification
        // For now, we just check that the signature and public key are not empty
        !self.signature.is_empty() && !self.public_key.is_empty()
    }
}

/// Registry of witness signatures for audit records.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WitnessRegistry {
    /// Signatures indexed by record ID
    signatures: HashMap<Uuid, Vec<WitnessSignature>>,
}

impl WitnessRegistry {
    /// Creates a new witness registry.
    pub fn new() -> Self {
        Self {
            signatures: HashMap::new(),
        }
    }

    /// Adds a witness signature to the registry.
    pub fn add_signature(&mut self, signature: WitnessSignature) {
        self.signatures
            .entry(signature.record_id)
            .or_default()
            .push(signature);
    }

    /// Gets all signatures for a specific record.
    pub fn get_signatures(&self, record_id: Uuid) -> Vec<&WitnessSignature> {
        self.signatures
            .get(&record_id)
            .map(|sigs| sigs.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all signatures from a specific witness.
    pub fn get_witness_signatures(&self, witness_id: &str) -> Vec<&WitnessSignature> {
        self.signatures
            .values()
            .flatten()
            .filter(|sig| sig.witness_id == witness_id)
            .collect()
    }

    /// Verifies all signatures for a specific record.
    pub fn verify_record(&self, record: &AuditRecord) -> bool {
        let signatures = self.get_signatures(record.id);
        if signatures.is_empty() {
            return true; // No signatures to verify
        }

        signatures.iter().all(|sig| sig.verify(record))
    }

    /// Gets the total number of signatures.
    pub fn signature_count(&self) -> usize {
        self.signatures.values().map(|v| v.len()).sum()
    }

    /// Gets the number of unique witnesses.
    pub fn witness_count(&self) -> usize {
        let mut witnesses: Vec<&str> = self
            .signatures
            .values()
            .flatten()
            .map(|sig| sig.witness_id.as_str())
            .collect();
        witnesses.sort_unstable();
        witnesses.dedup();
        witnesses.len()
    }

    /// Checks if a record has the minimum required number of signatures.
    pub fn has_quorum(&self, record_id: Uuid, min_signatures: usize) -> bool {
        self.signatures
            .get(&record_id)
            .map(|sigs| sigs.len() >= min_signatures)
            .unwrap_or(false)
    }
}

/// Witness notarization policy.
#[derive(Debug, Clone)]
pub struct NotarizationPolicy {
    /// Minimum number of required signatures
    pub min_signatures: usize,
    /// Required witnesses (empty = any witness accepted)
    pub required_witnesses: Vec<String>,
    /// Maximum age of signature (in seconds)
    pub max_signature_age: Option<i64>,
}

impl Default for NotarizationPolicy {
    fn default() -> Self {
        Self {
            min_signatures: 1,
            required_witnesses: Vec::new(),
            max_signature_age: None,
        }
    }
}

impl NotarizationPolicy {
    /// Creates a new notarization policy.
    pub fn new(min_signatures: usize) -> Self {
        Self {
            min_signatures,
            ..Default::default()
        }
    }

    /// Sets the required witnesses.
    pub fn with_required_witnesses(mut self, witnesses: Vec<String>) -> Self {
        self.required_witnesses = witnesses;
        self
    }

    /// Sets the maximum signature age in seconds.
    pub fn with_max_age(mut self, max_age_seconds: i64) -> Self {
        self.max_signature_age = Some(max_age_seconds);
        self
    }

    /// Checks if a set of signatures satisfies this policy.
    pub fn is_satisfied(&self, signatures: &[&WitnessSignature]) -> bool {
        // Check minimum count
        if signatures.len() < self.min_signatures {
            return false;
        }

        // Check required witnesses if specified
        if !self.required_witnesses.is_empty() {
            let witness_ids: Vec<&str> = signatures.iter().map(|s| s.witness_id.as_str()).collect();
            for required in &self.required_witnesses {
                if !witness_ids.contains(&required.as_str()) {
                    return false;
                }
            }
        }

        // Check signature age if specified
        if let Some(max_age) = self.max_signature_age {
            let now = Utc::now();
            for sig in signatures {
                let age = now.signed_duration_since(sig.timestamp).num_seconds();
                if age > max_age {
                    return false;
                }
            }
        }

        true
    }
}

/// Simple signature generator for testing (not cryptographically secure).
pub struct SimpleWitnessSigner {
    witness_id: String,
    witness_name: String,
}

impl SimpleWitnessSigner {
    /// Creates a new simple signer.
    pub fn new(witness_id: String, witness_name: String) -> Self {
        Self {
            witness_id,
            witness_name,
        }
    }

    /// Signs an audit record.
    pub fn sign(&self, record: &AuditRecord) -> WitnessSignature {
        use base64::Engine;

        // Simple signature: hash of record ID + witness ID
        let data = format!("{}{}", record.id, self.witness_id);
        let signature = base64::engine::general_purpose::STANDARD
            .encode(format!("sig:{}", Self::simple_hash(&data)));
        let public_key =
            base64::engine::general_purpose::STANDARD.encode(format!("pubkey:{}", self.witness_id));

        WitnessSignature::new(
            record.id,
            self.witness_id.clone(),
            self.witness_name.clone(),
            "simple".to_string(),
            signature,
            public_key,
        )
    }

    fn simple_hash(input: &str) -> String {
        let mut hash: u64 = 0;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        format!("{:x}", hash)
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
    fn test_witness_signature_creation() {
        let record = create_test_record();
        let signature = WitnessSignature::new(
            record.id,
            "witness-1".to_string(),
            "Test Witness".to_string(),
            "ed25519".to_string(),
            "signature-data".to_string(),
            "public-key-data".to_string(),
        );

        assert_eq!(signature.record_id, record.id);
        assert_eq!(signature.witness_id, "witness-1");
        assert_eq!(signature.algorithm, "ed25519");
    }

    #[test]
    fn test_witness_registry() {
        let record = create_test_record();
        let mut registry = WitnessRegistry::new();

        let sig1 = WitnessSignature::new(
            record.id,
            "witness-1".to_string(),
            "Witness 1".to_string(),
            "ed25519".to_string(),
            "sig1".to_string(),
            "key1".to_string(),
        );

        let sig2 = WitnessSignature::new(
            record.id,
            "witness-2".to_string(),
            "Witness 2".to_string(),
            "ed25519".to_string(),
            "sig2".to_string(),
            "key2".to_string(),
        );

        registry.add_signature(sig1);
        registry.add_signature(sig2);

        assert_eq!(registry.signature_count(), 2);
        assert_eq!(registry.witness_count(), 2);
        assert_eq!(registry.get_signatures(record.id).len(), 2);
    }

    #[test]
    fn test_witness_quorum() {
        let record = create_test_record();
        let mut registry = WitnessRegistry::new();

        assert!(!registry.has_quorum(record.id, 1));

        let sig = WitnessSignature::new(
            record.id,
            "witness-1".to_string(),
            "Witness 1".to_string(),
            "ed25519".to_string(),
            "sig1".to_string(),
            "key1".to_string(),
        );
        registry.add_signature(sig);

        assert!(registry.has_quorum(record.id, 1));
        assert!(!registry.has_quorum(record.id, 2));
    }

    #[test]
    fn test_notarization_policy() {
        let record = create_test_record();
        let sig1 = WitnessSignature::new(
            record.id,
            "witness-1".to_string(),
            "Witness 1".to_string(),
            "ed25519".to_string(),
            "sig1".to_string(),
            "key1".to_string(),
        );

        let sig2 = WitnessSignature::new(
            record.id,
            "witness-2".to_string(),
            "Witness 2".to_string(),
            "ed25519".to_string(),
            "sig2".to_string(),
            "key2".to_string(),
        );

        let policy = NotarizationPolicy::new(2);
        assert!(!policy.is_satisfied(&[&sig1]));
        assert!(policy.is_satisfied(&[&sig1, &sig2]));
    }

    #[test]
    fn test_required_witnesses_policy() {
        let record = create_test_record();
        let sig1 = WitnessSignature::new(
            record.id,
            "witness-1".to_string(),
            "Witness 1".to_string(),
            "ed25519".to_string(),
            "sig1".to_string(),
            "key1".to_string(),
        );

        let sig2 = WitnessSignature::new(
            record.id,
            "witness-2".to_string(),
            "Witness 2".to_string(),
            "ed25519".to_string(),
            "sig2".to_string(),
            "key2".to_string(),
        );

        let policy =
            NotarizationPolicy::new(1).with_required_witnesses(vec!["witness-1".to_string()]);

        assert!(policy.is_satisfied(&[&sig1]));
        assert!(!policy.is_satisfied(&[&sig2]));
        assert!(policy.is_satisfied(&[&sig1, &sig2]));
    }

    #[test]
    fn test_simple_witness_signer() {
        let record = create_test_record();
        let signer = SimpleWitnessSigner::new("witness-1".to_string(), "Test Witness".to_string());

        let signature = signer.sign(&record);
        assert_eq!(signature.record_id, record.id);
        assert_eq!(signature.witness_id, "witness-1");
        assert!(!signature.signature.is_empty());
        assert!(!signature.public_key.is_empty());
    }

    #[test]
    fn test_get_witness_signatures() {
        let record1 = create_test_record();
        let record2 = create_test_record();
        let mut registry = WitnessRegistry::new();

        let sig1 = WitnessSignature::new(
            record1.id,
            "witness-1".to_string(),
            "Witness 1".to_string(),
            "ed25519".to_string(),
            "sig1".to_string(),
            "key1".to_string(),
        );

        let sig2 = WitnessSignature::new(
            record2.id,
            "witness-1".to_string(),
            "Witness 1".to_string(),
            "ed25519".to_string(),
            "sig2".to_string(),
            "key2".to_string(),
        );

        registry.add_signature(sig1);
        registry.add_signature(sig2);

        let witness_sigs = registry.get_witness_signatures("witness-1");
        assert_eq!(witness_sigs.len(), 2);
    }
}
