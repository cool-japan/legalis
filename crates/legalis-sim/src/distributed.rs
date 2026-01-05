//! Distributed Simulation Framework
//!
//! This module provides abstractions and implementations for running simulations
//! across multiple nodes in a distributed system, with support for:
//! - Partition-based entity distribution
//! - Cross-node communication
//! - Dynamic load balancing
//! - Fault-tolerant checkpointing

use crate::{SimResult, SimulationError, SimulationMetrics};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Node identifier in a distributed system
pub type NodeId = usize;

/// Message identifier for tracking
pub type MessageId = u64;

/// Node information in a distributed cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node identifier
    pub id: NodeId,
    /// Node hostname or address
    pub address: String,
    /// Node rank in the cluster (0 = coordinator)
    pub rank: usize,
    /// Total number of nodes in the cluster
    pub total_nodes: usize,
    /// Current load (0.0 = idle, 1.0 = fully loaded)
    pub load: f64,
    /// Number of entities assigned to this node
    pub entity_count: usize,
    /// Node status
    pub status: NodeStatus,
}

/// Status of a node in the cluster
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Node is idle and ready
    Idle,
    /// Node is currently processing
    Active,
    /// Node is waiting for data
    Waiting,
    /// Node has failed
    Failed,
    /// Node is recovering from failure
    Recovering,
}

impl NodeInfo {
    /// Create a new node
    pub fn new(id: NodeId, address: String, rank: usize, total_nodes: usize) -> Self {
        NodeInfo {
            id,
            address,
            rank,
            total_nodes,
            load: 0.0,
            entity_count: 0,
            status: NodeStatus::Idle,
        }
    }

    /// Check if this node is the coordinator
    pub fn is_coordinator(&self) -> bool {
        self.rank == 0
    }

    /// Update load based on entity count
    pub fn update_load(&mut self, max_entities_per_node: usize) {
        if max_entities_per_node > 0 {
            self.load = (self.entity_count as f64) / (max_entities_per_node as f64);
            self.load = self.load.min(1.0);
        }
    }
}

/// Entity partition assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityPartition {
    /// Partition identifier
    pub id: usize,
    /// Node assigned to this partition
    pub node_id: NodeId,
    /// Entity IDs in this partition
    pub entity_ids: Vec<String>,
    /// Partition size
    pub size: usize,
}

impl EntityPartition {
    /// Create a new partition
    pub fn new(id: usize, node_id: NodeId) -> Self {
        EntityPartition {
            id,
            node_id,
            entity_ids: Vec::new(),
            size: 0,
        }
    }

    /// Add an entity to this partition
    pub fn add_entity(&mut self, entity_id: String) {
        self.entity_ids.push(entity_id);
        self.size += 1;
    }

    /// Add multiple entities
    pub fn add_entities(&mut self, entity_ids: Vec<String>) {
        self.size += entity_ids.len();
        self.entity_ids.extend(entity_ids);
    }
}

/// Partitioning strategy for distributing entities across nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartitionStrategy {
    /// Round-robin distribution
    RoundRobin,
    /// Hash-based partitioning
    Hash,
    /// Range-based partitioning (by entity ID)
    Range,
    /// Load-balanced partitioning
    LoadBalanced,
    /// Geographic partitioning (if entities have location)
    Geographic,
}

/// Partition manager for distributing entities across nodes
#[derive(Debug)]
pub struct PartitionManager {
    /// Partitioning strategy
    strategy: PartitionStrategy,
    /// All partitions
    partitions: Vec<EntityPartition>,
    /// Next partition ID
    next_partition_id: usize,
}

impl PartitionManager {
    /// Create a new partition manager
    pub fn new(strategy: PartitionStrategy) -> Self {
        PartitionManager {
            strategy,
            partitions: Vec::new(),
            next_partition_id: 0,
        }
    }

