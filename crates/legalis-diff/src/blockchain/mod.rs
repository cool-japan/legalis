//! Blockchain and distributed-ledger primitives for statute diffs (v0.5.4).
//!
//! This module records statute diffs onto an append-only, cryptographically
//! linked ledger and layers higher-level distributed-ledger features on top of
//! it. Everything here is pure Rust and self-contained — no network calls are
//! required for the core algorithms:
//!
//! - [`ledger`] — an immutable, Merkle-anchored block-chain of [`crate::StatuteDiff`]
//!   records with proof-of-work sealing, chain validation and inclusion proofs.
//! - [`contract`] — a deterministic, gas-metered smart-contract engine that
//!   triggers automated workflows when diffs are recorded.
//! - [`consensus`] — distributed consensus (proof-of-authority, weighted
//!   proof-of-stake leader election and a BFT voting tally) for verifying diffs
//!   across a validator set.
//! - [`token`] — an integer-exact token ledger with metered, pay-per-call
//!   pricing for paid API access.
//! - [`nft`] — ERC-721-style non-fungible tokens that uniquely tokenise
//!   important diffs together with their provenance history.
//!
//! # Deferred external bindings
//!
//! Settlement onto a *public* chain (Ethereum, Bitcoin, Polygon, …) requires a
//! live JSON-RPC endpoint and signing keys that this offline workspace does not
//! have. That binding is abstracted behind the [`ChainAnchor`] trait; two
//! local backends are provided ([`InMemoryAnchor`] and [`FileAnchor`]) so the
//! anchoring workflow is fully exercisable, and a real RPC client can be added
//! later as another implementation without touching callers.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::diff;
//! use legalis_diff::blockchain::{DiffLedger, record_diff};
//!
//! let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
//! let mut new = old.clone();
//! new.effect = Effect::new(EffectType::Revoke, "Revoked");
//! let d = diff(&old, &new).unwrap();
//!
//! let mut ledger = DiffLedger::new(2);
//! record_diff(&mut ledger, &d, "registrar").unwrap();
//! ledger.seal_pending().unwrap();
//! assert!(ledger.validate().is_ok());
//! ```

use crate::{DiffError, DiffResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub mod consensus;
pub mod contract;
pub mod ledger;
pub mod nft;
pub mod token;

pub use consensus::{
    ConsensusEngine, ConsensusOutcome, ConsensusStatus, Proposal, ProposalTally, SelectionMethod,
    Validator, Vote, VoteChoice,
};
pub use contract::{
    Action, Clause, ContractEngine, ContractEvent, ContractReceipt, EventKind, LegalWorkflow,
    SmartContract, Trigger, WorkflowState, WorkflowStep,
};
pub use ledger::{Block, DiffLedger, DiffRecord, MerkleProof, MerkleTree, record_diff};
pub use nft::{DiffNft, NftAttribute, NftMetadata, NftRegistry, ProvenanceEntry, is_mint_worthy};
pub use token::{ApiOperation, PricingTable, TokenLedger, TokenTransaction, UsageReport};

/// Computes a lowercase hex SHA-256 digest over a single byte slice.
pub(crate) fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Computes a lowercase hex SHA-256 digest over several byte slices.
///
/// The parts are length-prefixed before hashing so that, for example,
/// `["ab", "c"]` and `["a", "bc"]` produce different digests (domain
/// separation against trivial concatenation collisions).
pub(crate) fn sha256_parts(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    hex::encode(hasher.finalize())
}

/// Current UNIX timestamp in seconds, saturating to `0` before the epoch.
///
/// Kept panic-free so ledger construction never aborts on a misconfigured
/// system clock.
pub(crate) fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// A ledger address: a short, deterministic identifier derived from an owner
/// key or label.
///
/// Addresses are the canonical way to identify wallets ([`token`]) and NFT
/// owners ([`nft`]). They are derived by hashing the source material, so two
/// callers that start from the same public key obtain the same address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Address(String);

impl Address {
    /// Derives an address from arbitrary key material (e.g. a public key).
    ///
    /// The address is `0x` followed by the first 20 bytes (40 hex chars) of the
    /// SHA-256 of the material, mirroring the 20-byte address convention used
    /// by account-based chains.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::blockchain::Address;
    ///
    /// let a = Address::from_key(b"alice-public-key");
    /// let b = Address::from_key(b"alice-public-key");
    /// assert_eq!(a, b);
    /// assert!(a.as_str().starts_with("0x"));
    /// ```
    pub fn from_key(material: &[u8]) -> Self {
        let digest = sha256_hex(material);
        Self(format!("0x{}", &digest[..40]))
    }

    /// Wraps an already-formatted address string verbatim.
    ///
    /// Useful for well-known labels such as the mint/treasury account.
    pub fn from_label(label: impl Into<String>) -> Self {
        Self(label.into())
    }

    /// Returns the address as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A receipt returned after anchoring a payload hash to an external ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorReceipt {
    /// The hash that was anchored.
    pub payload_hash: String,
    /// A synthetic transaction identifier for the anchoring operation.
    pub anchor_id: String,
    /// Monotonic height/sequence at which the payload was anchored.
    pub height: u64,
    /// UNIX timestamp (seconds) of the anchoring operation.
    pub timestamp: u64,
    /// Human-readable network label.
    pub network: String,
}

/// Abstraction over an external ledger that statute-diff hashes can be anchored
/// to.
///
/// The pure-Rust backends in this module ([`InMemoryAnchor`], [`FileAnchor`])
/// implement the full anchoring/verification workflow locally. A production
/// deployment can add a `JsonRpcAnchor` that submits the same `payload_hash`
/// to a public chain without changing any caller code — that networked binding
/// is intentionally deferred.
pub trait ChainAnchor {
    /// Anchors a payload hash, returning a receipt. Anchoring the same hash
    /// twice returns the original receipt (idempotent).
    fn anchor(&mut self, payload_hash: &str) -> DiffResult<AnchorReceipt>;

    /// Returns `true` if the payload hash has been anchored.
    fn is_anchored(&self, payload_hash: &str) -> bool;

    /// Returns the receipt for an anchored payload hash, if present.
    fn receipt(&self, payload_hash: &str) -> Option<AnchorReceipt>;

    /// Human-readable network label for this anchor.
    fn network(&self) -> &str;
}

/// An in-memory [`ChainAnchor`] backend.
///
/// Anchored hashes and their receipts are held in memory; nothing is persisted.
#[derive(Debug, Clone)]
pub struct InMemoryAnchor {
    network: String,
    next_height: u64,
    receipts: HashMap<String, AnchorReceipt>,
}

impl InMemoryAnchor {
    /// Creates a new in-memory anchor with the given network label.
    pub fn new(network: impl Into<String>) -> Self {
        Self {
            network: network.into(),
            next_height: 0,
            receipts: HashMap::new(),
        }
    }

    /// Number of distinct payloads anchored so far.
    pub fn len(&self) -> usize {
        self.receipts.len()
    }

    /// Returns `true` if nothing has been anchored yet.
    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }
}

