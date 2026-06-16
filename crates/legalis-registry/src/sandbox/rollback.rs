//! Rollback-safe statute testing.
//!
//! The [`RollbackSafeTester`] runs mutations against a [`SandboxEnvironment`]
//! inside a transactional boundary backed by [`super::environment::SandboxCheckpoint`]s.
//! Two modes are supported:
//! - [`RollbackSafeTester::apply_then_discard`] always rolls the sandbox back to
//!   its pre-test state, so the test is purely observational (ideal for impact
//!   prediction that must leave the environment untouched);
//! - [`RollbackSafeTester::try_transaction`] commits the mutations only when the
//!   closure succeeds, rolling back on error.
//!
//! In both modes a SHA-256 integrity digest of the effective state is compared
//! before and after to guarantee that any rollback was byte-exact.

use super::environment::SandboxEnvironment;
use crate::RegistryResult;

/// The outcome of a rollback-safe test.
#[derive(Debug, Clone)]
pub struct RollbackOutcome<T> {
    /// Whether the mutations were committed (`true`) or discarded (`false`).
    pub committed: bool,
    /// The value produced by the test closure, if it succeeded.
    pub value: Option<T>,
    /// The error message produced by the test closure, if it failed.
    pub error: Option<String>,
    /// Whether the post-rollback integrity digest matched the pre-test digest.
    ///
    /// For committed transactions this is trivially `true` (nothing was rolled
    /// back); for discarded runs it confirms the rollback restored the exact
    /// prior state.
    pub integrity_verified: bool,
    /// Whether the closure changed the effective state while running.
    pub mutated_during_test: bool,
    /// Integrity digest of the effective state before the test.
    pub before_digest: String,
    /// Integrity digest of the effective state after the test resolved.
    pub after_digest: String,
}

impl<T> RollbackOutcome<T> {
    /// Returns whether the test closure succeeded.
    #[must_use]
    pub fn succeeded(&self) -> bool {
        self.error.is_none()
    }

    /// Returns a reference to the produced value, if any.
    #[must_use]
    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

/// Executes mutations against a sandbox within a transactional, integrity-checked boundary.
#[derive(Debug, Clone, Default)]
pub struct RollbackSafeTester;

impl RollbackSafeTester {
    /// Creates a new rollback-safe tester.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Runs a closure against the sandbox and then *always* rolls back.
    ///
    /// This is the apply-then-discard mode: the closure may freely stage and
    /// remove statutes, but the sandbox is guaranteed to be returned to its
    /// exact prior state afterwards (verified by integrity digest), regardless
    /// of whether the closure succeeded or failed.
    ///
    /// # Errors
    ///
    /// Propagates integrity/checkpoint errors from the sandbox itself (the test
    /// closure's own error is captured in the returned outcome, not returned
    /// here).
    pub fn apply_then_discard<F, T>(
        &self,
        env: &mut SandboxEnvironment,
        test: F,
    ) -> RegistryResult<RollbackOutcome<T>>
    where
        F: FnOnce(&mut SandboxEnvironment) -> RegistryResult<T>,
    {
        let checkpoint = env.checkpoint()?;
        let result = test(env);
        let post_run_digest = env.integrity_digest()?;
        let mutated = post_run_digest != checkpoint.digest;

        env.restore(&checkpoint)?;
        let restored_digest = env.integrity_digest()?;
        let integrity_verified = restored_digest == checkpoint.digest;

        let (value, error) = match result {
            Ok(value) => (Some(value), None),
            Err(err) => (None, Some(err.to_string())),
        };

        Ok(RollbackOutcome {
            committed: false,
            value,
            error,
            integrity_verified,
            mutated_during_test: mutated,
            before_digest: checkpoint.digest,
            after_digest: restored_digest,
        })
    }

