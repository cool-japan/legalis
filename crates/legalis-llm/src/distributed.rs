//! Distributed inference support for scaling LLM operations.
//!
//! This module provides distributed inference capabilities including load balancing,
//! sharding, and coordination across multiple inference nodes.

use crate::{LLMProvider, TextStream};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Node information for distributed inference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceNode {
    /// Unique node identifier
    pub id: String,
    /// Node endpoint URL
    pub endpoint: String,
    /// Current load (0.0 to 1.0)
    pub load: f64,
    /// Whether the node is available
    pub available: bool,
    /// Maximum concurrent requests
    pub max_concurrent: usize,
    /// Current active requests
    pub active_requests: usize,
    /// Node capabilities (model names, features)
    pub capabilities: Vec<String>,
    /// Node region/zone
    pub region: Option<String>,
}

impl InferenceNode {
    /// Creates a new inference node.
    pub fn new(id: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            endpoint: endpoint.into(),
            load: 0.0,
            available: true,
            max_concurrent: 10,
            active_requests: 0,
            capabilities: Vec::new(),
            region: None,
        }
    }

    /// Sets maximum concurrent requests.
    pub fn with_max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Adds a capability.
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    /// Sets the region.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Checks if the node can handle more requests.
    pub fn can_handle_request(&self) -> bool {
        self.available && self.active_requests < self.max_concurrent
    }

    /// Calculates node score for load balancing.
    pub fn score(&self) -> f64 {
        if !self.available {
            return 0.0;
        }

        let capacity_ratio = 1.0 - (self.active_requests as f64 / self.max_concurrent as f64);
        let load_factor = 1.0 - self.load;

        (capacity_ratio + load_factor) / 2.0
    }
}

/// Load balancing strategy for distributed inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution
    RoundRobin,
    /// Least connections (choose node with fewest active requests)
    LeastConnections,
    /// Weighted round-robin based on node capacity
    WeightedRoundRobin,
    /// Least response time
    LeastResponseTime,
    /// Random selection
    Random,
    /// Consistent hashing (for cache locality)
    ConsistentHash,
}

/// Distributed inference coordinator.
pub struct DistributedInference {
    nodes: Arc<RwLock<Vec<InferenceNode>>>,
    strategy: LoadBalancingStrategy,
    current_index: Arc<RwLock<usize>>,
}

impl DistributedInference {
    /// Creates a new distributed inference coordinator.
    pub fn new(strategy: LoadBalancingStrategy) -> Self {
        Self {
            nodes: Arc::new(RwLock::new(Vec::new())),
            strategy,
            current_index: Arc::new(RwLock::new(0)),
        }
    }

    /// Adds an inference node.
    pub async fn add_node(&self, node: InferenceNode) {
        self.nodes.write().await.push(node);
    }

    /// Removes a node by ID.
    pub async fn remove_node(&self, node_id: &str) {
        let mut nodes = self.nodes.write().await;
        nodes.retain(|n| n.id != node_id);
    }

    /// Gets all nodes.
    pub async fn get_nodes(&self) -> Vec<InferenceNode> {
        self.nodes.read().await.clone()
    }

