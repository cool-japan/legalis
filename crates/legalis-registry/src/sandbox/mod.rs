//! Regulatory Sandbox: statute simulation, impact prediction, A/B testing,
//! experiment tracking, and rollback-safe testing (v0.3.3).
//!
//! This module provides an isolated environment for regulatory experimentation
//! so that candidate statutes can be evaluated without ever mutating the
//! production registry:
//! - [`SandboxEnvironment`] forks a registry snapshot using copy-on-write
//!   overlay semantics so experiments are fully isolated from the base store.
//! - [`ImpactPredictionSandbox`] applies a candidate statute to a sample of
//!   entities and aggregates the predicted legal effects into an
//!   [`ImpactReport`], reusing the `legalis-core` condition engine.
//! - [`AbTest`] splits an entity sample into control and treatment cohorts and
//!   compares statute variants with effect-size and significance statistics.
//! - [`ExperimentRegistry`] tracks regulatory experiments with hypotheses, a
//!   guarded status lifecycle, metrics, and an append-only audit log.
//! - [`RollbackSafeTester`] runs transactional apply-then-discard tests with
//!   cryptographic integrity verification of the restored state.
//!
//! # Example
//!
//! ```
//! use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
//! use legalis_registry::sandbox::{
//!     ImpactPredictionSandbox, IsolationLevel, SandboxManager, SyntheticEntity,
//! };
//! use legalis_registry::{StatuteEntry, StatuteRegistry};
//!
//! let registry = StatuteRegistry::new();
//! let mut manager = SandboxManager::new();
//! let env_id = manager.create_environment("pilot", &registry, IsolationLevel::CopyOnWrite);
//!
//! // Stage a candidate statute that only lives inside the sandbox.
//! let candidate = Statute::new(
//!     "subsidy-2026",
//!     "Senior subsidy",
//!     Effect::new(EffectType::MonetaryTransfer, "subsidy").with_parameter("amount", "150"),
//! )
//! .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65));
//! let env = manager.environment_mut(env_id).expect("environment exists");
//! env.stage(StatuteEntry::new(candidate, "US")).expect("stage candidate");
//!
//! // Predict its impact on a synthetic population.
//! let entities = vec![
//!     SyntheticEntity::new("a").with_attribute("age", "70"),
//!     SyntheticEntity::new("b").with_attribute("age", "40"),
//! ];
//! let predictor = ImpactPredictionSandbox::with_default_model();
//! let env = manager.environment(env_id).expect("environment exists");
//! let report = predictor.predict(env, "subsidy-2026", &entities, None).expect("prediction");
//! assert_eq!(report.affected_count, 1);
//! ```

mod ab_test;
mod environment;
mod experiment;
mod impact;
mod rollback;

pub use ab_test::{
    AbTest, AbTestResult, CohortArm, DEFAULT_ALPHA, StatuteVariant, cohens_d, erf, mean,
    normal_cdf, sample_variance, two_proportion_z, two_sided_p_value,
};
pub use environment::{
    BaseLayer, IsolationLevel, SandboxCheckpoint, SandboxDiff, SandboxEnvironment,
};
pub use experiment::{
    Experiment, ExperimentLogEntry, ExperimentRegistry, ExperimentStatus, Hypothesis,
    HypothesisOutcome, MetricDirection,
};
pub use impact::{
    CohortImpact, EntityImpact, ImpactModel, ImpactPredictionSandbox, ImpactReport, SyntheticEntity,
};
pub use rollback::{RollbackOutcome, RollbackSafeTester};

use std::collections::HashMap;

use uuid::Uuid;

use crate::StatuteRegistry;

/// Top-level coordinator for sandbox environments and tracked experiments.
///
/// The manager owns a set of live [`SandboxEnvironment`]s keyed by id and an
/// [`ExperimentRegistry`], providing a single entry point for the regulatory
/// experimentation workflow.
#[derive(Debug, Clone, Default)]
pub struct SandboxManager {
    environments: HashMap<Uuid, SandboxEnvironment>,
    experiments: ExperimentRegistry,
}

impl SandboxManager {
    /// Creates an empty sandbox manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Forks a new sandbox environment from a registry snapshot.
    ///
    /// Returns the new environment's identifier.
    pub fn create_environment(
        &mut self,
        name: impl Into<String>,
        registry: &StatuteRegistry,
        isolation: IsolationLevel,
    ) -> Uuid {
        let env = SandboxEnvironment::from_registry(name, registry, isolation);
        let id = env.id;
        self.environments.insert(id, env);
        id
    }

    /// Inserts an externally constructed environment, returning its identifier.
    pub fn insert_environment(&mut self, env: SandboxEnvironment) -> Uuid {
        let id = env.id;
        self.environments.insert(id, env);
        id
    }

    /// Returns an environment by identifier.
    #[must_use]
    pub fn environment(&self, id: Uuid) -> Option<&SandboxEnvironment> {
        self.environments.get(&id)
    }

    /// Returns a mutable reference to an environment by identifier.
    pub fn environment_mut(&mut self, id: Uuid) -> Option<&mut SandboxEnvironment> {
        self.environments.get_mut(&id)
    }

