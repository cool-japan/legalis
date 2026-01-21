//! Predictive analytics for compliance violations.
//!
//! This module provides predictive models to forecast potential compliance violations
//! based on historical audit data and decision patterns.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for predictive analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveConfig {
    /// Lookback window in days for training
    pub lookback_days: i64,
    /// Prediction horizon in days
    pub prediction_horizon_days: i64,
    /// Minimum confidence threshold for predictions
    pub min_confidence: f64,
    /// Enable time-series forecasting
    pub enable_time_series: bool,
    /// Enable pattern-based prediction
    pub enable_pattern_based: bool,
}

impl Default for PredictiveConfig {
    fn default() -> Self {
        Self {
            lookback_days: 90,
            prediction_horizon_days: 30,
            min_confidence: 0.7,
            enable_time_series: true,
            enable_pattern_based: true,
        }
    }
}

/// Predicted violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationPrediction {
    /// Type of predicted violation
    pub violation_type: ViolationType,
    /// Predicted time of violation
    pub predicted_time: DateTime<Utc>,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Contributing factors
    pub risk_factors: Vec<RiskFactor>,
    /// Recommended actions
    pub recommendations: Vec<String>,
    /// Affected statute IDs
    pub affected_statutes: Vec<String>,
    /// Affected subjects
    pub affected_subjects: Vec<Uuid>,
}

/// Type of compliance violation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ViolationType {
    /// Excessive override rate
    ExcessiveOverrides,
    /// Unusual decision volume
    AbnormalVolume,
    /// Biased decision patterns
    BiasDetected,
    /// Retention policy violation
    RetentionViolation,
    /// Integrity check failure
    IntegrityRisk,
    /// GDPR compliance issue
    GdprViolation,
    /// SOX compliance issue
    SoxViolation,
    /// HIPAA compliance issue
    HipaaViolation,
    /// Custom violation
    Custom(String),
}

/// Risk factor contributing to a prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Factor name
    pub name: String,
    /// Current value
    pub current_value: f64,
    /// Threshold value
    pub threshold: f64,
    /// Risk score contribution (0.0-1.0)
    pub contribution: f64,
    /// Trend direction
    pub trend: TrendDirection,
}

/// Trend direction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

/// Predictive analytics engine.
pub struct PredictiveAnalyzer {
    config: PredictiveConfig,
    historical_stats: Option<HistoricalStats>,
}

/// Historical statistics for prediction.
#[derive(Debug, Clone)]
struct HistoricalStats {
    avg_daily_volume: f64,
    avg_override_rate: f64,
    #[allow(dead_code)]
    avg_discretionary_rate: f64,
    #[allow(dead_code)]
    statute_frequencies: HashMap<String, usize>,
    #[allow(dead_code)]
    hourly_patterns: Vec<f64>,
    #[allow(dead_code)]
    daily_patterns: Vec<f64>,
}

impl PredictiveAnalyzer {
    /// Creates a new predictive analyzer with default configuration.
    pub fn new() -> Self {
        Self::with_config(PredictiveConfig::default())
    }

    /// Creates a new predictive analyzer with custom configuration.
    pub fn with_config(config: PredictiveConfig) -> Self {
        Self {
            config,
            historical_stats: None,
        }
    }

    /// Trains the analyzer on historical data.
    pub fn train(&mut self, records: &[AuditRecord]) -> AuditResult<()> {
        if records.is_empty() {
            return Err(AuditError::InvalidRecord(
                "Cannot train on empty dataset".to_string(),
            ));
        }

        // Compute historical statistics
        let stats = Self::compute_historical_stats(records);
        self.historical_stats = Some(stats);

        Ok(())
    }

    /// Predicts potential violations in the next prediction horizon.
    pub fn predict_violations(
        &self,
        current_records: &[AuditRecord],
    ) -> AuditResult<Vec<ViolationPrediction>> {
        let stats = self.historical_stats.as_ref().ok_or_else(|| {
            AuditError::InvalidRecord("Analyzer must be trained before prediction".to_string())
        })?;

        let mut predictions = Vec::new();

        // Check for excessive override rate trend
        if self.config.enable_pattern_based
            && let Some(pred) = self.predict_excessive_overrides(current_records, stats)?
        {
            predictions.push(pred);
        }

        // Check for abnormal volume trends
        if self.config.enable_time_series
            && let Some(pred) = self.predict_abnormal_volume(current_records, stats)?
        {
            predictions.push(pred);
        }

        // Check for bias patterns
        if let Some(pred) = self.predict_bias(current_records, stats)? {
            predictions.push(pred);
        }

        // Check for retention violations
        if let Some(pred) = self.predict_retention_violation(current_records)? {
            predictions.push(pred);
        }

        // Filter by minimum confidence
        predictions.retain(|p| p.confidence >= self.config.min_confidence);

        // Sort by confidence descending
        predictions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        Ok(predictions)
    }

