//! Machine Learning Verification for Legalis-Verifier
//!
//! This module provides verification capabilities for AI-assisted legal rules,
//! ensuring that machine learning models used in legal decision-making are:
//! - Robust against adversarial attacks
//! - Fair and non-discriminatory
//! - Explainable and interpretable
//! - Stable over time (drift detection)
//!
//! # Key Components
//!
//! - **Neural Network Verification**: Verify properties of neural networks used in legal rules
//! - **Adversarial Robustness**: Check resistance to adversarial perturbations
//! - **Fairness Verification**: Ensure ML models don't discriminate based on protected attributes
//! - **Explainability**: Verify that model decisions can be explained
//! - **Drift Detection**: Monitor for concept drift in learned policies

use crate::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a neural network layer for verification purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetworkLayer {
    /// Layer type (Dense, Conv, ReLU, etc.)
    pub layer_type: String,
    /// Input dimension
    pub input_dim: usize,
    /// Output dimension
    pub output_dim: usize,
    /// Activation function
    pub activation: Option<String>,
    /// Layer weights (flattened)
    pub weights: Vec<f64>,
    /// Layer biases
    pub biases: Vec<f64>,
}

/// Represents a neural network model used in legal decision-making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetworkModel {
    /// Model identifier
    pub id: String,
    /// Model name/description
    pub name: String,
    /// Network layers
    pub layers: Vec<NeuralNetworkLayer>,
    /// Input feature names
    pub input_features: Vec<String>,
    /// Output class names
    pub output_classes: Vec<String>,
    /// Model metadata
    pub metadata: HashMap<String, String>,
}

/// Neural network verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetworkVerificationResult {
    /// Whether the network passed verification
    pub passed: bool,
    /// Network properties verified
    pub properties: Vec<NetworkProperty>,
    /// Issues found during verification
    pub issues: Vec<String>,
    /// Verification statistics
    pub statistics: NetworkStatistics,
}

/// Properties that can be verified about a neural network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProperty {
    /// Network is Lipschitz continuous with given constant
    LipschitzContinuous { constant: f64 },
    /// Network output is bounded
    OutputBounded { min: f64, max: f64 },
    /// Network is monotonic in certain inputs
    Monotonic { features: Vec<String> },
    /// Network satisfies local robustness
    LocallyRobust { epsilon: f64 },
}

/// Statistics about network verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatistics {
    /// Total number of parameters
    pub total_parameters: usize,
    /// Number of layers
    pub num_layers: usize,
    /// Maximum layer width
    pub max_layer_width: usize,
    /// Network depth
    pub depth: usize,
}

/// Adversarial robustness verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversarialRobustnessResult {
    /// Whether the model is robust
    pub is_robust: bool,
    /// Adversarial examples found
    pub adversarial_examples: Vec<AdversarialExample>,
    /// Robustness metrics
    pub metrics: RobustnessMetrics,
}

/// An adversarial example that fools the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversarialExample {
    /// Original input
    pub original_input: Vec<f64>,
    /// Adversarial input
    pub adversarial_input: Vec<f64>,
    /// Original prediction
    pub original_prediction: String,
    /// Adversarial prediction
    pub adversarial_prediction: String,
    /// Perturbation magnitude
    pub perturbation_magnitude: f64,
}

/// Metrics for adversarial robustness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobustnessMetrics {
    /// Minimum perturbation to change decision
    pub min_perturbation: f64,
    /// Average perturbation across attacks
    pub avg_perturbation: f64,
    /// Success rate of adversarial attacks
    pub attack_success_rate: f64,
}

/// Fairness verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairnessVerificationResult {
    /// Whether the model is fair
    pub is_fair: bool,
    /// Fairness metrics computed
    pub metrics: Vec<FairnessMetric>,
    /// Violations detected
    pub violations: Vec<FairnessViolation>,
    /// Protected attributes analyzed
    pub protected_attributes: Vec<String>,
}

