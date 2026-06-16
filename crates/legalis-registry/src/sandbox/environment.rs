//! Isolated statute simulation environments.
//!
//! A [`SandboxEnvironment`] forks a point-in-time view of a production
//! registry and layers a private, copy-on-write overlay on top of it. Reads
//! fall through to the shared immutable base layer unless the statute has been
//! staged or removed inside the sandbox, so experiments can never mutate the
//! production store. Each environment can be checkpointed and restored with
//! cryptographic integrity verification, which is the foundation for the
//! rollback-safe testing facilities in the [`super::rollback`] module.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    BackupMetadata, RegistryBackup, RegistryError, RegistryResult, StatuteEntry, StatuteRegistry,
};

/// Governs how a sandbox relates to the immutable base layer it forks from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolationLevel {
    /// Lazy copy-on-write: reads fall through to the shared base layer and
    /// writes are captured in a private overlay. The most memory-efficient
    /// mode and the default for experimentation.
    CopyOnWrite,
    /// The entire base layer is eagerly materialized into the overlay when the
    /// sandbox is created, producing a fully independent working set that no
    /// longer shares any structure with the base.
    FullCopy,
    /// Observation-only: any attempt to stage or remove a statute is rejected.
    /// Useful for read-only impact analysis against a frozen snapshot.
    ReadOnly,
}

impl IsolationLevel {
    /// Returns `true` when the isolation level forbids mutation.
    #[must_use]
    pub const fn is_read_only(&self) -> bool {
        matches!(self, IsolationLevel::ReadOnly)
    }

    /// Returns a human-readable label for the isolation level.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            IsolationLevel::CopyOnWrite => "copy-on-write",
            IsolationLevel::FullCopy => "full-copy",
            IsolationLevel::ReadOnly => "read-only",
        }
    }
}

/// An immutable, shareable snapshot of a registry's statutes.
///
/// The base layer is wrapped in an [`Arc`] inside a [`SandboxEnvironment`] so
/// that forking a sandbox (or creating many sandboxes from the same registry)
/// shares the base storage without copying it.
#[derive(Debug, Clone)]
pub struct BaseLayer {
    /// Identifier for this captured snapshot.
    pub snapshot_id: Uuid,
    /// When the snapshot was captured.
    pub captured_at: DateTime<Utc>,
    /// Statutes keyed by their statute identifier.
    pub statutes: HashMap<String, StatuteEntry>,
}

impl BaseLayer {
    /// Captures the current contents of a registry into an immutable base layer.
    #[must_use]
    pub fn from_registry(registry: &StatuteRegistry) -> Self {
        let statutes = registry
            .list()
            .into_iter()
            .map(|entry| (entry.statute.id.clone(), entry.clone()))
            .collect();
        Self {
            snapshot_id: Uuid::new_v4(),
            captured_at: Utc::now(),
            statutes,
        }
    }

    /// Creates an empty base layer (no production statutes).
    #[must_use]
    pub fn empty() -> Self {
        Self {
            snapshot_id: Uuid::new_v4(),
            captured_at: Utc::now(),
            statutes: HashMap::new(),
        }
    }

    /// Returns the number of statutes in the base layer.
    #[must_use]
    pub fn count(&self) -> usize {
        self.statutes.len()
    }

    /// Returns `true` when the base layer holds no statutes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }
}

/// Summary of how a sandbox diverges from its base layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SandboxDiff {
    /// Statute identifiers present in the sandbox but absent from the base.
    pub added: Vec<String>,
    /// Statute identifiers present in both, but modified inside the sandbox.
    pub modified: Vec<String>,
    /// Statute identifiers present in the base but removed inside the sandbox.
    pub removed: Vec<String>,
}

impl SandboxDiff {
    /// Returns the total number of divergences from the base layer.
    #[must_use]
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.modified.len() + self.removed.len()
    }

    /// Returns `true` when the sandbox is identical to its base layer.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total_changes() == 0
    }
}

/// A restorable snapshot of a sandbox's mutable state.
///
/// Because the base layer is immutable, a checkpoint only needs to capture the
/// overlay and tombstones plus an integrity digest of the resulting effective
/// state. Restoring re-applies that mutable state and verifies the digest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxCheckpoint {
    /// Identifier for this checkpoint.
    pub checkpoint_id: Uuid,
    /// The environment this checkpoint was taken from.
    pub environment_id: Uuid,
    /// When the checkpoint was captured.
    pub created_at: DateTime<Utc>,
    /// SHA-256 digest of the effective state at checkpoint time.
    pub digest: String,
    /// Captured overlay (added/modified statutes).
    overlay: HashMap<String, StatuteEntry>,
    /// Captured tombstones (removed statute identifiers).
    tombstones: HashSet<String>,
}

