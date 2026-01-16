//! ML-based anomaly detection for audit trails.
//!
//! This module provides machine learning-based anomaly detection algorithms
//! to identify unusual patterns in decision-making behavior.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for ML-based anomaly detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLAnomalyConfig {
    /// Sensitivity threshold (0.0-1.0), lower values detect more anomalies
    pub sensitivity: f64,
    /// Minimum number of records required for training
    pub min_training_samples: usize,
    /// Window size for temporal analysis (in hours)
    pub temporal_window_hours: i64,
    /// Enable isolation forest algorithm
    pub enable_isolation_forest: bool,
    /// Enable one-class SVM algorithm
    pub enable_one_class_svm: bool,
    /// Enable local outlier factor
    pub enable_lof: bool,
}

impl Default for MLAnomalyConfig {
    fn default() -> Self {
        Self {
            sensitivity: 0.1,
            min_training_samples: 100,
            temporal_window_hours: 24,
            enable_isolation_forest: true,
            enable_one_class_svm: false,
            enable_lof: true,
        }
    }
}

/// Detected anomaly with ML-based scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLAnomaly {
    /// Record that is anomalous
    pub record_id: Uuid,
    /// Timestamp of the anomaly
    pub timestamp: DateTime<Utc>,
    /// Anomaly score (0.0-1.0), higher = more anomalous
    pub anomaly_score: f64,
    /// Contributing factors
    pub factors: Vec<AnomalyFactor>,
    /// Detection algorithm used
    pub algorithm: String,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
}

/// Factor contributing to anomaly detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyFactor {
    /// Feature name
    pub feature: String,
    /// Feature value
    pub value: f64,
    /// Expected range
    pub expected_range: (f64, f64),
    /// Contribution to anomaly score
    pub contribution: f64,
}

/// Feature vector extracted from an audit record.
#[derive(Debug, Clone)]
struct FeatureVector {
    record_id: Uuid,
    features: Vec<f64>,
    feature_names: Vec<String>,
}

/// ML-based anomaly detector.
pub struct MLAnomalyDetector {
    config: MLAnomalyConfig,
    trained: bool,
    feature_stats: HashMap<String, FeatureStats>,
}

/// Statistics for a feature.
#[derive(Debug, Clone)]
struct FeatureStats {
    mean: f64,
    std_dev: f64,
    #[allow(dead_code)]
    min: f64,
    #[allow(dead_code)]
    max: f64,
    #[allow(dead_code)]
    count: usize,
}

impl MLAnomalyDetector {
    /// Creates a new ML anomaly detector with default configuration.
    pub fn new() -> Self {
        Self::with_config(MLAnomalyConfig::default())
    }

    /// Creates a new ML anomaly detector with custom configuration.
    pub fn with_config(config: MLAnomalyConfig) -> Self {
        Self {
            config,
            trained: false,
            feature_stats: HashMap::new(),
        }
    }

    /// Trains the detector on historical audit records.
    pub fn train(&mut self, records: &[AuditRecord]) -> AuditResult<()> {
        if records.len() < self.config.min_training_samples {
            return Err(AuditError::InvalidRecord(format!(
                "Insufficient training data: {} records, need at least {}",
                records.len(),
                self.config.min_training_samples
            )));
        }

        // Extract features from all records
        let feature_vectors = Self::extract_features(records);

        // Compute statistics for each feature
        self.feature_stats = Self::compute_feature_stats(&feature_vectors);

        self.trained = true;
        Ok(())
    }