    /// Predicts excessive override violations.
    fn predict_excessive_overrides(
        &self,
        records: &[AuditRecord],
        stats: &HistoricalStats,
    ) -> AuditResult<Option<ViolationPrediction>> {
        // Calculate current override rate
        let override_count = records
            .iter()
            .filter(|r| matches!(r.result, crate::DecisionResult::Overridden { .. }))
            .count();

        let current_override_rate = if !records.is_empty() {
            override_count as f64 / records.len() as f64
        } else {
            0.0
        };

        // Threshold: 2x historical average
        let threshold = stats.avg_override_rate * 2.0;

        if current_override_rate > threshold {
            let confidence = ((current_override_rate - threshold) / threshold).min(1.0);

            let risk_factors = vec![RiskFactor {
                name: "Override Rate".to_string(),
                current_value: current_override_rate,
                threshold,
                contribution: confidence,
                trend: if current_override_rate > stats.avg_override_rate * 1.5 {
                    TrendDirection::Increasing
                } else {
                    TrendDirection::Stable
                },
            }];

            let predicted_time = Utc::now() + Duration::days(7);

            return Ok(Some(ViolationPrediction {
                violation_type: ViolationType::ExcessiveOverrides,
                predicted_time,
                confidence,
                risk_factors,
                recommendations: vec![
                    "Review override justifications".to_string(),
                    "Audit decision-making process".to_string(),
                    "Consider rule refinement".to_string(),
                ],
                affected_statutes: Vec::new(),
                affected_subjects: Vec::new(),
            }));
        }

        Ok(None)
    }

    /// Predicts abnormal volume violations.
    fn predict_abnormal_volume(
        &self,
        records: &[AuditRecord],
        stats: &HistoricalStats,
    ) -> AuditResult<Option<ViolationPrediction>> {
        if records.is_empty() {
            return Ok(None);
        }

        // Group records by day
        let mut daily_volumes: HashMap<String, usize> = HashMap::new();
        for record in records {
            let date_key = record.timestamp.format("%Y-%m-%d").to_string();
            *daily_volumes.entry(date_key).or_insert(0) += 1;
        }

        // Find maximum daily volume
        let max_daily = *daily_volumes.values().max().unwrap_or(&0) as f64;

        // Threshold: 3x historical average
        let threshold = stats.avg_daily_volume * 3.0;

        if max_daily > threshold {
            let confidence = ((max_daily - threshold) / threshold).min(1.0);

            let risk_factors = vec![RiskFactor {
                name: "Daily Volume".to_string(),
                current_value: max_daily,
                threshold,
                contribution: confidence,
                trend: TrendDirection::Increasing,
            }];

            let predicted_time = Utc::now() + Duration::days(3);

            return Ok(Some(ViolationPrediction {
                violation_type: ViolationType::AbnormalVolume,
                predicted_time,
                confidence,
                risk_factors,
                recommendations: vec![
                    "Investigate cause of volume spike".to_string(),
                    "Check for system anomalies".to_string(),
                    "Review capacity planning".to_string(),
                ],
                affected_statutes: Vec::new(),
                affected_subjects: Vec::new(),
            }));
        }

        Ok(None)
    }

