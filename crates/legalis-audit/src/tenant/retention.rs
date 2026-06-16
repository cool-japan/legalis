//! Tenant-specific retention.
//!
//! [`TenantRetentionManager`] maps each tenant to its own
//! [`crate::RetentionPolicy`], with an optional cohort-wide default for tenants
//! that have not set their own. It can produce a non-destructive [`RetentionPlan`]
//! (what *would* be deleted) and apply retention destructively against a
//! [`TenantStore`], re-anchoring each affected tenant's chain.

use super::TenantId;
use super::isolation::TenantStore;
use crate::retention::RetentionPolicy;
use crate::{AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A non-destructive description of what a retention policy would do to one
/// tenant's records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionPlan {
    /// The tenant the plan applies to.
    pub tenant_id: TenantId,
    /// Total records currently held for the tenant.
    pub total: usize,
    /// Number of records that would be retained.
    pub to_retain: usize,
    /// Number of records that would be deleted.
    pub to_delete: usize,
    /// Whether a tenant-specific policy was used (`false` means the default).
    pub tenant_specific: bool,
}

/// The result of applying retention to one tenant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionOutcome {
    /// The tenant retention was applied to.
    pub tenant_id: TenantId,
    /// Number of records deleted.
    pub deleted: usize,
    /// Number of records retained.
    pub retained: usize,
}

/// Per-tenant retention policy manager.
#[derive(Debug, Clone, Default)]
pub struct TenantRetentionManager {
    default_policy: Option<RetentionPolicy>,
    per_tenant: HashMap<TenantId, RetentionPolicy>,
}

impl TenantRetentionManager {
    /// Creates a manager with no policies.
    pub fn new() -> Self {
        Self {
            default_policy: None,
            per_tenant: HashMap::new(),
        }
    }

    /// Sets the cohort-wide default policy (builder style).
    pub fn with_default(mut self, policy: RetentionPolicy) -> Self {
        self.default_policy = Some(policy);
        self
    }

    /// Sets a tenant-specific policy.
    pub fn set_policy(&mut self, tenant_id: TenantId, policy: RetentionPolicy) {
        self.per_tenant.insert(tenant_id, policy);
    }

    /// Removes a tenant-specific policy (the tenant then falls back to the
    /// default). Returns the removed policy, if any.
    pub fn clear_policy(&mut self, tenant_id: &TenantId) -> Option<RetentionPolicy> {
        self.per_tenant.remove(tenant_id)
    }

    /// Returns the effective policy for a tenant: its own if set, else the
    /// default.
    pub fn policy_for(&self, tenant_id: &TenantId) -> Option<&RetentionPolicy> {
        self.per_tenant
            .get(tenant_id)
            .or(self.default_policy.as_ref())
    }

    /// Returns `true` if the tenant has its own policy.
    pub fn has_tenant_policy(&self, tenant_id: &TenantId) -> bool {
        self.per_tenant.contains_key(tenant_id)
    }

    /// Identifies the records of `records` that the tenant's effective policy
    /// would delete. Returns an empty vector when the tenant has no effective
    /// policy.
    pub fn records_to_delete(
        &self,
        tenant_id: &TenantId,
        records: &[AuditRecord],
    ) -> Vec<AuditRecord> {
        match self.policy_for(tenant_id) {
            Some(policy) => policy.records_to_delete(records),
            None => Vec::new(),
        }
    }

    /// Builds a non-destructive retention plan for a single tenant from the
    /// store.
    pub fn plan_tenant(
        &self,
        store: &TenantStore,
        tenant_id: &TenantId,
    ) -> AuditResult<RetentionPlan> {
        let records = store.records_for(tenant_id)?;
        let total = records.len();
        let (to_delete, tenant_specific) = match self.policy_for(tenant_id) {
            Some(policy) => (
                policy.records_to_delete(&records).len(),
                self.has_tenant_policy(tenant_id),
            ),
            None => (0, false),
        };
        Ok(RetentionPlan {
            tenant_id: tenant_id.clone(),
            total,
            to_retain: total - to_delete,
            to_delete,
            tenant_specific,
        })
    }

    /// Builds retention plans for every tenant in the store.
    pub fn plan_all(&self, store: &TenantStore) -> AuditResult<Vec<RetentionPlan>> {
        let mut plans = Vec::new();
        for tenant in store.tenants()? {
            plans.push(self.plan_tenant(store, &tenant)?);
        }
        Ok(plans)
    }

    /// Applies the effective policy to a single tenant in the store,
    /// destructively removing expired records and re-anchoring the tenant's
    /// chain. Returns the outcome.
    pub fn apply_tenant(
        &self,
        store: &TenantStore,
        tenant_id: &TenantId,
    ) -> AuditResult<RetentionOutcome> {
        let deleted = match self.policy_for(tenant_id) {
            Some(policy) => {
                let policy = policy.clone();
                store.purge_tenant_where(tenant_id, move |r| policy.should_retain(r))?
            }
            None => 0,
        };
        let retained = store.count(tenant_id)?;
        Ok(RetentionOutcome {
            tenant_id: tenant_id.clone(),
            deleted,
            retained,
        })
    }

