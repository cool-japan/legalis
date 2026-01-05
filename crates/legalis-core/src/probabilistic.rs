//! Probabilistic Legal Reasoning
//!
//! This module provides tools for handling uncertainty in legal reasoning through:
//! - Bayesian networks for modeling conditional dependencies
//! - Probabilistic condition evaluation with confidence scores
//! - Monte Carlo simulation for outcome prediction
//! - Probabilistic entailment with confidence intervals
//! - Risk quantification for legal decisions
//!
//! # Examples
//!
//! ```
//! use legalis_core::probabilistic::{BayesianNetwork, ProbabilisticEvaluator};
//!
//! // Create a Bayesian network for legal reasoning
//! let mut network = BayesianNetwork::new();
//! network.add_node("age_over_18", 0.85);
//! network.add_node("has_license", 0.60);
//! network.add_conditional_probability("can_drive", &["age_over_18", "has_license"], 0.95);
//!
//! // Query the network
//! let prob = network.query("can_drive", &[("age_over_18", true), ("has_license", true)]);
//! assert!(prob > 0.8);
//! ```

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A node in a Bayesian network representing a legal proposition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BayesianNode {
    /// Unique identifier for the node
    pub id: String,
    /// Prior probability (unconditional probability)
    pub prior: f64,
    /// Parent nodes this node depends on
    pub parents: Vec<String>,
    /// Conditional probability table
    /// Maps parent states to probability of this node being true
    pub cpt: HashMap<Vec<bool>, f64>,
}

impl BayesianNode {
    /// Create a new Bayesian node with a prior probability
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::BayesianNode;
    ///
    /// let node = BayesianNode::new("age_over_18", 0.75);
    /// assert_eq!(node.id, "age_over_18");
    /// assert_eq!(node.prior, 0.75);
    /// ```
    pub fn new(id: impl Into<String>, prior: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&prior),
            "Prior probability must be between 0 and 1"
        );
        Self {
            id: id.into(),
            prior,
            parents: Vec::new(),
            cpt: HashMap::new(),
        }
    }

    /// Add a parent node dependency
    pub fn add_parent(&mut self, parent_id: String) {
        if !self.parents.contains(&parent_id) {
            self.parents.push(parent_id);
        }
    }

    /// Set conditional probability for a specific parent configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::BayesianNode;
    ///
    /// let mut node = BayesianNode::new("can_drive", 0.5);
    /// node.add_parent("age_over_18".to_string());
    /// node.add_parent("has_license".to_string());
    /// node.set_conditional_probability(vec![true, true], 0.95);
    /// node.set_conditional_probability(vec![true, false], 0.05);
    /// ```
    pub fn set_conditional_probability(&mut self, parent_states: Vec<bool>, probability: f64) {
        assert!(
            (0.0..=1.0).contains(&probability),
            "Probability must be between 0 and 1"
        );
        assert_eq!(
            parent_states.len(),
            self.parents.len(),
            "Parent states must match number of parents"
        );
        self.cpt.insert(parent_states, probability);
    }

    /// Get the probability of this node being true given parent states
    pub fn probability(&self, parent_states: &[(String, bool)]) -> f64 {
        if self.parents.is_empty() {
            return self.prior;
        }

        // Build parent state vector in correct order
        let state_vec: Vec<bool> = self
            .parents
            .iter()
            .map(|parent_id| {
                parent_states
                    .iter()
                    .find(|(id, _)| id == parent_id)
                    .map(|(_, state)| *state)
                    .unwrap_or(false)
            })
            .collect();

        self.cpt.get(&state_vec).copied().unwrap_or(self.prior)
    }
}

impl PartialEq for BayesianNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for BayesianNode {}

impl Hash for BayesianNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Bayesian Network for modeling uncertainty in legal reasoning
///
/// A Bayesian network represents probabilistic relationships between
/// legal conditions and outcomes. Nodes represent propositions
/// (e.g., "applicant is over 18") and edges represent dependencies.
///
/// # Examples
///
/// ```
/// use legalis_core::probabilistic::BayesianNetwork;
///
/// let mut network = BayesianNetwork::new();
///
/// // Add independent nodes
/// network.add_node("is_citizen", 0.90);
/// network.add_node("age_over_18", 0.75);
///
/// // Add dependent node
/// network.add_conditional_probability(
///     "can_vote",
///     &["is_citizen", "age_over_18"],
///     0.98
/// );
///
/// // Query probability
/// let evidence = vec![("is_citizen", true), ("age_over_18", true)];
/// let prob = network.query("can_vote", &evidence);
/// assert!(prob > 0.85);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BayesianNetwork {
    /// Nodes in the network
    nodes: HashMap<String, BayesianNode>,
}

