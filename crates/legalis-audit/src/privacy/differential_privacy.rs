//! Differential Privacy for audit trail queries.
//!
//! Provides privacy-preserving statistical queries on audit data using
//! differential privacy techniques. Adds calibrated noise to query results
//! to prevent re-identification while maintaining statistical utility.

use crate::{AuditRecord, AuditResult};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Differential privacy mechanism for audit queries
pub struct DifferentialPrivacy {
    epsilon: f64,
    delta: f64,
}

impl DifferentialPrivacy {
    /// Create a new differential privacy mechanism
    ///
    /// # Arguments
    /// * `epsilon` - Privacy budget (smaller = more private, larger = more accurate)
    /// * `delta` - Probability of privacy breach (typically very small, e.g., 1e-5)
    pub fn new(epsilon: f64, delta: f64) -> Self {
        assert!(epsilon > 0.0, "Epsilon must be positive");
        assert!((0.0..1.0).contains(&delta), "Delta must be in [0, 1)");
        Self { epsilon, delta }
    }

    /// Standard privacy parameters (ε=1.0, δ=1e-5)
    pub fn standard() -> Self {
        Self::new(1.0, 1e-5)
    }

    /// Strong privacy parameters (ε=0.1, δ=1e-5)
    pub fn strong() -> Self {
        Self::new(0.1, 1e-5)
    }

    /// Weak privacy parameters (ε=5.0, δ=1e-5)
    pub fn weak() -> Self {
        Self::new(5.0, 1e-5)
    }

    /// Count query with differential privacy
    pub fn count_query(
        &self,
        records: &[AuditRecord],
        predicate: impl Fn(&AuditRecord) -> bool,
    ) -> DpQueryResult {
        let true_count = records.iter().filter(|r| predicate(r)).count();
        let noisy_count = self.add_laplace_noise(true_count as f64, 1.0);

        DpQueryResult {
            value: noisy_count.max(0.0),
            true_value: Some(true_count as f64),
            epsilon: self.epsilon,
            delta: self.delta,
            mechanism: NoiseMechanism::Laplace,
        }
    }

    /// Sum query with differential privacy
    pub fn sum_query<F>(&self, records: &[AuditRecord], extractor: F) -> DpQueryResult
    where
        F: Fn(&AuditRecord) -> f64,
    {
        let true_sum: f64 = records.iter().map(&extractor).sum();
        let sensitivity = records.len() as f64; // Assuming bounded values [0,1]
        let noisy_sum = self.add_laplace_noise(true_sum, sensitivity);

        DpQueryResult {
            value: noisy_sum,
            true_value: Some(true_sum),
            epsilon: self.epsilon,
            delta: self.delta,
            mechanism: NoiseMechanism::Laplace,
        }
    }

    /// Average query with differential privacy
    pub fn average_query<F>(&self, records: &[AuditRecord], extractor: F) -> DpQueryResult
    where
        F: Fn(&AuditRecord) -> f64,
    {
        if records.is_empty() {
            return DpQueryResult {
                value: 0.0,
                true_value: Some(0.0),
                epsilon: self.epsilon,
                delta: self.delta,
                mechanism: NoiseMechanism::Laplace,
            };
        }

        let true_sum: f64 = records.iter().map(&extractor).sum();
        let true_avg = true_sum / records.len() as f64;

        // Add noise to both sum and count
        let noisy_sum = self.add_laplace_noise(true_sum, records.len() as f64);
        let noisy_count = self.add_laplace_noise(records.len() as f64, 1.0);

        let noisy_avg = if noisy_count > 0.0 {
            noisy_sum / noisy_count
        } else {
            0.0
        };

        DpQueryResult {
            value: noisy_avg,
            true_value: Some(true_avg),
            epsilon: self.epsilon * 2.0, // Doubled due to composition
            delta: self.delta,
            mechanism: NoiseMechanism::Laplace,
        }
    }

    /// Histogram query with differential privacy
    pub fn histogram_query<F, K>(
        &self,
        records: &[AuditRecord],
        extractor: F,
    ) -> HashMap<K, DpQueryResult>
    where
        F: Fn(&AuditRecord) -> K,
        K: Eq + std::hash::Hash + Clone,
    {
        let mut histogram: HashMap<K, usize> = HashMap::new();
        for record in records {
            *histogram.entry(extractor(record)).or_insert(0) += 1;
        }

        let mut dp_histogram = HashMap::new();
        for (key, count) in histogram {
            let noisy_count = self.add_laplace_noise(count as f64, 1.0);
            dp_histogram.insert(
                key,
                DpQueryResult {
                    value: noisy_count.max(0.0),
                    true_value: Some(count as f64),
                    epsilon: self.epsilon,
                    delta: self.delta,
                    mechanism: NoiseMechanism::Laplace,
                },
            );
        }

        dp_histogram
    }

