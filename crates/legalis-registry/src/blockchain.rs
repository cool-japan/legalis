//! Blockchain Integration Module (v0.2.2)
//!
//! This module provides blockchain integration features for statute registry:
//! - Ethereum hash anchoring for immutable statute records
//! - Bitcoin timestamping for tamper-proof audit trails
//! - NFT-based statute ownership tracking
//! - Decentralized registry nodes for distributed operation
//! - Zero-knowledge proofs for privacy-preserving verification

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;

// =============================================================================
// Ethereum Hash Anchoring
// =============================================================================

/// Ethereum network types supported for hash anchoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EthereumNetwork {
    /// Ethereum mainnet
    Mainnet,
    /// Sepolia testnet
    Sepolia,
    /// Goerli testnet (deprecated but still supported)
    Goerli,
    /// Local development network
    Local,
}

impl EthereumNetwork {
    /// Returns the chain ID for this network.
    pub fn chain_id(&self) -> u64 {
        match self {
            Self::Mainnet => 1,
            Self::Sepolia => 11155111,
            Self::Goerli => 5,
            Self::Local => 1337,
        }
    }

    /// Returns the network name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Sepolia => "sepolia",
            Self::Goerli => "goerli",
            Self::Local => "local",
        }
    }
}

/// Represents an Ethereum transaction hash.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EthTxHash(pub String);

impl EthTxHash {
    /// Creates a new Ethereum transaction hash.
    pub fn new(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    /// Returns the hash as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Ethereum anchoring record for a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumAnchor {
    /// Unique ID for this anchor
    pub anchor_id: Uuid,
    /// The statute ID being anchored
    pub statute_id: String,
    /// The statute version being anchored
    pub version: u32,
    /// SHA-256 hash of the statute content
    pub content_hash: String,
    /// Ethereum network where the hash was anchored
    pub network: EthereumNetwork,
    /// Transaction hash on Ethereum
    pub tx_hash: EthTxHash,
    /// Block number where the transaction was included
    pub block_number: u64,
    /// Timestamp when the anchor was created
    pub anchored_at: DateTime<Utc>,
    /// Gas used for the transaction
    pub gas_used: u64,
    /// Smart contract address (if using a dedicated contract)
    pub contract_address: Option<String>,
}

impl EthereumAnchor {
    /// Creates a new Ethereum anchor.
    pub fn new(
        statute_id: impl Into<String>,
        version: u32,
        content_hash: impl Into<String>,
        network: EthereumNetwork,
        tx_hash: EthTxHash,
        block_number: u64,
    ) -> Self {
        Self {
            anchor_id: Uuid::new_v4(),
            statute_id: statute_id.into(),
            version,
            content_hash: content_hash.into(),
            network,
            tx_hash,
            block_number,
            anchored_at: Utc::now(),
            gas_used: 0,
            contract_address: None,
        }
    }

    /// Sets the gas used for this anchor.
    pub fn with_gas_used(mut self, gas_used: u64) -> Self {
        self.gas_used = gas_used;
        self
    }

    /// Sets the contract address for this anchor.
    pub fn with_contract_address(mut self, address: impl Into<String>) -> Self {
        self.contract_address = Some(address.into());
        self
    }

    /// Returns the explorer URL for this transaction.
    pub fn explorer_url(&self) -> String {
        let base_url = match self.network {
            EthereumNetwork::Mainnet => "https://etherscan.io/tx/",
            EthereumNetwork::Sepolia => "https://sepolia.etherscan.io/tx/",
            EthereumNetwork::Goerli => "https://goerli.etherscan.io/tx/",
            EthereumNetwork::Local => "http://localhost:8545/tx/",
        };
        format!("{}{}", base_url, self.tx_hash.as_str())
    }
}

/// Manager for Ethereum hash anchoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumAnchorManager {
    /// All anchors stored by statute ID and version
    anchors: HashMap<String, HashMap<u32, EthereumAnchor>>,
    /// Default network to use for new anchors
    default_network: EthereumNetwork,
}

impl EthereumAnchorManager {
    /// Creates a new Ethereum anchor manager.
    pub fn new(default_network: EthereumNetwork) -> Self {
        Self {
            anchors: HashMap::new(),
            default_network,
        }
    }

    /// Adds an anchor to the manager.
    pub fn add_anchor(&mut self, anchor: EthereumAnchor) {
        self.anchors
            .entry(anchor.statute_id.clone())
            .or_default()
            .insert(anchor.version, anchor);
    }