impl BayesianNetwork {
    /// Create a new empty Bayesian network
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Add a node with a prior probability
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::BayesianNetwork;
    ///
    /// let mut network = BayesianNetwork::new();
    /// network.add_node("has_standing", 0.80);
    /// ```
    pub fn add_node(&mut self, id: impl Into<String>, prior: f64) {
        let id_str = id.into();
        self.nodes
            .insert(id_str.clone(), BayesianNode::new(id_str, prior));
    }

    /// Add a conditional probability for a node given its parents
    ///
    /// This creates the node if it doesn't exist and sets up dependencies.
    pub fn add_conditional_probability(
        &mut self,
        node_id: impl Into<String>,
        parent_ids: &[&str],
        probability: f64,
    ) {
        let node_id_str = node_id.into();

        // Ensure all parents exist
        for parent_id in parent_ids {
            if !self.nodes.contains_key(*parent_id) {
                self.add_node(*parent_id, 0.5); // Default prior
            }
        }

        // Get or create the node
        let node = self
            .nodes
            .entry(node_id_str.clone())
            .or_insert_with(|| BayesianNode::new(node_id_str.clone(), 0.5));

        // Add parents
        for parent_id in parent_ids {
            node.add_parent(parent_id.to_string());
        }

        // Set CPT entry for all parents being true
        let all_true = vec![true; parent_ids.len()];
        node.set_conditional_probability(all_true, probability);

        // Set reasonable defaults for other combinations
        if parent_ids.len() == 1 {
            node.set_conditional_probability(vec![false], probability * 0.1);
        } else if parent_ids.len() == 2 {
            node.set_conditional_probability(vec![true, false], probability * 0.3);
            node.set_conditional_probability(vec![false, true], probability * 0.3);
            node.set_conditional_probability(vec![false, false], probability * 0.05);
        }
    }

    /// Query the probability of a node given evidence
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::BayesianNetwork;
    ///
    /// let mut network = BayesianNetwork::new();
    /// network.add_node("employed", 0.70);
    /// network.add_conditional_probability("income_sufficient", &["employed"], 0.85);
    ///
    /// let prob = network.query("income_sufficient", &[("employed", true)]);
    /// assert!(prob > 0.7);
    /// ```
    pub fn query(&self, node_id: &str, evidence: &[(impl AsRef<str>, bool)]) -> f64 {
        let node = match self.nodes.get(node_id) {
            Some(n) => n,
            None => return 0.0,
        };

        let evidence_vec: Vec<(String, bool)> = evidence
            .iter()
            .map(|(k, v)| (k.as_ref().to_string(), *v))
            .collect();

        node.probability(&evidence_vec)
    }

    /// Get all nodes in the network
    pub fn nodes(&self) -> &HashMap<String, BayesianNode> {
        &self.nodes
    }

    /// Get number of nodes in the network
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Check if network contains a node
    pub fn contains_node(&self, id: &str) -> bool {
        self.nodes.contains_key(id)
    }

    /// Remove a node from the network
    pub fn remove_node(&mut self, id: &str) -> Option<BayesianNode> {
        self.nodes.remove(id)
    }
}

impl Default for BayesianNetwork {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of probabilistic evaluation with confidence score
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProbabilisticResult {
    /// The outcome (true/false)
    pub outcome: bool,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Explanation of how the confidence was computed
    pub explanation: String,
}

impl ProbabilisticResult {
    /// Create a new probabilistic result
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::ProbabilisticResult;
    ///
    /// let result = ProbabilisticResult::new(true, 0.85, "High confidence based on evidence");
    /// assert!(result.outcome);
    /// assert!(result.confidence > 0.8);
    /// ```
    pub fn new(outcome: bool, confidence: f64, explanation: impl Into<String>) -> Self {
        assert!(
            (0.0..=1.0).contains(&confidence),
            "Confidence must be between 0 and 1"
        );
        Self {
            outcome,
            confidence,
            explanation: explanation.into(),
        }
    }

    /// Check if the result is highly confident (>= 0.8)
    pub fn is_highly_confident(&self) -> bool {
        self.confidence >= 0.8
    }

    /// Check if the result is uncertain (< 0.6)
    pub fn is_uncertain(&self) -> bool {
        self.confidence < 0.6
    }

    /// Get confidence level as a category
    pub fn confidence_level(&self) -> &str {
        match self.confidence {
            c if c >= 0.9 => "Very High",
            c if c >= 0.8 => "High",
            c if c >= 0.6 => "Moderate",
            c if c >= 0.4 => "Low",
            _ => "Very Low",
        }
    }
}

/// Evaluator for probabilistic condition evaluation
///
/// # Examples
///
/// ```
/// use legalis_core::probabilistic::{ProbabilisticEvaluator, BayesianNetwork};
///
/// let mut network = BayesianNetwork::new();
/// network.add_node("age_verified", 0.90);
/// network.add_node("income_verified", 0.85);
/// network.add_conditional_probability("eligible", &["age_verified", "income_verified"], 0.95);
///
/// let evaluator = ProbabilisticEvaluator::new(network);
/// let result = evaluator.evaluate("eligible", &[
///     ("age_verified", true),
///     ("income_verified", true)
/// ]);
///
/// assert!(result.outcome);
/// assert!(result.confidence > 0.8);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProbabilisticEvaluator {
    /// The underlying Bayesian network
    network: BayesianNetwork,
    /// Threshold for considering a result as "true"
    threshold: f64,
}

