//! Bias detection in automated decisions.
//!
//! This module provides statistical analysis to detect potential biases in
//! decision-making systems. It can identify:
//! - Disparate impact across different groups
//! - Outcome disparities by actor or statute
//! - Unusual approval/rejection patterns
//! - Statistical significance of observed differences

use crate::{AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A bias detection report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasReport {
    /// When the analysis was performed
    pub analyzed_at: chrono::DateTime<chrono::Utc>,
    /// Total records analyzed
    pub total_records: usize,
    /// Detected bias indicators
    pub bias_indicators: Vec<BiasIndicator>,
    /// Statistical summary
    pub statistics: BiasStatistics,
}

/// A detected bias indicator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasIndicator {
    /// Type of bias detected
    pub bias_type: BiasType,
    /// Severity level (0.0 to 1.0)
    pub severity: f64,
    /// Description of the bias
    pub description: String,
    /// Affected groups or categories
    pub affected_groups: Vec<String>,
    /// Statistical evidence
    pub evidence: BiasEvidence,
}

/// Types of bias that can be detected.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BiasType {
    /// Disparate impact on different groups
    DisparateImpact,
    /// Approval rate bias
    ApprovalRateBias,
    /// Override pattern bias
    OverridePatternBias,
    /// Temporal bias (different outcomes at different times)
    TemporalBias,
    /// Actor bias (different outcomes by who makes the decision)
    ActorBias,
}

/// Statistical evidence for bias.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasEvidence {
    /// Expected outcome rate
    pub expected_rate: f64,
    /// Observed outcome rate
    pub observed_rate: f64,
    /// Statistical significance (p-value)
    pub p_value: f64,
    /// Sample sizes for groups
    pub sample_sizes: HashMap<String, usize>,
    /// Outcome rates for groups
    pub group_rates: HashMap<String, f64>,
}

/// Statistical summary for bias analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasStatistics {
    /// Overall approval rate
    pub overall_approval_rate: f64,
    /// Variance in approval rates across groups
    pub approval_rate_variance: f64,
    /// Number of distinct groups analyzed
    pub groups_analyzed: usize,
    /// Chi-square statistic
    pub chi_square: f64,
}

/// Bias detector for analyzing audit records.
pub struct BiasDetector {
    /// Confidence level for statistical tests (default: 0.95)
    confidence_level: f64,
    /// Minimum sample size per group
    min_sample_size: usize,
}

impl BiasDetector {
    /// Creates a new bias detector with default settings.
    pub fn new() -> Self {
        Self {
            confidence_level: 0.95,
            min_sample_size: 30,
        }
    }

    /// Sets the confidence level for statistical tests.
    pub fn with_confidence_level(mut self, confidence_level: f64) -> Self {
        self.confidence_level = confidence_level;
        self
    }

    /// Sets the minimum sample size per group.
    pub fn with_min_sample_size(mut self, min_sample_size: usize) -> Self {
        self.min_sample_size = min_sample_size;
        self
    }

    /// Analyzes records for potential biases.
    pub fn analyze(&self, records: &[AuditRecord]) -> AuditResult<BiasReport> {
        let mut bias_indicators = Vec::new();

        // Detect disparate impact by statute
        if let Some(indicator) = self.detect_disparate_impact_by_statute(records) {
            bias_indicators.push(indicator);
        }

        // Detect approval rate biases
        if let Some(indicator) = self.detect_approval_rate_bias(records) {
            bias_indicators.push(indicator);
        }

        // Detect override pattern biases
        if let Some(indicator) = self.detect_override_pattern_bias(records) {
            bias_indicators.push(indicator);
        }

        // Detect temporal biases
        if let Some(indicator) = self.detect_temporal_bias(records) {
            bias_indicators.push(indicator);
        }

        // Detect actor biases
        if let Some(indicator) = self.detect_actor_bias(records) {
            bias_indicators.push(indicator);
        }

        let statistics = self.calculate_statistics(records);

        Ok(BiasReport {
            analyzed_at: chrono::Utc::now(),
            total_records: records.len(),
            bias_indicators,
            statistics,
        })
    }

