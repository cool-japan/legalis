//! Tenant-scoped audit dashboards.
//!
//! [`TenantDashboard`] renders a single tenant's operational snapshot
//! ([`TenantDashboardSnapshot`]) — its statistics, statute/actor distributions,
//! recent hourly activity, and any triggered alerts. [`MultiTenantOverview`]
//! gives an operator a cohort-level view built only from per-tenant aggregates,
//! so it never exposes one tenant's records to another.
//!
//! The dashboard reuses the crate's existing dashboard primitives
//! ([`crate::dashboard::HourlyMetric`], [`crate::dashboard::AlertInfo`], etc.)
//! rather than re-modelling them.

use super::TenantId;
use super::analytics::{CrossTenantAnalytics, TenantStats};
use super::isolation::TenantStore;
use crate::dashboard::{AlertInfo, AlertType, EventSeverity, HourlyMetric};
use crate::{AuditRecord, AuditResult, DecisionResult};
use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for tenant dashboard rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantDashboardConfig {
    /// Number of trailing hours to include in the hourly trend.
    pub trend_hours: i64,
    /// Override-rate threshold above which a `HighOverrideRate` alert fires.
    pub override_rate_alert: f64,
    /// Void-rate threshold above which a `ComplianceViolation` alert fires.
    pub void_rate_alert: f64,
    /// Per-hour decision count above which a `VolumeSpike` alert fires.
    pub hourly_volume_alert: usize,
}

impl Default for TenantDashboardConfig {
    fn default() -> Self {
        Self {
            trend_hours: 24,
            override_rate_alert: 0.25,
            void_rate_alert: 0.1,
            hourly_volume_alert: 1000,
        }
    }
}

/// A tenant-scoped dashboard snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantDashboardSnapshot {
    /// The tenant this snapshot describes.
    pub tenant_id: TenantId,
    /// When the snapshot was produced.
    pub generated_at: DateTime<Utc>,
    /// Aggregate statistics for the tenant.
    pub stats: TenantStats,
    /// Decision counts by statute.
    pub statute_distribution: HashMap<String, usize>,
    /// Decision counts by actor description.
    pub actor_distribution: HashMap<String, usize>,
    /// Hourly activity over the configured trailing window (oldest first).
    pub hourly_trends: Vec<HourlyMetric>,
    /// Alerts triggered for the tenant.
    pub alerts: Vec<AlertInfo>,
}

/// Renders tenant-scoped dashboards from a [`TenantStore`].
pub struct TenantDashboard {
    config: TenantDashboardConfig,
}

impl TenantDashboard {
    /// Creates a dashboard with the default configuration.
    pub fn new() -> Self {
        Self {
            config: TenantDashboardConfig::default(),
        }
    }

    /// Creates a dashboard with a custom configuration.
    pub fn with_config(config: TenantDashboardConfig) -> Self {
        Self { config }
    }

    /// Renders a snapshot for a single tenant.
    pub fn render(
        &self,
        store: &TenantStore,
        tenant_id: &TenantId,
    ) -> AuditResult<TenantDashboardSnapshot> {
        let records = store.records_for(tenant_id)?;
        let generated_at = Utc::now();
        let stats = TenantStats::compute(tenant_id.clone(), &records);

        let statute_distribution = distribution(&records, |r| r.statute_id.clone());
        let actor_distribution = distribution(&records, actor_label);
        let hourly_trends = self.hourly_trends(&records, generated_at);
        let alerts = self.alerts(tenant_id, &stats, &hourly_trends, generated_at);

        Ok(TenantDashboardSnapshot {
            tenant_id: tenant_id.clone(),
            generated_at,
            stats,
            statute_distribution,
            actor_distribution,
            hourly_trends,
            alerts,
        })
    }

    /// Builds an hourly trend over the trailing window ending at `now`.
    fn hourly_trends(&self, records: &[AuditRecord], now: DateTime<Utc>) -> Vec<HourlyMetric> {
        let window = self.config.trend_hours.max(1);
        let start = floor_to_hour(now - Duration::hours(window - 1));
        let mut buckets: Vec<HourlyMetric> = (0..window)
            .map(|offset| HourlyMetric {
                hour: start + Duration::hours(offset),
                count: 0,
                overrides: 0,
            })
            .collect();

        for record in records {
            let bucket_hour = floor_to_hour(record.timestamp);
            if bucket_hour < start || bucket_hour > floor_to_hour(now) {
                continue;
            }
            let idx = (bucket_hour - start).num_hours();
            if idx < 0 || idx as usize >= buckets.len() {
                continue;
            }
            let bucket = &mut buckets[idx as usize];
            bucket.count += 1;
            if matches!(record.result, DecisionResult::Overridden { .. }) {
                bucket.overrides += 1;
            }
        }
        buckets
    }

