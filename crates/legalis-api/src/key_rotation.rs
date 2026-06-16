//! API key lifecycle and rotation management.
//!
//! This module implements a complete lifecycle/rotation manager for API keys
//! on top of the [`crate::auth::ApiKey`] type. It supports:
//!
//! - Registering keys and tracking their lifecycle state.
//! - Scheduling rotation based on a maximum key age (rotation interval).
//! - Grace periods during which a rotated (previous) key remains valid so that
//!   clients can migrate without downtime ("overlapping" keys).
//! - Automatic expiry of grace-period keys.
//! - Querying which keys are due for rotation and the overall key status.
//!
//! The manager hashes raw key material with SHA-256 so that the plaintext key is
//! never stored in the lifecycle state; only the hash is retained for lookups.

use crate::auth::ApiKey;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Lifecycle state of a managed API key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyState {
    /// Key is active and fully usable.
    Active,
    /// Key has been rotated out but is still accepted during its grace period.
    GracePeriod,
    /// Key has been retired and is no longer valid.
    Retired,
    /// Key has been explicitly revoked (compromise / manual action).
    Revoked,
}

/// Hashes a raw API key value into a hex-encoded SHA-256 digest.
///
/// The plaintext key is never persisted; only this digest is kept so lookups
/// can be performed without exposing secrets in the lifecycle store.
pub fn hash_key(raw_key: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(raw_key.as_bytes());
    hex::encode(hasher.finalize())
}

/// A managed key record tracking lifecycle metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedKey {
    /// Identifier of the underlying [`ApiKey`].
    pub key_id: Uuid,
    /// SHA-256 hash of the raw key value (never the plaintext).
    pub key_hash: String,
    /// Owner of the key.
    pub owner_id: Uuid,
    /// Current lifecycle state.
    pub state: KeyState,
    /// When the key was created / registered.
    pub created_at: DateTime<Utc>,
    /// When the key was last rotated (if ever).
    pub rotated_at: Option<DateTime<Utc>>,
    /// When the grace period ends (only meaningful for `GracePeriod` keys).
    pub grace_until: Option<DateTime<Utc>>,
    /// Identifier of the key that superseded this one (for rotation chains).
    pub superseded_by: Option<Uuid>,
}

impl ManagedKey {
    /// Returns the age of the key relative to `now`.
    pub fn age(&self, now: DateTime<Utc>) -> chrono::Duration {
        now - self.created_at
    }

    /// Returns whether this key is currently usable at `now`.
    ///
    /// Active keys are always usable. Grace-period keys are usable until their
    /// grace deadline elapses. Retired/revoked keys are never usable.
    pub fn is_usable(&self, now: DateTime<Utc>) -> bool {
        match self.state {
            KeyState::Active => true,
            KeyState::GracePeriod => self.grace_until.map(|g| now < g).unwrap_or(false),
            KeyState::Retired | KeyState::Revoked => false,
        }
    }
}

/// Configuration for the rotation manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationPolicy {
    /// Maximum age of an active key before it should be rotated, in seconds.
    pub max_age_secs: i64,
    /// Grace period (seconds) during which a rotated key remains usable.
    pub grace_period_secs: i64,
}

impl Default for RotationPolicy {
    fn default() -> Self {
        Self {
            // 90 days default rotation interval.
            max_age_secs: 90 * 24 * 60 * 60,
            // 7 day overlap grace period.
            grace_period_secs: 7 * 24 * 60 * 60,
        }
    }
}

impl RotationPolicy {
    /// Creates a new rotation policy.
    pub fn new(max_age_secs: i64, grace_period_secs: i64) -> Self {
        Self {
            max_age_secs,
            grace_period_secs,
        }
    }
}

/// Errors produced by the rotation manager.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RotationError {
    /// Key not found in the lifecycle store.
    #[error("managed key not found: {0}")]
    NotFound(Uuid),
    /// Key cannot be rotated because it is not active.
    #[error("key {0} is not active and cannot be rotated")]
    NotActive(Uuid),
}

/// Outcome of a rotation operation, exposing the freshly created key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationOutcome {
    /// Identifier of the previous (now grace-period) key.
    pub previous_key_id: Uuid,
    /// The newly created replacement key.
    pub new_key: ApiKey,
    /// When the previous key's grace period ends.
    pub grace_until: DateTime<Utc>,
}

/// Thread-safe API key rotation / lifecycle manager.
#[derive(Clone)]
pub struct KeyRotationManager {
    inner: Arc<RwLock<RotationState>>,
    policy: RotationPolicy,
}