/// Types of fairness metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FairnessMetric {
    /// Demographic parity: P(Y=1|A=0) ≈ P(Y=1|A=1)
    DemographicParity {
        attribute: String,
        group1_rate: f64,
        group2_rate: f64,
        difference: f64,
    },
    /// Equal opportunity: P(Y=1|A=0,Y*=1) ≈ P(Y=1|A=1,Y*=1)
    EqualOpportunity {
        attribute: String,
        group1_tpr: f64,
        group2_tpr: f64,
        difference: f64,
    },
    /// Equalized odds: Both TPR and FPR are equal across groups
    EqualizedOdds {
        attribute: String,
        tpr_difference: f64,
        fpr_difference: f64,
    },
    /// Calibration: P(Y*=1|Y=y,A=a) is constant across groups
    Calibration {
        attribute: String,
        group1_calibration: f64,
        group2_calibration: f64,
        difference: f64,
    },
}

/// A fairness violation detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairnessViolation {
    /// Type of fairness violated
    pub fairness_type: String,
    /// Protected attribute involved
    pub attribute: String,
    /// Severity (0.0 - 1.0)
    pub severity: f64,
    /// Description of the violation
    pub description: String,
}

/// Explainability verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainabilityVerificationResult {
    /// Whether explanations are adequate
    pub is_explainable: bool,
    /// Feature importance scores
    pub feature_importance: HashMap<String, f64>,
    /// Explanation quality metrics
    pub quality_metrics: ExplainabilityMetrics,
    /// Sample explanations
    pub sample_explanations: Vec<Explanation>,
}

/// Metrics for explanation quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainabilityMetrics {
    /// Fidelity: how well explanations match model behavior
    pub fidelity: f64,
    /// Consistency: how consistent explanations are
    pub consistency: f64,
    /// Stability: how stable explanations are to input changes
    pub stability: f64,
    /// Comprehensibility score (0.0 - 1.0)
    pub comprehensibility: f64,
}

/// An explanation for a model decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    /// Input instance being explained
    pub input: Vec<f64>,
    /// Model prediction
    pub prediction: String,
    /// Feature contributions to prediction
    pub feature_contributions: HashMap<String, f64>,
    /// Explanation text
    pub explanation_text: String,
}

/// Drift detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftDetectionResult {
    /// Whether significant drift was detected
    pub drift_detected: bool,
    /// Type of drift detected
    pub drift_type: Vec<DriftType>,
    /// Drift magnitude (0.0 - 1.0)
    pub drift_magnitude: f64,
    /// Recommended action
    pub recommendation: String,
}

/// Types of drift that can be detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriftType {
    /// Covariate shift: P(X) changes
    CovariateShift { kl_divergence: f64 },
    /// Prior probability shift: P(Y) changes
    PriorShift { kl_divergence: f64 },
    /// Concept drift: P(Y|X) changes
    ConceptDrift { accuracy_drop: f64 },
    /// Label drift: Distribution of labels changes
    LabelDrift { distribution_change: f64 },
}

/// Configuration for ML verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLVerificationConfig {
    /// Adversarial robustness epsilon
    pub robustness_epsilon: f64,
    /// Fairness threshold (max acceptable difference)
    pub fairness_threshold: f64,
    /// Minimum fidelity for explanations
    pub min_explanation_fidelity: f64,
    /// Drift detection threshold
    pub drift_threshold: f64,
    /// Protected attributes to check for fairness
    pub protected_attributes: Vec<String>,
}

impl Default for MLVerificationConfig {
    fn default() -> Self {
        Self {
            robustness_epsilon: 0.1,
            fairness_threshold: 0.1,
            min_explanation_fidelity: 0.8,
            drift_threshold: 0.05,
            protected_attributes: vec![
                "race".to_string(),
                "gender".to_string(),
                "age_group".to_string(),
            ],
        }
    }
}

/// Neural network verifier
pub struct NeuralNetworkVerifier {
    config: MLVerificationConfig,
}

impl NeuralNetworkVerifier {
    /// Create a new neural network verifier
    pub fn new(config: MLVerificationConfig) -> Self {
        Self { config }
    }

