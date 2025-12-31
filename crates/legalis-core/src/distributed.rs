//! Distributed legal reasoning framework.
//!
//! This module provides abstractions and implementations for distributed
//! statute evaluation, enabling legal reasoning across multiple nodes with
//! partition tolerance and eventual consistency.
//!
//! ## Features
//!
//! - **Distributed evaluation**: Evaluate statutes across multiple nodes
//! - **Partition tolerance**: Continue operating despite network partitions
//! - **Eventual consistency**: Statute registries converge to consistent state
//! - **Conflict resolution**: Automatic resolution of conflicting updates
//! - **Cross-shard coordination**: Query and aggregate results across shards
//!
//! ## Architecture
//!
//! The framework uses a trait-based design that allows for multiple
//! implementation strategies:
//!
//! - Local in-memory (for testing and single-node deployments)
//! - Network-based (for true distributed deployments)
//! - Hybrid (local with remote fallback)
//!
//! ## Example
//!
//! ```
//! use legalis_core::distributed::{LocalNode, NodeId, DistributedRegistry};
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! // Create a local node
//! let node = LocalNode::new(NodeId::new("node-1"));
//!
//! // Create a distributed registry
//! let mut registry = DistributedRegistry::new(node);
//!
//! // Add statutes
//! let statute = Statute::new("law-1", "Tax Law", Effect::new(EffectType::Grant, "Tax credit"));
//! registry.add_local(statute);
//!
//! assert_eq!(registry.len(), 1);
//! ```

use crate::Statute;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Unique identifier for a node in the distributed system.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(String);

impl NodeId {
    /// Creates a new node ID.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::NodeId;
    ///
    /// let node_id = NodeId::new("node-1");
    /// assert_eq!(node_id.as_str(), "node-1");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the node ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Vector clock for tracking causality in distributed systems.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorClock {
    clocks: HashMap<NodeId, u64>,
}

impl VectorClock {
    /// Creates a new empty vector clock.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::VectorClock;
    ///
    /// let clock = VectorClock::new();
    /// assert_eq!(clock.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            clocks: HashMap::new(),
        }
    }

    /// Increments the clock for a specific node.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::{VectorClock, NodeId};
    ///
    /// let mut clock = VectorClock::new();
    /// let node = NodeId::new("node-1");
    /// clock.increment(&node);
    /// assert_eq!(clock.get(&node), 1);
    /// ```
    pub fn increment(&mut self, node: &NodeId) {
        let counter = self.clocks.entry(node.clone()).or_insert(0);
        *counter += 1;
    }

    /// Gets the clock value for a specific node.
    pub fn get(&self, node: &NodeId) -> u64 {
        self.clocks.get(node).copied().unwrap_or(0)
    }

    /// Returns the number of nodes tracked.
    pub fn len(&self) -> usize {
        self.clocks.len()
    }

    /// Returns `true` if no nodes are tracked.
    pub fn is_empty(&self) -> bool {
        self.clocks.is_empty()
    }

    /// Merges another vector clock, taking the maximum for each node.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::{VectorClock, NodeId};
    ///
    /// let mut clock1 = VectorClock::new();
    /// let mut clock2 = VectorClock::new();
    /// let node1 = NodeId::new("node-1");
    /// let node2 = NodeId::new("node-2");
    ///
    /// clock1.increment(&node1);
    /// clock1.increment(&node1);
    /// clock2.increment(&node2);
    ///
    /// clock1.merge(&clock2);
    /// assert_eq!(clock1.get(&node1), 2);
    /// assert_eq!(clock1.get(&node2), 1);
    /// ```
    pub fn merge(&mut self, other: &VectorClock) {
        for (node, &value) in &other.clocks {
            let current = self.clocks.entry(node.clone()).or_insert(0);
            *current = (*current).max(value);
        }
    }

    /// Checks if this clock happens-before another clock.
    pub fn happens_before(&self, other: &VectorClock) -> bool {
        let mut strictly_less = false;
        for (node, &value) in &self.clocks {
            let other_value = other.get(node);
            if value > other_value {
                return false;
            }
            if value < other_value {
                strictly_less = true;
            }
        }
        for node in other.clocks.keys() {
            if !self.clocks.contains_key(node) {
                strictly_less = true;
            }
        }
        strictly_less
    }

    /// Checks if two clocks are concurrent (neither happens-before the other).
    pub fn is_concurrent(&self, other: &VectorClock) -> bool {
        !self.happens_before(other) && !other.happens_before(self)
    }
}

impl Default for VectorClock {
    fn default() -> Self {
        Self::new()
    }
}

