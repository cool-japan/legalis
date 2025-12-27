//! Scenario planning and analysis for policy futures.
//!
//! This module provides tools for exploring different policy scenarios,
//! analyzing scenario trees, and comparing probability-weighted outcomes.

use crate::{SimResult, SimulationMetrics};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A policy scenario with associated probability and metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    /// Unique identifier for the scenario
    pub id: String,
    /// Descriptive name
    pub name: String,
    /// Description of the scenario
    pub description: String,
    /// Probability of this scenario occurring (0.0 to 1.0)
    pub probability: f64,
    /// Simulation metrics for this scenario
    pub metrics: SimulationMetrics,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Scenario {
    /// Creates a new scenario.
    pub fn new(
        id: String,
        name: String,
        description: String,
        probability: f64,
        metrics: SimulationMetrics,
    ) -> SimResult<Self> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Probability must be between 0 and 1".to_string(),
            ));
        }

        Ok(Self {
            id,
            name,
            description,
            probability,
            metrics,
            tags: Vec::new(),
        })
    }

    /// Adds tags to the scenario.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Returns the expected value (probability-weighted deterministic ratio).
    pub fn expected_value(&self) -> f64 {
        self.probability * self.metrics.deterministic_ratio()
    }
}

/// Collection of scenarios for analysis.
#[derive(Debug, Clone)]
pub struct ScenarioSet {
    /// Scenarios in this set
    pub scenarios: Vec<Scenario>,
    /// Name of the scenario set
    pub name: String,
}

impl ScenarioSet {
    /// Creates a new scenario set.
    pub fn new(name: String) -> Self {
        Self {
            scenarios: Vec::new(),
            name,
        }
    }

    /// Adds a scenario to the set.
    pub fn add_scenario(mut self, scenario: Scenario) -> Self {
        self.scenarios.push(scenario);
        self
    }

    /// Validates that probabilities sum to 1.0.
    pub fn validate_probabilities(&self) -> SimResult<()> {
        let total_prob: f64 = self.scenarios.iter().map(|s| s.probability).sum();
        if (total_prob - 1.0).abs() > 0.01 {
            return Err(crate::SimulationError::InvalidConfiguration(format!(
                "Probabilities must sum to 1.0, got {:.3}",
                total_prob
            )));
        }
        Ok(())
    }

    /// Calculates the expected value across all scenarios.
    pub fn expected_value(&self) -> f64 {
        self.scenarios.iter().map(|s| s.expected_value()).sum()
    }

    /// Finds the best-case scenario (highest deterministic ratio).
    pub fn best_case(&self) -> Option<&Scenario> {
        self.scenarios.iter().max_by(|a, b| {
            a.metrics
                .deterministic_ratio()
                .partial_cmp(&b.metrics.deterministic_ratio())
                .unwrap()
        })
    }

    /// Finds the worst-case scenario (lowest deterministic ratio).
    pub fn worst_case(&self) -> Option<&Scenario> {
        self.scenarios.iter().min_by(|a, b| {
            a.metrics
                .deterministic_ratio()
                .partial_cmp(&b.metrics.deterministic_ratio())
                .unwrap()
        })
    }

