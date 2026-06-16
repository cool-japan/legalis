//! Cross-tenant analytics with isolation guarantees.
//!
//! The central guarantee of this module is structural: every public output type
//! ([`TenantStats`], [`CrossTenantReport`], [`TenantComparison`]) carries only
//! *aggregate statistics*. No [`AuditRecord`] ever crosses a tenant boundary, so
//! a cross-tenant report can be shared with an operator without leaking one
//! tenant's records into another tenant's view.
//!
//! Analytics are computed from a [`TenantStore`], which already enforces
//! per-tenant isolation, so the aggregation never has to merge raw records from
//! different tenants.

use super::TenantId;
use super::isolation::TenantStore;
use crate::{AuditRecord, AuditResult, DecisionResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Aggregate statistics for a single tenant. Contains no raw records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenantStats {
    /// The tenant these statistics describe.
    pub tenant_id: TenantId,
    /// Total decisions recorded for the tenant.
    pub total_decisions: usize,
    /// Count of deterministic (automatic) decisions.
    pub automatic_decisions: usize,
    /// Count of decisions requiring discretion.
    pub discretionary_decisions: usize,
    /// Count of human overrides.
    pub human_overrides: usize,
    /// Count of voided decisions.
    pub voided_decisions: usize,
    /// Override rate in `[0, 1]`.
    pub override_rate: f64,
    /// Void rate in `[0, 1]`.
    pub void_rate: f64,
    /// Number of distinct subjects.
    pub distinct_subjects: usize,
    /// Number of distinct statutes.
    pub distinct_statutes: usize,
    /// Earliest activity timestamp, if any.
    pub first_activity: Option<DateTime<Utc>>,
    /// Latest activity timestamp, if any.
    pub last_activity: Option<DateTime<Utc>>,
}

impl TenantStats {
    /// Computes statistics for a tenant from its records.
    pub fn compute(tenant_id: TenantId, records: &[AuditRecord]) -> Self {
        let total = records.len();
        let mut automatic = 0usize;
        let mut discretionary = 0usize;
        let mut overrides = 0usize;
        let mut voided = 0usize;
        let mut subjects: HashSet<uuid::Uuid> = HashSet::new();
        let mut statutes: HashSet<String> = HashSet::new();
        let mut first: Option<DateTime<Utc>> = None;
        let mut last: Option<DateTime<Utc>> = None;

        for record in records {
            match record.result {
                DecisionResult::Deterministic { .. } => automatic += 1,
                DecisionResult::RequiresDiscretion { .. } => discretionary += 1,
                DecisionResult::Overridden { .. } => overrides += 1,
                DecisionResult::Void { .. } => voided += 1,
            }
            subjects.insert(record.subject_id);
            statutes.insert(record.statute_id.clone());
            first = Some(match first {
                Some(current) if current <= record.timestamp => current,
                _ => record.timestamp,
            });
            last = Some(match last {
                Some(current) if current >= record.timestamp => current,
                _ => record.timestamp,
            });
        }

        let denom = total.max(1) as f64;
        Self {
            tenant_id,
            total_decisions: total,
            automatic_decisions: automatic,
            discretionary_decisions: discretionary,
            human_overrides: overrides,
            voided_decisions: voided,
            override_rate: overrides as f64 / denom,
            void_rate: voided as f64 / denom,
            distinct_subjects: subjects.len(),
            distinct_statutes: statutes.len(),
            first_activity: first,
            last_activity: last,
        }
    }
}

/// A side-by-side comparison of two tenants' aggregate statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantComparison {
    /// Statistics for the first tenant.
    pub left: TenantStats,
    /// Statistics for the second tenant.
    pub right: TenantStats,
    /// `left.override_rate - right.override_rate`.
    pub override_rate_delta: f64,
    /// `left.void_rate - right.void_rate`.
    pub void_rate_delta: f64,
    /// `left.total_decisions as i64 - right.total_decisions as i64`.
    pub volume_delta: i64,
}

/// A cross-tenant report: per-tenant aggregates plus cohort-level summaries.
///
/// Contains only [`TenantStats`]; no records cross tenant boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossTenantReport {
    /// Per-tenant statistics, one entry per tenant.
    pub per_tenant: Vec<TenantStats>,
    /// Total decisions across the whole cohort.
    pub total_decisions: usize,
    /// Number of tenants in the cohort.
    pub tenant_count: usize,
    /// Mean override rate across tenants (unweighted).
    pub mean_override_rate: f64,
    /// Median-absolute-deviation of override rate across tenants.
    pub override_rate_mad: f64,
    /// When the report was generated.
    pub generated_at: DateTime<Utc>,
}