/// Versioned statute with vector clock for conflict resolution.
#[derive(Clone, Debug)]
pub struct VersionedStatute {
    /// The statute
    pub statute: Statute,
    /// Vector clock for causality tracking
    pub version: VectorClock,
    /// Node that last modified this statute
    pub modified_by: NodeId,
    /// Timestamp of last modification (for tie-breaking)
    pub timestamp: u64,
}

impl VersionedStatute {
    /// Creates a new versioned statute.
    pub fn new(statute: Statute, node: NodeId) -> Self {
        let mut version = VectorClock::new();
        version.increment(&node);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            statute,
            version,
            modified_by: node,
            timestamp,
        }
    }

    /// Updates the statute, incrementing the version.
    pub fn update(&mut self, statute: Statute, node: NodeId) {
        self.statute = statute;
        self.version.increment(&node);
        self.modified_by = node;
        self.timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Resolves a conflict between two versions using Last-Write-Wins (LWW) strategy.
    ///
    /// Returns the winning version.
    pub fn resolve_conflict<'a>(
        a: &'a VersionedStatute,
        b: &'a VersionedStatute,
    ) -> &'a VersionedStatute {
        // If one happens-before the other, choose the later one
        if a.version.happens_before(&b.version) {
            return b;
        }
        if b.version.happens_before(&a.version) {
            return a;
        }

        // Concurrent updates: use timestamp for tie-breaking
        if a.timestamp > b.timestamp {
            a
        } else if b.timestamp > a.timestamp {
            b
        } else {
            // Same timestamp: use node ID for deterministic ordering
            if a.modified_by > b.modified_by { a } else { b }
        }
    }
}

/// Trait for distributed node operations.
pub trait DistributedNode: Send + Sync {
    /// Gets the node's unique identifier.
    fn node_id(&self) -> &NodeId;

    /// Checks if the node is available.
    fn is_available(&self) -> bool;

    /// Sends a statute to another node (in a real implementation, this would use networking).
    fn send_statute(&self, target: &NodeId, statute: &VersionedStatute) -> Result<(), String>;

    /// Receives statutes from other nodes (mock for now).
    fn receive_statutes(&self) -> Vec<VersionedStatute>;
}

/// Local node implementation for testing and single-node deployments.
pub struct LocalNode {
    id: NodeId,
    available: bool,
}

impl LocalNode {
    /// Creates a new local node.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::{LocalNode, NodeId, DistributedNode};
    ///
    /// let node = LocalNode::new(NodeId::new("node-1"));
    /// assert!(node.is_available());
    /// ```
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            available: true,
        }
    }

    /// Sets the node's availability status.
    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }
}

impl DistributedNode for LocalNode {
    fn node_id(&self) -> &NodeId {
        &self.id
    }

    fn is_available(&self) -> bool {
        self.available
    }

    fn send_statute(&self, _target: &NodeId, _statute: &VersionedStatute) -> Result<(), String> {
        if self.available {
            Ok(())
        } else {
            Err("Node unavailable".to_string())
        }
    }

    fn receive_statutes(&self) -> Vec<VersionedStatute> {
        Vec::new()
    }
}

/// Distributed statute registry with eventual consistency.
pub struct DistributedRegistry {
    node: Arc<dyn DistributedNode>,
    statutes: HashMap<String, VersionedStatute>,
}