    /// Predicts bias violations.
    fn predict_bias(
        &self,
        records: &[AuditRecord],
        _stats: &HistoricalStats,
    ) -> AuditResult<Option<ViolationPrediction>> {
        // Group decisions by statute
        let mut statute_outcomes: HashMap<String, (usize, usize)> = HashMap::new();

        for record in records {
            let (approved, total) = statute_outcomes
                .entry(record.statute_id.clone())
                .or_insert((0, 0));

            *total += 1;
            if let crate::DecisionResult::Deterministic { effect_applied, .. } = &record.result
                && effect_applied.contains("approve")
            {
                *approved += 1;
            }
        }

        // Check for extreme bias (>90% or <10% approval)
        for (statute_id, (approved, total)) in statute_outcomes {
            if total < 10 {
                continue; // Need sufficient data
            }

            let approval_rate = approved as f64 / total as f64;

            if !(0.1..=0.9).contains(&approval_rate) {
                let confidence = if approval_rate > 0.9 {
                    (approval_rate - 0.9) * 10.0
                } else {
                    (0.1 - approval_rate) * 10.0
                };

                let risk_factors = vec![RiskFactor {
                    name: format!("Approval Rate for {}", statute_id),
                    current_value: approval_rate,
                    threshold: 0.5,
                    contribution: confidence,
                    trend: TrendDirection::Stable,
                }];

                let predicted_time = Utc::now() + Duration::days(14);

                return Ok(Some(ViolationPrediction {
                    violation_type: ViolationType::BiasDetected,
                    predicted_time,
                    confidence,
                    risk_factors,
                    recommendations: vec![
                        format!("Review decision logic for statute {}", statute_id),
                        "Check for systematic bias".to_string(),
                        "Consider rule balancing".to_string(),
                    ],
                    affected_statutes: vec![statute_id],
                    affected_subjects: Vec::new(),
                }));
            }
        }

        Ok(None)
    }

    /// Predicts retention policy violations.
    fn predict_retention_violation(
        &self,
        records: &[AuditRecord],
    ) -> AuditResult<Option<ViolationPrediction>> {
        let now = Utc::now();
        let retention_limit = now - Duration::days(365 * 7); // 7 years

        // Check for very old records
        let old_records: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp < retention_limit)
            .collect();

        if !old_records.is_empty() {
            let confidence = (old_records.len() as f64 / records.len() as f64).min(1.0);

            let risk_factors = vec![RiskFactor {
                name: "Old Records".to_string(),
                current_value: old_records.len() as f64,
                threshold: 0.0,
                contribution: confidence,
                trend: TrendDirection::Increasing,
            }];

            let predicted_time = now + Duration::days(30);

            return Ok(Some(ViolationPrediction {
                violation_type: ViolationType::RetentionViolation,
                predicted_time,
                confidence,
                risk_factors,
                recommendations: vec![
                    "Review retention policies".to_string(),
                    "Archive old records".to_string(),
                    "Implement automated cleanup".to_string(),
                ],
                affected_statutes: Vec::new(),
                affected_subjects: old_records.iter().map(|r| r.subject_id).collect(),
            }));
        }

        Ok(None)
    }

    /// Computes historical statistics from records.
    fn compute_historical_stats(records: &[AuditRecord]) -> HistoricalStats {
        let total = records.len();

        // Daily volume
        let mut daily_counts: HashMap<String, usize> = HashMap::new();
        for record in records {
            let date_key = record.timestamp.format("%Y-%m-%d").to_string();
            *daily_counts.entry(date_key).or_insert(0) += 1;
        }
        let avg_daily_volume = if !daily_counts.is_empty() {
            daily_counts.values().sum::<usize>() as f64 / daily_counts.len() as f64
        } else {
            0.0
        };

        // Override rate
        let override_count = records
            .iter()
            .filter(|r| matches!(r.result, crate::DecisionResult::Overridden { .. }))
            .count();
        let avg_override_rate = if total > 0 {
            override_count as f64 / total as f64
        } else {
            0.0
        };

        // Discretionary rate
        let discretionary_count = records
            .iter()
            .filter(|r| matches!(r.result, crate::DecisionResult::RequiresDiscretion { .. }))
            .count();
        let avg_discretionary_rate = if total > 0 {
            discretionary_count as f64 / total as f64
        } else {
            0.0
        };

        // Statute frequencies
        let mut statute_frequencies: HashMap<String, usize> = HashMap::new();
        for record in records {
            *statute_frequencies
                .entry(record.statute_id.clone())
                .or_insert(0) += 1;
        }

        // Hourly patterns
        let mut hourly_counts = [0; 24];
        for record in records {
            let hour = record.timestamp.time().hour() as usize;
            hourly_counts[hour] += 1;
        }
        let hourly_patterns: Vec<f64> = hourly_counts
            .iter()
            .map(|&c| {
                if total > 0 {
                    c as f64 / total as f64
                } else {
                    0.0
                }
            })
            .collect();

        // Daily patterns (day of week)
        let mut daily_counts = [0; 7];
        for record in records {
            let day = record.timestamp.weekday().num_days_from_monday() as usize;
            daily_counts[day] += 1;
        }
        let daily_patterns: Vec<f64> = daily_counts
            .iter()
            .map(|&c| {
                if total > 0 {
                    c as f64 / total as f64
                } else {
                    0.0
                }
            })
            .collect();

        HistoricalStats {
            avg_daily_volume,
            avg_override_rate,
            avg_discretionary_rate,
            statute_frequencies,
            hourly_patterns,
            daily_patterns,
        }
    }

    /// Returns the configuration.
    pub fn config(&self) -> &PredictiveConfig {
        &self.config
    }

    /// Returns true if the analyzer has been trained.
    pub fn is_trained(&self) -> bool {
        self.historical_stats.is_some()
    }
}

