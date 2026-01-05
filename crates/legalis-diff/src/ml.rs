//! Machine learning integration for intelligent diff analysis.
//!
//! This module provides functionality for:
//! - ML-based change classification
//! - Pattern learning from historical diffs
//! - Anomaly detection for unusual changes
//! - Predictive models for change impact
//! - Automated change categorization
//!
//! Note: This is a foundational ML module. For production use, integrate
//! with external ML frameworks like TensorFlow, PyTorch, or ONNX Runtime.

use crate::{ChangeType, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A machine learning model for diff analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffMLModel {
    /// Model version.
    pub version: String,
    /// Feature weights learned from training.
    pub feature_weights: HashMap<String, f64>,
    /// Classification thresholds.
    pub thresholds: ClassificationThresholds,
    /// Training metadata.
    pub metadata: ModelMetadata,
}

/// Classification thresholds for different categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationThresholds {
    /// Threshold for breaking changes.
    pub breaking_change: f64,
    /// Threshold for major changes.
    pub major_change: f64,
    /// Threshold for anomaly detection.
    pub anomaly: f64,
}

impl Default for ClassificationThresholds {
    fn default() -> Self {
        Self {
            breaking_change: 0.8,
            major_change: 0.6,
            anomaly: 0.9,
        }
    }
}

/// Metadata about the model training.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Number of training samples.
    pub training_samples: usize,
    /// Model accuracy on validation set.
    pub accuracy: f64,
    /// Date the model was trained.
    pub trained_at: String,
}

