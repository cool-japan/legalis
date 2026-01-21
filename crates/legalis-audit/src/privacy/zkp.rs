//! Zero-Knowledge Proof (ZKP) for audit trails.
//!
//! Provides cryptographic proofs that allow verification of audit properties
//! without revealing the underlying audit data. This includes:
//! - Proof of record existence without revealing content
//! - Proof of integrity without revealing records
//! - Proof of compliance without revealing specific decisions

use crate::{AuditError, AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A zero-knowledge proof for audit verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    /// Type of proof
    pub proof_type: ProofType,
    /// Proof data (commitment, challenge, response)
    pub commitment: String,
    pub challenge: String,
    pub response: String,
    /// Public parameters
    pub public_params: HashMap<String, String>,
}

/// Types of zero-knowledge proofs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProofType {
    /// Prove a record exists without revealing its content
    RecordExistence,
    /// Prove integrity of audit trail without revealing records
    IntegrityProof,
    /// Prove compliance property without revealing specific decisions
    ComplianceProof,
    /// Prove record count within a range
    CountRangeProof,
    /// Prove attribute membership without revealing value
    AttributeMembership,
}

/// Generator for zero-knowledge proofs
pub struct ZkProofGenerator {
    secret_key: Vec<u8>,
}

impl ZkProofGenerator {
    /// Create a new ZK proof generator with a secret key
    pub fn new(secret_key: Vec<u8>) -> Self {
        Self { secret_key }
    }

    /// Generate a proof of record existence
    pub fn prove_record_existence(&self, record: &AuditRecord) -> AuditResult<ZkProof> {
        // Simplified ZK proof using Fiat-Shamir heuristic
        // In production, use proper ZK libraries like bellman or arkworks

        let commitment = self.commit_to_record(record);
        let challenge = self.generate_challenge(&commitment);
        let response = self.compute_response(&challenge, record);

        let mut public_params = HashMap::new();
        public_params.insert("record_id".to_string(), record.id.to_string());
        public_params.insert("timestamp".to_string(), record.timestamp.to_rfc3339());

        Ok(ZkProof {
            proof_type: ProofType::RecordExistence,
            commitment,
            challenge,
            response,
            public_params,
        })
    }

    /// Generate a proof of audit trail integrity
    pub fn prove_integrity(&self, records: &[AuditRecord]) -> AuditResult<ZkProof> {
        let commitment = self.commit_to_trail(records);
        let challenge = self.generate_challenge(&commitment);
        let response = self.compute_trail_response(&challenge, records);

        let mut public_params = HashMap::new();
        public_params.insert("record_count".to_string(), records.len().to_string());
        if let Some(first) = records.first() {
            public_params.insert("first_timestamp".to_string(), first.timestamp.to_rfc3339());
        }
        if let Some(last) = records.last() {
            public_params.insert("last_timestamp".to_string(), last.timestamp.to_rfc3339());
        }

        Ok(ZkProof {
            proof_type: ProofType::IntegrityProof,
            commitment,
            challenge,
            response,
            public_params,
        })
    }

    /// Generate a proof of compliance (e.g., GDPR compliance)
    pub fn prove_compliance(
        &self,
        records: &[AuditRecord],
        policy_id: &str,
    ) -> AuditResult<ZkProof> {
        let commitment = self.commit_to_compliance(records, policy_id);
        let challenge = self.generate_challenge(&commitment);
        let response = self.compute_compliance_response(&challenge, records, policy_id);

        let mut public_params = HashMap::new();
        public_params.insert("policy_id".to_string(), policy_id.to_string());
        public_params.insert("verified_records".to_string(), records.len().to_string());

        Ok(ZkProof {
            proof_type: ProofType::ComplianceProof,
            commitment,
            challenge,
            response,
            public_params,
        })
    }