    /// Gets an anchor for a specific statute version.
    pub fn get_anchor(&self, statute_id: &str, version: u32) -> Option<&EthereumAnchor> {
        self.anchors
            .get(statute_id)
            .and_then(|versions| versions.get(&version))
    }

    /// Gets all anchors for a statute.
    pub fn get_all_anchors(&self, statute_id: &str) -> Vec<&EthereumAnchor> {
        self.anchors
            .get(statute_id)
            .map(|versions| versions.values().collect())
            .unwrap_or_default()
    }

    /// Returns the total number of anchors.
    pub fn total_anchors(&self) -> usize {
        self.anchors.values().map(|v| v.len()).sum()
    }

    /// Returns the default network.
    pub fn default_network(&self) -> EthereumNetwork {
        self.default_network
    }

    /// Verifies that a statute's hash matches its anchor.
    pub fn verify_anchor(&self, statute_id: &str, version: u32, content_hash: &str) -> bool {
        self.get_anchor(statute_id, version)
            .map(|anchor| anchor.content_hash == content_hash)
            .unwrap_or(false)
    }
}

impl Default for EthereumAnchorManager {
    fn default() -> Self {
        Self::new(EthereumNetwork::Mainnet)
    }
}

// =============================================================================
// Bitcoin Timestamping
// =============================================================================

/// Bitcoin network types supported for timestamping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BitcoinNetwork {
    /// Bitcoin mainnet
    Mainnet,
    /// Bitcoin testnet
    Testnet,
    /// Bitcoin regtest (regression test mode)
    Regtest,
}

impl BitcoinNetwork {
    /// Returns the network name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Testnet => "testnet",
            Self::Regtest => "regtest",
        }
    }
}

/// Represents a Bitcoin transaction ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BitcoinTxId(pub String);

impl BitcoinTxId {
    /// Creates a new Bitcoin transaction ID.
    pub fn new(txid: impl Into<String>) -> Self {
        Self(txid.into())
    }

    /// Returns the transaction ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Bitcoin timestamp record for audit trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinTimestamp {
    /// Unique ID for this timestamp
    pub timestamp_id: Uuid,
    /// The statute ID being timestamped
    pub statute_id: String,
    /// The statute version being timestamped
    pub version: u32,
    /// SHA-256 hash of the audit event
    pub event_hash: String,
    /// Bitcoin network where the timestamp was created
    pub network: BitcoinNetwork,
    /// Transaction ID on Bitcoin
    pub txid: BitcoinTxId,
    /// Block height where the transaction was included
    pub block_height: u64,
    /// Block hash
    pub block_hash: String,
    /// Timestamp from the Bitcoin block header
    pub block_time: DateTime<Utc>,
    /// When the timestamp was created
    pub created_at: DateTime<Utc>,
    /// OpenTimestamps proof (if available)
    pub ots_proof: Option<Vec<u8>>,
}

impl BitcoinTimestamp {
    /// Creates a new Bitcoin timestamp.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        statute_id: impl Into<String>,
        version: u32,
        event_hash: impl Into<String>,
        network: BitcoinNetwork,
        txid: BitcoinTxId,
        block_height: u64,
        block_hash: impl Into<String>,
        block_time: DateTime<Utc>,
    ) -> Self {
        Self {
            timestamp_id: Uuid::new_v4(),
            statute_id: statute_id.into(),
            version,
            event_hash: event_hash.into(),
            network,
            txid,
            block_height,
            block_hash: block_hash.into(),
            block_time,
            created_at: Utc::now(),
            ots_proof: None,
        }
    }

    /// Sets the OpenTimestamps proof.
    pub fn with_ots_proof(mut self, proof: Vec<u8>) -> Self {
        self.ots_proof = Some(proof);
        self
    }

    /// Returns the explorer URL for this transaction.
    pub fn explorer_url(&self) -> String {
        let base_url = match self.network {
            BitcoinNetwork::Mainnet => "https://blockstream.info/tx/",
            BitcoinNetwork::Testnet => "https://blockstream.info/testnet/tx/",
            BitcoinNetwork::Regtest => "http://localhost:3000/tx/",
        };
        format!("{}{}", base_url, self.txid.as_str())
    }

    /// Checks if this timestamp has an OpenTimestamps proof.
    pub fn has_ots_proof(&self) -> bool {
        self.ots_proof.is_some()
    }
}

/// Manager for Bitcoin timestamping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinTimestampManager {
    /// All timestamps stored by statute ID and version
    timestamps: HashMap<String, HashMap<u32, Vec<BitcoinTimestamp>>>,
    /// Default network to use for new timestamps
    default_network: BitcoinNetwork,
}

