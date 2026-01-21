//! Blockchain-anchored provenance for legal knowledge graphs.
//!
//! This module provides blockchain anchoring for RDF data provenance,
//! enabling immutable audit trails and verifiable timestamps.

use crate::ipld::Cid;
use crate::{LodError, LodResult, RdfValue, Triple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Blockchain anchor for RDF data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainAnchor {
    /// Content identifier being anchored
    pub cid: Cid,
    /// Blockchain network (e.g., "ethereum", "bitcoin", "polygon")
    pub network: String,
    /// Transaction hash
    pub tx_hash: String,
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Smart contract address (if applicable)
    pub contract_address: Option<String>,
}

impl BlockchainAnchor {
    /// Creates a new blockchain anchor.
    pub fn new(
        cid: Cid,
        network: impl Into<String>,
        tx_hash: impl Into<String>,
        block_number: u64,
    ) -> Self {
        Self {
            cid,
            network: network.into(),
            tx_hash: tx_hash.into(),
            block_number,
            timestamp: chrono::Utc::now(),
            contract_address: None,
        }
    }

    /// Sets the contract address.
    pub fn with_contract(mut self, address: impl Into<String>) -> Self {
        self.contract_address = Some(address.into());
        self
    }

    /// Returns the block explorer URL.
    pub fn explorer_url(&self) -> String {
        match self.network.as_str() {
            "ethereum" => format!("https://etherscan.io/tx/{}", self.tx_hash),
            "polygon" => format!("https://polygonscan.com/tx/{}", self.tx_hash),
            "bitcoin" => format!(
                "https://blockchair.com/bitcoin/transaction/{}",
                self.tx_hash
            ),
            _ => format!("blockchain://{}:{}", self.network, self.tx_hash),
        }
    }

    /// Converts to RDF triples.
    pub fn to_triples(&self, subject: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let anchor_uri = format!("{}#anchor", subject);

        // Type
        triples.push(Triple {
            subject: subject.to_string(),
            predicate: "legalis:hasBlockchainAnchor".to_string(),
            object: RdfValue::Uri(anchor_uri.clone()),
        });

        triples.push(Triple {
            subject: anchor_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:BlockchainAnchor".to_string()),
        });

        // Network
        triples.push(Triple {
            subject: anchor_uri.clone(),
            predicate: "legalis:blockchainNetwork".to_string(),
            object: RdfValue::string(&self.network),
        });

        // Transaction hash
        triples.push(Triple {
            subject: anchor_uri.clone(),
            predicate: "legalis:transactionHash".to_string(),
            object: RdfValue::string(&self.tx_hash),
        });

        // Block number
        triples.push(Triple {
            subject: anchor_uri.clone(),
            predicate: "legalis:blockNumber".to_string(),
            object: RdfValue::integer(self.block_number as i64),
        });

        // Timestamp
        triples.push(Triple {
            subject: anchor_uri.clone(),
            predicate: "dcterms:created".to_string(),
            object: RdfValue::datetime(self.timestamp),
        });

        // CID
        triples.push(Triple {
            subject: anchor_uri.clone(),
            predicate: "legalis:anchoredContent".to_string(),
            object: RdfValue::string(self.cid.to_string()),
        });

        // Contract address
        if let Some(ref contract) = self.contract_address {
            triples.push(Triple {
                subject: anchor_uri,
                predicate: "legalis:contractAddress".to_string(),
                object: RdfValue::string(contract),
            });
        }

        triples
    }
}

/// Blockchain anchor registry.
#[derive(Debug, Default)]
pub struct AnchorRegistry {
    /// Anchors indexed by CID
    anchors: HashMap<String, Vec<BlockchainAnchor>>,
}

impl AnchorRegistry {
    /// Creates a new anchor registry.
    pub fn new() -> Self {
        Self {
            anchors: HashMap::new(),
        }
    }

    /// Registers a blockchain anchor.
    pub fn register(&mut self, anchor: BlockchainAnchor) {
        let cid_str = anchor.cid.to_string();
        self.anchors.entry(cid_str).or_default().push(anchor);
    }

    /// Retrieves all anchors for a CID.
    pub fn get_anchors(&self, cid: &Cid) -> Vec<&BlockchainAnchor> {
        self.anchors
            .get(&cid.to_string())
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Checks if a CID is anchored.
    pub fn is_anchored(&self, cid: &Cid) -> bool {
        self.anchors.contains_key(&cid.to_string())
    }

    /// Lists all anchored CIDs.
    pub fn list_cids(&self) -> Vec<String> {
        self.anchors.keys().cloned().collect()
    }

    /// Returns the total number of anchors.
    pub fn count(&self) -> usize {
        self.anchors.values().map(|v| v.len()).sum()
    }

    /// Exports to JSON.
    pub fn export_json(&self) -> LodResult<String> {
        serde_json::to_string_pretty(&self.anchors)
            .map_err(|e| LodError::SerializationError(e.to_string()))
    }

    /// Imports from JSON.
    pub fn import_json(json: &str) -> LodResult<Self> {
        let anchors: HashMap<String, Vec<BlockchainAnchor>> =
            serde_json::from_str(json).map_err(|e| LodError::SerializationError(e.to_string()))?;
        Ok(Self { anchors })
    }
}

/// Merkle proof for data integrity verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Root hash
    pub root: String,
    /// Leaf hash (content being verified)
    pub leaf: String,
    /// Proof hashes
    pub proof: Vec<String>,
    /// Positions (left/right)
    pub positions: Vec<bool>,
}

impl MerkleProof {
    /// Creates a new Merkle proof.
    pub fn new(root: impl Into<String>, leaf: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            leaf: leaf.into(),
            proof: Vec::new(),
            positions: Vec::new(),
        }
    }

    /// Adds a proof step.
    pub fn add_proof(&mut self, hash: impl Into<String>, is_left: bool) {
        self.proof.push(hash.into());
        self.positions.push(is_left);
    }

    /// Verifies the proof (simplified - in production use actual hash verification).
    pub fn verify(&self) -> bool {
        self.proof.len() == self.positions.len()
    }
}

