//! Monte Carlo simulation for probabilistic legal analysis.
//!
//! This module provides tools for running multiple simulations with different
//! random seeds and analyzing the distribution of outcomes to assess uncertainty
//! and compute confidence intervals.

use crate::{SimEngine, SimResult, SimulationMetrics};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for Monte Carlo simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloConfig {
    /// Number of simulation runs to perform.
    pub num_runs: usize,
    /// Confidence level for interval calculation (e.g., 0.95 for 95%).
    pub confidence_level: f64,
    /// Whether to run simulations in parallel.
    pub parallel: bool,
    /// Convergence threshold for early stopping (optional).
    pub convergence_threshold: Option<f64>,
    /// Minimum runs before checking convergence.
    pub min_runs_before_convergence: usize,
}

impl Default for MonteCarloConfig {
    fn default() -> Self {
        Self {
            num_runs: 1000,
            confidence_level: 0.95,
            parallel: true,
            convergence_threshold: None,
            min_runs_before_convergence: 100,
        }
    }
}

/// Results from a Monte Carlo simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloResult {
    /// Number of simulation runs completed.
    pub runs_completed: usize,
    /// Mean metrics across all runs.
    pub mean_metrics: HashMap<String, f64>,
    /// Standard deviation of metrics.
    pub std_dev: HashMap<String, f64>,
    /// Confidence intervals for each metric.
    pub confidence_intervals: HashMap<String, (f64, f64)>,
    /// Whether convergence was achieved (if threshold set).
    pub converged: bool,
    /// Individual run results (optional, for detailed analysis).
    pub individual_runs: Option<Vec<HashMap<String, f64>>>,
}

/// Monte Carlo runner for probabilistic simulation.
pub struct MonteCarloRunner {
    config: MonteCarloConfig,
}

impl MonteCarloRunner {
    /// Create a new Monte Carlo runner with the given configuration.
    pub fn new(config: MonteCarloConfig) -> Self {
        Self { config }
    }

    /// Run Monte Carlo simulation with the given engine.
    pub async fn run(&self, engine: &SimEngine) -> SimResult<MonteCarloResult> {
        let mut results = Vec::new();
        let mut converged = false;

        for run_idx in 0..self.config.num_runs {
            // Run simulation (engine should handle its own randomization)
            let metrics = engine.run_simulation().await;
            results.push(extract_metrics(&metrics));

            // Check convergence if configured
            if let Some(threshold) = self.config.convergence_threshold {
                if run_idx >= self.config.min_runs_before_convergence
                    && self.check_convergence(&results, threshold)
                {
                    converged = true;
                    break;
                }
            }
        }

        let runs_completed = results.len();

        // Calculate statistics
        let mean_metrics = calculate_mean(&results);
        let std_dev = calculate_std_dev(&results, &mean_metrics);
        let confidence_intervals = calculate_confidence_intervals(
            &mean_metrics,
            &std_dev,
            runs_completed,
            self.config.confidence_level,
        );

        Ok(MonteCarloResult {
            runs_completed,
            mean_metrics,
            std_dev,
            confidence_intervals,
            converged,
            individual_runs: Some(results),
        })
    }

    /// Run Monte Carlo simulation in parallel.
    pub async fn run_parallel(
        &self,
        engine_factory: impl Fn() -> SimEngine,
    ) -> SimResult<MonteCarloResult> {
        use tokio::task;

        let mut tasks = Vec::new();

        for _run_idx in 0..self.config.num_runs {
            let engine = engine_factory();
            tasks.push(task::spawn(async move { engine.run_simulation().await }));
        }

        let mut results = Vec::new();
        for task in tasks {
            let metrics = task.await.unwrap();
            results.push(extract_metrics(&metrics));
        }

        let runs_completed = results.len();
        let mean_metrics = calculate_mean(&results);
        let std_dev = calculate_std_dev(&results, &mean_metrics);
        let confidence_intervals = calculate_confidence_intervals(
            &mean_metrics,
            &std_dev,
            runs_completed,
            self.config.confidence_level,
        );

        Ok(MonteCarloResult {
            runs_completed,
            mean_metrics,
            std_dev,
            confidence_intervals,
            converged: false,
            individual_runs: Some(results),
        })
    }