struct RotationState {
    keys: HashMap<Uuid, ManagedKey>,
    /// Index from key hash -> key id for fast lookup by raw key.
    hash_index: HashMap<String, Uuid>,
}

impl KeyRotationManager {
    /// Creates a new manager with the given policy.
    pub fn new(policy: RotationPolicy) -> Self {
        Self {
            inner: Arc::new(RwLock::new(RotationState {
                keys: HashMap::new(),
                hash_index: HashMap::new(),
            })),
            policy,
        }
    }

    /// Creates a manager with the default policy.
    pub fn with_default_policy() -> Self {
        Self::new(RotationPolicy::default())
    }

    /// Returns the configured policy.
    pub fn policy(&self) -> &RotationPolicy {
        &self.policy
    }

    /// Registers an existing API key into the lifecycle store as `Active`.
    ///
    /// The `raw_key` is hashed and indexed; only the hash is retained.
    pub async fn register(&self, key: &ApiKey, raw_key: &str) {
        let key_hash = hash_key(raw_key);
        let created_at =
            DateTime::<Utc>::from_timestamp(key.created_at, 0).unwrap_or_else(Utc::now);
        let managed = ManagedKey {
            key_id: key.id,
            key_hash: key_hash.clone(),
            owner_id: key.owner_id,
            state: KeyState::Active,
            created_at,
            rotated_at: None,
            grace_until: None,
            superseded_by: None,
        };
        let mut state = self.inner.write().await;
        state.hash_index.insert(key_hash, key.id);
        state.keys.insert(key.id, managed);
    }

    /// Looks up the managed record for a raw key value, if usable at `now`.
    ///
    /// Returns `None` if the key is unknown or not currently usable.
    pub async fn lookup_usable(&self, raw_key: &str, now: DateTime<Utc>) -> Option<ManagedKey> {
        let key_hash = hash_key(raw_key);
        let state = self.inner.read().await;
        let key_id = state.hash_index.get(&key_hash)?;
        let managed = state.keys.get(key_id)?;
        if managed.is_usable(now) {
            Some(managed.clone())
        } else {
            None
        }
    }

    /// Returns the managed record for a key id.
    pub async fn get(&self, key_id: Uuid) -> Option<ManagedKey> {
        self.inner.read().await.keys.get(&key_id).cloned()
    }

    /// Returns all managed key records.
    pub async fn all(&self) -> Vec<ManagedKey> {
        self.inner.read().await.keys.values().cloned().collect()
    }

    /// Returns whether a key (by id) is due for rotation at `now` per policy.
    pub async fn is_due_for_rotation(&self, key_id: Uuid, now: DateTime<Utc>) -> Option<bool> {
        let state = self.inner.read().await;
        let managed = state.keys.get(&key_id)?;
        if managed.state != KeyState::Active {
            return Some(false);
        }
        Some(managed.age(now).num_seconds() >= self.policy.max_age_secs)
    }

    /// Returns the ids of all active keys due for rotation at `now`.
    pub async fn due_for_rotation(&self, now: DateTime<Utc>) -> Vec<Uuid> {
        let state = self.inner.read().await;
        state
            .keys
            .values()
            .filter(|m| {
                m.state == KeyState::Active && m.age(now).num_seconds() >= self.policy.max_age_secs
            })
            .map(|m| m.key_id)
            .collect()
    }

    /// Rotates a key: the old key enters its grace period and a new key is
    /// produced (via [`ApiKey::rotate`]) and registered as `Active`.
    ///
    /// Returns the rotation outcome including the new key's plaintext value (only
    /// available at creation time).
    pub async fn rotate(
        &self,
        old: &ApiKey,
        now: DateTime<Utc>,
    ) -> Result<RotationOutcome, RotationError> {
        let new_key = old.rotate();
        let grace_until = now + chrono::Duration::seconds(self.policy.grace_period_secs);
        let new_hash = hash_key(&new_key.key);
        let new_created_at = DateTime::<Utc>::from_timestamp(new_key.created_at, 0).unwrap_or(now);

        let mut state = self.inner.write().await;
        // Old key must exist and be active.
        {
            let managed = state
                .keys
                .get(&old.id)
                .ok_or(RotationError::NotFound(old.id))?;
            if managed.state != KeyState::Active {
                return Err(RotationError::NotActive(old.id));
            }
        }
        // Transition old key into grace period.
        if let Some(managed) = state.keys.get_mut(&old.id) {
            managed.state = KeyState::GracePeriod;
            managed.rotated_at = Some(now);
            managed.grace_until = Some(grace_until);
            managed.superseded_by = Some(new_key.id);
        }
        // Register the new key as active.
        let new_managed = ManagedKey {
            key_id: new_key.id,
            key_hash: new_hash.clone(),
            owner_id: new_key.owner_id,
            state: KeyState::Active,
            created_at: new_created_at,
            rotated_at: None,
            grace_until: None,
            superseded_by: None,
        };
        state.hash_index.insert(new_hash, new_key.id);
        state.keys.insert(new_key.id, new_managed);

        Ok(RotationOutcome {
            previous_key_id: old.id,
            new_key,
            grace_until,
        })
    }