impl BitcoinTimestampManager {
    /// Creates a new Bitcoin timestamp manager.
    pub fn new(default_network: BitcoinNetwork) -> Self {
        Self {
            timestamps: HashMap::new(),
            default_network,
        }
    }

    /// Adds a timestamp to the manager.
    pub fn add_timestamp(&mut self, timestamp: BitcoinTimestamp) {
        self.timestamps
            .entry(timestamp.statute_id.clone())
            .or_default()
            .entry(timestamp.version)
            .or_default()
            .push(timestamp);
    }

    /// Gets all timestamps for a specific statute version.
    pub fn get_timestamps(&self, statute_id: &str, version: u32) -> Vec<&BitcoinTimestamp> {
        self.timestamps
            .get(statute_id)
            .and_then(|versions| versions.get(&version))
            .map(|ts| ts.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all timestamps for all versions of a statute.
    pub fn get_all_timestamps(&self, statute_id: &str) -> Vec<&BitcoinTimestamp> {
        self.timestamps
            .get(statute_id)
            .map(|versions| versions.values().flatten().collect())
            .unwrap_or_default()
    }

    /// Returns the total number of timestamps.
    pub fn total_timestamps(&self) -> usize {
        self.timestamps
            .values()
            .flat_map(|v| v.values())
            .map(|t| t.len())
            .sum()
    }

    /// Returns the default network.
    pub fn default_network(&self) -> BitcoinNetwork {
        self.default_network
    }

    /// Gets the earliest timestamp for a statute version.
    pub fn earliest_timestamp(&self, statute_id: &str, version: u32) -> Option<&BitcoinTimestamp> {
        self.get_timestamps(statute_id, version)
            .into_iter()
            .min_by_key(|ts| ts.block_time)
    }

    /// Gets the latest timestamp for a statute version.
    pub fn latest_timestamp(&self, statute_id: &str, version: u32) -> Option<&BitcoinTimestamp> {
        self.get_timestamps(statute_id, version)
            .into_iter()
            .max_by_key(|ts| ts.block_time)
    }
}

impl Default for BitcoinTimestampManager {
    fn default() -> Self {
        Self::new(BitcoinNetwork::Mainnet)
    }
}

// =============================================================================
// NFT-based Statute Ownership
// =============================================================================

/// NFT standard used for statute ownership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NftStandard {
    /// ERC-721 (Ethereum non-fungible token)
    Erc721,
    /// ERC-1155 (Ethereum multi-token)
    Erc1155,
}

impl NftStandard {
    /// Returns the standard name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Erc721 => "ERC-721",
            Self::Erc1155 => "ERC-1155",
        }
    }
}

/// Represents an NFT contract address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NftContractAddress(pub String);

impl NftContractAddress {
    /// Creates a new NFT contract address.
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }

    /// Returns the address as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Represents an Ethereum wallet address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WalletAddress(pub String);

impl WalletAddress {
    /// Creates a new wallet address.
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }

    /// Returns the address as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// NFT ownership record for a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteNft {
    /// Unique NFT ID
    pub nft_id: Uuid,
    /// The statute ID this NFT represents
    pub statute_id: String,
    /// The statute version (if version-specific)
    pub version: Option<u32>,
    /// NFT standard used
    pub standard: NftStandard,
    /// Smart contract address
    pub contract_address: NftContractAddress,
    /// Token ID within the contract
    pub token_id: String,
    /// Current owner's wallet address
    pub owner: WalletAddress,
    /// Ethereum network where the NFT exists
    pub network: EthereumNetwork,
    /// Metadata URI (IPFS or HTTP)
    pub metadata_uri: String,
    /// When the NFT was minted
    pub minted_at: DateTime<Utc>,
    /// Transaction hash of the mint transaction
    pub mint_tx_hash: EthTxHash,
    /// Transfer history
    pub transfers: Vec<NftTransfer>,
}

