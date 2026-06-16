//! Differential privacy mechanisms and privacy-budget accounting.
//!
//! This submodule provides the `(ε, δ)`-differential-privacy machinery used to
//! protect federated aggregates and shared results:
//! - the [`LaplaceMechanism`] (pure `ε`-DP, calibrated to the L1 sensitivity),
//! - the [`GaussianMechanism`] (`(ε, δ)`-DP, calibrated to the L2 sensitivity),
//! - L2 gradient/update clipping ([`clip_l2_norm`]) to bound sensitivity, and
//! - a [`PrivacyAccountant`] that tracks cumulative budget under both basic
//!   (sequential) composition and the tighter advanced-composition bound.

use crate::error::{SimResult, SimulationError};
use rand::RngExt;
use serde::{Deserialize, Serialize};

/// An `(ε, δ)` privacy specification for a single mechanism invocation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct PrivacyParams {
    /// Privacy-loss parameter `ε` (smaller is more private; must be positive).
    pub epsilon: f64,
    /// Failure-probability parameter `δ` in `[0, 1)` (`0` for pure `ε`-DP).
    pub delta: f64,
}

impl PrivacyParams {
    /// Creates a validated `(ε, δ)` specification.
    pub fn new(epsilon: f64, delta: f64) -> SimResult<Self> {
        if epsilon <= 0.0 || !epsilon.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "epsilon must be positive and finite".to_string(),
            ));
        }
        if !(0.0..1.0).contains(&delta) || !delta.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "delta must lie in [0, 1)".to_string(),
            ));
        }
        Ok(Self { epsilon, delta })
    }

    /// Creates a pure `ε`-DP specification (`δ = 0`).
    pub fn pure(epsilon: f64) -> SimResult<Self> {
        Self::new(epsilon, 0.0)
    }

    /// Returns `true` for pure `ε`-DP (`δ == 0`).
    pub fn is_pure(&self) -> bool {
        self.delta == 0.0
    }
}

/// Identifies which additive-noise mechanism realises a privacy guarantee.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DpMechanism {
    /// Laplace mechanism (pure `ε`-DP).
    Laplace,
    /// Gaussian mechanism (`(ε, δ)`-DP).
    Gaussian,
}

/// Computes the Euclidean (L2) norm of a vector.
pub fn l2_norm(vector: &[f64]) -> f64 {
    vector.iter().map(|v| v * v).sum::<f64>().sqrt()
}

/// Clips the L2 norm of `vector` in place to at most `max_norm`.
///
/// This is the standard sensitivity-bounding step of DP-SGD / DP-FedAvg: it
/// rescales the vector by `min(1, max_norm / ‖vector‖)`. The original (pre-clip)
/// L2 norm is returned.
pub fn clip_l2_norm(vector: &mut [f64], max_norm: f64) -> f64 {
    let norm = l2_norm(vector);
    if norm > max_norm && norm > 0.0 {
        let factor = max_norm / norm;
        for v in vector.iter_mut() {
            *v *= factor;
        }
    }
    norm
}

/// The Laplace mechanism for pure `ε`-differential privacy.
///
/// Noise is drawn from `Laplace(0, b)` with scale `b = sensitivity / ε`, where
/// `sensitivity` is the L1 sensitivity of the query.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LaplaceMechanism {
    /// L1 sensitivity of the query being privatised.
    sensitivity: f64,
    /// Privacy-loss parameter `ε`.
    epsilon: f64,
}

impl LaplaceMechanism {
    /// Creates a Laplace mechanism for the given L1 `sensitivity` and `epsilon`.
    pub fn new(sensitivity: f64, epsilon: f64) -> SimResult<Self> {
        if sensitivity <= 0.0 || !sensitivity.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "sensitivity must be positive and finite".to_string(),
            ));
        }
        if epsilon <= 0.0 || !epsilon.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "epsilon must be positive and finite".to_string(),
            ));
        }
        Ok(Self {
            sensitivity,
            epsilon,
        })
    }

    /// Returns the Laplace scale `b = sensitivity / ε`.
    pub fn scale(&self) -> f64 {
        self.sensitivity / self.epsilon
    }

    /// Draws a single zero-mean Laplace noise sample via inverse-CDF sampling.
    pub fn sample<R: RngExt>(&self, rng: &mut R) -> f64 {
        let u: f64 = rng.random_range(-0.5..0.5);
        -self.scale() * u.signum() * (1.0 - 2.0 * u.abs()).ln()
    }

    /// Adds calibrated Laplace noise to a value.
    pub fn add_noise<R: RngExt>(&self, value: f64, rng: &mut R) -> f64 {
        value + self.sample(rng)
    }

    /// Adds independent Laplace noise to each element of a vector.
    pub fn privatize<R: RngExt>(&self, values: &[f64], rng: &mut R) -> Vec<f64> {
        values.iter().map(|&v| self.add_noise(v, rng)).collect()
    }

    /// Returns the `(ε, δ=0)` guarantee realised by this mechanism.
    pub fn privacy_params(&self) -> PrivacyParams {
        PrivacyParams {
            epsilon: self.epsilon,
            delta: 0.0,
        }
    }
}