    /// Detects disparate impact by statute.
    fn detect_disparate_impact_by_statute(&self, records: &[AuditRecord]) -> Option<BiasIndicator> {
        let mut statute_outcomes: HashMap<String, (usize, usize)> = HashMap::new();

        for record in records {
            let entry = statute_outcomes
                .entry(record.statute_id.clone())
                .or_insert((0, 0));
            entry.0 += 1; // total
            if self.is_positive_outcome(record) {
                entry.1 += 1; // positive outcomes
            }
        }

        // Find statutes with significantly different approval rates
        let overall_rate = self.calculate_overall_approval_rate(records);
        let mut max_deviation: f64 = 0.0;
        let mut affected_statutes = Vec::new();
        let mut group_rates = HashMap::new();
        let mut sample_sizes = HashMap::new();

        for (statute, (total, positive)) in &statute_outcomes {
            if *total >= self.min_sample_size {
                let rate = *positive as f64 / *total as f64;
                let deviation = (rate - overall_rate).abs();
                group_rates.insert(statute.clone(), rate);
                sample_sizes.insert(statute.clone(), *total);

                if deviation > 0.2 {
                    // 20% deviation threshold
                    max_deviation = max_deviation.max(deviation);
                    affected_statutes.push(statute.clone());
                }
            }
        }

        if !affected_statutes.is_empty() {
            let p_value = self.calculate_chi_square_p_value(&statute_outcomes, overall_rate);
            Some(BiasIndicator {
                bias_type: BiasType::DisparateImpact,
                severity: (max_deviation * 2.0_f64).min(1.0_f64),
                description: format!(
                    "Disparate impact detected across {} statutes with approval rates deviating by up to {:.1}%",
                    affected_statutes.len(),
                    max_deviation * 100.0
                ),
                affected_groups: affected_statutes,
                evidence: BiasEvidence {
                    expected_rate: overall_rate,
                    observed_rate: overall_rate,
                    p_value,
                    sample_sizes,
                    group_rates,
                },
            })
        } else {
            None
        }
    }

    /// Detects approval rate biases.
    fn detect_approval_rate_bias(&self, records: &[AuditRecord]) -> Option<BiasIndicator> {
        let overall_rate = self.calculate_overall_approval_rate(records);

        // Check if overall rate is extremely high or low
        if overall_rate > 0.95 {
            Some(BiasIndicator {
                bias_type: BiasType::ApprovalRateBias,
                severity: ((overall_rate - 0.95) * 20.0).min(1.0),
                description: format!(
                    "Unusually high approval rate: {:.1}% (expected: 50-95%)",
                    overall_rate * 100.0
                ),
                affected_groups: vec!["all".to_string()],
                evidence: BiasEvidence {
                    expected_rate: 0.75,
                    observed_rate: overall_rate,
                    p_value: 0.01,
                    sample_sizes: [("all".to_string(), records.len())].into_iter().collect(),
                    group_rates: [("all".to_string(), overall_rate)].into_iter().collect(),
                },
            })
        } else if overall_rate < 0.05 {
            Some(BiasIndicator {
                bias_type: BiasType::ApprovalRateBias,
                severity: ((0.05 - overall_rate) * 20.0).min(1.0),
                description: format!(
                    "Unusually low approval rate: {:.1}% (expected: 5-50%)",
                    overall_rate * 100.0
                ),
                affected_groups: vec!["all".to_string()],
                evidence: BiasEvidence {
                    expected_rate: 0.25,
                    observed_rate: overall_rate,
                    p_value: 0.01,
                    sample_sizes: [("all".to_string(), records.len())].into_iter().collect(),
                    group_rates: [("all".to_string(), overall_rate)].into_iter().collect(),
                },
            })
        } else {
            None
        }
    }

    /// Detects override pattern biases.
    fn detect_override_pattern_bias(&self, records: &[AuditRecord]) -> Option<BiasIndicator> {
        let override_count = records
            .iter()
            .filter(|r| matches!(r.result, crate::DecisionResult::Overridden { .. }))
            .count();

        let override_rate = override_count as f64 / records.len() as f64;

        if override_rate > 0.15 {
            // >15% override rate is concerning
            Some(BiasIndicator {
                bias_type: BiasType::OverridePatternBias,
                severity: ((override_rate - 0.15) * 10.0).min(1.0),
                description: format!(
                    "High override rate: {:.1}% (expected: <15%)",
                    override_rate * 100.0
                ),
                affected_groups: vec!["overrides".to_string()],
                evidence: BiasEvidence {
                    expected_rate: 0.10,
                    observed_rate: override_rate,
                    p_value: 0.05,
                    sample_sizes: [("overrides".to_string(), records.len())]
                        .into_iter()
                        .collect(),
                    group_rates: [("overrides".to_string(), override_rate)]
                        .into_iter()
                        .collect(),
                },
            })
        } else {
            None
        }
    }

