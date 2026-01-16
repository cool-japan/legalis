//! On-Chain Statute Verification
//!
//! This module provides functionality for verifying statutes on blockchain networks,
//! including proof generation, verification, and on-chain storage.

use crate::Statute;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt;

/// Blockchain network for verification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum BlockchainNetwork {
    /// Ethereum mainnet
    EthereumMainnet,
    /// Ethereum testnet (Goerli)
    EthereumGoerli,
    /// Polygon
    Polygon,
    /// Arbitrum
    Arbitrum,
    /// Optimism
    Optimism,
    /// Binance Smart Chain
    BinanceSmartChain,
    /// Avalanche
    Avalanche,
}

impl fmt::Display for BlockchainNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockchainNetwork::EthereumMainnet => write!(f, "Ethereum Mainnet"),
            BlockchainNetwork::EthereumGoerli => write!(f, "Ethereum Goerli"),
            BlockchainNetwork::Polygon => write!(f, "Polygon"),
            BlockchainNetwork::Arbitrum => write!(f, "Arbitrum"),
            BlockchainNetwork::Optimism => write!(f, "Optimism"),
            BlockchainNetwork::BinanceSmartChain => write!(f, "Binance Smart Chain"),
            BlockchainNetwork::Avalanche => write!(f, "Avalanche"),
        }
    }
}

/// Cryptographic proof of statute authenticity
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatuteProof {
    /// The statute ID
    pub statute_id: String,
    /// Content hash (SHA-256)
    pub content_hash: String,
    /// Merkle root for the statute
    pub merkle_root: String,
    /// Timestamp of proof generation
    pub timestamp: u64,
    /// Blockchain network
    pub network: BlockchainNetwork,
    /// Transaction hash (if recorded on-chain)
    pub tx_hash: Option<String>,
    /// Block number (if recorded on-chain)
    pub block_number: Option<u64>,
}

impl StatuteProof {
    /// Create a new statute proof
    pub fn new(
        statute_id: String,
        content_hash: String,
        merkle_root: String,
        network: BlockchainNetwork,
    ) -> Self {
        Self {
            statute_id,
            content_hash,
            merkle_root,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            network,
            tx_hash: None,
            block_number: None,
        }
    }

    /// Add on-chain transaction details
    pub fn with_tx_details(mut self, tx_hash: String, block_number: u64) -> Self {
        self.tx_hash = Some(tx_hash);
        self.block_number = Some(block_number);
        self
    }

    /// Check if proof is recorded on-chain
    pub fn is_on_chain(&self) -> bool {
        self.tx_hash.is_some() && self.block_number.is_some()
    }
}

/// On-chain statute verifier
///
/// # Example
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_core::blockchain_verification::{OnChainVerifier, BlockchainNetwork};
///
/// let statute = Statute::new("statute-001", "Test Statute", Effect::new(EffectType::Grant, "Test effect"));
///
/// let verifier = OnChainVerifier::new(BlockchainNetwork::EthereumGoerli);
/// let proof = verifier.generate_proof(&statute);
///
/// assert_eq!(proof.statute_id, "statute-001");
/// assert!(proof.content_hash.len() == 64); // SHA-256 hex string
/// assert!(verifier.verify_proof(&statute, &proof));
/// ```
pub struct OnChainVerifier {
    network: BlockchainNetwork,
    proofs: HashMap<String, StatuteProof>,
}

impl OnChainVerifier {
    /// Create a new on-chain verifier
    pub fn new(network: BlockchainNetwork) -> Self {
        Self {
            network,
            proofs: HashMap::new(),
        }
    }

    /// Generate a cryptographic proof for a statute
    pub fn generate_proof(&self, statute: &Statute) -> StatuteProof {
        let content_hash = self.hash_statute(statute);
        let merkle_root = self.compute_merkle_root(std::slice::from_ref(&content_hash));

        StatuteProof::new(statute.id.clone(), content_hash, merkle_root, self.network)
    }

    /// Verify a proof against a statute
    pub fn verify_proof(&self, statute: &Statute, proof: &StatuteProof) -> bool {
        let expected_hash = self.hash_statute(statute);
        proof.content_hash == expected_hash && proof.statute_id == statute.id
    }

    /// Store a proof (simulated on-chain storage)
    pub fn store_proof(&mut self, proof: StatuteProof) -> Result<(), VerificationError> {
        if self.proofs.contains_key(&proof.statute_id) {
            return Err(VerificationError::DuplicateProof(proof.statute_id.clone()));
        }

        self.proofs.insert(proof.statute_id.clone(), proof);
        Ok(())
    }

    /// Retrieve a stored proof
    pub fn get_proof(&self, statute_id: &str) -> Option<&StatuteProof> {
        self.proofs.get(statute_id)
    }

