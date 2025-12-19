//! Utility functions and helpers for common simulation operations.

use crate::{SimulationMetrics, StatuteMetrics};

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
}