    /// Lists all live environments.
    #[must_use]
    pub fn list_environments(&self) -> Vec<&SandboxEnvironment> {
        self.environments.values().collect()
    }

    /// Returns the number of live environments.
    #[must_use]
    pub fn environment_count(&self) -> usize {
        self.environments.len()
    }

    /// Returns `true` when no environments are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.environments.is_empty()
    }

    /// Drops an environment, returning it if present.
    pub fn drop_environment(&mut self, id: Uuid) -> Option<SandboxEnvironment> {
        self.environments.remove(&id)
    }

    /// Returns a shared reference to the experiment registry.
    #[must_use]
    pub fn experiments(&self) -> &ExperimentRegistry {
        &self.experiments
    }

    /// Returns a mutable reference to the experiment registry.
    pub fn experiments_mut(&mut self) -> &mut ExperimentRegistry {
        &mut self.experiments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StatuteEntry, StatuteRegistry};
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn sample_registry() -> StatuteRegistry {
        let mut registry = StatuteRegistry::new();
        for idx in 0..2 {
            let statute = Statute::new(
                format!("statute-{idx}"),
                "Statute",
                Effect::new(EffectType::Grant, "grant"),
            );
            registry
                .register(StatuteEntry::new(statute, "US"))
                .expect("register");
        }
        registry
    }

    #[test]
    fn test_manager_creates_and_lists_environments() {
        let registry = sample_registry();
        let mut manager = SandboxManager::new();
        assert!(manager.is_empty());
        let id = manager.create_environment("pilot", &registry, IsolationLevel::CopyOnWrite);
        assert_eq!(manager.environment_count(), 1);
        assert!(manager.environment(id).is_some());
        assert_eq!(manager.list_environments().len(), 1);
    }

    #[test]
    fn test_manager_drop_environment() {
        let registry = sample_registry();
        let mut manager = SandboxManager::new();
        let id = manager.create_environment("pilot", &registry, IsolationLevel::CopyOnWrite);
        let dropped = manager.drop_environment(id);
        assert!(dropped.is_some());
        assert_eq!(manager.environment_count(), 0);
    }

    #[test]
    fn test_manager_tracks_experiments() {
        let mut manager = SandboxManager::new();
        let hypothesis = Hypothesis::new(
            "Subsidy raises uptake",
            "uptake",
            0.3,
            0.5,
            MetricDirection::Increase,
        );
        let id = manager
            .experiments_mut()
            .register(Experiment::new("exp", "desc", hypothesis));
        manager
            .experiments_mut()
            .transition(id, ExperimentStatus::Running, "alice")
            .expect("transition");
        manager
            .experiments_mut()
            .record_metric(id, "uptake", 0.6)
            .expect("metric");
        let experiment = manager.experiments().get(id).expect("exists");
        assert_eq!(
            experiment.evaluate_hypothesis(),
            HypothesisOutcome::Supported
        );
    }

    #[test]
    fn test_end_to_end_sandbox_workflow() {
        // Build a production registry and an isolated sandbox over it.
        let registry = sample_registry();
        let mut manager = SandboxManager::new();
        let env_id = manager.create_environment("reform", &registry, IsolationLevel::CopyOnWrite);

        // Stage a candidate statute in the sandbox.
        let candidate = Statute::new(
            "reform-1",
            "Income support",
            Effect::new(EffectType::MonetaryTransfer, "support").with_parameter("amount", "300"),
        )
        .with_precondition(Condition::income(ComparisonOp::LessThan, 25000));
        {
            let env = manager.environment_mut(env_id).expect("env exists");
            env.stage(StatuteEntry::new(candidate, "US"))
                .expect("stage");
        }

        // Predict impact within the sandbox.
        let entities = vec![
            SyntheticEntity::new("p1").with_attribute("income", "10000"),
            SyntheticEntity::new("p2").with_attribute("income", "50000"),
            SyntheticEntity::new("p3").with_attribute("income", "20000"),
        ];
        let predictor = ImpactPredictionSandbox::with_default_model();
        let report = {
            let env = manager.environment(env_id).expect("env exists");
            predictor
                .predict(env, "reform-1", &entities, None)
                .expect("prediction")
        };
        assert_eq!(report.affected_count, 2);

        // Production registry remains untouched.
        assert_eq!(registry.count(), 2);
        assert!(!registry.contains("reform-1"));

        // Track the experiment and record the observed coverage.
        let hypothesis = Hypothesis::new(
            "Reform reaches most low-income filers",
            "coverage",
            0.4,
            0.6,
            MetricDirection::Increase,
        );
        let exp_id = manager
            .experiments_mut()
            .register(Experiment::new("reform-eval", "desc", hypothesis).with_sandbox(env_id));
        manager
            .experiments_mut()
            .transition(exp_id, ExperimentStatus::Running, "analyst")
            .expect("transition");
        manager
            .experiments_mut()
            .record_metric(exp_id, "coverage", report.coverage)
            .expect("metric");
        let experiment = manager.experiments().get(exp_id).expect("exists");
        assert_eq!(
            experiment.evaluate_hypothesis(),
            HypothesisOutcome::Supported
        );
    }
}