    /// Verify a statute against stored proof
    pub fn verify_against_stored(&self, statute: &Statute) -> Result<bool, VerificationError> {
        let proof = self
            .proofs
            .get(&statute.id)
            .ok_or_else(|| VerificationError::ProofNotFound(statute.id.clone()))?;

        Ok(self.verify_proof(statute, proof))
    }

    /// Hash statute content using SHA-256
    fn hash_statute(&self, statute: &Statute) -> String {
        let mut hasher = Sha256::new();

        // Hash all statute fields
        hasher.update(statute.id.as_bytes());
        hasher.update(statute.title.as_bytes());
        hasher.update(statute.effect.description.as_bytes());

        // Hash preconditions
        for condition in &statute.preconditions {
            hasher.update(format!("{:?}", condition).as_bytes());
        }

        // Hash metadata
        if let Some(jurisdiction) = &statute.jurisdiction {
            hasher.update(jurisdiction.as_bytes());
        }
        hasher.update(statute.version.to_string().as_bytes());

        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Compute Merkle root for a set of hashes
    fn compute_merkle_root(&self, hashes: &[String]) -> String {
        if hashes.is_empty() {
            return String::new();
        }

        if hashes.len() == 1 {
            return hashes[0].clone();
        }

        let mut current_level = hashes.to_vec();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in current_level.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    chunk[0].clone()
                };

                let hash = {
                    let mut hasher = Sha256::new();
                    hasher.update(combined.as_bytes());
                    hex::encode(hasher.finalize())
                };

                next_level.push(hash);
            }

            current_level = next_level;
        }

        current_level[0].clone()
    }

    /// Get number of stored proofs
    pub fn proof_count(&self) -> usize {
        self.proofs.len()
    }

    /// Clear all stored proofs
    pub fn clear_proofs(&mut self) {
        self.proofs.clear();
    }
}

// Helper function to convert bytes to hex (since hex crate might not be available)
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

/// Batch verifier for multiple statutes
///
/// # Example
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_core::blockchain_verification::{BatchVerifier, BlockchainNetwork};
///
/// let statute1 = Statute::new("statute-001", "First Statute", Effect::new(EffectType::Grant, "Test"));
/// let statute2 = Statute::new("statute-002", "Second Statute", Effect::new(EffectType::Obligation, "Test"));
///
/// let mut verifier = BatchVerifier::new(BlockchainNetwork::Polygon);
/// verifier.add_statute(statute1);
/// verifier.add_statute(statute2);
///
/// let batch_proof = verifier.generate_batch_proof();
/// assert_eq!(batch_proof.statute_count, 2);
/// ```
pub struct BatchVerifier {
    network: BlockchainNetwork,
    statutes: Vec<Statute>,
}

impl BatchVerifier {
    /// Create a new batch verifier
    pub fn new(network: BlockchainNetwork) -> Self {
        Self {
            network,
            statutes: Vec::new(),
        }
    }

    /// Add a statute to the batch
    pub fn add_statute(&mut self, statute: Statute) {
        self.statutes.push(statute);
    }

    /// Generate a batch proof for all statutes
    pub fn generate_batch_proof(&self) -> BatchProof {
        let verifier = OnChainVerifier::new(self.network);
        let individual_proofs: Vec<_> = self
            .statutes
            .iter()
            .map(|s| verifier.generate_proof(s))
            .collect();

        let hashes: Vec<_> = individual_proofs
            .iter()
            .map(|p| p.content_hash.clone())
            .collect();

        let merkle_root = verifier.compute_merkle_root(&hashes);

        BatchProof {
            statute_count: self.statutes.len(),
            merkle_root,
            individual_proofs,
            network: self.network,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Verify all statutes in the batch
    pub fn verify_batch(&self, batch_proof: &BatchProof) -> bool {
        if batch_proof.statute_count != self.statutes.len() {
            return false;
        }

        let verifier = OnChainVerifier::new(self.network);

        self.statutes
            .iter()
            .zip(batch_proof.individual_proofs.iter())
            .all(|(statute, proof)| verifier.verify_proof(statute, proof))
    }

    /// Get number of statutes in batch
    pub fn len(&self) -> usize {
        self.statutes.len()
    }

    /// Check if batch is empty
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }
}

/// Batch proof for multiple statutes
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchProof {
    /// Number of statutes in the batch
    pub statute_count: usize,
    /// Merkle root for the entire batch
    pub merkle_root: String,
    /// Individual proofs for each statute
    pub individual_proofs: Vec<StatuteProof>,
    /// Network
    pub network: BlockchainNetwork,
    /// Timestamp
    pub timestamp: u64,
}

impl BatchProof {
    /// Get proof for a specific statute ID
    pub fn get_proof(&self, statute_id: &str) -> Option<&StatuteProof> {
        self.individual_proofs
            .iter()
            .find(|p| p.statute_id == statute_id)
    }
}

/// Verification errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum VerificationError {
    #[error("Proof not found for statute: {0}")]
    ProofNotFound(String),

