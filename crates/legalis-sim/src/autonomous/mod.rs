//! Autonomous Simulation (v0.3.3).
//!
//! This module turns the simulation engine into a self-driving experimentation
//! platform. Instead of a human hand-picking parameters, scenarios and recovery
//! actions, the autonomous layer *closes the loop*: it proposes configurations,
//! observes a scalar objective (any metric derived from a
//! [`crate::SimulationMetrics`] run, or a user closure), and adapts. It is built
//! from five real, cooperating
//! building blocks, each in its own focused submodule:
//!
//! - **Self-tuning simulation parameters** ([`self_tuning`]) — an online UCB1
//!   multi-armed-bandit controller and a Metropolis simulated-annealing
//!   controller that drive parameters toward a [`self_tuning::TargetMetric`]
//!   (maximise / minimise / hit a target value).
//! - **Automated scenario generation** ([`scenario_generation`]) — full-factorial
//!   combinatorial designs, Latin-hypercube sampling, a low-discrepancy Halton
//!   (Sobol-like) quasi-random sequence, and a novelty-seeking generator that
//!   maximises behavioural diversity.
//! - **Intelligent parameter-space exploration** ([`exploration`]) — a
//!   from-scratch Gaussian-process surrogate (RBF kernel, Cholesky solve) with
//!   Expected-Improvement / UCB / Probability-of-Improvement acquisition driving
//!   Bayesian optimisation, plus a differential-evolution global optimiser.
//! - **Self-healing simulation systems** ([`self_healing`]) — invariant-based
//!   detection of degenerate (NaN/∞/out-of-range), diverged (explosion / large
//!   step) and stalled (stagnation) runs, with an escalating ladder of recovery
//!   strategies that auto-correct and restart from the last good state.
//! - **Meta-learning for simulation optimisation** ([`meta_learning`]) — a
//!   transferable ridge-regression performance model plus a similarity-weighted
//!   warm-start recommender that learns good starting parameters across past runs.
//!
//! All submodules share the [`ParameterSpace`] abstraction (named, bounded
//! dimensions with normalisation to/from the unit hypercube) and reuse the
//! crate's existing [`Objective`], [`ParameterBounds`] and
//! [`crate::OptimizationResult`] types rather than duplicating them.

pub mod exploration;
pub mod meta_learning;
pub mod scenario_generation;
pub mod self_healing;
pub mod self_tuning;

pub use exploration::*;
pub use meta_learning::*;
pub use scenario_generation::*;
pub use self_healing::*;
pub use self_tuning::*;

use crate::{Objective, ParameterBounds, SimResult, SimulationError};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A named, bounded parameter space over which the autonomous layer operates.
///
/// A space is an ordered list of dimensions, each with a name and
/// [`ParameterBounds`]. It bridges three representations used throughout the
/// module:
/// - the **named** form ([`HashMap<String, f64>`]) used by objective closures and
///   the crate's `optimization` / `calibration` modules,
/// - the **ordered actual** form ([`Vec<f64>`]) for vector arithmetic, and
/// - the **unit** form (a point in `[0, 1]^d`) for scale-free sampling, distances
///   and surrogate models.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParameterSpace {
    names: Vec<String>,
    bounds: Vec<ParameterBounds>,
}

impl ParameterSpace {
    /// Creates an empty parameter space.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a dimension, returning `self` for builder-style chaining.
    pub fn with_dimension(
        mut self,
        name: impl Into<String>,
        lower: f64,
        upper: f64,
    ) -> SimResult<Self> {
        self.add_dimension(name, lower, upper)?;
        Ok(self)
    }

