//! Multi-node audit synchronization for distributed audit trails
//!
//! This module provides functionality for synchronizing audit records across
//! multiple nodes in a distributed system, ensuring consistency and availability.

use crate::{AuditRecord, storage::AuditStorage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

pub mod aggregation;
pub mod consensus;
pub mod sync;

/// Errors that can occur during distributed operations
#[derive(Debug, Error)]
pub enum DistributedError {
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Synchronization failed: {0}")]
    SyncFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Consensus not reached: {0}")]
    ConsensusError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Invalid node state: {0}")]
    InvalidState(String),
}

/// Node identifier in the distributed system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

/// Status of a node in the distributed system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Node is active and reachable
    Active,
    /// Node is temporarily unavailable
    Unavailable,
    /// Node has failed
    Failed,
    /// Node is synchronizing
    Syncing,
}

/// Information about a node in the distributed system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: NodeId,
    pub address: String,
    pub region: String,
    pub status: NodeStatus,
    pub last_seen: DateTime<Utc>,
    pub record_count: usize,
    pub last_hash: Option<String>,
}

impl NodeInfo {
    pub fn new(id: NodeId, address: String, region: String) -> Self {
        Self {
            id,
            address,
            region,
            status: NodeStatus::Active,
            last_seen: Utc::now(),
            record_count: 0,
            last_hash: None,
        }
    }
}

/// Vector clock for causality tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VectorClock {
    clocks: HashMap<NodeId, u64>,
}

impl VectorClock {
    pub fn new() -> Self {
        Self {
            clocks: HashMap::new(),
        }
    }

    /// Increment the clock for a given node
    pub fn increment(&mut self, node_id: &NodeId) {
        *self.clocks.entry(node_id.clone()).or_insert(0) += 1;
    }

    /// Merge with another vector clock (taking maximum of each component)
    pub fn merge(&mut self, other: &VectorClock) {
        for (node_id, &clock) in &other.clocks {
            let entry = self.clocks.entry(node_id.clone()).or_insert(0);
            *entry = (*entry).max(clock);
        }
    }

    /// Check if this clock happens-before another clock
    pub fn happens_before(&self, other: &VectorClock) -> bool {
        let mut strict_less = false;

        // Check all nodes in self
        for (node_id, &self_clock) in &self.clocks {
            let other_clock = other.clocks.get(node_id).copied().unwrap_or(0);
            if self_clock > other_clock {
                return false;
            }
            if self_clock < other_clock {
                strict_less = true;
            }
        }

        // Check all nodes in other that aren't in self
        for (node_id, &other_clock) in &other.clocks {
            if !self.clocks.contains_key(node_id) && other_clock > 0 {
                strict_less = true;
            }
        }

        strict_less
    }

    /// Check if two clocks are concurrent (neither happens-before the other)
    pub fn is_concurrent(&self, other: &VectorClock) -> bool {
        !self.happens_before(other) && !other.happens_before(self)
    }

    /// Get the clock value for a specific node
    pub fn get(&self, node_id: &NodeId) -> u64 {
        self.clocks.get(node_id).copied().unwrap_or(0)
    }
}

impl Default for VectorClock {
    fn default() -> Self {
        Self::new()
    }
}

/// Audit record with distributed metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedRecord {
    pub record: AuditRecord,
    pub origin_node: NodeId,
    pub vector_clock: VectorClock,
    pub replicated_to: HashSet<NodeId>,
}

impl DistributedRecord {
    pub fn new(record: AuditRecord, origin_node: NodeId, vector_clock: VectorClock) -> Self {
        Self {
            record,
            origin_node,
            vector_clock,
            replicated_to: HashSet::new(),
        }
    }

    /// Mark this record as replicated to a node
    pub fn mark_replicated(&mut self, node_id: NodeId) {
        self.replicated_to.insert(node_id);
    }

    /// Check if this record has been replicated to a node
    pub fn is_replicated_to(&self, node_id: &NodeId) -> bool {
        self.replicated_to.contains(node_id)
    }
}

/// Manager for distributed audit trail nodes
pub struct DistributedAuditManager {
    node_id: NodeId,
    nodes: Arc<RwLock<HashMap<NodeId, NodeInfo>>>,
    vector_clock: Arc<RwLock<VectorClock>>,
    #[allow(dead_code)]
    storage: Arc<dyn AuditStorage>,
}

