//! Portfolio analysis for multi-statute optimization.
//!
//! This module provides tools for analyzing combinations of statutes,
//! finding optimal policy portfolios, and understanding interactions between policies.

use crate::{RiskMetrics, SimResult, SimulationMetrics};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A portfolio of statutes with their associated metrics.
#[derive(Debug, Clone)]
pub struct StatutePortfolio {
    /// Statutes in the portfolio with their weights (0.0 to 1.0)
    pub statutes: Vec<(String, f64)>,
    /// Aggregated metrics for the portfolio
    pub metrics: SimulationMetrics,
    /// Risk metrics for the portfolio
    pub risk_metrics: Option<RiskMetrics>,
}

impl StatutePortfolio {
    /// Creates a new portfolio with equal weights.
    pub fn new_equal_weight(statute_ids: Vec<String>, metrics: SimulationMetrics) -> Self {
        let weight = 1.0 / statute_ids.len() as f64;
        let statutes = statute_ids.into_iter().map(|id| (id, weight)).collect();

        Self {
            statutes,
            metrics,
            risk_metrics: None,
        }
    }

    /// Creates a new portfolio with custom weights.
    ///
    /// # Arguments
    /// * `statutes` - Vector of (statute_id, weight) pairs. Weights should sum to 1.0.
    pub fn new_weighted(
        statutes: Vec<(String, f64)>,
        metrics: SimulationMetrics,
    ) -> SimResult<Self> {
        let total_weight: f64 = statutes.iter().map(|(_, w)| w).sum();
        if (total_weight - 1.0).abs() > 0.01 {
            return Err(crate::SimulationError::InvalidConfiguration(format!(
                "Weights must sum to 1.0, got {}",
                total_weight
            )));
        }

        Ok(Self {
            statutes,
            metrics,
            risk_metrics: None,
        })
    }

    /// Adds risk metrics to the portfolio.
    pub fn with_risk_metrics(mut self, risk_metrics: RiskMetrics) -> Self {
        self.risk_metrics = Some(risk_metrics);
        self
    }

    /// Returns the weight of a specific statute in the portfolio.
    pub fn get_weight(&self, statute_id: &str) -> Option<f64> {
        self.statutes
            .iter()
            .find(|(id, _)| id == statute_id)
            .map(|(_, w)| *w)
    }

    /// Calculates the portfolio's expected return (mean deterministic ratio).
    pub fn expected_return(&self) -> f64 {
        self.metrics.deterministic_ratio()
    }

    /// Calculates the portfolio's risk (volatility of deterministic ratio).
    pub fn risk(&self) -> f64 {
        self.risk_metrics
            .as_ref()
            .map(|r| r.std_dev_deterministic)
            .unwrap_or(0.0)
    }

    /// Calculates the Sharpe ratio analog (return/risk).
    pub fn sharpe_ratio(&self) -> f64 {
        let risk = self.risk();
        if risk > 0.0 {
            self.expected_return() / risk
        } else {
            0.0
        }
    }
}

/// Efficient frontier point representing a risk-return trade-off.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontierPoint {
    /// Expected return (deterministic ratio)
    pub expected_return: f64,
    /// Risk (standard deviation)
    pub risk: f64,
    /// Portfolio weights that achieve this point
    pub weights: HashMap<String, f64>,
}

/// Efficient frontier analysis for statute portfolios.
#[derive(Debug, Clone)]
pub struct EfficientFrontier {
    /// Points on the efficient frontier
    pub frontier_points: Vec<FrontierPoint>,
    /// Statute IDs in the analysis
    pub statute_ids: Vec<String>,
}

impl EfficientFrontier {
    /// Creates a new efficient frontier analysis.
    pub fn new(statute_ids: Vec<String>) -> Self {
        Self {
            frontier_points: Vec::new(),
            statute_ids,
        }
    }

    /// Adds a point to the frontier.
    pub fn add_point(&mut self, point: FrontierPoint) {
        self.frontier_points.push(point);
    }

    /// Finds the portfolio with maximum Sharpe ratio.
    pub fn max_sharpe_portfolio(&self) -> Option<&FrontierPoint> {
        self.frontier_points
            .iter()
            .filter(|p| p.risk > 0.0)
            .max_by(|a, b| {
                let sharpe_a = a.expected_return / a.risk;
                let sharpe_b = b.expected_return / b.risk;
                sharpe_a.partial_cmp(&sharpe_b).unwrap()
            })
    }

    /// Finds the portfolio with minimum risk.
    pub fn min_risk_portfolio(&self) -> Option<&FrontierPoint> {
        self.frontier_points
            .iter()
            .min_by(|a, b| a.risk.partial_cmp(&b.risk).unwrap())
    }

