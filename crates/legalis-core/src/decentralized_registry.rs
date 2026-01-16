//! Decentralized Legal Registry
//!
//! This module provides a decentralized registry for legal statutes using
//! content-addressable storage and peer-to-peer synchronization.

use crate::Statute;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Content identifier (CID) for statutes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContentId(String);

impl ContentId {
    /// Create a new CID from statute content
    pub fn from_statute(statute: &Statute) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(statute.id.as_bytes());
        hasher.update(statute.title.as_bytes());
        hasher.update(statute.effect.description.as_bytes());

        for condition in &statute.preconditions {
            hasher.update(format!("{:?}", condition).as_bytes());
        }

        let hash = hasher.finalize();
        let cid = format!("bafk{}", hex::encode(&hash[..20])); // Simplified CID format
        ContentId(cid)
    }

    /// Get the CID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parse a CID from string
    pub fn parse(s: &str) -> Result<Self, RegistryError> {
        if s.starts_with("bafk") && s.len() > 4 {
            Ok(ContentId(s.to_string()))
        } else {
            Err(RegistryError::InvalidCid(s.to_string()))
        }
    }
}

impl fmt::Display for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Hex encoding helper
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

/// Decentralized statute registry using content-addressable storage
///
/// # Example
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_core::decentralized_registry::DecentralizedRegistry;
///
/// let statute = Statute::new("statute-001", "Test Statute", Effect::new(EffectType::Grant, "Test effect"));
///
/// let mut registry = DecentralizedRegistry::new("node-1");
/// let cid = registry.add_statute(statute.clone()).unwrap();
///
/// let retrieved = registry.get_statute(&cid).unwrap();
/// assert_eq!(retrieved.id, "statute-001");
/// ```
pub struct DecentralizedRegistry {
    /// Node identifier
    node_id: String,
    /// Content-addressable storage (CID -> Statute)
    storage: HashMap<ContentId, Statute>,
    /// Index: Statute ID -> CID
    id_index: HashMap<String, ContentId>,
    /// Peers in the network
    peers: HashSet<String>,
    /// Pinned CIDs (always kept in storage)
    pinned: HashSet<ContentId>,
}

impl DecentralizedRegistry {
    /// Create a new decentralized registry
    pub fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            storage: HashMap::new(),
            id_index: HashMap::new(),
            peers: HashSet::new(),
            pinned: HashSet::new(),
        }
    }

    /// Add a statute to the registry
    pub fn add_statute(&mut self, statute: Statute) -> Result<ContentId, RegistryError> {
        let cid = ContentId::from_statute(&statute);

        if self.storage.contains_key(&cid) {
            return Err(RegistryError::DuplicateContent(cid.to_string()));
        }

        self.id_index.insert(statute.id.clone(), cid.clone());
        self.storage.insert(cid.clone(), statute);

        Ok(cid)
    }

    /// Get a statute by CID
    pub fn get_statute(&self, cid: &ContentId) -> Option<&Statute> {
        self.storage.get(cid)
    }

    /// Get a statute by its ID
    pub fn get_statute_by_id(&self, id: &str) -> Option<&Statute> {
        self.id_index.get(id).and_then(|cid| self.storage.get(cid))
    }

    /// Pin a statute (prevent garbage collection)
    pub fn pin(&mut self, cid: &ContentId) -> Result<(), RegistryError> {
        if !self.storage.contains_key(cid) {
            return Err(RegistryError::ContentNotFound(cid.to_string()));
        }

        self.pinned.insert(cid.clone());
        Ok(())
    }

    /// Unpin a statute
    pub fn unpin(&mut self, cid: &ContentId) {
        self.pinned.remove(cid);
    }

    /// Check if a CID is pinned
    pub fn is_pinned(&self, cid: &ContentId) -> bool {
        self.pinned.contains(cid)
    }

    /// Add a peer to the network
    pub fn add_peer(&mut self, peer_id: impl Into<String>) {
        self.peers.insert(peer_id.into());
    }

    /// Remove a peer from the network
    pub fn remove_peer(&mut self, peer_id: &str) {
        self.peers.remove(peer_id);
    }

    /// Get all peers
    pub fn peers(&self) -> Vec<&str> {
        self.peers.iter().map(|s| s.as_str()).collect()
    }

    /// Synchronize with a peer (simulated)
    pub fn sync_with_peer(&mut self, peer: &mut DecentralizedRegistry) -> SyncResult {
        let mut added = 0;
        let mut skipped = 0;

        // Copy statutes from peer that we don't have
        for (cid, statute) in &peer.storage {
            if !self.storage.contains_key(cid) {
                self.storage.insert(cid.clone(), statute.clone());
                self.id_index.insert(statute.id.clone(), cid.clone());
                added += 1;
            } else {
                skipped += 1;
            }
        }

        // Copy our statutes to peer
        for (cid, statute) in &self.storage {
            if !peer.storage.contains_key(cid) {
                peer.storage.insert(cid.clone(), statute.clone());
                peer.id_index.insert(statute.id.clone(), cid.clone());
                added += 1;
            }
        }

        SyncResult {
            added,
            skipped,
            errors: 0,
        }
    }

    /// Get all CIDs in the registry
    pub fn list_cids(&self) -> Vec<ContentId> {
        self.storage.keys().cloned().collect()
    }

    /// Get number of statutes in registry
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Get node ID
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Garbage collection - remove unpinned items (simulated)
    pub fn garbage_collect(&mut self) -> usize {
        let to_remove: Vec<_> = self
            .storage
            .keys()
            .filter(|cid| !self.pinned.contains(cid))
            .cloned()
            .collect();

        let count = to_remove.len();

        for cid in to_remove {
            if let Some(statute) = self.storage.remove(&cid) {
                self.id_index.remove(&statute.id);
            }
        }

        count
    }
}

