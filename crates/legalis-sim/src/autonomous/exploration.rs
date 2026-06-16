//! Intelligent exploration of parameter space.
//!
//! Two complementary global search strategies sit on top of [`ParameterSpace`]:
//!
//! - [`BayesianOptimizer`] — sample-efficient Bayesian optimisation driven by a
//!   from-scratch [`GaussianProcess`] surrogate (RBF kernel, Cholesky-based exact
//!   inference) and an [`AcquisitionFunction`] (Expected Improvement, Upper
//!   Confidence Bound, or Probability of Improvement). Ideal when each evaluation
//!   is expensive.
//! - [`DifferentialEvolution`] — a robust population-based global optimiser
//!   (`DE/rand/1/bin`) for cheaper, possibly rugged objectives.
//!
//! Both reuse the crate's [`Objective`] and return a standard
//! [`OptimizationResult`].

use super::{
    ParameterSpace, cholesky_decompose, cholesky_solve, dot, forward_substitution, is_improvement,
    squared_distance, worst_objective_value,
};
use crate::{Objective, OptimizationResult, SimResult, SimulationError};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const TINY: f64 = 1e-12;

/// Standard-normal probability density function.
fn standard_normal_pdf(x: f64) -> f64 {
    (-(x * x) / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

/// Error function via the Abramowitz & Stegun 7.1.26 rational approximation.
fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let y = 1.0
        - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
            + 0.254829592)
            * t
            * (-x * x).exp();
    sign * y
}

/// Standard-normal cumulative distribution function.
fn standard_normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// A Gaussian-process regression surrogate with a squared-exponential (RBF) kernel.
///
/// Inference is exact: the kernel matrix is factorised with a Cholesky
/// decomposition (with automatic diagonal jitter for numerical stability), and
/// predictions return both the posterior mean and variance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaussianProcess {
    length_scale: f64,
    signal_variance: f64,
    noise_variance: f64,
    x_train: Vec<Vec<f64>>,
    y_mean: f64,
    l: Vec<Vec<f64>>,
    alpha: Vec<f64>,
    fitted: bool,
}

impl GaussianProcess {
    /// Creates an unfitted Gaussian process with the given RBF hyperparameters.
    pub fn new(length_scale: f64, signal_variance: f64, noise_variance: f64) -> SimResult<Self> {
        if length_scale <= 0.0 || !length_scale.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "length scale must be positive and finite".to_string(),
            ));
        }
        if signal_variance <= 0.0 || !signal_variance.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "signal variance must be positive and finite".to_string(),
            ));
        }
        if noise_variance < 0.0 || !noise_variance.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "noise variance must be non-negative and finite".to_string(),
            ));
        }
        Ok(Self {
            length_scale,
            signal_variance,
            noise_variance,
            x_train: Vec::new(),
            y_mean: 0.0,
            l: Vec::new(),
            alpha: Vec::new(),
            fitted: false,
        })
    }

    /// Returns whether the process has been fitted to data.
    pub fn is_fitted(&self) -> bool {
        self.fitted
    }

    fn kernel(&self, a: &[f64], b: &[f64]) -> f64 {
        let sq = squared_distance(a, b);
        self.signal_variance * (-0.5 * sq / (self.length_scale * self.length_scale)).exp()
    }

    /// Fits the process to training inputs `x` and targets `y`.
    pub fn fit(&mut self, x: &[Vec<f64>], y: &[f64]) -> SimResult<()> {
        if x.is_empty() || x.len() != y.len() {
            return Err(SimulationError::InvalidParameter(
                "training inputs and targets must be non-empty and equal length".to_string(),
            ));
        }
        let dim = x[0].len();
        if x.iter().any(|row| row.len() != dim) {
            return Err(SimulationError::InvalidParameter(
                "all training inputs must share the same dimensionality".to_string(),
            ));
        }
        let n = x.len();
        self.x_train = x.to_vec();
        self.y_mean = y.iter().sum::<f64>() / n as f64;
        let centered: Vec<f64> = y.iter().map(|v| v - self.y_mean).collect();

        // Build the kernel matrix and Cholesky-factor it, growing the diagonal
        // jitter until the matrix is numerically positive definite.
        let mut jitter = self.noise_variance.max(1e-10);
        let mut factored = None;
        for _ in 0..8 {
            let mut matrix = vec![vec![0.0; n]; n];
            for (i, row) in matrix.iter_mut().enumerate() {
                for (j, cell) in row.iter_mut().enumerate() {
                    *cell = self.kernel(&self.x_train[i], &self.x_train[j]);
                }
                row[i] += jitter;
            }
            match cholesky_decompose(&matrix) {
                Ok(l) => {
                    factored = Some(l);
                    break;
                }
                Err(_) => jitter *= 10.0,
            }
        }
        let l = factored.ok_or_else(|| {
            SimulationError::ExecutionError(
                "failed to factor the Gaussian-process kernel matrix".to_string(),
            )
        })?;
        self.alpha = cholesky_solve(&l, &centered);
        self.l = l;
        self.fitted = true;
        Ok(())
    }

    /// Returns the posterior mean and variance at point `x`.
    pub fn predict(&self, x: &[f64]) -> SimResult<(f64, f64)> {
        if !self.fitted {
            return Err(SimulationError::ExecutionError(
                "Gaussian process has not been fitted".to_string(),
            ));
        }
        let k_star: Vec<f64> = self.x_train.iter().map(|xi| self.kernel(xi, x)).collect();
        let mean = self.y_mean + dot(&k_star, &self.alpha);
        // Predictive variance: k(x, x) - v·v where L v = k_star.
        let v = forward_substitution(&self.l, &k_star);
        let variance = (self.kernel(x, x) - dot(&v, &v)).max(0.0);
        Ok((mean, variance))
    }
}