    /// Explicitly revokes a key by id (e.g. on suspected compromise).
    pub async fn revoke(&self, key_id: Uuid) -> Result<(), RotationError> {
        let mut state = self.inner.write().await;
        let managed = state
            .keys
            .get_mut(&key_id)
            .ok_or(RotationError::NotFound(key_id))?;
        managed.state = KeyState::Revoked;
        managed.grace_until = None;
        Ok(())
    }

    /// Expires grace-period keys whose grace deadline has passed, moving them to
    /// `Retired`. Returns the number of keys retired.
    pub async fn expire_grace_periods(&self, now: DateTime<Utc>) -> usize {
        let mut state = self.inner.write().await;
        let mut retired = 0;
        for managed in state.keys.values_mut() {
            if managed.state == KeyState::GracePeriod {
                let expired = managed.grace_until.map(|g| now >= g).unwrap_or(true);
                if expired {
                    managed.state = KeyState::Retired;
                    managed.grace_until = None;
                    retired += 1;
                }
            }
        }
        retired
    }

    /// Returns a summary of key states for status reporting.
    pub async fn status_summary(&self, now: DateTime<Utc>) -> KeyStatusSummary {
        let state = self.inner.read().await;
        let mut summary = KeyStatusSummary::default();
        for managed in state.keys.values() {
            match managed.state {
                KeyState::Active => {
                    summary.active += 1;
                    if managed.age(now).num_seconds() >= self.policy.max_age_secs {
                        summary.due_for_rotation += 1;
                    }
                }
                KeyState::GracePeriod => summary.grace_period += 1,
                KeyState::Retired => summary.retired += 1,
                KeyState::Revoked => summary.revoked += 1,
            }
        }
        summary.total = state.keys.len();
        summary
    }
}