impl SandboxCheckpoint {
    /// Number of staged statutes captured in this checkpoint.
    #[must_use]
    pub fn staged_count(&self) -> usize {
        self.overlay.len()
    }

    /// Number of tombstones captured in this checkpoint.
    #[must_use]
    pub fn removed_count(&self) -> usize {
        self.tombstones.len()
    }
}

/// An isolated, copy-on-write fork of a registry for regulatory experimentation.
///
/// The environment is a live runtime object (like [`StatuteRegistry`] itself)
/// and is therefore not serializable; persistable artifacts are produced via
/// [`SandboxEnvironment::checkpoint`], [`SandboxEnvironment::diff_from_base`],
/// and the impact / experiment reports.
#[derive(Debug, Clone)]
pub struct SandboxEnvironment {
    /// Unique environment identifier.
    pub id: Uuid,
    /// Human-readable environment name.
    pub name: String,
    /// When the environment was created.
    pub created_at: DateTime<Utc>,
    /// Isolation semantics for this environment.
    pub isolation: IsolationLevel,
    /// Shared immutable base layer.
    base: Arc<BaseLayer>,
    /// Copy-on-write overlay of staged (added/modified) statutes.
    overlay: HashMap<String, StatuteEntry>,
    /// Tombstones for statutes removed inside the sandbox.
    tombstones: HashSet<String>,
    /// Free-form metadata attached to the environment.
    metadata: HashMap<String, String>,
}

impl SandboxEnvironment {
    /// Forks a new sandbox from a registry snapshot.
    #[must_use]
    pub fn from_registry(
        name: impl Into<String>,
        registry: &StatuteRegistry,
        isolation: IsolationLevel,
    ) -> Self {
        let base = Arc::new(BaseLayer::from_registry(registry));
        Self::from_base(name, base, isolation)
    }