    /// Finds the most likely scenario (highest probability).
    pub fn most_likely(&self) -> Option<&Scenario> {
        self.scenarios
            .iter()
            .max_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap())
    }

    /// Calculates the variance of outcomes.
    pub fn variance(&self) -> f64 {
        let expected = self.expected_value();
        self.scenarios
            .iter()
            .map(|s| {
                let diff = s.metrics.deterministic_ratio() - expected;
                s.probability * diff * diff
            })
            .sum()
    }

    /// Calculates the standard deviation of outcomes.
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    /// Generates a comparison report.
    pub fn comparison_report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("=== Scenario Analysis: {} ===\n\n", self.name));

        report.push_str(&format!(
            "Expected Value: {:.2}%\n",
            self.expected_value() * 100.0
        ));
        report.push_str(&format!(
            "Standard Deviation: {:.2}%\n\n",
            self.std_dev() * 100.0
        ));

        if let Some(best) = self.best_case() {
            report.push_str(&format!(
                "Best Case: {} ({:.2}%)\n",
                best.name,
                best.metrics.deterministic_ratio() * 100.0
            ));
        }

        if let Some(worst) = self.worst_case() {
            report.push_str(&format!(
                "Worst Case: {} ({:.2}%)\n",
                worst.name,
                worst.metrics.deterministic_ratio() * 100.0
            ));
        }

        if let Some(likely) = self.most_likely() {
            report.push_str(&format!(
                "Most Likely: {} (p={:.2})\n\n",
                likely.name, likely.probability
            ));
        }

        report.push_str("All Scenarios:\n");
        for scenario in &self.scenarios {
            report.push_str(&format!(
                "  {} (p={:.2}): D={:.1}%, J={:.1}%\n",
                scenario.name,
                scenario.probability,
                scenario.metrics.deterministic_ratio() * 100.0,
                scenario.metrics.discretion_ratio() * 100.0
            ));
        }

        report
    }
}

/// A node in a scenario tree representing a decision point or outcome.
#[derive(Debug, Clone)]
pub struct ScenarioNode {
    /// Node identifier
    pub id: String,
    /// Node description
    pub description: String,
    /// Probability of reaching this node
    pub probability: f64,
    /// Metrics at this node
    pub metrics: Option<SimulationMetrics>,
    /// Child nodes
    pub children: Vec<ScenarioNode>,
}

impl ScenarioNode {
    /// Creates a new scenario node.
    pub fn new(id: String, description: String, probability: f64) -> SimResult<Self> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Probability must be between 0 and 1".to_string(),
            ));
        }

        Ok(Self {
            id,
            description,
            probability,
            metrics: None,
            children: Vec::new(),
        })
    }

    /// Sets metrics for this node.
    pub fn with_metrics(mut self, metrics: SimulationMetrics) -> Self {
        self.metrics = Some(metrics);
        self
    }

    /// Adds a child node.
    pub fn add_child(mut self, child: ScenarioNode) -> Self {
        self.children.push(child);
        self
    }

    /// Calculates the total probability of all leaf nodes.
    pub fn total_leaf_probability(&self) -> f64 {
        if self.children.is_empty() {
            self.probability
        } else {
            self.children
                .iter()
                .map(|child| child.total_leaf_probability() * self.probability)
                .sum()
        }
    }

    /// Collects all leaf scenarios.
    pub fn leaf_scenarios(&self) -> Vec<Scenario> {
        self.collect_leaves(self.probability, &self.id)
    }

    fn collect_leaves(&self, cumulative_prob: f64, path: &str) -> Vec<Scenario> {
        if self.children.is_empty() {
            // Leaf node
            if let Some(metrics) = &self.metrics {
                vec![Scenario {
                    id: self.id.clone(),
                    name: self.description.clone(),
                    description: path.to_string(),
                    probability: cumulative_prob,
                    metrics: metrics.clone(),
                    tags: Vec::new(),
                }]
            } else {
                Vec::new()
            }
        } else {
            // Internal node - collect from children
            let mut scenarios = Vec::new();
            for child in &self.children {
                let new_path = format!("{} -> {}", path, child.description);
                let child_prob = cumulative_prob * child.probability;
                scenarios.extend(child.collect_leaves(child_prob, &new_path));
            }
            scenarios
        }
    }
}

/// Scenario tree for decision analysis.
#[derive(Debug, Clone)]
pub struct ScenarioTree {
    /// Root node of the tree
    pub root: ScenarioNode,
    /// Name of the scenario tree
    pub name: String,
}

impl ScenarioTree {
    /// Creates a new scenario tree.
    pub fn new(name: String, root: ScenarioNode) -> Self {
        Self { root, name }
    }

