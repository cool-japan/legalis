//! Self-healing simulation systems.
//!
//! Long-running or autonomously-driven simulations can drift into invalid states:
//! producing non-finite metrics (degenerate), exploding without bound (diverged),
//! or freezing on a single value (stalled). This submodule detects those failure
//! modes through declarative [`Invariant`]s and an aggregating [`HealthMonitor`],
//! then repairs them with an escalating ladder of [`RecoveryStrategy`]s applied by
//! a [`SelfHealingController`] — shrinking the search step, perturbing parameters,
//! restoring the last known-good state, or resetting to a safe region — and
//! restarts the run from the corrected configuration.
//!
//! [`HealthMonitor::check_metrics`] additionally validates the crate's real
//! [`SimulationMetrics`] (outcome-count consistency and ratio bounds).

use super::{ParameterSpace, is_improvement, standard_normal, worst_objective_value};
use crate::{Objective, SimResult, SimulationError, SimulationMetrics};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A declarative invariant the monitored metric stream must satisfy.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum InvariantKind {
    /// The latest value must be finite (rules out `NaN` / `±∞`).
    Finite,
    /// The latest value must lie within `[min, max]`.
    InRange {
        /// Inclusive lower bound.
        min: f64,
        /// Inclusive upper bound.
        max: f64,
    },
    /// The latest value must be non-negative.
    NonNegative,
    /// Consecutive values must not jump by more than `max_delta` (divergence).
    MaxStep {
        /// Largest tolerated absolute step between consecutive values.
        max_delta: f64,
    },
    /// The magnitude of the latest value must not exceed `max_abs` (explosion).
    BoundedMagnitude {
        /// Largest tolerated absolute value.
        max_abs: f64,
    },
    /// The values must vary by at least `min_change` over the last `window`
    /// samples (anti-stagnation).
    NonStagnant {
        /// Number of trailing samples examined.
        window: usize,
        /// Minimum tolerated range (`max - min`) over the window.
        min_change: f64,
    },
}

impl InvariantKind {
    /// Maps a violation of this invariant to a [`HealthStatus`] category.
    fn to_status(self, reason: String) -> HealthStatus {
        match self {
            InvariantKind::Finite | InvariantKind::InRange { .. } | InvariantKind::NonNegative => {
                HealthStatus::Degenerate(reason)
            }
            InvariantKind::MaxStep { .. } | InvariantKind::BoundedMagnitude { .. } => {
                HealthStatus::Diverged(reason)
            }
            InvariantKind::NonStagnant { .. } => HealthStatus::Stalled(reason),
        }
    }
}

/// A named invariant over the monitored metric history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invariant {
    /// Human-readable invariant name.
    pub name: String,
    /// The invariant rule.
    pub kind: InvariantKind,
}

impl Invariant {
    /// Creates a named invariant.
    pub fn new(name: impl Into<String>, kind: InvariantKind) -> Self {
        Self {
            name: name.into(),
            kind,
        }
    }

    /// Evaluates the invariant against the full metric `history`, returning a
    /// violation if the most recent state breaks the rule.
    pub fn evaluate(&self, history: &[f64]) -> Option<InvariantViolation> {
        let last = *history.last()?;
        let index = history.len() - 1;
        let violation = |reason: String| {
            Some(InvariantViolation {
                invariant: self.name.clone(),
                reason,
                index,
                value: last,
            })
        };
        match self.kind {
            InvariantKind::Finite => {
                if !last.is_finite() {
                    return violation(format!("value {last} is not finite"));
                }
            }
            InvariantKind::InRange { min, max } => {
                if last < min || last > max {
                    return violation(format!("value {last} outside [{min}, {max}]"));
                }
            }
            InvariantKind::NonNegative => {
                if last < 0.0 {
                    return violation(format!("value {last} is negative"));
                }
            }
            InvariantKind::MaxStep { max_delta } => {
                if history.len() >= 2 {
                    let prev = history[history.len() - 2];
                    let step = (last - prev).abs();
                    if step > max_delta {
                        return violation(format!("step {step} exceeds maximum {max_delta}"));
                    }
                }
            }
            InvariantKind::BoundedMagnitude { max_abs } => {
                if last.abs() > max_abs {
                    return violation(format!("magnitude {} exceeds {max_abs}", last.abs()));
                }
            }
            InvariantKind::NonStagnant { window, min_change } => {
                if window >= 2 && history.len() >= window {
                    let slice = &history[history.len() - window..];
                    let max = slice.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    let min = slice.iter().copied().fold(f64::INFINITY, f64::min);
                    if (max - min) < min_change {
                        return violation(format!(
                            "range {} over {window} samples below {min_change}",
                            max - min
                        ));
                    }
                }
            }
        }
        None
    }
}