/// Result of a synchronization operation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncResult {
    /// Number of items added
    pub added: usize,
    /// Number of items skipped (already present)
    pub skipped: usize,
    /// Number of errors
    pub errors: usize,
}

impl SyncResult {
    /// Check if sync was successful
    pub fn is_success(&self) -> bool {
        self.errors == 0
    }

    /// Total items processed
    pub fn total(&self) -> usize {
        self.added + self.skipped + self.errors
    }
}

/// Distributed hash table for statute discovery
///
/// # Example
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_core::decentralized_registry::{DistributedHashTable, ContentId};
///
/// let statute = Statute::new("dht-test", "DHT Test", Effect::new(EffectType::Grant, "Test"));
///
/// let mut dht = DistributedHashTable::new();
/// let cid = ContentId::from_statute(&statute);
/// dht.announce(&cid, "node-1");
///
/// let providers = dht.find_providers(&cid);
/// assert_eq!(providers.len(), 1);
/// assert!(providers.contains(&"node-1".to_string()));
/// ```
pub struct DistributedHashTable {
    /// Map of CID -> list of provider node IDs
    providers: HashMap<ContentId, HashSet<String>>,
}

impl DistributedHashTable {
    /// Create a new DHT
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Announce that a node provides a CID
    pub fn announce(&mut self, cid: &ContentId, node_id: impl Into<String>) {
        self.providers
            .entry(cid.clone())
            .or_default()
            .insert(node_id.into());
    }

    /// Find providers for a CID
    pub fn find_providers(&self, cid: &ContentId) -> Vec<String> {
        self.providers
            .get(cid)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Remove a provider announcement
    pub fn remove_provider(&mut self, cid: &ContentId, node_id: &str) {
        if let Some(providers) = self.providers.get_mut(cid) {
            providers.remove(node_id);

            if providers.is_empty() {
                self.providers.remove(cid);
            }
        }
    }

    /// Get total number of announcements
    pub fn announcement_count(&self) -> usize {
        self.providers.values().map(|set| set.len()).sum()
    }
}

impl Default for DistributedHashTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Registry errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum RegistryError {
    #[error("Invalid CID: {0}")]
    InvalidCid(String),

    #[error("Content not found: {0}")]
    ContentNotFound(String),

    #[error("Duplicate content: {0}")]
    DuplicateContent(String),

    #[error("Synchronization failed: {0}")]
    SyncFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Effect, EffectType};

    #[test]
    fn test_content_id_generation() {
        let statute = Statute::new("test-001", "Test", Effect::new(EffectType::Grant, "Test"));

        let cid = ContentId::from_statute(&statute);
        assert!(cid.as_str().starts_with("bafk"));
    }