    /// Adds a dimension in place.
    pub fn add_dimension(
        &mut self,
        name: impl Into<String>,
        lower: f64,
        upper: f64,
    ) -> SimResult<()> {
        let name = name.into();
        if name.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "parameter name cannot be empty".to_string(),
            ));
        }
        if self.names.iter().any(|n| n == &name) {
            return Err(SimulationError::InvalidParameter(format!(
                "duplicate parameter dimension '{name}'"
            )));
        }
        let bounds = ParameterBounds::new(lower, upper)?;
        self.names.push(name);
        self.bounds.push(bounds);
        Ok(())
    }

    /// Returns the number of dimensions.
    pub fn dimensions(&self) -> usize {
        self.names.len()
    }

    /// Returns whether the space has no dimensions.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Returns the ordered dimension names.
    pub fn names(&self) -> &[String] {
        &self.names
    }

    /// Returns the ordered dimension bounds.
    pub fn bounds(&self) -> &[ParameterBounds] {
        &self.bounds
    }

    /// Returns the bounds for a named dimension, if present.
    pub fn bounds_for(&self, name: &str) -> Option<&ParameterBounds> {
        self.names
            .iter()
            .position(|n| n == name)
            .map(|i| &self.bounds[i])
    }

    /// Returns the width (`upper - lower`) of dimension `index`.
    pub fn width(&self, index: usize) -> f64 {
        self.bounds[index].upper - self.bounds[index].lower
    }

    fn require_non_empty(&self) -> SimResult<()> {
        if self.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "parameter space has no dimensions".to_string(),
            ));
        }
        Ok(())
    }

    /// Maps a unit-cube point (`[0, 1]^d`) to an ordered actual-value vector.
    pub fn denormalize(&self, unit: &[f64]) -> SimResult<Vec<f64>> {
        if unit.len() != self.dimensions() {
            return Err(SimulationError::InvalidParameter(format!(
                "unit point has {} components, space has {} dimensions",
                unit.len(),
                self.dimensions()
            )));
        }
        Ok(unit
            .iter()
            .zip(self.bounds.iter())
            .map(|(u, b)| b.lower + u.clamp(0.0, 1.0) * (b.upper - b.lower))
            .collect())
    }

    /// Maps a unit-cube point to a named parameter assignment.
    pub fn denormalize_named(&self, unit: &[f64]) -> SimResult<HashMap<String, f64>> {
        let values = self.denormalize(unit)?;
        Ok(self.to_named(&values))
    }

    /// Maps an ordered actual-value vector to a unit-cube point.
    pub fn normalize(&self, values: &[f64]) -> SimResult<Vec<f64>> {
        if values.len() != self.dimensions() {
            return Err(SimulationError::InvalidParameter(format!(
                "value vector has {} components, space has {} dimensions",
                values.len(),
                self.dimensions()
            )));
        }
        Ok(values
            .iter()
            .zip(self.bounds.iter())
            .map(|(v, b)| {
                let width = b.upper - b.lower;
                if width.abs() < f64::EPSILON {
                    0.0
                } else {
                    ((v - b.lower) / width).clamp(0.0, 1.0)
                }
            })
            .collect())
    }

    /// Maps a named assignment to a unit-cube point (missing names default to the
    /// dimension's lower bound).
    pub fn normalize_named(&self, named: &HashMap<String, f64>) -> Vec<f64> {
        self.names
            .iter()
            .zip(self.bounds.iter())
            .map(|(name, b)| {
                let v = named.get(name).copied().unwrap_or(b.lower);
                let width = b.upper - b.lower;
                if width.abs() < f64::EPSILON {
                    0.0
                } else {
                    ((v - b.lower) / width).clamp(0.0, 1.0)
                }
            })
            .collect()
    }

    /// Converts an ordered actual-value vector to a named assignment.
    pub fn to_named(&self, values: &[f64]) -> HashMap<String, f64> {
        self.names
            .iter()
            .cloned()
            .zip(values.iter().copied())
            .collect()
    }

    /// Converts a named assignment to an ordered actual-value vector (missing
    /// names default to the dimension's lower bound).
    pub fn to_vector(&self, named: &HashMap<String, f64>) -> Vec<f64> {
        self.names
            .iter()
            .zip(self.bounds.iter())
            .map(|(name, b)| named.get(name).copied().unwrap_or(b.lower))
            .collect()
    }

    /// Clamps a named assignment into the space's bounds.
    pub fn clamp_named(&self, named: &HashMap<String, f64>) -> HashMap<String, f64> {
        self.names
            .iter()
            .zip(self.bounds.iter())
            .map(|(name, b)| {
                let v = named.get(name).copied().unwrap_or(b.lower);
                (name.clone(), b.clamp(v))
            })
            .collect()
    }

    /// Returns the geometric centre of the space as a named assignment.
    pub fn center(&self) -> HashMap<String, f64> {
        self.names
            .iter()
            .zip(self.bounds.iter())
            .map(|(name, b)| (name.clone(), 0.5 * (b.lower + b.upper)))
            .collect()
    }

    /// Draws a uniform random unit-cube point.
    pub fn random_unit<R: RngExt>(&self, rng: &mut R) -> SimResult<Vec<f64>> {
        self.require_non_empty()?;
        Ok((0..self.dimensions())
            .map(|_| rng.random_range(0.0..1.0))
            .collect())
    }

    /// Draws a uniform random named assignment inside the space.
    pub fn random_named<R: RngExt>(&self, rng: &mut R) -> SimResult<HashMap<String, f64>> {
        let unit = self.random_unit(rng)?;
        self.denormalize_named(&unit)
    }
}