impl ProbabilisticEvaluator {
    /// Create a new probabilistic evaluator with default threshold (0.5)
    pub fn new(network: BayesianNetwork) -> Self {
        Self {
            network,
            threshold: 0.5,
        }
    }

    /// Create a new evaluator with a custom threshold
    pub fn with_threshold(network: BayesianNetwork, threshold: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&threshold),
            "Threshold must be between 0 and 1"
        );
        Self { network, threshold }
    }

    /// Evaluate a proposition given evidence
    pub fn evaluate(
        &self,
        proposition: &str,
        evidence: &[(impl AsRef<str>, bool)],
    ) -> ProbabilisticResult {
        let probability = self.network.query(proposition, evidence);
        let outcome = probability >= self.threshold;
        let explanation = format!(
            "Probability: {:.2}%, Threshold: {:.2}%",
            probability * 100.0,
            self.threshold * 100.0
        );

        ProbabilisticResult::new(outcome, probability, explanation)
    }

    /// Get the underlying network
    pub fn network(&self) -> &BayesianNetwork {
        &self.network
    }

    /// Get the threshold
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Set a new threshold
    pub fn set_threshold(&mut self, threshold: f64) {
        assert!(
            (0.0..=1.0).contains(&threshold),
            "Threshold must be between 0 and 1"
        );
        self.threshold = threshold;
    }
}

/// Monte Carlo simulation result with statistical distribution
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimulationResult {
    /// Number of simulations run
    pub iterations: usize,
    /// Mean outcome (proportion of successful outcomes)
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Minimum observed value
    pub min: f64,
    /// Maximum observed value
    pub max: f64,
    /// Confidence interval (lower bound, upper bound) at 95%
    pub confidence_interval: (f64, f64),
    /// Outcome distribution (histogram)
    pub distribution: Vec<(String, usize)>,
}

impl SimulationResult {
    /// Create a new simulation result
    pub fn new(
        iterations: usize,
        mean: f64,
        std_dev: f64,
        min: f64,
        max: f64,
        confidence_interval: (f64, f64),
        distribution: Vec<(String, usize)>,
    ) -> Self {
        Self {
            iterations,
            mean,
            std_dev,
            min,
            max,
            confidence_interval,
            distribution,
        }
    }

    /// Check if the result is statistically significant
    pub fn is_significant(&self) -> bool {
        self.std_dev < 0.1
    }

    /// Get the confidence interval width
    pub fn confidence_width(&self) -> f64 {
        self.confidence_interval.1 - self.confidence_interval.0
    }

    /// Get the coefficient of variation (relative std deviation)
    pub fn coefficient_of_variation(&self) -> f64 {
        if self.mean == 0.0 {
            0.0
        } else {
            self.std_dev / self.mean
        }
    }
}

/// Monte Carlo simulator for legal outcome prediction
///
/// Uses random sampling to estimate probability distributions
/// for complex legal scenarios with multiple uncertain variables.
///
/// # Examples
///
/// ```
/// use legalis_core::probabilistic::{MonteCarloSimulator, BayesianNetwork};
///
/// let mut network = BayesianNetwork::new();
/// network.add_node("age_verified", 0.85);
/// network.add_node("income_verified", 0.80);
/// network.add_conditional_probability("eligible", &["age_verified", "income_verified"], 0.90);
///
/// let simulator = MonteCarloSimulator::new(network);
/// let result = simulator.simulate("eligible", 1000);
///
/// println!("Mean probability: {:.2}%", result.mean * 100.0);
/// println!("Confidence interval: ({:.2}, {:.2})", result.confidence_interval.0, result.confidence_interval.1);
/// assert!(result.iterations == 1000);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MonteCarloSimulator {
    /// The underlying Bayesian network
    network: BayesianNetwork,
    /// Random seed for reproducibility
    seed: Option<u64>,
}

impl MonteCarloSimulator {
    /// Create a new Monte Carlo simulator
    pub fn new(network: BayesianNetwork) -> Self {
        Self {
            network,
            seed: None,
        }
    }

    /// Create a simulator with a specific random seed
    pub fn with_seed(network: BayesianNetwork, seed: u64) -> Self {
        Self {
            network,
            seed: Some(seed),
        }
    }