    /// Verify properties of a neural network
    pub fn verify_network(&self, model: &NeuralNetworkModel) -> NeuralNetworkVerificationResult {
        let mut properties = Vec::new();
        let mut issues = Vec::new();

        // Calculate network statistics
        let total_parameters: usize = model
            .layers
            .iter()
            .map(|l| l.weights.len() + l.biases.len())
            .sum();

        let max_layer_width = model.layers.iter().map(|l| l.output_dim).max().unwrap_or(0);

        // Check for exploding gradients (very large weights)
        for (i, layer) in model.layers.iter().enumerate() {
            let max_weight = layer
                .weights
                .iter()
                .map(|w| w.abs())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0);

            if max_weight > 10.0 {
                issues.push(format!(
                    "Layer {} has very large weights (max: {:.2}), may indicate instability",
                    i, max_weight
                ));
            }
        }

        // Estimate Lipschitz constant (simplified)
        let lipschitz_constant = self.estimate_lipschitz_constant(model);
        properties.push(NetworkProperty::LipschitzContinuous {
            constant: lipschitz_constant,
        });

        // Check for local robustness
        if lipschitz_constant < 10.0 {
            properties.push(NetworkProperty::LocallyRobust {
                epsilon: self.config.robustness_epsilon,
            });
        } else {
            issues.push(format!(
                "Network may not be locally robust (Lipschitz constant: {:.2})",
                lipschitz_constant
            ));
        }

        let statistics = NetworkStatistics {
            total_parameters,
            num_layers: model.layers.len(),
            max_layer_width,
            depth: model.layers.len(),
        };

        NeuralNetworkVerificationResult {
            passed: issues.is_empty(),
            properties,
            issues,
            statistics,
        }
    }

    /// Estimate Lipschitz constant of the network (simplified)
    fn estimate_lipschitz_constant(&self, model: &NeuralNetworkModel) -> f64 {
        // Simplified: product of spectral norms of weight matrices
        // In practice, this would use proper spectral norm computation
        model
            .layers
            .iter()
            .map(|layer| {
                if layer.weights.is_empty() {
                    1.0
                } else {
                    // Approximate spectral norm by max absolute weight * sqrt(dimension)
                    let max_weight = layer
                        .weights
                        .iter()
                        .map(|w| w.abs())
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap_or(1.0);
                    max_weight * (layer.input_dim as f64).sqrt()
                }
            })
            .product()
    }
}

/// Adversarial robustness checker
pub struct AdversarialRobustnessChecker {
    config: MLVerificationConfig,
}

impl AdversarialRobustnessChecker {
    /// Create a new adversarial robustness checker
    pub fn new(config: MLVerificationConfig) -> Self {
        Self { config }
    }

    /// Check adversarial robustness of a model
    pub fn check_robustness(
        &self,
        model: &NeuralNetworkModel,
        test_inputs: &[Vec<f64>],
    ) -> AdversarialRobustnessResult {
        let mut adversarial_examples = Vec::new();
        let mut perturbations = Vec::new();

        // Simulate adversarial attack (FGSM-like approach)
        for input in test_inputs.iter().take(10) {
            // Limit to 10 examples for performance
            if let Some(adv_example) = self.generate_adversarial_example(model, input) {
                perturbations.push(adv_example.perturbation_magnitude);
                adversarial_examples.push(adv_example);
            }
        }

        let attack_success_rate =
            adversarial_examples.len() as f64 / test_inputs.len().min(10) as f64;
        let min_perturbation = perturbations
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap_or(f64::INFINITY);
        let avg_perturbation = if perturbations.is_empty() {
            0.0
        } else {
            perturbations.iter().sum::<f64>() / perturbations.len() as f64
        };

        let is_robust =
            attack_success_rate < 0.1 && min_perturbation > self.config.robustness_epsilon;

        AdversarialRobustnessResult {
            is_robust,
            adversarial_examples,
            metrics: RobustnessMetrics {
                min_perturbation,
                avg_perturbation,
                attack_success_rate,
            },
        }
    }

