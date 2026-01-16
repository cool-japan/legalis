//! Zero-Knowledge Proofs for Privacy-Preserving Evaluation
//!
//! This module provides zero-knowledge proof functionality for evaluating
//! legal statutes while preserving privacy of sensitive data.

use crate::{ComparisonOp, Condition};
use sha2::{Digest, Sha256};
use std::fmt;

/// Zero-knowledge proof system type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ZkProofSystem {
    /// zk-SNARKs (Succinct Non-Interactive Argument of Knowledge)
    Snark,
    /// zk-STARKs (Scalable Transparent Argument of Knowledge)
    Stark,
    /// Bulletproofs
    Bulletproof,
    /// Groth16
    Groth16,
    /// PLONK
    Plonk,
}

impl fmt::Display for ZkProofSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkProofSystem::Snark => write!(f, "zk-SNARK"),
            ZkProofSystem::Stark => write!(f, "zk-STARK"),
            ZkProofSystem::Bulletproof => write!(f, "Bulletproof"),
            ZkProofSystem::Groth16 => write!(f, "Groth16"),
            ZkProofSystem::Plonk => write!(f, "PLONK"),
        }
    }
}

/// A zero-knowledge proof
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZkProof {
    /// Proof system used
    pub system: ZkProofSystem,
    /// Proof data (serialized)
    pub proof_data: Vec<u8>,
    /// Public inputs
    pub public_inputs: Vec<u8>,
    /// Commitment hash
    pub commitment: String,
    /// Proof generation timestamp
    pub timestamp: u64,
}

impl ZkProof {
    /// Create a new zero-knowledge proof
    pub fn new(system: ZkProofSystem, proof_data: Vec<u8>, public_inputs: Vec<u8>) -> Self {
        let commitment = Self::compute_commitment(&proof_data, &public_inputs);

        Self {
            system,
            proof_data,
            public_inputs,
            commitment,
            timestamp: current_timestamp(),
        }
    }

    /// Compute commitment hash
    fn compute_commitment(proof_data: &[u8], public_inputs: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(proof_data);
        hasher.update(public_inputs);
        let result = hasher.finalize();
        hex_encode(&result)
    }

    /// Get proof size in bytes
    pub fn size(&self) -> usize {
        self.proof_data.len() + self.public_inputs.len()
    }

    /// Check if proof is compact (< 1KB)
    pub fn is_compact(&self) -> bool {
        self.size() < 1024
    }
}

/// Zero-knowledge circuit for condition evaluation
///
/// # Example
///
/// ```
/// use legalis_core::{Condition, ComparisonOp};
/// use legalis_core::zkp::{ZkCircuit, ZkProofSystem};
///
/// let condition = Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 };
/// let circuit = ZkCircuit::from_condition(&condition);
///
/// assert_eq!(circuit.num_constraints(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct ZkCircuit {
    /// Circuit constraints
    constraints: Vec<Constraint>,
    /// Public inputs
    public_inputs: Vec<String>,
    /// Private inputs (witness)
    private_inputs: Vec<String>,
}

impl ZkCircuit {
    /// Create a new empty circuit
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            public_inputs: Vec::new(),
            private_inputs: Vec::new(),
        }
    }

    /// Create a circuit from a condition
    pub fn from_condition(condition: &Condition) -> Self {
        let mut circuit = Self::new();
        circuit.compile_condition(condition);
        circuit
    }

    /// Add a constraint to the circuit
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Add a public input
    pub fn add_public_input(&mut self, name: String) {
        self.public_inputs.push(name);
    }

    /// Add a private input
    pub fn add_private_input(&mut self, name: String) {
        self.private_inputs.push(name);
    }

    /// Compile a condition into circuit constraints
    fn compile_condition(&mut self, condition: &Condition) {
        match condition {
            Condition::Age { operator, value } => {
                self.add_private_input("age".to_string());
                self.add_public_input(format!("threshold_{}", value));
                self.add_constraint(Constraint::Comparison {
                    left: "age".to_string(),
                    right: format!("threshold_{}", value),
                    op: *operator,
                });
            }
            Condition::Income { operator, value } => {
                self.add_private_input("income".to_string());
                self.add_public_input(format!("threshold_{}", value));
                self.add_constraint(Constraint::Comparison {
                    left: "income".to_string(),
                    right: format!("threshold_{}", value),
                    op: *operator,
                });
            }
            Condition::And(left, right) => {
                self.compile_condition(left);
                self.compile_condition(right);
                self.add_constraint(Constraint::And { operands: 2 });
            }
            Condition::Or(left, right) => {
                self.compile_condition(left);
                self.compile_condition(right);
                self.add_constraint(Constraint::Or { operands: 2 });
            }
            Condition::Not(cond) => {
                self.compile_condition(cond);
                self.add_constraint(Constraint::Not);
            }
            _ => {
                // Other condition types can be added
                self.add_constraint(Constraint::Custom {
                    description: format!("{:?}", condition),
                });
            }
        }
    }

    /// Get number of constraints
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    /// Get number of public inputs
    pub fn num_public_inputs(&self) -> usize {
        self.public_inputs.len()
    }

    /// Get number of private inputs
    pub fn num_private_inputs(&self) -> usize {
        self.private_inputs.len()
    }
}