    /// Detects anomalies in the given records using trained model.
    pub fn detect(&self, records: &[AuditRecord]) -> AuditResult<Vec<MLAnomaly>> {
        if !self.trained {
            return Err(AuditError::InvalidRecord(
                "Detector must be trained before detection".to_string(),
            ));
        }

        let mut anomalies = Vec::new();

        // Extract features
        let feature_vectors = Self::extract_features(records);

        for fv in &feature_vectors {
            let mut total_score = 0.0;
            let mut factors = Vec::new();

            // Compute anomaly score for each feature
            for (i, &value) in fv.features.iter().enumerate() {
                let feature_name = &fv.feature_names[i];
                if let Some(stats) = self.feature_stats.get(feature_name) {
                    let z_score = if stats.std_dev > 0.0 {
                        (value - stats.mean).abs() / stats.std_dev
                    } else {
                        0.0
                    };

                    // Anomaly if z-score > 3 (99.7% rule)
                    if z_score > 3.0 {
                        let contribution = (z_score - 3.0) / 10.0; // Normalize
                        factors.push(AnomalyFactor {
                            feature: feature_name.clone(),
                            value,
                            expected_range: (
                                stats.mean - 3.0 * stats.std_dev,
                                stats.mean + 3.0 * stats.std_dev,
                            ),
                            contribution: contribution.min(1.0),
                        });
                        total_score += contribution;
                    }
                }
            }

            // Normalize total score
            let anomaly_score = (total_score / fv.features.len() as f64).min(1.0);

            // Create anomaly if score exceeds threshold
            if anomaly_score >= self.config.sensitivity {
                let record = records.iter().find(|r| r.id == fv.record_id).unwrap();
                anomalies.push(MLAnomaly {
                    record_id: fv.record_id,
                    timestamp: record.timestamp,
                    anomaly_score,
                    factors,
                    algorithm: "Statistical Z-Score".to_string(),
                    confidence: 1.0 - self.config.sensitivity,
                });
            }
        }

        // Apply isolation forest if enabled
        if self.config.enable_isolation_forest {
            let iso_anomalies = self.isolation_forest_detect(&feature_vectors, records)?;
            anomalies.extend(iso_anomalies);
        }

        // Apply LOF if enabled
        if self.config.enable_lof {
            let lof_anomalies = self.local_outlier_factor_detect(&feature_vectors, records)?;
            anomalies.extend(lof_anomalies);
        }

        // Remove duplicates and sort by score
        anomalies.sort_by(|a, b| b.anomaly_score.partial_cmp(&a.anomaly_score).unwrap());
        anomalies.dedup_by_key(|a| a.record_id);

        Ok(anomalies)
    }

    /// Isolation Forest algorithm for anomaly detection.
    fn isolation_forest_detect(
        &self,
        feature_vectors: &[FeatureVector],
        records: &[AuditRecord],
    ) -> AuditResult<Vec<MLAnomaly>> {
        let mut anomalies = Vec::new();

        // Simplified isolation forest: compute average path length
        for fv in feature_vectors {
            let avg_path_length = self.compute_isolation_score(&fv.features, feature_vectors);

            // Anomaly score based on path length
            let anomaly_score = if avg_path_length < 2.0 {
                1.0 - (avg_path_length / 2.0)
            } else {
                0.0
            };

            if anomaly_score >= self.config.sensitivity {
                let record = records.iter().find(|r| r.id == fv.record_id).unwrap();
                anomalies.push(MLAnomaly {
                    record_id: fv.record_id,
                    timestamp: record.timestamp,
                    anomaly_score,
                    factors: vec![],
                    algorithm: "Isolation Forest".to_string(),
                    confidence: anomaly_score,
                });
            }
        }

        Ok(anomalies)
    }