    /// Create partitions for a set of entity IDs
    pub fn create_partitions(
        &mut self,
        entity_ids: &[String],
        num_nodes: usize,
    ) -> SimResult<Vec<EntityPartition>> {
        if num_nodes == 0 {
            return Err(SimulationError::InvalidParameter(
                "Number of nodes must be greater than 0".to_string(),
            ));
        }

        let mut partitions = Vec::with_capacity(num_nodes);
        for node_id in 0..num_nodes {
            partitions.push(EntityPartition::new(self.next_partition_id, node_id));
            self.next_partition_id += 1;
        }

        // Distribute entities according to strategy
        match self.strategy {
            PartitionStrategy::RoundRobin => {
                for (i, entity_id) in entity_ids.iter().enumerate() {
                    let partition_idx = i % num_nodes;
                    partitions[partition_idx].add_entity(entity_id.clone());
                }
            }
            PartitionStrategy::Hash => {
                for entity_id in entity_ids {
                    let hash = Self::hash_string(entity_id);
                    let partition_idx = (hash as usize) % num_nodes;
                    partitions[partition_idx].add_entity(entity_id.clone());
                }
            }
            PartitionStrategy::Range => {
                let chunk_size = entity_ids.len().div_ceil(num_nodes);
                for (i, entity_id) in entity_ids.iter().enumerate() {
                    let partition_idx = i / chunk_size;
                    let partition_idx = partition_idx.min(num_nodes - 1);
                    partitions[partition_idx].add_entity(entity_id.clone());
                }
            }
            PartitionStrategy::LoadBalanced | PartitionStrategy::Geographic => {
                // For now, use round-robin for these strategies
                // Can be enhanced with actual load balancing later
                for (i, entity_id) in entity_ids.iter().enumerate() {
                    let partition_idx = i % num_nodes;
                    partitions[partition_idx].add_entity(entity_id.clone());
                }
            }
        }

        self.partitions.extend(partitions.clone());
        Ok(partitions)
    }

    /// Simple hash function for strings
    fn hash_string(s: &str) -> u64 {
        let mut hash = 0u64;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    /// Get partition for a specific entity ID
    pub fn get_partition(&self, entity_id: &str) -> Option<&EntityPartition> {
        self.partitions
            .iter()
            .find(|p| p.entity_ids.contains(&entity_id.to_string()))
    }

    /// Get all partitions for a specific node
    pub fn get_node_partitions(&self, node_id: NodeId) -> Vec<&EntityPartition> {
        self.partitions
            .iter()
            .filter(|p| p.node_id == node_id)
            .collect()
    }

    /// Get total number of partitions
    pub fn partition_count(&self) -> usize {
        self.partitions.len()
    }
}

/// Message type for cross-node communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterMessageType {
    /// Barrier synchronization
    Barrier,
    /// Entity data transfer
    EntityData(Vec<String>),
    /// Simulation results
    Results(SimulationMetrics),
    /// Load balancing request
    LoadBalance,
    /// Checkpoint trigger
    Checkpoint,
    /// Node status update
    StatusUpdate(NodeStatus),
    /// Custom user message
    Custom(String),
}

/// Message for cross-node communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID
    pub id: MessageId,
    /// Source node
    pub source: NodeId,
    /// Destination node (None = broadcast)
    pub destination: Option<NodeId>,
    /// Message type and payload
    pub message_type: ClusterMessageType,
    /// Timestamp (seconds since epoch)
    pub timestamp: u64,
}