    /// Converts the tree to a scenario set.
    pub fn to_scenario_set(&self) -> ScenarioSet {
        let scenarios = self.root.leaf_scenarios();
        let mut set = ScenarioSet::new(self.name.clone());
        for scenario in scenarios {
            set = set.add_scenario(scenario);
        }
        set
    }

    /// Calculates expected value over the entire tree.
    pub fn expected_value(&self) -> f64 {
        self.to_scenario_set().expected_value()
    }
}

/// Sensitivity analysis for scenario assumptions.
#[derive(Debug, Clone)]
pub struct ScenarioSensitivity {
    /// Base scenario set
    pub base_scenarios: ScenarioSet,
    /// Alternative probability distributions
    pub probability_variants: HashMap<String, Vec<f64>>,
}

impl ScenarioSensitivity {
    /// Creates a new sensitivity analysis.
    pub fn new(base_scenarios: ScenarioSet) -> Self {
        Self {
            base_scenarios,
            probability_variants: HashMap::new(),
        }
    }

    /// Adds a probability variant for testing.
    pub fn add_variant(mut self, name: String, probabilities: Vec<f64>) -> SimResult<Self> {
        if probabilities.len() != self.base_scenarios.scenarios.len() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Probability variant must have same length as scenarios".to_string(),
            ));
        }

        let sum: f64 = probabilities.iter().sum();
        if (sum - 1.0).abs() > 0.01 {
            return Err(crate::SimulationError::InvalidConfiguration(format!(
                "Probabilities must sum to 1.0, got {:.3}",
                sum
            )));
        }

        self.probability_variants.insert(name, probabilities);
        Ok(self)
    }

    /// Calculates expected values under different probability assumptions.
    pub fn sensitivity_results(&self) -> HashMap<String, f64> {
        let mut results = HashMap::new();

        // Base case
        results.insert("Base".to_string(), self.base_scenarios.expected_value());

        // Variants
        for (variant_name, probs) in &self.probability_variants {
            let mut expected = 0.0;
            for (i, scenario) in self.base_scenarios.scenarios.iter().enumerate() {
                expected += probs[i] * scenario.metrics.deterministic_ratio();
            }
            results.insert(variant_name.clone(), expected);
        }

        results
    }

    /// Generates a sensitivity report.
    pub fn sensitivity_report(&self) -> String {
        let results = self.sensitivity_results();
        let base_value = results.get("Base").unwrap_or(&0.0);

        let mut report = String::new();
        report.push_str("=== Scenario Sensitivity Analysis ===\n\n");
        report.push_str(&format!(
            "Base Expected Value: {:.2}%\n\n",
            base_value * 100.0
        ));

        report.push_str("Sensitivity to Probability Assumptions:\n");
        for (name, value) in &results {
            if name != "Base" {
                let diff = value - base_value;
                report.push_str(&format!(
                    "  {}: {:.2}% ({:+.2}%)\n",
                    name,
                    value * 100.0,
                    diff * 100.0
                ));
            }
        }

        report
    }
}

/// Scenario ranking by multiple criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioRanking {
    /// Scenario ID
    pub scenario_id: String,
    /// Overall score (0.0 to 1.0)
    pub score: f64,
    /// Component scores
    pub components: HashMap<String, f64>,
}

/// Multi-criteria scenario evaluator.
#[derive(Debug, Clone)]
pub struct ScenarioEvaluator {
    /// Criteria weights (must sum to 1.0)
    pub criteria_weights: HashMap<String, f64>,
}

impl ScenarioEvaluator {
    /// Creates a new evaluator with criteria weights.
    pub fn new(criteria_weights: HashMap<String, f64>) -> SimResult<Self> {
        let sum: f64 = criteria_weights.values().sum();
        if (sum - 1.0).abs() > 0.01 {
            return Err(crate::SimulationError::InvalidConfiguration(format!(
                "Criteria weights must sum to 1.0, got {:.3}",
                sum
            )));
        }

        Ok(Self { criteria_weights })
    }