    /// Generate a proof that record count is within a range
    pub fn prove_count_range(
        &self,
        actual_count: usize,
        min: usize,
        max: usize,
    ) -> AuditResult<ZkProof> {
        if actual_count < min || actual_count > max {
            return Err(AuditError::InvalidRecord(
                "Count not in specified range".to_string(),
            ));
        }

        let commitment = self.commit_to_count(actual_count);
        let challenge = self.generate_challenge(&commitment);
        let response = self.compute_count_response(&challenge, actual_count);

        let mut public_params = HashMap::new();
        public_params.insert("min_count".to_string(), min.to_string());
        public_params.insert("max_count".to_string(), max.to_string());

        Ok(ZkProof {
            proof_type: ProofType::CountRangeProof,
            commitment,
            challenge,
            response,
            public_params,
        })
    }

    /// Generate a proof that an attribute belongs to a set
    pub fn prove_attribute_membership(
        &self,
        attribute_value: &str,
        allowed_values: &[String],
    ) -> AuditResult<ZkProof> {
        if !allowed_values.contains(&attribute_value.to_string()) {
            return Err(AuditError::InvalidRecord(
                "Attribute not in allowed set".to_string(),
            ));
        }

        let commitment = self.commit_to_attribute(attribute_value);
        let challenge = self.generate_challenge(&commitment);
        let response = self.compute_attribute_response(&challenge, attribute_value);

        let mut public_params = HashMap::new();
        public_params.insert("set_size".to_string(), allowed_values.len().to_string());
        // Hash of allowed values set (for public verification)
        let set_hash = self.hash_string_set(allowed_values);
        public_params.insert("set_hash".to_string(), set_hash);

        Ok(ZkProof {
            proof_type: ProofType::AttributeMembership,
            commitment,
            challenge,
            response,
            public_params,
        })
    }

    // Helper methods for ZK proof generation

    fn commit_to_record(&self, record: &AuditRecord) -> String {
        let data = format!(
            "{}{}{}",
            record.id,
            record.record_hash,
            self.secret_key.len()
        );
        self.hash_data(&data)
    }

    fn commit_to_trail(&self, records: &[AuditRecord]) -> String {
        let hashes: Vec<String> = records.iter().map(|r| r.record_hash.clone()).collect();
        let data = format!("{}{}", hashes.join(":"), self.secret_key.len());
        self.hash_data(&data)
    }

    fn commit_to_compliance(&self, records: &[AuditRecord], policy_id: &str) -> String {
        let data = format!("{}:{}:{}", policy_id, records.len(), self.secret_key.len());
        self.hash_data(&data)
    }

    fn commit_to_count(&self, count: usize) -> String {
        let data = format!("{}:{}", count, self.secret_key.len());
        self.hash_data(&data)
    }

    fn commit_to_attribute(&self, attribute: &str) -> String {
        let data = format!("{}:{}", attribute, self.secret_key.len());
        self.hash_data(&data)
    }

    fn generate_challenge(&self, commitment: &str) -> String {
        self.hash_data(&format!("{}:challenge", commitment))
    }

    fn compute_response(&self, challenge: &str, record: &AuditRecord) -> String {
        let data = format!("{}:{}:{}", challenge, record.id, self.secret_key.len());
        self.hash_data(&data)
    }

    fn compute_trail_response(&self, challenge: &str, records: &[AuditRecord]) -> String {
        let data = format!("{}:{}:{}", challenge, records.len(), self.secret_key.len());
        self.hash_data(&data)
    }

    fn compute_compliance_response(
        &self,
        challenge: &str,
        records: &[AuditRecord],
        policy_id: &str,
    ) -> String {
        let data = format!(
            "{}:{}:{}:{}",
            challenge,
            policy_id,
            records.len(),
            self.secret_key.len()
        );
        self.hash_data(&data)
    }

    fn compute_count_response(&self, challenge: &str, count: usize) -> String {
        let data = format!("{}:{}:{}", challenge, count, self.secret_key.len());
        self.hash_data(&data)
    }

    fn compute_attribute_response(&self, challenge: &str, attribute: &str) -> String {
        let data = format!("{}:{}:{}", challenge, attribute, self.secret_key.len());
        self.hash_data(&data)
    }

    fn hash_data(&self, data: &str) -> String {
        // Simple hash for demonstration
        // In production, use proper cryptographic hash (SHA-256, SHA-3)
        let mut hash: u64 = 5381;
        for byte in data.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        format!("{:016x}", hash)
    }