impl CrossTenantReport {
    /// Returns tenants sorted by descending decision volume.
    pub fn by_volume(&self) -> Vec<&TenantStats> {
        let mut entries: Vec<&TenantStats> = self.per_tenant.iter().collect();
        entries.sort_by(|a, b| {
            b.total_decisions
                .cmp(&a.total_decisions)
                .then_with(|| a.tenant_id.cmp(&b.tenant_id))
        });
        entries
    }

    /// Returns tenants sorted by descending override rate.
    pub fn by_override_rate(&self) -> Vec<&TenantStats> {
        let mut entries: Vec<&TenantStats> = self.per_tenant.iter().collect();
        entries.sort_by(|a, b| {
            b.override_rate
                .total_cmp(&a.override_rate)
                .then_with(|| a.tenant_id.cmp(&b.tenant_id))
        });
        entries
    }

    /// Returns tenants whose override rate is a robust outlier, i.e. whose
    /// modified z-score (Iglewicz–Hoaglin) exceeds `threshold`.
    ///
    /// A common choice is `threshold = 3.5`. The estimator uses the median and
    /// the median absolute deviation (MAD); when more than half the tenants
    /// share an identical rate the MAD collapses to zero, so a scaled
    /// mean-absolute-deviation fallback is used instead. When every tenant has
    /// the same rate no outliers are reported.
    pub fn override_rate_outliers(&self, threshold: f64) -> Vec<&TenantStats> {
        if self.per_tenant.len() < 3 {
            return Vec::new();
        }
        let rates: Vec<f64> = self.per_tenant.iter().map(|s| s.override_rate).collect();
        let med = median(&rates);
        let mad = self.override_rate_mad;
        // 0.6745 scales the MAD to be a consistent estimator of the std-dev;
        // 1.253314 does the same for the mean absolute deviation.
        let modified_z = |x: f64| -> f64 {
            if mad > f64::EPSILON {
                0.6745 * (x - med).abs() / mad
            } else {
                let mean_ad =
                    rates.iter().map(|v| (v - med).abs()).sum::<f64>() / rates.len() as f64;
                if mean_ad <= f64::EPSILON {
                    0.0
                } else {
                    (x - med).abs() / (1.253314 * mean_ad)
                }
            }
        };
        self.per_tenant
            .iter()
            .filter(|s| modified_z(s.override_rate) > threshold)
            .collect()
    }

    /// Returns the percentile rank in `[0, 1]` of a tenant's decision volume
    /// within the cohort (fraction of tenants with a strictly smaller volume).
    /// Returns `None` if the tenant is not present.
    pub fn volume_percentile(&self, tenant_id: &TenantId) -> Option<f64> {
        let target = self
            .per_tenant
            .iter()
            .find(|s| &s.tenant_id == tenant_id)?
            .total_decisions;
        if self.per_tenant.len() <= 1 {
            return Some(1.0);
        }
        let smaller = self
            .per_tenant
            .iter()
            .filter(|s| s.total_decisions < target)
            .count();
        Some(smaller as f64 / (self.per_tenant.len() - 1) as f64)
    }
}

/// Computes cross-tenant analytics from a [`TenantStore`].
pub struct CrossTenantAnalytics;

impl CrossTenantAnalytics {
    /// Builds a full cross-tenant report for every tenant in the store.
    pub fn report(store: &TenantStore) -> AuditResult<CrossTenantReport> {
        let tenants = store.tenants()?;
        let mut per_tenant = Vec::with_capacity(tenants.len());
        for tenant in tenants {
            let records = store.records_for(&tenant)?;
            per_tenant.push(TenantStats::compute(tenant, &records));
        }

        let total_decisions = per_tenant.iter().map(|s| s.total_decisions).sum();
        let tenant_count = per_tenant.len();
        let rates: Vec<f64> = per_tenant.iter().map(|s| s.override_rate).collect();
        let mean_override_rate = if rates.is_empty() {
            0.0
        } else {
            rates.iter().sum::<f64>() / rates.len() as f64
        };
        let override_rate_mad = mad(&rates);

        Ok(CrossTenantReport {
            per_tenant,
            total_decisions,
            tenant_count,
            mean_override_rate,
            override_rate_mad,
            generated_at: Utc::now(),
        })
    }

    /// Computes statistics for a single tenant.
    pub fn tenant_stats(store: &TenantStore, tenant_id: &TenantId) -> AuditResult<TenantStats> {
        let records = store.records_for(tenant_id)?;
        Ok(TenantStats::compute(tenant_id.clone(), &records))
    }

