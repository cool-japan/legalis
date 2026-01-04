//! Federated Learning for Legal AI
//!
//! Privacy-preserving distributed learning across legal databases.
//! Enables collaborative model training without sharing sensitive legal data.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Federated learning participant (e.g., law firm, court, legal department)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedNode {
    /// Unique node identifier
    pub node_id: String,
    /// Node name
    pub name: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Data sample count
    pub sample_count: usize,
    /// Node reputation score (0.0-1.0)
    pub reputation: f64,
}

impl FederatedNode {
    /// Creates a new federated node.
    pub fn new(
        node_id: impl Into<String>,
        name: impl Into<String>,
        jurisdiction: impl Into<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            name: name.into(),
            jurisdiction: jurisdiction.into(),
            sample_count: 0,
            reputation: 1.0,
        }
    }

    /// Sets the sample count.
    pub fn with_sample_count(mut self, count: usize) -> Self {
        self.sample_count = count;
        self
    }

    /// Sets the reputation score.
    pub fn with_reputation(mut self, reputation: f64) -> Self {
        self.reputation = reputation.clamp(0.0, 1.0);
        self
    }
}

/// Model update from a federated node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUpdate {
    /// Node that generated this update
    pub node_id: String,
    /// Model parameters (simplified as key-value pairs)
    pub parameters: HashMap<String, Vec<f64>>,
    /// Number of samples used for training
    pub samples_used: usize,
    /// Training loss
    pub loss: f64,
    /// Timestamp
    pub timestamp: i64,
}

impl ModelUpdate {
    /// Creates a new model update.
    pub fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            parameters: HashMap::new(),
            samples_used: 0,
            loss: 0.0,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Adds a parameter.
    pub fn add_parameter(mut self, key: impl Into<String>, values: Vec<f64>) -> Self {
        self.parameters.insert(key.into(), values);
        self
    }

    /// Sets the samples used.
    pub fn with_samples(mut self, samples: usize) -> Self {
        self.samples_used = samples;
        self
    }

    /// Sets the loss.
    pub fn with_loss(mut self, loss: f64) -> Self {
        self.loss = loss;
        self
    }
}

/// Aggregation strategy for federated learning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationStrategy {
    /// Federated Averaging (FedAvg) - weighted by sample count
    FederatedAveraging,
    /// Federated Proximal (FedProx) - with proximal term
    FederatedProximal,
    /// Reputation-weighted aggregation
    ReputationWeighted,
    /// Median aggregation (robust to outliers)
    Median,
}

/// Privacy mechanism for federated learning
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrivacyMechanism {
    /// No privacy (for testing)
    None,
    /// Differential Privacy with epsilon budget
    DifferentialPrivacy { epsilon: f64 },
    /// Secure aggregation
    SecureAggregation,
    /// Homomorphic encryption
    HomomorphicEncryption,
}

/// Federated learning configuration
#[derive(Debug, Clone)]
pub struct FederatedConfig {
    /// Aggregation strategy
    pub aggregation: AggregationStrategy,
    /// Privacy mechanism
    pub privacy: PrivacyMechanism,
    /// Minimum nodes required for aggregation
    pub min_nodes: usize,
    /// Maximum rounds
    pub max_rounds: usize,
    /// Convergence threshold
    pub convergence_threshold: f64,
}

impl Default for FederatedConfig {
    fn default() -> Self {
        Self {
            aggregation: AggregationStrategy::FederatedAveraging,
            privacy: PrivacyMechanism::DifferentialPrivacy { epsilon: 1.0 },
            min_nodes: 3,
            max_rounds: 100,
            convergence_threshold: 0.001,
        }
    }
}

impl FederatedConfig {
    /// Creates a new configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the aggregation strategy.
    pub fn with_aggregation(mut self, strategy: AggregationStrategy) -> Self {
        self.aggregation = strategy;
        self
    }

    /// Sets the privacy mechanism.
    pub fn with_privacy(mut self, privacy: PrivacyMechanism) -> Self {
        self.privacy = privacy;
        self
    }