    /// Forks a new sandbox from an already-captured base layer.
    #[must_use]
    pub fn from_base(
        name: impl Into<String>,
        base: Arc<BaseLayer>,
        isolation: IsolationLevel,
    ) -> Self {
        let overlay = if isolation == IsolationLevel::FullCopy {
            base.statutes.clone()
        } else {
            HashMap::new()
        };
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            created_at: Utc::now(),
            isolation,
            base,
            overlay,
            tombstones: HashSet::new(),
            metadata: HashMap::new(),
        }
    }

    /// Creates an empty sandbox with no base statutes.
    #[must_use]
    pub fn empty(name: impl Into<String>, isolation: IsolationLevel) -> Self {
        Self::from_base(name, Arc::new(BaseLayer::empty()), isolation)
    }

    /// Creates a copy-on-write branch of this sandbox.
    ///
    /// The new sandbox shares the same immutable base layer (no copy) and
    /// inherits a private copy of the current overlay and tombstones, so the
    /// branch and its parent evolve independently.
    #[must_use]
    pub fn fork(&self, name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            created_at: Utc::now(),
            isolation: self.isolation,
            base: Arc::clone(&self.base),
            overlay: self.overlay.clone(),
            tombstones: self.tombstones.clone(),
            metadata: self.metadata.clone(),
        }
    }

    /// Returns the identifier of the shared base snapshot.
    #[must_use]
    pub fn base_snapshot_id(&self) -> Uuid {
        self.base.snapshot_id
    }

    /// Returns the number of statutes in the base layer.
    #[must_use]
    pub fn base_count(&self) -> usize {
        self.base.count()
    }

    /// Stages a statute entry into the sandbox overlay.
    ///
    /// If the statute exists in the base layer this records a modification; if
    /// it had been removed inside the sandbox the tombstone is cleared.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] when the sandbox is
    /// [`IsolationLevel::ReadOnly`].
    pub fn stage(&mut self, entry: StatuteEntry) -> RegistryResult<()> {
        self.ensure_writable()?;
        let statute_id = entry.statute.id.clone();
        self.tombstones.remove(&statute_id);
        self.overlay.insert(statute_id, entry);
        Ok(())
    }

    /// Convenience wrapper that wraps a statute in a [`StatuteEntry`] and stages it.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`SandboxEnvironment::stage`].
    pub fn stage_statute(
        &mut self,
        statute: legalis_core::Statute,
        jurisdiction: impl Into<String>,
    ) -> RegistryResult<()> {
        self.stage(StatuteEntry::new(statute, jurisdiction))
    }

    /// Removes a statute from the sandbox's effective view.
    ///
    /// Returns whether the statute had been visible before removal.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] when the sandbox is
    /// [`IsolationLevel::ReadOnly`].
    pub fn remove(&mut self, statute_id: &str) -> RegistryResult<bool> {
        self.ensure_writable()?;
        let existed = self.contains(statute_id);
        self.overlay.remove(statute_id);
        if self.base.statutes.contains_key(statute_id) {
            self.tombstones.insert(statute_id.to_string());
        }
        Ok(existed)
    }

    /// Returns the effective view of a statute, applying overlay and tombstones.
    #[must_use]
    pub fn effective(&self, statute_id: &str) -> Option<&StatuteEntry> {
        if self.tombstones.contains(statute_id) {
            return None;
        }
        self.overlay
            .get(statute_id)
            .or_else(|| self.base.statutes.get(statute_id))
    }

    /// Returns whether a statute is visible in the sandbox.
    #[must_use]
    pub fn contains(&self, statute_id: &str) -> bool {
        self.effective(statute_id).is_some()
    }

    /// Returns whether a statute was modified (overlaid over an existing base entry).
    #[must_use]
    pub fn is_modified(&self, statute_id: &str) -> bool {
        self.overlay.contains_key(statute_id) && self.base.statutes.contains_key(statute_id)
    }

    /// Returns whether a statute was newly added inside the sandbox.
    #[must_use]
    pub fn is_added(&self, statute_id: &str) -> bool {
        self.overlay.contains_key(statute_id) && !self.base.statutes.contains_key(statute_id)
    }

    /// Returns the sorted set of effective statute identifiers.
    #[must_use]
    pub fn effective_ids(&self) -> Vec<String> {
        let mut set: HashSet<String> = HashSet::new();
        for id in self.base.statutes.keys() {
            if !self.tombstones.contains(id) {
                set.insert(id.clone());
            }
        }
        for id in self.overlay.keys() {
            set.insert(id.clone());
        }
        let mut ids: Vec<String> = set.into_iter().collect();
        ids.sort();
        ids
    }

    /// Returns the effective statute entries, sorted by identifier.
    #[must_use]
    pub fn effective_entries(&self) -> Vec<&StatuteEntry> {
        self.effective_ids()
            .into_iter()
            .filter_map(|id| self.effective(&id))
            .collect()
    }

    /// Returns the number of effective statutes in the sandbox.
    #[must_use]
    pub fn count(&self) -> usize {
        self.effective_ids().len()
    }

    /// Returns `true` when the sandbox exposes no statutes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Returns the number of staged (added or modified) statutes.
    #[must_use]
    pub fn staged_count(&self) -> usize {
        self.overlay.len()
    }

    /// Returns the number of tombstoned (removed) statutes.
    #[must_use]
    pub fn tombstone_count(&self) -> usize {
        self.tombstones.len()
    }

    /// Sets a metadata key/value pair on the environment.
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Gets a metadata value by key.
    #[must_use]
    pub fn metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Computes the divergence of the sandbox from its base layer.
    #[must_use]
    pub fn diff_from_base(&self) -> SandboxDiff {
        let mut added = Vec::new();
        let mut modified = Vec::new();
        for id in self.overlay.keys() {
            if self.base.statutes.contains_key(id) {
                modified.push(id.clone());
            } else {
                added.push(id.clone());
            }
        }
        let mut removed: Vec<String> = self.tombstones.iter().cloned().collect();
        added.sort();
        modified.sort();
        removed.sort();
        SandboxDiff {
            added,
            modified,
            removed,
        }
    }

    /// Materializes the sandbox's effective state into a standalone registry.
    ///
    /// The returned registry is a fresh, independent [`StatuteRegistry`] that
    /// can be queried with the full registry API while leaving production
    /// untouched.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`StatuteRegistry::restore_from_backup`].
    pub fn materialize(&self) -> RegistryResult<StatuteRegistry> {
        let statutes: Vec<StatuteEntry> = self.effective_entries().into_iter().cloned().collect();
        let mut versions: HashMap<String, HashMap<u32, StatuteEntry>> = HashMap::new();
        for entry in &statutes {
            versions
                .entry(entry.statute.id.clone())
                .or_default()
                .insert(entry.version, entry.clone());
        }
        let metadata = BackupMetadata {
            created_at: Utc::now(),
            format_version: "1.0".to_string(),
            statute_count: statutes.len(),
            event_count: 0,
            description: Some(format!("materialized sandbox: {}", self.name)),
        };
        let backup = RegistryBackup {
            statutes,
            versions,
            events: Vec::new(),
            metadata,
        };
        let mut registry = StatuteRegistry::new();
        registry.restore_from_backup(backup)?;
        Ok(registry)
    }

    /// Produces a deterministic, canonical byte representation of the effective state.
    fn canonical_state(&self) -> RegistryResult<Vec<u8>> {
        let entries = self.effective_entries();
        serde_json::to_vec(&entries).map_err(|err| {
            RegistryError::InvalidOperation(format!("failed to serialize sandbox state: {err}"))
        })
    }

    /// Computes a SHA-256 integrity digest over the sandbox's effective state.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] if the state cannot be
    /// canonicalized for hashing.
    pub fn integrity_digest(&self) -> RegistryResult<String> {
        let bytes = self.canonical_state()?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        Ok(hex::encode(hasher.finalize()))
    }

    /// Captures a restorable checkpoint of the sandbox's mutable state.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`SandboxEnvironment::integrity_digest`].
    pub fn checkpoint(&self) -> RegistryResult<SandboxCheckpoint> {
        let digest = self.integrity_digest()?;
        Ok(SandboxCheckpoint {
            checkpoint_id: Uuid::new_v4(),
            environment_id: self.id,
            created_at: Utc::now(),
            digest,
            overlay: self.overlay.clone(),
            tombstones: self.tombstones.clone(),
        })
    }

    /// Restores the sandbox to a previously captured checkpoint.
    ///
    /// After restoring, the integrity digest of the effective state is
    /// recomputed and compared against the checkpoint's digest to guarantee an
    /// exact, byte-identical restoration.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] if the checkpoint belongs to
    /// a different environment or if the post-restore integrity check fails.
    pub fn restore(&mut self, checkpoint: &SandboxCheckpoint) -> RegistryResult<()> {
        if checkpoint.environment_id != self.id {
            return Err(RegistryError::InvalidOperation(format!(
                "checkpoint {} does not belong to environment {}",
                checkpoint.checkpoint_id, self.id
            )));
        }
        self.overlay = checkpoint.overlay.clone();
        self.tombstones = checkpoint.tombstones.clone();
        let digest = self.integrity_digest()?;
        if digest != checkpoint.digest {
            return Err(RegistryError::InvalidOperation(
                "sandbox integrity verification failed after restore".to_string(),
            ));
        }
        Ok(())
    }

    /// Returns an error when the sandbox is read-only.
    fn ensure_writable(&self) -> RegistryResult<()> {
        if self.isolation.is_read_only() {
            return Err(RegistryError::InvalidOperation(
                "sandbox is read-only and cannot be mutated".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    fn sample_registry() -> StatuteRegistry {
        let mut registry = StatuteRegistry::new();
        for idx in 0..3 {
            let statute = Statute::new(
                format!("statute-{idx}"),
                format!("Statute {idx}"),
                Effect::new(EffectType::Grant, "grant"),
            );
            let entry = StatuteEntry::new(statute, "US");
            registry.register(entry).expect("register should succeed");
        }
        registry
    }

    fn candidate(id: &str) -> StatuteEntry {
        let statute = Statute::new(id, "Candidate", Effect::new(EffectType::Obligation, "duty"));
        StatuteEntry::new(statute, "US")
    }

    #[test]
    fn test_environment_forks_base_without_mutating_registry() {
        let registry = sample_registry();
        let env = SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        assert_eq!(env.base_count(), 3);
        assert_eq!(env.count(), 3);
        assert!(env.staged_count() == 0);
        assert_eq!(registry.count(), 3);
    }

    #[test]
    fn test_copy_on_write_read_through() {
        let registry = sample_registry();
        let env = SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        let entry = env.effective("statute-1").expect("base statute visible");
        assert_eq!(entry.statute.id, "statute-1");
        // No overlay was created by a read.
        assert_eq!(env.staged_count(), 0);
    }

    #[test]
    fn test_stage_adds_to_overlay_only() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        env.stage(candidate("statute-new"))
            .expect("stage candidate");
        assert!(env.contains("statute-new"));
        assert!(env.is_added("statute-new"));
        assert_eq!(env.count(), 4);
        // Production registry untouched.
        assert_eq!(registry.count(), 3);
    }

    #[test]
    fn test_modify_existing_statute_in_sandbox() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        env.stage(candidate("statute-0"))
            .expect("override existing");
        assert!(env.is_modified("statute-0"));
        let entry = env.effective("statute-0").expect("modified visible");
        assert_eq!(entry.statute.title, "Candidate");
    }

    #[test]
    fn test_remove_creates_tombstone() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        let removed = env.remove("statute-2").expect("remove");
        assert!(removed);
        assert!(!env.contains("statute-2"));
        assert_eq!(env.tombstone_count(), 1);
        assert_eq!(env.count(), 2);
        // Base layer untouched.
        assert_eq!(env.base_count(), 3);
    }

    #[test]
    fn test_diff_from_base() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        env.stage(candidate("statute-new")).expect("add");
        env.stage(candidate("statute-0")).expect("modify");
        env.remove("statute-1").expect("remove");
        let diff = env.diff_from_base();
        assert_eq!(diff.added, vec!["statute-new".to_string()]);
        assert_eq!(diff.modified, vec!["statute-0".to_string()]);
        assert_eq!(diff.removed, vec!["statute-1".to_string()]);
        assert_eq!(diff.total_changes(), 3);
    }

    #[test]
    fn test_full_copy_isolation() {
        let registry = sample_registry();
        let env = SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::FullCopy);
        // FullCopy materializes the base into the overlay eagerly.
        assert_eq!(env.staged_count(), 3);
        assert_eq!(env.count(), 3);
    }

    #[test]
    fn test_read_only_rejects_mutation() {
        let registry = sample_registry();
        let mut env = SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::ReadOnly);
        let result = env.stage(candidate("x"));
        assert!(result.is_err());
        let result = env.remove("statute-0");
        assert!(result.is_err());
    }

    #[test]
    fn test_fork_is_independent() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        env.stage(candidate("statute-new")).expect("add");
        let mut branch = env.fork("branch");
        branch.stage(candidate("branch-only")).expect("branch add");
        // Branch shares the base snapshot.
        assert_eq!(branch.base_snapshot_id(), env.base_snapshot_id());
        // Branch inherited the parent overlay.
        assert!(branch.contains("statute-new"));
        // Mutations are independent.
        assert!(branch.contains("branch-only"));
        assert!(!env.contains("branch-only"));
    }

    #[test]
    fn test_materialize_into_registry() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        env.stage(candidate("statute-new")).expect("add");
        env.remove("statute-0").expect("remove");
        let mut materialized = env.materialize().expect("materialize");
        assert_eq!(materialized.count(), 3);
        assert!(materialized.get("statute-new").is_some());
        assert!(materialized.get("statute-0").is_none());
    }

    #[test]
    fn test_checkpoint_and_restore_round_trip() {
        let registry = sample_registry();
        let mut env =
            SandboxEnvironment::from_registry("exp", &registry, IsolationLevel::CopyOnWrite);
        let checkpoint = env.checkpoint().expect("checkpoint");
        env.stage(candidate("statute-new")).expect("mutate");
        env.remove("statute-0").expect("mutate");
        assert_ne!(env.integrity_digest().expect("digest"), checkpoint.digest);
        env.restore(&checkpoint).expect("restore");
        assert_eq!(env.integrity_digest().expect("digest"), checkpoint.digest);
        assert_eq!(env.count(), 3);
        assert!(!env.contains("statute-new"));
        assert!(env.contains("statute-0"));
    }

    #[test]
    fn test_restore_rejects_foreign_checkpoint() {
        let registry = sample_registry();
        let env_a = SandboxEnvironment::from_registry("a", &registry, IsolationLevel::CopyOnWrite);
        let mut env_b =
            SandboxEnvironment::from_registry("b", &registry, IsolationLevel::CopyOnWrite);
        let checkpoint = env_a.checkpoint().expect("checkpoint");
        let result = env_b.restore(&checkpoint);
        assert!(result.is_err());
    }
}