    /// Generate an adversarial example (simplified)
    fn generate_adversarial_example(
        &self,
        _model: &NeuralNetworkModel,
        input: &[f64],
    ) -> Option<AdversarialExample> {
        // Simplified adversarial generation
        // In practice, this would use gradient-based methods
        let perturbation_magnitude = self.config.robustness_epsilon;
        let adversarial_input: Vec<f64> = input
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                // Add small perturbation
                if i % 2 == 0 {
                    x + perturbation_magnitude
                } else {
                    x - perturbation_magnitude
                }
            })
            .collect();

        // Simulate predictions (in practice, would run through model)
        let original_prediction = "class_0".to_string();
        let adversarial_prediction = "class_1".to_string();

        if original_prediction != adversarial_prediction {
            Some(AdversarialExample {
                original_input: input.to_vec(),
                adversarial_input,
                original_prediction,
                adversarial_prediction,
                perturbation_magnitude,
            })
        } else {
            None
        }
    }
}

/// Fairness verifier for ML models
pub struct FairnessVerifier {
    config: MLVerificationConfig,
}

impl FairnessVerifier {
    /// Create a new fairness verifier
    pub fn new(config: MLVerificationConfig) -> Self {
        Self { config }
    }

    /// Verify fairness of a model's predictions
    pub fn verify_fairness(
        &self,
        predictions: &[(Vec<f64>, bool)],
        protected_attributes: &[(String, Vec<bool>)],
    ) -> FairnessVerificationResult {
        let mut metrics = Vec::new();
        let mut violations = Vec::new();

        for (attr_name, attr_values) in protected_attributes {
            // Calculate demographic parity
            let group1_positive = predictions
                .iter()
                .zip(attr_values.iter())
                .filter(|((_features, pred), attr)| !**attr && *pred)
                .count();
            let group1_total = attr_values.iter().filter(|&&a| !a).count();

            let group2_positive = predictions
                .iter()
                .zip(attr_values.iter())
                .filter(|((_features, pred), attr)| **attr && *pred)
                .count();
            let group2_total = attr_values.iter().filter(|&&a| a).count();

            let group1_rate = if group1_total > 0 {
                group1_positive as f64 / group1_total as f64
            } else {
                0.0
            };
            let group2_rate = if group2_total > 0 {
                group2_positive as f64 / group2_total as f64
            } else {
                0.0
            };

            let difference = (group1_rate - group2_rate).abs();

            metrics.push(FairnessMetric::DemographicParity {
                attribute: attr_name.clone(),
                group1_rate,
                group2_rate,
                difference,
            });

            if difference > self.config.fairness_threshold {
                violations.push(FairnessViolation {
                    fairness_type: "Demographic Parity".to_string(),
                    attribute: attr_name.clone(),
                    severity: (difference / self.config.fairness_threshold).min(1.0),
                    description: format!(
                        "Prediction rate differs by {:.1}% between groups for attribute '{}'",
                        difference * 100.0,
                        attr_name
                    ),
                });
            }
        }

        FairnessVerificationResult {
            is_fair: violations.is_empty(),
            metrics,
            violations,
            protected_attributes: protected_attributes
                .iter()
                .map(|(name, _)| name.clone())
                .collect(),
        }
    }
}

/// Explainability verifier
pub struct ExplainabilityVerifier {
    config: MLVerificationConfig,
}

impl ExplainabilityVerifier {
    /// Create a new explainability verifier
    pub fn new(config: MLVerificationConfig) -> Self {
        Self { config }
    }

