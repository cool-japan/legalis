//! Policy Analysis Module
//!
//! This module provides tools for comprehensive policy analysis including:
//! - Multi-objective policy optimization
//! - Policy sensitivity analysis and dashboards
//! - Distributional impact visualization
//! - Stakeholder impact matrices
//! - Policy comparison frameworks

use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a policy objective to be optimized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyObjective {
    /// Name of the objective
    pub name: String,
    /// Description of what is being optimized
    pub description: String,
    /// Weight of this objective (higher = more important)
    pub weight: f64,
    /// Whether to maximize (true) or minimize (false)
    pub maximize: bool,
    /// Target value if known
    pub target: Option<f64>,
}

impl PolicyObjective {
    /// Create a new policy objective
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        weight: f64,
        maximize: bool,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            weight,
            maximize,
            target: None,
        }
    }

    /// Set a target value for this objective
    pub fn with_target(mut self, target: f64) -> Self {
        self.target = Some(target);
        self
    }

    /// Evaluate this objective given a metric value
    /// Returns a normalized score (0.0 to 1.0)
    pub fn evaluate(&self, value: f64) -> f64 {
        if let Some(target) = self.target {
            // Score based on distance to target
            let distance = (value - target).abs();
            let max_distance = target.abs().max(1.0);
            (1.0 - (distance / max_distance)).max(0.0)
        } else {
            // Simple normalization (assumes positive values)
            if self.maximize {
                value.max(0.0)
            } else {
                // For minimization, invert the score
                if value > 0.0 { 1.0 / value } else { 1.0 }
            }
        }
    }
}

/// Multi-objective optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiObjectiveResult {
    /// Policy parameters that were tested
    pub parameters: HashMap<String, f64>,
    /// Scores for each objective
    pub objective_scores: HashMap<String, f64>,
    /// Weighted total score
    pub total_score: f64,
    /// Whether this is a Pareto optimal solution
    pub pareto_optimal: bool,
}

/// Multi-objective policy optimizer
#[derive(Debug, Clone)]
pub struct MultiObjectiveOptimizer {
    objectives: Vec<PolicyObjective>,
    results: Vec<MultiObjectiveResult>,
}

impl MultiObjectiveOptimizer {
    /// Create a new multi-objective optimizer
    pub fn new() -> Self {
        Self {
            objectives: Vec::new(),
            results: Vec::new(),
        }
    }

    /// Add an objective to optimize
    pub fn add_objective(&mut self, objective: PolicyObjective) {
        self.objectives.push(objective);
    }

    /// Evaluate a policy configuration
    pub fn evaluate(
        &self,
        parameters: HashMap<String, f64>,
        metrics: &HashMap<String, f64>,
    ) -> SimResult<MultiObjectiveResult> {
        let mut objective_scores = HashMap::new();
        let mut total_score = 0.0;

        for objective in &self.objectives {
            let value = metrics.get(&objective.name).ok_or_else(|| {
                SimulationError::InvalidConfiguration(format!(
                    "Metric '{}' not found",
                    objective.name
                ))
            })?;

            let score = objective.evaluate(*value);
            objective_scores.insert(objective.name.clone(), score);
            total_score += score * objective.weight;
        }

        Ok(MultiObjectiveResult {
            parameters,
            objective_scores,
            total_score,
            pareto_optimal: false, // Will be determined later
        })
    }

    /// Add a result to the optimizer
    pub fn add_result(&mut self, result: MultiObjectiveResult) {
        self.results.push(result);
    }

    /// Find Pareto optimal solutions
    pub fn find_pareto_frontier(&mut self) -> Vec<MultiObjectiveResult> {
        let n = self.results.len();
        let mut dominated = vec![false; n];

        // Check dominance
        for i in 0..n {
            for (j, dom) in dominated.iter_mut().enumerate().take(n) {
                if i == j {
                    continue;
                }

                let dominates = self.dominates(i, j);
                if dominates {
                    *dom = true;
                }
            }
        }

        // Mark Pareto optimal solutions
        for (i, result) in self.results.iter_mut().enumerate() {
            result.pareto_optimal = !dominated[i];
        }

        self.results
            .iter()
            .filter(|r| r.pareto_optimal)
            .cloned()
            .collect()
    }