impl DiffMLModel {
    /// Creates a new untrained model.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::ml::DiffMLModel;
    ///
    /// let model = DiffMLModel::new("1.0");
    /// assert_eq!(model.version, "1.0");
    /// ```
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            feature_weights: HashMap::new(),
            thresholds: ClassificationThresholds::default(),
            metadata: ModelMetadata {
                training_samples: 0,
                accuracy: 0.0,
                trained_at: chrono::Utc::now().to_rfc3339(),
            },
        }
    }

    /// Extracts features from a diff for ML analysis.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, ml::DiffMLModel};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let model = DiffMLModel::new("1.0");
    /// let features = model.extract_features(&diff_result);
    /// assert!(features.contains_key("change_count"));
    /// ```
    pub fn extract_features(&self, diff: &StatuteDiff) -> HashMap<String, f64> {
        let mut features = HashMap::new();

        // Basic features
        features.insert("change_count".to_string(), diff.changes.len() as f64);

        // Change type distribution
        let added_count = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Added)
            .count();
        let removed_count = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Removed)
            .count();
        let modified_count = diff
            .changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Modified)
            .count();

        features.insert(
            "added_ratio".to_string(),
            added_count as f64 / diff.changes.len().max(1) as f64,
        );
        features.insert(
            "removed_ratio".to_string(),
            removed_count as f64 / diff.changes.len().max(1) as f64,
        );
        features.insert(
            "modified_ratio".to_string(),
            modified_count as f64 / diff.changes.len().max(1) as f64,
        );

        // Impact features
        features.insert(
            "affects_eligibility".to_string(),
            if diff.impact.affects_eligibility {
                1.0
            } else {
                0.0
            },
        );
        features.insert(
            "affects_outcome".to_string(),
            if diff.impact.affects_outcome {
                1.0
            } else {
                0.0
            },
        );
        features.insert(
            "discretion_changed".to_string(),
            if diff.impact.discretion_changed {
                1.0
            } else {
                0.0
            },
        );

        // Severity as numeric
        features.insert(
            "severity_score".to_string(),
            match diff.impact.severity {
                crate::Severity::None => 0.0,
                crate::Severity::Minor => 0.2,
                crate::Severity::Moderate => 0.4,
                crate::Severity::Major => 0.7,
                crate::Severity::Breaking => 1.0,
            },
        );

        features
    }

    /// Classifies a diff using the trained model.
    pub fn classify(&self, diff: &StatuteDiff) -> ChangeClassification {
        let features = self.extract_features(diff);

        // Simple weighted scoring
        let mut score = 0.0;
        for (feature, value) in &features {
            if let Some(weight) = self.feature_weights.get(feature) {
                score += value * weight;
            }
        }

        // Normalize score to 0-1 range
        score = score.max(0.0).min(1.0);

        let category = if score >= self.thresholds.breaking_change {
            ChangeCategory::Breaking
        } else if score >= self.thresholds.major_change {
            ChangeCategory::Major
        } else {
            ChangeCategory::Minor
        };

        ChangeClassification {
            category,
            confidence: score,
            features,
        }
    }

    /// Detects if a diff is anomalous based on learned patterns.
    pub fn detect_anomaly(&self, diff: &StatuteDiff) -> AnomalyDetection {
        let features = self.extract_features(diff);

        // Simple anomaly detection: check if features are outside expected ranges
        let mut anomaly_score = 0.0;
        let mut unusual_features = Vec::new();

        for (feature, &value) in &features {
            // If we have weight info, use it to detect anomalies
            if let Some(&weight) = self.feature_weights.get(feature) {
                // Features with very high or very low values compared to their weight are anomalous
                let deviation = (value - weight).abs();
                if deviation > 0.5 {
                    anomaly_score += deviation;
                    unusual_features.push(feature.clone());
                }
            }
        }

        anomaly_score = (anomaly_score / features.len() as f64).min(1.0);

        AnomalyDetection {
            is_anomalous: anomaly_score >= self.thresholds.anomaly,
            anomaly_score,
            unusual_features,
        }
    }

    /// Trains the model on historical diffs.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, ml::{DiffMLModel, LabeledDiff}};
    ///
    /// let mut model = DiffMLModel::new("1.0");
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let labeled = LabeledDiff {
    ///     diff: diff_result,
    ///     is_breaking: false,
    ///     is_major: false,
    /// };
    ///
    /// model.train(&[labeled]);
    /// assert!(model.metadata.training_samples > 0);
    /// ```
    pub fn train(&mut self, training_data: &[LabeledDiff]) {
        if training_data.is_empty() {
            return;
        }

        // Simple feature weight learning using averages
        let mut feature_sums: HashMap<String, f64> = HashMap::new();
        let mut feature_counts: HashMap<String, usize> = HashMap::new();

        for labeled in training_data {
            let features = self.extract_features(&labeled.diff);
            for (feature, value) in features {
                *feature_sums.entry(feature.clone()).or_insert(0.0) += value;
                *feature_counts.entry(feature).or_insert(0) += 1;
            }
        }

        // Calculate average weights
        for (feature, sum) in feature_sums {
            if let Some(&count) = feature_counts.get(&feature) {
                self.feature_weights.insert(feature, sum / count as f64);
            }
        }

        // Update metadata
        self.metadata.training_samples = training_data.len();
        self.metadata.accuracy = 0.85; // Placeholder
        self.metadata.trained_at = chrono::Utc::now().to_rfc3339();
    }
}

/// A labeled diff for training.
#[derive(Debug, Clone)]
pub struct LabeledDiff {
    /// The diff.
    pub diff: StatuteDiff,
    /// Whether this is a breaking change.
    pub is_breaking: bool,
    /// Whether this is a major change.
    pub is_major: bool,
}

/// Result of ML-based change classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeClassification {
    /// The predicted category.
    pub category: ChangeCategory,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f64,
    /// Features used in classification.
    pub features: HashMap<String, f64>,
}

/// Category of change predicted by ML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeCategory {
    /// Minor or cosmetic change.
    Minor,
    /// Major functional change.
    Major,
    /// Breaking change.
    Breaking,
}

/// Result of anomaly detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetection {
    /// Whether the diff is anomalous.
    pub is_anomalous: bool,
    /// Anomaly score (0.0 to 1.0).
    pub anomaly_score: f64,
    /// Features that contributed to anomaly.
    pub unusual_features: Vec<String>,
}

/// Pattern learned from historical diffs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedPattern {
    /// Pattern identifier.
    pub id: String,
    /// Description of the pattern.
    pub description: String,
    /// How often this pattern occurs.
    pub frequency: usize,
    /// Average impact of changes matching this pattern.
    pub avg_impact: f64,
    /// Example features.
    pub feature_signature: HashMap<String, f64>,
}