impl Default for InMemoryAnchor {
    fn default() -> Self {
        Self::new("in-memory")
    }
}

impl ChainAnchor for InMemoryAnchor {
    fn anchor(&mut self, payload_hash: &str) -> DiffResult<AnchorReceipt> {
        if payload_hash.is_empty() {
            return Err(DiffError::InvalidTransaction(
                "cannot anchor an empty payload hash".to_string(),
            ));
        }
        if let Some(existing) = self.receipts.get(payload_hash) {
            return Ok(existing.clone());
        }
        let height = self.next_height;
        self.next_height += 1;
        let anchor_id = sha256_parts(&[payload_hash.as_bytes(), &height.to_le_bytes()]);
        let receipt = AnchorReceipt {
            payload_hash: payload_hash.to_string(),
            anchor_id,
            height,
            timestamp: current_timestamp(),
            network: self.network.clone(),
        };
        self.receipts
            .insert(payload_hash.to_string(), receipt.clone());
        Ok(receipt)
    }

    fn is_anchored(&self, payload_hash: &str) -> bool {
        self.receipts.contains_key(payload_hash)
    }

    fn receipt(&self, payload_hash: &str) -> Option<AnchorReceipt> {
        self.receipts.get(payload_hash).cloned()
    }

    fn network(&self) -> &str {
        &self.network
    }
}

/// A file-backed [`ChainAnchor`] backend.
///
/// Receipts are persisted as a JSON document at the configured path, so anchors
/// survive across process restarts. Each [`FileAnchor::anchor`] call appends the
/// new receipt and rewrites the file atomically-enough for single-process use.
#[derive(Debug, Clone)]
pub struct FileAnchor {
    network: String,
    path: std::path::PathBuf,
    next_height: u64,
    receipts: HashMap<String, AnchorReceipt>,
}

impl FileAnchor {
    /// Opens (or creates) a file-backed anchor at `path`, loading any existing
    /// receipts.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if an existing file cannot be
    /// read or parsed.
    pub fn open(
        network: impl Into<String>,
        path: impl Into<std::path::PathBuf>,
    ) -> DiffResult<Self> {
        let path = path.into();
        let (next_height, receipts) = if path.exists() {
            let bytes = std::fs::read(&path).map_err(|e| {
                DiffError::SerializationError(format!("failed to read anchor file: {}", e))
            })?;
            let receipts: HashMap<String, AnchorReceipt> =
                serde_json::from_slice(&bytes).map_err(|e| {
                    DiffError::SerializationError(format!("failed to parse anchor file: {}", e))
                })?;
            let next = receipts.values().map(|r| r.height + 1).max().unwrap_or(0);
            (next, receipts)
        } else {
            (0, HashMap::new())
        };
        Ok(Self {
            network: network.into(),
            path,
            next_height,
            receipts,
        })
    }

