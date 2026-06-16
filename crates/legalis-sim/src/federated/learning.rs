//! Federated learning over per-organization local models.
//!
//! Each organization trains a [`LocalModel`] (a logistic-regression classifier
//! trained by stochastic gradient descent) on its own private data and emits a
//! [`ModelUpdate`]. A [`FederatedAveraging`] coordinator combines those updates
//! into a new global model using **FedAvg** (sample-weighted parameter
//! averaging), with an optional **DP-FedAvg** path that clips each update and
//! adds calibrated Gaussian noise to the averaged model.

use super::OrgId;
use super::differential_privacy::{GaussianMechanism, clip_l2_norm};
use crate::error::{SimResult, SimulationError};
use rand::RngExt;
use serde::{Deserialize, Serialize};

/// Numerically stable logistic sigmoid.
fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

/// Dot product of two equal-length slices.
fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Binary cross-entropy loss for a single prediction, with clamping for stability.
fn binary_cross_entropy(prediction: f64, label: f64) -> f64 {
    let p = prediction.clamp(1e-12, 1.0 - 1e-12);
    -(label * p.ln() + (1.0 - label) * (1.0 - p).ln())
}

/// A logistic-regression model trained locally by one organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModel {
    /// Identifier of the owning organization.
    pub org_id: OrgId,
    /// Feature weights.
    pub weights: Vec<f64>,
    /// Bias term.
    pub bias: f64,
    /// SGD learning rate.
    pub learning_rate: f64,
}

impl LocalModel {
    /// Creates a zero-initialised model with `num_features` weights.
    pub fn new(org_id: OrgId, num_features: usize, learning_rate: f64) -> SimResult<Self> {
        if num_features == 0 {
            return Err(SimulationError::InvalidParameter(
                "model must have at least one feature".to_string(),
            ));
        }
        if learning_rate <= 0.0 || !learning_rate.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "learning rate must be positive and finite".to_string(),
            ));
        }
        Ok(Self {
            org_id,
            weights: vec![0.0; num_features],
            bias: 0.0,
            learning_rate,
        })
    }

    /// Returns the number of input features.
    pub fn num_features(&self) -> usize {
        self.weights.len()
    }

    /// Predicts the probability of the positive class for a feature vector.
    pub fn predict(&self, features: &[f64]) -> SimResult<f64> {
        if features.len() != self.weights.len() {
            return Err(SimulationError::InvalidParameter(format!(
                "expected {} features, got {}",
                self.weights.len(),
                features.len()
            )));
        }
        Ok(sigmoid(dot(&self.weights, features) + self.bias))
    }

    /// Runs one SGD epoch over `data`, returning the mean cross-entropy loss.
    pub fn train_epoch(&mut self, data: &[(Vec<f64>, f64)]) -> SimResult<f64> {
        if data.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "training data cannot be empty".to_string(),
            ));
        }
        let lr = self.learning_rate;
        let mut total_loss = 0.0;
        for (features, label) in data {
            if features.len() != self.weights.len() {
                return Err(SimulationError::InvalidParameter(format!(
                    "expected {} features, got {}",
                    self.weights.len(),
                    features.len()
                )));
            }
            let prediction = sigmoid(dot(&self.weights, features) + self.bias);
            let gradient = prediction - label;
            for (w, &x) in self.weights.iter_mut().zip(features.iter()) {
                *w -= lr * gradient * x;
            }
            self.bias -= lr * gradient;
            total_loss += binary_cross_entropy(prediction, *label);
        }
        Ok(total_loss / data.len() as f64)
    }

    /// Runs `epochs` SGD epochs, returning the final mean loss.
    pub fn train(&mut self, data: &[(Vec<f64>, f64)], epochs: usize) -> SimResult<f64> {
        if epochs == 0 {
            return Err(SimulationError::InvalidParameter(
                "epochs must be greater than zero".to_string(),
            ));
        }
        let mut last_loss = 0.0;
        for _ in 0..epochs {
            last_loss = self.train_epoch(data)?;
        }
        Ok(last_loss)
    }

    /// Computes the mean cross-entropy loss on `data` without updating weights.
    pub fn loss(&self, data: &[(Vec<f64>, f64)]) -> SimResult<f64> {
        if data.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "loss data cannot be empty".to_string(),
            ));
        }
        let mut total = 0.0;
        for (features, label) in data {
            total += binary_cross_entropy(self.predict(features)?, *label);
        }
        Ok(total / data.len() as f64)
    }

    /// Returns the flattened parameter vector `[weights..., bias]`.
    pub fn parameters(&self) -> Vec<f64> {
        let mut params = self.weights.clone();
        params.push(self.bias);
        params
    }

    /// Overwrites the model parameters from a flattened `[weights..., bias]` vector.
    pub fn set_parameters(&mut self, params: &[f64]) -> SimResult<()> {
        if params.len() != self.weights.len() + 1 {
            return Err(SimulationError::InvalidParameter(format!(
                "expected {} parameters, got {}",
                self.weights.len() + 1,
                params.len()
            )));
        }
        let (weights, bias) = params.split_at(self.weights.len());
        self.weights.copy_from_slice(weights);
        self.bias = bias[0];
        Ok(())
    }

    /// Builds a [`ModelUpdate`] from the current parameters and a sample count.
    pub fn to_update(&self, num_samples: usize) -> ModelUpdate {
        ModelUpdate {
            org_id: self.org_id.clone(),
            parameters: self.parameters(),
            num_samples,
        }
    }
}