    /// Finds the portfolio with maximum return.
    pub fn max_return_portfolio(&self) -> Option<&FrontierPoint> {
        self.frontier_points
            .iter()
            .max_by(|a, b| a.expected_return.partial_cmp(&b.expected_return).unwrap())
    }

    /// Sorts frontier points by risk (ascending).
    pub fn sort_by_risk(&mut self) {
        self.frontier_points
            .sort_by(|a, b| a.risk.partial_cmp(&b.risk).unwrap());
    }
}

/// Correlation matrix for statute interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationMatrix {
    /// Statute IDs
    pub statute_ids: Vec<String>,
    /// Correlation coefficients (symmetric matrix)
    pub correlations: Vec<Vec<f64>>,
}

impl CorrelationMatrix {
    /// Creates a new correlation matrix from statute metrics.
    ///
    /// # Arguments
    /// * `statute_metrics` - Map of statute IDs to multiple simulation results
    pub fn calculate(statute_metrics: &HashMap<String, Vec<SimulationMetrics>>) -> SimResult<Self> {
        if statute_metrics.is_empty() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Cannot calculate correlation matrix with empty data".to_string(),
            ));
        }

        let statute_ids: Vec<String> = statute_metrics.keys().cloned().collect();
        let n = statute_ids.len();
        let mut correlations = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in 0..n {
                let id_i = &statute_ids[i];
                let id_j = &statute_ids[j];

                if i == j {
                    correlations[i][j] = 1.0;
                } else {
                    let metrics_i = &statute_metrics[id_i];
                    let metrics_j = &statute_metrics[id_j];

                    let corr = Self::pearson_correlation(metrics_i, metrics_j);
                    correlations[i][j] = corr;
                }
            }
        }

        Ok(Self {
            statute_ids,
            correlations,
        })
    }

    /// Calculates Pearson correlation coefficient between two statute metrics.
    fn pearson_correlation(
        metrics_a: &[SimulationMetrics],
        metrics_b: &[SimulationMetrics],
    ) -> f64 {
        let n = metrics_a.len().min(metrics_b.len());
        if n == 0 {
            return 0.0;
        }

        let values_a: Vec<f64> = metrics_a.iter().map(|m| m.deterministic_ratio()).collect();
        let values_b: Vec<f64> = metrics_b.iter().map(|m| m.deterministic_ratio()).collect();

        let mean_a = values_a.iter().sum::<f64>() / n as f64;
        let mean_b = values_b.iter().sum::<f64>() / n as f64;

        let mut numerator = 0.0;
        let mut sum_sq_a = 0.0;
        let mut sum_sq_b = 0.0;

        for i in 0..n {
            let diff_a = values_a[i] - mean_a;
            let diff_b = values_b[i] - mean_b;
            numerator += diff_a * diff_b;
            sum_sq_a += diff_a * diff_a;
            sum_sq_b += diff_b * diff_b;
        }

        let denominator = (sum_sq_a * sum_sq_b).sqrt();
        if denominator > 0.0 {
            numerator / denominator
        } else {
            0.0
        }
    }

    /// Gets the correlation between two statutes.
    pub fn get_correlation(&self, statute_a: &str, statute_b: &str) -> Option<f64> {
        let idx_a = self.statute_ids.iter().position(|id| id == statute_a)?;
        let idx_b = self.statute_ids.iter().position(|id| id == statute_b)?;
        Some(self.correlations[idx_a][idx_b])
    }

    /// Finds pairs of statutes with high correlation (> threshold).
    pub fn high_correlation_pairs(&self, threshold: f64) -> Vec<(String, String, f64)> {
        let mut pairs = Vec::new();
        let n = self.statute_ids.len();

        for i in 0..n {
            for j in (i + 1)..n {
                let corr = self.correlations[i][j];
                if corr.abs() > threshold {
                    pairs.push((
                        self.statute_ids[i].clone(),
                        self.statute_ids[j].clone(),
                        corr,
                    ));
                }
            }
        }

        pairs.sort_by(|a, b| b.2.abs().partial_cmp(&a.2.abs()).unwrap());
        pairs
    }
}

/// Portfolio diversification metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversificationMetrics {
    /// Number of statutes in portfolio
    pub num_statutes: usize,
    /// Effective number of statutes (accounts for unequal weights)
    pub effective_num_statutes: f64,
    /// Diversification ratio (portfolio risk / weighted average risk)
    pub diversification_ratio: f64,
    /// Concentration (Herfindahl index of weights)
    pub concentration: f64,
}

