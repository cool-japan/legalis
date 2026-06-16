//! Self-tuning simulation parameters.
//!
//! Two online controllers adapt simulation parameters toward a
//! [`TargetMetric`] while the simulation runs:
//!
//! - [`UcbBanditTuner`] — a UCB1 multi-armed bandit that picks among a discrete
//!   set of candidate configurations, balancing exploration and exploitation
//!   from observed rewards.
//! - [`SimulatedAnnealingTuner`] — a Metropolis simulated-annealing controller
//!   that perturbs a continuous configuration in the unit hypercube and accepts
//!   worsening moves with a temperature-dependent probability, escaping local
//!   optima before cooling to a refined solution.
//!
//! [`SelfTuningController`] wraps either strategy behind one closed-loop
//! [`SelfTuningController::tune`] interface, and both controllers also expose an
//! online `propose` / `observe` pair for hand-driven loops.

use super::{ParameterSpace, standard_normal};
use crate::{SimResult, SimulationError};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// What "good" means for a self-tuned metric.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TargetMetric {
    /// Larger measured values are better.
    Maximize,
    /// Smaller measured values are better.
    Minimize,
    /// Drive the measured value toward a specific target.
    Target(f64),
}

impl TargetMetric {
    /// Converts a measured value into a loss to be **minimised** (lower is better).
    pub fn loss(&self, measured: f64) -> f64 {
        match self {
            TargetMetric::Maximize => -measured,
            TargetMetric::Minimize => measured,
            TargetMetric::Target(t) => (measured - t).abs(),
        }
    }

    /// Converts a measured value into a reward to be **maximised** (higher is better).
    pub fn reward(&self, measured: f64) -> f64 {
        -self.loss(measured)
    }
}

/// A single bandit arm: one candidate configuration and its reward statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanditArm {
    /// The configuration this arm represents.
    pub parameters: HashMap<String, f64>,
    /// Number of times this arm has been pulled.
    pub pulls: usize,
    /// Sum of rewards observed for this arm.
    pub total_reward: f64,
}

impl BanditArm {
    /// Creates a fresh, unpulled arm.
    pub fn new(parameters: HashMap<String, f64>) -> Self {
        Self {
            parameters,
            pulls: 0,
            total_reward: 0.0,
        }
    }

    /// Mean observed reward (`0.0` if never pulled).
    pub fn mean_reward(&self) -> f64 {
        if self.pulls == 0 {
            0.0
        } else {
            self.total_reward / self.pulls as f64
        }
    }
}

/// A UCB1 multi-armed-bandit tuner over a discrete set of configurations.
///
/// Each round it selects the arm maximising
/// `mean_reward + c · sqrt(2 · ln(total_pulls) / arm_pulls)`, trying every arm
/// once before exploiting. Rewards are derived from the [`TargetMetric`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UcbBanditTuner {
    arms: Vec<BanditArm>,
    total_pulls: usize,
    exploration_c: f64,
    target: TargetMetric,
}