    /// Runs a closure as a transaction: commit on success, roll back on error.
    ///
    /// When the closure returns `Ok`, the mutations are kept and the outcome is
    /// marked committed. When it returns `Err`, the sandbox is restored to its
    /// pre-transaction state with an integrity check.
    ///
    /// # Errors
    ///
    /// Propagates integrity/checkpoint errors from the sandbox itself.
    pub fn try_transaction<F, T>(
        &self,
        env: &mut SandboxEnvironment,
        test: F,
    ) -> RegistryResult<RollbackOutcome<T>>
    where
        F: FnOnce(&mut SandboxEnvironment) -> RegistryResult<T>,
    {
        let checkpoint = env.checkpoint()?;
        match test(env) {
            Ok(value) => {
                let after_digest = env.integrity_digest()?;
                let mutated = after_digest != checkpoint.digest;
                Ok(RollbackOutcome {
                    committed: true,
                    value: Some(value),
                    error: None,
                    integrity_verified: true,
                    mutated_during_test: mutated,
                    before_digest: checkpoint.digest,
                    after_digest,
                })
            }
            Err(err) => {
                env.restore(&checkpoint)?;
                let restored_digest = env.integrity_digest()?;
                let integrity_verified = restored_digest == checkpoint.digest;
                Ok(RollbackOutcome {
                    committed: false,
                    value: None,
                    error: Some(err.to_string()),
                    integrity_verified,
                    mutated_during_test: false,
                    before_digest: checkpoint.digest,
                    after_digest: restored_digest,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandbox::environment::{IsolationLevel, SandboxEnvironment};
    use crate::{RegistryError, StatuteEntry, StatuteRegistry};
    use legalis_core::{Effect, EffectType, Statute};

    fn sample_registry() -> StatuteRegistry {
        let mut registry = StatuteRegistry::new();
        for idx in 0..3 {
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

    fn candidate(id: &str) -> StatuteEntry {
        let statute = Statute::new(id, "Candidate", Effect::new(EffectType::Obligation, "duty"));
        StatuteEntry::new(statute, "US")
    }

    #[test]
    fn test_apply_then_discard_reverts_changes() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        let tester = RollbackSafeTester::new();
        let outcome = tester
            .apply_then_discard(&mut env, |sandbox| {
                sandbox.stage(candidate("statute-new"))?;
                sandbox.remove("statute-0")?;
                Ok(sandbox.count())
            })
            .expect("test runs");
        // Closure observed the mutated count (3 - 1 removed + 1 added = 3).
        assert_eq!(outcome.value(), Some(&3));
        assert!(outcome.succeeded());
        assert!(!outcome.committed);
        assert!(outcome.mutated_during_test);
        assert!(outcome.integrity_verified);
        // Environment fully restored.
        assert_eq!(env.count(), 3);
        assert!(!env.contains("statute-new"));
        assert!(env.contains("statute-0"));
    }

    #[test]
    fn test_apply_then_discard_restores_after_error() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        let tester = RollbackSafeTester::new();
        let outcome: RollbackOutcome<()> = tester
            .apply_then_discard(&mut env, |sandbox| {
                sandbox.stage(candidate("statute-new"))?;
                Err(RegistryError::InvalidOperation("boom".to_string()))
            })
            .expect("test runs");
        assert!(!outcome.succeeded());
        assert!(outcome.error.is_some());
        assert!(outcome.integrity_verified);
        assert_eq!(env.count(), 3);
        assert!(!env.contains("statute-new"));
    }

    #[test]
    fn test_try_transaction_commits_on_success() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        let tester = RollbackSafeTester::new();
        let outcome = tester
            .try_transaction(&mut env, |sandbox| {
                sandbox.stage(candidate("statute-new"))?;
                Ok(())
            })
            .expect("test runs");
        assert!(outcome.committed);
        assert!(outcome.mutated_during_test);
        // Mutations persisted.
        assert!(env.contains("statute-new"));
        assert_eq!(env.count(), 4);
    }

    #[test]
    fn test_try_transaction_rolls_back_on_error() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        let tester = RollbackSafeTester::new();
        let outcome: RollbackOutcome<()> = tester
            .try_transaction(&mut env, |sandbox| {
                sandbox.stage(candidate("statute-new"))?;
                sandbox.remove("statute-1")?;
                Err(RegistryError::InvalidOperation("rollback".to_string()))
            })
            .expect("test runs");
        assert!(!outcome.committed);
        assert!(outcome.integrity_verified);
        // Rolled back to original state.
        assert_eq!(env.count(), 3);
        assert!(!env.contains("statute-new"));
        assert!(env.contains("statute-1"));
    }

    #[test]
    fn test_no_mutation_detected_when_closure_is_read_only() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        let tester = RollbackSafeTester::new();
        let outcome = tester
            .apply_then_discard(&mut env, |sandbox| Ok(sandbox.count()))
            .expect("test runs");
        assert!(!outcome.mutated_during_test);
        assert!(outcome.integrity_verified);
        assert_eq!(outcome.before_digest, outcome.after_digest);
    }
}