/// The Gaussian mechanism for `(ε, δ)`-differential privacy.
///
/// Noise is drawn from `N(0, σ²)` with the classical calibration
/// `σ = sensitivity · √(2 ln(1.25 / δ)) / ε`, where `sensitivity` is the L2
/// sensitivity of the query (this analysis is valid for `ε < 1`).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GaussianMechanism {
    /// L2 sensitivity of the query being privatised.
    sensitivity: f64,
    /// Privacy specification (requires `δ > 0`).
    params: PrivacyParams,
}

impl GaussianMechanism {
    /// Creates a Gaussian mechanism for the given L2 `sensitivity`, `epsilon`, `delta`.
    pub fn new(sensitivity: f64, epsilon: f64, delta: f64) -> SimResult<Self> {
        if sensitivity <= 0.0 || !sensitivity.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "sensitivity must be positive and finite".to_string(),
            ));
        }
        let params = PrivacyParams::new(epsilon, delta)?;
        if params.delta <= 0.0 {
            return Err(SimulationError::InvalidParameter(
                "Gaussian mechanism requires delta > 0".to_string(),
            ));
        }
        Ok(Self {
            sensitivity,
            params,
        })
    }

    /// Returns the noise standard deviation `σ`.
    pub fn sigma(&self) -> f64 {
        self.sensitivity * (2.0 * (1.25 / self.params.delta).ln()).sqrt() / self.params.epsilon
    }

    /// Draws a single zero-mean Gaussian noise sample via the Box-Muller transform.
    pub fn sample<R: RngExt>(&self, rng: &mut R) -> f64 {
        let u1: f64 = rng.random_range(f64::EPSILON..1.0);
        let u2: f64 = rng.random_range(0.0..1.0);
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        self.sigma() * z
    }

    /// Adds calibrated Gaussian noise to a value.
    pub fn add_noise<R: RngExt>(&self, value: f64, rng: &mut R) -> f64 {
        value + self.sample(rng)
    }

    /// Adds independent Gaussian noise to each element of a vector.
    pub fn privatize<R: RngExt>(&self, values: &[f64], rng: &mut R) -> Vec<f64> {
        values.iter().map(|&v| self.add_noise(v, rng)).collect()
    }

    /// Returns the `(ε, δ)` guarantee realised by this mechanism.
    pub fn privacy_params(&self) -> PrivacyParams {
        self.params
    }
}

/// Total privacy budget available to a workload.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PrivacyBudget {
    /// Maximum cumulative `ε` that may be spent.
    pub epsilon: f64,
    /// Maximum cumulative `δ` that may be spent.
    pub delta: f64,
}

/// Tracks cumulative privacy expenditure and enforces a fixed budget.
///
/// Each released query is charged via [`PrivacyAccountant::spend`], which applies
/// **basic (sequential) composition**: cumulative `ε` and `δ` are the sums of the
/// per-query values, and a query that would exceed the budget is rejected. The
/// tighter [`PrivacyAccountant::advanced_composition_epsilon`] bound is also
/// available for reporting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAccountant {
    /// The total budget.
    budget: PrivacyBudget,
    /// Cumulative `ε` spent so far.
    spent_epsilon: f64,
    /// Cumulative `δ` spent so far.
    spent_delta: f64,
    /// Per-query history (for composition analysis).
    history: Vec<PrivacyParams>,
}

