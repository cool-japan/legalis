//! Regulatory experiment tracking.
//!
//! The [`ExperimentRegistry`] records regulatory experiments, each carrying a
//! testable [`Hypothesis`], a guarded status lifecycle ([`ExperimentStatus`]),
//! arbitrary numeric metrics, and an append-only audit log. The lifecycle
//! transitions are validated so that experiments can only move through legal
//! states, and every mutation is recorded for auditability.

use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{RegistryError, RegistryResult};

/// The direction in which a metric is expected to move for the hypothesis to hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricDirection {
    /// The metric should increase relative to the baseline.
    Increase,
    /// The metric should decrease relative to the baseline.
    Decrease,
}

/// The result of evaluating a hypothesis against an observed metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HypothesisOutcome {
    /// The observed value reached or passed the target.
    Supported,
    /// The observed value moved against the hypothesis past the baseline.
    Refuted,
    /// The observed value lies between baseline and target.
    Inconclusive,
}

/// A testable statement about a regulatory experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    /// The hypothesis statement (the alternative hypothesis).
    pub statement: String,
    /// The metric name used to evaluate the hypothesis.
    pub success_metric: String,
    /// The baseline (status quo) value of the metric.
    pub baseline_value: f64,
    /// The target value the metric must reach for support.
    pub target_value: f64,
    /// The expected direction of change.
    pub direction: MetricDirection,
}

impl Hypothesis {
    /// Creates a new hypothesis.
    #[must_use]
    pub fn new(
        statement: impl Into<String>,
        success_metric: impl Into<String>,
        baseline_value: f64,
        target_value: f64,
        direction: MetricDirection,
    ) -> Self {
        Self {
            statement: statement.into(),
            success_metric: success_metric.into(),
            baseline_value,
            target_value,
            direction,
        }
    }

    /// Evaluates the hypothesis against an observed metric value.
    #[must_use]
    pub fn evaluate(&self, observed: f64) -> HypothesisOutcome {
        match self.direction {
            MetricDirection::Increase => {
                if observed >= self.target_value {
                    HypothesisOutcome::Supported
                } else if observed <= self.baseline_value {
                    HypothesisOutcome::Refuted
                } else {
                    HypothesisOutcome::Inconclusive
                }
            }
            MetricDirection::Decrease => {
                if observed <= self.target_value {
                    HypothesisOutcome::Supported
                } else if observed >= self.baseline_value {
                    HypothesisOutcome::Refuted
                } else {
                    HypothesisOutcome::Inconclusive
                }
            }
        }
    }
}

/// The lifecycle status of a regulatory experiment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperimentStatus {
    /// Being designed; not yet running.
    Draft,
    /// Actively collecting data.
    Running,
    /// Temporarily suspended.
    Paused,
    /// Finished with a recorded conclusion.
    Completed,
    /// Cancelled before completion.
    Aborted,
}

impl ExperimentStatus {
    /// Returns a stable label for the status.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            ExperimentStatus::Draft => "draft",
            ExperimentStatus::Running => "running",
            ExperimentStatus::Paused => "paused",
            ExperimentStatus::Completed => "completed",
            ExperimentStatus::Aborted => "aborted",
        }
    }

    /// Returns `true` when the status is terminal (no further transitions).
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            ExperimentStatus::Completed | ExperimentStatus::Aborted
        )
    }

    /// Returns whether a transition to `next` is permitted from this status.
    #[must_use]
    pub const fn can_transition_to(&self, next: ExperimentStatus) -> bool {
        matches!(
            (*self, next),
            (ExperimentStatus::Draft, ExperimentStatus::Running)
                | (ExperimentStatus::Draft, ExperimentStatus::Aborted)
                | (ExperimentStatus::Running, ExperimentStatus::Paused)
                | (ExperimentStatus::Running, ExperimentStatus::Completed)
                | (ExperimentStatus::Running, ExperimentStatus::Aborted)
                | (ExperimentStatus::Paused, ExperimentStatus::Running)
                | (ExperimentStatus::Paused, ExperimentStatus::Aborted)
        )
    }
}

/// An append-only audit log entry for an experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentLogEntry {
    /// When the action occurred.
    pub timestamp: DateTime<Utc>,
    /// Who performed the action.
    pub actor: String,
    /// The action category (e.g. `status_transition`, `record_metric`).
    pub action: String,
    /// Free-form detail about the action.
    pub detail: String,
}