impl UcbBanditTuner {
    /// Creates a tuner with the given exploration constant and target.
    pub fn new(target: TargetMetric, exploration_c: f64) -> SimResult<Self> {
        if exploration_c < 0.0 || !exploration_c.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "exploration constant must be non-negative and finite".to_string(),
            ));
        }
        Ok(Self {
            arms: Vec::new(),
            total_pulls: 0,
            exploration_c,
            target,
        })
    }

    /// Adds a candidate configuration as a new arm.
    pub fn add_arm(&mut self, parameters: HashMap<String, f64>) {
        self.arms.push(BanditArm::new(parameters));
    }

    /// Seeds arms from candidate configurations (e.g. from scenario generation).
    pub fn with_arms(mut self, configurations: Vec<HashMap<String, f64>>) -> Self {
        for config in configurations {
            self.add_arm(config);
        }
        self
    }

    /// Returns the arms.
    pub fn arms(&self) -> &[BanditArm] {
        &self.arms
    }

    /// Total number of pulls across all arms.
    pub fn total_pulls(&self) -> usize {
        self.total_pulls
    }

    /// Selects the next arm index by the UCB1 criterion.
    pub fn select(&self) -> SimResult<usize> {
        if self.arms.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "bandit has no arms to select".to_string(),
            ));
        }
        // Untried arms have infinite priority: try each at least once.
        if let Some(idx) = self.arms.iter().position(|a| a.pulls == 0) {
            return Ok(idx);
        }
        let total = self.total_pulls.max(1) as f64;
        let mut best_idx = 0;
        let mut best_score = f64::NEG_INFINITY;
        for (idx, arm) in self.arms.iter().enumerate() {
            let exploration = self.exploration_c * (2.0 * total.ln() / arm.pulls as f64).sqrt();
            let score = arm.mean_reward() + exploration;
            if score > best_score {
                best_score = score;
                best_idx = idx;
            }
        }
        Ok(best_idx)
    }

    /// Records a measured metric for the given arm, updating its statistics.
    pub fn record(&mut self, arm_index: usize, measured: f64) -> SimResult<()> {
        let reward = self.target.reward(measured);
        let arm = self.arms.get_mut(arm_index).ok_or_else(|| {
            SimulationError::InvalidParameter(format!("arm index {arm_index} out of range"))
        })?;
        arm.pulls += 1;
        arm.total_reward += reward;
        self.total_pulls += 1;
        Ok(())
    }

    /// Returns the arm with the highest mean reward, if any have been pulled.
    pub fn best_arm(&self) -> Option<&BanditArm> {
        self.arms.iter().filter(|a| a.pulls > 0).max_by(|a, b| {
            a.mean_reward()
                .partial_cmp(&b.mean_reward())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Runs a closed tuning loop, evaluating the selected arm each round.
    ///
    /// `eval_fn` maps a configuration to the measured metric value.
    pub fn tune<F>(&mut self, rounds: usize, mut eval_fn: F) -> SimResult<TuningOutcome>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        if self.arms.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "bandit has no arms to tune".to_string(),
            ));
        }
        if rounds == 0 {
            return Err(SimulationError::InvalidParameter(
                "rounds must be greater than zero".to_string(),
            ));
        }
        let mut outcome = TuningOutcome::empty(self.target);
        for _ in 0..rounds {
            let idx = self.select()?;
            let params = self.arms[idx].parameters.clone();
            let measured = eval_fn(&params);
            self.record(idx, measured)?;
            outcome.observe(params, measured);
        }
        Ok(outcome)
    }
}

/// A Metropolis simulated-annealing tuner over a continuous parameter space.
///
/// State lives in the unit hypercube; each proposal is a Gaussian perturbation
/// (scaled by the current step size) of the current point, clamped to `[0, 1]`.
/// A worsening move is accepted with probability `exp(-Δloss / temperature)`,
/// and the temperature decays geometrically toward a floor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedAnnealingTuner {
    space: ParameterSpace,
    target: TargetMetric,
    current_unit: Vec<f64>,
    current_loss: f64,
    best_unit: Vec<f64>,
    best_loss: f64,
    best_metric: f64,
    temperature: f64,
    initial_temperature: f64,
    cooling_rate: f64,
    min_temperature: f64,
    step_scale: f64,
    initialized: bool,
}