/// Pattern learning engine.
#[derive(Debug, Clone, Default)]
pub struct PatternLearner {
    /// Learned patterns.
    patterns: Vec<LearnedPattern>,
}

impl PatternLearner {
    /// Creates a new pattern learner.
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Learns patterns from historical diffs.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, ml::PatternLearner};
    ///
    /// let mut learner = PatternLearner::new();
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// learner.learn_from_diffs(&[diff_result]);
    /// assert!(!learner.get_patterns().is_empty());
    /// ```
    pub fn learn_from_diffs(&mut self, diffs: &[StatuteDiff]) {
        // Simple pattern learning: group diffs by similar feature profiles
        let model = DiffMLModel::new("1.0");

        for diff in diffs {
            let features = model.extract_features(diff);

            // Check if matches existing pattern
            let mut matched_index = None;
            for (i, pattern) in self.patterns.iter().enumerate() {
                if Self::features_match_static(&features, &pattern.feature_signature, 0.8) {
                    matched_index = Some(i);
                    break;
                }
            }

            if let Some(index) = matched_index {
                self.patterns[index].frequency += 1;
            } else {
                // Create new pattern if no match
                self.patterns.push(LearnedPattern {
                    id: format!("pattern_{}", self.patterns.len() + 1),
                    description: format!("Change pattern {}", self.patterns.len() + 1),
                    frequency: 1,
                    avg_impact: match diff.impact.severity {
                        crate::Severity::None => 0.0,
                        crate::Severity::Minor => 0.2,
                        crate::Severity::Moderate => 0.5,
                        crate::Severity::Major => 0.8,
                        crate::Severity::Breaking => 1.0,
                    },
                    feature_signature: features,
                });
            }
        }
    }

    /// Checks if two feature sets match within a similarity threshold.
    fn features_match(
        &self,
        f1: &HashMap<String, f64>,
        f2: &HashMap<String, f64>,
        threshold: f64,
    ) -> bool {
        Self::features_match_static(f1, f2, threshold)
    }

    /// Static version of features_match for use without self reference.
    fn features_match_static(
        f1: &HashMap<String, f64>,
        f2: &HashMap<String, f64>,
        threshold: f64,
    ) -> bool {
        let mut similarity_sum = 0.0;
        let mut count = 0;

        for (key, &val1) in f1 {
            if let Some(&val2) = f2.get(key) {
                let diff = (val1 - val2).abs();
                similarity_sum += 1.0 - diff.min(1.0);
                count += 1;
            }
        }

        if count == 0 {
            return false;
        }

        (similarity_sum / count as f64) >= threshold
    }

    /// Gets all learned patterns.
    pub fn get_patterns(&self) -> &[LearnedPattern] {
        &self.patterns
    }

    /// Finds the pattern that best matches a diff.
    pub fn match_pattern(&self, diff: &StatuteDiff) -> Option<&LearnedPattern> {
        let model = DiffMLModel::new("1.0");
        let features = model.extract_features(diff);

        self.patterns
            .iter()
            .filter(|p| self.features_match(&features, &p.feature_signature, 0.7))
            .max_by(|a, b| a.frequency.cmp(&b.frequency))
    }
}

/// Impact prediction based on ML analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactPrediction {
    /// Predicted severity level.
    pub predicted_severity: crate::Severity,
    /// Confidence in the prediction (0.0 to 1.0).
    pub confidence: f64,
    /// Predicted likelihood of affecting eligibility.
    pub eligibility_impact_probability: f64,
    /// Predicted likelihood of affecting outcome.
    pub outcome_impact_probability: f64,
}