    /// Run Monte Carlo simulation for a target proposition
    ///
    /// # Arguments
    ///
    /// * `target` - The proposition to simulate
    /// * `iterations` - Number of simulation runs
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::{MonteCarloSimulator, BayesianNetwork};
    ///
    /// let mut network = BayesianNetwork::new();
    /// network.add_node("condition_met", 0.75);
    ///
    /// let simulator = MonteCarloSimulator::new(network);
    /// let result = simulator.simulate("condition_met", 5000);
    ///
    /// assert!(result.mean > 0.7 && result.mean < 0.8);
    /// ```
    pub fn simulate(&self, target: &str, iterations: usize) -> SimulationResult {
        let mut outcomes = Vec::with_capacity(iterations);
        let mut rng = self.seed.unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        for _ in 0..iterations {
            // Generate random evidence for all nodes
            let evidence: Vec<(String, bool)> = self
                .network
                .nodes()
                .iter()
                .map(|(id, node)| {
                    // Simple LCG random number generator
                    rng = (rng.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
                    let random_value = (rng as f64) / (0x7fffffff as f64);
                    (id.clone(), random_value < node.prior)
                })
                .collect();

            let prob = self.network.query(target, &evidence);
            outcomes.push(prob);
        }

        // Calculate statistics
        let mean = outcomes.iter().sum::<f64>() / iterations as f64;
        let variance = outcomes.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / iterations as f64;
        let std_dev = variance.sqrt();
        let min = outcomes.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = outcomes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // 95% confidence interval
        let z_score = 1.96;
        let margin = z_score * std_dev / (iterations as f64).sqrt();
        let confidence_interval = ((mean - margin).max(0.0), (mean + margin).min(1.0));

        // Build histogram
        let mut distribution = Vec::new();
        let buckets = 10;
        let mut counts = vec![0; buckets];
        for &outcome in &outcomes {
            let bucket = ((outcome * buckets as f64).floor() as usize).min(buckets - 1);
            counts[bucket] += 1;
        }
        for (i, count) in counts.iter().enumerate() {
            let lower = i as f64 / buckets as f64;
            let upper = (i + 1) as f64 / buckets as f64;
            distribution.push((format!("{:.2}-{:.2}", lower, upper), *count));
        }

        SimulationResult::new(
            iterations,
            mean,
            std_dev,
            min,
            max,
            confidence_interval,
            distribution,
        )
    }

    /// Simulate with specific evidence fixed
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::{MonteCarloSimulator, BayesianNetwork};
    ///
    /// let mut network = BayesianNetwork::new();
    /// network.add_node("age_verified", 0.85);
    /// network.add_conditional_probability("eligible", &["age_verified"], 0.90);
    ///
    /// let simulator = MonteCarloSimulator::new(network);
    /// let result = simulator.simulate_with_evidence(
    ///     "eligible",
    ///     &[("age_verified", true)],
    ///     1000
    /// );
    ///
    /// assert!(result.mean > 0.8);
    /// ```
    pub fn simulate_with_evidence(
        &self,
        target: &str,
        fixed_evidence: &[(&str, bool)],
        iterations: usize,
    ) -> SimulationResult {
        let mut outcomes = Vec::with_capacity(iterations);
        let mut rng = self.seed.unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        let fixed_map: HashMap<&str, bool> = fixed_evidence.iter().copied().collect();

        for _ in 0..iterations {
            // Generate random evidence for non-fixed nodes
            let mut evidence: Vec<(String, bool)> = Vec::new();
            for (id, node) in self.network.nodes().iter() {
                let value = if let Some(&fixed_value) = fixed_map.get(id.as_str()) {
                    fixed_value
                } else {
                    rng = (rng.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
                    let random_value = (rng as f64) / (0x7fffffff as f64);
                    random_value < node.prior
                };
                evidence.push((id.clone(), value));
            }

            let prob = self.network.query(target, &evidence);
            outcomes.push(prob);
        }

        // Calculate statistics (same as above)
        let mean = outcomes.iter().sum::<f64>() / iterations as f64;
        let variance = outcomes.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / iterations as f64;
        let std_dev = variance.sqrt();
        let min = outcomes.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = outcomes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        let z_score = 1.96;
        let margin = z_score * std_dev / (iterations as f64).sqrt();
        let confidence_interval = ((mean - margin).max(0.0), (mean + margin).min(1.0));

        let buckets = 10;
        let mut counts = vec![0; buckets];
        for &outcome in &outcomes {
            let bucket = ((outcome * buckets as f64).floor() as usize).min(buckets - 1);
            counts[bucket] += 1;
        }
        let mut distribution = Vec::new();
        for (i, count) in counts.iter().enumerate() {
            let lower = i as f64 / buckets as f64;
            let upper = (i + 1) as f64 / buckets as f64;
            distribution.push((format!("{:.2}-{:.2}", lower, upper), *count));
        }

        SimulationResult::new(
            iterations,
            mean,
            std_dev,
            min,
            max,
            confidence_interval,
            distribution,
        )
    }

    /// Get the underlying network
    pub fn network(&self) -> &BayesianNetwork {
        &self.network
    }

    /// Get the random seed if set
    pub fn seed(&self) -> Option<u64> {
        self.seed
    }
}

/// Probabilistic entailment result with confidence intervals
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProbabilisticEntailment {
    /// The entailed proposition
    pub proposition: String,
    /// Probability of entailment
    pub probability: f64,
    /// Confidence interval (lower, upper)
    pub confidence_interval: (f64, f64),
    /// Supporting evidence
    pub evidence: Vec<(String, bool)>,
    /// Explanation of the entailment
    pub explanation: String,
}

impl ProbabilisticEntailment {
    /// Create a new probabilistic entailment
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::ProbabilisticEntailment;
    ///
    /// let entailment = ProbabilisticEntailment::new(
    ///     "eligible_for_benefit",
    ///     0.85,
    ///     (0.80, 0.90),
    ///     vec![("age_verified".to_string(), true)],
    ///     "High confidence based on verified age"
    /// );
    ///
    /// assert_eq!(entailment.proposition, "eligible_for_benefit");
    /// assert!(entailment.is_highly_probable());
    /// ```
    pub fn new(
        proposition: impl Into<String>,
        probability: f64,
        confidence_interval: (f64, f64),
        evidence: Vec<(String, bool)>,
        explanation: impl Into<String>,
    ) -> Self {
        assert!(
            (0.0..=1.0).contains(&probability),
            "Probability must be between 0 and 1"
        );
        Self {
            proposition: proposition.into(),
            probability,
            confidence_interval,
            evidence,
            explanation: explanation.into(),
        }
    }

    /// Check if the entailment is highly probable (>= 0.8)
    pub fn is_highly_probable(&self) -> bool {
        self.probability >= 0.8
    }

    /// Check if the entailment is uncertain (< 0.6)
    pub fn is_uncertain(&self) -> bool {
        self.probability < 0.6
    }

    /// Get the confidence interval width
    pub fn confidence_width(&self) -> f64 {
        self.confidence_interval.1 - self.confidence_interval.0
    }

    /// Get probability category
    pub fn probability_category(&self) -> &str {
        match self.probability {
            p if p >= 0.9 => "Very High",
            p if p >= 0.75 => "High",
            p if p >= 0.5 => "Moderate",
            p if p >= 0.25 => "Low",
            _ => "Very Low",
        }
    }
}

/// Probabilistic entailment engine
///
/// Computes what legal conclusions follow from statutes and facts
/// with associated probability and confidence intervals.
///
/// # Examples
///
/// ```
/// use legalis_core::probabilistic::{ProbabilisticEntailmentEngine, BayesianNetwork};
///
/// let mut network = BayesianNetwork::new();
/// network.add_node("age_over_65", 0.30);
/// network.add_node("income_below_threshold", 0.40);
/// network.add_conditional_probability(
///     "eligible_for_senior_discount",
///     &["age_over_65", "income_below_threshold"],
///     0.95
/// );
///
/// let engine = ProbabilisticEntailmentEngine::new(network);
/// let entailments = engine.entail(&[
///     ("age_over_65", true),
///     ("income_below_threshold", true)
/// ]);
///
/// assert!(!entailments.is_empty());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProbabilisticEntailmentEngine {
    /// The underlying Bayesian network
    network: BayesianNetwork,
    /// Probability threshold for considering entailment
    threshold: f64,
    /// Number of Monte Carlo simulations for confidence intervals
    simulations: usize,
}

impl ProbabilisticEntailmentEngine {
    /// Create a new probabilistic entailment engine
    pub fn new(network: BayesianNetwork) -> Self {
        Self {
            network,
            threshold: 0.5,
            simulations: 1000,
        }
    }