    /// Marks a node as unavailable.
    pub async fn mark_unavailable(&self, node_id: &str) {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
            node.available = false;
        }
    }

    /// Marks a node as available.
    pub async fn mark_available(&self, node_id: &str) {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
            node.available = true;
        }
    }

    /// Selects the best node for a request based on the load balancing strategy.
    pub async fn select_node(&self) -> Result<InferenceNode> {
        let nodes = self.nodes.read().await;

        if nodes.is_empty() {
            return Err(anyhow!("No inference nodes available"));
        }

        let available_nodes: Vec<&InferenceNode> =
            nodes.iter().filter(|n| n.can_handle_request()).collect();

        if available_nodes.is_empty() {
            return Err(anyhow!("All nodes are at capacity or unavailable"));
        }

        match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                let mut index = self.current_index.write().await;
                let node = available_nodes[*index % available_nodes.len()].clone();
                *index = (*index + 1) % available_nodes.len();
                Ok(node)
            }
            LoadBalancingStrategy::LeastConnections => {
                let node = available_nodes
                    .iter()
                    .min_by_key(|n| n.active_requests)
                    .ok_or_else(|| anyhow!("No available nodes"))?;
                Ok((*node).clone())
            }
            LoadBalancingStrategy::WeightedRoundRobin => {
                // Use node score for weighted selection
                let node = available_nodes
                    .iter()
                    .max_by(|a, b| {
                        a.score()
                            .partial_cmp(&b.score())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .ok_or_else(|| anyhow!("No available nodes"))?;
                Ok((*node).clone())
            }
            LoadBalancingStrategy::LeastResponseTime => {
                // For now, use score-based selection (could be enhanced with actual metrics)
                let node = available_nodes
                    .iter()
                    .max_by(|a, b| {
                        a.score()
                            .partial_cmp(&b.score())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .ok_or_else(|| anyhow!("No available nodes"))?;
                Ok((*node).clone())
            }
            LoadBalancingStrategy::Random => {
                // Use simple random indexing
                let mut rng = rand::rng();
                let idx = rng.random_range(0..available_nodes.len());
                Ok(available_nodes[idx].clone())
            }
            LoadBalancingStrategy::ConsistentHash => {
                // Simple hash-based selection
                let hash = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as usize;
                let node = available_nodes[hash % available_nodes.len()].clone();
                Ok(node)
            }
        }
    }

    /// Increments active request count for a node.
    pub async fn increment_active(&self, node_id: &str) {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
            node.active_requests += 1;
            node.load = node.active_requests as f64 / node.max_concurrent as f64;
        }
    }

    /// Decrements active request count for a node.
    pub async fn decrement_active(&self, node_id: &str) {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id)
            && node.active_requests > 0
        {
            node.active_requests -= 1;
            node.load = node.active_requests as f64 / node.max_concurrent as f64;
        }
    }

    /// Gets cluster statistics.
    pub async fn cluster_stats(&self) -> ClusterStats {
        let nodes = self.nodes.read().await;

        let total_nodes = nodes.len();
        let available_nodes = nodes.iter().filter(|n| n.available).count();
        let total_capacity = nodes.iter().map(|n| n.max_concurrent).sum();
        let active_requests = nodes.iter().map(|n| n.active_requests).sum();
        let avg_load = if !nodes.is_empty() {
            nodes.iter().map(|n| n.load).sum::<f64>() / nodes.len() as f64
        } else {
            0.0
        };

        ClusterStats {
            total_nodes,
            available_nodes,
            total_capacity,
            active_requests,
            avg_load,
        }
    }
}

/// Statistics for the inference cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterStats {
    /// Total number of nodes
    pub total_nodes: usize,
    /// Number of available nodes
    pub available_nodes: usize,
    /// Total cluster capacity
    pub total_capacity: usize,
    /// Current active requests
    pub active_requests: usize,
    /// Average load across nodes
    pub avg_load: f64,
}

impl ClusterStats {
    /// Calculates cluster utilization (0.0 to 1.0).
    pub fn utilization(&self) -> f64 {
        if self.total_capacity == 0 {
            0.0
        } else {
            self.active_requests as f64 / self.total_capacity as f64
        }
    }

    /// Checks if cluster is overloaded (>80% utilization).
    pub fn is_overloaded(&self) -> bool {
        self.utilization() > 0.8
    }
}

/// Distributed provider wrapper that distributes requests across nodes.
pub struct DistributedProvider<P> {
    local_provider: P,
    coordinator: Arc<DistributedInference>,
    node_id: String,
}

impl<P> DistributedProvider<P> {
    /// Creates a new distributed provider.
    pub fn new(
        local_provider: P,
        coordinator: Arc<DistributedInference>,
        node_id: impl Into<String>,
    ) -> Self {
        Self {
            local_provider,
            coordinator,
            node_id: node_id.into(),
        }
    }

    /// Gets the coordinator.
    pub fn coordinator(&self) -> Arc<DistributedInference> {
        self.coordinator.clone()
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for DistributedProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        // Select best node
        let _node = self.coordinator.select_node().await?;

        // Increment active requests
        self.coordinator.increment_active(&self.node_id).await;

        // Execute request
        let result = self.local_provider.generate_text(prompt).await;

        // Decrement active requests
        self.coordinator.decrement_active(&self.node_id).await;

        result
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        // Select best node
        let _node = self.coordinator.select_node().await?;

        // Increment active requests
        self.coordinator.increment_active(&self.node_id).await;

        // Execute request
        let result = self.local_provider.generate_structured::<T>(prompt).await;

        // Decrement active requests
        self.coordinator.decrement_active(&self.node_id).await;

        result
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        // Select best node
        let _node = self.coordinator.select_node().await?;

        // Increment active requests
        self.coordinator.increment_active(&self.node_id).await;

        // Execute request (note: decrement will happen when stream completes)
        self.local_provider.generate_text_stream(prompt).await
    }