    /// Check if result i dominates result j
    fn dominates(&self, i: usize, j: usize) -> bool {
        let result_i = &self.results[i];
        let result_j = &self.results[j];

        let mut better_in_at_least_one = false;
        for objective in &self.objectives {
            let score_i = result_i
                .objective_scores
                .get(&objective.name)
                .unwrap_or(&0.0);
            let score_j = result_j
                .objective_scores
                .get(&objective.name)
                .unwrap_or(&0.0);

            if score_i < score_j {
                return false; // i is worse in at least one objective
            }
            if score_i > score_j {
                better_in_at_least_one = true;
            }
        }

        better_in_at_least_one
    }

    /// Get all results
    pub fn results(&self) -> &[MultiObjectiveResult] {
        &self.results
    }

    /// Get the best overall result by total score
    pub fn best_result(&self) -> Option<&MultiObjectiveResult> {
        self.results
            .iter()
            .max_by(|a, b| a.total_score.partial_cmp(&b.total_score).unwrap())
    }
}

impl Default for MultiObjectiveOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Policy sensitivity analysis data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityPoint {
    /// Parameter value
    pub parameter_value: f64,
    /// Resulting metric values
    pub metrics: HashMap<String, f64>,
}

/// Policy sensitivity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySensitivity {
    /// Parameter name
    pub parameter_name: String,
    /// Baseline parameter value
    pub baseline_value: f64,
    /// Data points for sensitivity analysis
    pub points: Vec<SensitivityPoint>,
}

impl PolicySensitivity {
    /// Create a new sensitivity analysis
    pub fn new(parameter_name: impl Into<String>, baseline_value: f64) -> Self {
        Self {
            parameter_name: parameter_name.into(),
            baseline_value,
            points: Vec::new(),
        }
    }

    /// Add a data point
    pub fn add_point(&mut self, parameter_value: f64, metrics: HashMap<String, f64>) {
        self.points.push(SensitivityPoint {
            parameter_value,
            metrics,
        });
    }

    /// Calculate sensitivity coefficient for a metric
    /// (percentage change in metric / percentage change in parameter)
    pub fn sensitivity_coefficient(&self, metric_name: &str) -> Option<f64> {
        if self.points.len() < 2 {
            return None;
        }

        // Find baseline metrics
        let baseline = self.points.iter().min_by(|a, b| {
            let diff_a = (a.parameter_value - self.baseline_value).abs();
            let diff_b = (b.parameter_value - self.baseline_value).abs();
            diff_a.partial_cmp(&diff_b).unwrap()
        })?;

        let baseline_metric = baseline.metrics.get(metric_name)?;

        // Calculate average sensitivity across all points
        let mut total_sensitivity = 0.0;
        let mut count = 0;

        for point in &self.points {
            if (point.parameter_value - baseline.parameter_value).abs() < 1e-10 {
                continue;
            }

            if let Some(metric_value) = point.metrics.get(metric_name) {
                let param_change =
                    (point.parameter_value - baseline.parameter_value) / baseline.parameter_value;
                let metric_change = (metric_value - baseline_metric) / baseline_metric;

                if param_change.abs() > 1e-10 {
                    total_sensitivity += metric_change / param_change;
                    count += 1;
                }
            }
        }

        if count > 0 {
            Some(total_sensitivity / count as f64)
        } else {
            None
        }
    }

    /// Get metric value at a specific parameter value
    pub fn get_metric_at(&self, parameter_value: f64, metric_name: &str) -> Option<f64> {
        self.points
            .iter()
            .find(|p| (p.parameter_value - parameter_value).abs() < 1e-10)
            .and_then(|p| p.metrics.get(metric_name).copied())
    }
}