impl Default for ZkCircuit {
    fn default() -> Self {
        Self::new()
    }
}

/// Circuit constraint types
#[derive(Debug, Clone)]
pub enum Constraint {
    /// Comparison constraint
    Comparison {
        left: String,
        right: String,
        op: ComparisonOp,
    },
    /// Logical AND
    And { operands: usize },
    /// Logical OR
    Or { operands: usize },
    /// Logical NOT
    Not,
    /// Range constraint
    Range {
        variable: String,
        min: i64,
        max: i64,
    },
    /// Custom constraint
    Custom { description: String },
}

/// Zero-knowledge prover
///
/// # Example
///
/// ```
/// use legalis_core::{Condition, ComparisonOp};
/// use legalis_core::zkp::{ZkProver, ZkCircuit, ZkProofSystem, PrivateWitness};
///
/// let condition = Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 };
/// let circuit = ZkCircuit::from_condition(&condition);
///
/// let mut witness = PrivateWitness::new();
/// witness.set("age", 25);
///
/// let prover = ZkProver::new(ZkProofSystem::Snark);
/// let proof = prover.prove(&circuit, &witness).unwrap();
///
/// assert!(proof.is_compact());
/// ```
pub struct ZkProver {
    system: ZkProofSystem,
}

impl ZkProver {
    /// Create a new ZK prover
    pub fn new(system: ZkProofSystem) -> Self {
        Self { system }
    }

    /// Generate a proof for a circuit with given witness
    pub fn prove(&self, circuit: &ZkCircuit, witness: &PrivateWitness) -> Result<ZkProof, ZkError> {
        // Simulate proof generation (in real implementation, would use actual ZK library)
        let proof_data = self.simulate_proof_generation(circuit, witness)?;
        let public_inputs = self.extract_public_inputs(circuit);

        Ok(ZkProof::new(self.system, proof_data, public_inputs))
    }

    /// Simulate proof generation (placeholder for real ZK library)
    fn simulate_proof_generation(
        &self,
        circuit: &ZkCircuit,
        witness: &PrivateWitness,
    ) -> Result<Vec<u8>, ZkError> {
        // Validate witness has all required private inputs
        for input in &circuit.private_inputs {
            if !witness.has(input) {
                return Err(ZkError::MissingWitness(input.clone()));
            }
        }

        // Simulate proof data (in reality, this would be cryptographic proof)
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", circuit.constraints).as_bytes());
        hasher.update(format!("{:?}", witness.values).as_bytes());

        Ok(hasher.finalize().to_vec())
    }

    /// Extract public inputs from circuit
    fn extract_public_inputs(&self, circuit: &ZkCircuit) -> Vec<u8> {
        circuit.public_inputs.join(",").into_bytes()
    }
}

/// Zero-knowledge verifier
///
/// # Example
///
/// ```
/// use legalis_core::{Condition, ComparisonOp};
/// use legalis_core::zkp::{ZkProver, ZkVerifier, ZkCircuit, ZkProofSystem, PrivateWitness};
///
/// let condition = Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 };
/// let circuit = ZkCircuit::from_condition(&condition);
///
/// let mut witness = PrivateWitness::new();
/// witness.set("age", 25);
///
/// let prover = ZkProver::new(ZkProofSystem::Snark);
/// let proof = prover.prove(&circuit, &witness).unwrap();
///
/// let verifier = ZkVerifier::new(ZkProofSystem::Snark);
/// assert!(verifier.verify(&circuit, &proof).unwrap());
/// ```
pub struct ZkVerifier {
    system: ZkProofSystem,
}

impl ZkVerifier {
    /// Create a new ZK verifier
    pub fn new(system: ZkProofSystem) -> Self {
        Self { system }
    }