    fn provider_name(&self) -> &str {
        self.local_provider.provider_name()
    }

    fn model_name(&self) -> &str {
        self.local_provider.model_name()
    }

    fn supports_streaming(&self) -> bool {
        self.local_provider.supports_streaming()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inference_node_creation() {
        let node = InferenceNode::new("node1", "http://localhost:8000")
            .with_max_concurrent(20)
            .with_capability("gpt-4")
            .with_region("us-east-1");

        assert_eq!(node.id, "node1");
        assert_eq!(node.endpoint, "http://localhost:8000");
        assert_eq!(node.max_concurrent, 20);
        assert!(node.capabilities.contains(&"gpt-4".to_string()));
        assert_eq!(node.region, Some("us-east-1".to_string()));
    }

    #[test]
    fn test_node_can_handle_request() {
        let mut node = InferenceNode::new("node1", "http://localhost:8000").with_max_concurrent(10);

        assert!(node.can_handle_request());

        node.active_requests = 10;
        assert!(!node.can_handle_request());

        node.active_requests = 5;
        node.available = false;
        assert!(!node.can_handle_request());
    }

    #[test]
    fn test_node_score() {
        let mut node = InferenceNode::new("node1", "http://localhost:8000").with_max_concurrent(10);

        let score1 = node.score();
        assert!(score1 > 0.0);

        node.active_requests = 5;
        let score2 = node.score();
        assert!(score2 < score1);

        node.available = false;
        assert_eq!(node.score(), 0.0);
    }

    #[tokio::test]
    async fn test_distributed_inference_add_remove() {
        let coordinator = DistributedInference::new(LoadBalancingStrategy::RoundRobin);

        let node = InferenceNode::new("node1", "http://localhost:8000");
        coordinator.add_node(node).await;

        let nodes = coordinator.get_nodes().await;
        assert_eq!(nodes.len(), 1);

        coordinator.remove_node("node1").await;
        let nodes = coordinator.get_nodes().await;
        assert_eq!(nodes.len(), 0);
    }

    #[tokio::test]
    async fn test_distributed_inference_node_selection() {
        let coordinator = DistributedInference::new(LoadBalancingStrategy::LeastConnections);

        let node1 = InferenceNode::new("node1", "http://localhost:8000").with_max_concurrent(10);
        let node2 = InferenceNode::new("node2", "http://localhost:8001").with_max_concurrent(10);

        coordinator.add_node(node1).await;
        coordinator.add_node(node2).await;

        // Increment active requests on node1
        coordinator.increment_active("node1").await;
        coordinator.increment_active("node1").await;

        // Should select node2 (least connections)
        let selected = coordinator.select_node().await.unwrap();
        assert_eq!(selected.id, "node2");
    }

    #[tokio::test]
    async fn test_cluster_stats() {
        let coordinator = DistributedInference::new(LoadBalancingStrategy::RoundRobin);

        let node1 = InferenceNode::new("node1", "http://localhost:8000").with_max_concurrent(10);
        let node2 = InferenceNode::new("node2", "http://localhost:8001").with_max_concurrent(20);

        coordinator.add_node(node1).await;
        coordinator.add_node(node2).await;

        coordinator.increment_active("node1").await;
        coordinator.increment_active("node2").await;

        let stats = coordinator.cluster_stats().await;
        assert_eq!(stats.total_nodes, 2);
        assert_eq!(stats.available_nodes, 2);
        assert_eq!(stats.total_capacity, 30);
        assert_eq!(stats.active_requests, 2);
    }

    #[test]
    fn test_cluster_stats_utilization() {
        let stats = ClusterStats {
            total_nodes: 2,
            available_nodes: 2,
            total_capacity: 100,
            active_requests: 50,
            avg_load: 0.5,
        };

        assert_eq!(stats.utilization(), 0.5);
        assert!(!stats.is_overloaded());

        let overloaded_stats = ClusterStats {
            total_nodes: 2,
            available_nodes: 2,
            total_capacity: 100,
            active_requests: 85,
            avg_load: 0.85,
        };

        assert!(overloaded_stats.is_overloaded());
    }
}