/// An acquisition function balancing exploration and exploitation.
///
/// Each variant's [`AcquisitionFunction::evaluate`] returns a score to be
/// **maximised** when choosing the next point, correctly oriented for both
/// maximisation and minimisation objectives.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AcquisitionFunction {
    /// Expected improvement over the current best, with exploration margin `xi`.
    ExpectedImprovement {
        /// Exploration margin (larger encourages exploration).
        xi: f64,
    },
    /// Upper/lower confidence bound with width multiplier `beta`.
    UpperConfidenceBound {
        /// Confidence-bound width multiplier.
        beta: f64,
    },
    /// Probability of improving over the current best, with margin `xi`.
    ProbabilityOfImprovement {
        /// Exploration margin.
        xi: f64,
    },
}

impl AcquisitionFunction {
    /// Scores a candidate given its posterior `mean`/`std`, the incumbent `best`,
    /// and the objective sense. Higher is always better.
    pub fn evaluate(&self, mean: f64, std: f64, best: f64, objective: Objective) -> f64 {
        match self {
            AcquisitionFunction::ExpectedImprovement { xi } => {
                if std < TINY {
                    return 0.0;
                }
                let improvement = match objective {
                    Objective::Maximize => mean - best - xi,
                    Objective::Minimize => best - mean - xi,
                };
                let z = improvement / std;
                (improvement * standard_normal_cdf(z) + std * standard_normal_pdf(z)).max(0.0)
            }
            AcquisitionFunction::ProbabilityOfImprovement { xi } => {
                if std < TINY {
                    return 0.0;
                }
                let improvement = match objective {
                    Objective::Maximize => mean - best - xi,
                    Objective::Minimize => best - mean - xi,
                };
                standard_normal_cdf(improvement / std)
            }
            AcquisitionFunction::UpperConfidenceBound { beta } => match objective {
                Objective::Maximize => mean + beta * std,
                Objective::Minimize => -(mean - beta * std),
            },
        }
    }
}