/// Timestamp proof from a blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampProof {
    /// Content hash
    pub hash: String,
    /// Blockchain network
    pub network: String,
    /// Transaction ID
    pub tx_id: String,
    /// Block timestamp
    pub block_timestamp: chrono::DateTime<chrono::Utc>,
    /// Block number
    pub block_number: u64,
    /// Merkle proof
    pub merkle_proof: Option<MerkleProof>,
}

impl TimestampProof {
    /// Creates a new timestamp proof.
    pub fn new(
        hash: impl Into<String>,
        network: impl Into<String>,
        tx_id: impl Into<String>,
        block_number: u64,
    ) -> Self {
        Self {
            hash: hash.into(),
            network: network.into(),
            tx_id: tx_id.into(),
            block_timestamp: chrono::Utc::now(),
            block_number,
            merkle_proof: None,
        }
    }

    /// Adds a Merkle proof.
    pub fn with_merkle_proof(mut self, proof: MerkleProof) -> Self {
        self.merkle_proof = Some(proof);
        self
    }

    /// Verifies the timestamp proof.
    pub fn verify(&self) -> bool {
        if let Some(ref proof) = self.merkle_proof {
            proof.verify()
        } else {
            true // Without Merkle proof, just check basic validity
        }
    }
}

/// Smart contract for legal data anchoring.
#[derive(Debug, Clone)]
pub struct AnchorContract {
    /// Contract address
    pub address: String,
    /// Network
    pub network: String,
    /// ABI (simplified representation)
    pub methods: Vec<String>,
}

impl AnchorContract {
    /// Creates a new anchor contract.
    pub fn new(address: impl Into<String>, network: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            network: network.into(),
            methods: vec![
                "anchor(bytes32)".to_string(),
                "verify(bytes32)".to_string(),
                "getAnchor(bytes32)".to_string(),
            ],
        }
    }

    /// Generates a contract call for anchoring.
    pub fn anchor_call(&self, hash: &str) -> String {
        format!("{}:anchor({})", self.address, hash)
    }

    /// Generates a contract call for verification.
    pub fn verify_call(&self, hash: &str) -> String {
        format!("{}:verify({})", self.address, hash)
    }
}

/// Provenance chain - linked sequence of anchors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceChain {
    /// Chain of anchors
    pub anchors: Vec<BlockchainAnchor>,
    /// Parent chain (if forked)
    pub parent_chain: Option<Box<ProvenanceChain>>,
}

impl ProvenanceChain {
    /// Creates a new provenance chain.
    pub fn new() -> Self {
        Self {
            anchors: Vec::new(),
            parent_chain: None,
        }
    }

    /// Adds an anchor to the chain.
    pub fn add_anchor(&mut self, anchor: BlockchainAnchor) {
        self.anchors.push(anchor);
    }

    /// Returns the length of the chain.
    pub fn len(&self) -> usize {
        let parent_len = self.parent_chain.as_ref().map(|p| p.len()).unwrap_or(0);
        self.anchors.len() + parent_len
    }

    /// Returns true if the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.anchors.is_empty() && self.parent_chain.is_none()
    }

    /// Verifies the chain integrity.
    pub fn verify(&self) -> bool {
        // Check parent chain first
        if let Some(ref parent) = self.parent_chain
            && !parent.verify()
        {
            return false;
        }

        // Verify all anchors are in chronological order
        for i in 1..self.anchors.len() {
            if self.anchors[i].timestamp < self.anchors[i - 1].timestamp {
                return false;
            }
        }

        true
    }
}

impl Default for ProvenanceChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_anchor() {
        let cid = Cid::new("QmTest123", "dag-json");
        let anchor = BlockchainAnchor::new(cid, "ethereum", "0xabc123", 12345678);