/// Stakeholder group for impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stakeholder {
    /// Name of the stakeholder group
    pub name: String,
    /// Description of the group
    pub description: String,
    /// Size/weight of this stakeholder group
    pub weight: f64,
}

impl Stakeholder {
    /// Create a new stakeholder
    pub fn new(name: impl Into<String>, description: impl Into<String>, weight: f64) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            weight,
        }
    }
}

/// Impact on a stakeholder group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeholderImpact {
    /// Stakeholder group
    pub stakeholder: String,
    /// Metric impacts (metric name -> value)
    pub impacts: HashMap<String, f64>,
    /// Overall impact score (positive = beneficial, negative = harmful)
    pub overall_score: f64,
}

/// Stakeholder impact matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeholderMatrix {
    /// Stakeholder groups
    stakeholders: Vec<Stakeholder>,
    /// Impacts by stakeholder
    impacts: HashMap<String, StakeholderImpact>,
}

impl StakeholderMatrix {
    /// Create a new stakeholder matrix
    pub fn new() -> Self {
        Self {
            stakeholders: Vec::new(),
            impacts: HashMap::new(),
        }
    }

    /// Add a stakeholder group
    pub fn add_stakeholder(&mut self, stakeholder: Stakeholder) {
        self.stakeholders.push(stakeholder);
    }

    /// Set impact for a stakeholder
    pub fn set_impact(
        &mut self,
        stakeholder_name: impl Into<String>,
        impacts: HashMap<String, f64>,
    ) {
        let name = stakeholder_name.into();

        // Calculate overall score as weighted sum
        let overall_score = impacts.values().sum();

        self.impacts.insert(
            name.clone(),
            StakeholderImpact {
                stakeholder: name,
                impacts,
                overall_score,
            },
        );
    }

    /// Get impact for a stakeholder
    pub fn get_impact(&self, stakeholder_name: &str) -> Option<&StakeholderImpact> {
        self.impacts.get(stakeholder_name)
    }

    /// Get all stakeholders
    pub fn stakeholders(&self) -> &[Stakeholder] {
        &self.stakeholders
    }

    /// Get winners and losers
    pub fn winners_and_losers(&self) -> (Vec<String>, Vec<String>) {
        let mut winners = Vec::new();
        let mut losers = Vec::new();

        for (name, impact) in &self.impacts {
            if impact.overall_score > 0.0 {
                winners.push(name.clone());
            } else if impact.overall_score < 0.0 {
                losers.push(name.clone());
            }
        }

        winners.sort_by(|a, b| {
            let score_a = self.impacts.get(a).unwrap().overall_score;
            let score_b = self.impacts.get(b).unwrap().overall_score;
            score_b.partial_cmp(&score_a).unwrap()
        });

        losers.sort_by(|a, b| {
            let score_a = self.impacts.get(a).unwrap().overall_score;
            let score_b = self.impacts.get(b).unwrap().overall_score;
            score_a.partial_cmp(&score_b).unwrap()
        });

        (winners, losers)
    }

    /// Generate a report
    pub fn report(&self) -> String {
        let mut report = String::from("Stakeholder Impact Matrix\n");
        report.push_str("==========================\n\n");

        for stakeholder in &self.stakeholders {
            report.push_str(&format!(
                "Stakeholder: {} (weight: {:.2})\n",
                stakeholder.name, stakeholder.weight
            ));
            report.push_str(&format!("  {}\n", stakeholder.description));

            if let Some(impact) = self.impacts.get(&stakeholder.name) {
                report.push_str(&format!("  Overall Score: {:.2}\n", impact.overall_score));
                report.push_str("  Impacts:\n");
                for (metric, value) in &impact.impacts {
                    report.push_str(&format!("    {}: {:.2}\n", metric, value));
                }
            }
            report.push('\n');
        }

        let (winners, losers) = self.winners_and_losers();
        report.push_str("Summary:\n");
        report.push_str(&format!("  Winners: {:?}\n", winners));
        report.push_str(&format!("  Losers: {:?}\n", losers));

        report
    }
}

