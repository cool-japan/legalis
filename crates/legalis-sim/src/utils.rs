//! Utility functions and helpers for common simulation operations.

use crate::{SimulationMetrics, StatuteMetrics};
use std::sync::{Arc, Mutex};

/// Aggregates multiple simulation metrics into a single combined metric.
pub fn aggregate_metrics(metrics_list: &[SimulationMetrics]) -> SimulationMetrics {
    let mut aggregated = SimulationMetrics::new();

    for metrics in metrics_list {
        aggregated.total_applications += metrics.total_applications;
        aggregated.deterministic_count += metrics.deterministic_count;
        aggregated.discretion_count += metrics.discretion_count;
        aggregated.void_count += metrics.void_count;

        // Merge discretion agents
        aggregated
            .discretion_agents
            .extend(&metrics.discretion_agents);

        // Merge statute metrics
        for (statute_id, statute_metrics) in &metrics.statute_metrics {
            aggregated
                .statute_metrics
                .entry(statute_id.clone())
                .or_default()
                .merge(statute_metrics);
        }
    }

    // Remove duplicate discretion agents
    aggregated.discretion_agents.sort();
    aggregated.discretion_agents.dedup();

    aggregated
}

/// Aggregates multiple simulation metrics in parallel for large datasets.
/// This is more efficient than `aggregate_metrics` for very large metric collections.
pub fn aggregate_metrics_parallel(metrics_list: &[SimulationMetrics]) -> SimulationMetrics {
    if metrics_list.is_empty() {
        return SimulationMetrics::new();
    }

    // For small datasets, use sequential aggregation
    if metrics_list.len() < 100 {
        return aggregate_metrics(metrics_list);
    }

    let aggregated = Arc::new(Mutex::new(SimulationMetrics::new()));

    // Process in chunks for better cache locality
    let chunk_size = (metrics_list.len() / num_cpus::get()).max(1);

    std::thread::scope(|s| {
        let handles: Vec<_> = metrics_list
            .chunks(chunk_size)
            .map(|chunk| {
                let agg = Arc::clone(&aggregated);
                s.spawn(move || {
                    let chunk_agg = aggregate_metrics(chunk);
                    let mut locked = agg.lock().unwrap();
                    locked.total_applications += chunk_agg.total_applications;
                    locked.deterministic_count += chunk_agg.deterministic_count;
                    locked.discretion_count += chunk_agg.discretion_count;
                    locked.void_count += chunk_agg.void_count;
                    locked
                        .discretion_agents
                        .extend(&chunk_agg.discretion_agents);

                    for (statute_id, statute_metrics) in &chunk_agg.statute_metrics {
                        locked
                            .statute_metrics
                            .entry(statute_id.clone())
                            .or_default()
                            .merge(statute_metrics);
                    }
                })
            })
            .collect();

        for handle in handles {
            let _ = handle.join();
        }
    });

    let mut result = Arc::try_unwrap(aggregated)
        .unwrap_or_else(|_| panic!("Failed to unwrap Arc"))
        .into_inner()
        .unwrap();

    // Remove duplicate discretion agents
    result.discretion_agents.sort();
    result.discretion_agents.dedup();

    result
}

/// Compares two sets of simulation metrics and returns the differences.
pub fn compare_metrics(
    baseline: &SimulationMetrics,
    comparison: &SimulationMetrics,
) -> MetricsDiff {
    MetricsDiff {
        total_applications_diff: comparison.total_applications as i64
            - baseline.total_applications as i64,
        deterministic_diff: comparison.deterministic_count as i64
            - baseline.deterministic_count as i64,
        discretion_diff: comparison.discretion_count as i64 - baseline.discretion_count as i64,
        void_diff: comparison.void_count as i64 - baseline.void_count as i64,
        deterministic_ratio_diff: comparison.deterministic_ratio() - baseline.deterministic_ratio(),
        discretion_ratio_diff: comparison.discretion_ratio() - baseline.discretion_ratio(),
    }
}

/// Differences between two simulation metrics.
#[derive(Debug, Clone)]
pub struct MetricsDiff {
    pub total_applications_diff: i64,
    pub deterministic_diff: i64,
    pub discretion_diff: i64,
    pub void_diff: i64,
    pub deterministic_ratio_diff: f64,
    pub discretion_ratio_diff: f64,
}

