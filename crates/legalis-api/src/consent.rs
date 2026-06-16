//! Consent management.
//!
//! Implements a consent ledger for data-processing purposes (GDPR Art. 6/7 style
//! lawful-basis tracking). Subjects grant or withdraw consent for named
//! processing purposes; every state change is recorded immutably with a version
//! so the full consent history is auditable. The store answers the operative
//! question — "does subject S currently consent to purpose P?" — and supports
//! consent expiry.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// The state of a consent decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentStatus {
    /// Consent has been granted.
    Granted,
    /// Consent has been explicitly withdrawn.
    Withdrawn,
}

/// A single immutable consent record (one event in the ledger).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Unique record id.
    pub id: Uuid,
    /// Subject (data subject / user) the consent pertains to.
    pub subject_id: String,
    /// Processing purpose this consent governs (e.g. `"analytics"`).
    pub purpose: String,
    /// Whether consent was granted or withdrawn by this event.
    pub status: ConsentStatus,
    /// Monotonic version for this (subject, purpose) pair.
    pub version: u64,
    /// When the event was recorded.
    pub recorded_at: DateTime<Utc>,
    /// Optional expiry for a granted consent.
    pub expires_at: Option<DateTime<Utc>>,
    /// Free-form provenance (e.g. policy version, UI source).
    pub source: Option<String>,
}

impl ConsentRecord {
    /// Returns whether this record represents currently-effective consent at
    /// `now` (granted and not expired).
    pub fn is_effective(&self, now: DateTime<Utc>) -> bool {
        self.status == ConsentStatus::Granted && self.expires_at.map(|e| now < e).unwrap_or(true)
    }
}

/// Errors produced by the consent store.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConsentError {
    /// No consent record exists for the (subject, purpose).
    #[error("no consent record for subject {subject} purpose {purpose}")]
    NotFound { subject: String, purpose: String },
}

/// Key for indexing consent state by subject + purpose.
type ConsentKey = (String, String);

/// Thread-safe consent ledger.
#[derive(Clone, Default)]
pub struct ConsentStore {
    inner: Arc<RwLock<ConsentState>>,
}

#[derive(Default)]
struct ConsentState {
    /// Full append-only history of consent events.
    history: Vec<ConsentRecord>,
    /// Latest record per (subject, purpose).
    latest: HashMap<ConsentKey, ConsentRecord>,
}

impl ConsentStore {
    /// Creates an empty consent store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a grant of consent for a (subject, purpose), superseding any
    /// previous state. Returns the new record.
    pub async fn grant(
        &self,
        subject_id: impl Into<String>,
        purpose: impl Into<String>,
        expires_at: Option<DateTime<Utc>>,
        source: Option<String>,
    ) -> ConsentRecord {
        self.record(
            subject_id.into(),
            purpose.into(),
            ConsentStatus::Granted,
            expires_at,
            source,
        )
        .await
    }

    /// Records a withdrawal of consent for a (subject, purpose). Returns the new
    /// record.
    pub async fn withdraw(
        &self,
        subject_id: impl Into<String>,
        purpose: impl Into<String>,
        source: Option<String>,
    ) -> ConsentRecord {
        self.record(
            subject_id.into(),
            purpose.into(),
            ConsentStatus::Withdrawn,
            None,
            source,
        )
        .await
    }

    async fn record(
        &self,
        subject_id: String,
        purpose: String,
        status: ConsentStatus,
        expires_at: Option<DateTime<Utc>>,
        source: Option<String>,
    ) -> ConsentRecord {
        let key = (subject_id.clone(), purpose.clone());
        let mut state = self.inner.write().await;
        let next_version = state.latest.get(&key).map(|r| r.version + 1).unwrap_or(1);
        let record = ConsentRecord {
            id: Uuid::new_v4(),
            subject_id,
            purpose,
            status,
            version: next_version,
            recorded_at: Utc::now(),
            expires_at,
            source,
        };
        state.history.push(record.clone());
        state.latest.insert(key, record.clone());
        record
    }

    /// Returns the latest consent record for a (subject, purpose), if any.
    pub async fn latest(&self, subject_id: &str, purpose: &str) -> Option<ConsentRecord> {
        let key = (subject_id.to_string(), purpose.to_string());
        self.inner.read().await.latest.get(&key).cloned()
    }

    /// Returns whether the subject currently consents to the purpose at `now`.
    pub async fn has_consent(&self, subject_id: &str, purpose: &str, now: DateTime<Utc>) -> bool {
        self.latest(subject_id, purpose)
            .await
            .map(|r| r.is_effective(now))
            .unwrap_or(false)
    }