/// Aggregate status of all managed keys.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyStatusSummary {
    /// Total managed keys.
    pub total: usize,
    /// Active keys.
    pub active: usize,
    /// Keys currently in their grace period.
    pub grace_period: usize,
    /// Retired keys.
    pub retired: usize,
    /// Revoked keys.
    pub revoked: usize,
    /// Active keys that are past their rotation interval.
    pub due_for_rotation: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Role;

    fn make_key() -> (ApiKey, String) {
        let key = ApiKey::new("test".to_string(), Uuid::new_v4(), Role::ApiClient);
        let raw = key.key.clone();
        (key, raw)
    }

    #[test]
    fn test_hash_key_deterministic_and_distinct() {
        assert_eq!(hash_key("abc"), hash_key("abc"));
        assert_ne!(hash_key("abc"), hash_key("abd"));
        // 32-byte SHA-256 -> 64 hex chars.
        assert_eq!(hash_key("abc").len(), 64);
    }

    #[tokio::test]
    async fn test_register_and_lookup() {
        let mgr = KeyRotationManager::with_default_policy();
        let (key, raw) = make_key();
        mgr.register(&key, &raw).await;

        let now = Utc::now();
        let found = mgr.lookup_usable(&raw, now).await;
        assert!(found.is_some());
        let found = found.expect("registered key must be found");
        assert_eq!(found.key_id, key.id);
        assert_eq!(found.state, KeyState::Active);

        // Unknown key is not found.
        assert!(mgr.lookup_usable("lgl_unknown", now).await.is_none());
    }

    #[tokio::test]
    async fn test_due_for_rotation() {
        let policy = RotationPolicy::new(100, 50);
        let mgr = KeyRotationManager::new(policy);
        let (mut key, raw) = make_key();
        // Created 200 seconds ago -> older than 100s max age.
        key.created_at = Utc::now().timestamp() - 200;
        mgr.register(&key, &raw).await;

        let now = Utc::now();
        assert_eq!(mgr.is_due_for_rotation(key.id, now).await, Some(true));
        let due = mgr.due_for_rotation(now).await;
        assert!(due.contains(&key.id));
    }

    #[tokio::test]
    async fn test_not_due_when_fresh() {
        let policy = RotationPolicy::new(10_000, 50);
        let mgr = KeyRotationManager::new(policy);
        let (key, raw) = make_key();
        mgr.register(&key, &raw).await;
        let now = Utc::now();
        assert_eq!(mgr.is_due_for_rotation(key.id, now).await, Some(false));
        assert!(mgr.due_for_rotation(now).await.is_empty());
    }

    #[tokio::test]
    async fn test_rotate_grace_period_overlap() {
        let policy = RotationPolicy::new(100, 3600);
        let mgr = KeyRotationManager::new(policy);
        let (key, raw) = make_key();
        mgr.register(&key, &raw).await;

        let now = Utc::now();
        let outcome = mgr.rotate(&key, now).await.expect("rotation must succeed");
        assert_eq!(outcome.previous_key_id, key.id);
        assert_ne!(outcome.new_key.id, key.id);
        assert_eq!(outcome.new_key.previous_key_id, Some(key.id));

        // Old key still usable during grace period.
        let old_managed = mgr.get(key.id).await.expect("old key must exist");
        assert_eq!(old_managed.state, KeyState::GracePeriod);
        assert!(mgr.lookup_usable(&raw, now).await.is_some());

        // New key usable immediately.
        assert!(mgr.lookup_usable(&outcome.new_key.key, now).await.is_some());

        // After grace period elapses, old key is no longer usable.
        let after = now + chrono::Duration::seconds(3601);
        assert!(mgr.lookup_usable(&raw, after).await.is_none());
    }

    #[tokio::test]
    async fn test_rotate_unknown_key_errors() {
        let mgr = KeyRotationManager::with_default_policy();
        let (key, _raw) = make_key();
        let err = mgr.rotate(&key, Utc::now()).await.unwrap_err();
        assert_eq!(err, RotationError::NotFound(key.id));
    }

    #[tokio::test]
    async fn test_rotate_non_active_errors() {
        let policy = RotationPolicy::new(100, 100);
        let mgr = KeyRotationManager::new(policy);
        let (key, raw) = make_key();
        mgr.register(&key, &raw).await;
        let now = Utc::now();
        // First rotation moves key into grace period.
        mgr.rotate(&key, now).await.expect("first rotation");
        // Second rotation of the same (now grace-period) key must fail.
        let err = mgr.rotate(&key, now).await.unwrap_err();
        assert_eq!(err, RotationError::NotActive(key.id));
    }

    #[tokio::test]
    async fn test_expire_grace_periods() {
        let policy = RotationPolicy::new(100, 10);
        let mgr = KeyRotationManager::new(policy);
        let (key, raw) = make_key();
        mgr.register(&key, &raw).await;
        let now = Utc::now();
        mgr.rotate(&key, now).await.expect("rotation");

        // Before deadline: nothing retired.
        assert_eq!(mgr.expire_grace_periods(now).await, 0);
        // After deadline: the old key is retired.
        let after = now + chrono::Duration::seconds(11);
        assert_eq!(mgr.expire_grace_periods(after).await, 1);
        let old = mgr.get(key.id).await.expect("old key");
        assert_eq!(old.state, KeyState::Retired);
    }

    #[tokio::test]
    async fn test_revoke() {
        let mgr = KeyRotationManager::with_default_policy();
        let (key, raw) = make_key();
        mgr.register(&key, &raw).await;
        mgr.revoke(key.id).await.expect("revoke");
        let now = Utc::now();
        assert!(mgr.lookup_usable(&raw, now).await.is_none());
        let m = mgr.get(key.id).await.expect("key");
        assert_eq!(m.state, KeyState::Revoked);

        // Revoking unknown key errors.
        let unknown = Uuid::new_v4();
        assert_eq!(
            mgr.revoke(unknown).await,
            Err(RotationError::NotFound(unknown))
        );
    }

    #[tokio::test]
    async fn test_status_summary() {
        let policy = RotationPolicy::new(100, 3600);
        let mgr = KeyRotationManager::new(policy);
        let (k1, r1) = make_key();
        let (k2, r2) = make_key();
        let (mut k3, r3) = make_key();
        k3.created_at = Utc::now().timestamp() - 500; // due for rotation
        mgr.register(&k1, &r1).await;
        mgr.register(&k2, &r2).await;
        mgr.register(&k3, &r3).await;

        let now = Utc::now();
        mgr.rotate(&k2, now).await.expect("rotate k2");

        let summary = mgr.status_summary(now).await;
        // k1 active+fresh, k2 -> grace + new active key, k3 active+due.
        assert_eq!(summary.total, 4);
        assert_eq!(summary.grace_period, 1);
        assert!(summary.active >= 3);
        assert!(summary.due_for_rotation >= 1);
    }
}