/// Returns `true` if `candidate` improves on `incumbent` under `objective`.
pub(crate) fn is_improvement(objective: Objective, candidate: f64, incumbent: f64) -> bool {
    match objective {
        Objective::Maximize => candidate > incumbent,
        Objective::Minimize => candidate < incumbent,
    }
}

/// Returns the worst representable objective value (used to seed incumbents).
pub(crate) fn worst_objective_value(objective: Objective) -> f64 {
    match objective {
        Objective::Maximize => f64::NEG_INFINITY,
        Objective::Minimize => f64::INFINITY,
    }
}

/// Draws a standard-normal sample via the Box-Muller transform.
pub(crate) fn standard_normal<R: RngExt>(rng: &mut R) -> f64 {
    // Guard the lower endpoint away from zero so `ln` is finite.
    let u1: f64 = rng.random_range(1e-12..1.0);
    let u2: f64 = rng.random_range(0.0..1.0);
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

/// Dot product of two equal-length slices.
pub(crate) fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Squared Euclidean distance between two equal-length points.
pub(crate) fn squared_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y) * (x - y)).sum()
}

/// Euclidean distance between two equal-length points.
pub(crate) fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    squared_distance(a, b).sqrt()
}

/// Computes the lower-triangular Cholesky factor `L` of a symmetric
/// positive-definite matrix `a` such that `a == L Lᵀ`.
///
/// Returns an error if `a` is not positive definite (e.g. a non-PD kernel
/// matrix), which callers handle by adding jitter to the diagonal.
pub(crate) fn cholesky_decompose(a: &[Vec<f64>]) -> SimResult<Vec<Vec<f64>>> {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            // Subtract the running dot product of the already-computed prefixes
            // of rows i and j (the standard Cholesky-Banachiewicz recurrence).
            let prefix: f64 = l[i]
                .iter()
                .zip(l[j].iter())
                .take(j)
                .map(|(x, y)| x * y)
                .sum();
            let sum = a[i][j] - prefix;
            if i == j {
                if sum <= 0.0 {
                    return Err(SimulationError::ExecutionError(
                        "matrix is not positive definite".to_string(),
                    ));
                }
                l[i][j] = sum.sqrt();
            } else {
                l[i][j] = sum / l[j][j];
            }
        }
    }
    Ok(l)
}

/// Solves `L y = b` for `y` by forward substitution (`L` lower-triangular).
pub(crate) fn forward_substitution(l: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = l.len();
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut sum = b[i];
        for k in 0..i {
            sum -= l[i][k] * y[k];
        }
        y[i] = sum / l[i][i];
    }
    y
}