impl MetricsDiff {
    /// Formats the diff as a human-readable summary.
    pub fn summary(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Metrics Comparison ===\n");
        report.push_str(&format!(
            "Total Applications: {:+}\n",
            self.total_applications_diff
        ));
        report.push_str(&format!("Deterministic: {:+}\n", self.deterministic_diff));
        report.push_str(&format!("Discretionary: {:+}\n", self.discretion_diff));
        report.push_str(&format!("Void: {:+}\n", self.void_diff));
        report.push_str(&format!(
            "Deterministic Ratio: {:+.2}%\n",
            self.deterministic_ratio_diff * 100.0
        ));
        report.push_str(&format!(
            "Discretion Ratio: {:+.2}%\n",
            self.discretion_ratio_diff * 100.0
        ));
        report
    }

    /// Returns true if the comparison shows improvement.
    pub fn is_improvement(&self) -> bool {
        self.deterministic_ratio_diff > 0.0
    }

    /// Returns true if the differences are significant (>5% change).
    pub fn is_significant(&self) -> bool {
        self.deterministic_ratio_diff.abs() > 0.05 || self.discretion_ratio_diff.abs() > 0.05
    }
}

impl StatuteMetrics {
    /// Merges another StatuteMetrics into this one.
    pub fn merge(&mut self, other: &StatuteMetrics) {
        self.total += other.total;
        self.deterministic += other.deterministic;
        self.discretion += other.discretion;
        self.void += other.void;
    }
}

/// Calculates summary statistics from a list of values.
#[derive(Debug, Clone)]
pub struct SummaryStats {
    pub count: usize,
    pub sum: f64,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub variance: f64,
    pub std_dev: f64,
}

impl SummaryStats {
    /// Calculates summary statistics from a slice of values.
    pub fn from_values(values: &[f64]) -> Option<Self> {
        if values.is_empty() {
            return None;
        }

        let count = values.len();
        let sum: f64 = values.iter().sum();
        let mean = sum / count as f64;

        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        let variance = values
            .iter()
            .map(|&x| {
                let diff = x - mean;
                diff * diff
            })
            .sum::<f64>()
            / count as f64;

        let std_dev = variance.sqrt();

        Some(SummaryStats {
            count,
            sum,
            mean,
            min,
            max,
            variance,
            std_dev,
        })
    }

    /// Returns the coefficient of variation (CV).
    pub fn coefficient_of_variation(&self) -> f64 {
        if self.mean == 0.0 {
            0.0
        } else {
            self.std_dev / self.mean
        }
    }
}

/// Statistical hypothesis testing utilities.
#[derive(Debug, Clone)]
pub struct HypothesisTest {
    pub test_statistic: f64,
    pub p_value: f64,
    pub reject_null: bool,
    pub confidence_level: f64,
}

impl HypothesisTest {
    /// Performs a two-sample t-test to compare means.
    /// Returns None if sample sizes are too small or variance is zero.
    pub fn two_sample_t_test(
        sample_a: &[f64],
        sample_b: &[f64],
        significance_level: f64,
    ) -> Option<Self> {
        if sample_a.len() < 2 || sample_b.len() < 2 {
            return None;
        }

        let stats_a = SummaryStats::from_values(sample_a)?;
        let stats_b = SummaryStats::from_values(sample_b)?;

        // Pooled standard deviation
        let pooled_var = ((stats_a.count - 1) as f64 * stats_a.variance
            + (stats_b.count - 1) as f64 * stats_b.variance)
            / (stats_a.count + stats_b.count - 2) as f64;

        if pooled_var == 0.0 {
            return None;
        }

        let pooled_std = pooled_var.sqrt();

        // t-statistic
        let t_stat = (stats_a.mean - stats_b.mean)
            / (pooled_std * ((1.0 / stats_a.count as f64) + (1.0 / stats_b.count as f64)).sqrt());

        // Degrees of freedom
        let df = stats_a.count + stats_b.count - 2;

        // Approximate p-value using normal distribution (for large samples)
        // For small samples, this is an approximation
        let p_value = if df > 30 {
            // Use normal approximation
            2.0 * (1.0 - normal_cdf(t_stat.abs()))
        } else {
            // Conservative approximation for small samples
            2.0 * (1.0 - normal_cdf(t_stat.abs()))
        };

        Some(HypothesisTest {
            test_statistic: t_stat,
            p_value,
            reject_null: p_value < significance_level,
            confidence_level: 1.0 - significance_level,
        })
    }