impl DistributedRegistry {
    /// Creates a new distributed registry.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::{DistributedRegistry, LocalNode, NodeId};
    ///
    /// let node = LocalNode::new(NodeId::new("node-1"));
    /// let registry = DistributedRegistry::new(node);
    /// assert_eq!(registry.len(), 0);
    /// ```
    pub fn new<N: DistributedNode + 'static>(node: N) -> Self {
        Self {
            node: Arc::new(node),
            statutes: HashMap::new(),
        }
    }

    /// Adds a statute to the local registry.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::{DistributedRegistry, LocalNode, NodeId};
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let node = LocalNode::new(NodeId::new("node-1"));
    /// let mut registry = DistributedRegistry::new(node);
    ///
    /// let statute = Statute::new("law-1", "Tax Law", Effect::new(EffectType::Grant, "Tax credit"));
    /// registry.add_local(statute);
    /// assert_eq!(registry.len(), 1);
    /// ```
    pub fn add_local(&mut self, statute: Statute) {
        let id = statute.id.clone();
        let versioned = VersionedStatute::new(statute, self.node.node_id().clone());
        self.statutes.insert(id, versioned);
    }

    /// Updates a statute, handling version conflicts.
    pub fn update(&mut self, statute: Statute) -> Result<(), String> {
        let id = statute.id.clone();
        if let Some(existing) = self.statutes.get_mut(&id) {
            existing.update(statute, self.node.node_id().clone());
            Ok(())
        } else {
            Err(format!("Statute {} not found", id))
        }
    }

    /// Merges a versioned statute from another node, resolving conflicts.
    pub fn merge(&mut self, remote: VersionedStatute) {
        let id = remote.statute.id.clone();
        if let Some(local) = self.statutes.get(&id) {
            let winner = VersionedStatute::resolve_conflict(local, &remote);
            self.statutes.insert(id, winner.clone());
        } else {
            self.statutes.insert(id, remote);
        }
    }

    /// Gets a statute by ID.
    pub fn get(&self, id: &str) -> Option<&Statute> {
        self.statutes.get(id).map(|v| &v.statute)
    }

    /// Returns the number of statutes in the registry.
    pub fn len(&self) -> usize {
        self.statutes.len()
    }

    /// Returns `true` if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }

    /// Synchronizes with received statutes from other nodes.
    pub fn sync(&mut self) {
        let received = self.node.receive_statutes();
        for statute in received {
            self.merge(statute);
        }
    }

    /// Returns an iterator over all statutes.
    pub fn iter(&self) -> impl Iterator<Item = &Statute> {
        self.statutes.values().map(|v| &v.statute)
    }
}

/// Conflict resolution strategy for handling concurrent updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConflictStrategy {
    /// Last Write Wins (LWW) - use timestamps
    LastWriteWins,
    /// Most Recent Version - use vector clocks
    MostRecentVersion,
    /// Manual - requires explicit resolution
    Manual,
}

/// Partition-tolerant conflict resolver.
pub struct ConflictResolver {
    strategy: ConflictStrategy,
}

impl ConflictResolver {
    /// Creates a new conflict resolver with the specified strategy.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::{ConflictResolver, ConflictStrategy};
    ///
    /// let resolver = ConflictResolver::new(ConflictStrategy::LastWriteWins);
    /// ```
    pub fn new(strategy: ConflictStrategy) -> Self {
        Self { strategy }
    }

    /// Resolves a conflict between two versioned statutes.
    pub fn resolve<'a>(
        &self,
        local: &'a VersionedStatute,
        remote: &'a VersionedStatute,
    ) -> &'a VersionedStatute {
        match self.strategy {
            ConflictStrategy::LastWriteWins => VersionedStatute::resolve_conflict(local, remote),
            ConflictStrategy::MostRecentVersion => {
                if local.version.happens_before(&remote.version) {
                    remote
                } else if remote.version.happens_before(&local.version) {
                    local
                } else {
                    // Concurrent: fall back to LWW
                    VersionedStatute::resolve_conflict(local, remote)
                }
            }
            ConflictStrategy::Manual => {
                // For manual resolution, prefer local (application must handle)
                local
            }
        }
    }
}

/// Gossip protocol for eventual consistency.
pub struct GossipProtocol {
    sync_interval_ms: u64,
    max_batch_size: usize,
}

impl GossipProtocol {
    /// Creates a new gossip protocol with default settings.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::GossipProtocol;
    ///
    /// let gossip = GossipProtocol::new();
    /// ```
    pub fn new() -> Self {
        Self {
            sync_interval_ms: 1000,
            max_batch_size: 100,
        }
    }

    /// Creates a gossip protocol with custom settings.
    pub fn with_config(sync_interval_ms: u64, max_batch_size: usize) -> Self {
        Self {
            sync_interval_ms,
            max_batch_size,
        }
    }

    /// Gets the sync interval in milliseconds.
    pub fn sync_interval(&self) -> u64 {
        self.sync_interval_ms
    }

    /// Gets the maximum batch size for syncing.
    pub fn max_batch_size(&self) -> usize {
        self.max_batch_size
    }

    /// Selects peers to gossip with (simple round-robin for now).
    pub fn select_peers(
        &self,
        all_nodes: &[NodeId],
        current: &NodeId,
        count: usize,
    ) -> Vec<NodeId> {
        all_nodes
            .iter()
            .filter(|n| *n != current)
            .take(count)
            .cloned()
            .collect()
    }
}

impl Default for GossipProtocol {
    fn default() -> Self {
        Self::new()
    }
}

/// Shard identifier for distributed storage.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShardId(u32);