    /// Create an engine with custom threshold
    pub fn with_threshold(network: BayesianNetwork, threshold: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&threshold),
            "Threshold must be between 0 and 1"
        );
        Self {
            network,
            threshold,
            simulations: 1000,
        }
    }

    /// Set the number of Monte Carlo simulations
    pub fn set_simulations(&mut self, simulations: usize) {
        self.simulations = simulations;
    }

    /// Compute probabilistic entailments from evidence
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::{ProbabilisticEntailmentEngine, BayesianNetwork};
    ///
    /// let mut network = BayesianNetwork::new();
    /// network.add_node("has_license", 0.70);
    /// network.add_conditional_probability("can_practice_law", &["has_license"], 0.98);
    ///
    /// let engine = ProbabilisticEntailmentEngine::new(network);
    /// let entailments = engine.entail(&[("has_license", true)]);
    ///
    /// assert!(entailments.iter().any(|e| e.proposition == "can_practice_law"));
    /// ```
    pub fn entail(&self, evidence: &[(&str, bool)]) -> Vec<ProbabilisticEntailment> {
        let mut entailments = Vec::new();
        let evidence_vec: Vec<(String, bool)> =
            evidence.iter().map(|(k, v)| (k.to_string(), *v)).collect();

        // Check each node in the network
        for (node_id, _) in self.network.nodes().iter() {
            // Skip nodes that are in the evidence
            if evidence.iter().any(|(id, _)| id == node_id) {
                continue;
            }

            let probability = self.network.query(node_id, &evidence_vec);

            if probability >= self.threshold {
                // Use Monte Carlo to estimate confidence interval
                let simulator = MonteCarloSimulator::new(self.network.clone());
                let sim_result =
                    simulator.simulate_with_evidence(node_id, evidence, self.simulations);

                let explanation = format!(
                    "Probability: {:.1}%, Category: {}, CI: ({:.2}, {:.2})",
                    probability * 100.0,
                    if probability >= 0.8 {
                        "High"
                    } else if probability >= 0.6 {
                        "Moderate"
                    } else {
                        "Low"
                    },
                    sim_result.confidence_interval.0,
                    sim_result.confidence_interval.1
                );

                entailments.push(ProbabilisticEntailment::new(
                    node_id.clone(),
                    probability,
                    sim_result.confidence_interval,
                    evidence_vec.clone(),
                    explanation,
                ));
            }
        }

        entailments
    }

    /// Get highly probable entailments (>= 0.8)
    pub fn highly_probable_entailments(
        &self,
        evidence: &[(&str, bool)],
    ) -> Vec<ProbabilisticEntailment> {
        self.entail(evidence)
            .into_iter()
            .filter(|e| e.is_highly_probable())
            .collect()
    }

    /// Get the underlying network
    pub fn network(&self) -> &BayesianNetwork {
        &self.network
    }

    /// Get the threshold
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Get the number of simulations
    pub fn simulations(&self) -> usize {
        self.simulations
    }
}