    /// Derives alerts from the tenant's statistics and recent activity.
    fn alerts(
        &self,
        tenant_id: &TenantId,
        stats: &TenantStats,
        hourly: &[HourlyMetric],
        now: DateTime<Utc>,
    ) -> Vec<AlertInfo> {
        let mut alerts = Vec::new();
        if stats.total_decisions > 0 && stats.override_rate > self.config.override_rate_alert {
            alerts.push(AlertInfo {
                id: Uuid::new_v4(),
                alert_type: AlertType::HighOverrideRate,
                message: format!(
                    "tenant '{tenant_id}' override rate {:.1}% exceeds threshold {:.1}%",
                    stats.override_rate * 100.0,
                    self.config.override_rate_alert * 100.0
                ),
                severity: EventSeverity::High,
                triggered_at: now,
                related_records: Vec::new(),
            });
        }
        if stats.total_decisions > 0 && stats.void_rate > self.config.void_rate_alert {
            alerts.push(AlertInfo {
                id: Uuid::new_v4(),
                alert_type: AlertType::ComplianceViolation,
                message: format!(
                    "tenant '{tenant_id}' void rate {:.1}% exceeds threshold {:.1}%",
                    stats.void_rate * 100.0,
                    self.config.void_rate_alert * 100.0
                ),
                severity: EventSeverity::Warning,
                triggered_at: now,
                related_records: Vec::new(),
            });
        }
        if let Some(peak) = hourly.iter().map(|h| h.count).max()
            && peak > self.config.hourly_volume_alert
        {
            alerts.push(AlertInfo {
                id: Uuid::new_v4(),
                alert_type: AlertType::VolumeSpike,
                message: format!(
                    "tenant '{tenant_id}' peak hourly volume {peak} exceeds threshold {}",
                    self.config.hourly_volume_alert
                ),
                severity: EventSeverity::Warning,
                triggered_at: now,
                related_records: Vec::new(),
            });
        }
        alerts
    }
}

impl Default for TenantDashboard {
    fn default() -> Self {
        Self::new()
    }
}

/// A compact, isolation-safe tile summarising one tenant for an operator
/// overview. Contains only aggregate values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantTile {
    /// The tenant.
    pub tenant_id: TenantId,
    /// Total decisions.
    pub total_decisions: usize,
    /// Override rate in `[0, 1]`.
    pub override_rate: f64,
    /// Void rate in `[0, 1]`.
    pub void_rate: f64,
    /// Latest activity timestamp, if any.
    pub last_activity: Option<DateTime<Utc>>,
    /// Number of alerts triggered for this tenant.
    pub alert_count: usize,
}

/// A cohort-level overview built only from per-tenant aggregates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiTenantOverview {
    /// When the overview was produced.
    pub generated_at: DateTime<Utc>,
    /// Number of tenants.
    pub tenant_count: usize,
    /// Total decisions across all tenants.
    pub total_records: usize,
    /// One tile per tenant, sorted by descending volume.
    pub tiles: Vec<TenantTile>,
}

impl MultiTenantOverview {
    /// Builds an overview across every tenant in the store.
    pub fn build(store: &TenantStore, dashboard: &TenantDashboard) -> AuditResult<Self> {
        let report = CrossTenantAnalytics::report(store)?;
        let mut tiles = Vec::with_capacity(report.per_tenant.len());
        for stats in &report.per_tenant {
            let snapshot = dashboard.render(store, &stats.tenant_id)?;
            tiles.push(TenantTile {
                tenant_id: stats.tenant_id.clone(),
                total_decisions: stats.total_decisions,
                override_rate: stats.override_rate,
                void_rate: stats.void_rate,
                last_activity: stats.last_activity,
                alert_count: snapshot.alerts.len(),
            });
        }
        tiles.sort_by(|a, b| {
            b.total_decisions
                .cmp(&a.total_decisions)
                .then_with(|| a.tenant_id.cmp(&b.tenant_id))
        });
        Ok(Self {
            generated_at: Utc::now(),
            tenant_count: report.tenant_count,
            total_records: report.total_decisions,
            tiles,
        })
    }
}