impl StatuteNft {
    /// Creates a new statute NFT.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        statute_id: impl Into<String>,
        version: Option<u32>,
        standard: NftStandard,
        contract_address: NftContractAddress,
        token_id: impl Into<String>,
        owner: WalletAddress,
        network: EthereumNetwork,
        metadata_uri: impl Into<String>,
        mint_tx_hash: EthTxHash,
    ) -> Self {
        Self {
            nft_id: Uuid::new_v4(),
            statute_id: statute_id.into(),
            version,
            standard,
            contract_address,
            token_id: token_id.into(),
            owner,
            network,
            metadata_uri: metadata_uri.into(),
            minted_at: Utc::now(),
            mint_tx_hash,
            transfers: Vec::new(),
        }
    }

    /// Records a transfer of this NFT.
    pub fn add_transfer(&mut self, from: WalletAddress, to: WalletAddress, tx_hash: EthTxHash) {
        let transfer = NftTransfer {
            from,
            to: to.clone(),
            tx_hash,
            transferred_at: Utc::now(),
        };
        self.transfers.push(transfer);
        self.owner = to;
    }

    /// Returns the number of times this NFT has been transferred.
    pub fn transfer_count(&self) -> usize {
        self.transfers.len()
    }

    /// Returns the OpenSea URL for this NFT (mainnet only).
    pub fn opensea_url(&self) -> Option<String> {
        if self.network == EthereumNetwork::Mainnet {
            Some(format!(
                "https://opensea.io/assets/{}/{}",
                self.contract_address.as_str(),
                self.token_id
            ))
        } else {
            None
        }
    }
}

/// NFT transfer record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftTransfer {
    /// Previous owner
    pub from: WalletAddress,
    /// New owner
    pub to: WalletAddress,
    /// Transaction hash
    pub tx_hash: EthTxHash,
    /// When the transfer occurred
    pub transferred_at: DateTime<Utc>,
}

/// Manager for NFT-based statute ownership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftOwnershipManager {
    /// All NFTs stored by statute ID
    nfts: HashMap<String, Vec<StatuteNft>>,
    /// NFTs indexed by token ID for quick lookup
    token_index: HashMap<String, Uuid>,
}

impl NftOwnershipManager {
    /// Creates a new NFT ownership manager.
    pub fn new() -> Self {
        Self {
            nfts: HashMap::new(),
            token_index: HashMap::new(),
        }
    }

    /// Adds an NFT to the manager.
    pub fn add_nft(&mut self, nft: StatuteNft) {
        self.token_index.insert(nft.token_id.clone(), nft.nft_id);
        self.nfts
            .entry(nft.statute_id.clone())
            .or_default()
            .push(nft);
    }

    /// Gets an NFT by its token ID.
    pub fn get_nft_by_token_id(&self, token_id: &str) -> Option<&StatuteNft> {
        let nft_id = self.token_index.get(token_id)?;
        self.nfts
            .values()
            .flatten()
            .find(|nft| nft.nft_id == *nft_id)
    }

    /// Gets an NFT by its token ID (mutable).
    pub fn get_nft_by_token_id_mut(&mut self, token_id: &str) -> Option<&mut StatuteNft> {
        let nft_id = *self.token_index.get(token_id)?;
        self.nfts
            .values_mut()
            .flatten()
            .find(|nft| nft.nft_id == nft_id)
    }

    /// Gets all NFTs for a statute.
    pub fn get_nfts_for_statute(&self, statute_id: &str) -> Vec<&StatuteNft> {
        self.nfts
            .get(statute_id)
            .map(|nfts| nfts.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all NFTs owned by a wallet address.
    pub fn get_nfts_by_owner(&self, owner: &WalletAddress) -> Vec<&StatuteNft> {
        self.nfts
            .values()
            .flatten()
            .filter(|nft| nft.owner == *owner)
            .collect()
    }

    /// Returns the total number of NFTs.
    pub fn total_nfts(&self) -> usize {
        self.nfts.values().map(|v| v.len()).sum()
    }

    /// Records a transfer for an NFT.
    pub fn transfer_nft(
        &mut self,
        token_id: &str,
        from: WalletAddress,
        to: WalletAddress,
        tx_hash: EthTxHash,
    ) -> bool {
        if let Some(nft) = self.get_nft_by_token_id_mut(token_id) {
            nft.add_transfer(from, to, tx_hash);
            true
        } else {
            false
        }
    }
}

impl Default for NftOwnershipManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Decentralized Registry Nodes
// =============================================================================

/// Node type in the decentralized registry network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    /// Full node with complete registry data
    Full,
    /// Light node with partial data
    Light,
    /// Validator node participating in consensus
    Validator,
    /// Archive node with full history
    Archive,
}

impl NodeType {
    /// Returns the node type name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Light => "light",
            Self::Validator => "validator",
            Self::Archive => "archive",
        }
    }
}

/// Represents a node in the decentralized registry network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryNode {
    /// Unique node ID
    pub node_id: Uuid,
    /// Node type
    pub node_type: NodeType,
    /// Node public key for verification
    pub public_key: String,
    /// Node's network address (IP:port or domain)
    pub address: String,
    /// Node's reputation score (0.0 to 1.0)
    pub reputation: f64,
    /// When the node was first seen
    pub joined_at: DateTime<Utc>,
    /// Last time the node was active
    pub last_seen: DateTime<Utc>,
    /// Whether the node is currently online
    pub is_online: bool,
    /// Total number of statutes synced
    pub statutes_count: usize,
    /// Geographic region (for routing optimization)
    pub region: Option<String>,
}