/// A detected invariant violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantViolation {
    /// Name of the violated invariant.
    pub invariant: String,
    /// Human-readable explanation.
    pub reason: String,
    /// Index in the metric history where the violation occurred.
    pub index: usize,
    /// The offending value.
    pub value: f64,
}

/// The health classification of a monitored run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// All invariants satisfied.
    Healthy,
    /// A degenerate state (non-finite or out-of-range value).
    Degenerate(String),
    /// A diverging state (explosion or excessive step).
    Diverged(String),
    /// A stalled state (stagnation).
    Stalled(String),
}

impl HealthStatus {
    /// Returns whether the status is [`HealthStatus::Healthy`].
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }
}

/// Aggregates invariants over a streaming metric history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitor {
    invariants: Vec<Invariant>,
    history: Vec<f64>,
    last_status: HealthStatus,
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self {
            invariants: Vec::new(),
            history: Vec::new(),
            last_status: HealthStatus::Healthy,
        }
    }
}

impl HealthMonitor {
    /// Creates an empty monitor with no invariants.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a monitor with standard invariants: finiteness and bounded
    /// magnitude (explosion detection).
    pub fn standard() -> Self {
        Self::new()
            .with_invariant(Invariant::new("finite", InvariantKind::Finite))
            .with_invariant(Invariant::new(
                "bounded",
                InvariantKind::BoundedMagnitude { max_abs: 1e12 },
            ))
    }

    /// Adds an invariant, returning `self` for chaining.
    pub fn with_invariant(mut self, invariant: Invariant) -> Self {
        self.invariants.push(invariant);
        self
    }

    /// Adds an invariant in place.
    pub fn add_invariant(&mut self, invariant: Invariant) {
        self.invariants.push(invariant);
    }

    /// Returns the recorded metric history.
    pub fn history(&self) -> &[f64] {
        &self.history
    }

    /// Returns the most recently computed status.
    pub fn current_status(&self) -> &HealthStatus {
        &self.last_status
    }

    /// Clears the history and resets the status to healthy.
    pub fn reset(&mut self) {
        self.history.clear();
        self.last_status = HealthStatus::Healthy;
    }

    /// Records a metric sample and returns the resulting health status.
    ///
    /// Invariants are evaluated in registration order; the first violation
    /// determines the status, so order them most-severe first.
    pub fn push(&mut self, value: f64) -> HealthStatus {
        self.history.push(value);
        let mut status = HealthStatus::Healthy;
        for invariant in &self.invariants {
            if let Some(violation) = invariant.evaluate(&self.history) {
                status = invariant.kind.to_status(violation.reason);
                break;
            }
        }
        self.last_status = status.clone();
        status
    }

    /// Validates the crate's [`SimulationMetrics`], returning any violations.
    ///
    /// Checks that the outcome counts sum to the total and that the derived
    /// ratios are finite and within `[0, 1]`.
    pub fn check_metrics(metrics: &SimulationMetrics) -> Vec<InvariantViolation> {
        let mut violations = Vec::new();
        let sum = metrics.deterministic_count + metrics.discretion_count + metrics.void_count;
        if sum != metrics.total_applications {
            violations.push(InvariantViolation {
                invariant: "outcome_count_consistency".to_string(),
                reason: format!(
                    "deterministic+discretion+void ({sum}) != total_applications ({})",
                    metrics.total_applications
                ),
                index: 0,
                value: sum as f64,
            });
        }
        for (name, ratio) in [
            ("deterministic_ratio", metrics.deterministic_ratio()),
            ("discretion_ratio", metrics.discretion_ratio()),
        ] {
            if !ratio.is_finite() || !(0.0..=1.0).contains(&ratio) {
                violations.push(InvariantViolation {
                    invariant: name.to_string(),
                    reason: format!("{name} = {ratio} is outside [0, 1]"),
                    index: 0,
                    value: ratio,
                });
            }
        }
        violations
    }
}

/// A corrective action taken in response to a failing run.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RecoveryStrategy {
    /// Shrink the search/perturbation step by `factor` and restore a safe state.
    ReduceStep {
        /// Multiplicative shrink factor (`0 < factor < 1`).
        factor: f64,
    },
    /// Perturb the current parameters by Gaussian noise of the given unit-space `scale`.
    PerturbParameters {
        /// Unit-space perturbation scale.
        scale: f64,
    },
    /// Restore the last known-good configuration (or the centre if none).
    RestoreCheckpoint,
    /// Reset to the geometric centre of the parameter space.
    ResetToCenter,
    /// Project the current parameters back inside the bounds.
    ClampToBounds,
}