/// A tracked regulatory experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    /// Unique experiment identifier.
    pub id: Uuid,
    /// Human-readable experiment name.
    pub name: String,
    /// Description of the experiment.
    pub description: String,
    /// The hypothesis under test.
    pub hypothesis: Hypothesis,
    /// Current lifecycle status.
    pub status: ExperimentStatus,
    /// The experiment owner.
    pub owner: String,
    /// Linked sandbox environment, if any.
    pub sandbox_id: Option<Uuid>,
    /// Recorded numeric metrics.
    pub metrics: BTreeMap<String, f64>,
    /// Tags for categorization.
    pub tags: Vec<String>,
    /// Append-only audit log.
    pub audit_log: Vec<ExperimentLogEntry>,
    /// Final conclusion, set when the experiment completes.
    pub conclusion: Option<String>,
    /// When the experiment was created.
    pub created_at: DateTime<Utc>,
    /// When the experiment was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Experiment {
    /// Creates a new experiment in the [`ExperimentStatus::Draft`] state.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        hypothesis: Hypothesis,
    ) -> Self {
        let now = Utc::now();
        let mut experiment = Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            hypothesis,
            status: ExperimentStatus::Draft,
            owner: "unassigned".to_string(),
            sandbox_id: None,
            metrics: BTreeMap::new(),
            tags: Vec::new(),
            audit_log: Vec::new(),
            conclusion: None,
            created_at: now,
            updated_at: now,
        };
        experiment.append_log("system", "create", "experiment created");
        experiment
    }

    /// Sets the experiment owner (builder style).
    #[must_use]
    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = owner.into();
        self
    }

    /// Links the experiment to a sandbox environment (builder style).
    #[must_use]
    pub fn with_sandbox(mut self, sandbox_id: Uuid) -> Self {
        self.sandbox_id = Some(sandbox_id);
        self
    }

    /// Adds a tag (builder style).
    #[must_use]
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Appends an entry to the audit log.
    pub fn append_log(
        &mut self,
        actor: impl Into<String>,
        action: impl Into<String>,
        detail: impl Into<String>,
    ) {
        self.audit_log.push(ExperimentLogEntry {
            timestamp: Utc::now(),
            actor: actor.into(),
            action: action.into(),
            detail: detail.into(),
        });
    }

    /// Transitions the experiment to a new status, validating the transition.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] when the transition is not
    /// permitted by the lifecycle.
    pub fn transition(
        &mut self,
        next: ExperimentStatus,
        actor: impl Into<String>,
    ) -> RegistryResult<()> {
        if !self.status.can_transition_to(next) {
            return Err(RegistryError::InvalidOperation(format!(
                "invalid status transition from {} to {}",
                self.status.label(),
                next.label()
            )));
        }
        let detail = format!("{} -> {}", self.status.label(), next.label());
        self.status = next;
        self.updated_at = Utc::now();
        self.append_log(actor, "status_transition", detail);
        Ok(())
    }

    /// Records a numeric metric and logs the action.
    pub fn record_metric(&mut self, name: impl Into<String>, value: f64) {
        let name = name.into();
        self.metrics.insert(name.clone(), value);
        self.updated_at = Utc::now();
        self.append_log("system", "record_metric", format!("{name}={value}"));
    }

    /// Returns a recorded metric value by name.
    #[must_use]
    pub fn metric(&self, name: &str) -> Option<f64> {
        self.metrics.get(name).copied()
    }

    /// Concludes the experiment, transitioning it to [`ExperimentStatus::Completed`].
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] when the experiment is not in
    /// a state from which it can be completed.
    pub fn conclude(
        &mut self,
        actor: impl Into<String>,
        summary: impl Into<String>,
    ) -> RegistryResult<()> {
        let actor = actor.into();
        let summary = summary.into();
        self.transition(ExperimentStatus::Completed, actor.clone())?;
        self.conclusion = Some(summary.clone());
        self.append_log(actor, "conclude", summary);
        Ok(())
    }

    /// Evaluates the hypothesis against the recorded success metric.
    ///
    /// Returns [`HypothesisOutcome::Inconclusive`] when the success metric has
    /// not been recorded.
    #[must_use]
    pub fn evaluate_hypothesis(&self) -> HypothesisOutcome {
        match self.metrics.get(&self.hypothesis.success_metric) {
            Some(value) => self.hypothesis.evaluate(*value),
            None => HypothesisOutcome::Inconclusive,
        }
    }
}