/// Solves `A x = b` given the Cholesky factor `L` of `A` (`A == L Lᵀ`).
pub(crate) fn cholesky_solve(l: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = l.len();
    let y = forward_substitution(l, b);
    // Back substitution: Lᵀ x = y.
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = y[i];
        for k in (i + 1)..n {
            sum -= l[k][i] * x[k];
        }
        x[i] = sum / l[i][i];
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    fn unit_space() -> ParameterSpace {
        ParameterSpace::new()
            .with_dimension("x", 0.0, 10.0)
            .unwrap()
            .with_dimension("y", -5.0, 5.0)
            .unwrap()
    }

    #[test]
    fn test_parameter_space_construction_and_errors() {
        let space = unit_space();
        assert_eq!(space.dimensions(), 2);
        assert!(!space.is_empty());
        assert_eq!(space.names(), &["x".to_string(), "y".to_string()]);
        assert!((space.width(0) - 10.0).abs() < 1e-12);

        // Duplicate and empty names rejected.
        assert!(space.clone().with_dimension("x", 0.0, 1.0).is_err());
        assert!(space.clone().with_dimension("", 0.0, 1.0).is_err());
        // Invalid bounds rejected by ParameterBounds.
        let mut bad = ParameterSpace::new();
        assert!(bad.add_dimension("z", 5.0, 1.0).is_err());
    }

    #[test]
    fn test_normalize_denormalize_roundtrip() {
        let space = unit_space();
        let unit = vec![0.25, 0.75];
        let actual = space.denormalize(&unit).unwrap();
        assert!((actual[0] - 2.5).abs() < 1e-12);
        assert!((actual[1] - 2.5).abs() < 1e-12);
        let back = space.normalize(&actual).unwrap();
        assert!((back[0] - 0.25).abs() < 1e-12);
        assert!((back[1] - 0.75).abs() < 1e-12);

        // Dimension mismatch is an error.
        assert!(space.denormalize(&[0.5]).is_err());
        assert!(space.normalize(&[1.0]).is_err());
    }

    #[test]
    fn test_named_conversions_and_center() {
        let space = unit_space();
        let named = space.center();
        assert!((named["x"] - 5.0).abs() < 1e-12);
        assert!((named["y"] - 0.0).abs() < 1e-12);

        let vector = space.to_vector(&named);
        assert_eq!(vector.len(), 2);
        let renamed = space.to_named(&vector);
        assert!((renamed["x"] - 5.0).abs() < 1e-12);

        // Missing names default to lower bound; out-of-range values clamp.
        let mut partial = HashMap::new();
        partial.insert("x".to_string(), 100.0);
        let clamped = space.clamp_named(&partial);
        assert!((clamped["x"] - 10.0).abs() < 1e-12);
        assert!((clamped["y"] - (-5.0)).abs() < 1e-12);
    }

    #[test]
    fn test_random_points_within_bounds() {
        let space = unit_space();
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..50 {
            let named = space.random_named(&mut rng).unwrap();
            assert!((0.0..=10.0).contains(&named["x"]));
            assert!((-5.0..=5.0).contains(&named["y"]));
        }
        // Empty space cannot produce a point.
        assert!(ParameterSpace::new().random_unit(&mut rng).is_err());
    }

    #[test]
    fn test_cholesky_solves_linear_system() {
        // A = [[4, 2], [2, 3]] is symmetric positive definite.
        let a = vec![vec![4.0, 2.0], vec![2.0, 3.0]];
        let l = cholesky_decompose(&a).unwrap();
        // Reconstruct A from L Lᵀ.
        let reconstructed = [
            [l[0][0] * l[0][0], l[0][0] * l[1][0]],
            [l[1][0] * l[0][0], l[1][0] * l[1][0] + l[1][1] * l[1][1]],
        ];
        for i in 0..2 {
            for j in 0..2 {
                assert!((reconstructed[i][j] - a[i][j]).abs() < 1e-9);
            }
        }
        // Solve A x = b for a known x = [1, -1] => b = [2, -1].
        let x = cholesky_solve(&l, &[2.0, -1.0]);
        assert!((x[0] - 1.0).abs() < 1e-9);
        assert!((x[1] + 1.0).abs() < 1e-9);

        // Non-PD matrix is rejected.
        let non_pd = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
        assert!(cholesky_decompose(&non_pd).is_err());
    }

    #[test]
    fn test_objective_helpers() {
        assert!(is_improvement(Objective::Maximize, 2.0, 1.0));
        assert!(!is_improvement(Objective::Maximize, 0.5, 1.0));
        assert!(is_improvement(Objective::Minimize, 0.5, 1.0));
        assert_eq!(
            worst_objective_value(Objective::Maximize),
            f64::NEG_INFINITY
        );
        assert_eq!(worst_objective_value(Objective::Minimize), f64::INFINITY);
    }
}