impl DiversificationMetrics {
    /// Calculates diversification metrics for a portfolio.
    ///
    /// # Arguments
    /// * `weights` - Statute weights
    /// * `individual_risks` - Risk (std dev) for each statute
    /// * `portfolio_risk` - Overall portfolio risk
    pub fn calculate(
        weights: &[(String, f64)],
        individual_risks: &HashMap<String, f64>,
        portfolio_risk: f64,
    ) -> Self {
        let num_statutes = weights.len();

        // Effective number of statutes (inverse of sum of squared weights)
        let sum_sq_weights: f64 = weights.iter().map(|(_, w)| w * w).sum();
        let effective_num = if sum_sq_weights > 0.0 {
            1.0 / sum_sq_weights
        } else {
            0.0
        };

        // Weighted average risk
        let weighted_avg_risk: f64 = weights
            .iter()
            .map(|(id, w)| w * individual_risks.get(id).unwrap_or(&0.0))
            .sum();

        // Diversification ratio
        let div_ratio = if portfolio_risk > 0.0 {
            weighted_avg_risk / portfolio_risk
        } else {
            1.0
        };

        // Concentration (Herfindahl index)
        let concentration = sum_sq_weights;

        Self {
            num_statutes,
            effective_num_statutes: effective_num,
            diversification_ratio: div_ratio,
            concentration,
        }
    }
}

/// Portfolio optimizer for finding optimal statute combinations.
#[derive(Debug)]
pub struct PortfolioOptimizer {
    /// Statute IDs to optimize over
    pub statute_ids: Vec<String>,
    /// Historical metrics for each statute
    pub statute_metrics: HashMap<String, Vec<SimulationMetrics>>,
}

impl PortfolioOptimizer {
    /// Creates a new portfolio optimizer.
    pub fn new() -> Self {
        Self {
            statute_ids: Vec::new(),
            statute_metrics: HashMap::new(),
        }
    }

    /// Adds a statute to the optimization universe.
    pub fn add_statute(mut self, statute_id: String, metrics: Vec<SimulationMetrics>) -> Self {
        self.statute_ids.push(statute_id.clone());
        self.statute_metrics.insert(statute_id, metrics);
        self
    }

    /// Finds the equal-weight portfolio.
    pub fn equal_weight_portfolio(&self) -> SimResult<StatutePortfolio> {
        if self.statute_ids.is_empty() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "No statutes in optimizer".to_string(),
            ));
        }

        let weight = 1.0 / self.statute_ids.len() as f64;
        let weights: Vec<(String, f64)> = self
            .statute_ids
            .iter()
            .map(|id| (id.clone(), weight))
            .collect();

        let aggregated_metrics = self.aggregate_metrics(&weights)?;
        StatutePortfolio::new_weighted(weights, aggregated_metrics)
    }

    /// Finds the maximum Sharpe ratio portfolio using a simple grid search.
    pub fn max_sharpe_portfolio(&self, num_samples: usize) -> SimResult<StatutePortfolio> {
        if self.statute_ids.is_empty() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "No statutes in optimizer".to_string(),
            ));
        }

        let mut best_sharpe = f64::NEG_INFINITY;
        let mut best_weights = Vec::new();
        let mut best_metrics = SimulationMetrics::new();

        // Simple random search
        for _ in 0..num_samples {
            let weights = self.generate_random_weights();
            if let Ok(metrics) = self.aggregate_metrics(&weights)
                && let Ok(risk_metrics) = RiskMetrics::calculate(std::slice::from_ref(&metrics))
            {
                let sharpe = if risk_metrics.std_dev_deterministic > 0.0 {
                    risk_metrics.mean_deterministic / risk_metrics.std_dev_deterministic
                } else {
                    0.0
                };

                if sharpe > best_sharpe {
                    best_sharpe = sharpe;
                    best_weights = weights;
                    best_metrics = metrics;
                }
            }
        }

        if best_weights.is_empty() {
            return self.equal_weight_portfolio();
        }

        StatutePortfolio::new_weighted(best_weights, best_metrics)
    }

    /// Generates random portfolio weights that sum to 1.0.
    fn generate_random_weights(&self) -> Vec<(String, f64)> {
        let n = self.statute_ids.len();
        let mut raw_weights: Vec<f64> = (0..n).map(|_| rand::random::<f64>()).collect();
        let sum: f64 = raw_weights.iter().sum();

        raw_weights.iter_mut().for_each(|w| *w /= sum);

        self.statute_ids
            .iter()
            .zip(raw_weights.iter())
            .map(|(id, w)| (id.clone(), *w))
            .collect()
    }

    /// Aggregates metrics based on portfolio weights.
    fn aggregate_metrics(&self, weights: &[(String, f64)]) -> SimResult<SimulationMetrics> {
        let mut aggregated = SimulationMetrics::new();

        for (statute_id, weight) in weights {
            if let Some(metrics_list) = self.statute_metrics.get(statute_id)
                && let Some(metrics) = metrics_list.first()
            {
                // Weighted aggregation
                aggregated.total_applications +=
                    (metrics.total_applications as f64 * weight) as usize;
                aggregated.deterministic_count +=
                    (metrics.deterministic_count as f64 * weight) as usize;
                aggregated.discretion_count += (metrics.discretion_count as f64 * weight) as usize;
                aggregated.void_count += (metrics.void_count as f64 * weight) as usize;
            }
        }

        Ok(aggregated)
    }
}