    /// Detects temporal biases.
    fn detect_temporal_bias(&self, records: &[AuditRecord]) -> Option<BiasIndicator> {
        use chrono::Timelike;

        let mut hour_outcomes: HashMap<u32, (usize, usize)> = HashMap::new();

        for record in records {
            let hour = record.timestamp.hour();
            let entry = hour_outcomes.entry(hour).or_insert((0, 0));
            entry.0 += 1;
            if self.is_positive_outcome(record) {
                entry.1 += 1;
            }
        }

        // Check for significant variance across hours
        let mut max_rate: f64 = 0.0;
        let mut min_rate: f64 = 1.0;
        let mut group_rates = HashMap::new();

        for (hour, (total, positive)) in &hour_outcomes {
            if *total >= 10 {
                // minimum sample
                let rate = *positive as f64 / *total as f64;
                max_rate = max_rate.max(rate);
                min_rate = min_rate.min(rate);
                group_rates.insert(format!("hour_{}", hour), rate);
            }
        }

        let variance = max_rate - min_rate;
        if variance > 0.3 {
            // 30% variance
            Some(BiasIndicator {
                bias_type: BiasType::TemporalBias,
                severity: ((variance - 0.3_f64) * 2.0_f64).min(1.0_f64),
                description: format!(
                    "Temporal bias detected: {:.1}% variance in outcomes across different hours",
                    variance * 100.0
                ),
                affected_groups: vec!["time_based".to_string()],
                evidence: BiasEvidence {
                    expected_rate: 0.5,
                    observed_rate: (max_rate + min_rate) / 2.0,
                    p_value: 0.05,
                    sample_sizes: HashMap::new(),
                    group_rates,
                },
            })
        } else {
            None
        }
    }

    /// Detects actor biases.
    fn detect_actor_bias(&self, records: &[AuditRecord]) -> Option<BiasIndicator> {
        let mut actor_outcomes: HashMap<String, (usize, usize)> = HashMap::new();

        for record in records {
            let actor_key = match &record.actor {
                crate::Actor::System { component } => format!("system:{}", component),
                crate::Actor::User { user_id, .. } => format!("user:{}", user_id),
                crate::Actor::External { system_id } => format!("external:{}", system_id),
            };
            let entry = actor_outcomes.entry(actor_key).or_insert((0, 0));
            entry.0 += 1;
            if self.is_positive_outcome(record) {
                entry.1 += 1;
            }
        }

        let overall_rate = self.calculate_overall_approval_rate(records);
        let mut max_deviation: f64 = 0.0;
        let mut affected_actors = Vec::new();
        let mut group_rates = HashMap::new();

        for (actor, (total, positive)) in &actor_outcomes {
            if *total >= self.min_sample_size {
                let rate = *positive as f64 / *total as f64;
                let deviation = (rate - overall_rate).abs();
                group_rates.insert(actor.clone(), rate);

                if deviation > 0.25 {
                    // 25% deviation
                    max_deviation = max_deviation.max(deviation);
                    affected_actors.push(actor.clone());
                }
            }
        }

        if !affected_actors.is_empty() {
            Some(BiasIndicator {
                bias_type: BiasType::ActorBias,
                severity: (max_deviation * 2.0_f64).min(1.0_f64),
                description: format!(
                    "Actor bias detected: {} actors show approval rates deviating by up to {:.1}%",
                    affected_actors.len(),
                    max_deviation * 100.0
                ),
                affected_groups: affected_actors,
                evidence: BiasEvidence {
                    expected_rate: overall_rate,
                    observed_rate: overall_rate,
                    p_value: 0.05,
                    sample_sizes: HashMap::new(),
                    group_rates,
                },
            })
        } else {
            None
        }
    }

    /// Checks if a record has a positive outcome.
    fn is_positive_outcome(&self, record: &AuditRecord) -> bool {
        matches!(record.result, crate::DecisionResult::Deterministic { .. })
    }

    /// Calculates overall approval rate.
    fn calculate_overall_approval_rate(&self, records: &[AuditRecord]) -> f64 {
        if records.is_empty() {
            return 0.0;
        }
        let positive = records
            .iter()
            .filter(|r| self.is_positive_outcome(r))
            .count();
        positive as f64 / records.len() as f64
    }

    /// Calculates chi-square p-value (simplified).
    fn calculate_chi_square_p_value(
        &self,
        outcomes: &HashMap<String, (usize, usize)>,
        expected_rate: f64,
    ) -> f64 {
        let mut chi_square = 0.0;
        for (total, positive) in outcomes.values() {
            let expected = *total as f64 * expected_rate;
            let observed = *positive as f64;
            if expected > 0.0 {
                chi_square += (observed - expected).powi(2) / expected;
            }
        }
        // Simplified p-value approximation
        if chi_square > 10.0 {
            0.001
        } else if chi_square > 5.0 {
            0.025
        } else {
            0.1
        }
    }