impl ShardId {
    /// Creates a new shard ID.
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the shard ID as a u32.
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// Shard assignment strategy.
pub struct ShardRouter {
    num_shards: u32,
}

impl ShardRouter {
    /// Creates a new shard router with the specified number of shards.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::ShardRouter;
    ///
    /// let router = ShardRouter::new(4);
    /// ```
    pub fn new(num_shards: u32) -> Self {
        Self { num_shards }
    }

    /// Assigns a statute to a shard based on its ID.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::ShardRouter;
    ///
    /// let router = ShardRouter::new(4);
    /// let shard = router.assign_shard("statute-123");
    /// assert!(shard.as_u32() < 4);
    /// ```
    pub fn assign_shard(&self, statute_id: &str) -> ShardId {
        let hash = self.hash_string(statute_id);
        ShardId::new(hash % self.num_shards)
    }

    /// Simple string hashing function.
    fn hash_string(&self, s: &str) -> u32 {
        let mut hash = 0u32;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }

    /// Returns the number of shards.
    pub fn num_shards(&self) -> u32 {
        self.num_shards
    }
}

/// Distributed entailment engine for legal reasoning across shards.
pub struct DistributedEntailmentEngine {
    shards: HashMap<ShardId, Arc<dyn DistributedNode>>,
    router: ShardRouter,
}

impl DistributedEntailmentEngine {
    /// Creates a new distributed entailment engine.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::{DistributedEntailmentEngine, ShardRouter};
    ///
    /// let router = ShardRouter::new(4);
    /// let engine = DistributedEntailmentEngine::new(router);
    /// ```
    pub fn new(router: ShardRouter) -> Self {
        Self {
            shards: HashMap::new(),
            router,
        }
    }

    /// Adds a node to handle a specific shard.
    pub fn add_shard<N: DistributedNode + 'static>(&mut self, shard_id: ShardId, node: N) {
        self.shards.insert(shard_id, Arc::new(node));
    }

    /// Gets the shard responsible for a statute.
    pub fn get_shard(&self, statute_id: &str) -> Option<&Arc<dyn DistributedNode>> {
        let shard_id = self.router.assign_shard(statute_id);
        self.shards.get(&shard_id)
    }

    /// Returns the number of shards.
    pub fn shard_count(&self) -> usize {
        self.shards.len()
    }

    /// Queries all shards and aggregates results.
    pub fn query_all_shards(&self) -> Vec<NodeId> {
        self.shards
            .values()
            .filter(|node| node.is_available())
            .map(|node| node.node_id().clone())
            .collect()
    }
}

/// Cross-shard query coordinator.
pub struct CrossShardCoordinator {
    timeout_ms: u64,
}

impl CrossShardCoordinator {
    /// Creates a new cross-shard coordinator.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::distributed::CrossShardCoordinator;
    ///
    /// let coordinator = CrossShardCoordinator::new();
    /// assert_eq!(coordinator.timeout(), 5000);
    /// ```
    pub fn new() -> Self {
        Self { timeout_ms: 5000 }
    }

    /// Creates a coordinator with custom timeout.
    pub fn with_timeout(timeout_ms: u64) -> Self {
        Self { timeout_ms }
    }

    /// Gets the timeout in milliseconds.
    pub fn timeout(&self) -> u64 {
        self.timeout_ms
    }

    /// Coordinates a query across multiple shards (mock implementation).
    pub fn coordinate_query(&self, shard_ids: &[ShardId]) -> Vec<ShardId> {
        shard_ids.to_vec()
    }

    /// Aggregates results from multiple shards.
    pub fn aggregate_results<T>(&self, results: Vec<Vec<T>>) -> Vec<T> {
        results.into_iter().flatten().collect()
    }
}