impl Default for PortfolioOptimizer {
    fn default() -> Self {
        Self::new()
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
    fn test_portfolio_creation() {
        let metrics = create_test_metrics(0.7);
        let portfolio = StatutePortfolio::new_equal_weight(
            vec!["statute1".to_string(), "statute2".to_string()],
            metrics,
        );

        assert_eq!(portfolio.statutes.len(), 2);
        assert_eq!(portfolio.get_weight("statute1"), Some(0.5));
        assert_eq!(portfolio.get_weight("statute2"), Some(0.5));
    }

    #[test]
    fn test_weighted_portfolio() {
        let metrics = create_test_metrics(0.7);
        let weights = vec![("statute1".to_string(), 0.6), ("statute2".to_string(), 0.4)];

        let portfolio = StatutePortfolio::new_weighted(weights, metrics).unwrap();
        assert_eq!(portfolio.get_weight("statute1"), Some(0.6));
        assert_eq!(portfolio.get_weight("statute2"), Some(0.4));
    }

    #[test]
    fn test_invalid_weights() {
        let metrics = create_test_metrics(0.7);
        let weights = vec![("statute1".to_string(), 0.6), ("statute2".to_string(), 0.6)];

        assert!(StatutePortfolio::new_weighted(weights, metrics).is_err());
    }

    #[test]
    fn test_efficient_frontier() {
        let mut frontier =
            EfficientFrontier::new(vec!["statute1".to_string(), "statute2".to_string()]);

        let mut weights1 = HashMap::new();
        weights1.insert("statute1".to_string(), 1.0);
        frontier.add_point(FrontierPoint {
            expected_return: 0.7,
            risk: 0.1,
            weights: weights1,
        });

        let mut weights2 = HashMap::new();
        weights2.insert("statute2".to_string(), 1.0);
        frontier.add_point(FrontierPoint {
            expected_return: 0.8,
            risk: 0.15,
            weights: weights2,
        });

        assert_eq!(frontier.frontier_points.len(), 2);
        assert!(frontier.max_sharpe_portfolio().is_some());
        assert!(frontier.min_risk_portfolio().is_some());
        assert!(frontier.max_return_portfolio().is_some());
    }

    #[test]
    fn test_correlation_matrix() {
        let mut statute_metrics = HashMap::new();
        statute_metrics.insert(
            "statute1".to_string(),
            vec![create_test_metrics(0.7), create_test_metrics(0.75)],
        );
        statute_metrics.insert(
            "statute2".to_string(),
            vec![create_test_metrics(0.6), create_test_metrics(0.65)],
        );

        let corr_matrix = CorrelationMatrix::calculate(&statute_metrics).unwrap();

        assert_eq!(corr_matrix.statute_ids.len(), 2);
        assert_eq!(
            corr_matrix.get_correlation("statute1", "statute1"),
            Some(1.0)
        );
    }

    #[test]
    fn test_diversification_metrics() {
        let weights = vec![("statute1".to_string(), 0.6), ("statute2".to_string(), 0.4)];

        let mut risks = HashMap::new();
        risks.insert("statute1".to_string(), 0.1);
        risks.insert("statute2".to_string(), 0.15);

        let metrics = DiversificationMetrics::calculate(&weights, &risks, 0.08);

        assert_eq!(metrics.num_statutes, 2);
        assert!(metrics.effective_num_statutes > 0.0);
        assert!(metrics.diversification_ratio > 0.0);
    }

    #[test]
    fn test_portfolio_optimizer() {
        let optimizer = PortfolioOptimizer::new()
            .add_statute("statute1".to_string(), vec![create_test_metrics(0.7)])
            .add_statute("statute2".to_string(), vec![create_test_metrics(0.8)]);

        let equal_weight = optimizer.equal_weight_portfolio().unwrap();
        assert_eq!(equal_weight.statutes.len(), 2);
    }

    #[test]
    fn test_high_correlation_pairs() {
        let mut statute_metrics = HashMap::new();
        statute_metrics.insert(
            "statute1".to_string(),
            vec![create_test_metrics(0.7), create_test_metrics(0.75)],
        );
        statute_metrics.insert(
            "statute2".to_string(),
            vec![create_test_metrics(0.7), create_test_metrics(0.75)],
        );

        let corr_matrix = CorrelationMatrix::calculate(&statute_metrics).unwrap();
        let pairs = corr_matrix.high_correlation_pairs(0.5);

        assert!(!pairs.is_empty());
    }
}