/// A registry of regulatory experiments.
#[derive(Debug, Clone, Default)]
pub struct ExperimentRegistry {
    experiments: HashMap<Uuid, Experiment>,
}

impl ExperimentRegistry {
    /// Creates an empty experiment registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an experiment and returns its identifier.
    pub fn register(&mut self, experiment: Experiment) -> Uuid {
        let id = experiment.id;
        self.experiments.insert(id, experiment);
        id
    }

    /// Returns an experiment by identifier.
    #[must_use]
    pub fn get(&self, id: Uuid) -> Option<&Experiment> {
        self.experiments.get(&id)
    }

    /// Returns a mutable reference to an experiment by identifier.
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Experiment> {
        self.experiments.get_mut(&id)
    }

    /// Removes an experiment, returning it if present.
    pub fn remove(&mut self, id: Uuid) -> Option<Experiment> {
        self.experiments.remove(&id)
    }

    /// Lists all experiments.
    #[must_use]
    pub fn list(&self) -> Vec<&Experiment> {
        self.experiments.values().collect()
    }

    /// Returns the number of registered experiments.
    #[must_use]
    pub fn count(&self) -> usize {
        self.experiments.len()
    }

    /// Returns `true` when no experiments are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.experiments.is_empty()
    }

    /// Returns experiments matching the given status.
    #[must_use]
    pub fn by_status(&self, status: ExperimentStatus) -> Vec<&Experiment> {
        self.experiments
            .values()
            .filter(|experiment| experiment.status == status)
            .collect()
    }

    /// Returns experiments carrying the given tag.
    #[must_use]
    pub fn by_tag(&self, tag: &str) -> Vec<&Experiment> {
        self.experiments
            .values()
            .filter(|experiment| experiment.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Transitions an experiment's status by identifier.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::StatuteNotFound`] when the experiment does not
    /// exist, or propagates lifecycle errors from [`Experiment::transition`].
    pub fn transition(
        &mut self,
        id: Uuid,
        next: ExperimentStatus,
        actor: impl Into<String>,
    ) -> RegistryResult<()> {
        let experiment = self
            .experiments
            .get_mut(&id)
            .ok_or_else(|| RegistryError::StatuteNotFound(id.to_string()))?;
        experiment.transition(next, actor)
    }

    /// Records a metric on an experiment by identifier.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::StatuteNotFound`] when the experiment does not exist.
    pub fn record_metric(
        &mut self,
        id: Uuid,
        name: impl Into<String>,
        value: f64,
    ) -> RegistryResult<()> {
        let experiment = self
            .experiments
            .get_mut(&id)
            .ok_or_else(|| RegistryError::StatuteNotFound(id.to_string()))?;
        experiment.record_metric(name, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hypothesis() -> Hypothesis {
        Hypothesis::new(
            "Treatment raises benefit uptake",
            "uptake_rate",
            0.40,
            0.55,
            MetricDirection::Increase,
        )
    }

    #[test]
    fn test_valid_status_lifecycle() {
        let mut experiment = Experiment::new("exp", "desc", sample_hypothesis());
        assert_eq!(experiment.status, ExperimentStatus::Draft);
        experiment
            .transition(ExperimentStatus::Running, "alice")
            .expect("draft -> running");
        experiment
            .transition(ExperimentStatus::Paused, "alice")
            .expect("running -> paused");
        experiment
            .transition(ExperimentStatus::Running, "alice")
            .expect("paused -> running");
        assert_eq!(experiment.status, ExperimentStatus::Running);
    }

    #[test]
    fn test_invalid_status_transition_rejected() {
        let mut experiment = Experiment::new("exp", "desc", sample_hypothesis());
        // Cannot jump straight from Draft to Completed.
        let result = experiment.transition(ExperimentStatus::Completed, "alice");
        assert!(result.is_err());
        assert_eq!(experiment.status, ExperimentStatus::Draft);
    }

    #[test]
    fn test_terminal_status_is_final() {
        let mut experiment = Experiment::new("exp", "desc", sample_hypothesis());
        experiment
            .transition(ExperimentStatus::Aborted, "alice")
            .expect("draft -> aborted");
        assert!(experiment.status.is_terminal());
        let result = experiment.transition(ExperimentStatus::Running, "alice");
        assert!(result.is_err());
    }

    #[test]
    fn test_hypothesis_outcomes_increase() {
        let hypothesis = sample_hypothesis();
        assert_eq!(hypothesis.evaluate(0.60), HypothesisOutcome::Supported);
        assert_eq!(hypothesis.evaluate(0.35), HypothesisOutcome::Refuted);
        assert_eq!(hypothesis.evaluate(0.48), HypothesisOutcome::Inconclusive);
    }

    #[test]
    fn test_hypothesis_outcomes_decrease() {
        let hypothesis = Hypothesis::new(
            "Treatment lowers default rate",
            "default_rate",
            0.20,
            0.10,
            MetricDirection::Decrease,
        );
        assert_eq!(hypothesis.evaluate(0.08), HypothesisOutcome::Supported);
        assert_eq!(hypothesis.evaluate(0.25), HypothesisOutcome::Refuted);
        assert_eq!(hypothesis.evaluate(0.15), HypothesisOutcome::Inconclusive);
    }

    #[test]
    fn test_metrics_and_hypothesis_evaluation() {
        let mut experiment = Experiment::new("exp", "desc", sample_hypothesis());
        // No metric yet -> inconclusive.
        assert_eq!(
            experiment.evaluate_hypothesis(),
            HypothesisOutcome::Inconclusive
        );
        experiment.record_metric("uptake_rate", 0.60);
        assert_eq!(experiment.metric("uptake_rate"), Some(0.60));
        assert_eq!(
            experiment.evaluate_hypothesis(),
            HypothesisOutcome::Supported
        );
    }

    #[test]
    fn test_audit_log_records_actions() {
        let mut experiment = Experiment::new("exp", "desc", sample_hypothesis());
        let initial = experiment.audit_log.len();
        experiment
            .transition(ExperimentStatus::Running, "alice")
            .expect("transition");
        experiment.record_metric("uptake_rate", 0.5);
        assert!(experiment.audit_log.len() > initial + 1);
        let actions: Vec<&str> = experiment
            .audit_log
            .iter()
            .map(|entry| entry.action.as_str())
            .collect();
        assert!(actions.contains(&"status_transition"));
        assert!(actions.contains(&"record_metric"));
    }

    #[test]
    fn test_conclude_sets_conclusion() {
        let mut experiment = Experiment::new("exp", "desc", sample_hypothesis());
        experiment
            .transition(ExperimentStatus::Running, "alice")
            .expect("running");
        experiment
            .conclude("alice", "Treatment outperformed control")
            .expect("conclude");
        assert_eq!(experiment.status, ExperimentStatus::Completed);
        assert_eq!(
            experiment.conclusion.as_deref(),
            Some("Treatment outperformed control")
        );
    }

    #[test]
    fn test_registry_register_and_query() {
        let mut registry = ExperimentRegistry::new();
        let id = registry.register(
            Experiment::new("exp", "desc", sample_hypothesis())
                .with_owner("alice")
                .with_tag("housing"),
        );
        assert_eq!(registry.count(), 1);
        assert!(registry.get(id).is_some());
        assert_eq!(registry.by_status(ExperimentStatus::Draft).len(), 1);
        assert_eq!(registry.by_tag("housing").len(), 1);
        assert_eq!(registry.by_tag("missing").len(), 0);
    }

    #[test]
    fn test_registry_transition_and_metric_by_id() {
        let mut registry = ExperimentRegistry::new();
        let id = registry.register(Experiment::new("exp", "desc", sample_hypothesis()));
        registry
            .transition(id, ExperimentStatus::Running, "alice")
            .expect("transition");
        registry
            .record_metric(id, "uptake_rate", 0.7)
            .expect("metric");
        let experiment = registry.get(id).expect("exists");
        assert_eq!(experiment.status, ExperimentStatus::Running);
        assert_eq!(experiment.metric("uptake_rate"), Some(0.7));
        // Unknown id errors.
        assert!(
            registry
                .transition(Uuid::new_v4(), ExperimentStatus::Running, "x")
                .is_err()
        );
    }
}