    /// Requires that the subject consents to the purpose, returning an error if
    /// no effective consent exists.
    pub async fn require_consent(
        &self,
        subject_id: &str,
        purpose: &str,
        now: DateTime<Utc>,
    ) -> Result<ConsentRecord, ConsentError> {
        match self.latest(subject_id, purpose).await {
            Some(record) if record.is_effective(now) => Ok(record),
            _ => Err(ConsentError::NotFound {
                subject: subject_id.to_string(),
                purpose: purpose.to_string(),
            }),
        }
    }

    /// Returns the full consent history for a subject, ordered by record time.
    pub async fn history_for(&self, subject_id: &str) -> Vec<ConsentRecord> {
        let state = self.inner.read().await;
        let mut records: Vec<ConsentRecord> = state
            .history
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .cloned()
            .collect();
        records.sort_by_key(|r| r.recorded_at);
        records
    }

    /// Returns all currently-effective consents for a subject at `now`.
    pub async fn effective_purposes(&self, subject_id: &str, now: DateTime<Utc>) -> Vec<String> {
        let state = self.inner.read().await;
        state
            .latest
            .values()
            .filter(|r| r.subject_id == subject_id && r.is_effective(now))
            .map(|r| r.purpose.clone())
            .collect()
    }

    /// Total number of recorded consent events.
    pub async fn event_count(&self) -> usize {
        self.inner.read().await.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_grant_and_check() {
        let store = ConsentStore::new();
        let now = Utc::now();
        assert!(!store.has_consent("alice", "analytics", now).await);

        let rec = store.grant("alice", "analytics", None, None).await;
        assert_eq!(rec.version, 1);
        assert_eq!(rec.status, ConsentStatus::Granted);
        assert!(store.has_consent("alice", "analytics", now).await);
    }

    #[tokio::test]
    async fn test_withdraw_revokes() {
        let store = ConsentStore::new();
        let now = Utc::now();
        store.grant("bob", "marketing", None, None).await;
        assert!(store.has_consent("bob", "marketing", now).await);

        let w = store.withdraw("bob", "marketing", None).await;
        assert_eq!(w.status, ConsentStatus::Withdrawn);
        assert_eq!(w.version, 2);
        assert!(!store.has_consent("bob", "marketing", now).await);
    }

    #[tokio::test]
    async fn test_versions_increment() {
        let store = ConsentStore::new();
        let r1 = store.grant("c", "p", None, None).await;
        let r2 = store.withdraw("c", "p", None).await;
        let r3 = store.grant("c", "p", None, None).await;
        assert_eq!((r1.version, r2.version, r3.version), (1, 2, 3));
    }

    #[tokio::test]
    async fn test_expiry() {
        let store = ConsentStore::new();
        let now = Utc::now();
        let past = now - chrono::Duration::hours(1);
        store.grant("d", "p", Some(past), None).await;
        // Granted but already expired -> not effective.
        assert!(!store.has_consent("d", "p", now).await);

        let future = now + chrono::Duration::hours(1);
        store.grant("d", "p", Some(future), None).await;
        assert!(store.has_consent("d", "p", now).await);
    }

    #[tokio::test]
    async fn test_require_consent() {
        let store = ConsentStore::new();
        let now = Utc::now();
        let err = store.require_consent("e", "p", now).await.unwrap_err();
        assert_eq!(
            err,
            ConsentError::NotFound {
                subject: "e".to_string(),
                purpose: "p".to_string()
            }
        );

        store.grant("e", "p", None, None).await;
        assert!(store.require_consent("e", "p", now).await.is_ok());
    }

    #[tokio::test]
    async fn test_history_and_effective_purposes() {
        let store = ConsentStore::new();
        let now = Utc::now();
        store.grant("f", "analytics", None, None).await;
        store.grant("f", "marketing", None, None).await;
        store.withdraw("f", "marketing", None).await;

        let history = store.history_for("f").await;
        assert_eq!(history.len(), 3);

        let effective = store.effective_purposes("f", now).await;
        assert!(effective.contains(&"analytics".to_string()));
        assert!(!effective.contains(&"marketing".to_string()));

        assert_eq!(store.event_count().await, 3);
    }

    #[tokio::test]
    async fn test_latest_with_source() {
        let store = ConsentStore::new();
        store
            .grant("g", "p", None, Some("policy-v2".to_string()))
            .await;
        let latest = store.latest("g", "p").await.expect("latest");
        assert_eq!(latest.source.as_deref(), Some("policy-v2"));
    }
}