    /// Evaluates a scenario set and returns rankings.
    pub fn rank_scenarios(&self, scenario_set: &ScenarioSet) -> Vec<ScenarioRanking> {
        let mut rankings = Vec::new();

        for scenario in &scenario_set.scenarios {
            let mut components = HashMap::new();
            let mut total_score = 0.0;

            // Deterministic ratio criterion
            if let Some(weight) = self.criteria_weights.get("deterministic") {
                let score = scenario.metrics.deterministic_ratio();
                components.insert("deterministic".to_string(), score);
                total_score += weight * score;
            }

            // Probability criterion
            if let Some(weight) = self.criteria_weights.get("probability") {
                components.insert("probability".to_string(), scenario.probability);
                total_score += weight * scenario.probability;
            }

            // Low discretion criterion
            if let Some(weight) = self.criteria_weights.get("low_discretion") {
                let score = 1.0 - scenario.metrics.discretion_ratio();
                components.insert("low_discretion".to_string(), score);
                total_score += weight * score;
            }

            rankings.push(ScenarioRanking {
                scenario_id: scenario.id.clone(),
                score: total_score,
                components,
            });
        }

        // Sort by score descending
        rankings.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        rankings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::LawApplicationResult;
    use legalis_core::{Effect, EffectType, LegalResult};
    use uuid::Uuid;

    fn create_test_metrics(det_ratio: f64) -> SimulationMetrics {
        let mut metrics = SimulationMetrics::new();
        let det_count = (det_ratio * 100.0) as usize;
        let disc_count = 100 - det_count;

        for _ in 0..det_count {
            metrics.record_result(&LawApplicationResult {
                agent_id: Uuid::new_v4(),
                statute_id: "test".to_string(),
                result: LegalResult::Deterministic(Effect::new(EffectType::Grant, "Test")),
            });
        }

        for _ in 0..disc_count {
            metrics.record_result(&LawApplicationResult {
                agent_id: Uuid::new_v4(),
                statute_id: "test".to_string(),
                result: LegalResult::JudicialDiscretion {
                    issue: "Test".to_string(),
                    context_id: Uuid::new_v4(),
                    narrative_hint: None,
                },
            });
        }

        metrics
    }

    #[test]
    fn test_scenario_creation() {
        let metrics = create_test_metrics(0.7);
        let scenario = Scenario::new(
            "s1".to_string(),
            "Test Scenario".to_string(),
            "A test scenario".to_string(),
            0.5,
            metrics,
        )
        .unwrap();

        assert_eq!(scenario.id, "s1");
        assert_eq!(scenario.probability, 0.5);
        assert_eq!(scenario.expected_value(), 0.5 * 0.7);
    }

    #[test]
    fn test_invalid_probability() {
        let metrics = create_test_metrics(0.7);
        assert!(
            Scenario::new(
                "s1".to_string(),
                "Test".to_string(),
                "Test".to_string(),
                1.5,
                metrics
            )
            .is_err()
        );
    }

    #[test]
    fn test_scenario_set() {
        let s1 = Scenario::new(
            "s1".to_string(),
            "Optimistic".to_string(),
            "Best case".to_string(),
            0.3,
            create_test_metrics(0.9),
        )
        .unwrap();

        let s2 = Scenario::new(
            "s2".to_string(),
            "Pessimistic".to_string(),
            "Worst case".to_string(),
            0.7,
            create_test_metrics(0.5),
        )
        .unwrap();

        let set = ScenarioSet::new("Test Set".to_string())
            .add_scenario(s1)
            .add_scenario(s2);

        assert_eq!(set.scenarios.len(), 2);
        assert!(set.validate_probabilities().is_ok());

        let expected = 0.3 * 0.9 + 0.7 * 0.5;
        assert!((set.expected_value() - expected).abs() < 0.01);

        assert!(set.best_case().is_some());
        assert!(set.worst_case().is_some());
        assert!(set.most_likely().is_some());
    }

    #[test]
    fn test_scenario_tree() {
        let root = ScenarioNode::new("root".to_string(), "Start".to_string(), 1.0)
            .unwrap()
            .add_child(
                ScenarioNode::new("child1".to_string(), "Option A".to_string(), 0.6)
                    .unwrap()
                    .with_metrics(create_test_metrics(0.8)),
            )
            .add_child(
                ScenarioNode::new("child2".to_string(), "Option B".to_string(), 0.4)
                    .unwrap()
                    .with_metrics(create_test_metrics(0.6)),
            );

        let tree = ScenarioTree::new("Test Tree".to_string(), root);
        let scenario_set = tree.to_scenario_set();

        assert_eq!(scenario_set.scenarios.len(), 2);
    }

    #[test]
    fn test_scenario_sensitivity() {
        let s1 = Scenario::new(
            "s1".to_string(),
            "S1".to_string(),
            "Scenario 1".to_string(),
            0.5,
            create_test_metrics(0.8),
        )
        .unwrap();

        let s2 = Scenario::new(
            "s2".to_string(),
            "S2".to_string(),
            "Scenario 2".to_string(),
            0.5,
            create_test_metrics(0.6),
        )
        .unwrap();

        let set = ScenarioSet::new("Test".to_string())
            .add_scenario(s1)
            .add_scenario(s2);

        let sensitivity = ScenarioSensitivity::new(set)
            .add_variant("More S1".to_string(), vec![0.7, 0.3])
            .unwrap();

        let results = sensitivity.sensitivity_results();
        assert!(results.contains_key("Base"));
        assert!(results.contains_key("More S1"));
    }

    #[test]
    fn test_scenario_evaluator() {
        let mut weights = HashMap::new();
        weights.insert("deterministic".to_string(), 0.6);
        weights.insert("probability".to_string(), 0.4);

        let evaluator = ScenarioEvaluator::new(weights).unwrap();

        let s1 = Scenario::new(
            "s1".to_string(),
            "S1".to_string(),
            "Scenario 1".to_string(),
            0.8,
            create_test_metrics(0.7),
        )
        .unwrap();

        let s2 = Scenario::new(
            "s2".to_string(),
            "S2".to_string(),
            "Scenario 2".to_string(),
            0.2,
            create_test_metrics(0.9),
        )
        .unwrap();

        let set = ScenarioSet::new("Test".to_string())
            .add_scenario(s1)
            .add_scenario(s2);

        let rankings = evaluator.rank_scenarios(&set);
        assert_eq!(rankings.len(), 2);
        assert!(rankings[0].score >= rankings[1].score);
    }

    #[test]
    fn test_scenario_variance() {
        let s1 = Scenario::new(
            "s1".to_string(),
            "S1".to_string(),
            "Scenario 1".to_string(),
            0.5,
            create_test_metrics(0.9),
        )
        .unwrap();

        let s2 = Scenario::new(
            "s2".to_string(),
            "S2".to_string(),
            "Scenario 2".to_string(),
            0.5,
            create_test_metrics(0.5),
        )
        .unwrap();

        let set = ScenarioSet::new("Test".to_string())
            .add_scenario(s1)
            .add_scenario(s2);

        let variance = set.variance();
        let std_dev = set.std_dev();

        assert!(variance > 0.0);
        assert!((std_dev - variance.sqrt()).abs() < 0.001);
    }

    #[test]
    fn test_comparison_report() {
        let s1 = Scenario::new(
            "s1".to_string(),
            "Optimistic".to_string(),
            "Best case".to_string(),
            0.3,
            create_test_metrics(0.9),
        )
        .unwrap();

        let set = ScenarioSet::new("Test Set".to_string()).add_scenario(s1);

        let report = set.comparison_report();
        assert!(report.contains("Scenario Analysis"));
        assert!(report.contains("Expected Value"));
        assert!(report.contains("Optimistic"));
    }
}