impl RegistryNode {
    /// Creates a new registry node.
    pub fn new(
        node_type: NodeType,
        public_key: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            node_id: Uuid::new_v4(),
            node_type,
            public_key: public_key.into(),
            address: address.into(),
            reputation: 1.0,
            joined_at: now,
            last_seen: now,
            is_online: true,
            statutes_count: 0,
            region: None,
        }
    }

    /// Updates the last seen timestamp.
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
        self.is_online = true;
    }

    /// Marks the node as offline.
    pub fn mark_offline(&mut self) {
        self.is_online = false;
    }

    /// Updates the reputation score.
    pub fn update_reputation(&mut self, score: f64) {
        self.reputation = score.clamp(0.0, 1.0);
    }

    /// Sets the geographic region.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Checks if the node is a validator.
    pub fn is_validator(&self) -> bool {
        self.node_type == NodeType::Validator
    }

    /// Checks if the node is trusted (reputation > 0.8).
    pub fn is_trusted(&self) -> bool {
        self.reputation > 0.8
    }
}

/// Manager for decentralized registry nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizedNodeManager {
    /// All nodes in the network
    nodes: HashMap<Uuid, RegistryNode>,
    /// Nodes indexed by address for quick lookup
    address_index: HashMap<String, Uuid>,
    /// This node's ID (if running as a node)
    local_node_id: Option<Uuid>,
}

impl DecentralizedNodeManager {
    /// Creates a new decentralized node manager.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            address_index: HashMap::new(),
            local_node_id: None,
        }
    }

    /// Adds a node to the network.
    pub fn add_node(&mut self, node: RegistryNode) {
        self.address_index
            .insert(node.address.clone(), node.node_id);
        self.nodes.insert(node.node_id, node);
    }

    /// Gets a node by its ID.
    pub fn get_node(&self, node_id: &Uuid) -> Option<&RegistryNode> {
        self.nodes.get(node_id)
    }

    /// Gets a node by its ID (mutable).
    pub fn get_node_mut(&mut self, node_id: &Uuid) -> Option<&mut RegistryNode> {
        self.nodes.get_mut(node_id)
    }

    /// Gets a node by its address.
    pub fn get_node_by_address(&self, address: &str) -> Option<&RegistryNode> {
        let node_id = self.address_index.get(address)?;
        self.nodes.get(node_id)
    }

    /// Gets all nodes of a specific type.
    pub fn get_nodes_by_type(&self, node_type: NodeType) -> Vec<&RegistryNode> {
        self.nodes
            .values()
            .filter(|n| n.node_type == node_type)
            .collect()
    }

    /// Gets all online nodes.
    pub fn get_online_nodes(&self) -> Vec<&RegistryNode> {
        self.nodes.values().filter(|n| n.is_online).collect()
    }

    /// Gets all trusted nodes.
    pub fn get_trusted_nodes(&self) -> Vec<&RegistryNode> {
        self.nodes.values().filter(|n| n.is_trusted()).collect()
    }

    /// Gets all validator nodes.
    pub fn get_validators(&self) -> Vec<&RegistryNode> {
        self.get_nodes_by_type(NodeType::Validator)
    }

    /// Returns the total number of nodes.
    pub fn total_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Sets the local node ID.
    pub fn set_local_node(&mut self, node_id: Uuid) {
        self.local_node_id = Some(node_id);
    }

    /// Gets the local node ID.
    pub fn local_node_id(&self) -> Option<Uuid> {
        self.local_node_id
    }

    /// Gets nodes in a specific region.
    pub fn get_nodes_by_region(&self, region: &str) -> Vec<&RegistryNode> {
        self.nodes
            .values()
            .filter(|n| n.region.as_deref() == Some(region))
            .collect()
    }

    /// Removes a node from the network.
    pub fn remove_node(&mut self, node_id: &Uuid) -> Option<RegistryNode> {
        let node = self.nodes.remove(node_id)?;
        self.address_index.remove(&node.address);
        Some(node)
    }
}

impl Default for DecentralizedNodeManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Zero-Knowledge Proofs for Privacy
// =============================================================================

/// Type of zero-knowledge proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZkProofType {
    /// Proof of statute existence without revealing content
    Existence,
    /// Proof of statute version without revealing details
    Version,
    /// Proof of compliance without revealing statute text
    Compliance,
    /// Proof of ownership without revealing owner identity
    Ownership,
    /// Range proof for numeric statute properties
    Range,
}