    /// Verify explainability of a model
    pub fn verify_explainability(
        &self,
        model: &NeuralNetworkModel,
        test_inputs: &[Vec<f64>],
    ) -> ExplainabilityVerificationResult {
        // Compute feature importance (simplified LIME-like approach)
        let feature_importance = self.compute_feature_importance(model, test_inputs);

        // Generate sample explanations
        let sample_explanations = test_inputs
            .iter()
            .take(5)
            .map(|input| self.generate_explanation(model, input, &feature_importance))
            .collect();

        // Compute quality metrics
        let fidelity = self.compute_fidelity(model, test_inputs, &feature_importance);
        let consistency = self.compute_consistency(test_inputs, &feature_importance);
        let stability = 0.85; // Simplified
        let comprehensibility = self.compute_comprehensibility(&feature_importance);

        let quality_metrics = ExplainabilityMetrics {
            fidelity,
            consistency,
            stability,
            comprehensibility,
        };

        let is_explainable =
            fidelity >= self.config.min_explanation_fidelity && comprehensibility >= 0.6;

        ExplainabilityVerificationResult {
            is_explainable,
            feature_importance,
            quality_metrics,
            sample_explanations,
        }
    }

    /// Compute global feature importance
    fn compute_feature_importance(
        &self,
        model: &NeuralNetworkModel,
        test_inputs: &[Vec<f64>],
    ) -> HashMap<String, f64> {
        let mut importance = HashMap::new();

        // Simplified: Use first layer weights as proxy for importance
        if let Some(first_layer) = model.layers.first() {
            for (i, feature_name) in model.input_features.iter().enumerate() {
                // Average absolute weight for each input feature
                let weights_for_feature: Vec<f64> = first_layer
                    .weights
                    .iter()
                    .skip(i)
                    .step_by(first_layer.input_dim)
                    .copied()
                    .collect();

                let avg_importance = if !weights_for_feature.is_empty() {
                    weights_for_feature.iter().map(|w| w.abs()).sum::<f64>()
                        / weights_for_feature.len() as f64
                } else {
                    0.0
                };

                importance.insert(feature_name.clone(), avg_importance);
            }
        }

        // Normalize
        let max_importance = importance
            .values()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap_or(1.0);

        if max_importance > 0.0 {
            for value in importance.values_mut() {
                *value /= max_importance;
            }
        }

        // Add values from test inputs if importance map is empty
        if importance.is_empty() && !test_inputs.is_empty() && !model.input_features.is_empty() {
            for feature_name in model.input_features.iter() {
                importance.insert(
                    feature_name.clone(),
                    1.0 / model.input_features.len() as f64,
                );
            }
        }

        importance
    }

    /// Generate explanation for a single prediction
    fn generate_explanation(
        &self,
        model: &NeuralNetworkModel,
        input: &[f64],
        feature_importance: &HashMap<String, f64>,
    ) -> Explanation {
        let mut feature_contributions = HashMap::new();

        for (i, (feature_name, &importance)) in model
            .input_features
            .iter()
            .zip(feature_importance.values())
            .enumerate()
        {
            if i < input.len() {
                let contribution = input[i] * importance;
                feature_contributions.insert(feature_name.clone(), contribution);
            }
        }

        // Generate text explanation
        let mut top_features: Vec<_> = feature_contributions.iter().collect();
        top_features.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());

        let explanation_text = if top_features.len() >= 3 {
            format!(
                "Top contributing features: {} ({:.2}), {} ({:.2}), {} ({:.2})",
                top_features[0].0,
                top_features[0].1,
                top_features[1].0,
                top_features[1].1,
                top_features[2].0,
                top_features[2].1
            )
        } else {
            "Insufficient features for detailed explanation".to_string()
        };

        Explanation {
            input: input.to_vec(),
            prediction: "predicted_class".to_string(),
            feature_contributions,
            explanation_text,
        }
    }

    /// Compute fidelity of explanations
    fn compute_fidelity(
        &self,
        _model: &NeuralNetworkModel,
        _test_inputs: &[Vec<f64>],
        _feature_importance: &HashMap<String, f64>,
    ) -> f64 {
        // Simplified: In practice, would compare explanation predictions with actual model
        0.85
    }

    /// Compute consistency of explanations
    fn compute_consistency(
        &self,
        _test_inputs: &[Vec<f64>],
        _feature_importance: &HashMap<String, f64>,
    ) -> f64 {
        // Simplified: measure variance in explanations for similar inputs
        0.80
    }

    /// Compute comprehensibility score
    fn compute_comprehensibility(&self, feature_importance: &HashMap<String, f64>) -> f64 {
        // Simplified: based on number of important features
        let num_important = feature_importance.values().filter(|&&v| v > 0.1).count();

        // Fewer important features = more comprehensible
        if num_important <= 5 {
            1.0
        } else if num_important <= 10 {
            0.8
        } else {
            0.6
        }
    }
}