    /// Verify a proof against a circuit
    pub fn verify(&self, circuit: &ZkCircuit, proof: &ZkProof) -> Result<bool, ZkError> {
        // Check proof system matches
        if proof.system != self.system {
            return Err(ZkError::SystemMismatch {
                expected: self.system,
                actual: proof.system,
            });
        }

        // Simulate verification (in real implementation, would use actual ZK library)
        Ok(self.simulate_verification(circuit, proof))
    }

    /// Simulate verification (placeholder for real ZK library)
    fn simulate_verification(&self, _circuit: &ZkCircuit, proof: &ZkProof) -> bool {
        // In a real implementation, this would perform cryptographic verification
        // For now, we just check that proof data is non-empty
        !proof.proof_data.is_empty()
    }
}

/// Private witness for zero-knowledge proofs
#[derive(Debug, Clone)]
pub struct PrivateWitness {
    values: std::collections::HashMap<String, i64>,
}

impl PrivateWitness {
    /// Create a new empty witness
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }

    /// Set a private value
    pub fn set(&mut self, key: impl Into<String>, value: i64) {
        self.values.insert(key.into(), value);
    }

    /// Get a private value
    pub fn get(&self, key: &str) -> Option<i64> {
        self.values.get(key).copied()
    }

    /// Check if witness has a value
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    /// Get number of values
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if witness is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Default for PrivateWitness {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Helper function for hex encoding
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Zero-knowledge proof errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum ZkError {
    #[error("Missing witness value: {0}")]
    MissingWitness(String),

    #[error("Invalid circuit: {0}")]
    InvalidCircuit(String),

    #[error("Proof system mismatch: expected {expected}, got {actual}")]
    SystemMismatch {
        expected: ZkProofSystem,
        actual: ZkProofSystem,
    },

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zkproof_creation() {
        let proof = ZkProof::new(ZkProofSystem::Snark, vec![1, 2, 3], vec![4, 5, 6]);

        assert_eq!(proof.system, ZkProofSystem::Snark);
        assert_eq!(proof.size(), 6);
        assert!(proof.is_compact());
    }

    #[test]
    fn test_circuit_from_condition() {
        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let circuit = ZkCircuit::from_condition(&condition);

        assert_eq!(circuit.num_constraints(), 1);
        assert_eq!(circuit.num_private_inputs(), 1);
        assert_eq!(circuit.num_public_inputs(), 1);
    }

    #[test]
    fn test_complex_circuit() {
        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        );

        let circuit = ZkCircuit::from_condition(&condition);

        assert!(circuit.num_constraints() >= 3); // 2 comparisons + 1 AND
    }

    #[test]
    fn test_prover_and_verifier() {
        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let circuit = ZkCircuit::from_condition(&condition);

        let mut witness = PrivateWitness::new();
        witness.set("age", 25);

        let prover = ZkProver::new(ZkProofSystem::Snark);
        let proof = prover.prove(&circuit, &witness).unwrap();

        let verifier = ZkVerifier::new(ZkProofSystem::Snark);
        assert!(verifier.verify(&circuit, &proof).unwrap());
    }

    #[test]
    fn test_missing_witness() {
        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let circuit = ZkCircuit::from_condition(&condition);

        let witness = PrivateWitness::new(); // Empty witness

        let prover = ZkProver::new(ZkProofSystem::Snark);
        let result = prover.prove(&circuit, &witness);

        assert!(result.is_err());
    }

    #[test]
    fn test_system_mismatch() {
        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let circuit = ZkCircuit::from_condition(&condition);

        let mut witness = PrivateWitness::new();
        witness.set("age", 25);

        let prover = ZkProver::new(ZkProofSystem::Snark);
        let proof = prover.prove(&circuit, &witness).unwrap();

        let verifier = ZkVerifier::new(ZkProofSystem::Stark);
        let result = verifier.verify(&circuit, &proof);

        assert!(result.is_err());
    }

    #[test]
    fn test_proof_system_display() {
        assert_eq!(ZkProofSystem::Snark.to_string(), "zk-SNARK");
        assert_eq!(ZkProofSystem::Stark.to_string(), "zk-STARK");
        assert_eq!(ZkProofSystem::Groth16.to_string(), "Groth16");
    }

    #[test]
    fn test_private_witness() {
        let mut witness = PrivateWitness::new();

        assert!(witness.is_empty());

        witness.set("age", 30);
        witness.set("income", 50000);

        assert_eq!(witness.len(), 2);
        assert!(witness.has("age"));
        assert_eq!(witness.get("age"), Some(30));
    }
}