/// Bayesian optimiser over a [`ParameterSpace`] using a GP surrogate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BayesianOptimizer {
    space: ParameterSpace,
    gp: GaussianProcess,
    acquisition: AcquisitionFunction,
    objective: Objective,
    candidate_pool: usize,
    observations: Vec<(Vec<f64>, f64)>,
    best_value: f64,
    best_unit: Option<Vec<f64>>,
}

impl BayesianOptimizer {
    /// Creates a Bayesian optimiser.
    ///
    /// `candidate_pool` is the number of random candidates scored by the
    /// acquisition function when suggesting each new point.
    pub fn new(
        space: ParameterSpace,
        gp: GaussianProcess,
        acquisition: AcquisitionFunction,
        objective: Objective,
        candidate_pool: usize,
    ) -> SimResult<Self> {
        if space.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "Bayesian optimisation requires a non-empty parameter space".to_string(),
            ));
        }
        if candidate_pool == 0 {
            return Err(SimulationError::InvalidParameter(
                "candidate pool must be greater than zero".to_string(),
            ));
        }
        Ok(Self {
            space,
            gp,
            acquisition,
            objective,
            candidate_pool,
            observations: Vec::new(),
            best_value: worst_objective_value(objective),
            best_unit: None,
        })
    }

    /// Returns the best objective value observed so far.
    pub fn best_value(&self) -> f64 {
        self.best_value
    }

    /// Returns the best configuration observed so far, if any.
    pub fn best_parameters(&self) -> SimResult<Option<HashMap<String, f64>>> {
        match &self.best_unit {
            Some(unit) => Ok(Some(self.space.denormalize_named(unit)?)),
            None => Ok(None),
        }
    }

    /// Returns the number of recorded observations.
    pub fn num_observations(&self) -> usize {
        self.observations.len()
    }

    fn refit(&mut self) -> SimResult<()> {
        let xs: Vec<Vec<f64>> = self.observations.iter().map(|(x, _)| x.clone()).collect();
        let ys: Vec<f64> = self.observations.iter().map(|(_, y)| *y).collect();
        self.gp.fit(&xs, &ys)
    }

    /// Records an evaluated configuration and refits the surrogate.
    pub fn observe(&mut self, parameters: &HashMap<String, f64>, value: f64) -> SimResult<()> {
        let unit = self.space.normalize_named(parameters);
        if is_improvement(self.objective, value, self.best_value) || self.best_unit.is_none() {
            self.best_value = value;
            self.best_unit = Some(unit.clone());
        }
        self.observations.push((unit, value));
        self.refit()
    }

    /// Suggests the next configuration to evaluate.
    ///
    /// With no observations yet, returns a uniform random point; otherwise scores
    /// `candidate_pool` random candidates with the acquisition function and
    /// returns the best.
    pub fn suggest<R: RngExt>(&self, rng: &mut R) -> SimResult<HashMap<String, f64>> {
        if self.observations.is_empty() {
            return self.space.random_named(rng);
        }
        let mut best_unit = self.space.random_unit(rng)?;
        let mut best_acq = f64::NEG_INFINITY;
        for _ in 0..self.candidate_pool {
            let candidate = self.space.random_unit(rng)?;
            let (mean, variance) = self.gp.predict(&candidate)?;
            let acq =
                self.acquisition
                    .evaluate(mean, variance.sqrt(), self.best_value, self.objective);
            if acq > best_acq {
                best_acq = acq;
                best_unit = candidate;
            }
        }
        self.space.denormalize_named(&best_unit)
    }

    /// Runs a full Bayesian-optimisation loop.
    ///
    /// `n_init` random points seed the surrogate, then `n_iter` acquisition-driven
    /// iterations refine the search. `eval_fn` maps a configuration to its value.
    pub fn optimize<F, R: RngExt>(
        &mut self,
        n_init: usize,
        n_iter: usize,
        mut eval_fn: F,
        rng: &mut R,
    ) -> SimResult<OptimizationResult>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        if n_init == 0 {
            return Err(SimulationError::InvalidParameter(
                "n_init must be greater than zero".to_string(),
            ));
        }
        let mut history = Vec::with_capacity(n_init + n_iter);

        for _ in 0..n_init {
            let params = self.space.random_named(rng)?;
            let value = eval_fn(&params);
            self.observe(&params, value)?;
            history.push((params, value));
        }
        for _ in 0..n_iter {
            let params = self.suggest(rng)?;
            let value = eval_fn(&params);
            self.observe(&params, value)?;
            history.push((params, value));
        }

        let best_parameters = self
            .best_parameters()?
            .unwrap_or_else(|| self.space.center());
        Ok(OptimizationResult {
            best_parameters,
            best_objective: self.best_value,
            iterations: n_init + n_iter,
            converged: true,
            history,
        })
    }
}