    /// Local Outlier Factor (LOF) algorithm for anomaly detection.
    fn local_outlier_factor_detect(
        &self,
        feature_vectors: &[FeatureVector],
        records: &[AuditRecord],
    ) -> AuditResult<Vec<MLAnomaly>> {
        let mut anomalies = Vec::new();
        let k = 10.min(feature_vectors.len() / 2); // Number of neighbors

        for (idx, fv) in feature_vectors.iter().enumerate() {
            // Compute k-nearest neighbors
            let mut distances: Vec<(usize, f64)> = feature_vectors
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(i, other)| (i, Self::euclidean_distance(&fv.features, &other.features)))
                .collect();

            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let neighbors: Vec<_> = distances.iter().take(k).collect();

            if neighbors.is_empty() {
                continue;
            }

            // Compute local reachability density
            let avg_neighbor_dist: f64 = neighbors.iter().map(|(_, d)| d).sum::<f64>() / k as f64;

            // LOF score
            let lof_score = if avg_neighbor_dist > 0.0 {
                let local_density = 1.0 / avg_neighbor_dist;
                let neighbor_densities: Vec<f64> = neighbors
                    .iter()
                    .map(|(i, _)| {
                        let n_dists: Vec<f64> = feature_vectors
                            .iter()
                            .enumerate()
                            .filter(|(j, _)| j != i)
                            .map(|(_, other)| {
                                Self::euclidean_distance(
                                    &feature_vectors[*i].features,
                                    &other.features,
                                )
                            })
                            .collect();
                        let avg_n_dist = n_dists.iter().sum::<f64>() / n_dists.len() as f64;
                        if avg_n_dist > 0.0 {
                            1.0 / avg_n_dist
                        } else {
                            1.0
                        }
                    })
                    .collect();

                let avg_neighbor_density =
                    neighbor_densities.iter().sum::<f64>() / neighbor_densities.len() as f64;

                if local_density > 0.0 {
                    avg_neighbor_density / local_density
                } else {
                    1.0
                }
            } else {
                1.0
            };

            // Anomaly score: LOF > 1.5 is considered anomalous
            let anomaly_score = if lof_score > 1.5 {
                ((lof_score - 1.5) / 5.0).min(1.0)
            } else {
                0.0
            };

            if anomaly_score >= self.config.sensitivity {
                let record = records.iter().find(|r| r.id == fv.record_id).unwrap();
                anomalies.push(MLAnomaly {
                    record_id: fv.record_id,
                    timestamp: record.timestamp,
                    anomaly_score,
                    factors: vec![],
                    algorithm: "Local Outlier Factor".to_string(),
                    confidence: anomaly_score,
                });
            }
        }

        Ok(anomalies)
    }

    /// Computes isolation score for a data point.
    fn compute_isolation_score(&self, point: &[f64], all_points: &[FeatureVector]) -> f64 {
        // Simplified: compute average distance to all other points
        let distances: Vec<f64> = all_points
            .iter()
            .map(|fv| Self::euclidean_distance(point, &fv.features))
            .collect();

        distances.iter().sum::<f64>() / distances.len() as f64
    }

    /// Computes Euclidean distance between two feature vectors.
    fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Extracts feature vectors from audit records.
    fn extract_features(records: &[AuditRecord]) -> Vec<FeatureVector> {
        records
            .iter()
            .map(|record| {
                let mut features = Vec::new();
                let mut feature_names = Vec::new();

                // Temporal features
                let hour_of_day = record.timestamp.time().hour() as f64;
                features.push(hour_of_day);
                feature_names.push("hour_of_day".to_string());

                let day_of_week = record.timestamp.weekday().num_days_from_monday() as f64;
                features.push(day_of_week);
                feature_names.push("day_of_week".to_string());

                // Decision type features
                let is_override = match &record.result {
                    crate::DecisionResult::Overridden { .. } => 1.0,
                    _ => 0.0,
                };
                features.push(is_override);
                feature_names.push("is_override".to_string());

                let is_discretionary = match &record.result {
                    crate::DecisionResult::RequiresDiscretion { .. } => 1.0,
                    _ => 0.0,
                };
                features.push(is_discretionary);
                feature_names.push("is_discretionary".to_string());

                // Context complexity
                let context_size = record.context.attributes.len() as f64;
                features.push(context_size);
                feature_names.push("context_complexity".to_string());

                let conditions_count = record.context.evaluated_conditions.len() as f64;
                features.push(conditions_count);
                feature_names.push("conditions_count".to_string());

                FeatureVector {
                    record_id: record.id,
                    features,
                    feature_names,
                }
            })
            .collect()
    }

    /// Computes statistics for each feature across all vectors.
    fn compute_feature_stats(vectors: &[FeatureVector]) -> HashMap<String, FeatureStats> {
        let mut stats_map = HashMap::new();

        if vectors.is_empty() {
            return stats_map;
        }

        let num_features = vectors[0].features.len();

        for i in 0..num_features {
            let feature_name = &vectors[0].feature_names[i];
            let values: Vec<f64> = vectors.iter().map(|v| v.features[i]).collect();

            let count = values.len();
            let mean = values.iter().sum::<f64>() / count as f64;
            let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / count as f64;
            let std_dev = variance.sqrt();
            let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

            stats_map.insert(
                feature_name.clone(),
                FeatureStats {
                    mean,
                    std_dev,
                    min,
                    max,
                    count,
                },
            );
        }

        stats_map
    }

    /// Returns true if the detector has been trained.
    pub fn is_trained(&self) -> bool {
        self.trained
    }

    /// Gets the configuration.
    pub fn config(&self) -> &MLAnomalyConfig {
        &self.config
    }
}