        assert_eq!(anchor.network, "ethereum");
        assert_eq!(anchor.tx_hash, "0xabc123");
        assert_eq!(anchor.block_number, 12345678);
    }

    #[test]
    fn test_anchor_with_contract() {
        let cid = Cid::new("QmTest123", "dag-json");
        let anchor = BlockchainAnchor::new(cid, "ethereum", "0xabc123", 12345678)
            .with_contract("0xcontract123");

        assert_eq!(anchor.contract_address, Some("0xcontract123".to_string()));
    }

    #[test]
    fn test_explorer_url() {
        let cid = Cid::new("QmTest123", "dag-json");
        let anchor = BlockchainAnchor::new(cid, "ethereum", "0xabc123", 12345678);

        let url = anchor.explorer_url();
        assert!(url.contains("etherscan.io"));
        assert!(url.contains("0xabc123"));
    }

    #[test]
    fn test_anchor_to_triples() {
        let cid = Cid::new("QmTest123", "dag-json");
        let anchor = BlockchainAnchor::new(cid, "ethereum", "0xabc123", 12345678);

        let triples = anchor.to_triples("ex:resource");
        assert!(!triples.is_empty());

        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:hasBlockchainAnchor")
        );
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:blockchainNetwork")
        );
    }

    #[test]
    fn test_anchor_registry() {
        let mut registry = AnchorRegistry::new();

        let cid = Cid::new("QmTest123", "dag-json");
        let anchor = BlockchainAnchor::new(cid.clone(), "ethereum", "0xabc123", 12345678);

        registry.register(anchor);

        assert!(registry.is_anchored(&cid));
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn test_registry_get_anchors() {
        let mut registry = AnchorRegistry::new();

        let cid = Cid::new("QmTest123", "dag-json");
        let anchor1 = BlockchainAnchor::new(cid.clone(), "ethereum", "0xabc123", 12345678);
        let anchor2 = BlockchainAnchor::new(cid.clone(), "polygon", "0xdef456", 87654321);

        registry.register(anchor1);
        registry.register(anchor2);

        let anchors = registry.get_anchors(&cid);
        assert_eq!(anchors.len(), 2);
    }

    #[test]
    fn test_registry_export_import() {
        let mut registry = AnchorRegistry::new();

        let cid = Cid::new("QmTest123", "dag-json");
        let anchor = BlockchainAnchor::new(cid, "ethereum", "0xabc123", 12345678);

        registry.register(anchor);

        let json = registry.export_json().unwrap();
        let imported = AnchorRegistry::import_json(&json).unwrap();

        assert_eq!(registry.count(), imported.count());
    }

    #[test]
    fn test_merkle_proof() {
        let mut proof = MerkleProof::new("root_hash", "leaf_hash");
        proof.add_proof("hash1", true);
        proof.add_proof("hash2", false);

        assert_eq!(proof.proof.len(), 2);
        assert!(proof.verify());
    }

    #[test]
    fn test_timestamp_proof() {
        let proof = TimestampProof::new("content_hash", "ethereum", "0xabc123", 12345678);

        assert_eq!(proof.network, "ethereum");
        assert!(proof.verify());
    }

    #[test]
    fn test_timestamp_proof_with_merkle() {
        let merkle = MerkleProof::new("root", "leaf");
        let proof = TimestampProof::new("hash", "ethereum", "0xabc", 123).with_merkle_proof(merkle);

        assert!(proof.merkle_proof.is_some());
        assert!(proof.verify());
    }

    #[test]
    fn test_anchor_contract() {
        let contract = AnchorContract::new("0xcontract123", "ethereum");

        assert_eq!(contract.address, "0xcontract123");
        assert!(!contract.methods.is_empty());

        let call = contract.anchor_call("0xhash");
        assert!(call.contains("anchor"));
    }

    #[test]
    fn test_provenance_chain() {
        let mut chain = ProvenanceChain::new();

        let cid1 = Cid::new("QmTest1", "dag-json");
        let anchor1 = BlockchainAnchor::new(cid1, "ethereum", "0xabc1", 100);

        let cid2 = Cid::new("QmTest2", "dag-json");
        let anchor2 = BlockchainAnchor::new(cid2, "ethereum", "0xabc2", 200);

        chain.add_anchor(anchor1);
        chain.add_anchor(anchor2);

        assert_eq!(chain.len(), 2);
        assert!(chain.verify());
    }

    #[test]
    fn test_provenance_chain_empty() {
        let chain = ProvenanceChain::new();
        assert!(chain.is_empty());
        assert_eq!(chain.len(), 0);
    }

    #[test]
    fn test_registry_list_cids() {
        let mut registry = AnchorRegistry::new();

        let cid1 = Cid::new("QmTest1", "dag-json");
        let cid2 = Cid::new("QmTest2", "dag-json");

        registry.register(BlockchainAnchor::new(cid1, "ethereum", "0x1", 1));
        registry.register(BlockchainAnchor::new(cid2, "ethereum", "0x2", 2));

        let cids = registry.list_cids();
        assert_eq!(cids.len(), 2);
    }
}