/// Risk quantification for legal decisions
///
/// Assesses risk levels associated with legal decisions using
/// probabilistic analysis and Monte Carlo simulation.
///
/// # Examples
///
/// ```
/// use legalis_core::probabilistic::{RiskQuantifier, BayesianNetwork};
///
/// let mut network = BayesianNetwork::new();
/// network.add_node("regulatory_compliance", 0.85);
/// network.add_node("financial_penalty_risk", 0.15);
/// network.add_conditional_probability(
///     "overall_risk",
///     &["regulatory_compliance", "financial_penalty_risk"],
///     0.20
/// );
///
/// let quantifier = RiskQuantifier::new(network);
/// let risk_level = quantifier.quantify_risk("overall_risk", &[
///     ("regulatory_compliance", false),
///     ("financial_penalty_risk", true)
/// ]);
///
/// println!("Risk probability: {:.2}%", risk_level.risk_probability * 100.0);
/// assert!(risk_level.risk_probability > 0.0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RiskQuantifier {
    /// The underlying Bayesian network
    network: BayesianNetwork,
    /// Number of simulations for risk assessment
    simulations: usize,
}

/// Result of risk quantification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RiskLevel {
    /// Risk identifier
    pub risk_id: String,
    /// Probability of the risk occurring
    pub risk_probability: f64,
    /// Confidence interval for the risk
    pub confidence_interval: (f64, f64),
    /// Risk category (Low, Moderate, High, Critical)
    pub category: RiskCategory,
    /// Explanation of the risk assessment
    pub explanation: String,
}

/// Risk category classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskCategory {
    /// Low risk (< 0.25)
    Low,
    /// Moderate risk (0.25 - 0.5)
    Moderate,
    /// High risk (0.5 - 0.75)
    High,
    /// Critical risk (>= 0.75)
    Critical,
}