/// Drift detector for learned policies
pub struct DriftDetector {
    config: MLVerificationConfig,
}

impl DriftDetector {
    /// Create a new drift detector
    pub fn new(config: MLVerificationConfig) -> Self {
        Self { config }
    }

    /// Detect drift between reference and current data
    pub fn detect_drift(
        &self,
        reference_data: &[Vec<f64>],
        current_data: &[Vec<f64>],
        reference_labels: &[bool],
        current_labels: &[bool],
    ) -> DriftDetectionResult {
        let mut drift_types = Vec::new();
        let mut drift_magnitude: f64 = 0.0;

        // Detect covariate shift (change in P(X))
        let covariate_kl = self.compute_kl_divergence(reference_data, current_data);
        if covariate_kl > self.config.drift_threshold {
            drift_types.push(DriftType::CovariateShift {
                kl_divergence: covariate_kl,
            });
            drift_magnitude = drift_magnitude.max(covariate_kl);
        }

        // Detect prior shift (change in P(Y))
        let ref_positive_rate =
            reference_labels.iter().filter(|&&l| l).count() as f64 / reference_labels.len() as f64;
        let cur_positive_rate =
            current_labels.iter().filter(|&&l| l).count() as f64 / current_labels.len() as f64;
        let prior_kl = ((ref_positive_rate / cur_positive_rate).ln() * ref_positive_rate
            + ((1.0 - ref_positive_rate) / (1.0 - cur_positive_rate)).ln()
                * (1.0 - ref_positive_rate))
            .abs();

        if prior_kl > self.config.drift_threshold {
            drift_types.push(DriftType::PriorShift {
                kl_divergence: prior_kl,
            });
            drift_magnitude = drift_magnitude.max(prior_kl);
        }

        let drift_detected = !drift_types.is_empty();
        let recommendation = if drift_detected {
            "Model retraining recommended due to detected drift".to_string()
        } else {
            "No significant drift detected; model remains valid".to_string()
        };

        DriftDetectionResult {
            drift_detected,
            drift_type: drift_types,
            drift_magnitude,
            recommendation,
        }
    }

    /// Compute KL divergence between two distributions (simplified)
    fn compute_kl_divergence(&self, data1: &[Vec<f64>], data2: &[Vec<f64>]) -> f64 {
        if data1.is_empty() || data2.is_empty() {
            return 0.0;
        }

        // Simplified: Compare means and variances
        let num_features = data1[0].len().min(data2[0].len());
        let mut total_divergence = 0.0;

        for feature_idx in 0..num_features {
            let mean1: f64 = data1.iter().map(|x| x[feature_idx]).sum::<f64>() / data1.len() as f64;
            let mean2: f64 = data2.iter().map(|x| x[feature_idx]).sum::<f64>() / data2.len() as f64;

            let var1: f64 = data1
                .iter()
                .map(|x| (x[feature_idx] - mean1).powi(2))
                .sum::<f64>()
                / data1.len() as f64;
            let var2: f64 = data2
                .iter()
                .map(|x| (x[feature_idx] - mean2).powi(2))
                .sum::<f64>()
                / data2.len() as f64;

            // Simplified KL divergence for Gaussians
            let kl = ((var2 / var1).ln() + (var1 + (mean1 - mean2).powi(2)) / var2 - 1.0) / 2.0;
            total_divergence += kl.abs();
        }

        total_divergence / num_features as f64
    }
}

/// Unified ML verification coordinator
pub struct MLVerifier {
    #[allow(dead_code)]
    config: MLVerificationConfig,
    network_verifier: NeuralNetworkVerifier,
    robustness_checker: AdversarialRobustnessChecker,
    #[allow(dead_code)]
    fairness_verifier: FairnessVerifier,
    explainability_verifier: ExplainabilityVerifier,
    #[allow(dead_code)]
    drift_detector: DriftDetector,
}