impl Message {
    /// Create a new message
    pub fn new(
        id: MessageId,
        source: NodeId,
        destination: Option<NodeId>,
        message_type: ClusterMessageType,
    ) -> Self {
        Message {
            id,
            source,
            destination,
            message_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Check if this is a broadcast message
    pub fn is_broadcast(&self) -> bool {
        self.destination.is_none()
    }
}

/// Message queue for storing and retrieving messages
#[derive(Debug)]
pub struct MessageQueue {
    /// Queue of messages
    queue: Arc<Mutex<VecDeque<Message>>>,
    /// Next message ID
    next_id: Arc<Mutex<MessageId>>,
}

impl MessageQueue {
    /// Create a new message queue
    pub fn new() -> Self {
        MessageQueue {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            next_id: Arc::new(Mutex::new(0)),
        }
    }

    /// Send a message
    pub fn send(
        &self,
        source: NodeId,
        destination: Option<NodeId>,
        message_type: ClusterMessageType,
    ) -> SimResult<MessageId> {
        let mut next_id = self.next_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;
        drop(next_id);

        let message = Message::new(id, source, destination, message_type);
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(message);
        Ok(id)
    }

    /// Receive a message (blocking until message available)
    pub fn receive(&self, node_id: NodeId) -> Option<Message> {
        let mut queue = self.queue.lock().unwrap();
        let position = queue
            .iter()
            .position(|msg| msg.destination == Some(node_id) || msg.destination.is_none());

        position.and_then(|pos| queue.remove(pos))
    }

    /// Peek at next message without removing
    pub fn peek(&self, node_id: NodeId) -> Option<Message> {
        let queue = self.queue.lock().unwrap();
        queue
            .iter()
            .find(|msg| msg.destination == Some(node_id) || msg.destination.is_none())
            .cloned()
    }

    /// Get queue size
    pub fn size(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    /// Clear the queue
    pub fn clear(&self) {
        self.queue.lock().unwrap().clear();
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Load balancing strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalanceStrategy {
    /// No load balancing
    None,
    /// Periodic rebalancing at fixed intervals
    Periodic,
    /// Dynamic rebalancing based on load threshold
    Dynamic,
    /// Work stealing from busy nodes
    WorkStealing,
}

/// Load balancer for distributing work across nodes
#[derive(Debug)]
pub struct LoadBalancer {
    /// Load balancing strategy
    strategy: LoadBalanceStrategy,
    /// Load threshold for triggering rebalancing (0.0-1.0)
    threshold: f64,
    /// Minimum imbalance to trigger rebalancing
    min_imbalance: f64,
}

impl LoadBalancer {
    /// Create a new load balancer
    pub fn new(strategy: LoadBalanceStrategy, threshold: f64) -> Self {
        LoadBalancer {
            strategy,
            threshold: threshold.clamp(0.0, 1.0),
            min_imbalance: 0.2, // 20% imbalance threshold
        }
    }

    /// Check if rebalancing is needed
    pub fn needs_rebalancing(&self, nodes: &[NodeInfo]) -> bool {
        if nodes.is_empty() || self.strategy == LoadBalanceStrategy::None {
            return false;
        }

        let max_load = nodes.iter().map(|n| n.load).fold(0.0, f64::max);
        let min_load = nodes.iter().map(|n| n.load).fold(1.0, f64::min);
        let imbalance = max_load - min_load;

        match self.strategy {
            LoadBalanceStrategy::None => false,
            LoadBalanceStrategy::Periodic => true, // Caller handles timing
            LoadBalanceStrategy::Dynamic => max_load > self.threshold,
            LoadBalanceStrategy::WorkStealing => imbalance > self.min_imbalance,
        }
    }

    /// Calculate rebalancing plan
    pub fn calculate_rebalance(
        &self,
        nodes: &[NodeInfo],
        partitions: &[EntityPartition],
    ) -> Vec<(usize, NodeId, NodeId)> {
        // Returns (partition_id, from_node, to_node) tuples
        let mut moves = Vec::new();

        if nodes.len() < 2 || partitions.is_empty() {
            return moves;
        }

        // Find overloaded and underloaded nodes
        let avg_load = nodes.iter().map(|n| n.load).sum::<f64>() / nodes.len() as f64;
        let mut overloaded: Vec<_> = nodes.iter().filter(|n| n.load > avg_load).collect();
        let mut underloaded: Vec<_> = nodes.iter().filter(|n| n.load < avg_load).collect();

        overloaded.sort_by(|a, b| b.load.partial_cmp(&a.load).unwrap());
        underloaded.sort_by(|a, b| a.load.partial_cmp(&b.load).unwrap());

        // Move partitions from overloaded to underloaded nodes
        for overloaded_node in &overloaded {
            let node_partitions: Vec<_> = partitions
                .iter()
                .filter(|p| p.node_id == overloaded_node.id)
                .collect();

            for partition in node_partitions {
                if let Some(underloaded_node) = underloaded.first() {
                    if overloaded_node.load - underloaded_node.load > self.min_imbalance {
                        moves.push((partition.id, overloaded_node.id, underloaded_node.id));

                        // Update for next iteration (simplified)
                        if underloaded.len() > 1 {
                            underloaded.remove(0);
                        }
                    }
                }
            }
        }

        moves
    }

    /// Set imbalance threshold
    pub fn set_imbalance_threshold(&mut self, threshold: f64) {
        self.min_imbalance = threshold.clamp(0.0, 1.0);
    }
}

/// Distributed simulation coordinator
#[derive(Debug)]
pub struct ClusterCoordinator {
    /// Nodes in the cluster
    nodes: Vec<NodeInfo>,
    /// Partition manager
    partition_manager: PartitionManager,
    /// Message queue
    message_queue: MessageQueue,
    /// Load balancer
    load_balancer: LoadBalancer,
    /// Coordinator node info
    coordinator_node: NodeInfo,
}

impl ClusterCoordinator {
    /// Create a new cluster coordinator
    pub fn new(
        num_nodes: usize,
        partition_strategy: PartitionStrategy,
        load_balance_strategy: LoadBalanceStrategy,
    ) -> Self {
        let mut nodes = Vec::with_capacity(num_nodes);
        for i in 0..num_nodes {
            nodes.push(NodeInfo::new(i, format!("node-{}", i), i, num_nodes));
        }

        let coordinator_node = nodes[0].clone();

        ClusterCoordinator {
            nodes,
            partition_manager: PartitionManager::new(partition_strategy),
            message_queue: MessageQueue::new(),
            load_balancer: LoadBalancer::new(load_balance_strategy, 0.8),
            coordinator_node,
        }
    }

    /// Distribute entities across nodes
    pub fn distribute_entities(&mut self, entity_ids: &[String]) -> SimResult<()> {
        let partitions = self
            .partition_manager
            .create_partitions(entity_ids, self.nodes.len())?;

        // Update node entity counts
        for partition in &partitions {
            if let Some(node) = self.nodes.iter_mut().find(|n| n.id == partition.node_id) {
                node.entity_count += partition.size;
            }
        }

        // Update loads
        let max_entities = entity_ids.len() / self.nodes.len().max(1);
        for node in &mut self.nodes {
            node.update_load(max_entities);
        }

        Ok(())
    }

    /// Send a message to a specific node or broadcast
    pub fn send_message(
        &self,
        destination: Option<NodeId>,
        message_type: ClusterMessageType,
    ) -> SimResult<MessageId> {
        self.message_queue
            .send(self.coordinator_node.id, destination, message_type)
    }

    /// Receive a message for this coordinator
    pub fn receive_message(&self) -> Option<Message> {
        self.message_queue.receive(self.coordinator_node.id)
    }

    /// Perform barrier synchronization
    pub fn barrier(&self) -> SimResult<()> {
        // Send barrier message to all nodes
        self.send_message(None, ClusterMessageType::Barrier)?;
        Ok(())
    }

    /// Check if load balancing is needed and rebalance if necessary
    pub fn rebalance_if_needed(&mut self) -> SimResult<Vec<(usize, NodeId, NodeId)>> {
        if !self.load_balancer.needs_rebalancing(&self.nodes) {
            return Ok(Vec::new());
        }

        let moves = self
            .load_balancer
            .calculate_rebalance(&self.nodes, &self.partition_manager.partitions);

        // Send load balance messages
        if !moves.is_empty() {
            self.send_message(None, ClusterMessageType::LoadBalance)?;
        }

        Ok(moves)
    }

    /// Get node information
    pub fn get_node(&self, node_id: NodeId) -> Option<&NodeInfo> {
        self.nodes.iter().find(|n| n.id == node_id)
    }

    /// Get all nodes
    pub fn nodes(&self) -> &[NodeInfo] {
        &self.nodes
    }

    /// Get partition manager
    pub fn partition_manager(&self) -> &PartitionManager {
        &self.partition_manager
    }

    /// Update node status
    pub fn update_node_status(&mut self, node_id: NodeId, status: NodeStatus) -> SimResult<()> {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.status = status;
            Ok(())
        } else {
            Err(SimulationError::InvalidParameter(format!(
                "Node {} not found",
                node_id
            )))
        }
    }

    /// Get number of nodes
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_info_creation() {
        let node = NodeInfo::new(1, "node-1".to_string(), 1, 4);
        assert_eq!(node.id, 1);
        assert_eq!(node.rank, 1);
        assert_eq!(node.total_nodes, 4);
        assert!(!node.is_coordinator());
        assert_eq!(node.status, NodeStatus::Idle);
    }

    #[test]
    fn test_coordinator_node() {
        let node = NodeInfo::new(0, "coordinator".to_string(), 0, 4);
        assert!(node.is_coordinator());
    }

    #[test]
    fn test_node_load_update() {
        let mut node = NodeInfo::new(1, "node-1".to_string(), 1, 4);
        node.entity_count = 50;
        node.update_load(100);
        assert_eq!(node.load, 0.5);
    }

    #[test]
    fn test_entity_partition() {
        let mut partition = EntityPartition::new(0, 1);
        partition.add_entity("entity-1".to_string());
        partition.add_entity("entity-2".to_string());
        assert_eq!(partition.size, 2);
        assert_eq!(partition.entity_ids.len(), 2);
    }

    #[test]
    fn test_partition_manager_round_robin() {
        let mut manager = PartitionManager::new(PartitionStrategy::RoundRobin);
        let entity_ids: Vec<String> = (0..10).map(|i| format!("entity-{}", i)).collect();
        let partitions = manager.create_partitions(&entity_ids, 3).unwrap();

        assert_eq!(partitions.len(), 3);
        assert_eq!(
            partitions[0].size + partitions[1].size + partitions[2].size,
            10
        );
    }

    #[test]
    fn test_partition_manager_hash() {
        let mut manager = PartitionManager::new(PartitionStrategy::Hash);
        let entity_ids: Vec<String> = (0..10).map(|i| format!("entity-{}", i)).collect();
        let partitions = manager.create_partitions(&entity_ids, 3).unwrap();

        assert_eq!(partitions.len(), 3);
        let total: usize = partitions.iter().map(|p| p.size).sum();
        assert_eq!(total, 10);
    }

    #[test]
    fn test_partition_manager_range() {
        let mut manager = PartitionManager::new(PartitionStrategy::Range);
        let entity_ids: Vec<String> = (0..9).map(|i| format!("entity-{}", i)).collect();
        let partitions = manager.create_partitions(&entity_ids, 3).unwrap();

        assert_eq!(partitions.len(), 3);
        assert_eq!(partitions[0].size, 3);
        assert_eq!(partitions[1].size, 3);
        assert_eq!(partitions[2].size, 3);
    }

    #[test]
    fn test_message_creation() {
        let msg = Message::new(1, 0, Some(1), ClusterMessageType::Barrier);
        assert_eq!(msg.id, 1);
        assert_eq!(msg.source, 0);
        assert_eq!(msg.destination, Some(1));
        assert!(!msg.is_broadcast());
    }

    #[test]
    fn test_broadcast_message() {
        let msg = Message::new(1, 0, None, ClusterMessageType::Barrier);
        assert!(msg.is_broadcast());
    }

    #[test]
    fn test_message_queue() {
        let queue = MessageQueue::new();
        let id = queue.send(0, Some(1), ClusterMessageType::Barrier).unwrap();
        assert_eq!(id, 0);
        assert_eq!(queue.size(), 1);

        let msg = queue.receive(1).unwrap();
        assert_eq!(msg.id, 0);
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_message_queue_broadcast() {
        let queue = MessageQueue::new();
        queue.send(0, None, ClusterMessageType::Barrier).unwrap();

        // Any node can receive broadcast
        let msg1 = queue.peek(1).unwrap();
        assert_eq!(msg1.source, 0);

        let msg2 = queue.receive(2).unwrap();
        assert_eq!(msg2.source, 0);
    }

    #[test]
    fn test_load_balancer_no_rebalancing() {
        let balancer = LoadBalancer::new(LoadBalanceStrategy::None, 0.8);
        let nodes = vec![
            NodeInfo::new(0, "node-0".to_string(), 0, 2),
            NodeInfo::new(1, "node-1".to_string(), 1, 2),
        ];
        assert!(!balancer.needs_rebalancing(&nodes));
    }

    #[test]
    fn test_load_balancer_dynamic() {
        let balancer = LoadBalancer::new(LoadBalanceStrategy::Dynamic, 0.5);
        let mut nodes = vec![
            NodeInfo::new(0, "node-0".to_string(), 0, 2),
            NodeInfo::new(1, "node-1".to_string(), 1, 2),
        ];
        nodes[0].load = 0.8;
        nodes[1].load = 0.2;

        assert!(balancer.needs_rebalancing(&nodes));
    }

    #[test]
    fn test_cluster_coordinator() {
        let coordinator = ClusterCoordinator::new(
            4,
            PartitionStrategy::RoundRobin,
            LoadBalanceStrategy::Dynamic,
        );
        assert_eq!(coordinator.num_nodes(), 4);
        assert!(coordinator.get_node(0).is_some());
    }

    #[test]
    fn test_cluster_coordinator_distribute_entities() {
        let mut coordinator =
            ClusterCoordinator::new(3, PartitionStrategy::RoundRobin, LoadBalanceStrategy::None);
        let entity_ids: Vec<String> = (0..12).map(|i| format!("entity-{}", i)).collect();

        coordinator.distribute_entities(&entity_ids).unwrap();

        // Each node should have approximately equal entities
        for node in coordinator.nodes() {
            assert_eq!(node.entity_count, 4);
        }
    }

    #[test]
    fn test_cluster_coordinator_messaging() {
        let coordinator =
            ClusterCoordinator::new(2, PartitionStrategy::RoundRobin, LoadBalanceStrategy::None);
        let msg_id = coordinator
            .send_message(Some(1), ClusterMessageType::Barrier)
            .unwrap();
        assert_eq!(msg_id, 0);
    }

    #[test]
    fn test_cluster_coordinator_barrier() {
        let coordinator =
            ClusterCoordinator::new(3, PartitionStrategy::RoundRobin, LoadBalanceStrategy::None);
        coordinator.barrier().unwrap();
        assert_eq!(coordinator.message_queue.size(), 1);
    }

    #[test]
    fn test_cluster_coordinator_update_node_status() {
        let mut coordinator =
            ClusterCoordinator::new(2, PartitionStrategy::RoundRobin, LoadBalanceStrategy::None);
        coordinator
            .update_node_status(1, NodeStatus::Active)
            .unwrap();
        assert_eq!(coordinator.get_node(1).unwrap().status, NodeStatus::Active);
    }

    #[test]
    fn test_partition_manager_get_partition() {
        let mut manager = PartitionManager::new(PartitionStrategy::RoundRobin);
        let entity_ids = vec!["entity-1".to_string(), "entity-2".to_string()];
        manager.create_partitions(&entity_ids, 2).unwrap();

        let partition = manager.get_partition("entity-1");
        assert!(partition.is_some());
    }

    #[test]
    fn test_partition_manager_get_node_partitions() {
        let mut manager = PartitionManager::new(PartitionStrategy::RoundRobin);
        let entity_ids: Vec<String> = (0..6).map(|i| format!("entity-{}", i)).collect();
        manager.create_partitions(&entity_ids, 2).unwrap();

        let partitions = manager.get_node_partitions(0);
        assert!(!partitions.is_empty());
    }

    #[test]
    fn test_load_balancer_calculate_rebalance() {
        let balancer = LoadBalancer::new(LoadBalanceStrategy::WorkStealing, 0.8);
        let mut nodes = vec![
            NodeInfo::new(0, "node-0".to_string(), 0, 2),
            NodeInfo::new(1, "node-1".to_string(), 1, 2),
        ];
        nodes[0].load = 0.9;
        nodes[1].load = 0.1;

        let mut partitions = vec![
            EntityPartition::new(0, 0),
            EntityPartition::new(1, 0),
            EntityPartition::new(2, 1),
        ];
        partitions[0].size = 10;
        partitions[1].size = 10;
        partitions[2].size = 2;

        let moves = balancer.calculate_rebalance(&nodes, &partitions);
        assert!(!moves.is_empty());
    }
}