/// A parameter update submitted by one organization for aggregation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUpdate {
    /// Identifier of the contributing organization.
    pub org_id: OrgId,
    /// Flattened model parameters `[weights..., bias]`.
    pub parameters: Vec<f64>,
    /// Number of local samples the update was trained on (FedAvg weight).
    pub num_samples: usize,
}

/// Configuration for the differentially-private FedAvg path (DP-FedAvg).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DpFedConfig {
    /// Per-update L2 clipping bound (sensitivity control).
    pub clip_norm: f64,
    /// Privacy-loss parameter `ε` charged per round.
    pub epsilon: f64,
    /// Failure-probability parameter `δ` (must be `> 0` for the Gaussian mechanism).
    pub delta: f64,
}

impl DpFedConfig {
    /// Creates a validated DP-FedAvg configuration.
    pub fn new(clip_norm: f64, epsilon: f64, delta: f64) -> SimResult<Self> {
        if clip_norm <= 0.0 || !clip_norm.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "clip_norm must be positive and finite".to_string(),
            ));
        }
        if delta <= 0.0 {
            return Err(SimulationError::InvalidParameter(
                "DP-FedAvg requires delta > 0".to_string(),
            ));
        }
        // Validate (epsilon, delta) jointly.
        GaussianMechanism::new(clip_norm, epsilon, delta)?;
        Ok(Self {
            clip_norm,
            epsilon,
            delta,
        })
    }
}

/// The FedAvg coordinator that maintains and updates the global model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedAveraging {
    /// Current global parameters `[weights..., bias]`.
    global_parameters: Vec<f64>,
    /// Number of parameters (`num_features + 1`).
    num_params: usize,
    /// Completed aggregation rounds.
    round: usize,
    /// Optional DP-FedAvg configuration.
    dp_config: Option<DpFedConfig>,
}

impl FederatedAveraging {
    /// Creates a coordinator for models with `num_params` parameters.
    pub fn new(num_params: usize) -> SimResult<Self> {
        if num_params == 0 {
            return Err(SimulationError::InvalidParameter(
                "num_params must be greater than zero".to_string(),
            ));
        }
        Ok(Self {
            global_parameters: vec![0.0; num_params],
            num_params,
            round: 0,
            dp_config: None,
        })
    }

    /// Enables the DP-FedAvg path with the given configuration.
    pub fn set_dp(&mut self, config: DpFedConfig) {
        self.dp_config = Some(config);
    }

    /// Disables the DP-FedAvg path (plain weighted FedAvg).
    pub fn clear_dp(&mut self) {
        self.dp_config = None;
    }

    /// Returns whether the DP-FedAvg path is enabled.
    pub fn dp_enabled(&self) -> bool {
        self.dp_config.is_some()
    }

    /// Returns the DP-FedAvg configuration, if any.
    pub fn dp_config(&self) -> Option<DpFedConfig> {
        self.dp_config
    }