impl ZkProofType {
    /// Returns the proof type name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Existence => "existence",
            Self::Version => "version",
            Self::Compliance => "compliance",
            Self::Ownership => "ownership",
            Self::Range => "range",
        }
    }
}

/// Represents a zero-knowledge proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    /// Unique proof ID
    pub proof_id: Uuid,
    /// Type of proof
    pub proof_type: ZkProofType,
    /// The statute ID this proof relates to
    pub statute_id: String,
    /// Proof data (serialized proof)
    pub proof_data: Vec<u8>,
    /// Public inputs (commitment hashes, etc.)
    pub public_inputs: Vec<String>,
    /// Verification key hash
    pub verification_key_hash: String,
    /// When the proof was generated
    pub created_at: DateTime<Utc>,
    /// Proof generation time in milliseconds
    pub generation_time_ms: u64,
    /// Whether the proof has been verified
    pub verified: bool,
}

impl ZkProof {
    /// Creates a new zero-knowledge proof.
    pub fn new(
        proof_type: ZkProofType,
        statute_id: impl Into<String>,
        proof_data: Vec<u8>,
        public_inputs: Vec<String>,
        verification_key_hash: impl Into<String>,
    ) -> Self {
        Self {
            proof_id: Uuid::new_v4(),
            proof_type,
            statute_id: statute_id.into(),
            proof_data,
            public_inputs,
            verification_key_hash: verification_key_hash.into(),
            created_at: Utc::now(),
            generation_time_ms: 0,
            verified: false,
        }
    }

    /// Sets the generation time.
    pub fn with_generation_time(mut self, time_ms: u64) -> Self {
        self.generation_time_ms = time_ms;
        self
    }

    /// Marks the proof as verified.
    pub fn mark_verified(&mut self) {
        self.verified = true;
    }

    /// Returns the proof size in bytes.
    pub fn size_bytes(&self) -> usize {
        self.proof_data.len()
    }
}

/// Manager for zero-knowledge proofs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofManager {
    /// All proofs stored by statute ID
    proofs: HashMap<String, Vec<ZkProof>>,
    /// Proofs indexed by proof ID for quick lookup
    proof_index: HashMap<Uuid, String>,
}

impl ZkProofManager {
    /// Creates a new ZK proof manager.
    pub fn new() -> Self {
        Self {
            proofs: HashMap::new(),
            proof_index: HashMap::new(),
        }
    }

    /// Adds a proof to the manager.
    pub fn add_proof(&mut self, proof: ZkProof) {
        self.proof_index
            .insert(proof.proof_id, proof.statute_id.clone());
        self.proofs
            .entry(proof.statute_id.clone())
            .or_default()
            .push(proof);
    }

    /// Gets a proof by its ID.
    pub fn get_proof(&self, proof_id: &Uuid) -> Option<&ZkProof> {
        let statute_id = self.proof_index.get(proof_id)?;
        self.proofs
            .get(statute_id)?
            .iter()
            .find(|p| p.proof_id == *proof_id)
    }

    /// Gets a proof by its ID (mutable).
    pub fn get_proof_mut(&mut self, proof_id: &Uuid) -> Option<&mut ZkProof> {
        let statute_id = self.proof_index.get(proof_id)?.clone();
        self.proofs
            .get_mut(&statute_id)?
            .iter_mut()
            .find(|p| p.proof_id == *proof_id)
    }