impl Default for CrossShardCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Effect, EffectType};

    #[test]
    fn test_node_id() {
        let node_id = NodeId::new("node-1");
        assert_eq!(node_id.as_str(), "node-1");
    }

    #[test]
    fn test_vector_clock_increment() {
        let mut clock = VectorClock::new();
        let node = NodeId::new("node-1");
        clock.increment(&node);
        assert_eq!(clock.get(&node), 1);
        clock.increment(&node);
        assert_eq!(clock.get(&node), 2);
    }

    #[test]
    fn test_vector_clock_merge() {
        let mut clock1 = VectorClock::new();
        let mut clock2 = VectorClock::new();
        let node1 = NodeId::new("node-1");
        let node2 = NodeId::new("node-2");

        clock1.increment(&node1);
        clock1.increment(&node1);
        clock2.increment(&node2);

        clock1.merge(&clock2);
        assert_eq!(clock1.get(&node1), 2);
        assert_eq!(clock1.get(&node2), 1);
    }

    #[test]
    fn test_vector_clock_happens_before() {
        let mut clock1 = VectorClock::new();
        let mut clock2 = VectorClock::new();
        let node = NodeId::new("node-1");

        clock1.increment(&node);
        clock2.increment(&node);
        clock2.increment(&node);

        assert!(clock1.happens_before(&clock2));
        assert!(!clock2.happens_before(&clock1));
    }

    #[test]
    fn test_local_node() {
        let node = LocalNode::new(NodeId::new("node-1"));
        assert!(node.is_available());
        assert_eq!(node.node_id().as_str(), "node-1");
    }

    #[test]
    fn test_distributed_registry() {
        let node = LocalNode::new(NodeId::new("node-1"));
        let mut registry = DistributedRegistry::new(node);

        let statute = Statute::new(
            "law-1",
            "Tax Law",
            Effect::new(EffectType::Grant, "Tax credit"),
        );
        registry.add_local(statute);

        assert_eq!(registry.len(), 1);
        assert!(registry.get("law-1").is_some());
    }

    #[test]
    fn test_conflict_resolution() {
        let node1 = NodeId::new("node-1");
        let node2 = NodeId::new("node-2");

        let statute1 = Statute::new(
            "law-1",
            "Version 1",
            Effect::new(EffectType::Grant, "Benefit"),
        );
        let statute2 = Statute::new(
            "law-1",
            "Version 2",
            Effect::new(EffectType::Grant, "Better Benefit"),
        );

        let v1 = VersionedStatute::new(statute1, node1);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let v2 = VersionedStatute::new(statute2, node2);

        // v2 should win (later timestamp)
        let winner = VersionedStatute::resolve_conflict(&v1, &v2);
        assert_eq!(&winner.statute.title, "Version 2");
    }

    #[test]
    fn test_conflict_resolver() {
        let resolver = ConflictResolver::new(ConflictStrategy::LastWriteWins);
        let node1 = NodeId::new("node-1");
        let node2 = NodeId::new("node-2");

        let statute1 = Statute::new("law-1", "V1", Effect::new(EffectType::Grant, "Benefit"));
        let statute2 = Statute::new("law-1", "V2", Effect::new(EffectType::Grant, "Benefit"));

        let v1 = VersionedStatute::new(statute1, node1);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let v2 = VersionedStatute::new(statute2, node2);

        let winner = resolver.resolve(&v1, &v2);
        assert_eq!(&winner.statute.title, "V2");
    }

    #[test]
    fn test_gossip_protocol() {
        let gossip = GossipProtocol::new();
        assert_eq!(gossip.sync_interval(), 1000);
        assert_eq!(gossip.max_batch_size(), 100);

        let nodes = vec![
            NodeId::new("node-1"),
            NodeId::new("node-2"),
            NodeId::new("node-3"),
        ];
        let current = NodeId::new("node-1");
        let peers = gossip.select_peers(&nodes, &current, 2);
        assert!(peers.len() <= 2);
        assert!(!peers.contains(&current));
    }

    #[test]
    fn test_shard_router() {
        let router = ShardRouter::new(4);
        assert_eq!(router.num_shards(), 4);

        let shard1 = router.assign_shard("statute-1");
        let shard2 = router.assign_shard("statute-1");
        assert_eq!(shard1, shard2); // Same statute should go to same shard

        let shard3 = router.assign_shard("statute-2");
        // Different statutes may or may not go to different shards (depends on hash)
        assert!(shard3.as_u32() < 4);
    }

    #[test]
    fn test_distributed_entailment_engine() {
        let router = ShardRouter::new(4);
        let mut engine = DistributedEntailmentEngine::new(router);

        let node = LocalNode::new(NodeId::new("node-1"));
        engine.add_shard(ShardId::new(0), node);

        assert_eq!(engine.shard_count(), 1);
    }

    #[test]
    fn test_cross_shard_coordinator() {
        let coordinator = CrossShardCoordinator::new();
        assert_eq!(coordinator.timeout(), 5000);

        let shards = vec![ShardId::new(0), ShardId::new(1), ShardId::new(2)];
        let result = coordinator.coordinate_query(&shards);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_coordinator_aggregate() {
        let coordinator = CrossShardCoordinator::new();
        let results = vec![vec![1, 2, 3], vec![4, 5], vec![6]];
        let aggregated = coordinator.aggregate_results(results);
        assert_eq!(aggregated, vec![1, 2, 3, 4, 5, 6]);
    }
}