impl RiskCategory {
    /// Get risk category from probability
    pub fn from_probability(probability: f64) -> Self {
        match probability {
            p if p >= 0.75 => RiskCategory::Critical,
            p if p >= 0.5 => RiskCategory::High,
            p if p >= 0.25 => RiskCategory::Moderate,
            _ => RiskCategory::Low,
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> &str {
        match self {
            RiskCategory::Low => "Low risk - proceed with normal monitoring",
            RiskCategory::Moderate => "Moderate risk - enhanced monitoring recommended",
            RiskCategory::High => "High risk - immediate attention required",
            RiskCategory::Critical => "Critical risk - urgent action needed",
        }
    }
}

impl RiskLevel {
    /// Create a new risk level
    pub fn new(
        risk_id: impl Into<String>,
        risk_probability: f64,
        confidence_interval: (f64, f64),
        explanation: impl Into<String>,
    ) -> Self {
        let category = RiskCategory::from_probability(risk_probability);
        Self {
            risk_id: risk_id.into(),
            risk_probability,
            confidence_interval,
            category,
            explanation: explanation.into(),
        }
    }

    /// Check if risk is acceptable (< 0.5)
    pub fn is_acceptable(&self) -> bool {
        self.risk_probability < 0.5
    }

    /// Check if risk requires immediate action
    pub fn requires_immediate_action(&self) -> bool {
        matches!(self.category, RiskCategory::High | RiskCategory::Critical)
    }
}

impl RiskQuantifier {
    /// Create a new risk quantifier
    pub fn new(network: BayesianNetwork) -> Self {
        Self {
            network,
            simulations: 5000,
        }
    }

    /// Create a quantifier with custom simulation count
    pub fn with_simulations(network: BayesianNetwork, simulations: usize) -> Self {
        Self {
            network,
            simulations,
        }
    }

    /// Quantify risk for a specific risk proposition
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::probabilistic::{RiskQuantifier, BayesianNetwork, RiskCategory};
    ///
    /// let mut network = BayesianNetwork::new();
    /// network.add_node("lawsuit_risk", 0.30);
    ///
    /// let quantifier = RiskQuantifier::new(network);
    /// let risk = quantifier.quantify_risk("lawsuit_risk", &[]);
    ///
    /// assert_eq!(risk.category, RiskCategory::Moderate);
    /// ```
    pub fn quantify_risk(&self, risk_id: &str, evidence: &[(&str, bool)]) -> RiskLevel {
        let probability = self.network.query(risk_id, evidence);

        // Use Monte Carlo for confidence interval
        let simulator = MonteCarloSimulator::new(self.network.clone());
        let sim_result = simulator.simulate_with_evidence(risk_id, evidence, self.simulations);

        let category = RiskCategory::from_probability(probability);
        let explanation = format!(
            "{} - Probability: {:.1}%, CI: ({:.2}, {:.2})",
            category.description(),
            probability * 100.0,
            sim_result.confidence_interval.0,
            sim_result.confidence_interval.1
        );

        RiskLevel::new(
            risk_id,
            probability,
            sim_result.confidence_interval,
            explanation,
        )
    }

    /// Assess multiple risks simultaneously
    pub fn assess_risks(&self, risk_ids: &[&str], evidence: &[(&str, bool)]) -> Vec<RiskLevel> {
        risk_ids
            .iter()
            .map(|risk_id| self.quantify_risk(risk_id, evidence))
            .collect()
    }

    /// Get high-priority risks (High or Critical)
    pub fn high_priority_risks(
        &self,
        risk_ids: &[&str],
        evidence: &[(&str, bool)],
    ) -> Vec<RiskLevel> {
        self.assess_risks(risk_ids, evidence)
            .into_iter()
            .filter(|r| r.requires_immediate_action())
            .collect()
    }

    /// Get the underlying network
    pub fn network(&self) -> &BayesianNetwork {
        &self.network
    }

    /// Get the number of simulations
    pub fn simulations(&self) -> usize {
        self.simulations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bayesian_node_creation() {
        let node = BayesianNode::new("test", 0.7);
        assert_eq!(node.id, "test");
        assert_eq!(node.prior, 0.7);
        assert!(node.parents.is_empty());
    }

    #[test]
    fn test_bayesian_network_basic() {
        let mut network = BayesianNetwork::new();
        network.add_node("A", 0.6);
        assert_eq!(network.node_count(), 1);
        assert!(network.contains_node("A"));
    }

    #[test]
    fn test_conditional_probability() {
        let mut network = BayesianNetwork::new();
        network.add_node("parent", 0.7);
        network.add_conditional_probability("child", &["parent"], 0.9);

        let prob = network.query("child", &[("parent", true)]);
        assert!((prob - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_probabilistic_evaluator() {
        let mut network = BayesianNetwork::new();
        network.add_node("condition", 0.8);

        let evaluator = ProbabilisticEvaluator::new(network);
        let evidence: &[(&str, bool)] = &[];
        let result = evaluator.evaluate("condition", evidence);

        assert!(result.outcome);
        assert!((result.confidence - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_probabilistic_result_confidence_levels() {
        let high = ProbabilisticResult::new(true, 0.95, "test");
        assert_eq!(high.confidence_level(), "Very High");
        assert!(high.is_highly_confident());

        let low = ProbabilisticResult::new(false, 0.3, "test");
        assert_eq!(low.confidence_level(), "Very Low");
        assert!(low.is_uncertain());
    }

    #[test]
    fn test_network_node_removal() {
        let mut network = BayesianNetwork::new();
        network.add_node("temp", 0.5);
        assert!(network.contains_node("temp"));

        network.remove_node("temp");
        assert!(!network.contains_node("temp"));
    }

    #[test]
    fn test_evaluator_threshold() {
        let mut network = BayesianNetwork::new();
        network.add_node("test", 0.6);

        let mut evaluator = ProbabilisticEvaluator::with_threshold(network, 0.7);
        assert_eq!(evaluator.threshold(), 0.7);

        evaluator.set_threshold(0.5);
        assert_eq!(evaluator.threshold(), 0.5);
    }

    #[test]
    fn test_monte_carlo_simulation() {
        let mut network = BayesianNetwork::new();
        network.add_node("condition", 0.75);

        let simulator = MonteCarloSimulator::with_seed(network, 42);
        let result = simulator.simulate("condition", 1000);

        assert_eq!(result.iterations, 1000);
        assert!(result.mean > 0.5 && result.mean < 1.0);
        assert!(result.std_dev >= 0.0);
        assert_eq!(result.distribution.len(), 10);
    }

    #[test]
    fn test_monte_carlo_with_evidence() {
        let mut network = BayesianNetwork::new();
        network.add_node("parent", 0.8);
        network.add_conditional_probability("child", &["parent"], 0.9);

        let simulator = MonteCarloSimulator::with_seed(network, 42);
        let result = simulator.simulate_with_evidence("child", &[("parent", true)], 500);

        assert_eq!(result.iterations, 500);
        assert!(result.mean > 0.7);
    }

    #[test]
    fn test_simulation_result_helpers() {
        let result = SimulationResult::new(1000, 0.75, 0.05, 0.5, 0.95, (0.70, 0.80), vec![]);

        assert!(result.is_significant());
        assert!((result.confidence_width() - 0.10).abs() < 0.001);
        assert!((result.coefficient_of_variation() - 0.0667).abs() < 0.001);
    }

    #[test]
    fn test_simulator_seed() {
        let mut network = BayesianNetwork::new();
        network.add_node("test", 0.5);

        let simulator = MonteCarloSimulator::with_seed(network, 123);
        assert_eq!(simulator.seed(), Some(123));
    }

    #[test]
    fn test_probabilistic_entailment_creation() {
        let entailment = ProbabilisticEntailment::new(
            "test_proposition",
            0.85,
            (0.80, 0.90),
            vec![("evidence".to_string(), true)],
            "Test explanation",
        );

        assert_eq!(entailment.proposition, "test_proposition");
        assert!(entailment.is_highly_probable());
        assert!(!entailment.is_uncertain());
        assert_eq!(entailment.probability_category(), "High");
    }

    #[test]
    fn test_probabilistic_entailment_engine() {
        let mut network = BayesianNetwork::new();
        network.add_node("condition_a", 0.7);
        network.add_conditional_probability("conclusion", &["condition_a"], 0.9);

        let engine = ProbabilisticEntailmentEngine::new(network);
        let entailments = engine.entail(&[("condition_a", true)]);

        assert!(!entailments.is_empty());
        assert!(entailments.iter().any(|e| e.proposition == "conclusion"));
    }

    #[test]
    fn test_highly_probable_entailments() {
        let mut network = BayesianNetwork::new();
        network.add_node("low_prob", 0.3);
        network.add_node("high_prob", 0.9);

        let engine = ProbabilisticEntailmentEngine::new(network);
        let high = engine.highly_probable_entailments(&[]);

        assert!(high.iter().any(|e| e.proposition == "high_prob"));
        assert!(!high.iter().any(|e| e.proposition == "low_prob"));
    }

    #[test]
    fn test_risk_category_from_probability() {
        assert_eq!(RiskCategory::from_probability(0.1), RiskCategory::Low);
        assert_eq!(RiskCategory::from_probability(0.3), RiskCategory::Moderate);
        assert_eq!(RiskCategory::from_probability(0.6), RiskCategory::High);
        assert_eq!(RiskCategory::from_probability(0.8), RiskCategory::Critical);
    }

    #[test]
    fn test_risk_quantifier() {
        let mut network = BayesianNetwork::new();
        network.add_node("compliance_risk", 0.35);

        let quantifier = RiskQuantifier::new(network);
        let risk = quantifier.quantify_risk("compliance_risk", &[]);

        assert_eq!(risk.risk_id, "compliance_risk");
        assert_eq!(risk.category, RiskCategory::Moderate);
        assert!(risk.is_acceptable());
        assert!(!risk.requires_immediate_action());
    }

    #[test]
    fn test_risk_assessment_multiple() {
        let mut network = BayesianNetwork::new();
        network.add_node("risk_a", 0.2);
        network.add_node("risk_b", 0.8);

        let quantifier = RiskQuantifier::new(network);
        let risks = quantifier.assess_risks(&["risk_a", "risk_b"], &[]);

        assert_eq!(risks.len(), 2);
        assert!(risks[0].is_acceptable());
        assert!(risks[1].requires_immediate_action());
    }

    #[test]
    fn test_high_priority_risks() {
        let mut network = BayesianNetwork::new();
        network.add_node("low_risk", 0.1);
        network.add_node("high_risk", 0.7);

        let quantifier = RiskQuantifier::new(network);
        let high_priority = quantifier.high_priority_risks(&["low_risk", "high_risk"], &[]);

        assert_eq!(high_priority.len(), 1);
        assert_eq!(high_priority[0].risk_id, "high_risk");
    }
}