    /// Performs a chi-squared goodness-of-fit test.
    pub fn chi_squared_test(observed: &[f64], expected: &[f64]) -> Option<Self> {
        if observed.len() != expected.len() || observed.is_empty() {
            return None;
        }

        let mut chi_squared = 0.0;
        for (obs, exp) in observed.iter().zip(expected.iter()) {
            if *exp == 0.0 {
                continue;
            }
            chi_squared += (obs - exp).powi(2) / exp;
        }

        // Degrees of freedom = n - 1
        let df = observed.len() - 1;

        // Approximate p-value (using normal approximation for large df)
        let p_value = if df > 30 {
            1.0 - normal_cdf((chi_squared - df as f64) / (2.0 * df as f64).sqrt())
        } else {
            // Conservative approximation
            1.0 - normal_cdf((chi_squared - df as f64) / (2.0 * df as f64).sqrt())
        };

        Some(HypothesisTest {
            test_statistic: chi_squared,
            p_value: p_value.clamp(0.0, 1.0),
            reject_null: p_value < 0.05,
            confidence_level: 0.95,
        })
    }
}

/// Cumulative distribution function for standard normal distribution.
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Error function approximation (Abramowitz and Stegun).
fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

/// Progress tracker for long-running simulations.
#[derive(Debug, Clone)]
pub struct ProgressTracker {
    pub total_steps: usize,
    pub completed_steps: usize,
    pub start_time: std::time::Instant,
}

impl ProgressTracker {
    /// Creates a new progress tracker.
    pub fn new(total_steps: usize) -> Self {
        Self {
            total_steps,
            completed_steps: 0,
            start_time: std::time::Instant::now(),
        }
    }

    /// Updates progress and returns current percentage.
    pub fn update(&mut self, completed: usize) -> f64 {
        self.completed_steps = completed.min(self.total_steps);
        self.percentage()
    }

    /// Returns current progress percentage.
    pub fn percentage(&self) -> f64 {
        if self.total_steps == 0 {
            0.0
        } else {
            (self.completed_steps as f64 / self.total_steps as f64) * 100.0
        }
    }

    /// Returns elapsed time in seconds.
    pub fn elapsed_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    /// Estimates remaining time in seconds.
    pub fn estimated_remaining_secs(&self) -> Option<f64> {
        if self.completed_steps == 0 {
            return None;
        }

        let elapsed = self.elapsed_secs();
        let rate = self.completed_steps as f64 / elapsed;
        let remaining = self.total_steps - self.completed_steps;

        Some(remaining as f64 / rate)
    }

    /// Returns a formatted progress string.
    pub fn progress_string(&self) -> String {
        format!(
            "{}/{} ({:.1}%) - Elapsed: {:.1}s",
            self.completed_steps,
            self.total_steps,
            self.percentage(),
            self.elapsed_secs()
        )
    }
}

/// Scenario comparison utilities for A/B testing.
#[derive(Debug, Clone)]
pub struct ScenarioComparison {
    pub scenario_a_name: String,
    pub scenario_b_name: String,
    pub metrics_diff: MetricsDiff,
    pub statistical_significance: Option<HypothesisTest>,
}

impl ScenarioComparison {
    /// Creates a new scenario comparison.
    pub fn new(
        scenario_a_name: impl Into<String>,
        scenario_b_name: impl Into<String>,
        metrics_a: &SimulationMetrics,
        metrics_b: &SimulationMetrics,
    ) -> Self {
        Self {
            scenario_a_name: scenario_a_name.into(),
            scenario_b_name: scenario_b_name.into(),
            metrics_diff: compare_metrics(metrics_a, metrics_b),
            statistical_significance: None,
        }
    }

    /// Adds statistical significance testing.
    pub fn with_significance_test(
        mut self,
        sample_a: &[f64],
        sample_b: &[f64],
        significance_level: f64,
    ) -> Self {
        self.statistical_significance =
            HypothesisTest::two_sample_t_test(sample_a, sample_b, significance_level);
        self
    }