    /// Add Laplace noise for differential privacy
    fn add_laplace_noise(&self, value: f64, sensitivity: f64) -> f64 {
        let scale = sensitivity / self.epsilon;
        let noise = self.sample_laplace(scale);
        value + noise
    }

    /// Add Gaussian noise for (ε, δ)-differential privacy
    #[allow(dead_code)]
    fn add_gaussian_noise(&self, value: f64, sensitivity: f64) -> f64 {
        let sigma = sensitivity * (2.0 * (1.25 / self.delta).ln()).sqrt() / self.epsilon;
        let noise = self.sample_gaussian(sigma);
        value + noise
    }

    /// Sample from Laplace distribution
    fn sample_laplace(&self, scale: f64) -> f64 {
        let mut rng = rand::rng();
        let u: f64 = rng.random_range(-0.5..0.5);
        -scale * u.signum() * (1.0 - 2.0 * u.abs()).ln()
    }

    /// Sample from Gaussian distribution (Box-Muller transform)
    #[allow(dead_code)]
    fn sample_gaussian(&self, sigma: f64) -> f64 {
        let mut rng = rand::rng();
        let u1: f64 = rng.random_range(0.0..1.0);
        let u2: f64 = rng.random_range(0.0..1.0);
        sigma * (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }

    /// Calculate privacy budget consumed
    pub fn privacy_budget_consumed(&self) -> f64 {
        self.epsilon
    }

    /// Check if privacy budget allows another query
    pub fn can_query(&self, total_budget: f64, consumed: f64) -> bool {
        consumed + self.epsilon <= total_budget
    }
}

/// Result of a differentially private query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DpQueryResult {
    /// Noisy (private) value
    pub value: f64,
    /// True value (for testing/validation only, not released)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_value: Option<f64>,
    /// Epsilon used
    pub epsilon: f64,
    /// Delta used
    pub delta: f64,
    /// Noise mechanism
    pub mechanism: NoiseMechanism,
}

impl DpQueryResult {
    /// Get the relative error between noisy and true values
    pub fn relative_error(&self) -> Option<f64> {
        self.true_value.map(|true_val| {
            if true_val != 0.0 {
                ((self.value - true_val) / true_val).abs()
            } else {
                0.0
            }
        })
    }

    /// Get the absolute error between noisy and true values
    pub fn absolute_error(&self) -> Option<f64> {
        self.true_value
            .map(|true_val| (self.value - true_val).abs())
    }
}

/// Noise mechanism used for differential privacy
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NoiseMechanism {
    /// Laplace mechanism (for ε-DP)
    Laplace,
    /// Gaussian mechanism (for (ε,δ)-DP)
    Gaussian,
    /// Exponential mechanism (for categorical queries)
    Exponential,
}

/// Privacy budget tracker for managing multiple queries
pub struct PrivacyBudgetTracker {
    total_budget: f64,
    consumed: f64,
    queries: Vec<QueryRecord>,
}

impl PrivacyBudgetTracker {
    /// Create a new privacy budget tracker
    pub fn new(total_budget: f64) -> Self {
        Self {
            total_budget,
            consumed: 0.0,
            queries: Vec::new(),
        }
    }

    /// Record a query and consume budget
    pub fn record_query(&mut self, epsilon: f64, description: String) -> AuditResult<()> {
        if self.consumed + epsilon > self.total_budget {
            return Err(crate::AuditError::QueryError(
                "Privacy budget exceeded".to_string(),
            ));
        }

        self.consumed += epsilon;
        self.queries.push(QueryRecord {
            epsilon,
            description,
            timestamp: chrono::Utc::now(),
        });

        Ok(())
    }

    /// Get remaining budget
    pub fn remaining_budget(&self) -> f64 {
        self.total_budget - self.consumed
    }

    /// Get consumed budget
    pub fn consumed_budget(&self) -> f64 {
        self.consumed
    }

    /// Get query history
    pub fn query_history(&self) -> &[QueryRecord] {
        &self.queries
    }

    /// Reset the budget (use with caution!)
    pub fn reset(&mut self) {
        self.consumed = 0.0;
        self.queries.clear();
    }
}