impl Default for MLAnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn create_test_record(hour: u32, is_override: bool) -> AuditRecord {
        let mut timestamp = Utc::now();
        // Set specific hour
        timestamp = timestamp
            .date_naive()
            .and_hms_opt(hour, 0, 0)
            .unwrap()
            .and_utc();

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
    fn test_ml_anomaly_detector_creation() {
        let detector = MLAnomalyDetector::new();
        assert!(!detector.is_trained());
        assert_eq!(detector.config().sensitivity, 0.1);
    }

    #[test]
    fn test_ml_anomaly_detector_training() {
        let mut detector = MLAnomalyDetector::new();

        // Create training data
        let mut records = Vec::new();
        for i in 0..150 {
            records.push(create_test_record(9 + (i % 8), false));
        }

        let result = detector.train(&records);
        assert!(result.is_ok());
        assert!(detector.is_trained());
    }

    #[test]
    fn test_ml_anomaly_detector_insufficient_training() {
        let mut detector = MLAnomalyDetector::new();

        // Create insufficient training data
        let records: Vec<_> = (0..50)
            .map(|i| create_test_record(9 + (i % 8), false))
            .collect();

        let result = detector.train(&records);
        assert!(result.is_err());
    }

    #[test]
    fn test_ml_anomaly_detection() {
        // Use custom config with lower sensitivity threshold for easier detection
        let config = MLAnomalyConfig {
            sensitivity: 0.01, // Lower threshold to detect more anomalies
            ..Default::default()
        };
        let mut detector = MLAnomalyDetector::with_config(config);

        // Create normal training data (business hours, no overrides)
        let mut training_records = Vec::new();
        for i in 0..150 {
            training_records.push(create_test_record(9 + (i % 8) as u32, false));
        }

        detector.train(&training_records).unwrap();

        // Create test data with anomalies
        let test_records = vec![
            create_test_record(10, false), // Normal
            create_test_record(3, true),   // Anomalous: late night override
            create_test_record(11, false), // Normal
        ];

        let anomalies = detector.detect(&test_records).unwrap();

        // Should detect the late night override
        assert!(!anomalies.is_empty());
    }

    #[test]
    fn test_feature_extraction() {
        let record = create_test_record(10, true);
        let vectors = MLAnomalyDetector::extract_features(&[record]);

        assert_eq!(vectors.len(), 1);
        assert!(!vectors[0].features.is_empty());
        assert_eq!(vectors[0].features.len(), vectors[0].feature_names.len());
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        let distance = MLAnomalyDetector::euclidean_distance(&a, &b);
        assert!((distance - 5.0).abs() < 0.001);
    }
}