impl SimulatedAnnealingTuner {
    /// Creates an annealing tuner starting at the centre of `space`.
    pub fn new(
        space: ParameterSpace,
        target: TargetMetric,
        initial_temperature: f64,
        cooling_rate: f64,
        step_scale: f64,
    ) -> SimResult<Self> {
        if space.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "annealing tuner requires a non-empty parameter space".to_string(),
            ));
        }
        if initial_temperature <= 0.0 || !initial_temperature.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "initial temperature must be positive and finite".to_string(),
            ));
        }
        if !(0.0..1.0).contains(&cooling_rate) {
            return Err(SimulationError::InvalidParameter(
                "cooling rate must lie in [0, 1)".to_string(),
            ));
        }
        if step_scale <= 0.0 || !step_scale.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "step scale must be positive and finite".to_string(),
            ));
        }
        let current_unit = vec![0.5; space.dimensions()];
        Ok(Self {
            best_unit: current_unit.clone(),
            current_unit,
            space,
            target,
            current_loss: f64::INFINITY,
            best_loss: f64::INFINITY,
            best_metric: f64::NAN,
            temperature: initial_temperature,
            initial_temperature,
            cooling_rate,
            min_temperature: initial_temperature * 1e-4,
            step_scale,
            initialized: false,
        })
    }

    /// Returns the current temperature.
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Returns the best measured metric so far (`NaN` before any observation).
    pub fn best_metric(&self) -> f64 {
        self.best_metric
    }

    /// Returns the best configuration found so far.
    pub fn best_parameters(&self) -> SimResult<HashMap<String, f64>> {
        self.space.denormalize_named(&self.best_unit)
    }

    /// Shrinks the proposal step size by `factor` (used by self-healing recovery).
    pub fn scale_step(&mut self, factor: f64) {
        if factor > 0.0 && factor.is_finite() {
            self.step_scale *= factor;
        }
    }

    /// Proposes the next candidate configuration to evaluate.
    pub fn propose<R: RngExt>(&self, rng: &mut R) -> SimResult<HashMap<String, f64>> {
        let candidate: Vec<f64> = self
            .current_unit
            .iter()
            .map(|&u| (u + self.step_scale * standard_normal(rng)).clamp(0.0, 1.0))
            .collect();
        self.space.denormalize_named(&candidate)
    }

    /// Incorporates the measured metric for the most recently proposed `candidate`.
    ///
    /// Applies the Metropolis acceptance rule against the current loss, updates the
    /// incumbent best, and cools the temperature.
    pub fn observe<R: RngExt>(
        &mut self,
        candidate: &HashMap<String, f64>,
        measured: f64,
        rng: &mut R,
    ) -> SimResult<bool> {
        let candidate_unit = self.space.normalize_named(candidate);
        let loss = self.target.loss(measured);

        let accepted = if !self.initialized {
            // First observation seeds the incumbent state.
            self.initialized = true;
            true
        } else if loss <= self.current_loss {
            true
        } else {
            let delta = loss - self.current_loss;
            let probability = (-delta / self.temperature).exp();
            rng.random_range(0.0..1.0) < probability
        };

        if accepted {
            self.current_unit = candidate_unit.clone();
            self.current_loss = loss;
        }

        if loss < self.best_loss || self.best_metric.is_nan() {
            self.best_loss = loss;
            self.best_unit = candidate_unit;
            self.best_metric = measured;
        }

        // Cool toward the floor.
        self.temperature = (self.temperature * self.cooling_rate).max(self.min_temperature);
        Ok(accepted)
    }

    /// Resets the temperature to its initial value (re-heating for a new phase).
    pub fn reheat(&mut self) {
        self.temperature = self.initial_temperature;
    }

    /// Runs a closed annealing loop for `steps` evaluations.
    pub fn tune<F, R: RngExt>(
        &mut self,
        steps: usize,
        mut eval_fn: F,
        rng: &mut R,
    ) -> SimResult<TuningOutcome>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        if steps == 0 {
            return Err(SimulationError::InvalidParameter(
                "steps must be greater than zero".to_string(),
            ));
        }
        let mut outcome = TuningOutcome::empty(self.target);
        for _ in 0..steps {
            let candidate = self.propose(rng)?;
            let measured = eval_fn(&candidate);
            self.observe(&candidate, measured, rng)?;
            outcome.observe(candidate, measured);
        }
        outcome.best_parameters = self.best_parameters()?;
        outcome.best_metric = self.best_metric;
        outcome.best_loss = self.best_loss;
        Ok(outcome)
    }
}

/// The result of a closed-loop self-tuning run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningOutcome {
    /// Best configuration found.
    pub best_parameters: HashMap<String, f64>,
    /// Measured metric at the best configuration.
    pub best_metric: f64,
    /// Loss of the best configuration (under the [`TargetMetric`]).
    pub best_loss: f64,
    /// Number of evaluations performed.
    pub evaluations: usize,
    /// History of `(configuration, measured metric)` per evaluation.
    pub history: Vec<(HashMap<String, f64>, f64)>,
    /// The optimisation target used.
    pub target: TargetMetric,
}