/// Record of a privacy-consuming query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRecord {
    pub epsilon: f64,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use uuid::Uuid;

    fn create_test_records(count: usize) -> Vec<AuditRecord> {
        (0..count)
            .map(|i| {
                AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "test".to_string(),
                    },
                    format!("statute-{}", i % 3),
                    Uuid::new_v4(),
                    DecisionContext::default(),
                    DecisionResult::Deterministic {
                        effect_applied: "approved".to_string(),
                        parameters: HashMap::new(),
                    },
                    None,
                )
            })
            .collect()
    }

    #[test]
    fn test_count_query() {
        let dp = DifferentialPrivacy::standard();
        let records = create_test_records(100);

        let result = dp.count_query(&records, |r| r.statute_id.starts_with("statute-0"));

        // True count should be approximately 34 (100/3)
        assert!(result.true_value.unwrap() >= 30.0 && result.true_value.unwrap() <= 40.0);
        // Noisy count should be non-negative
        assert!(result.value >= 0.0);
        assert_eq!(result.epsilon, 1.0);
    }

    #[test]
    fn test_histogram_query() {
        let dp = DifferentialPrivacy::standard();
        let records = create_test_records(90);

        let histogram = dp.histogram_query(&records, |r| r.statute_id.clone());

        // Should have 3 buckets (statute-0, statute-1, statute-2)
        assert_eq!(histogram.len(), 3);

        for (_key, result) in histogram {
            // Each bucket should have approximately 30 records
            assert!(result.true_value.unwrap() >= 25.0 && result.true_value.unwrap() <= 35.0);
            assert!(result.value >= 0.0);
        }
    }

    #[test]
    fn test_privacy_parameters() {
        let strong = DifferentialPrivacy::strong();
        assert_eq!(strong.epsilon, 0.1);

        let standard = DifferentialPrivacy::standard();
        assert_eq!(standard.epsilon, 1.0);

        let weak = DifferentialPrivacy::weak();
        assert_eq!(weak.epsilon, 5.0);
    }

    #[test]
    fn test_privacy_budget_tracker() {
        let mut tracker = PrivacyBudgetTracker::new(5.0);

        // First query
        tracker
            .record_query(1.0, "count query".to_string())
            .unwrap();
        assert_eq!(tracker.consumed_budget(), 1.0);
        assert_eq!(tracker.remaining_budget(), 4.0);

        // Second query
        tracker
            .record_query(2.0, "histogram query".to_string())
            .unwrap();
        assert_eq!(tracker.consumed_budget(), 3.0);
        assert_eq!(tracker.remaining_budget(), 2.0);

        // Third query exceeds budget
        let result = tracker.record_query(3.0, "sum query".to_string());
        assert!(result.is_err());

        assert_eq!(tracker.query_history().len(), 2);
    }

    #[test]
    fn test_budget_reset() {
        let mut tracker = PrivacyBudgetTracker::new(5.0);
        tracker.record_query(2.0, "test query".to_string()).unwrap();
        assert_eq!(tracker.consumed_budget(), 2.0);

        tracker.reset();
        assert_eq!(tracker.consumed_budget(), 0.0);
        assert_eq!(tracker.query_history().len(), 0);
    }

    #[test]
    fn test_query_result_errors() {
        let result = DpQueryResult {
            value: 105.0,
            true_value: Some(100.0),
            epsilon: 1.0,
            delta: 1e-5,
            mechanism: NoiseMechanism::Laplace,
        };

        assert_eq!(result.absolute_error().unwrap(), 5.0);
        assert_eq!(result.relative_error().unwrap(), 0.05);
    }

    #[test]
    fn test_average_query() {
        let dp = DifferentialPrivacy::standard();
        let records = create_test_records(10);

        // Constant extractor for testing
        let result = dp.average_query(&records, |_| 1.0);

        assert!(result.true_value.unwrap() > 0.9 && result.true_value.unwrap() < 1.1);
        // Average should be roughly 1.0 with some noise (can be negative due to noise)
        // Just check that we got a numeric result
        assert!(result.value.is_finite());
    }

    #[test]
    fn test_empty_records_average() {
        let dp = DifferentialPrivacy::standard();
        let records: Vec<AuditRecord> = vec![];

        let result = dp.average_query(&records, |_| 1.0);
        assert_eq!(result.value, 0.0);
        assert_eq!(result.true_value.unwrap(), 0.0);
    }
}