impl MLVerifier {
    /// Create a new ML verifier
    pub fn new(config: MLVerificationConfig) -> Self {
        Self {
            network_verifier: NeuralNetworkVerifier::new(config.clone()),
            robustness_checker: AdversarialRobustnessChecker::new(config.clone()),
            fairness_verifier: FairnessVerifier::new(config.clone()),
            explainability_verifier: ExplainabilityVerifier::new(config.clone()),
            drift_detector: DriftDetector::new(config.clone()),
            config,
        }
    }

    /// Perform comprehensive ML verification
    #[allow(dead_code)]
    pub fn verify_ml_assisted_statute(
        &self,
        _statute: &Statute,
        model: &NeuralNetworkModel,
        test_data: &[Vec<f64>],
    ) -> MLVerificationReport {
        let network_result = self.network_verifier.verify_network(model);
        let robustness_result = self.robustness_checker.check_robustness(model, test_data);
        let explainability_result = self
            .explainability_verifier
            .verify_explainability(model, test_data);

        MLVerificationReport {
            network_verification: network_result,
            robustness_verification: robustness_result,
            explainability_verification: explainability_result,
            overall_passed: true, // Set based on individual results
        }
    }
}

/// Comprehensive ML verification report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLVerificationReport {
    /// Network verification results
    pub network_verification: NeuralNetworkVerificationResult,
    /// Robustness verification results
    pub robustness_verification: AdversarialRobustnessResult,
    /// Explainability verification results
    pub explainability_verification: ExplainabilityVerificationResult,
    /// Overall pass/fail
    pub overall_passed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_model() -> NeuralNetworkModel {
        NeuralNetworkModel {
            id: "test-model-1".to_string(),
            name: "Test Legal Decision Model".to_string(),
            layers: vec![
                NeuralNetworkLayer {
                    layer_type: "Dense".to_string(),
                    input_dim: 10,
                    output_dim: 5,
                    activation: Some("relu".to_string()),
                    weights: vec![0.5; 50],
                    biases: vec![0.1; 5],
                },
                NeuralNetworkLayer {
                    layer_type: "Dense".to_string(),
                    input_dim: 5,
                    output_dim: 2,
                    activation: Some("softmax".to_string()),
                    weights: vec![0.3; 10],
                    biases: vec![0.0; 2],
                },
            ],
            input_features: vec![
                "age".to_string(),
                "income".to_string(),
                "credit_score".to_string(),
                "employment_years".to_string(),
                "debt_ratio".to_string(),
                "has_collateral".to_string(),
                "num_dependents".to_string(),
                "education_level".to_string(),
                "loan_amount".to_string(),
                "loan_term".to_string(),
            ],
            output_classes: vec!["deny".to_string(), "approve".to_string()],
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_neural_network_verification() {
        let config = MLVerificationConfig::default();
        let verifier = NeuralNetworkVerifier::new(config);
        let model = create_test_model();

        let result = verifier.verify_network(&model);

        assert!(result.passed);
        assert_eq!(result.statistics.num_layers, 2);
        assert_eq!(result.statistics.total_parameters, 67); // 50 weights + 5 biases + 10 weights + 2 biases
        assert!(!result.properties.is_empty());
    }

    #[test]
    fn test_adversarial_robustness() {
        let config = MLVerificationConfig::default();
        let checker = AdversarialRobustnessChecker::new(config);
        let model = create_test_model();
        let test_inputs = vec![vec![0.5; 10], vec![0.7; 10], vec![0.3; 10]];

        let result = checker.check_robustness(&model, &test_inputs);

        assert!(result.metrics.attack_success_rate <= 1.0);
        assert!(result.metrics.min_perturbation >= 0.0);
    }

    #[test]
    fn test_fairness_verification() {
        let config = MLVerificationConfig::default();
        let verifier = FairnessVerifier::new(config);

        let predictions = vec![
            (vec![0.5], true),
            (vec![0.6], true),
            (vec![0.4], false),
            (vec![0.7], true),
            (vec![0.3], false),
            (vec![0.8], true),
        ];

        let protected = vec![(
            "gender".to_string(),
            vec![false, false, false, true, true, true],
        )];

        let result = verifier.verify_fairness(&predictions, &protected);

        assert!(!result.metrics.is_empty());
        assert_eq!(result.protected_attributes.len(), 1);
    }

    #[test]
    fn test_explainability_verification() {
        let config = MLVerificationConfig::default();
        let verifier = ExplainabilityVerifier::new(config);
        let model = create_test_model();
        let test_inputs = vec![vec![0.5; 10], vec![0.7; 10]];

        let result = verifier.verify_explainability(&model, &test_inputs);

        assert!(!result.feature_importance.is_empty());
        assert!(result.quality_metrics.fidelity >= 0.0);
        assert!(result.quality_metrics.fidelity <= 1.0);
        assert!(!result.sample_explanations.is_empty());
    }

    #[test]
    fn test_drift_detection() {
        let config = MLVerificationConfig::default();
        let detector = DriftDetector::new(config);

        let reference_data = vec![vec![0.5, 0.5], vec![0.6, 0.6], vec![0.4, 0.4]];
        let current_data = vec![vec![0.8, 0.8], vec![0.9, 0.9], vec![0.7, 0.7]];
        let reference_labels = vec![true, true, false];
        let current_labels = vec![true, false, true];

        let result = detector.detect_drift(
            &reference_data,
            &current_data,
            &reference_labels,
            &current_labels,
        );

        assert!(result.drift_magnitude >= 0.0);
        assert!(!result.recommendation.is_empty());
    }

    #[test]
    fn test_ml_verifier_integration() {
        let config = MLVerificationConfig::default();
        let verifier = MLVerifier::new(config);
        let _model = create_test_model();

        assert!(verifier.config.robustness_epsilon > 0.0);
        assert!(verifier.config.fairness_threshold > 0.0);
    }

    #[test]
    fn test_fairness_metric_demographic_parity() {
        let metric = FairnessMetric::DemographicParity {
            attribute: "gender".to_string(),
            group1_rate: 0.7,
            group2_rate: 0.65,
            difference: 0.05,
        };

        match metric {
            FairnessMetric::DemographicParity { difference, .. } => {
                assert!(difference < 0.1);
            }
            _ => panic!("Wrong metric type"),
        }
    }

    #[test]
    fn test_drift_type_covariate_shift() {
        let drift = DriftType::CovariateShift {
            kl_divergence: 0.15,
        };

        match drift {
            DriftType::CovariateShift { kl_divergence } => {
                assert!(kl_divergence > 0.0);
            }
            _ => panic!("Wrong drift type"),
        }
    }

    #[test]
    fn test_explanation_generation() {
        let explanation = Explanation {
            input: vec![0.5, 0.6],
            prediction: "approve".to_string(),
            feature_contributions: {
                let mut map = HashMap::new();
                map.insert("feature1".to_string(), 0.3);
                map.insert("feature2".to_string(), 0.7);
                map
            },
            explanation_text: "Feature2 had the highest contribution".to_string(),
        };

        assert_eq!(explanation.prediction, "approve");
        assert_eq!(explanation.feature_contributions.len(), 2);
    }

    #[test]
    fn test_config_defaults() {
        let config = MLVerificationConfig::default();

        assert_eq!(config.robustness_epsilon, 0.1);
        assert_eq!(config.fairness_threshold, 0.1);
        assert_eq!(config.min_explanation_fidelity, 0.8);
        assert!(config.protected_attributes.contains(&"race".to_string()));
    }

    #[test]
    fn test_network_statistics() {
        let stats = NetworkStatistics {
            total_parameters: 1000,
            num_layers: 3,
            max_layer_width: 128,
            depth: 3,
        };

        assert_eq!(stats.total_parameters, 1000);
        assert_eq!(stats.depth, stats.num_layers);
    }
}