impl DistributedAuditManager {
    /// Create a new distributed audit manager
    pub fn new(node_id: NodeId, storage: Arc<dyn AuditStorage>) -> Self {
        let mut nodes = HashMap::new();
        let node_info = NodeInfo::new(
            node_id.clone(),
            "localhost:8080".to_string(),
            "default".to_string(),
        );
        nodes.insert(node_id.clone(), node_info);

        Self {
            node_id: node_id.clone(),
            nodes: Arc::new(RwLock::new(nodes)),
            vector_clock: Arc::new(RwLock::new(VectorClock::new())),
            storage,
        }
    }

    /// Get the local node ID
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Register a new node in the cluster
    pub fn register_node(&self, node_info: NodeInfo) -> Result<(), DistributedError> {
        let mut nodes = self.nodes.write().map_err(|e| {
            DistributedError::InvalidState(format!("Failed to acquire write lock: {}", e))
        })?;
        nodes.insert(node_info.id.clone(), node_info);
        Ok(())
    }

    /// Get information about a specific node
    pub fn get_node(&self, node_id: &NodeId) -> Result<NodeInfo, DistributedError> {
        let nodes = self.nodes.read().map_err(|e| {
            DistributedError::InvalidState(format!("Failed to acquire read lock: {}", e))
        })?;
        nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| DistributedError::NodeNotFound(node_id.0.clone()))
    }

    /// Get all nodes in the cluster
    pub fn get_all_nodes(&self) -> Result<Vec<NodeInfo>, DistributedError> {
        let nodes = self.nodes.read().map_err(|e| {
            DistributedError::InvalidState(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(nodes.values().cloned().collect())
    }

    /// Update the status of a node
    pub fn update_node_status(
        &self,
        node_id: &NodeId,
        status: NodeStatus,
    ) -> Result<(), DistributedError> {
        let mut nodes = self.nodes.write().map_err(|e| {
            DistributedError::InvalidState(format!("Failed to acquire write lock: {}", e))
        })?;

        let node = nodes
            .get_mut(node_id)
            .ok_or_else(|| DistributedError::NodeNotFound(node_id.0.clone()))?;

        node.status = status;
        node.last_seen = Utc::now();
        Ok(())
    }

    /// Remove a node from the cluster
    pub fn remove_node(&self, node_id: &NodeId) -> Result<(), DistributedError> {
        let mut nodes = self.nodes.write().map_err(|e| {
            DistributedError::InvalidState(format!("Failed to acquire write lock: {}", e))
        })?;
        nodes.remove(node_id);
        Ok(())
    }

    /// Get the current vector clock
    pub fn get_vector_clock(&self) -> Result<VectorClock, DistributedError> {
        let clock = self.vector_clock.read().map_err(|e| {
            DistributedError::InvalidState(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(clock.clone())
    }

    /// Increment the local vector clock
    pub fn increment_clock(&self) -> Result<(), DistributedError> {
        let mut clock = self.vector_clock.write().map_err(|e| {
            DistributedError::InvalidState(format!("Failed to acquire write lock: {}", e))
        })?;
        clock.increment(&self.node_id);
        Ok(())
    }

    /// Merge a remote vector clock with the local one
    pub fn merge_clock(&self, remote_clock: &VectorClock) -> Result<(), DistributedError> {
        let mut clock = self.vector_clock.write().map_err(|e| {
            DistributedError::InvalidState(format!("Failed to acquire write lock: {}", e))
        })?;
        clock.merge(remote_clock);
        Ok(())
    }

    /// Get count of active nodes
    pub fn active_node_count(&self) -> Result<usize, DistributedError> {
        let nodes = self.nodes.read().map_err(|e| {
            DistributedError::InvalidState(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(nodes
            .values()
            .filter(|n| n.status == NodeStatus::Active)
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::memory::MemoryStorage;

    #[test]
    fn test_node_id_creation() {
        let id1 = NodeId::new("node-1");
        let id2 = NodeId::generate();

        assert_eq!(id1.0, "node-1");
        assert!(!id2.0.is_empty());
    }

    #[test]
    fn test_node_info_creation() {
        let node_id = NodeId::new("node-1");
        let node_info = NodeInfo::new(
            node_id.clone(),
            "127.0.0.1:8080".to_string(),
            "us-east-1".to_string(),
        );

        assert_eq!(node_info.id, node_id);
        assert_eq!(node_info.address, "127.0.0.1:8080");
        assert_eq!(node_info.region, "us-east-1");
        assert_eq!(node_info.status, NodeStatus::Active);
        assert_eq!(node_info.record_count, 0);
        assert_eq!(node_info.last_hash, None);
    }

    #[test]
    fn test_vector_clock_increment() {
        let mut clock = VectorClock::new();
        let node1 = NodeId::new("node-1");
        let node2 = NodeId::new("node-2");

        clock.increment(&node1);
        assert_eq!(clock.get(&node1), 1);
        assert_eq!(clock.get(&node2), 0);

        clock.increment(&node1);
        clock.increment(&node2);
        assert_eq!(clock.get(&node1), 2);
        assert_eq!(clock.get(&node2), 1);
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
        let node1 = NodeId::new("node-1");

        clock1.increment(&node1);
        clock2.increment(&node1);
        clock2.increment(&node1);

        assert!(clock1.happens_before(&clock2));
        assert!(!clock2.happens_before(&clock1));
    }

    #[test]
    fn test_vector_clock_concurrent() {
        let mut clock1 = VectorClock::new();
        let mut clock2 = VectorClock::new();
        let node1 = NodeId::new("node-1");
        let node2 = NodeId::new("node-2");

        clock1.increment(&node1);
        clock2.increment(&node2);

        assert!(clock1.is_concurrent(&clock2));
        assert!(clock2.is_concurrent(&clock1));
    }

    #[test]
    fn test_distributed_audit_manager() {
        let node_id = NodeId::new("node-1");
        let storage = Arc::new(MemoryStorage::new());
        let manager = DistributedAuditManager::new(node_id.clone(), storage);

        assert_eq!(manager.node_id(), &node_id);
        assert_eq!(manager.active_node_count().unwrap(), 1);
    }

    #[test]
    fn test_register_node() {
        let node_id = NodeId::new("node-1");
        let storage = Arc::new(MemoryStorage::new());
        let manager = DistributedAuditManager::new(node_id, storage);

        let new_node = NodeInfo::new(
            NodeId::new("node-2"),
            "127.0.0.1:8081".to_string(),
            "us-west-1".to_string(),
        );

        manager.register_node(new_node.clone()).unwrap();

        let retrieved = manager.get_node(&NodeId::new("node-2")).unwrap();
        assert_eq!(retrieved.id, new_node.id);
        assert_eq!(retrieved.address, new_node.address);
    }

    #[test]
    fn test_update_node_status() {
        let node_id = NodeId::new("node-1");
        let storage = Arc::new(MemoryStorage::new());
        let manager = DistributedAuditManager::new(node_id.clone(), storage);

        manager
            .update_node_status(&node_id, NodeStatus::Syncing)
            .unwrap();

        let node = manager.get_node(&node_id).unwrap();
        assert_eq!(node.status, NodeStatus::Syncing);
    }

    #[test]
    fn test_remove_node() {
        let node_id = NodeId::new("node-1");
        let storage = Arc::new(MemoryStorage::new());
        let manager = DistributedAuditManager::new(node_id.clone(), storage);

        let new_node = NodeInfo::new(
            NodeId::new("node-2"),
            "127.0.0.1:8081".to_string(),
            "us-west-1".to_string(),
        );

        manager.register_node(new_node).unwrap();
        assert_eq!(manager.get_all_nodes().unwrap().len(), 2);

        manager.remove_node(&NodeId::new("node-2")).unwrap();
        assert_eq!(manager.get_all_nodes().unwrap().len(), 1);
    }

    #[test]
    fn test_distributed_record() {
        use crate::{Actor, EventType};

        let record = AuditRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::AutomaticDecision,
            statute_id: "GDPR-Art6".to_string(),
            subject_id: Uuid::new_v4(),
            actor: Actor::System {
                component: "test".to_string(),
            },
            context: crate::DecisionContext::default(),
            result: crate::DecisionResult::Deterministic {
                effect_applied: "allowed".to_string(),
                parameters: std::collections::HashMap::new(),
            },
            previous_hash: None,
            record_hash: "hash123".to_string(),
        };

        let node_id = NodeId::new("node-1");
        let mut clock = VectorClock::new();
        clock.increment(&node_id);

        let mut dist_record = DistributedRecord::new(record.clone(), node_id.clone(), clock);

        assert_eq!(dist_record.origin_node, node_id);
        assert!(!dist_record.is_replicated_to(&NodeId::new("node-2")));

        dist_record.mark_replicated(NodeId::new("node-2"));
        assert!(dist_record.is_replicated_to(&NodeId::new("node-2")));
    }
}
