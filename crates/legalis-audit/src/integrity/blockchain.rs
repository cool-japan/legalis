//! Blockchain anchoring for audit trail immutability.
//!
//! This module provides functionality to anchor audit records to blockchains
//! for immutable timestamping and third-party verification.

use crate::AuditRecord;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Supported blockchain networks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockchainNetwork {
    /// Bitcoin mainnet
    Bitcoin,
    /// Bitcoin testnet
    BitcoinTestnet,
    /// Ethereum mainnet
    Ethereum,
    /// Ethereum testnet (Sepolia)
    EthereumTestnet,
    /// Custom/private blockchain
    Custom,
}

impl BlockchainNetwork {
    /// Gets the network name as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            BlockchainNetwork::Bitcoin => "bitcoin",
            BlockchainNetwork::BitcoinTestnet => "bitcoin-testnet",
            BlockchainNetwork::Ethereum => "ethereum",
            BlockchainNetwork::EthereumTestnet => "ethereum-testnet",
            BlockchainNetwork::Custom => "custom",
        }
    }
}

/// A blockchain anchor for an audit record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainAnchor {
    /// Unique identifier for this anchor
    pub id: Uuid,
    /// Record ID being anchored
    pub record_id: Uuid,
    /// Blockchain network
    pub network: BlockchainNetwork,
    /// Transaction hash on the blockchain
    pub transaction_hash: String,
    /// Block number where the transaction was included
    pub block_number: Option<u64>,
    /// Block hash where the transaction was included
    pub block_hash: Option<String>,
    /// Timestamp when the anchor was created
    pub timestamp: DateTime<Utc>,
    /// Hash of the record that was anchored
    pub record_hash: String,
    /// Merkle root if multiple records were batched
    pub merkle_root: Option<String>,
    /// Number of confirmations
    pub confirmations: u32,
    /// Transaction fee paid (in smallest unit)
    pub fee: Option<u64>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl BlockchainAnchor {
    /// Creates a new blockchain anchor.
    pub fn new(
        record_id: Uuid,
        network: BlockchainNetwork,
        transaction_hash: String,
        record_hash: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            record_id,
            network,
            transaction_hash,
            block_number: None,
            block_hash: None,
            timestamp: Utc::now(),
            record_hash,
            merkle_root: None,
            confirmations: 0,
            fee: None,
            metadata: HashMap::new(),
        }
    }

    /// Sets the block information.
    pub fn with_block(mut self, block_number: u64, block_hash: String) -> Self {
        self.block_number = Some(block_number);
        self.block_hash = Some(block_hash);
        self
    }

    /// Sets the Merkle root for batch anchoring.
    pub fn with_merkle_root(mut self, merkle_root: String) -> Self {
        self.merkle_root = Some(merkle_root);
        self
    }

    /// Sets the number of confirmations.
    pub fn with_confirmations(mut self, confirmations: u32) -> Self {
        self.confirmations = confirmations;
        self
    }

    /// Sets the transaction fee.
    pub fn with_fee(mut self, fee: u64) -> Self {
        self.fee = Some(fee);
        self
    }

    /// Adds metadata to the anchor.
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Checks if the anchor is confirmed (has at least N confirmations).
    pub fn is_confirmed(&self, min_confirmations: u32) -> bool {
        self.confirmations >= min_confirmations
    }

    /// Verifies this anchor against a record.
    pub fn verify(&self, record: &AuditRecord) -> bool {
        if record.id != self.record_id {
            return false;
        }

        // Verify the record hash matches
        self.record_hash == record.record_hash && !self.transaction_hash.is_empty()
    }

    /// Gets the blockchain explorer URL for this transaction.
    pub fn explorer_url(&self) -> String {
        match self.network {
            BlockchainNetwork::Bitcoin => {
                format!("https://blockchain.info/tx/{}", self.transaction_hash)
            }
            BlockchainNetwork::BitcoinTestnet => {
                format!(
                    "https://blockstream.info/testnet/tx/{}",
                    self.transaction_hash
                )
            }
            BlockchainNetwork::Ethereum => {
                format!("https://etherscan.io/tx/{}", self.transaction_hash)
            }
            BlockchainNetwork::EthereumTestnet => {
                format!("https://sepolia.etherscan.io/tx/{}", self.transaction_hash)
            }
            BlockchainNetwork::Custom => {
                format!("tx:{}", self.transaction_hash)
            }
        }
    }
}

/// Registry of blockchain anchors for audit records.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnchorRegistry {
    /// Anchors indexed by record ID
    anchors: HashMap<Uuid, Vec<BlockchainAnchor>>,
}

impl AnchorRegistry {
    /// Creates a new anchor registry.
    pub fn new() -> Self {
        Self {
            anchors: HashMap::new(),
        }
    }