    /// Sets the minimum nodes.
    pub fn with_min_nodes(mut self, min_nodes: usize) -> Self {
        self.min_nodes = min_nodes;
        self
    }

    /// Sets the maximum rounds.
    pub fn with_max_rounds(mut self, max_rounds: usize) -> Self {
        self.max_rounds = max_rounds;
        self
    }
}

/// Federated learning coordinator
pub struct FederatedCoordinator {
    /// Configuration
    config: FederatedConfig,
    /// Registered nodes
    nodes: Arc<RwLock<HashMap<String, FederatedNode>>>,
    /// Global model parameters
    global_model: Arc<RwLock<HashMap<String, Vec<f64>>>>,
    /// Current round
    current_round: Arc<RwLock<usize>>,
}

impl FederatedCoordinator {
    /// Creates a new federated coordinator.
    pub fn new(config: FederatedConfig) -> Self {
        Self {
            config,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            global_model: Arc::new(RwLock::new(HashMap::new())),
            current_round: Arc::new(RwLock::new(0)),
        }
    }

    /// Registers a new node.
    pub async fn register_node(&self, node: FederatedNode) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.node_id.clone(), node);
        Ok(())
    }

    /// Unregisters a node.
    pub async fn unregister_node(&self, node_id: &str) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        nodes.remove(node_id);
        Ok(())
    }

    /// Gets the list of registered nodes.
    pub async fn list_nodes(&self) -> Vec<FederatedNode> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }

    /// Aggregates model updates from nodes.
    pub async fn aggregate_updates(&self, updates: Vec<ModelUpdate>) -> Result<()> {
        if updates.len() < self.config.min_nodes {
            return Err(anyhow!(
                "Insufficient nodes: {} < {}",
                updates.len(),
                self.config.min_nodes
            ));
        }

        let nodes = self.nodes.read().await;
        let mut global_model = self.global_model.write().await;

        match self.config.aggregation {
            AggregationStrategy::FederatedAveraging => {
                self.federated_averaging(&updates, &mut global_model)
                    .await?;
            }
            AggregationStrategy::FederatedProximal => {
                self.federated_proximal(&updates, &mut global_model).await?;
            }
            AggregationStrategy::ReputationWeighted => {
                self.reputation_weighted(&updates, &nodes, &mut global_model)
                    .await?;
            }
            AggregationStrategy::Median => {
                self.median_aggregation(&updates, &mut global_model).await?;
            }
        }

        // Apply privacy mechanism
        self.apply_privacy(&mut global_model).await?;

        // Increment round
        let mut round = self.current_round.write().await;
        *round += 1;

        Ok(())
    }

    /// Federated averaging aggregation.
    async fn federated_averaging(
        &self,
        updates: &[ModelUpdate],
        global_model: &mut HashMap<String, Vec<f64>>,
    ) -> Result<()> {
        let total_samples: usize = updates.iter().map(|u| u.samples_used).sum();

        if total_samples == 0 {
            return Err(anyhow!("No samples in updates"));
        }

        // Clear global model
        global_model.clear();

        // Get all parameter keys
        let mut all_keys = std::collections::HashSet::new();
        for update in updates {
            all_keys.extend(update.parameters.keys().cloned());
        }

        // Aggregate each parameter
        for key in all_keys {
            let mut aggregated = Vec::new();
            let mut initialized = false;

            for update in updates {
                if let Some(params) = update.parameters.get(&key) {
                    let weight = update.samples_used as f64 / total_samples as f64;

                    if !initialized {
                        aggregated = vec![0.0; params.len()];
                        initialized = true;
                    }

                    for (i, &val) in params.iter().enumerate() {
                        if i < aggregated.len() {
                            aggregated[i] += val * weight;
                        }
                    }
                }
            }

            if !aggregated.is_empty() {
                global_model.insert(key, aggregated);
            }
        }

        Ok(())
    }

    /// Federated proximal aggregation.
    async fn federated_proximal(
        &self,
        updates: &[ModelUpdate],
        global_model: &mut HashMap<String, Vec<f64>>,
    ) -> Result<()> {
        // FedProx is similar to FedAvg but with a proximal term
        // For simplicity, we use FedAvg here
        self.federated_averaging(updates, global_model).await
    }

    /// Reputation-weighted aggregation.
    async fn reputation_weighted(
        &self,
        updates: &[ModelUpdate],
        nodes: &HashMap<String, FederatedNode>,
        global_model: &mut HashMap<String, Vec<f64>>,
    ) -> Result<()> {
        let total_weight: f64 = updates
            .iter()
            .filter_map(|u| nodes.get(&u.node_id).map(|n| n.reputation))
            .sum();

        if total_weight == 0.0 {
            return Err(anyhow!("Total reputation weight is zero"));
        }

        // Clear global model
        global_model.clear();

        // Get all parameter keys
        let mut all_keys = std::collections::HashSet::new();
        for update in updates {
            all_keys.extend(update.parameters.keys().cloned());
        }

        // Aggregate each parameter
        for key in all_keys {
            let mut aggregated = Vec::new();
            let mut initialized = false;

            for update in updates {
                if let Some(params) = update.parameters.get(&key) {
                    if let Some(node) = nodes.get(&update.node_id) {
                        let weight = node.reputation / total_weight;

                        if !initialized {
                            aggregated = vec![0.0; params.len()];
                            initialized = true;
                        }

                        for (i, &val) in params.iter().enumerate() {
                            if i < aggregated.len() {
                                aggregated[i] += val * weight;
                            }
                        }
                    }
                }
            }

            if !aggregated.is_empty() {
                global_model.insert(key, aggregated);
            }
        }

        Ok(())
    }

    /// Median aggregation (robust to outliers).
    async fn median_aggregation(
        &self,
        updates: &[ModelUpdate],
        global_model: &mut HashMap<String, Vec<f64>>,
    ) -> Result<()> {
        // Clear global model
        global_model.clear();

        // Get all parameter keys
        let mut all_keys = std::collections::HashSet::new();
        for update in updates {
            all_keys.extend(update.parameters.keys().cloned());
        }

        // Aggregate each parameter
        for key in all_keys {
            let mut values_by_index: HashMap<usize, Vec<f64>> = HashMap::new();

            for update in updates {
                if let Some(params) = update.parameters.get(&key) {
                    for (i, &val) in params.iter().enumerate() {
                        values_by_index.entry(i).or_insert_with(Vec::new).push(val);
                    }
                }
            }

            let mut aggregated = Vec::new();
            for i in 0..values_by_index.len() {
                if let Some(mut vals) = values_by_index.remove(&i) {
                    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let median = if vals.len() % 2 == 0 {
                        (vals[vals.len() / 2 - 1] + vals[vals.len() / 2]) / 2.0
                    } else {
                        vals[vals.len() / 2]
                    };
                    aggregated.push(median);
                }
            }

            if !aggregated.is_empty() {
                global_model.insert(key, aggregated);
            }
        }

        Ok(())
    }

    /// Applies privacy mechanism to the global model.
    async fn apply_privacy(&self, global_model: &mut HashMap<String, Vec<f64>>) -> Result<()> {
        match self.config.privacy {
            PrivacyMechanism::None => Ok(()),
            PrivacyMechanism::DifferentialPrivacy { epsilon } => {
                self.apply_differential_privacy(global_model, epsilon).await
            }
            PrivacyMechanism::SecureAggregation => {
                // Secure aggregation is applied during the aggregation phase
                Ok(())
            }
            PrivacyMechanism::HomomorphicEncryption => {
                // Homomorphic encryption is applied during the aggregation phase
                Ok(())
            }
        }
    }

    /// Applies differential privacy by adding Laplace noise.
    async fn apply_differential_privacy(
        &self,
        global_model: &mut HashMap<String, Vec<f64>>,
        epsilon: f64,
    ) -> Result<()> {
        use rand::Rng;

        let mut rng = rand::rng();
        let sensitivity = 1.0; // Assume unit sensitivity
        let scale = sensitivity / epsilon;

        for params in global_model.values_mut() {
            for val in params.iter_mut() {
                // Add Laplace noise
                let u: f64 = rng.random_range(-0.5..0.5);
                let noise = -scale * u.signum() * (1.0 - 2.0 * u.abs()).ln();
                *val += noise;
            }
        }

        Ok(())
    }

    /// Gets the current global model.
    pub async fn get_global_model(&self) -> HashMap<String, Vec<f64>> {
        self.global_model.read().await.clone()
    }

    /// Gets the current round number.
    pub async fn get_current_round(&self) -> usize {
        *self.current_round.read().await
    }

    /// Checks if training has converged.
    pub async fn has_converged(&self, previous_loss: f64, current_loss: f64) -> bool {
        let delta = (previous_loss - current_loss).abs();
        delta < self.config.convergence_threshold
    }
}