    /// Gets all proofs for a statute.
    pub fn get_proofs_for_statute(&self, statute_id: &str) -> Vec<&ZkProof> {
        self.proofs
            .get(statute_id)
            .map(|proofs| proofs.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all proofs of a specific type for a statute.
    pub fn get_proofs_by_type(&self, statute_id: &str, proof_type: ZkProofType) -> Vec<&ZkProof> {
        self.get_proofs_for_statute(statute_id)
            .into_iter()
            .filter(|p| p.proof_type == proof_type)
            .collect()
    }

    /// Gets all verified proofs for a statute.
    pub fn get_verified_proofs(&self, statute_id: &str) -> Vec<&ZkProof> {
        self.get_proofs_for_statute(statute_id)
            .into_iter()
            .filter(|p| p.verified)
            .collect()
    }

    /// Returns the total number of proofs.
    pub fn total_proofs(&self) -> usize {
        self.proofs.values().map(|v| v.len()).sum()
    }

    /// Verifies a proof (marks it as verified).
    pub fn verify_proof(&mut self, proof_id: &Uuid) -> bool {
        if let Some(proof) = self.get_proof_mut(proof_id) {
            proof.mark_verified();
            true
        } else {
            false
        }
    }

    /// Calculates the total size of all proofs in bytes.
    pub fn total_proof_size(&self) -> usize {
        self.proofs.values().flatten().map(|p| p.size_bytes()).sum()
    }
}

impl Default for ZkProofManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Utility Functions
// =============================================================================

/// Computes the SHA-256 hash of a byte slice.
pub fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Computes the SHA-256 hash of a string.
pub fn compute_string_hash(data: &str) -> String {
    compute_hash(data.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethereum_network() {
        assert_eq!(EthereumNetwork::Mainnet.chain_id(), 1);
        assert_eq!(EthereumNetwork::Sepolia.chain_id(), 11155111);
        assert_eq!(EthereumNetwork::Mainnet.name(), "mainnet");
    }

    #[test]
    fn test_ethereum_anchor() {
        let anchor = EthereumAnchor::new(
            "TEST-001",
            1,
            "hash123",
            EthereumNetwork::Sepolia,
            EthTxHash::new("0xabc"),
            12345,
        )
        .with_gas_used(21000)
        .with_contract_address("0x123");

        assert_eq!(anchor.statute_id, "TEST-001");
        assert_eq!(anchor.version, 1);
        assert_eq!(anchor.gas_used, 21000);
        assert_eq!(anchor.contract_address, Some("0x123".to_string()));
        assert!(anchor.explorer_url().contains("sepolia"));
    }

    #[test]
    fn test_ethereum_anchor_manager() {
        let mut manager = EthereumAnchorManager::new(EthereumNetwork::Mainnet);

        let anchor = EthereumAnchor::new(
            "TEST-001",
            1,
            "hash123",
            EthereumNetwork::Mainnet,
            EthTxHash::new("0xabc"),
            12345,
        );

        manager.add_anchor(anchor);

        assert_eq!(manager.total_anchors(), 1);
        assert!(manager.get_anchor("TEST-001", 1).is_some());
        assert!(manager.verify_anchor("TEST-001", 1, "hash123"));
        assert!(!manager.verify_anchor("TEST-001", 1, "wrong_hash"));
    }

    #[test]
    fn test_bitcoin_network() {
        assert_eq!(BitcoinNetwork::Mainnet.name(), "mainnet");
        assert_eq!(BitcoinNetwork::Testnet.name(), "testnet");
    }

    #[test]
    fn test_bitcoin_timestamp() {
        let timestamp = BitcoinTimestamp::new(
            "TEST-001",
            1,
            "event_hash",
            BitcoinNetwork::Mainnet,
            BitcoinTxId::new("txid123"),
            700000,
            "block_hash",
            Utc::now(),
        )
        .with_ots_proof(vec![1, 2, 3]);

        assert_eq!(timestamp.statute_id, "TEST-001");
        assert!(timestamp.has_ots_proof());
        assert!(timestamp.explorer_url().contains("blockstream"));
    }

    #[test]
    fn test_bitcoin_timestamp_manager() {
        let mut manager = BitcoinTimestampManager::new(BitcoinNetwork::Mainnet);

        let timestamp1 = BitcoinTimestamp::new(
            "TEST-001",
            1,
            "event1",
            BitcoinNetwork::Mainnet,
            BitcoinTxId::new("tx1"),
            700000,
            "block1",
            Utc::now(),
        );

        let timestamp2 = BitcoinTimestamp::new(
            "TEST-001",
            1,
            "event2",
            BitcoinNetwork::Mainnet,
            BitcoinTxId::new("tx2"),
            700001,
            "block2",
            Utc::now(),
        );

        manager.add_timestamp(timestamp1);
        manager.add_timestamp(timestamp2);

        assert_eq!(manager.total_timestamps(), 2);
        assert_eq!(manager.get_timestamps("TEST-001", 1).len(), 2);
    }

    #[test]
    fn test_nft_standard() {
        assert_eq!(NftStandard::Erc721.name(), "ERC-721");
        assert_eq!(NftStandard::Erc1155.name(), "ERC-1155");
    }

    #[test]
    fn test_statute_nft() {
        let mut nft = StatuteNft::new(
            "TEST-001",
            Some(1),
            NftStandard::Erc721,
            NftContractAddress::new("0x123"),
            "1",
            WalletAddress::new("0xabc"),
            EthereumNetwork::Mainnet,
            "ipfs://metadata",
            EthTxHash::new("0xmint"),
        );

        assert_eq!(nft.transfer_count(), 0);

        nft.add_transfer(
            WalletAddress::new("0xabc"),
            WalletAddress::new("0xdef"),
            EthTxHash::new("0xtransfer"),
        );

        assert_eq!(nft.transfer_count(), 1);
        assert_eq!(nft.owner.as_str(), "0xdef");
        assert!(nft.opensea_url().is_some());
    }

    #[test]
    fn test_nft_ownership_manager() {
        let mut manager = NftOwnershipManager::new();

        let nft = StatuteNft::new(
            "TEST-001",
            Some(1),
            NftStandard::Erc721,
            NftContractAddress::new("0x123"),
            "token1",
            WalletAddress::new("0xabc"),
            EthereumNetwork::Mainnet,
            "ipfs://metadata",
            EthTxHash::new("0xmint"),
        );

        manager.add_nft(nft);

        assert_eq!(manager.total_nfts(), 1);
        assert!(manager.get_nft_by_token_id("token1").is_some());
        assert_eq!(manager.get_nfts_for_statute("TEST-001").len(), 1);

        let success = manager.transfer_nft(
            "token1",
            WalletAddress::new("0xabc"),
            WalletAddress::new("0xdef"),
            EthTxHash::new("0xtransfer"),
        );

        assert!(success);

        let nft = manager.get_nft_by_token_id("token1").unwrap();
        assert_eq!(nft.owner.as_str(), "0xdef");
    }

    #[test]
    fn test_node_type() {
        assert_eq!(NodeType::Full.name(), "full");
        assert_eq!(NodeType::Validator.name(), "validator");
    }

    #[test]
    fn test_registry_node() {
        let mut node = RegistryNode::new(NodeType::Validator, "pubkey123", "192.168.1.1:8080")
            .with_region("us-east");

        assert!(node.is_validator());
        assert!(node.is_trusted());
        assert!(node.is_online);

        node.mark_offline();
        assert!(!node.is_online);

        node.update_reputation(0.5);
        assert!(!node.is_trusted());
    }

    #[test]
    fn test_decentralized_node_manager() {
        let mut manager = DecentralizedNodeManager::new();

        let node1 = RegistryNode::new(NodeType::Validator, "key1", "node1.example.com");
        let node_id1 = node1.node_id;

        let node2 = RegistryNode::new(NodeType::Full, "key2", "node2.example.com");

        manager.add_node(node1);
        manager.add_node(node2);

        assert_eq!(manager.total_nodes(), 2);
        assert_eq!(manager.get_validators().len(), 1);
        assert_eq!(manager.get_online_nodes().len(), 2);

        manager.set_local_node(node_id1);
        assert_eq!(manager.local_node_id(), Some(node_id1));
    }

    #[test]
    fn test_zk_proof_type() {
        assert_eq!(ZkProofType::Existence.name(), "existence");
        assert_eq!(ZkProofType::Compliance.name(), "compliance");
    }

    #[test]
    fn test_zk_proof() {
        let mut proof = ZkProof::new(
            ZkProofType::Existence,
            "TEST-001",
            vec![1, 2, 3, 4],
            vec!["input1".to_string()],
            "vk_hash",
        )
        .with_generation_time(1000);

        assert_eq!(proof.size_bytes(), 4);
        assert!(!proof.verified);

        proof.mark_verified();
        assert!(proof.verified);
    }

    #[test]
    fn test_zk_proof_manager() {
        let mut manager = ZkProofManager::new();

        let proof1 = ZkProof::new(
            ZkProofType::Existence,
            "TEST-001",
            vec![1, 2, 3],
            vec![],
            "vk1",
        );
        let proof_id1 = proof1.proof_id;

        let proof2 = ZkProof::new(
            ZkProofType::Compliance,
            "TEST-001",
            vec![4, 5, 6],
            vec![],
            "vk2",
        );

        manager.add_proof(proof1);
        manager.add_proof(proof2);

        assert_eq!(manager.total_proofs(), 2);
        assert_eq!(manager.get_proofs_for_statute("TEST-001").len(), 2);
        assert_eq!(
            manager
                .get_proofs_by_type("TEST-001", ZkProofType::Existence)
                .len(),
            1
        );

        manager.verify_proof(&proof_id1);
        assert_eq!(manager.get_verified_proofs("TEST-001").len(), 1);
        assert_eq!(manager.total_proof_size(), 6);
    }

    #[test]
    fn test_compute_hash() {
        let hash1 = compute_string_hash("test");
        let hash2 = compute_string_hash("test");
        let hash3 = compute_string_hash("different");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // SHA-256 produces 64 hex characters
    }
}