    /// Calculates statistical summary.
    fn calculate_statistics(&self, records: &[AuditRecord]) -> BiasStatistics {
        let overall_rate = self.calculate_overall_approval_rate(records);

        // Calculate variance across statutes
        let mut statute_rates: HashMap<String, f64> = HashMap::new();
        let mut statute_counts: HashMap<String, usize> = HashMap::new();

        for record in records {
            *statute_counts.entry(record.statute_id.clone()).or_insert(0) += 1;
        }

        for (statute, count) in &statute_counts {
            let positive = records
                .iter()
                .filter(|r| r.statute_id == *statute && self.is_positive_outcome(r))
                .count();
            statute_rates.insert(statute.clone(), positive as f64 / *count as f64);
        }

        let variance = if !statute_rates.is_empty() {
            let mean = statute_rates.values().sum::<f64>() / statute_rates.len() as f64;
            statute_rates
                .values()
                .map(|rate| (rate - mean).powi(2))
                .sum::<f64>()
                / statute_rates.len() as f64
        } else {
            0.0
        };

        BiasStatistics {
            overall_approval_rate: overall_rate,
            approval_rate_variance: variance,
            groups_analyzed: statute_rates.len(),
            chi_square: 0.0, // Simplified
        }
    }
}

impl Default for BiasDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;
    use uuid::Uuid;

    fn create_test_record(statute_id: &str, positive: bool) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute_id.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            if positive {
                DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: StdHashMap::new(),
                }
            } else {
                DecisionResult::RequiresDiscretion {
                    issue: "review required".to_string(),
                    narrative_hint: None,
                    assigned_to: None,
                }
            },
            None,
        )
    }

    #[test]
    fn test_bias_detector_creation() {
        let detector = BiasDetector::new();
        assert_eq!(detector.confidence_level, 0.95);
        assert_eq!(detector.min_sample_size, 30);
    }

    #[test]
    fn test_bias_detector_with_settings() {
        let detector = BiasDetector::new()
            .with_confidence_level(0.99)
            .with_min_sample_size(50);
        assert_eq!(detector.confidence_level, 0.99);
        assert_eq!(detector.min_sample_size, 50);
    }

    #[test]
    fn test_overall_approval_rate() {
        let detector = BiasDetector::new();
        let records = vec![
            create_test_record("statute-1", true),
            create_test_record("statute-1", true),
            create_test_record("statute-1", false),
        ];

        let rate = detector.calculate_overall_approval_rate(&records);
        assert!((rate - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_bias_analysis_no_bias() {
        let detector = BiasDetector::new().with_min_sample_size(5);
        let records = vec![
            create_test_record("statute-1", true),
            create_test_record("statute-1", true),
            create_test_record("statute-1", false),
            create_test_record("statute-2", true),
            create_test_record("statute-2", false),
            create_test_record("statute-2", false),
        ];

        let report = detector.analyze(&records).unwrap();
        assert_eq!(report.total_records, 6);
    }

    #[test]
    fn test_high_approval_rate_bias() {
        let detector = BiasDetector::new();
        let mut records = Vec::new();
        for _ in 0..100 {
            records.push(create_test_record("statute-1", true));
        }

        let report = detector.analyze(&records).unwrap();
        assert!(
            report
                .bias_indicators
                .iter()
                .any(|i| i.bias_type == BiasType::ApprovalRateBias)
        );
    }

    #[test]
    fn test_override_pattern_bias() {
        let detector = BiasDetector::new();
        let mut records = Vec::new();
        for i in 0..100 {
            let mut record = create_test_record("statute-1", true);
            if i < 20 {
                record.result = DecisionResult::Overridden {
                    original_result: Box::new(DecisionResult::Deterministic {
                        effect_applied: "test".to_string(),
                        parameters: StdHashMap::new(),
                    }),
                    new_result: Box::new(DecisionResult::Deterministic {
                        effect_applied: "override".to_string(),
                        parameters: StdHashMap::new(),
                    }),
                    justification: "test".to_string(),
                };
            }
            records.push(record);
        }

        let report = detector.analyze(&records).unwrap();
        assert!(
            report
                .bias_indicators
                .iter()
                .any(|i| i.bias_type == BiasType::OverridePatternBias)
        );
    }

    #[test]
    fn test_statistics_calculation() {
        let detector = BiasDetector::new();
        let records = vec![
            create_test_record("statute-1", true),
            create_test_record("statute-1", false),
            create_test_record("statute-2", true),
            create_test_record("statute-2", true),
        ];

        let stats = detector.calculate_statistics(&records);
        assert_eq!(stats.groups_analyzed, 2);
        assert!(stats.overall_approval_rate > 0.0);
    }
}