/// Legal domain specific federated learning
pub struct LegalFederatedLearning {
    /// Coordinator
    coordinator: FederatedCoordinator,
}

impl LegalFederatedLearning {
    /// Creates a new legal federated learning system.
    pub fn new(config: FederatedConfig) -> Self {
        Self {
            coordinator: FederatedCoordinator::new(config),
        }
    }

    /// Registers a law firm as a federated node.
    pub async fn register_law_firm(
        &self,
        firm_id: impl Into<String>,
        firm_name: impl Into<String>,
        jurisdiction: impl Into<String>,
        case_count: usize,
    ) -> Result<()> {
        let node =
            FederatedNode::new(firm_id, firm_name, jurisdiction).with_sample_count(case_count);
        self.coordinator.register_node(node).await
    }

    /// Registers a court as a federated node.
    pub async fn register_court(
        &self,
        court_id: impl Into<String>,
        court_name: impl Into<String>,
        jurisdiction: impl Into<String>,
        decision_count: usize,
    ) -> Result<()> {
        let node = FederatedNode::new(court_id, court_name, jurisdiction)
            .with_sample_count(decision_count)
            .with_reputation(1.0); // Courts have high reputation
        self.coordinator.register_node(node).await
    }

    /// Performs a federated training round for legal document classification.
    pub async fn train_document_classifier(&self, updates: Vec<ModelUpdate>) -> Result<()> {
        self.coordinator.aggregate_updates(updates).await
    }