impl Default for StakeholderMatrix {
    fn default() -> Self {
        Self::new()
    }
}

/// Policy distributional impact data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDistributionalImpact {
    /// Income/wealth decile (0-9, where 0 is lowest)
    pub decile: usize,
    /// Average impact for this decile
    pub average_impact: f64,
    /// Number of entities in this decile
    pub count: usize,
    /// Total impact for this decile
    pub total_impact: f64,
}

/// Policy distributional analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDistributionalAnalysis {
    /// Impact by decile
    pub impacts_by_decile: Vec<PolicyDistributionalImpact>,
    /// Gini coefficient before policy
    pub gini_before: f64,
    /// Gini coefficient after policy
    pub gini_after: f64,
    /// Whether the policy is progressive (benefits lower deciles more)
    pub is_progressive: bool,
}

impl PolicyDistributionalAnalysis {
    /// Create a new policy distributional analysis
    pub fn new(
        impacts_by_decile: Vec<PolicyDistributionalImpact>,
        gini_before: f64,
        gini_after: f64,
    ) -> Self {
        // Check if progressive: lower deciles should have higher average impact
        let is_progressive = if impacts_by_decile.len() >= 2 {
            let first_half: f64 = impacts_by_decile
                .iter()
                .take(impacts_by_decile.len() / 2)
                .map(|d| d.average_impact)
                .sum::<f64>()
                / (impacts_by_decile.len() / 2) as f64;

            let second_half: f64 = impacts_by_decile
                .iter()
                .skip(impacts_by_decile.len() / 2)
                .map(|d| d.average_impact)
                .sum::<f64>()
                / impacts_by_decile.len().div_ceil(2) as f64;

            first_half > second_half
        } else {
            false
        };

        Self {
            impacts_by_decile,
            gini_before,
            gini_after,
            is_progressive,
        }
    }

    /// Calculate the concentration index (similar to Gini for health/impact)
    pub fn concentration_index(&self) -> f64 {
        if self.impacts_by_decile.is_empty() {
            return 0.0;
        }

        let n = self.impacts_by_decile.len();
        let mut sum_product = 0.0;
        let mut sum_impact = 0.0;

        for (i, impact) in self.impacts_by_decile.iter().enumerate() {
            let rank = (i + 1) as f64;
            sum_product += rank * impact.average_impact;
            sum_impact += impact.average_impact;
        }

        if sum_impact == 0.0 {
            return 0.0;
        }

        let mean_impact = sum_impact / n as f64;
        (2.0 / (n as f64 * mean_impact)) * sum_product - (n as f64 + 1.0) / n as f64
    }

    /// Generate visualization data (suitable for charts)
    pub fn to_chart_data(&self) -> Vec<(String, f64)> {
        self.impacts_by_decile
            .iter()
            .map(|d| (format!("D{}", d.decile), d.average_impact))
            .collect()
    }
}

/// Policy comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyComparison {
    /// Name of policy A
    pub policy_a: String,
    /// Name of policy B
    pub policy_b: String,
    /// Metrics for policy A
    pub metrics_a: HashMap<String, f64>,
    /// Metrics for policy B
    pub metrics_b: HashMap<String, f64>,
    /// Differences (B - A)
    pub differences: HashMap<String, f64>,
    /// Percentage changes ((B - A) / A * 100)
    pub percentage_changes: HashMap<String, f64>,
}