    /// Applies retention to every tenant in the store.
    pub fn apply_all(&self, store: &TenantStore) -> AuditResult<Vec<RetentionOutcome>> {
        let mut outcomes = Vec::new();
        for tenant in store.tenants()? {
            outcomes.push(self.apply_tenant(store, &tenant)?);
        }
        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {
    use super::super::TenantContext;
    use super::super::tests::{sample_record, tid};
    use super::*;
    use chrono::{Duration, Utc};

    fn aged_record(statute: &str, days_ago: i64) -> AuditRecord {
        let mut record = sample_record(statute);
        record.timestamp = Utc::now() - Duration::days(days_ago);
        record
    }

    fn populate() -> TenantStore {
        let store = TenantStore::new();
        let ctx_a = TenantContext::new(tid("tenant-a"));
        let ctx_b = TenantContext::new(tid("tenant-b"));
        // Tenant A: a mix of fresh and very old records.
        store.record(&ctx_a, aged_record("s", 1)).expect("a fresh");
        store.record(&ctx_a, aged_record("s", 5)).expect("a recent");
        store.record(&ctx_a, aged_record("s", 400)).expect("a old");
        store
            .record(&ctx_a, aged_record("s", 500))
            .expect("a older");
        // Tenant B: all old records.
        store.record(&ctx_b, aged_record("s", 100)).expect("b");
        store.record(&ctx_b, aged_record("s", 200)).expect("b");
        store
    }

    #[test]
    fn test_policy_resolution() {
        let mut mgr = TenantRetentionManager::new().with_default(RetentionPolicy::new(365));
        mgr.set_policy(tid("tenant-a"), RetentionPolicy::new(30));
        assert!(mgr.has_tenant_policy(&tid("tenant-a")));
        assert!(!mgr.has_tenant_policy(&tid("tenant-b")));
        // Tenant A uses its own 30-day policy; B falls back to the 365 default.
        assert_eq!(
            mgr.policy_for(&tid("tenant-a")).map(|p| p.max_age),
            Some(Duration::days(30))
        );
        assert_eq!(
            mgr.policy_for(&tid("tenant-b")).map(|p| p.max_age),
            Some(Duration::days(365))
        );
    }

    #[test]
    fn test_plan_is_non_destructive() {
        let store = populate();
        let mut mgr = TenantRetentionManager::new().with_default(RetentionPolicy::new(365));
        mgr.set_policy(tid("tenant-a"), RetentionPolicy::new(30));

        let plan_a = mgr.plan_tenant(&store, &tid("tenant-a")).expect("plan a");
        assert_eq!(plan_a.total, 4);
        assert_eq!(plan_a.to_delete, 2); // the 400- and 500-day-old records
        assert!(plan_a.tenant_specific);

        let plan_b = mgr.plan_tenant(&store, &tid("tenant-b")).expect("plan b");
        assert_eq!(plan_b.to_delete, 0); // both within 365 days
        assert!(!plan_b.tenant_specific);

        // Planning must not have changed the store.
        assert_eq!(store.count(&tid("tenant-a")).expect("count"), 4);
    }

    #[test]
    fn test_apply_deletes_and_reanchors() {
        let store = populate();
        let mut mgr = TenantRetentionManager::new();
        mgr.set_policy(tid("tenant-a"), RetentionPolicy::new(30));

        let outcome = mgr.apply_tenant(&store, &tid("tenant-a")).expect("apply");
        assert_eq!(outcome.deleted, 2);
        assert_eq!(outcome.retained, 2);
        assert_eq!(store.count(&tid("tenant-a")).expect("count"), 2);
        // Re-anchored chain still verifies after the purge.
        assert!(store.verify_tenant(&tid("tenant-a")).expect("verify"));
        // Tenant B has no policy and is untouched.
        assert_eq!(store.count(&tid("tenant-b")).expect("count"), 2);
    }

    #[test]
    fn test_apply_all_respects_per_tenant_policies() {
        let store = populate();
        let mut mgr = TenantRetentionManager::new().with_default(RetentionPolicy::new(365));
        mgr.set_policy(tid("tenant-a"), RetentionPolicy::new(30));
        mgr.set_policy(tid("tenant-b"), RetentionPolicy::new(150));

        let outcomes = mgr.apply_all(&store).expect("apply all");
        let by_tenant: HashMap<TenantId, RetentionOutcome> = outcomes
            .into_iter()
            .map(|o| (o.tenant_id.clone(), o))
            .collect();
        assert_eq!(by_tenant[&tid("tenant-a")].deleted, 2);
        assert_eq!(by_tenant[&tid("tenant-b")].deleted, 1); // only the 200-day record
        assert!(store.verify_all().expect("verify all"));
    }
}