    fn check_convergence(&self, results: &[HashMap<String, f64>], threshold: f64) -> bool {
        if results.len() < 2 {
            return false;
        }

        // Check if coefficient of variation is below threshold for all metrics
        let mean = calculate_mean(results);
        let std_dev = calculate_std_dev(results, &mean);

        for (key, mean_val) in &mean {
            if let Some(std_val) = std_dev.get(key) {
                if mean_val.abs() > 1e-10 {
                    let cv = std_val / mean_val.abs();
                    if cv > threshold {
                        return false;
                    }
                }
            }
        }

        true
    }
}

/// Extract numeric metrics from SimulationMetrics.
fn extract_metrics(metrics: &SimulationMetrics) -> HashMap<String, f64> {
    let mut result = HashMap::new();

    result.insert(
        "total_applications".to_string(),
        metrics.total_applications as f64,
    );
    result.insert(
        "deterministic_count".to_string(),
        metrics.deterministic_count as f64,
    );
    result.insert(
        "discretion_count".to_string(),
        metrics.discretion_count as f64,
    );
    result.insert("void_count".to_string(), metrics.void_count as f64);

    // Add statute-specific metrics
    for (statute_name, statute_metrics) in &metrics.statute_metrics {
        result.insert(
            format!("{}_total", statute_name),
            statute_metrics.total as f64,
        );
        result.insert(
            format!("{}_deterministic", statute_name),
            statute_metrics.deterministic as f64,
        );
        result.insert(
            format!("{}_discretion", statute_name),
            statute_metrics.discretion as f64,
        );
        result.insert(
            format!("{}_void", statute_name),
            statute_metrics.void as f64,
        );
    }

    result
}

/// Calculate mean of all metrics across runs.
fn calculate_mean(results: &[HashMap<String, f64>]) -> HashMap<String, f64> {
    if results.is_empty() {
        return HashMap::new();
    }

    let mut sums: HashMap<String, f64> = HashMap::new();
    let n = results.len() as f64;

    for result in results {
        for (key, value) in result {
            *sums.entry(key.clone()).or_insert(0.0) += value;
        }
    }

    sums.iter().map(|(k, v)| (k.clone(), v / n)).collect()
}

/// Calculate standard deviation of metrics.
fn calculate_std_dev(
    results: &[HashMap<String, f64>],
    means: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    if results.len() < 2 {
        return HashMap::new();
    }

    let mut variances: HashMap<String, f64> = HashMap::new();
    let n = results.len() as f64;

    for result in results {
        for (key, value) in result {
            if let Some(mean) = means.get(key) {
                let diff = value - mean;
                *variances.entry(key.clone()).or_insert(0.0) += diff * diff;
            }
        }
    }

    variances
        .iter()
        .map(|(k, v)| (k.clone(), (v / (n - 1.0)).sqrt()))
        .collect()
}

/// Calculate confidence intervals using t-distribution approximation.
fn calculate_confidence_intervals(
    means: &HashMap<String, f64>,
    std_devs: &HashMap<String, f64>,
    n: usize,
    confidence_level: f64,
) -> HashMap<String, (f64, f64)> {
    let mut intervals = HashMap::new();

    // Use t-distribution critical value (approximation for large n)
    let t_value = if n > 30 {
        // Use normal approximation for large n
        match confidence_level {
            cl if (cl - 0.90).abs() < 0.01 => 1.645,
            cl if (cl - 0.95).abs() < 0.01 => 1.96,
            cl if (cl - 0.99).abs() < 0.01 => 2.576,
            _ => 1.96, // default to 95%
        }
    } else {
        // Simple t-table approximation for small n
        2.0 + (30 - n as i32).max(0) as f64 * 0.05
    };

    let sqrt_n = (n as f64).sqrt();

    for (key, mean) in means {
        if let Some(std_dev) = std_devs.get(key) {
            let margin = t_value * std_dev / sqrt_n;
            intervals.insert(key.clone(), (mean - margin, mean + margin));
        }
    }

    intervals
}