impl PolicyComparison {
    /// Create a new policy comparison
    pub fn new(
        policy_a: impl Into<String>,
        policy_b: impl Into<String>,
        metrics_a: HashMap<String, f64>,
        metrics_b: HashMap<String, f64>,
    ) -> Self {
        let mut differences = HashMap::new();
        let mut percentage_changes = HashMap::new();

        for (key, value_b) in &metrics_b {
            if let Some(value_a) = metrics_a.get(key) {
                let diff = value_b - value_a;
                differences.insert(key.clone(), diff);

                if value_a.abs() > 1e-10 {
                    let pct_change = (diff / value_a) * 100.0;
                    percentage_changes.insert(key.clone(), pct_change);
                }
            }
        }

        Self {
            policy_a: policy_a.into(),
            policy_b: policy_b.into(),
            metrics_a,
            metrics_b,
            differences,
            percentage_changes,
        }
    }

    /// Get the better policy for a metric (higher is better)
    pub fn better_for(&self, metric: &str) -> Option<String> {
        let diff = self.differences.get(metric)?;
        if diff.abs() < 1e-10 {
            None // Tie
        } else if *diff > 0.0 {
            Some(self.policy_b.clone())
        } else {
            Some(self.policy_a.clone())
        }
    }

    /// Generate a comparison report
    pub fn report(&self) -> String {
        let mut report = String::from("Policy Comparison\n");
        report.push_str("=================\n\n");
        report.push_str(&format!("Policy A: {}\n", self.policy_a));
        report.push_str(&format!("Policy B: {}\n\n", self.policy_b));

        report.push_str("Metric Comparisons:\n");
        for (metric, diff) in &self.differences {
            let value_a = self.metrics_a.get(metric).unwrap_or(&0.0);
            let value_b = self.metrics_b.get(metric).unwrap_or(&0.0);
            let pct_change = self.percentage_changes.get(metric).unwrap_or(&0.0);

            report.push_str(&format!(
                "  {}: {:.2} -> {:.2} (Δ {:.2}, {:.1}%)\n",
                metric, value_a, value_b, diff, pct_change
            ));
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_objective() {
        let obj = PolicyObjective::new("revenue", "Tax revenue", 1.0, true);
        assert_eq!(obj.name, "revenue");
        assert_eq!(obj.weight, 1.0);
        assert!(obj.maximize);

        // Test evaluation
        let score = obj.evaluate(100.0);
        assert!(score > 0.0);
    }

    #[test]
    fn test_policy_objective_with_target() {
        let obj =
            PolicyObjective::new("compliance", "Compliance rate", 1.0, true).with_target(95.0);

        assert_eq!(obj.target, Some(95.0));

        // Exact match should score 1.0
        let score = obj.evaluate(95.0);
        assert!((score - 1.0).abs() < 1e-10);

        // Close to target should score high
        let score = obj.evaluate(94.0);
        assert!(score > 0.95);
    }

    #[test]
    fn test_multi_objective_optimizer() {
        let mut optimizer = MultiObjectiveOptimizer::new();

        let obj1 = PolicyObjective::new("revenue", "Tax revenue", 1.0, true);
        let obj2 = PolicyObjective::new("compliance", "Compliance cost", 0.5, false);

        optimizer.add_objective(obj1);
        optimizer.add_objective(obj2);

        // Evaluate a configuration
        let mut params = HashMap::new();
        params.insert("tax_rate".to_string(), 0.25);

        let mut metrics = HashMap::new();
        metrics.insert("revenue".to_string(), 100000.0);
        metrics.insert("compliance".to_string(), 5000.0);

        let result = optimizer.evaluate(params, &metrics).unwrap();
        assert!(result.total_score > 0.0);
        assert_eq!(result.objective_scores.len(), 2);
    }

    #[test]
    fn test_pareto_frontier() {
        let mut optimizer = MultiObjectiveOptimizer::new();

        optimizer.add_objective(PolicyObjective::new("revenue", "Revenue", 1.0, true));
        optimizer.add_objective(PolicyObjective::new("fairness", "Fairness", 1.0, true));

        // Add some results
        let result1 = MultiObjectiveResult {
            parameters: HashMap::new(),
            objective_scores: {
                let mut map = HashMap::new();
                map.insert("revenue".to_string(), 100.0);
                map.insert("fairness".to_string(), 50.0);
                map
            },
            total_score: 150.0,
            pareto_optimal: false,
        };

        let result2 = MultiObjectiveResult {
            parameters: HashMap::new(),
            objective_scores: {
                let mut map = HashMap::new();
                map.insert("revenue".to_string(), 80.0);
                map.insert("fairness".to_string(), 80.0);
                map
            },
            total_score: 160.0,
            pareto_optimal: false,
        };

        let result3 = MultiObjectiveResult {
            parameters: HashMap::new(),
            objective_scores: {
                let mut map = HashMap::new();
                map.insert("revenue".to_string(), 60.0);
                map.insert("fairness".to_string(), 60.0);
                map
            },
            total_score: 120.0,
            pareto_optimal: false,
        };

        optimizer.add_result(result1);
        optimizer.add_result(result2);
        optimizer.add_result(result3);

        let frontier = optimizer.find_pareto_frontier();

        // result3 is dominated, so it should not be in the frontier
        assert_eq!(frontier.len(), 2);
    }

    #[test]
    fn test_policy_sensitivity() {
        let mut sensitivity = PolicySensitivity::new("tax_rate", 0.25);

        let mut metrics1 = HashMap::new();
        metrics1.insert("revenue".to_string(), 100000.0);
        sensitivity.add_point(0.25, metrics1);

        let mut metrics2 = HashMap::new();
        metrics2.insert("revenue".to_string(), 120000.0);
        sensitivity.add_point(0.30, metrics2);

        let coef = sensitivity.sensitivity_coefficient("revenue").unwrap();

        // 20% increase in revenue from 20% increase in rate = coefficient of 1.0
        assert!((coef - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_stakeholder_matrix() {
        let mut matrix = StakeholderMatrix::new();

        matrix.add_stakeholder(Stakeholder::new("workers", "Working class", 1.0));
        matrix.add_stakeholder(Stakeholder::new("businesses", "Business owners", 0.8));

        let mut impacts1 = HashMap::new();
        impacts1.insert("income_change".to_string(), 1000.0);
        matrix.set_impact("workers", impacts1);

        let mut impacts2 = HashMap::new();
        impacts2.insert("profit_change".to_string(), -500.0);
        matrix.set_impact("businesses", impacts2);

        let (winners, losers) = matrix.winners_and_losers();

        assert_eq!(winners, vec!["workers"]);
        assert_eq!(losers, vec!["businesses"]);
    }

    #[test]
    fn test_stakeholder_report() {
        let mut matrix = StakeholderMatrix::new();

        matrix.add_stakeholder(Stakeholder::new("citizens", "General public", 1.0));

        let mut impacts = HashMap::new();
        impacts.insert("benefit".to_string(), 500.0);
        matrix.set_impact("citizens", impacts);

        let report = matrix.report();
        assert!(report.contains("Stakeholder Impact Matrix"));
        assert!(report.contains("citizens"));
    }

    #[test]
    fn test_distributional_impact() {
        let impacts = vec![
            PolicyDistributionalImpact {
                decile: 0,
                average_impact: 100.0,
                count: 100,
                total_impact: 10000.0,
            },
            PolicyDistributionalImpact {
                decile: 1,
                average_impact: 80.0,
                count: 100,
                total_impact: 8000.0,
            },
            PolicyDistributionalImpact {
                decile: 2,
                average_impact: 60.0,
                count: 100,
                total_impact: 6000.0,
            },
        ];

        let analysis = PolicyDistributionalAnalysis::new(impacts, 0.35, 0.32);

        assert!(analysis.is_progressive); // Lower deciles benefit more
        assert!(analysis.gini_after < analysis.gini_before); // Inequality reduced
    }

    #[test]
    fn test_distributional_chart_data() {
        let impacts = vec![
            PolicyDistributionalImpact {
                decile: 0,
                average_impact: 100.0,
                count: 100,
                total_impact: 10000.0,
            },
            PolicyDistributionalImpact {
                decile: 1,
                average_impact: 80.0,
                count: 100,
                total_impact: 8000.0,
            },
        ];

        let analysis = PolicyDistributionalAnalysis::new(impacts, 0.35, 0.35);
        let chart_data = analysis.to_chart_data();

        assert_eq!(chart_data.len(), 2);
        assert_eq!(chart_data[0].0, "D0");
        assert_eq!(chart_data[0].1, 100.0);
    }

    #[test]
    fn test_policy_comparison() {
        let mut metrics_a = HashMap::new();
        metrics_a.insert("revenue".to_string(), 100000.0);
        metrics_a.insert("compliance_cost".to_string(), 5000.0);

        let mut metrics_b = HashMap::new();
        metrics_b.insert("revenue".to_string(), 110000.0);
        metrics_b.insert("compliance_cost".to_string(), 6000.0);

        let comparison = PolicyComparison::new("Current", "Proposed", metrics_a, metrics_b);

        assert_eq!(comparison.differences.get("revenue").unwrap(), &10000.0);
        assert_eq!(
            comparison.differences.get("compliance_cost").unwrap(),
            &1000.0
        );

        let better = comparison.better_for("revenue").unwrap();
        assert_eq!(better, "Proposed");
    }

    #[test]
    fn test_policy_comparison_report() {
        let mut metrics_a = HashMap::new();
        metrics_a.insert("revenue".to_string(), 100000.0);

        let mut metrics_b = HashMap::new();
        metrics_b.insert("revenue".to_string(), 120000.0);

        let comparison = PolicyComparison::new("PolicyA", "PolicyB", metrics_a, metrics_b);
        let report = comparison.report();

        assert!(report.contains("Policy Comparison"));
        assert!(report.contains("PolicyA"));
        assert!(report.contains("PolicyB"));
        assert!(report.contains("revenue"));
    }

    #[test]
    fn test_concentration_index() {
        let impacts = vec![
            PolicyDistributionalImpact {
                decile: 0,
                average_impact: 10.0,
                count: 100,
                total_impact: 1000.0,
            },
            PolicyDistributionalImpact {
                decile: 1,
                average_impact: 20.0,
                count: 100,
                total_impact: 2000.0,
            },
            PolicyDistributionalImpact {
                decile: 2,
                average_impact: 30.0,
                count: 100,
                total_impact: 3000.0,
            },
        ];

        let analysis = PolicyDistributionalAnalysis::new(impacts, 0.3, 0.3);
        let ci = analysis.concentration_index();

        // Positive CI means concentration in higher deciles (regressive)
        assert!(ci > 0.0);
    }

    #[test]
    fn test_sensitivity_multiple_metrics() {
        let mut sensitivity = PolicySensitivity::new("tax_rate", 0.20);

        let mut metrics1 = HashMap::new();
        metrics1.insert("revenue".to_string(), 80000.0);
        metrics1.insert("compliance".to_string(), 4000.0);
        sensitivity.add_point(0.20, metrics1);

        let mut metrics2 = HashMap::new();
        metrics2.insert("revenue".to_string(), 100000.0);
        metrics2.insert("compliance".to_string(), 5000.0);
        sensitivity.add_point(0.25, metrics2);

        let revenue_coef = sensitivity.sensitivity_coefficient("revenue").unwrap();
        let compliance_coef = sensitivity.sensitivity_coefficient("compliance").unwrap();

        assert!(revenue_coef > 0.0);
        assert!(compliance_coef > 0.0);
    }

    #[test]
    fn test_get_metric_at() {
        let mut sensitivity = PolicySensitivity::new("rate", 0.5);

        let mut metrics = HashMap::new();
        metrics.insert("value".to_string(), 123.0);
        sensitivity.add_point(0.5, metrics);

        let value = sensitivity.get_metric_at(0.5, "value").unwrap();
        assert_eq!(value, 123.0);

        let missing = sensitivity.get_metric_at(0.6, "value");
        assert!(missing.is_none());
    }
}