    #[test]
    fn test_add_and_retrieve() {
        let statute = Statute::new("test-002", "Test", Effect::new(EffectType::Grant, "Test"));

        let mut registry = DecentralizedRegistry::new("node-1");
        let cid = registry.add_statute(statute.clone()).unwrap();

        let retrieved = registry.get_statute(&cid).unwrap();
        assert_eq!(retrieved.id, "test-002");

        let by_id = registry.get_statute_by_id("test-002").unwrap();
        assert_eq!(by_id.id, "test-002");
    }

    #[test]
    fn test_pinning() {
        let statute = Statute::new(
            "pin-test",
            "Pin Test",
            Effect::new(EffectType::Grant, "Test"),
        );

        let mut registry = DecentralizedRegistry::new("node-1");
        let cid = registry.add_statute(statute).unwrap();

        assert!(!registry.is_pinned(&cid));

        registry.pin(&cid).unwrap();
        assert!(registry.is_pinned(&cid));

        registry.unpin(&cid);
        assert!(!registry.is_pinned(&cid));
    }

    #[test]
    fn test_peer_management() {
        let mut registry = DecentralizedRegistry::new("node-1");

        assert_eq!(registry.peers().len(), 0);

        registry.add_peer("node-2");
        registry.add_peer("node-3");

        assert_eq!(registry.peers().len(), 2);
        assert!(registry.peers().contains(&"node-2"));

        registry.remove_peer("node-2");
        assert_eq!(registry.peers().len(), 1);
    }

    #[test]
    fn test_synchronization() {
        let statute1 = Statute::new(
            "sync-001",
            "Sync Test 1",
            Effect::new(EffectType::Grant, "Test"),
        );
        let statute2 = Statute::new(
            "sync-002",
            "Sync Test 2",
            Effect::new(EffectType::Obligation, "Test"),
        );

        let mut registry1 = DecentralizedRegistry::new("node-1");
        let mut registry2 = DecentralizedRegistry::new("node-2");

        registry1.add_statute(statute1).unwrap();
        registry2.add_statute(statute2).unwrap();

        let result = registry1.sync_with_peer(&mut registry2);

        assert!(result.is_success());
        assert_eq!(result.added, 2);

        assert_eq!(registry1.len(), 2);
        assert_eq!(registry2.len(), 2);
    }

    #[test]
    fn test_garbage_collection() {
        let statute1 = Statute::new(
            "gc-001",
            "GC Test 1",
            Effect::new(EffectType::Grant, "Test"),
        );
        let statute2 = Statute::new(
            "gc-002",
            "GC Test 2",
            Effect::new(EffectType::Grant, "Test"),
        );

        let mut registry = DecentralizedRegistry::new("node-1");
        let cid1 = registry.add_statute(statute1).unwrap();
        let cid2 = registry.add_statute(statute2).unwrap();

        // Pin only the first statute
        registry.pin(&cid1).unwrap();

        assert_eq!(registry.len(), 2);

        // Garbage collect
        let removed = registry.garbage_collect();

        // Only unpinned statute should be removed
        assert_eq!(removed, 1);
        assert_eq!(registry.len(), 1);
        assert!(registry.get_statute(&cid1).is_some());
        assert!(registry.get_statute(&cid2).is_none());
    }

    #[test]
    fn test_dht() {
        let mut dht = DistributedHashTable::new();

        let statute = Statute::new(
            "dht-test",
            "DHT Test",
            Effect::new(EffectType::Grant, "Test"),
        );

        let cid = ContentId::from_statute(&statute);

        dht.announce(&cid, "node-1");
        dht.announce(&cid, "node-2");

        let providers = dht.find_providers(&cid);
        assert_eq!(providers.len(), 2);

        dht.remove_provider(&cid, "node-1");

        let providers = dht.find_providers(&cid);
        assert_eq!(providers.len(), 1);

        assert_eq!(dht.announcement_count(), 1);
    }

    #[test]
    fn test_cid_parse() {
        let cid_str = "bafkabc123";
        let cid = ContentId::parse(cid_str).unwrap();
        assert_eq!(cid.as_str(), cid_str);

        let invalid = ContentId::parse("invalid");
        assert!(invalid.is_err());
    }
}