impl TuningOutcome {
    fn empty(target: TargetMetric) -> Self {
        Self {
            best_parameters: HashMap::new(),
            best_metric: f64::NAN,
            best_loss: f64::INFINITY,
            evaluations: 0,
            history: Vec::new(),
            target,
        }
    }

    fn observe(&mut self, params: HashMap<String, f64>, measured: f64) {
        let loss = self.target.loss(measured);
        if loss < self.best_loss || self.best_metric.is_nan() {
            self.best_loss = loss;
            self.best_metric = measured;
            self.best_parameters = params.clone();
        }
        self.history.push((params, measured));
        self.evaluations += 1;
    }
}

/// The tuning strategy backing a [`SelfTuningController`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TuningStrategy {
    /// UCB1 multi-armed bandit over discrete configurations.
    Bandit(UcbBanditTuner),
    /// Metropolis simulated annealing over a continuous space.
    Annealing(SimulatedAnnealingTuner),
}

/// A unified self-tuning controller over either tuning strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfTuningController {
    strategy: TuningStrategy,
}

impl SelfTuningController {
    /// Builds a controller backed by a UCB bandit.
    pub fn bandit(tuner: UcbBanditTuner) -> Self {
        Self {
            strategy: TuningStrategy::Bandit(tuner),
        }
    }

    /// Builds a controller backed by simulated annealing.
    pub fn annealing(tuner: SimulatedAnnealingTuner) -> Self {
        Self {
            strategy: TuningStrategy::Annealing(tuner),
        }
    }

    /// Returns a reference to the underlying strategy.
    pub fn strategy(&self) -> &TuningStrategy {
        &self.strategy
    }