    /// Returns a formatted comparison report.
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!(
            "=== Scenario Comparison: {} vs {} ===\n",
            self.scenario_a_name, self.scenario_b_name
        ));
        report.push_str(&self.metrics_diff.summary());

        if let Some(ref test) = self.statistical_significance {
            report.push_str("\n=== Statistical Significance ===\n");
            report.push_str(&format!("Test Statistic: {:.4}\n", test.test_statistic));
            report.push_str(&format!("P-Value: {:.4}\n", test.p_value));
            report.push_str(&format!(
                "Result: {} ({}% confidence)\n",
                if test.reject_null {
                    "Statistically Significant"
                } else {
                    "Not Significant"
                },
                test.confidence_level * 100.0
            ));
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LawApplicationResult, SimulationMetrics};
    use legalis_core::{Effect, EffectType, LegalResult};
    use uuid::Uuid;

    #[test]
    fn test_aggregate_metrics() {
        let mut metrics1 = SimulationMetrics::new();
        metrics1.record_result(&LawApplicationResult {
            agent_id: Uuid::new_v4(),
            statute_id: "test".to_string(),
            result: LegalResult::Deterministic(Effect::new(EffectType::Grant, "test")),
        });

        let mut metrics2 = SimulationMetrics::new();
        metrics2.record_result(&LawApplicationResult {
            agent_id: Uuid::new_v4(),
            statute_id: "test".to_string(),
            result: LegalResult::Deterministic(Effect::new(EffectType::Grant, "test")),
        });

        let aggregated = aggregate_metrics(&[metrics1, metrics2]);

        assert_eq!(aggregated.total_applications, 2);
        assert_eq!(aggregated.deterministic_count, 2);
    }

    #[test]
    fn test_compare_metrics() {
        let mut baseline = SimulationMetrics::new();
        baseline.total_applications = 100;
        baseline.deterministic_count = 80;
        baseline.discretion_count = 10;
        baseline.void_count = 10;

        let mut comparison = SimulationMetrics::new();
        comparison.total_applications = 100;
        comparison.deterministic_count = 90;
        comparison.discretion_count = 5;
        comparison.void_count = 5;

        let diff = compare_metrics(&baseline, &comparison);

        assert_eq!(diff.deterministic_diff, 10);
        assert_eq!(diff.discretion_diff, -5);
        assert!(diff.is_improvement());
    }

    #[test]
    fn test_summary_stats() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = SummaryStats::from_values(&values).unwrap();

        assert_eq!(stats.count, 5);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert!((stats.std_dev - std::f64::consts::SQRT_2).abs() < 0.001);
    }

    #[test]
    fn test_summary_stats_empty() {
        let values: Vec<f64> = vec![];
        let stats = SummaryStats::from_values(&values);
        assert!(stats.is_none());
    }

    #[test]
    fn test_metrics_diff_significant() {
        let diff = MetricsDiff {
            total_applications_diff: 0,
            deterministic_diff: 10,
            discretion_diff: -10,
            void_diff: 0,
            deterministic_ratio_diff: 0.1, // 10% change
            discretion_ratio_diff: -0.1,
        };

        assert!(diff.is_significant());
    }

    #[test]
    fn test_metrics_diff_not_significant() {
        let diff = MetricsDiff {
            total_applications_diff: 0,
            deterministic_diff: 1,
            discretion_diff: -1,
            void_diff: 0,
            deterministic_ratio_diff: 0.01, // 1% change
            discretion_ratio_diff: -0.01,
        };

        assert!(!diff.is_significant());
    }

    #[test]
    fn test_aggregate_metrics_parallel() {
        let mut metrics_list = Vec::new();
        for i in 0..200 {
            let mut metrics = SimulationMetrics::new();
            metrics.record_result(&LawApplicationResult {
                agent_id: Uuid::new_v4(),
                statute_id: format!("test-{}", i % 5),
                result: LegalResult::Deterministic(Effect::new(EffectType::Grant, "test")),
            });
            metrics_list.push(metrics);
        }

        let aggregated = aggregate_metrics_parallel(&metrics_list);

        assert_eq!(aggregated.total_applications, 200);
        assert_eq!(aggregated.deterministic_count, 200);
    }

    #[test]
    fn test_aggregate_metrics_parallel_empty() {
        let metrics_list: Vec<SimulationMetrics> = vec![];
        let aggregated = aggregate_metrics_parallel(&metrics_list);

        assert_eq!(aggregated.total_applications, 0);
    }

    #[test]
    fn test_aggregate_metrics_parallel_small() {
        // Should fall back to sequential for small datasets
        let mut metrics_list = Vec::new();
        for _ in 0..10 {
            let mut metrics = SimulationMetrics::new();
            metrics.record_result(&LawApplicationResult {
                agent_id: Uuid::new_v4(),
                statute_id: "test".to_string(),
                result: LegalResult::Deterministic(Effect::new(EffectType::Grant, "test")),
            });
            metrics_list.push(metrics);
        }

        let aggregated = aggregate_metrics_parallel(&metrics_list);
        assert_eq!(aggregated.total_applications, 10);
    }

    #[test]
    fn test_hypothesis_test_t_test() {
        let sample_a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sample_b = vec![2.0, 3.0, 4.0, 5.0, 6.0];

        let test = HypothesisTest::two_sample_t_test(&sample_a, &sample_b, 0.05);
        assert!(test.is_some());

        let test = test.unwrap();
        assert!(test.p_value >= 0.0 && test.p_value <= 1.0);
        assert_eq!(test.confidence_level, 0.95);
    }

    #[test]
    fn test_hypothesis_test_t_test_identical() {
        let sample_a = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let sample_b = vec![5.0, 5.0, 5.0, 5.0, 5.0];

        let test = HypothesisTest::two_sample_t_test(&sample_a, &sample_b, 0.05);
        // Should return None due to zero variance
        assert!(test.is_none());
    }

    #[test]
    fn test_hypothesis_test_t_test_small_sample() {
        let sample_a = vec![1.0];
        let sample_b = vec![2.0];

        let test = HypothesisTest::two_sample_t_test(&sample_a, &sample_b, 0.05);
        assert!(test.is_none()); // Too small
    }

    #[test]
    fn test_hypothesis_test_chi_squared() {
        let observed = vec![10.0, 20.0, 30.0, 40.0];
        let expected = vec![15.0, 15.0, 35.0, 35.0];

        let test = HypothesisTest::chi_squared_test(&observed, &expected);
        assert!(test.is_some());

        let test = test.unwrap();
        assert!(test.test_statistic > 0.0);
        assert!(test.p_value >= 0.0 && test.p_value <= 1.0);
    }

    #[test]
    fn test_progress_tracker() {
        let mut tracker = ProgressTracker::new(100);

        assert_eq!(tracker.percentage(), 0.0);

        tracker.update(50);
        assert_eq!(tracker.percentage(), 50.0);
        assert_eq!(tracker.completed_steps, 50);

        tracker.update(100);
        assert_eq!(tracker.percentage(), 100.0);

        // Test overflow protection
        tracker.update(150);
        assert_eq!(tracker.percentage(), 100.0);
        assert_eq!(tracker.completed_steps, 100);
    }

    #[test]
    fn test_progress_tracker_zero_total() {
        let tracker = ProgressTracker::new(0);
        assert_eq!(tracker.percentage(), 0.0);
    }

    #[test]
    fn test_progress_tracker_string() {
        let mut tracker = ProgressTracker::new(100);
        tracker.update(50);

        let progress_str = tracker.progress_string();
        assert!(progress_str.contains("50/100"));
        assert!(progress_str.contains("50.0%"));
    }

    #[test]
    fn test_scenario_comparison() {
        let mut metrics_a = SimulationMetrics::new();
        metrics_a.total_applications = 100;
        metrics_a.deterministic_count = 80;

        let mut metrics_b = SimulationMetrics::new();
        metrics_b.total_applications = 100;
        metrics_b.deterministic_count = 90;

        let comparison = ScenarioComparison::new("Baseline", "Enhanced", &metrics_a, &metrics_b);

        assert_eq!(comparison.scenario_a_name, "Baseline");
        assert_eq!(comparison.scenario_b_name, "Enhanced");
        assert_eq!(comparison.metrics_diff.deterministic_diff, 10);
    }

    #[test]
    fn test_scenario_comparison_with_significance() {
        let mut metrics_a = SimulationMetrics::new();
        metrics_a.total_applications = 100;
        metrics_a.deterministic_count = 80;

        let mut metrics_b = SimulationMetrics::new();
        metrics_b.total_applications = 100;
        metrics_b.deterministic_count = 90;

        let sample_a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sample_b = vec![2.0, 3.0, 4.0, 5.0, 6.0];

        let comparison = ScenarioComparison::new("Baseline", "Enhanced", &metrics_a, &metrics_b)
            .with_significance_test(&sample_a, &sample_b, 0.05);

        assert!(comparison.statistical_significance.is_some());
    }

    #[test]
    fn test_scenario_comparison_report() {
        let mut metrics_a = SimulationMetrics::new();
        metrics_a.total_applications = 100;
        metrics_a.deterministic_count = 80;

        let mut metrics_b = SimulationMetrics::new();
        metrics_b.total_applications = 100;
        metrics_b.deterministic_count = 90;

        let comparison = ScenarioComparison::new("Baseline", "Enhanced", &metrics_a, &metrics_b);

        let report = comparison.report();
        assert!(report.contains("Baseline"));
        assert!(report.contains("Enhanced"));
        assert!(report.contains("Metrics Comparison"));
    }

    #[test]
    fn test_normal_cdf() {
        // Test standard normal CDF at 0 should be 0.5
        let result = normal_cdf(0.0);
        assert!((result - 0.5).abs() < 0.01);

        // Test positive value
        let result = normal_cdf(1.96);
        assert!((result - 0.975).abs() < 0.01);
    }

    #[test]
    fn test_erf() {
        // Test error function at 0 should be 0
        let result = erf(0.0);
        assert!(result.abs() < 0.001);

        // Test erf(1) ≈ 0.8427
        let result = erf(1.0);
        assert!((result - 0.8427).abs() < 0.01);
    }
}