/// Variance reduction using antithetic variates.
pub struct AntitheticVariates {
    base_seed: u64,
}

impl AntitheticVariates {
    /// Create new antithetic variates generator.
    pub fn new(base_seed: u64) -> Self {
        Self { base_seed }
    }

    /// Generate paired seeds for variance reduction.
    pub fn generate_pairs(&self, n: usize) -> Vec<(u64, u64)> {
        (0..n)
            .map(|i| {
                let seed1 = self.base_seed.wrapping_add(i as u64 * 2);
                let seed2 = seed1.wrapping_add(1);
                (seed1, seed2)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimEngineBuilder;
    use legalis_core::BasicEntity;

    #[tokio::test]
    async fn test_monte_carlo_basic() {
        let config = MonteCarloConfig {
            num_runs: 10,
            confidence_level: 0.95,
            parallel: false,
            convergence_threshold: None,
            min_runs_before_convergence: 5,
        };

        let runner = MonteCarloRunner::new(config);

        // Create a simple simulation
        let mut builder = SimEngineBuilder::new().validate(false);
        for _ in 0..100 {
            builder = builder.add_entity(Box::new(BasicEntity::new()));
        }
        let engine = builder.build().unwrap();

        let result = runner.run(&engine).await.unwrap();

        assert_eq!(result.runs_completed, 10);
        assert!(result.mean_metrics.contains_key("total_applications"));
        assert!(result.std_dev.contains_key("total_applications"));
        assert!(
            result
                .confidence_intervals
                .contains_key("total_applications")
        );
    }

    #[test]
    fn test_calculate_mean() {
        let mut results = Vec::new();

        let mut r1 = HashMap::new();
        r1.insert("metric1".to_string(), 10.0);
        r1.insert("metric2".to_string(), 20.0);
        results.push(r1);

        let mut r2 = HashMap::new();
        r2.insert("metric1".to_string(), 20.0);
        r2.insert("metric2".to_string(), 30.0);
        results.push(r2);

        let mean = calculate_mean(&results);

        assert_eq!(mean.get("metric1"), Some(&15.0));
        assert_eq!(mean.get("metric2"), Some(&25.0));
    }

    #[test]
    fn test_calculate_std_dev() {
        let mut results = Vec::new();

        let mut r1 = HashMap::new();
        r1.insert("metric1".to_string(), 10.0);
        results.push(r1);

        let mut r2 = HashMap::new();
        r2.insert("metric1".to_string(), 20.0);
        results.push(r2);

        let mean = calculate_mean(&results);
        let std_dev = calculate_std_dev(&results, &mean);

        // Std dev of [10, 20] with sample std dev = sqrt((10-15)^2 + (20-15)^2 / 1) = sqrt(50) ≈ 7.07
        let expected = (50.0_f64).sqrt();
        assert!((std_dev.get("metric1").unwrap() - expected).abs() < 0.01);
    }

    #[test]
    fn test_antithetic_variates() {
        let av = AntitheticVariates::new(12345);
        let pairs = av.generate_pairs(5);

        assert_eq!(pairs.len(), 5);
        for (seed1, seed2) in pairs {
            assert_eq!(seed2, seed1.wrapping_add(1));
        }
    }

    #[tokio::test]
    async fn test_convergence_detection() {
        let config = MonteCarloConfig {
            num_runs: 1000,
            confidence_level: 0.95,
            parallel: false,
            convergence_threshold: Some(0.01), // 1% coefficient of variation
            min_runs_before_convergence: 10,
        };

        let runner = MonteCarloRunner::new(config);

        // Create deterministic simulation for testing convergence
        let mut builder = SimEngineBuilder::new().validate(false);
        for _ in 0..50 {
            builder = builder.add_entity(Box::new(BasicEntity::new()));
        }
        let engine = builder.build().unwrap();

        let result = runner.run(&engine).await.unwrap();

        // With deterministic simulation, should converge quickly
        assert!(result.runs_completed < 1000 || result.converged);
    }
}