impl PrivacyAccountant {
    /// Creates an accountant with the given total `ε` and `δ` budget.
    pub fn new(epsilon: f64, delta: f64) -> SimResult<Self> {
        if epsilon <= 0.0 || !epsilon.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "budget epsilon must be positive and finite".to_string(),
            ));
        }
        if !(0.0..1.0).contains(&delta) || !delta.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "budget delta must lie in [0, 1)".to_string(),
            ));
        }
        Ok(Self {
            budget: PrivacyBudget { epsilon, delta },
            spent_epsilon: 0.0,
            spent_delta: 0.0,
            history: Vec::new(),
        })
    }

    /// Returns the total budget.
    pub fn budget(&self) -> PrivacyBudget {
        self.budget
    }

    /// Returns the cumulative `ε` spent.
    pub fn spent_epsilon(&self) -> f64 {
        self.spent_epsilon
    }

    /// Returns the cumulative `δ` spent.
    pub fn spent_delta(&self) -> f64 {
        self.spent_delta
    }

    /// Returns the remaining `ε` budget.
    pub fn remaining_epsilon(&self) -> f64 {
        (self.budget.epsilon - self.spent_epsilon).max(0.0)
    }

    /// Returns the remaining `δ` budget.
    pub fn remaining_delta(&self) -> f64 {
        (self.budget.delta - self.spent_delta).max(0.0)
    }

    /// Returns `true` if the given query can be charged without exceeding budget.
    pub fn can_spend(&self, params: &PrivacyParams) -> bool {
        const TOL: f64 = 1e-12;
        self.spent_epsilon + params.epsilon <= self.budget.epsilon + TOL
            && self.spent_delta + params.delta <= self.budget.delta + TOL
    }

    /// Charges a query against the budget (basic composition).
    ///
    /// Returns an error if the query would exhaust the available budget.
    pub fn spend(&mut self, params: PrivacyParams) -> SimResult<()> {
        if !self.can_spend(&params) {
            return Err(SimulationError::InvalidParameter(format!(
                "privacy budget exhausted: requested (ε={:.4}, δ={:.6}), remaining (ε={:.4}, δ={:.6})",
                params.epsilon,
                params.delta,
                self.remaining_epsilon(),
                self.remaining_delta()
            )));
        }
        self.spent_epsilon += params.epsilon;
        self.spent_delta += params.delta;
        self.history.push(params);
        Ok(())
    }

    /// Returns the number of charged queries.
    pub fn num_queries(&self) -> usize {
        self.history.len()
    }

    /// Returns the per-query history.
    pub fn history(&self) -> &[PrivacyParams] {
        &self.history
    }

    /// Resets all expenditure to zero (keeping the budget).
    pub fn reset(&mut self) {
        self.spent_epsilon = 0.0;
        self.spent_delta = 0.0;
        self.history.clear();
    }

    /// Computes the advanced-composition `ε'` bound for the charged queries.
    ///
    /// Using the Dwork–Rothblum–Vadhan bound for `k` mechanisms each `ε`-DP, the
    /// composition is `(ε', kδ + δ')`-DP with
    /// `ε' = ε·√(2k ln(1/δ')) + k·ε·(e^ε − 1)`, taking `ε` as the largest
    /// per-query value. This is typically far smaller than the basic-composition
    /// sum `kε` for many small queries.
    pub fn advanced_composition_epsilon(&self, delta_prime: f64) -> SimResult<f64> {
        if delta_prime <= 0.0 || delta_prime >= 1.0 || !delta_prime.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "delta_prime must lie in (0, 1)".to_string(),
            ));
        }
        let k = self.history.len() as f64;
        if k == 0.0 {
            return Ok(0.0);
        }
        let eps = self
            .history
            .iter()
            .map(|p| p.epsilon)
            .fold(0.0_f64, f64::max);
        let term1 = eps * (2.0 * k * (1.0 / delta_prime).ln()).sqrt();
        let term2 = k * eps * (eps.exp() - 1.0);
        Ok(term1 + term2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn test_privacy_params_validation() {
        assert!(PrivacyParams::new(1.0, 0.0).is_ok());
        assert!(PrivacyParams::new(0.0, 0.0).is_err());
        assert!(PrivacyParams::new(-1.0, 0.0).is_err());
        assert!(PrivacyParams::new(1.0, 1.0).is_err());
        assert!(PrivacyParams::new(1.0, -0.1).is_err());
        assert!(PrivacyParams::pure(0.5).unwrap().is_pure());
    }

    #[test]
    fn test_l2_norm_and_clip() {
        assert!((l2_norm(&[3.0, 4.0]) - 5.0).abs() < 1e-9);

        let mut v = vec![3.0, 4.0];
        let original = clip_l2_norm(&mut v, 1.0);
        assert!((original - 5.0).abs() < 1e-9);
        assert!((l2_norm(&v) - 1.0).abs() < 1e-9);

        // No-op when already within the bound.
        let mut w = vec![0.3, 0.4];
        let n = clip_l2_norm(&mut w, 10.0);
        assert!((n - 0.5).abs() < 1e-9);
        assert!((w[0] - 0.3).abs() < 1e-12 && (w[1] - 0.4).abs() < 1e-12);
    }

    #[test]
    fn test_laplace_scale_and_mean() {
        let mut rng = StdRng::seed_from_u64(1);
        let mech = LaplaceMechanism::new(1.0, 1.0).unwrap();
        assert!((mech.scale() - 1.0).abs() < 1e-12);

        let samples: Vec<f64> = (0..5000).map(|_| mech.sample(&mut rng)).collect();
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        assert!(mean.abs() < 0.2, "laplace mean {mean} not near zero");
    }

    #[test]
    fn test_laplace_privatize_len() {
        let mut rng = StdRng::seed_from_u64(2);
        let mech = LaplaceMechanism::new(2.0, 0.5).unwrap();
        let out = mech.privatize(&[1.0, 2.0, 3.0], &mut rng);
        assert_eq!(out.len(), 3);
        assert!((mech.scale() - 4.0).abs() < 1e-12);
    }

    #[test]
    fn test_laplace_rejects_bad_params() {
        assert!(LaplaceMechanism::new(0.0, 1.0).is_err());
        assert!(LaplaceMechanism::new(1.0, 0.0).is_err());
    }

    #[test]
    fn test_gaussian_sigma_and_mean() {
        let mut rng = StdRng::seed_from_u64(3);
        let mech = GaussianMechanism::new(1.0, 0.5, 1e-5).unwrap();
        assert!(mech.sigma() > 0.0);
        assert!(GaussianMechanism::new(1.0, 0.5, 0.0).is_err());

        let samples: Vec<f64> = (0..5000).map(|_| mech.sample(&mut rng)).collect();
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        assert!(mean.abs() < 0.5 * mech.sigma(), "gaussian mean {mean} off");
    }

    #[test]
    fn test_accountant_spend_and_remaining() {
        let mut acc = PrivacyAccountant::new(1.0, 1e-3).unwrap();
        acc.spend(PrivacyParams::new(0.3, 1e-4).unwrap()).unwrap();
        acc.spend(PrivacyParams::new(0.4, 1e-4).unwrap()).unwrap();
        assert_eq!(acc.num_queries(), 2);
        assert!((acc.spent_epsilon() - 0.7).abs() < 1e-9);
        assert!((acc.remaining_epsilon() - 0.3).abs() < 1e-9);
    }

    #[test]
    fn test_accountant_budget_exhausted() {
        let mut acc = PrivacyAccountant::new(0.5, 0.0).unwrap();
        acc.spend(PrivacyParams::pure(0.4).unwrap()).unwrap();
        let params = PrivacyParams::pure(0.2).unwrap();
        assert!(!acc.can_spend(&params));
        assert!(acc.spend(params).is_err());
        // The rejected query left state unchanged.
        assert!((acc.spent_epsilon() - 0.4).abs() < 1e-9);
    }

    #[test]
    fn test_accountant_reset() {
        let mut acc = PrivacyAccountant::new(1.0, 1e-3).unwrap();
        acc.spend(PrivacyParams::new(0.5, 1e-4).unwrap()).unwrap();
        acc.reset();
        assert_eq!(acc.num_queries(), 0);
        assert!((acc.spent_epsilon()).abs() < 1e-12);
    }

    #[test]
    fn test_advanced_composition_tighter_than_basic() {
        let mut acc = PrivacyAccountant::new(100.0, 1.0 - 1e-9).unwrap();
        for _ in 0..50 {
            acc.spend(PrivacyParams::new(0.1, 1e-6).unwrap()).unwrap();
        }
        let basic = acc.spent_epsilon(); // 50 * 0.1 = 5.0
        let advanced = acc.advanced_composition_epsilon(1e-5).unwrap();
        assert!((basic - 5.0).abs() < 1e-9);
        assert!(advanced < basic, "advanced {advanced} >= basic {basic}");
        assert!(acc.advanced_composition_epsilon(0.0).is_err());
    }
}