    /// Runs a closed-loop tuning campaign for `iterations` evaluations.
    ///
    /// The `rng` is consumed by the annealing strategy; the bandit strategy is
    /// deterministic and ignores it.
    pub fn tune<F, R: RngExt>(
        &mut self,
        iterations: usize,
        eval_fn: F,
        rng: &mut R,
    ) -> SimResult<TuningOutcome>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        match &mut self.strategy {
            TuningStrategy::Bandit(tuner) => tuner.tune(iterations, eval_fn),
            TuningStrategy::Annealing(tuner) => tuner.tune(iterations, eval_fn, rng),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    fn space() -> ParameterSpace {
        ParameterSpace::new()
            .with_dimension("x", -10.0, 10.0)
            .unwrap()
    }

    #[test]
    fn test_target_metric_loss_and_reward() {
        assert!((TargetMetric::Maximize.loss(3.0) + 3.0).abs() < 1e-12);
        assert!((TargetMetric::Minimize.loss(3.0) - 3.0).abs() < 1e-12);
        assert!((TargetMetric::Target(5.0).loss(3.0) - 2.0).abs() < 1e-12);
        assert!((TargetMetric::Maximize.reward(3.0) - 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_bandit_tries_each_arm_then_exploits() {
        let mut tuner = UcbBanditTuner::new(TargetMetric::Maximize, 1.0).unwrap();
        let mut a = HashMap::new();
        a.insert("x".to_string(), 1.0);
        let mut b = HashMap::new();
        b.insert("x".to_string(), 9.0);
        tuner.add_arm(a);
        tuner.add_arm(b);

        // Each arm tried once before any exploitation.
        let first = tuner.select().unwrap();
        tuner
            .record(first, if first == 0 { 1.0 } else { 9.0 })
            .unwrap();
        let second = tuner.select().unwrap();
        assert_ne!(first, second);
        tuner
            .record(second, if second == 0 { 1.0 } else { 9.0 })
            .unwrap();

        // Arm with x=9 has the higher reward; it should dominate selection.
        for _ in 0..20 {
            let idx = tuner.select().unwrap();
            tuner
                .record(idx, tuner.arms()[idx].parameters["x"])
                .unwrap();
        }
        let best = tuner.best_arm().unwrap();
        assert!((best.parameters["x"] - 9.0).abs() < 1e-9);
        assert!(tuner.total_pulls() >= 22);
    }

    #[test]
    fn test_bandit_record_out_of_range_errors() {
        let mut tuner = UcbBanditTuner::new(TargetMetric::Minimize, 1.0).unwrap();
        assert!(tuner.select().is_err());
        let mut p = HashMap::new();
        p.insert("x".to_string(), 0.0);
        tuner.add_arm(p);
        assert!(tuner.record(5, 1.0).is_err());
    }

    #[test]
    fn test_bandit_tune_finds_best_configuration() {
        // Reward peaks at x = 4: maximise -(x-4)^2.
        let configs: Vec<HashMap<String, f64>> = [0.0, 2.0, 4.0, 6.0, 8.0]
            .iter()
            .map(|&v| {
                let mut m = HashMap::new();
                m.insert("x".to_string(), v);
                m
            })
            .collect();
        let mut tuner = UcbBanditTuner::new(TargetMetric::Maximize, 0.5)
            .unwrap()
            .with_arms(configs);
        let outcome = tuner
            .tune(60, |p| {
                let x = p["x"];
                -(x - 4.0) * (x - 4.0)
            })
            .unwrap();
        assert!((outcome.best_parameters["x"] - 4.0).abs() < 1e-9);
        assert_eq!(outcome.evaluations, 60);
    }

    #[test]
    fn test_annealing_validation() {
        assert!(
            SimulatedAnnealingTuner::new(
                ParameterSpace::new(),
                TargetMetric::Minimize,
                1.0,
                0.95,
                0.1
            )
            .is_err()
        );
        assert!(
            SimulatedAnnealingTuner::new(space(), TargetMetric::Minimize, -1.0, 0.95, 0.1).is_err()
        );
        assert!(
            SimulatedAnnealingTuner::new(space(), TargetMetric::Minimize, 1.0, 1.5, 0.1).is_err()
        );
        assert!(
            SimulatedAnnealingTuner::new(space(), TargetMetric::Minimize, 1.0, 0.95, 0.0).is_err()
        );
    }

    #[test]
    fn test_annealing_minimizes_quadratic() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut tuner =
            SimulatedAnnealingTuner::new(space(), TargetMetric::Minimize, 5.0, 0.95, 0.2).unwrap();
        // Minimise x^2 (optimum at x = 0).
        let outcome = tuner.tune(400, |p| p["x"] * p["x"], &mut rng).unwrap();
        assert!(
            outcome.best_metric < 1.0,
            "best metric {}",
            outcome.best_metric
        );
        assert!(outcome.best_parameters["x"].abs() < 1.5);
        assert!(tuner.temperature() < 5.0);
    }

    #[test]
    fn test_annealing_target_metric() {
        let mut rng = StdRng::seed_from_u64(11);
        let mut tuner =
            SimulatedAnnealingTuner::new(space(), TargetMetric::Target(3.0), 4.0, 0.9, 0.25)
                .unwrap();
        // Measured value equals x; drive it toward 3.0.
        let outcome = tuner.tune(400, |p| p["x"], &mut rng).unwrap();
        assert!((outcome.best_metric - 3.0).abs() < 0.5);
    }

    #[test]
    fn test_self_tuning_controller_dispatch() {
        let mut rng = StdRng::seed_from_u64(3);
        let tuner =
            SimulatedAnnealingTuner::new(space(), TargetMetric::Minimize, 3.0, 0.9, 0.2).unwrap();
        let mut controller = SelfTuningController::annealing(tuner);
        let outcome = controller
            .tune(200, |p| (p["x"] - 2.0).powi(2), &mut rng)
            .unwrap();
        assert!((outcome.best_parameters["x"] - 2.0).abs() < 1.5);
        assert!(matches!(
            controller.strategy(),
            TuningStrategy::Annealing(_)
        ));
    }
}