    #[error("Duplicate proof for statute: {0}")]
    DuplicateProof(String),

    #[error("Invalid proof: {0}")]
    InvalidProof(String),

    #[error("Network mismatch: expected {expected}, got {actual}")]
    NetworkMismatch { expected: String, actual: String },

    #[error("Blockchain error: {0}")]
    BlockchainError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_generation() {
        let statute = Statute::new(
            "test-001",
            "Test Statute",
            crate::Effect::new(crate::EffectType::Grant, "Test"),
        );

        let verifier = OnChainVerifier::new(BlockchainNetwork::EthereumGoerli);
        let proof = verifier.generate_proof(&statute);

        assert_eq!(proof.statute_id, "test-001");
        assert_eq!(proof.content_hash.len(), 64); // SHA-256 hex = 64 chars
        assert!(!proof.is_on_chain());
    }

    #[test]
    fn test_proof_verification() {
        let statute = Statute::new(
            "test-002",
            "Test Statute 2",
            crate::Effect::new(crate::EffectType::Obligation, "Test"),
        );

        let verifier = OnChainVerifier::new(BlockchainNetwork::Polygon);
        let proof = verifier.generate_proof(&statute);

        assert!(verifier.verify_proof(&statute, &proof));
    }

    #[test]
    fn test_proof_storage() {
        let statute = Statute::new(
            "test-003",
            "Test Statute 3",
            crate::Effect::new(crate::EffectType::Grant, "Test"),
        );

        let mut verifier = OnChainVerifier::new(BlockchainNetwork::Arbitrum);
        let proof = verifier.generate_proof(&statute);

        assert!(verifier.store_proof(proof).is_ok());
        assert_eq!(verifier.proof_count(), 1);

        assert!(verifier.verify_against_stored(&statute).unwrap());
    }

    #[test]
    fn test_duplicate_proof_error() {
        let statute = Statute::new(
            "test-004",
            "Test",
            crate::Effect::new(crate::EffectType::Grant, "Test"),
        );

        let mut verifier = OnChainVerifier::new(BlockchainNetwork::Optimism);
        let proof1 = verifier.generate_proof(&statute);
        let proof2 = verifier.generate_proof(&statute);

        assert!(verifier.store_proof(proof1).is_ok());
        assert!(verifier.store_proof(proof2).is_err());
    }

    #[test]
    fn test_batch_verification() {
        let statute1 = Statute::new(
            "batch-001",
            "Batch Test 1",
            crate::Effect::new(crate::EffectType::Grant, "Test"),
        );
        let statute2 = Statute::new(
            "batch-002",
            "Batch Test 2",
            crate::Effect::new(crate::EffectType::Obligation, "Test"),
        );

        let mut verifier = BatchVerifier::new(BlockchainNetwork::BinanceSmartChain);
        verifier.add_statute(statute1);
        verifier.add_statute(statute2);

        assert_eq!(verifier.len(), 2);

        let batch_proof = verifier.generate_batch_proof();
        assert_eq!(batch_proof.statute_count, 2);

        assert!(verifier.verify_batch(&batch_proof));
    }

    #[test]
    fn test_network_display() {
        assert_eq!(
            BlockchainNetwork::EthereumMainnet.to_string(),
            "Ethereum Mainnet"
        );
        assert_eq!(BlockchainNetwork::Polygon.to_string(), "Polygon");
    }

    #[test]
    fn test_proof_with_tx_details() {
        let proof = StatuteProof::new(
            "test".to_string(),
            "hash".to_string(),
            "root".to_string(),
            BlockchainNetwork::Avalanche,
        );

        assert!(!proof.is_on_chain());

        let proof_with_tx = proof.with_tx_details("0xabc123".to_string(), 12345);
        assert!(proof_with_tx.is_on_chain());
        assert_eq!(proof_with_tx.tx_hash.unwrap(), "0xabc123");
        assert_eq!(proof_with_tx.block_number.unwrap(), 12345);
    }

    #[test]
    fn test_clear_proofs() {
        let mut verifier = OnChainVerifier::new(BlockchainNetwork::EthereumGoerli);

        let statute = Statute::new(
            "clear-test",
            "Clear Test",
            crate::Effect::new(crate::EffectType::Grant, "Test"),
        );

        let proof = verifier.generate_proof(&statute);
        verifier.store_proof(proof).unwrap();

        assert_eq!(verifier.proof_count(), 1);

        verifier.clear_proofs();
        assert_eq!(verifier.proof_count(), 0);
    }
}