    /// Adds a blockchain anchor to the registry.
    pub fn add_anchor(&mut self, anchor: BlockchainAnchor) {
        self.anchors
            .entry(anchor.record_id)
            .or_default()
            .push(anchor);
    }

    /// Gets all anchors for a specific record.
    pub fn get_anchors(&self, record_id: Uuid) -> Vec<&BlockchainAnchor> {
        self.anchors
            .get(&record_id)
            .map(|anchors| anchors.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all anchors for a specific network.
    pub fn get_network_anchors(&self, network: BlockchainNetwork) -> Vec<&BlockchainAnchor> {
        self.anchors
            .values()
            .flatten()
            .filter(|anchor| anchor.network == network)
            .collect()
    }

    /// Verifies all anchors for a specific record.
    pub fn verify_record(&self, record: &AuditRecord) -> bool {
        let anchors = self.get_anchors(record.id);
        if anchors.is_empty() {
            return true; // No anchors to verify
        }

        anchors.iter().all(|anchor| anchor.verify(record))
    }

    /// Gets the total number of anchors.
    pub fn anchor_count(&self) -> usize {
        self.anchors.values().map(|v| v.len()).sum()
    }

    /// Gets the number of confirmed anchors (with at least N confirmations).
    pub fn confirmed_anchor_count(&self, min_confirmations: u32) -> usize {
        self.anchors
            .values()
            .flatten()
            .filter(|a| a.is_confirmed(min_confirmations))
            .count()
    }

    /// Checks if a record has at least one confirmed anchor.
    pub fn is_record_anchored(&self, record_id: Uuid, min_confirmations: u32) -> bool {
        self.anchors
            .get(&record_id)
            .map(|anchors| anchors.iter().any(|a| a.is_confirmed(min_confirmations)))
            .unwrap_or(false)
    }
}

/// Configuration for blockchain anchoring.
#[derive(Debug, Clone)]
pub struct AnchorConfig {
    /// Blockchain network to use
    pub network: BlockchainNetwork,
    /// Batch size for anchoring multiple records
    pub batch_size: usize,
    /// Minimum confirmations required
    pub min_confirmations: u32,
    /// Maximum fee willing to pay (in smallest unit)
    pub max_fee: Option<u64>,
}

impl Default for AnchorConfig {
    fn default() -> Self {
        Self {
            network: BlockchainNetwork::Bitcoin,
            batch_size: 100,
            min_confirmations: 6,
            max_fee: None,
        }
    }
}

impl AnchorConfig {
    /// Creates a new anchor configuration.
    pub fn new(network: BlockchainNetwork) -> Self {
        Self {
            network,
            ..Default::default()
        }
    }

    /// Sets the batch size.
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    /// Sets the minimum confirmations.
    pub fn with_min_confirmations(mut self, confirmations: u32) -> Self {
        self.min_confirmations = confirmations;
        self
    }

    /// Sets the maximum fee.
    pub fn with_max_fee(mut self, fee: u64) -> Self {
        self.max_fee = Some(fee);
        self
    }
}

/// Simple blockchain anchor generator for testing (not actual blockchain interaction).
pub struct SimpleBlockchainAnchorer {
    config: AnchorConfig,
}

impl SimpleBlockchainAnchorer {
    /// Creates a new simple anchorer.
    pub fn new(config: AnchorConfig) -> Self {
        Self { config }
    }

    /// Anchors an audit record to the blockchain.
    pub fn anchor(&self, record: &AuditRecord) -> BlockchainAnchor {
        // Generate a fake transaction hash
        let tx_hash = format!(
            "{:x}",
            Self::simple_hash(&format!("{}{}", record.id, record.record_hash))
        );

        BlockchainAnchor::new(
            record.id,
            self.config.network,
            tx_hash,
            record.record_hash.clone(),
        )
        .with_confirmations(self.config.min_confirmations)
    }

    /// Batch anchors multiple records using a Merkle tree.
    pub fn anchor_batch(&self, records: &[AuditRecord]) -> Vec<BlockchainAnchor> {
        // Calculate Merkle root
        let hashes: Vec<String> = records.iter().map(|r| r.record_hash.clone()).collect();
        let merkle_root = self.calculate_merkle_root(&hashes);

        // Generate single transaction for the batch
        let tx_hash = format!("{:x}", Self::simple_hash(&merkle_root));

        // Create anchors for each record
        records
            .iter()
            .map(|record| {
                BlockchainAnchor::new(
                    record.id,
                    self.config.network,
                    tx_hash.clone(),
                    record.record_hash.clone(),
                )
                .with_merkle_root(merkle_root.clone())
                .with_confirmations(self.config.min_confirmations)
            })
            .collect()
    }

    fn calculate_merkle_root(&self, hashes: &[String]) -> String {
        if hashes.is_empty() {
            return String::new();
        }
        if hashes.len() == 1 {
            return hashes[0].clone();
        }

        // Simple Merkle root calculation
        let combined = hashes.join("");
        format!("{:x}", Self::simple_hash(&combined))
    }

    fn simple_hash(input: &str) -> u64 {
        let mut hash: u64 = 0;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
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
    fn test_blockchain_anchor_creation() {
        let record = create_test_record();
        let anchor = BlockchainAnchor::new(
            record.id,
            BlockchainNetwork::Bitcoin,
            "abc123".to_string(),
            record.record_hash.clone(),
        );

        assert_eq!(anchor.record_id, record.id);
        assert_eq!(anchor.network, BlockchainNetwork::Bitcoin);
        assert_eq!(anchor.transaction_hash, "abc123");
    }

    #[test]
    fn test_anchor_registry() {
        let record = create_test_record();
        let mut registry = AnchorRegistry::new();

        let anchor1 = BlockchainAnchor::new(
            record.id,
            BlockchainNetwork::Bitcoin,
            "tx1".to_string(),
            record.record_hash.clone(),
        );

        let anchor2 = BlockchainAnchor::new(
            record.id,
            BlockchainNetwork::Ethereum,
            "tx2".to_string(),
            record.record_hash.clone(),
        );

        registry.add_anchor(anchor1);
        registry.add_anchor(anchor2);

        assert_eq!(registry.anchor_count(), 2);
        assert_eq!(registry.get_anchors(record.id).len(), 2);
    }

    #[test]
    fn test_anchor_confirmations() {
        let record = create_test_record();
        let anchor = BlockchainAnchor::new(
            record.id,
            BlockchainNetwork::Bitcoin,
            "tx1".to_string(),
            record.record_hash.clone(),
        )
        .with_confirmations(6);

        assert!(anchor.is_confirmed(6));
        assert!(!anchor.is_confirmed(7));
    }

    #[test]
    fn test_simple_anchorer() {
        let record = create_test_record();
        let config = AnchorConfig::new(BlockchainNetwork::Bitcoin);
        let anchorer = SimpleBlockchainAnchorer::new(config);

        let anchor = anchorer.anchor(&record);
        assert_eq!(anchor.record_id, record.id);
        assert_eq!(anchor.network, BlockchainNetwork::Bitcoin);
        assert!(!anchor.transaction_hash.is_empty());
        assert!(anchor.verify(&record));
    }

    #[test]
    fn test_batch_anchoring() {
        let records = vec![create_test_record(), create_test_record()];
        let config = AnchorConfig::new(BlockchainNetwork::Ethereum);
        let anchorer = SimpleBlockchainAnchorer::new(config);

        let anchors = anchorer.anchor_batch(&records);
        assert_eq!(anchors.len(), 2);
        assert!(anchors[0].merkle_root.is_some());
        assert_eq!(anchors[0].transaction_hash, anchors[1].transaction_hash);
    }

    #[test]
    fn test_explorer_url() {
        let record = create_test_record();
        let anchor = BlockchainAnchor::new(
            record.id,
            BlockchainNetwork::Bitcoin,
            "abc123".to_string(),
            record.record_hash.clone(),
        );

        let url = anchor.explorer_url();
        assert!(url.contains("blockchain.info"));
        assert!(url.contains("abc123"));
    }

    #[test]
    fn test_network_anchors() {
        let record = create_test_record();
        let mut registry = AnchorRegistry::new();

        let anchor1 = BlockchainAnchor::new(
            record.id,
            BlockchainNetwork::Bitcoin,
            "tx1".to_string(),
            record.record_hash.clone(),
        );

        let anchor2 = BlockchainAnchor::new(
            record.id,
            BlockchainNetwork::Ethereum,
            "tx2".to_string(),
            record.record_hash.clone(),
        );

        registry.add_anchor(anchor1);
        registry.add_anchor(anchor2);

        assert_eq!(
            registry
                .get_network_anchors(BlockchainNetwork::Bitcoin)
                .len(),
            1
        );
        assert_eq!(
            registry
                .get_network_anchors(BlockchainNetwork::Ethereum)
                .len(),
            1
        );
    }

    #[test]
    fn test_confirmed_anchor_count() {
        let record = create_test_record();
        let mut registry = AnchorRegistry::new();

        let anchor = BlockchainAnchor::new(
            record.id,
            BlockchainNetwork::Bitcoin,
            "tx1".to_string(),
            record.record_hash.clone(),
        )
        .with_confirmations(10);

        registry.add_anchor(anchor);

        assert_eq!(registry.confirmed_anchor_count(6), 1);
        assert_eq!(registry.confirmed_anchor_count(11), 0);
    }

    #[test]
    fn test_blockchain_network_as_str() {
        assert_eq!(BlockchainNetwork::Bitcoin.as_str(), "bitcoin");
        assert_eq!(BlockchainNetwork::Ethereum.as_str(), "ethereum");
        assert_eq!(BlockchainNetwork::Custom.as_str(), "custom");
    }
}
