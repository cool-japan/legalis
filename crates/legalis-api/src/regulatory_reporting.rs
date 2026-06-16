//! Regulatory reporting APIs.
//!
//! Aggregates audit activity into structured compliance reports over a time
//! window. Reports summarise operational activity (event-type counts, success /
//! failure rates, distinct actors, affected resources) and surface compliance
//! signals (failed-operation rate, security-relevant events such as permission
//! changes and key lifecycle events). The reports are serialisable so they can
//! be returned over HTTP or persisted for regulators.

use crate::audit::{AuditEntry, AuditEventType, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A time range for a report, inclusive of `start`, exclusive of `end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportPeriod {
    /// Start of the period (inclusive).
    pub start: DateTime<Utc>,
    /// End of the period (exclusive).
    pub end: DateTime<Utc>,
}

impl ReportPeriod {
    /// Creates a report period, normalising so that `start <= end`.
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        if start <= end {
            Self { start, end }
        } else {
            Self {
                start: end,
                end: start,
            }
        }
    }

    /// Returns whether a timestamp falls within the period.
    pub fn contains(&self, ts: DateTime<Utc>) -> bool {
        ts >= self.start && ts < self.end
    }
}

/// A generated regulatory / compliance report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// The period the report covers.
    pub period: ReportPeriod,
    /// When the report was generated.
    pub generated_at: DateTime<Utc>,
    /// Total audited events within the period.
    pub total_events: usize,
    /// Count of events by event type (stable key ordering).
    pub events_by_type: BTreeMap<String, usize>,
    /// Successful operations.
    pub successful: usize,
    /// Failed operations.
    pub failed: usize,
    /// Partially-successful operations.
    pub partial: usize,
    /// Number of distinct actors (users) observed.
    pub distinct_actors: usize,
    /// Number of distinct resources touched.
    pub distinct_resources: usize,
    /// Count of security-relevant events (permission / key lifecycle changes).
    pub security_events: usize,
    /// Failure rate over the period in `[0, 1]`.
    pub failure_rate: f64,
}

impl ComplianceReport {
    /// Returns whether the failure rate is below the given threshold.
    pub fn within_failure_threshold(&self, threshold: f64) -> bool {
        self.failure_rate <= threshold
    }
}

/// Returns whether an event type is security-relevant for compliance reporting.
fn is_security_event(event_type: &AuditEventType) -> bool {
    matches!(
        event_type,
        AuditEventType::PermissionGranted
            | AuditEventType::PermissionRevoked
            | AuditEventType::ApiKeyCreated
            | AuditEventType::ApiKeyRotated
            | AuditEventType::ApiKeyRevoked
            | AuditEventType::UserLogin
            | AuditEventType::UserLogout
            | AuditEventType::ConfigurationChanged
    )
}

/// Renders an event type as its serialised snake_case label.
fn event_type_label(event_type: &AuditEventType) -> String {
    serde_json::to_value(event_type)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string())
}

/// Generates a compliance report from audit entries restricted to `period`.
pub fn generate_report(entries: &[AuditEntry], period: ReportPeriod) -> ComplianceReport {
    use std::collections::HashSet;

    let mut events_by_type: BTreeMap<String, usize> = BTreeMap::new();
    let mut successful = 0usize;
    let mut failed = 0usize;
    let mut partial = 0usize;
    let mut actors: HashSet<&str> = HashSet::new();
    let mut resources: HashSet<&str> = HashSet::new();
    let mut security_events = 0usize;
    let mut total = 0usize;

    for entry in entries.iter().filter(|e| period.contains(e.timestamp)) {
        total += 1;
        *events_by_type
            .entry(event_type_label(&entry.event_type))
            .or_insert(0) += 1;
        match entry.result {
            AuditResult::Success => successful += 1,
            AuditResult::Failure => failed += 1,
            AuditResult::Partial => partial += 1,
        }
        actors.insert(entry.user_id.as_str());
        if let Some(rid) = &entry.resource_id {
            resources.insert(rid.as_str());
        }
        if is_security_event(&entry.event_type) {
            security_events += 1;
        }
    }

    let failure_rate = if total > 0 {
        failed as f64 / total as f64
    } else {
        0.0
    };

    ComplianceReport {
        period,
        generated_at: Utc::now(),
        total_events: total,
        events_by_type,
        successful,
        failed,
        partial,
        distinct_actors: actors.len(),
        distinct_resources: resources.len(),
        security_events,
        failure_rate,
    }
}

/// A per-actor activity breakdown, useful for data-subject access requests and
/// accountability reporting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorActivityReport {
    /// The actor (user id).
    pub user_id: String,
    /// The period covered.
    pub period: ReportPeriod,
    /// Total events attributed to the actor.
    pub total_events: usize,
    /// Event counts by type.
    pub events_by_type: BTreeMap<String, usize>,
    /// Resources the actor touched.
    pub resources: Vec<String>,
}

/// Generates a per-actor activity report restricted to `period`.
pub fn generate_actor_report(
    entries: &[AuditEntry],
    user_id: &str,
    period: ReportPeriod,
) -> ActorActivityReport {
    let mut events_by_type: BTreeMap<String, usize> = BTreeMap::new();
    let mut resources: Vec<String> = Vec::new();
    let mut total = 0usize;

    for entry in entries
        .iter()
        .filter(|e| e.user_id == user_id && period.contains(e.timestamp))
    {
        total += 1;
        *events_by_type
            .entry(event_type_label(&entry.event_type))
            .or_insert(0) += 1;
        if let Some(rid) = &entry.resource_id
            && !resources.contains(rid)
        {
            resources.push(rid.clone());
        }
    }
    resources.sort();

    ActorActivityReport {
        user_id: user_id.to_string(),
        period,
        total_events: total,
        events_by_type,
        resources,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(
        ts: DateTime<Utc>,
        event_type: AuditEventType,
        user: &str,
        resource: Option<&str>,
        result: AuditResult,
    ) -> AuditEntry {
        AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: ts,
            event_type,
            user_id: user.to_string(),
            username: user.to_string(),
            resource_id: resource.map(|r| r.to_string()),
            resource_type: Some("statute".to_string()),
            action: "act".to_string(),
            details: serde_json::json!({}),
            ip_address: None,
            user_agent: None,
            result,
            error_message: None,
        }
    }

    #[test]
    fn test_report_period_normalises() {
        let a = Utc::now();
        let b = a + chrono::Duration::hours(1);
        let p = ReportPeriod::new(b, a);
        assert!(p.start <= p.end);
    }

    #[test]
    fn test_period_contains() {
        let start = Utc::now();
        let end = start + chrono::Duration::hours(2);
        let p = ReportPeriod::new(start, end);
        assert!(p.contains(start));
        assert!(p.contains(start + chrono::Duration::hours(1)));
        assert!(!p.contains(end));
        assert!(!p.contains(start - chrono::Duration::seconds(1)));
    }

    #[test]
    fn test_generate_report_counts() {
        let base = Utc::now();
        let period = ReportPeriod::new(base, base + chrono::Duration::hours(1));
        let entries = vec![
            entry(
                base,
                AuditEventType::StatuteCreated,
                "alice",
                Some("s1"),
                AuditResult::Success,
            ),
            entry(
                base,
                AuditEventType::StatuteCreated,
                "bob",
                Some("s2"),
                AuditResult::Failure,
            ),
            entry(
                base,
                AuditEventType::PermissionGranted,
                "alice",
                Some("s1"),
                AuditResult::Success,
            ),
            // Outside the period -> excluded.
            entry(
                base + chrono::Duration::hours(2),
                AuditEventType::StatuteDeleted,
                "carol",
                Some("s3"),
                AuditResult::Success,
            ),
        ];
        let report = generate_report(&entries, period);
        assert_eq!(report.total_events, 3);
        assert_eq!(report.successful, 2);
        assert_eq!(report.failed, 1);
        assert_eq!(report.distinct_actors, 2);
        assert_eq!(report.distinct_resources, 2);
        assert_eq!(report.security_events, 1);
        assert_eq!(report.events_by_type.get("statute_created"), Some(&2));
        assert!((report.failure_rate - (1.0 / 3.0)).abs() < 1e-9);
    }

    #[test]
    fn test_failure_threshold() {
        let base = Utc::now();
        let period = ReportPeriod::new(base, base + chrono::Duration::hours(1));
        let entries = vec![
            entry(
                base,
                AuditEventType::StatuteCreated,
                "a",
                None,
                AuditResult::Success,
            ),
            entry(
                base,
                AuditEventType::StatuteCreated,
                "a",
                None,
                AuditResult::Success,
            ),
        ];
        let report = generate_report(&entries, period);
        assert_eq!(report.failure_rate, 0.0);
        assert!(report.within_failure_threshold(0.01));
    }

    #[test]
    fn test_empty_report() {
        let base = Utc::now();
        let period = ReportPeriod::new(base, base + chrono::Duration::hours(1));
        let report = generate_report(&[], period);
        assert_eq!(report.total_events, 0);
        assert_eq!(report.failure_rate, 0.0);
        assert!(report.events_by_type.is_empty());
    }

    #[test]
    fn test_actor_report() {
        let base = Utc::now();
        let period = ReportPeriod::new(base, base + chrono::Duration::hours(1));
        let entries = vec![
            entry(
                base,
                AuditEventType::StatuteCreated,
                "alice",
                Some("s1"),
                AuditResult::Success,
            ),
            entry(
                base,
                AuditEventType::StatuteUpdated,
                "alice",
                Some("s2"),
                AuditResult::Success,
            ),
            entry(
                base,
                AuditEventType::StatuteCreated,
                "bob",
                Some("s3"),
                AuditResult::Success,
            ),
        ];
        let report = generate_actor_report(&entries, "alice", period);
        assert_eq!(report.total_events, 2);
        assert_eq!(report.resources, vec!["s1", "s2"]);
        assert_eq!(report.events_by_type.get("statute_created"), Some(&1));
        assert_eq!(report.events_by_type.get("statute_updated"), Some(&1));
    }

    #[test]
    fn test_security_event_classification() {
        assert!(is_security_event(&AuditEventType::ApiKeyRotated));
        assert!(is_security_event(&AuditEventType::PermissionRevoked));
        assert!(!is_security_event(&AuditEventType::StatuteCreated));
        assert!(!is_security_event(&AuditEventType::SimulationExecuted));
    }
}