    /// Returns the current global parameters.
    pub fn global_parameters(&self) -> &[f64] {
        &self.global_parameters
    }

    /// Returns the number of model parameters.
    pub fn num_params(&self) -> usize {
        self.num_params
    }

    /// Returns the number of completed aggregation rounds.
    pub fn round(&self) -> usize {
        self.round
    }

    /// Validates that every update has the expected parameter dimension.
    fn validate_updates(&self, updates: &[ModelUpdate]) -> SimResult<()> {
        if updates.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "no model updates to aggregate".to_string(),
            ));
        }
        for update in updates {
            if update.parameters.len() != self.num_params {
                return Err(SimulationError::InvalidParameter(format!(
                    "update from '{}' has {} parameters, expected {}",
                    update.org_id,
                    update.parameters.len(),
                    self.num_params
                )));
            }
        }
        Ok(())
    }

    /// Aggregates local updates into a new global model.
    ///
    /// With DP disabled this is sample-weighted FedAvg. With DP enabled it is
    /// DP-FedAvg: each update is L2-clipped, the *unweighted* mean is taken, and
    /// Gaussian noise with sensitivity `clip_norm / n` is added (the textbook
    /// add/remove-one sensitivity of the mean of `n` clipped updates).
    pub fn aggregate<R: RngExt>(
        &mut self,
        updates: &[ModelUpdate],
        rng: &mut R,
    ) -> SimResult<Vec<f64>> {
        self.validate_updates(updates)?;

        let new_params = if let Some(config) = self.dp_config {
            let n = updates.len() as f64;
            let mut acc = vec![0.0; self.num_params];
            for update in updates {
                let mut params = update.parameters.clone();
                clip_l2_norm(&mut params, config.clip_norm);
                for (a, p) in acc.iter_mut().zip(params.iter()) {
                    *a += p;
                }
            }
            for a in acc.iter_mut() {
                *a /= n;
            }
            let mechanism =
                GaussianMechanism::new(config.clip_norm / n, config.epsilon, config.delta)?;
            for a in acc.iter_mut() {
                *a = mechanism.add_noise(*a, rng);
            }
            acc
        } else {
            let total: usize = updates.iter().map(|u| u.num_samples).sum();
            if total == 0 {
                return Err(SimulationError::InvalidParameter(
                    "total sample count across updates is zero".to_string(),
                ));
            }
            let mut acc = vec![0.0; self.num_params];
            for update in updates {
                let weight = update.num_samples as f64 / total as f64;
                for (a, p) in acc.iter_mut().zip(update.parameters.iter()) {
                    *a += weight * p;
                }
            }
            acc
        };

        self.global_parameters = new_params.clone();
        self.round += 1;
        Ok(new_params)
    }

    /// Sets the global parameters directly and advances the round counter.
    pub fn set_global(&mut self, params: Vec<f64>) -> SimResult<()> {
        if params.len() != self.num_params {
            return Err(SimulationError::InvalidParameter(format!(
                "expected {} parameters, got {}",
                self.num_params,
                params.len()
            )));
        }
        self.global_parameters = params;
        self.round += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    fn separable_data() -> Vec<(Vec<f64>, f64)> {
        let mut data = Vec::new();
        for i in 1..=20 {
            data.push((vec![i as f64], 1.0));
            data.push((vec![-(i as f64)], 0.0));
        }
        data
    }

    #[test]
    fn test_local_model_validation() {
        assert!(LocalModel::new("a".to_string(), 0, 0.1).is_err());
        assert!(LocalModel::new("a".to_string(), 3, 0.0).is_err());
        let m = LocalModel::new("a".to_string(), 3, 0.1).unwrap();
        assert_eq!(m.num_features(), 3);
    }

    #[test]
    fn test_predict_range_and_dim_check() {
        let model = LocalModel::new("a".to_string(), 2, 0.1).unwrap();
        let p = model.predict(&[1.0, -1.0]).unwrap();
        assert!((0.0..=1.0).contains(&p));
        assert!(model.predict(&[1.0]).is_err());
    }

    #[test]
    fn test_training_reduces_loss() {
        let mut model = LocalModel::new("a".to_string(), 1, 0.2).unwrap();
        let data = separable_data();
        let initial = model.loss(&data).unwrap();
        let final_loss = model.train(&data, 200).unwrap();
        assert!(
            final_loss < initial,
            "loss did not drop: {initial} -> {final_loss}"
        );
        // After learning, a large positive feature predicts the positive class.
        assert!(model.predict(&[10.0]).unwrap() > 0.5);
        assert!(model.predict(&[-10.0]).unwrap() < 0.5);
    }

    #[test]
    fn test_parameters_roundtrip() {
        let mut model = LocalModel::new("a".to_string(), 3, 0.1).unwrap();
        model.set_parameters(&[1.0, 2.0, 3.0, 0.5]).unwrap();
        assert_eq!(model.parameters(), vec![1.0, 2.0, 3.0, 0.5]);
        assert!(model.set_parameters(&[1.0, 2.0]).is_err());
    }

    #[test]
    fn test_model_update_construction() {
        let model = LocalModel::new("org-1".to_string(), 2, 0.1).unwrap();
        let update = model.to_update(42);
        assert_eq!(update.org_id, "org-1");
        assert_eq!(update.num_samples, 42);
        assert_eq!(update.parameters.len(), 3);
    }

    #[test]
    fn test_fedavg_weighted() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut coordinator = FederatedAveraging::new(2).unwrap();
        let updates = vec![
            ModelUpdate {
                org_id: "a".to_string(),
                parameters: vec![0.0, 0.0],
                num_samples: 1,
            },
            ModelUpdate {
                org_id: "b".to_string(),
                parameters: vec![4.0, 8.0],
                num_samples: 3,
            },
        ];
        let global = coordinator.aggregate(&updates, &mut rng).unwrap();
        // Weighted mean: (1*0 + 3*4)/4 = 3, (1*0 + 3*8)/4 = 6.
        assert!((global[0] - 3.0).abs() < 1e-9);
        assert!((global[1] - 6.0).abs() < 1e-9);
        assert_eq!(coordinator.round(), 1);
    }

    #[test]
    fn test_fedavg_empty_and_dim_errors() {
        let mut rng = StdRng::seed_from_u64(2);
        let mut coordinator = FederatedAveraging::new(2).unwrap();
        assert!(coordinator.aggregate(&[], &mut rng).is_err());
        let bad = vec![ModelUpdate {
            org_id: "a".to_string(),
            parameters: vec![1.0],
            num_samples: 1,
        }];
        assert!(coordinator.aggregate(&bad, &mut rng).is_err());
    }

    #[test]
    fn test_dp_fedavg_applies_noise() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut coordinator = FederatedAveraging::new(2).unwrap();
        coordinator.set_dp(DpFedConfig::new(1.0, 0.5, 1e-5).unwrap());
        assert!(coordinator.dp_enabled());

        let updates = vec![
            ModelUpdate {
                org_id: "a".to_string(),
                parameters: vec![0.3, 0.4],
                num_samples: 10,
            },
            ModelUpdate {
                org_id: "b".to_string(),
                parameters: vec![0.3, 0.4],
                num_samples: 10,
            },
        ];
        // Both updates identical and within the clip bound, so the noiseless mean
        // would be exactly [0.3, 0.4]; DP noise must perturb it.
        let global = coordinator.aggregate(&updates, &mut rng).unwrap();
        let perturbed = (global[0] - 0.3).abs() > 1e-9 || (global[1] - 0.4).abs() > 1e-9;
        assert!(perturbed, "DP-FedAvg produced no noise");
    }

    #[test]
    fn test_dp_config_validation() {
        assert!(DpFedConfig::new(0.0, 0.5, 1e-5).is_err());
        assert!(DpFedConfig::new(1.0, 0.5, 0.0).is_err());
        assert!(DpFedConfig::new(1.0, 0.5, 1e-5).is_ok());
    }

    #[test]
    fn test_set_global_advances_round() {
        let mut coordinator = FederatedAveraging::new(3).unwrap();
        coordinator.set_global(vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(coordinator.round(), 1);
        assert_eq!(coordinator.global_parameters(), &[1.0, 2.0, 3.0]);
        assert!(coordinator.set_global(vec![1.0]).is_err());
    }
}