/// Tallies records into a distribution keyed by `key`.
fn distribution<F>(records: &[AuditRecord], key: F) -> HashMap<String, usize>
where
    F: Fn(&AuditRecord) -> String,
{
    let mut map: HashMap<String, usize> = HashMap::new();
    for record in records {
        *map.entry(key(record)).or_insert(0) += 1;
    }
    map
}

/// A short label describing a record's actor.
fn actor_label(record: &AuditRecord) -> String {
    match &record.actor {
        crate::Actor::System { component } => format!("system:{component}"),
        crate::Actor::User { role, .. } => format!("user:{role}"),
        crate::Actor::External { system_id } => format!("external:{system_id}"),
    }
}

/// Truncates a timestamp to the start of its hour.
fn floor_to_hour(ts: DateTime<Utc>) -> DateTime<Utc> {
    ts.with_minute(0)
        .and_then(|t| t.with_second(0))
        .and_then(|t| t.with_nanosecond(0))
        .unwrap_or(ts)
}

#[cfg(test)]
mod tests {
    use super::super::TenantContext;
    use super::super::tests::{sample_record, tid};
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

    fn overridden(statute: &str) -> AuditRecord {
        AuditRecord::new(
            EventType::HumanOverride,
            Actor::User {
                user_id: "u".to_string(),
                role: "reviewer".to_string(),
            },
            statute.to_string(),
            uuid::Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Overridden {
                original_result: Box::new(DecisionResult::Void {
                    reason: "x".to_string(),
                }),
                new_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "ok".to_string(),
                    parameters: StdHashMap::new(),
                }),
                justification: "manual".to_string(),
            },
            None,
        )
    }

    #[test]
    fn test_render_snapshot_distributions() {
        let store = TenantStore::new();
        let ctx = TenantContext::new(tid("tenant-a"));
        store.record(&ctx, sample_record("alpha")).expect("r");
        store.record(&ctx, sample_record("alpha")).expect("r");
        store.record(&ctx, sample_record("beta")).expect("r");

        let dash = TenantDashboard::new();
        let snap = dash.render(&store, &tid("tenant-a")).expect("render");
        assert_eq!(snap.stats.total_decisions, 3);
        assert_eq!(snap.statute_distribution.get("alpha"), Some(&2));
        assert_eq!(snap.statute_distribution.get("beta"), Some(&1));
        assert_eq!(snap.actor_distribution.get("system:engine"), Some(&3));
        assert_eq!(snap.hourly_trends.len(), 24);
    }

    #[test]
    fn test_high_override_alert() {
        let store = TenantStore::new();
        let ctx = TenantContext::new(tid("tenant-a"));
        for _ in 0..6 {
            store.record(&ctx, overridden("s")).expect("r");
        }
        for _ in 0..4 {
            store.record(&ctx, sample_record("s")).expect("r");
        }
        let dash = TenantDashboard::new();
        let snap = dash.render(&store, &tid("tenant-a")).expect("render");
        assert!(
            snap.alerts
                .iter()
                .any(|a| a.alert_type == AlertType::HighOverrideRate)
        );
    }

    #[test]
    fn test_overview_is_isolation_safe() {
        let store = TenantStore::new();
        let ctx_a = TenantContext::new(tid("tenant-a"));
        let ctx_b = TenantContext::new(tid("tenant-b"));
        for _ in 0..5 {
            store.record(&ctx_a, sample_record("s")).expect("a");
        }
        store.record(&ctx_b, sample_record("s")).expect("b");

        let dash = TenantDashboard::new();
        let overview = MultiTenantOverview::build(&store, &dash).expect("overview");
        assert_eq!(overview.tenant_count, 2);
        assert_eq!(overview.total_records, 6);
        // Sorted by descending volume.
        assert_eq!(overview.tiles[0].tenant_id, tid("tenant-a"));
        // No raw records leak into the overview.
        let json = serde_json::to_string(&overview).expect("serialize");
        assert!(!json.contains("record_hash"));
    }
}