    fn persist(&self) -> DiffResult<()> {
        let bytes = serde_json::to_vec_pretty(&self.receipts).map_err(|e| {
            DiffError::SerializationError(format!("failed to serialize anchor file: {}", e))
        })?;
        std::fs::write(&self.path, bytes).map_err(|e| {
            DiffError::SerializationError(format!("failed to write anchor file: {}", e))
        })
    }

    /// Number of distinct payloads anchored so far.
    pub fn len(&self) -> usize {
        self.receipts.len()
    }

    /// Returns `true` if nothing has been anchored yet.
    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }
}

impl ChainAnchor for FileAnchor {
    fn anchor(&mut self, payload_hash: &str) -> DiffResult<AnchorReceipt> {
        if payload_hash.is_empty() {
            return Err(DiffError::InvalidTransaction(
                "cannot anchor an empty payload hash".to_string(),
            ));
        }
        if let Some(existing) = self.receipts.get(payload_hash) {
            return Ok(existing.clone());
        }
        let height = self.next_height;
        self.next_height += 1;
        let anchor_id = sha256_parts(&[payload_hash.as_bytes(), &height.to_le_bytes()]);
        let receipt = AnchorReceipt {
            payload_hash: payload_hash.to_string(),
            anchor_id,
            height,
            timestamp: current_timestamp(),
            network: self.network.clone(),
        };
        self.receipts
            .insert(payload_hash.to_string(), receipt.clone());
        self.persist()?;
        Ok(receipt)
    }

    fn is_anchored(&self, payload_hash: &str) -> bool {
        self.receipts.contains_key(payload_hash)
    }

    fn receipt(&self, payload_hash: &str) -> Option<AnchorReceipt> {
        self.receipts.get(payload_hash).cloned()
    }

    fn network(&self) -> &str {
        &self.network
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hex_is_deterministic_and_hex() {
        let a = sha256_hex(b"legalis");
        let b = sha256_hex(b"legalis");
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_sha256_parts_domain_separation() {
        let x = sha256_parts(&[b"ab", b"c"]);
        let y = sha256_parts(&[b"a", b"bc"]);
        assert_ne!(x, y);
    }

    #[test]
    fn test_address_from_key_deterministic() {
        let a = Address::from_key(b"alice");
        let b = Address::from_key(b"alice");
        let c = Address::from_key(b"bob");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a.as_str().starts_with("0x"));
        assert_eq!(a.as_str().len(), 42);
    }

    #[test]
    fn test_address_label_and_display() {
        let treasury = Address::from_label("treasury");
        assert_eq!(treasury.as_str(), "treasury");
        assert_eq!(format!("{}", treasury), "treasury");
    }

    #[test]
    fn test_in_memory_anchor_idempotent() {
        let mut anchor = InMemoryAnchor::new("test-net");
        assert!(anchor.is_empty());
        let r1 = anchor.anchor("deadbeef").unwrap();
        let r2 = anchor.anchor("deadbeef").unwrap();
        assert_eq!(r1, r2);
        assert_eq!(anchor.len(), 1);
        assert!(anchor.is_anchored("deadbeef"));
        assert_eq!(anchor.network(), "test-net");
    }

    #[test]
    fn test_in_memory_anchor_distinct_heights() {
        let mut anchor = InMemoryAnchor::default();
        let r1 = anchor.anchor("aaaa").unwrap();
        let r2 = anchor.anchor("bbbb").unwrap();
        assert_ne!(r1.height, r2.height);
        assert_ne!(r1.anchor_id, r2.anchor_id);
        assert_eq!(anchor.receipt("aaaa").unwrap(), r1);
    }

    #[test]
    fn test_in_memory_anchor_rejects_empty() {
        let mut anchor = InMemoryAnchor::default();
        assert!(anchor.anchor("").is_err());
    }

    #[test]
    fn test_file_anchor_persists_across_reopen() {
        let mut path = std::env::temp_dir();
        path.push(format!("legalis_anchor_{}.json", current_timestamp()));
        let _ = std::fs::remove_file(&path);

        {
            let mut anchor = FileAnchor::open("file-net", &path).unwrap();
            anchor.anchor("hash-one").unwrap();
            anchor.anchor("hash-two").unwrap();
            assert_eq!(anchor.len(), 2);
        }

        // Reopen: receipts should be reloaded and the height counter continue.
        let mut reopened = FileAnchor::open("file-net", &path).unwrap();
        assert!(reopened.is_anchored("hash-one"));
        assert!(reopened.is_anchored("hash-two"));
        let r = reopened.anchor("hash-three").unwrap();
        assert_eq!(r.height, 2);

        let _ = std::fs::remove_file(&path);
    }
}