/// The outcome of [`SelfHealingController::observe`] for one step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingAction {
    /// The recovery strategy applied (`None` when the run was healthy).
    pub strategy: Option<RecoveryStrategy>,
    /// The configuration to use next (corrected if a restart was triggered).
    pub corrected_parameters: HashMap<String, f64>,
    /// Whether the run should restart from the corrected configuration.
    pub restart: bool,
    /// The detected health status.
    pub status: HealthStatus,
    /// Human-readable description of the action.
    pub message: String,
}

/// A recorded healing intervention during a [`SelfHealingController::run`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingIncident {
    /// The step at which the incident occurred.
    pub step: usize,
    /// The detected status.
    pub status: HealthStatus,
    /// The recovery strategy applied.
    pub strategy: RecoveryStrategy,
    /// Description of the intervention.
    pub message: String,
}

/// Summary of a self-healing run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingReport {
    /// Number of steps executed.
    pub steps: usize,
    /// Number of restarts triggered.
    pub restarts: usize,
    /// Recorded healing incidents.
    pub incidents: Vec<HealingIncident>,
    /// Final health status.
    pub final_status: HealthStatus,
    /// Best healthy configuration found, if any.
    pub best_parameters: Option<HashMap<String, f64>>,
    /// Best healthy metric value.
    pub best_metric: f64,
    /// Whether the run recovered (healed at least once and ended healthy).
    pub recovered: bool,
}

/// Drives a simulation, detecting failures and auto-correcting them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfHealingController {
    space: ParameterSpace,
    monitor: HealthMonitor,
    ladder: Vec<RecoveryStrategy>,
    last_good: Option<(HashMap<String, f64>, f64)>,
    perturb_scale: f64,
    consecutive_failures: usize,
    max_restarts: usize,
}

impl SelfHealingController {
    /// Creates a controller over `space` allowing up to `max_restarts` restarts.
    ///
    /// Seeds the standard invariants and a default escalating recovery ladder.
    pub fn new(space: ParameterSpace, max_restarts: usize) -> SimResult<Self> {
        if space.is_empty() {
            return Err(SimulationError::ConfigurationError(
                "self-healing requires a non-empty parameter space".to_string(),
            ));
        }
        Ok(Self {
            space,
            monitor: HealthMonitor::standard(),
            ladder: vec![
                RecoveryStrategy::ReduceStep { factor: 0.5 },
                RecoveryStrategy::PerturbParameters { scale: 0.2 },
                RecoveryStrategy::RestoreCheckpoint,
                RecoveryStrategy::ResetToCenter,
            ],
            last_good: None,
            perturb_scale: 0.1,
            consecutive_failures: 0,
            max_restarts,
        })
    }

    /// Replaces the health monitor.
    pub fn with_monitor(mut self, monitor: HealthMonitor) -> Self {
        self.monitor = monitor;
        self
    }