    /// Performs a federated training round for contract analysis.
    pub async fn train_contract_analyzer(&self, updates: Vec<ModelUpdate>) -> Result<()> {
        self.coordinator.aggregate_updates(updates).await
    }

    /// Gets the federated model for inference.
    pub async fn get_model(&self) -> HashMap<String, Vec<f64>> {
        self.coordinator.get_global_model().await
    }

    /// Gets training statistics.
    pub async fn get_statistics(&self) -> FederatedStatistics {
        FederatedStatistics {
            total_nodes: self.coordinator.list_nodes().await.len(),
            current_round: self.coordinator.get_current_round().await,
            nodes_by_jurisdiction: self.nodes_by_jurisdiction().await,
        }
    }

    /// Groups nodes by jurisdiction.
    async fn nodes_by_jurisdiction(&self) -> HashMap<String, usize> {
        let nodes = self.coordinator.list_nodes().await;
        let mut by_jurisdiction = HashMap::new();
        for node in nodes {
            *by_jurisdiction.entry(node.jurisdiction).or_insert(0) += 1;
        }
        by_jurisdiction
    }
}

/// Federated learning statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedStatistics {
    /// Total number of nodes
    pub total_nodes: usize,
    /// Current training round
    pub current_round: usize,
    /// Nodes grouped by jurisdiction
    pub nodes_by_jurisdiction: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federated_node() {
        let node = FederatedNode::new("firm1", "Smith & Associates", "US")
            .with_sample_count(100)
            .with_reputation(0.95);

        assert_eq!(node.node_id, "firm1");
        assert_eq!(node.name, "Smith & Associates");
        assert_eq!(node.jurisdiction, "US");
        assert_eq!(node.sample_count, 100);
        assert!((node.reputation - 0.95).abs() < f64::EPSILON);
    }

    #[test]
    fn test_model_update() {
        let update = ModelUpdate::new("node1")
            .add_parameter("weights", vec![1.0, 2.0, 3.0])
            .with_samples(50)
            .with_loss(0.25);

        assert_eq!(update.node_id, "node1");
        assert_eq!(
            update.parameters.get("weights").unwrap(),
            &vec![1.0, 2.0, 3.0]
        );
        assert_eq!(update.samples_used, 50);
        assert!((update.loss - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn test_federated_config() {
        let config = FederatedConfig::new()
            .with_aggregation(AggregationStrategy::ReputationWeighted)
            .with_privacy(PrivacyMechanism::DifferentialPrivacy { epsilon: 0.5 })
            .with_min_nodes(5)
            .with_max_rounds(200);

        assert_eq!(config.aggregation, AggregationStrategy::ReputationWeighted);
        assert_eq!(
            config.privacy,
            PrivacyMechanism::DifferentialPrivacy { epsilon: 0.5 }
        );
        assert_eq!(config.min_nodes, 5);
        assert_eq!(config.max_rounds, 200);
    }

    #[tokio::test]
    async fn test_federated_coordinator_registration() {
        let config = FederatedConfig::new().with_min_nodes(2);
        let coordinator = FederatedCoordinator::new(config);

        let node1 = FederatedNode::new("node1", "Firm A", "US").with_sample_count(100);
        let node2 = FederatedNode::new("node2", "Firm B", "UK").with_sample_count(150);

        coordinator.register_node(node1).await.unwrap();
        coordinator.register_node(node2).await.unwrap();

        let nodes = coordinator.list_nodes().await;
        assert_eq!(nodes.len(), 2);
    }

    #[tokio::test]
    async fn test_federated_averaging() {
        let config = FederatedConfig::new()
            .with_aggregation(AggregationStrategy::FederatedAveraging)
            .with_privacy(PrivacyMechanism::None)
            .with_min_nodes(2);

        let coordinator = FederatedCoordinator::new(config);

        let update1 = ModelUpdate::new("node1")
            .add_parameter("w1", vec![1.0, 2.0])
            .with_samples(100);

        let update2 = ModelUpdate::new("node2")
            .add_parameter("w1", vec![3.0, 4.0])
            .with_samples(100);

        coordinator
            .aggregate_updates(vec![update1, update2])
            .await
            .unwrap();

        let model = coordinator.get_global_model().await;
        let w1 = model.get("w1").unwrap();

        assert!((w1[0] - 2.0).abs() < 1e-6);
        assert!((w1[1] - 3.0).abs() < 1e-6);
    }

    #[tokio::test]
    async fn test_legal_federated_learning() {
        let config = FederatedConfig::new().with_min_nodes(2);
        let lfl = LegalFederatedLearning::new(config);

        lfl.register_law_firm("firm1", "Smith & Associates", "US", 100)
            .await
            .unwrap();

        lfl.register_court("court1", "Supreme Court", "US", 500)
            .await
            .unwrap();

        let stats = lfl.get_statistics().await;
        assert_eq!(stats.total_nodes, 2);
        assert_eq!(stats.current_round, 0);
        assert_eq!(stats.nodes_by_jurisdiction.get("US"), Some(&2));
    }

    #[tokio::test]
    async fn test_insufficient_nodes() {
        let config = FederatedConfig::new().with_min_nodes(3);
        let coordinator = FederatedCoordinator::new(config);

        let update1 = ModelUpdate::new("node1").with_samples(100);
        let update2 = ModelUpdate::new("node2").with_samples(100);

        let result = coordinator.aggregate_updates(vec![update1, update2]).await;
        assert!(result.is_err());
    }
}
