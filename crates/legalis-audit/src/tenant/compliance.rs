//! Tenant-scoped compliance reporting.
//!
//! [`TenantComplianceReporter`] produces a per-tenant [`TenantComplianceReport`]
//! that reuses the crate's existing [`crate::ComplianceReport`] for the decision
//! breakdown, adds a per-tenant integrity verification result, and — when a
//! retention policy is supplied — a [`RetentionComplianceStatus`] describing
//! whether the tenant currently holds any records past their retention horizon.

use super::TenantId;
use super::isolation::TenantStore;
use super::retention::TenantRetentionManager;
use crate::retention::RetentionPolicy;
use crate::{AuditRecord, AuditResult, ComplianceReport, DecisionResult};
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Whether a tenant's holdings satisfy its retention policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionComplianceStatus {
    /// The configured retention horizon in days.
    pub max_age_days: i64,
    /// Number of records held past the retention horizon.
    pub overdue_records: usize,
    /// `true` when no records are overdue.
    pub compliant: bool,
}

/// A per-tenant compliance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantComplianceReport {
    /// The tenant the report describes.
    pub tenant_id: TenantId,
    /// The reused decision-breakdown compliance report.
    pub report: ComplianceReport,
    /// Number of voided decisions (not broken out by [`ComplianceReport`]).
    pub voided_decisions: usize,
    /// Whether the tenant's per-tenant chain verified.
    pub integrity_verified: bool,
    /// Retention compliance, present only when a policy applies to the tenant.
    pub retention_status: Option<RetentionComplianceStatus>,
}

impl TenantComplianceReport {
    /// Returns `true` if the tenant passes both integrity and (when present)
    /// retention compliance.
    pub fn is_compliant(&self) -> bool {
        self.integrity_verified
            && self
                .retention_status
                .as_ref()
                .map(|s| s.compliant)
                .unwrap_or(true)
    }
}

/// Produces tenant-scoped compliance reports from a [`TenantStore`].
pub struct TenantComplianceReporter;

impl TenantComplianceReporter {
    /// Builds a compliance report for a single tenant.
    ///
    /// When `policy` is `Some`, retention compliance is evaluated against it.
    pub fn report(
        store: &TenantStore,
        tenant_id: &TenantId,
        policy: Option<&RetentionPolicy>,
    ) -> AuditResult<TenantComplianceReport> {
        let records = store.records_for(tenant_id)?;
        let integrity_verified = store.verify_tenant(tenant_id).unwrap_or(false);
        let report = build_compliance_report(&records, integrity_verified);
        let voided_decisions = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Void { .. }))
            .count();
        let retention_status = policy.map(|p| retention_status(p, &records));

        Ok(TenantComplianceReport {
            tenant_id: tenant_id.clone(),
            report,
            voided_decisions,
            integrity_verified,
            retention_status,
        })
    }

    /// Builds compliance reports for every tenant in the store, resolving each
    /// tenant's effective retention policy through the supplied manager.
    pub fn report_all(
        store: &TenantStore,
        manager: &TenantRetentionManager,
    ) -> AuditResult<Vec<TenantComplianceReport>> {
        let mut reports = Vec::new();
        for tenant in store.tenants()? {
            let policy = manager.policy_for(&tenant).cloned();
            reports.push(Self::report(store, &tenant, policy.as_ref())?);
        }
        Ok(reports)
    }
}

/// Builds a [`ComplianceReport`] from a tenant's records.
fn build_compliance_report(records: &[AuditRecord], integrity_verified: bool) -> ComplianceReport {
    let total = records.len();
    let mut automatic = 0usize;
    let mut discretionary = 0usize;
    let mut overrides = 0usize;
    for record in records {
        match record.result {
            DecisionResult::Deterministic { .. } => automatic += 1,
            DecisionResult::RequiresDiscretion { .. } => discretionary += 1,
            DecisionResult::Overridden { .. } => overrides += 1,
            DecisionResult::Void { .. } => {}
        }
    }
    ComplianceReport {
        total_decisions: total,
        automatic_decisions: automatic,
        discretionary_decisions: discretionary,
        human_overrides: overrides,
        integrity_verified,
        generated_at: Utc::now(),
    }
}

/// Evaluates retention compliance of `records` against `policy`.
fn retention_status(
    policy: &RetentionPolicy,
    records: &[AuditRecord],
) -> RetentionComplianceStatus {
    let overdue = policy.records_to_delete(records).len();
    RetentionComplianceStatus {
        max_age_days: policy.max_age.num_days(),
        overdue_records: overdue,
        compliant: overdue == 0,
    }
}

#[cfg(test)]
mod tests {
    use super::super::TenantContext;
    use super::super::tests::{sample_record, tid};
    use super::*;
    use chrono::{Duration, Utc};

    fn aged(statute: &str, days_ago: i64) -> AuditRecord {
        let mut record = sample_record(statute);
        record.timestamp = Utc::now() - Duration::days(days_ago);
        record
    }

    #[test]
    fn test_report_breakdown_and_integrity() {
        let store = TenantStore::new();
        let ctx = TenantContext::new(tid("tenant-a"));
        for _ in 0..4 {
            store.record(&ctx, sample_record("s")).expect("r");
        }
        let report =
            TenantComplianceReporter::report(&store, &tid("tenant-a"), None).expect("report");
        assert_eq!(report.report.total_decisions, 4);
        assert_eq!(report.report.automatic_decisions, 4);
        assert!(report.integrity_verified);
        assert!(report.retention_status.is_none());
        assert!(report.is_compliant());
    }

    #[test]
    fn test_retention_status_flags_overdue() {
        let store = TenantStore::new();
        let ctx = TenantContext::new(tid("tenant-a"));
        store.record(&ctx, aged("s", 5)).expect("fresh");
        store.record(&ctx, aged("s", 400)).expect("old");

        let policy = RetentionPolicy::new(30);
        let report = TenantComplianceReporter::report(&store, &tid("tenant-a"), Some(&policy))
            .expect("report");
        let status = report.retention_status.as_ref().expect("status");
        assert_eq!(status.max_age_days, 30);
        assert_eq!(status.overdue_records, 1);
        assert!(!status.compliant);
        assert!(!report.is_compliant());
    }

    #[test]
    fn test_report_all_uses_manager_policies() {
        let store = TenantStore::new();
        let ctx_a = TenantContext::new(tid("tenant-a"));
        let ctx_b = TenantContext::new(tid("tenant-b"));
        store.record(&ctx_a, aged("s", 400)).expect("a");
        store.record(&ctx_b, aged("s", 10)).expect("b");

        let mut manager = TenantRetentionManager::new().with_default(RetentionPolicy::new(365));
        manager.set_policy(tid("tenant-a"), RetentionPolicy::new(30));

        let reports = TenantComplianceReporter::report_all(&store, &manager).expect("reports");
        assert_eq!(reports.len(), 2);
        let report_a = reports
            .iter()
            .find(|r| r.tenant_id == tid("tenant-a"))
            .expect("a");
        // Tenant A's 400-day record is overdue under its 30-day policy.
        assert!(!report_a.is_compliant());
        let report_b = reports
            .iter()
            .find(|r| r.tenant_id == tid("tenant-b"))
            .expect("b");
        // Tenant B's 10-day record is within the 365-day default.
        assert!(report_b.is_compliant());
    }
}