    /// Compares two tenants' aggregate statistics.
    pub fn compare(
        store: &TenantStore,
        left: &TenantId,
        right: &TenantId,
    ) -> AuditResult<TenantComparison> {
        let left_stats = Self::tenant_stats(store, left)?;
        let right_stats = Self::tenant_stats(store, right)?;
        let override_rate_delta = left_stats.override_rate - right_stats.override_rate;
        let void_rate_delta = left_stats.void_rate - right_stats.void_rate;
        let volume_delta = left_stats.total_decisions as i64 - right_stats.total_decisions as i64;
        Ok(TenantComparison {
            left: left_stats,
            right: right_stats,
            override_rate_delta,
            void_rate_delta,
            volume_delta,
        })
    }
}

/// Returns the median of a slice (0.0 for empty input).
fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted: Vec<f64> = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let mid = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

/// Returns the median absolute deviation of a slice (0.0 for empty input).
fn mad(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let med = median(values);
    let deviations: Vec<f64> = values.iter().map(|v| (v - med).abs()).collect();
    median(&deviations)
}

#[cfg(test)]
mod tests {
    use super::super::TenantContext;
    use super::super::tests::{sample_record, tid};
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn overridden_record(statute: &str) -> AuditRecord {
        AuditRecord::new(
            EventType::HumanOverride,
            Actor::User {
                user_id: "u".to_string(),
                role: "reviewer".to_string(),
            },
            statute.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Overridden {
                original_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "deny".to_string(),
                    parameters: HashMap::new(),
                }),
                new_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "approve".to_string(),
                    parameters: HashMap::new(),
                }),
                justification: "manual".to_string(),
            },
            None,
        )
    }

    fn populate() -> TenantStore {
        let store = TenantStore::new();
        let ctx_a = TenantContext::new(tid("tenant-a"));
        let ctx_b = TenantContext::new(tid("tenant-b"));
        // Tenant A: mostly automatic, low override.
        for _ in 0..9 {
            store.record(&ctx_a, sample_record("s-a")).expect("a");
        }
        store
            .record(&ctx_a, overridden_record("s-a"))
            .expect("a-ovr");
        // Tenant B: half overrides (high override rate).
        for _ in 0..2 {
            store.record(&ctx_b, sample_record("s-b")).expect("b");
        }
        for _ in 0..2 {
            store
                .record(&ctx_b, overridden_record("s-b"))
                .expect("b-ovr");
        }
        store
    }

    #[test]
    fn test_tenant_stats_compute() {
        let store = populate();
        let stats = CrossTenantAnalytics::tenant_stats(&store, &tid("tenant-a")).expect("stats");
        assert_eq!(stats.total_decisions, 10);
        assert_eq!(stats.human_overrides, 1);
        assert!((stats.override_rate - 0.1).abs() < 1e-9);
        assert!(stats.first_activity.is_some());
        assert!(stats.last_activity.is_some());
    }

    #[test]
    fn test_cross_tenant_report_aggregates_only() {
        let store = populate();
        let report = CrossTenantAnalytics::report(&store).expect("report");
        assert_eq!(report.tenant_count, 2);
        assert_eq!(report.total_decisions, 14);
        // The report serializes without any record payloads.
        let json = serde_json::to_string(&report).expect("serialize");
        assert!(!json.contains("record_hash"));
        assert!(!json.contains("subject_id"));
    }

    #[test]
    fn test_by_volume_and_override_ordering() {
        let store = populate();
        let report = CrossTenantAnalytics::report(&store).expect("report");
        let by_volume = report.by_volume();
        assert_eq!(by_volume[0].tenant_id, tid("tenant-a"));
        let by_override = report.by_override_rate();
        // Tenant B has the higher override rate (0.5 vs 0.1).
        assert_eq!(by_override[0].tenant_id, tid("tenant-b"));
    }

    #[test]
    fn test_compare_and_percentile() {
        let store = populate();
        let cmp = CrossTenantAnalytics::compare(&store, &tid("tenant-b"), &tid("tenant-a"))
            .expect("compare");
        assert!(cmp.override_rate_delta > 0.0);
        assert_eq!(cmp.volume_delta, 4 - 10);

        let report = CrossTenantAnalytics::report(&store).expect("report");
        let pct = report.volume_percentile(&tid("tenant-a")).expect("pct");
        assert!((pct - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_override_rate_outliers() {
        let store = TenantStore::new();
        // Ten calm tenants and one extreme tenant.
        for i in 0..10 {
            let ctx = TenantContext::new(tid(&format!("calm-{i}")));
            for _ in 0..10 {
                store.record(&ctx, sample_record("s")).expect("calm");
            }
        }
        let loud = TenantContext::new(tid("loud"));
        for _ in 0..10 {
            store.record(&loud, overridden_record("s")).expect("loud");
        }
        let report = CrossTenantAnalytics::report(&store).expect("report");
        let outliers = report.override_rate_outliers(3.5);
        assert!(outliers.iter().any(|s| s.tenant_id == tid("loud")));
    }
}