    fn hash_string_set(&self, strings: &[String]) -> String {
        let mut sorted = strings.to_vec();
        sorted.sort();
        self.hash_data(&sorted.join(":"))
    }
}

/// Verifier for zero-knowledge proofs
pub struct ZkProofVerifier {
    #[allow(dead_code)]
    public_key: Vec<u8>,
}

impl ZkProofVerifier {
    /// Create a new ZK proof verifier
    pub fn new(public_key: Vec<u8>) -> Self {
        Self { public_key }
    }

    /// Verify a zero-knowledge proof
    pub fn verify(&self, proof: &ZkProof) -> AuditResult<bool> {
        match proof.proof_type {
            ProofType::RecordExistence => self.verify_record_existence(proof),
            ProofType::IntegrityProof => self.verify_integrity(proof),
            ProofType::ComplianceProof => self.verify_compliance(proof),
            ProofType::CountRangeProof => self.verify_count_range(proof),
            ProofType::AttributeMembership => self.verify_attribute_membership(proof),
        }
    }

    fn verify_record_existence(&self, proof: &ZkProof) -> AuditResult<bool> {
        // Verify that the proof structure is valid
        if proof.commitment.is_empty() || proof.challenge.is_empty() || proof.response.is_empty() {
            return Ok(false);
        }

        // Verify the challenge was derived correctly
        let expected_challenge = self.recompute_challenge(&proof.commitment);
        if proof.challenge != expected_challenge {
            return Ok(false);
        }

        // Verify record_id is a valid UUID
        if let Some(id_str) = proof.public_params.get("record_id") {
            if Uuid::parse_str(id_str).is_err() {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        Ok(true)
    }

    fn verify_integrity(&self, proof: &ZkProof) -> AuditResult<bool> {
        if proof.commitment.is_empty() || proof.challenge.is_empty() || proof.response.is_empty() {
            return Ok(false);
        }

        let expected_challenge = self.recompute_challenge(&proof.commitment);
        if proof.challenge != expected_challenge {
            return Ok(false);
        }

        // Verify record count is reasonable
        if let Some(count_str) = proof.public_params.get("record_count")
            && count_str.parse::<usize>().is_err()
        {
            return Ok(false);
        }

        Ok(true)
    }

    fn verify_compliance(&self, proof: &ZkProof) -> AuditResult<bool> {
        if proof.commitment.is_empty() || proof.challenge.is_empty() || proof.response.is_empty() {
            return Ok(false);
        }

        let expected_challenge = self.recompute_challenge(&proof.commitment);
        if proof.challenge != expected_challenge {
            return Ok(false);
        }

        // Verify policy_id exists
        if !proof.public_params.contains_key("policy_id") {
            return Ok(false);
        }

        Ok(true)
    }

    fn verify_count_range(&self, proof: &ZkProof) -> AuditResult<bool> {
        if proof.commitment.is_empty() || proof.challenge.is_empty() || proof.response.is_empty() {
            return Ok(false);
        }

        let expected_challenge = self.recompute_challenge(&proof.commitment);
        if proof.challenge != expected_challenge {
            return Ok(false);
        }

        // Verify range parameters exist
        if !proof.public_params.contains_key("min_count")
            || !proof.public_params.contains_key("max_count")
        {
            return Ok(false);
        }

        Ok(true)
    }

    fn verify_attribute_membership(&self, proof: &ZkProof) -> AuditResult<bool> {
        if proof.commitment.is_empty() || proof.challenge.is_empty() || proof.response.is_empty() {
            return Ok(false);
        }

        let expected_challenge = self.recompute_challenge(&proof.commitment);
        if proof.challenge != expected_challenge {
            return Ok(false);
        }

        // Verify set parameters exist
        if !proof.public_params.contains_key("set_size")
            || !proof.public_params.contains_key("set_hash")
        {
            return Ok(false);
        }

        Ok(true)
    }

    fn recompute_challenge(&self, commitment: &str) -> String {
        let mut hash: u64 = 5381;
        let data = format!("{}:challenge", commitment);
        for byte in data.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        format!("{:016x}", hash)
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
    fn test_record_existence_proof() {
        let secret_key = vec![1, 2, 3, 4];
        let generator = ZkProofGenerator::new(secret_key.clone());
        let verifier = ZkProofVerifier::new(secret_key);

        let record = create_test_record();
        let proof = generator.prove_record_existence(&record).unwrap();

        assert_eq!(proof.proof_type, ProofType::RecordExistence);
        assert!(verifier.verify(&proof).unwrap());
    }

    #[test]
    fn test_integrity_proof() {
        let secret_key = vec![1, 2, 3, 4];
        let generator = ZkProofGenerator::new(secret_key.clone());
        let verifier = ZkProofVerifier::new(secret_key);

        let records: Vec<AuditRecord> = (0..5).map(|_| create_test_record()).collect();
        let proof = generator.prove_integrity(&records).unwrap();

        assert_eq!(proof.proof_type, ProofType::IntegrityProof);
        assert!(verifier.verify(&proof).unwrap());
        assert_eq!(proof.public_params.get("record_count").unwrap(), "5");
    }

    #[test]
    fn test_compliance_proof() {
        let secret_key = vec![1, 2, 3, 4];
        let generator = ZkProofGenerator::new(secret_key.clone());
        let verifier = ZkProofVerifier::new(secret_key);

        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();
        let proof = generator
            .prove_compliance(&records, "GDPR-Article-15")
            .unwrap();

        assert_eq!(proof.proof_type, ProofType::ComplianceProof);
        assert!(verifier.verify(&proof).unwrap());
        assert_eq!(
            proof.public_params.get("policy_id").unwrap(),
            "GDPR-Article-15"
        );
    }

    #[test]
    fn test_count_range_proof() {
        let secret_key = vec![1, 2, 3, 4];
        let generator = ZkProofGenerator::new(secret_key.clone());
        let verifier = ZkProofVerifier::new(secret_key);

        let proof = generator.prove_count_range(50, 10, 100).unwrap();

        assert_eq!(proof.proof_type, ProofType::CountRangeProof);
        assert!(verifier.verify(&proof).unwrap());
        assert_eq!(proof.public_params.get("min_count").unwrap(), "10");
        assert_eq!(proof.public_params.get("max_count").unwrap(), "100");
    }

    #[test]
    fn test_count_range_proof_out_of_range() {
        let secret_key = vec![1, 2, 3, 4];
        let generator = ZkProofGenerator::new(secret_key);

        let result = generator.prove_count_range(150, 10, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_attribute_membership_proof() {
        let secret_key = vec![1, 2, 3, 4];
        let generator = ZkProofGenerator::new(secret_key.clone());
        let verifier = ZkProofVerifier::new(secret_key);

        let allowed_values = vec![
            "approved".to_string(),
            "rejected".to_string(),
            "pending".to_string(),
        ];
        let proof = generator
            .prove_attribute_membership("approved", &allowed_values)
            .unwrap();

        assert_eq!(proof.proof_type, ProofType::AttributeMembership);
        assert!(verifier.verify(&proof).unwrap());
        assert_eq!(proof.public_params.get("set_size").unwrap(), "3");
    }

    #[test]
    fn test_attribute_membership_not_in_set() {
        let secret_key = vec![1, 2, 3, 4];
        let generator = ZkProofGenerator::new(secret_key);

        let allowed_values = vec!["approved".to_string(), "rejected".to_string()];
        let result = generator.prove_attribute_membership("invalid", &allowed_values);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_proof_verification() {
        let secret_key = vec![1, 2, 3, 4];
        let verifier = ZkProofVerifier::new(secret_key);

        let mut proof = ZkProof {
            proof_type: ProofType::RecordExistence,
            commitment: "invalid".to_string(),
            challenge: "wrong".to_string(),
            response: "bad".to_string(),
            public_params: HashMap::new(),
        };

        // Missing record_id
        assert!(!verifier.verify(&proof).unwrap());

        // Add record_id but challenge is still wrong
        proof
            .public_params
            .insert("record_id".to_string(), Uuid::new_v4().to_string());
        assert!(!verifier.verify(&proof).unwrap());
    }
}