impl Default for PredictiveAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn create_test_record(days_ago: i64, is_override: bool) -> AuditRecord {
        let timestamp = Utc::now() - Duration::days(days_ago);

        let result = if is_override {
            DecisionResult::Overridden {
                original_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: HashMap::new(),
                }),
                new_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "denied".to_string(),
                    parameters: HashMap::new(),
                }),
                justification: "test override".to_string(),
            }
        } else {
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            }
        };

        AuditRecord {
            id: Uuid::new_v4(),
            timestamp,
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "test".to_string(),
            },
            statute_id: "test-statute".to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result,
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    #[test]
    fn test_predictive_analyzer_creation() {
        let analyzer = PredictiveAnalyzer::new();
        assert!(!analyzer.is_trained());
        assert_eq!(analyzer.config().lookback_days, 90);
    }

    #[test]
    fn test_predictive_analyzer_training() {
        let mut analyzer = PredictiveAnalyzer::new();

        let records: Vec<_> = (0..100).map(|i| create_test_record(i, false)).collect();

        let result = analyzer.train(&records);
        assert!(result.is_ok());
        assert!(analyzer.is_trained());
    }

    #[test]
    fn test_predict_excessive_overrides() {
        let mut analyzer = PredictiveAnalyzer::new();

        // Training data with low override rate
        let training: Vec<_> = (0..100).map(|i| create_test_record(i, false)).collect();
        analyzer.train(&training).unwrap();

        // Test data with high override rate
        let test: Vec<_> = (0..10).map(|i| create_test_record(i, true)).collect();

        let predictions = analyzer.predict_violations(&test).unwrap();

        // Should detect excessive overrides
        assert!(!predictions.is_empty());
        assert!(
            predictions
                .iter()
                .any(|p| p.violation_type == ViolationType::ExcessiveOverrides)
        );
    }

    #[test]
    fn test_predict_abnormal_volume() {
        let mut analyzer = PredictiveAnalyzer::new();

        // Training data with normal volume
        let training: Vec<_> = (0..50).map(|i| create_test_record(i, false)).collect();
        analyzer.train(&training).unwrap();

        // Test data with high volume
        let test: Vec<_> = (0..200).map(|_| create_test_record(0, false)).collect();

        let predictions = analyzer.predict_violations(&test).unwrap();

        // Should detect abnormal volume
        assert!(
            predictions
                .iter()
                .any(|p| p.violation_type == ViolationType::AbnormalVolume)
        );
    }

    #[test]
    fn test_predict_retention_violation() {
        let mut analyzer = PredictiveAnalyzer::new();

        // Training data
        let training: Vec<_> = (0..50).map(|i| create_test_record(i, false)).collect();
        analyzer.train(&training).unwrap();

        // Test data with very old records
        let test: Vec<_> = (2555..2560).map(|i| create_test_record(i, false)).collect();

        let predictions = analyzer.predict_violations(&test).unwrap();

        // Should detect retention violation
        assert!(
            predictions
                .iter()
                .any(|p| p.violation_type == ViolationType::RetentionViolation)
        );
    }

    #[test]
    fn test_empty_training_data() {
        let mut analyzer = PredictiveAnalyzer::new();
        let records: Vec<AuditRecord> = Vec::new();

        let result = analyzer.train(&records);
        assert!(result.is_err());
    }

    #[test]
    fn test_prediction_without_training() {
        let analyzer = PredictiveAnalyzer::new();
        let records: Vec<AuditRecord> = vec![create_test_record(0, false)];

        let result = analyzer.predict_violations(&records);
        assert!(result.is_err());
    }
}