    /// Replaces the recovery ladder (must be non-empty).
    pub fn with_ladder(mut self, ladder: Vec<RecoveryStrategy>) -> SimResult<Self> {
        if ladder.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "recovery ladder cannot be empty".to_string(),
            ));
        }
        self.ladder = ladder;
        Ok(self)
    }

    /// Sets the base perturbation/random-walk scale.
    pub fn set_perturb_scale(&mut self, scale: f64) {
        if scale > 0.0 && scale.is_finite() {
            self.perturb_scale = scale;
        }
    }

    /// Returns the monitor.
    pub fn monitor(&self) -> &HealthMonitor {
        &self.monitor
    }

    /// Returns the current perturbation scale.
    pub fn perturb_scale(&self) -> f64 {
        self.perturb_scale
    }

    fn checkpoint_or_center(&self) -> HashMap<String, f64> {
        match &self.last_good {
            Some((params, _)) => params.clone(),
            None => self.space.center(),
        }
    }

    fn perturb_named<R: RngExt>(
        &self,
        params: &HashMap<String, f64>,
        scale: f64,
        rng: &mut R,
    ) -> SimResult<HashMap<String, f64>> {
        let unit = self.space.normalize_named(params);
        let perturbed: Vec<f64> = unit
            .iter()
            .map(|&u| (u + scale * standard_normal(rng)).clamp(0.0, 1.0))
            .collect();
        self.space.denormalize_named(&perturbed)
    }

    /// Observes one `(parameters, metric)` step, returning the healing action.
    pub fn observe<R: RngExt>(
        &mut self,
        parameters: HashMap<String, f64>,
        metric: f64,
        rng: &mut R,
    ) -> SimResult<HealingAction> {
        let status = self.monitor.push(metric);
        if status.is_healthy() {
            self.consecutive_failures = 0;
            self.last_good = Some((parameters.clone(), metric));
            return Ok(HealingAction {
                strategy: None,
                corrected_parameters: self.space.clamp_named(&parameters),
                restart: false,
                status,
                message: "healthy".to_string(),
            });
        }

        self.consecutive_failures += 1;
        let idx = (self.consecutive_failures - 1).min(self.ladder.len() - 1);
        let strategy = self.ladder[idx];

        let corrected = match strategy {
            RecoveryStrategy::ReduceStep { factor } => {
                self.perturb_scale *= factor;
                self.checkpoint_or_center()
            }
            RecoveryStrategy::PerturbParameters { scale } => {
                self.perturb_named(&parameters, scale, rng)?
            }
            RecoveryStrategy::RestoreCheckpoint => self.checkpoint_or_center(),
            RecoveryStrategy::ResetToCenter => self.space.center(),
            RecoveryStrategy::ClampToBounds => self.space.clamp_named(&parameters),
        };

        let message = format!("{status:?} -> applied {strategy:?}");
        Ok(HealingAction {
            strategy: Some(strategy),
            corrected_parameters: corrected,
            restart: true,
            status,
            message,
        })
    }

    /// Runs a self-healing closed loop for up to `max_steps`.
    ///
    /// `step_fn` evaluates a configuration into a scalar health metric (it may
    /// return non-finite values, which the monitor will catch). On a healthy step
    /// the controller tracks the best metric (per `objective`) and random-walks to
    /// a new candidate; on a failing step it auto-corrects and restarts.
    pub fn run<F, R: RngExt>(
        &mut self,
        max_steps: usize,
        objective: Objective,
        mut step_fn: F,
        rng: &mut R,
    ) -> SimResult<HealingReport>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        if max_steps == 0 {
            return Err(SimulationError::InvalidParameter(
                "max_steps must be greater than zero".to_string(),
            ));
        }

        let mut current = self.checkpoint_or_center();
        let mut incidents = Vec::new();
        let mut restarts = 0usize;
        let mut best_metric = worst_objective_value(objective);
        let mut best_parameters: Option<HashMap<String, f64>> = None;
        let mut steps = 0usize;

        for step in 0..max_steps {
            steps += 1;
            let metric = step_fn(&current);
            let action = self.observe(current.clone(), metric, rng)?;

            if action.restart {
                restarts += 1;
                incidents.push(HealingIncident {
                    step,
                    status: action.status.clone(),
                    strategy: action.strategy.unwrap_or(RecoveryStrategy::ClampToBounds),
                    message: action.message,
                });
                current = action.corrected_parameters;
                if restarts > self.max_restarts {
                    break;
                }
            } else {
                if is_improvement(objective, metric, best_metric) || best_parameters.is_none() {
                    best_metric = metric;
                    best_parameters = Some(current.clone());
                }
                // Explore a fresh nearby candidate for the next step.
                current = self.perturb_named(&current, self.perturb_scale, rng)?;
            }
        }

        let final_status = self.monitor.current_status().clone();
        let recovered = restarts > 0 && final_status.is_healthy();
        Ok(HealingReport {
            steps,
            restarts,
            incidents,
            final_status,
            best_parameters,
            best_metric,
            recovered,
        })
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
    fn test_invariant_detects_degenerate_and_diverged() {
        let finite = Invariant::new("finite", InvariantKind::Finite);
        assert!(finite.evaluate(&[1.0]).is_none());
        assert!(finite.evaluate(&[f64::NAN]).is_some());
        assert!(finite.evaluate(&[f64::INFINITY]).is_some());

        let step = Invariant::new("step", InvariantKind::MaxStep { max_delta: 5.0 });
        assert!(step.evaluate(&[1.0]).is_none());
        assert!(step.evaluate(&[1.0, 3.0]).is_none());
        assert!(step.evaluate(&[1.0, 100.0]).is_some());

        let mag = Invariant::new("mag", InvariantKind::BoundedMagnitude { max_abs: 10.0 });
        assert!(mag.evaluate(&[-5.0]).is_none());
        assert!(mag.evaluate(&[-50.0]).is_some());
    }

    #[test]
    fn test_invariant_range_and_stagnation() {
        let range = Invariant::new("range", InvariantKind::InRange { min: 0.0, max: 1.0 });
        assert!(range.evaluate(&[0.5]).is_none());
        assert!(range.evaluate(&[2.0]).is_some());

        let nonneg = Invariant::new("nn", InvariantKind::NonNegative);
        assert!(nonneg.evaluate(&[-1.0]).is_some());

        let stag = Invariant::new(
            "stag",
            InvariantKind::NonStagnant {
                window: 3,
                min_change: 0.5,
            },
        );
        // Flat history of identical values is stagnant.
        assert!(stag.evaluate(&[1.0, 1.0, 1.0]).is_some());
        // Varying history is fine.
        assert!(stag.evaluate(&[1.0, 2.0, 3.0]).is_none());
    }

    #[test]
    fn test_health_monitor_status_mapping() {
        let mut monitor = HealthMonitor::standard();
        assert_eq!(monitor.push(1.0), HealthStatus::Healthy);
        assert!(monitor.current_status().is_healthy());
        let status = monitor.push(f64::NAN);
        assert!(matches!(status, HealthStatus::Degenerate(_)));
        assert_eq!(monitor.history().len(), 2);
        monitor.reset();
        assert_eq!(monitor.history().len(), 0);
        assert!(monitor.current_status().is_healthy());
    }

    #[test]
    fn test_check_metrics_reuses_simulation_metrics() {
        let mut metrics = SimulationMetrics::new();
        metrics.total_applications = 100;
        metrics.deterministic_count = 70;
        metrics.discretion_count = 20;
        metrics.void_count = 10;
        assert!(HealthMonitor::check_metrics(&metrics).is_empty());

        // Inconsistent counts are flagged.
        metrics.void_count = 5;
        let violations = HealthMonitor::check_metrics(&metrics);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].invariant, "outcome_count_consistency");
    }

    #[test]
    fn test_controller_heals_healthy_step() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut controller = SelfHealingController::new(space(), 5).unwrap();
        let mut params = HashMap::new();
        params.insert("x".to_string(), 1.0);
        let action = controller.observe(params, 2.0, &mut rng).unwrap();
        assert!(!action.restart);
        assert!(action.strategy.is_none());
        assert!(action.status.is_healthy());
    }

    #[test]
    fn test_controller_escalates_recovery() {
        let mut rng = StdRng::seed_from_u64(2);
        let mut controller = SelfHealingController::new(space(), 10).unwrap();
        let mut params = HashMap::new();
        params.insert("x".to_string(), 1.0);

        // First failure: ReduceStep (shrinks the perturb scale).
        let scale_before = controller.perturb_scale();
        let a1 = controller
            .observe(params.clone(), f64::NAN, &mut rng)
            .unwrap();
        assert!(a1.restart);
        assert!(matches!(
            a1.strategy,
            Some(RecoveryStrategy::ReduceStep { .. })
        ));
        assert!(controller.perturb_scale() < scale_before);

        // Second consecutive failure escalates to PerturbParameters.
        let a2 = controller.observe(params, f64::NAN, &mut rng).unwrap();
        assert!(matches!(
            a2.strategy,
            Some(RecoveryStrategy::PerturbParameters { .. })
        ));
    }

    #[test]
    fn test_controller_run_recovers_from_divergence() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut controller = SelfHealingController::new(space(), 50).unwrap();
        // The objective is healthy everywhere except a "trap" region near x > 8,
        // which returns NaN; the controller must detect and steer away.
        let report = controller
            .run(
                80,
                Objective::Minimize,
                |p| {
                    let x = p["x"];
                    if x > 8.0 { f64::NAN } else { x * x }
                },
                &mut rng,
            )
            .unwrap();
        assert!(report.steps > 0);
        assert!(report.best_parameters.is_some());
        // A finite best metric was found despite the NaN trap.
        assert!(report.best_metric.is_finite());

        assert!(
            controller
                .run(0, Objective::Minimize, |_| 0.0, &mut rng)
                .is_err()
        );
    }

    #[test]
    fn test_with_ladder_validation() {
        let controller = SelfHealingController::new(space(), 3).unwrap();
        assert!(controller.with_ladder(vec![]).is_err());
        let ok = SelfHealingController::new(space(), 3)
            .unwrap()
            .with_ladder(vec![RecoveryStrategy::ResetToCenter]);
        assert!(ok.is_ok());
    }
}