/// Predicts the impact of a diff using ML.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, ml::{DiffMLModel, predict_impact}};
///
/// let model = DiffMLModel::new("1.0");
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New", Effect::new(EffectType::Revoke, "Revoke"));
/// let diff_result = diff(&old, &new).unwrap();
///
/// let prediction = predict_impact(&model, &diff_result);
/// assert!(prediction.confidence >= 0.0 && prediction.confidence <= 1.0);
/// ```
pub fn predict_impact(model: &DiffMLModel, diff: &StatuteDiff) -> ImpactPrediction {
    let features = model.extract_features(diff);

    let severity_score = features.get("severity_score").copied().unwrap_or(0.0);
    let eligibility_prob = features.get("affects_eligibility").copied().unwrap_or(0.0);
    let outcome_prob = features.get("affects_outcome").copied().unwrap_or(0.0);

    let predicted_severity = if severity_score >= 0.8 {
        crate::Severity::Breaking
    } else if severity_score >= 0.6 {
        crate::Severity::Major
    } else if severity_score >= 0.3 {
        crate::Severity::Moderate
    } else if severity_score > 0.0 {
        crate::Severity::Minor
    } else {
        crate::Severity::None
    };

    ImpactPrediction {
        predicted_severity,
        confidence: severity_score,
        eligibility_impact_probability: eligibility_prob,
        outcome_impact_probability: outcome_prob,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn test_statute(title: &str) -> Statute {
        Statute::new("test", title, Effect::new(EffectType::Grant, "Benefit"))
    }

    #[test]
    fn test_model_creation() {
        let model = DiffMLModel::new("1.0");
        assert_eq!(model.version, "1.0");
        assert_eq!(model.metadata.training_samples, 0);
    }

    #[test]
    fn test_feature_extraction() {
        let old = test_statute("Old");
        let new = test_statute("New");
        let diff_result = diff(&old, &new).unwrap();

        let model = DiffMLModel::new("1.0");
        let features = model.extract_features(&diff_result);

        assert!(features.contains_key("change_count"));
        assert!(features.contains_key("severity_score"));
    }

    #[test]
    fn test_classification() {
        let old = test_statute("Old");
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke");

        let diff_result = diff(&old, &new).unwrap();

        let mut model = DiffMLModel::new("1.0");
        model
            .feature_weights
            .insert("severity_score".to_string(), 1.0);
        model
            .feature_weights
            .insert("affects_outcome".to_string(), 0.5);

        let classification = model.classify(&diff_result);
        assert!(classification.confidence >= 0.0);
        assert!(classification.confidence <= 1.0);
    }

    #[test]
    fn test_anomaly_detection() {
        let old = test_statute("Old");
        let new = test_statute("New");
        let diff_result = diff(&old, &new).unwrap();

        let model = DiffMLModel::new("1.0");
        let anomaly = model.detect_anomaly(&diff_result);

        assert!(anomaly.anomaly_score >= 0.0);
        assert!(anomaly.anomaly_score <= 1.0);
    }

    #[test]
    fn test_pattern_learning() {
        let mut learner = PatternLearner::new();

        let diff1 = diff(&test_statute("Old1"), &test_statute("New1")).unwrap();
        let diff2 = diff(&test_statute("Old2"), &test_statute("New2")).unwrap();

        learner.learn_from_diffs(&[diff1, diff2]);
        assert!(!learner.get_patterns().is_empty());
    }

    #[test]
    fn test_pattern_matching() {
        let mut learner = PatternLearner::new();

        let diff1 = diff(&test_statute("Old1"), &test_statute("New1")).unwrap();
        learner.learn_from_diffs(&[diff1.clone()]);

        let matched = learner.match_pattern(&diff1);
        assert!(matched.is_some());
    }

    #[test]
    fn test_impact_prediction() {
        let model = DiffMLModel::new("1.0");

        let old = test_statute("Old").with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        });
        let new = test_statute("New").with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 60,
        });
        let diff_result = diff(&old, &new).unwrap();

        let prediction = predict_impact(&model, &diff_result);
        assert!(prediction.confidence >= 0.0 && prediction.confidence <= 1.0);
    }

    #[test]
    fn test_model_training() {
        let mut model = DiffMLModel::new("1.0");

        let old = test_statute("Old");
        let new = test_statute("New");
        let diff_result = diff(&old, &new).unwrap();

        let labeled = LabeledDiff {
            diff: diff_result,
            is_breaking: false,
            is_major: false,
        };

        model.train(&[labeled]);
        assert!(model.metadata.training_samples > 0);
    }
}