/// `DE/rand/1/bin` differential-evolution global optimiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifferentialEvolution {
    space: ParameterSpace,
    objective: Objective,
    population_size: usize,
    crossover_rate: f64,
    differential_weight: f64,
    max_generations: usize,
}

impl DifferentialEvolution {
    /// Creates a differential-evolution optimiser.
    pub fn new(
        space: ParameterSpace,
        objective: Objective,
        population_size: usize,
        crossover_rate: f64,
        differential_weight: f64,
        max_generations: usize,
    ) -> SimResult<Self> {
        if space.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "differential evolution requires a non-empty parameter space".to_string(),
            ));
        }
        if population_size < 4 {
            return Err(SimulationError::InvalidParameter(
                "population size must be at least 4".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&crossover_rate) {
            return Err(SimulationError::InvalidParameter(
                "crossover rate must lie in [0, 1]".to_string(),
            ));
        }
        if differential_weight <= 0.0 || differential_weight > 2.0 {
            return Err(SimulationError::InvalidParameter(
                "differential weight must lie in (0, 2]".to_string(),
            ));
        }
        if max_generations == 0 {
            return Err(SimulationError::InvalidParameter(
                "max generations must be greater than zero".to_string(),
            ));
        }
        Ok(Self {
            space,
            objective,
            population_size,
            crossover_rate,
            differential_weight,
            max_generations,
        })
    }

    /// Selects three distinct population indices, all different from `exclude`.
    fn pick_three<R: RngExt>(&self, exclude: usize, rng: &mut R) -> (usize, usize, usize) {
        let n = self.population_size;
        let mut pick = || loop {
            let candidate = rng.random_range(0..n);
            if candidate != exclude {
                return candidate;
            }
        };
        let a = pick();
        let mut b = pick();
        while b == a {
            b = pick();
        }
        let mut c = pick();
        while c == a || c == b {
            c = pick();
        }
        (a, b, c)
    }

    /// Runs the evolutionary search, returning the best configuration found.
    pub fn optimize<F, R: RngExt>(
        &self,
        mut eval_fn: F,
        rng: &mut R,
    ) -> SimResult<OptimizationResult>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        let dims = self.space.dimensions();

        // Initialise the population uniformly in the unit cube.
        let mut population: Vec<Vec<f64>> = Vec::with_capacity(self.population_size);
        let mut fitness: Vec<f64> = Vec::with_capacity(self.population_size);
        for _ in 0..self.population_size {
            let unit = self.space.random_unit(rng)?;
            let params = self.space.denormalize_named(&unit)?;
            fitness.push(eval_fn(&params));
            population.push(unit);
        }

        let mut best_idx = 0;
        for i in 1..self.population_size {
            if is_improvement(self.objective, fitness[i], fitness[best_idx]) {
                best_idx = i;
            }
        }
        let mut best_value = fitness[best_idx];
        let mut best_unit = population[best_idx].clone();

        let mut history = Vec::with_capacity(self.max_generations);
        for _ in 0..self.max_generations {
            for i in 0..self.population_size {
                let (a, b, c) = self.pick_three(i, rng);
                let j_rand = rng.random_range(0..dims);
                let mut trial = population[i].clone();
                for d in 0..dims {
                    if d == j_rand || rng.random_range(0.0..1.0) < self.crossover_rate {
                        let mutated = population[a][d]
                            + self.differential_weight * (population[b][d] - population[c][d]);
                        trial[d] = mutated.clamp(0.0, 1.0);
                    }
                }
                let trial_params = self.space.denormalize_named(&trial)?;
                let trial_fitness = eval_fn(&trial_params);
                if is_improvement(self.objective, trial_fitness, fitness[i])
                    || (trial_fitness - fitness[i]).abs() < TINY
                {
                    population[i] = trial;
                    fitness[i] = trial_fitness;
                    if is_improvement(self.objective, trial_fitness, best_value) {
                        best_value = trial_fitness;
                        best_unit = population[i].clone();
                    }
                }
            }
            let best_params = self.space.denormalize_named(&best_unit)?;
            history.push((best_params, best_value));
        }

        Ok(OptimizationResult {
            best_parameters: self.space.denormalize_named(&best_unit)?,
            best_objective: best_value,
            iterations: self.max_generations,
            converged: true,
            history,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    fn space_1d() -> ParameterSpace {
        ParameterSpace::new()
            .with_dimension("x", -5.0, 5.0)
            .unwrap()
    }

    fn space_2d() -> ParameterSpace {
        ParameterSpace::new()
            .with_dimension("x", -5.0, 5.0)
            .unwrap()
            .with_dimension("y", -5.0, 5.0)
            .unwrap()
    }

    #[test]
    fn test_gaussian_process_interpolates() {
        let mut gp = GaussianProcess::new(1.0, 1.0, 1e-8).unwrap();
        assert!(!gp.is_fitted());
        // Fit to y = sin(x) samples.
        let xs: Vec<Vec<f64>> = (0..10).map(|i| vec![i as f64 * 0.3]).collect();
        let ys: Vec<f64> = xs.iter().map(|p| p[0].sin()).collect();
        gp.fit(&xs, &ys).unwrap();
        assert!(gp.is_fitted());

        // Predicting at a training point reproduces its value with low variance.
        let (mean, variance) = gp.predict(&xs[3]).unwrap();
        assert!((mean - ys[3]).abs() < 1e-3, "mean {mean} vs {}", ys[3]);
        assert!(variance < 1e-2);

        // Far from data, predictive variance grows.
        let (_, far_var) = gp.predict(&[10.0]).unwrap();
        assert!(far_var > variance);
    }

    #[test]
    fn test_gaussian_process_validation() {
        assert!(GaussianProcess::new(0.0, 1.0, 0.1).is_err());
        assert!(GaussianProcess::new(1.0, -1.0, 0.1).is_err());
        let mut gp = GaussianProcess::new(1.0, 1.0, 0.1).unwrap();
        assert!(gp.predict(&[0.0]).is_err());
        assert!(gp.fit(&[], &[]).is_err());
        assert!(gp.fit(&[vec![1.0]], &[1.0, 2.0]).is_err());
    }

    #[test]
    fn test_acquisition_orientation() {
        // For maximisation, higher mean yields higher EI.
        let ei = AcquisitionFunction::ExpectedImprovement { xi: 0.0 };
        let high = ei.evaluate(2.0, 1.0, 1.0, Objective::Maximize);
        let low = ei.evaluate(0.0, 1.0, 1.0, Objective::Maximize);
        assert!(high > low);
        // Zero std yields zero EI.
        assert_eq!(ei.evaluate(5.0, 0.0, 1.0, Objective::Maximize), 0.0);

        // UCB rewards uncertainty.
        let ucb = AcquisitionFunction::UpperConfidenceBound { beta: 2.0 };
        let certain = ucb.evaluate(1.0, 0.1, 0.0, Objective::Maximize);
        let uncertain = ucb.evaluate(1.0, 1.0, 0.0, Objective::Maximize);
        assert!(uncertain > certain);

        // PI is a probability in [0, 1].
        let pi = AcquisitionFunction::ProbabilityOfImprovement { xi: 0.0 };
        let p = pi.evaluate(1.0, 1.0, 0.5, Objective::Maximize);
        assert!((0.0..=1.0).contains(&p));
    }

    #[test]
    fn test_normal_helpers() {
        assert!((standard_normal_cdf(0.0) - 0.5).abs() < 1e-6);
        assert!(standard_normal_cdf(5.0) > 0.999);
        assert!(standard_normal_cdf(-5.0) < 0.001);
        assert!((erf(0.0)).abs() < 1e-9);
        assert!(
            (standard_normal_pdf(0.0) - 1.0 / (2.0 * std::f64::consts::PI).sqrt()).abs() < 1e-9
        );
    }

    #[test]
    fn test_bayesian_optimizer_minimizes() {
        let mut rng = StdRng::seed_from_u64(123);
        let gp = GaussianProcess::new(0.25, 1.0, 1e-6).unwrap();
        let acquisition = AcquisitionFunction::ExpectedImprovement { xi: 0.01 };
        let mut optimizer =
            BayesianOptimizer::new(space_1d(), gp, acquisition, Objective::Minimize, 200).unwrap();
        // Minimise (x - 2)^2; optimum at x = 2.
        let result = optimizer
            .optimize(5, 25, |p| (p["x"] - 2.0).powi(2), &mut rng)
            .unwrap();
        assert!(
            result.best_objective < 0.5,
            "objective {}",
            result.best_objective
        );
        assert!((result.best_parameters["x"] - 2.0).abs() < 1.0);
        assert_eq!(result.iterations, 30);
        assert_eq!(optimizer.num_observations(), 30);
    }

    #[test]
    fn test_bayesian_optimizer_validation_and_suggest() {
        let mut rng = StdRng::seed_from_u64(1);
        let gp = GaussianProcess::new(1.0, 1.0, 1e-6).unwrap();
        assert!(
            BayesianOptimizer::new(
                ParameterSpace::new(),
                gp.clone(),
                AcquisitionFunction::UpperConfidenceBound { beta: 1.0 },
                Objective::Maximize,
                10
            )
            .is_err()
        );

        let mut optimizer = BayesianOptimizer::new(
            space_1d(),
            gp,
            AcquisitionFunction::UpperConfidenceBound { beta: 1.0 },
            Objective::Maximize,
            10,
        )
        .unwrap();
        // First suggestion (no data) is a valid random point.
        let first = optimizer.suggest(&mut rng).unwrap();
        assert!((-5.0..=5.0).contains(&first["x"]));
        assert!(optimizer.best_parameters().unwrap().is_none());
        optimizer.observe(&first, 1.0).unwrap();
        assert!(optimizer.best_parameters().unwrap().is_some());
    }

    #[test]
    fn test_differential_evolution_minimizes_2d() {
        let mut rng = StdRng::seed_from_u64(77);
        let de =
            DifferentialEvolution::new(space_2d(), Objective::Minimize, 20, 0.9, 0.8, 60).unwrap();
        // Sphere function; optimum at origin.
        let result = de
            .optimize(|p| p["x"] * p["x"] + p["y"] * p["y"], &mut rng)
            .unwrap();
        assert!(
            result.best_objective < 0.1,
            "objective {}",
            result.best_objective
        );
        assert!(result.best_parameters["x"].abs() < 0.5);
        assert!(result.best_parameters["y"].abs() < 0.5);
        assert_eq!(result.history.len(), 60);
    }

    #[test]
    fn test_differential_evolution_validation() {
        let space = space_2d();
        assert!(
            DifferentialEvolution::new(space.clone(), Objective::Minimize, 3, 0.9, 0.8, 10)
                .is_err()
        );
        assert!(
            DifferentialEvolution::new(space.clone(), Objective::Minimize, 10, 1.5, 0.8, 10)
                .is_err()
        );
        assert!(
            DifferentialEvolution::new(space.clone(), Objective::Minimize, 10, 0.9, 3.0, 10)
                .is_err()
        );
        assert!(DifferentialEvolution::new(space, Objective::Minimize, 10, 0.9, 0.8, 0).is_err());
    }
}
