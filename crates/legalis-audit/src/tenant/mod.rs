//! Multi-tenant audit support.
//!
//! This module turns the single-tenant audit primitives of the crate into a
//! strictly isolated, multi-tenant system. Every tenant gets its own namespace
//! with an independent hash chain, and the public API makes it structurally hard
//! to leak one tenant's records into another tenant's view.
//!
//! The design is deliberately *additive*: it reuses the existing
//! [`AuditRecord`], [`crate::storage::AuditStorage`], [`crate::RetentionPolicy`],
//! and [`crate::ComplianceReport`] types rather than re-modelling them.
//!
//! ## Tenant identity & context
//! - [`TenantId`] is a validated, namespace-safe identifier.
//! - [`TenantContext`] carries the active tenant (and optional acting principal /
//!   trace id) and is the unit of *context propagation*: it stamps records on
//!   write and decides ownership on read.
//! - [`TenantRegistry`] is the catalogue of known tenants and their metadata.
//!
//! ## Isolation
//! - [`isolation::TenantStore`] is a self-contained multi-tenant store that keeps
//!   each tenant's records in a separate namespace with its own chain. Reads for
//!   one tenant can never observe another tenant's records.
//! - [`isolation::TenantScopedStorage`] wraps *any* existing
//!   [`crate::storage::AuditStorage`] backend and transparently scopes it to a
//!   single tenant, so multi-tenancy composes with SQLite/JSONL/etc.
//!
//! ## Analytics, retention, dashboards, reporting
//! - [`analytics::CrossTenantAnalytics`] computes cross-tenant aggregates with an
//!   isolation guarantee: the report exposes only per-tenant *statistics*, never
//!   raw records.
//! - [`retention::TenantRetentionManager`] maps each tenant to its own
//!   [`crate::RetentionPolicy`].
//! - [`dashboard::TenantDashboard`] renders a tenant-scoped operational snapshot.
//! - [`compliance::TenantComplianceReporter`] produces per-tenant compliance
//!   reports.

pub mod analytics;
pub mod compliance;
pub mod dashboard;
pub mod isolation;
pub mod retention;

pub use analytics::{CrossTenantAnalytics, CrossTenantReport, TenantComparison, TenantStats};
pub use compliance::{RetentionComplianceStatus, TenantComplianceReport, TenantComplianceReporter};
pub use dashboard::{MultiTenantOverview, TenantDashboard, TenantDashboardSnapshot, TenantTile};
pub use isolation::{TenantScopedStorage, TenantStore};
pub use retention::{RetentionOutcome, RetentionPlan, TenantRetentionManager};

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// The metadata key under which a record's owning tenant is recorded.
///
/// The key is intentionally namespaced so it cannot collide with user-supplied
/// metadata. It is never accepted as part of a [`TenantId`] (see
/// [`TenantId::new`]), so a record cannot spoof ownership through its id.
pub const TENANT_METADATA_KEY: &str = "__legalis_tenant";

/// Maximum permitted length of a [`TenantId`].
pub const MAX_TENANT_ID_LEN: usize = 128;

/// A validated, namespace-safe tenant identifier.
///
/// Identifiers are restricted to ASCII alphanumerics plus `-`, `_`, `.`, and `:`
/// so that they are safe to use as storage namespaces, file-name fragments, and
/// map keys without escaping. Empty or over-long identifiers are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TenantId(String);

impl TenantId {
    /// Creates a validated tenant id.
    ///
    /// # Errors
    /// Returns [`AuditError::InvalidRecord`] if the id is empty, longer than
    /// [`MAX_TENANT_ID_LEN`], or contains characters outside the safe set.
    pub fn new(id: impl Into<String>) -> AuditResult<Self> {
        let id = id.into();
        if id.is_empty() {
            return Err(AuditError::InvalidRecord(
                "tenant id must not be empty".to_string(),
            ));
        }
        if id.len() > MAX_TENANT_ID_LEN {
            return Err(AuditError::InvalidRecord(format!(
                "tenant id exceeds {MAX_TENANT_ID_LEN} characters"
            )));
        }
        if !id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ':'))
        {
            return Err(AuditError::InvalidRecord(format!(
                "tenant id '{id}' contains illegal characters (allowed: alphanumeric, '-', '_', '.', ':')"
            )));
        }
        Ok(Self(id))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the id, returning the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for TenantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Reads the owning tenant of a record, if it has been stamped.
///
/// Returns `None` for un-stamped records or records whose stamp is not a valid
/// [`TenantId`].
pub fn tenant_of(record: &AuditRecord) -> Option<TenantId> {
    record
        .context
        .metadata
        .get(TENANT_METADATA_KEY)
        .and_then(|raw| TenantId::new(raw.clone()).ok())
}

/// The active tenant context, used to propagate tenancy through writes and reads.
///
/// A context binds operations to exactly one [`TenantId`]. It is the single point
/// that decides which records belong to the active tenant ([`TenantContext::owns`])
/// and that stamps new records on write ([`TenantContext::stamp`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantContext {
    /// The active tenant.
    pub tenant_id: TenantId,
    /// The acting principal within the tenant (optional, for attribution).
    pub actor: Option<String>,
    /// A correlation / trace id for context propagation (optional).
    pub trace_id: Option<String>,
}

impl TenantContext {
    /// Creates a context for the given tenant.
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            actor: None,
            trace_id: None,
        }
    }

    /// Sets the acting principal (builder style).
    pub fn with_actor(mut self, actor: impl Into<String>) -> Self {
        self.actor = Some(actor.into());
        self
    }

    /// Sets the correlation / trace id (builder style).
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Stamps a record with this context's tenant id.
    ///
    /// The tenant id is written into the record's context metadata. Because the
    /// record hash does not cover metadata, stamping never invalidates an
    /// existing hash chain.
    pub fn stamp(&self, record: &mut AuditRecord) {
        record.context.metadata.insert(
            TENANT_METADATA_KEY.to_string(),
            self.tenant_id.as_str().to_string(),
        );
    }

    /// Returns `true` if the record belongs to this context's tenant.
    pub fn owns(&self, record: &AuditRecord) -> bool {
        matches!(tenant_of(record), Some(t) if t == self.tenant_id)
    }
}

/// Service tier for a tenant. Tiers can drive default quotas, retention, and
/// dashboard styling but carry no behaviour of their own here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TenantTier {
    /// No-cost tier.
    Free,
    /// Standard paid tier.
    Standard,
    /// Premium tier.
    Premium,
    /// Enterprise tier with the highest guarantees.
    Enterprise,
}

impl TenantTier {
    /// A short, stable label for the tier.
    pub fn label(&self) -> &'static str {
        match self {
            TenantTier::Free => "free",
            TenantTier::Standard => "standard",
            TenantTier::Premium => "premium",
            TenantTier::Enterprise => "enterprise",
        }
    }
}

/// Descriptive metadata for a registered tenant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantMetadata {
    /// The tenant identifier.
    pub tenant_id: TenantId,
    /// Human-readable display name.
    pub display_name: String,
    /// Service tier.
    pub tier: TenantTier,
    /// Data residency region (e.g. `eu-west-1`), free-form.
    pub data_region: String,
    /// When the tenant was registered.
    pub created_at: DateTime<Utc>,
    /// Whether the tenant is currently active. Inactive tenants are retained for
    /// audit purposes but cannot mint new [`TenantContext`]s via the registry.
    pub active: bool,
}

impl TenantMetadata {
    /// Creates active metadata with the current timestamp.
    pub fn new(tenant_id: TenantId, display_name: impl Into<String>, tier: TenantTier) -> Self {
        Self {
            tenant_id,
            display_name: display_name.into(),
            tier,
            data_region: "default".to_string(),
            created_at: Utc::now(),
            active: true,
        }
    }

    /// Sets the data region (builder style).
    pub fn with_data_region(mut self, region: impl Into<String>) -> Self {
        self.data_region = region.into();
        self
    }
}

/// A catalogue of known tenants and their [`TenantMetadata`].
///
/// The registry enforces unique tenant ids and is the authority on whether a
/// tenant is active. It does not store any audit records itself.
#[derive(Debug, Clone, Default)]
pub struct TenantRegistry {
    tenants: HashMap<TenantId, TenantMetadata>,
}

impl TenantRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self {
            tenants: HashMap::new(),
        }
    }

    /// Registers a new tenant.
    ///
    /// # Errors
    /// Returns [`AuditError::InvalidRecord`] if the tenant id is already
    /// registered.
    pub fn register(&mut self, metadata: TenantMetadata) -> AuditResult<()> {
        if self.tenants.contains_key(&metadata.tenant_id) {
            return Err(AuditError::InvalidRecord(format!(
                "tenant '{}' is already registered",
                metadata.tenant_id
            )));
        }
        self.tenants.insert(metadata.tenant_id.clone(), metadata);
        Ok(())
    }

    /// Convenience constructor that registers a fresh active tenant from parts.
    pub fn register_new(
        &mut self,
        tenant_id: TenantId,
        display_name: impl Into<String>,
        tier: TenantTier,
    ) -> AuditResult<()> {
        self.register(TenantMetadata::new(tenant_id, display_name, tier))
    }

    /// Returns the metadata for a tenant, if registered.
    pub fn get(&self, tenant_id: &TenantId) -> Option<&TenantMetadata> {
        self.tenants.get(tenant_id)
    }

    /// Returns `true` if the tenant is registered and active.
    pub fn is_active(&self, tenant_id: &TenantId) -> bool {
        matches!(self.tenants.get(tenant_id), Some(m) if m.active)
    }

    /// Marks a tenant inactive.
    ///
    /// # Errors
    /// Returns [`AuditError::InvalidRecord`] if the tenant is not registered.
    pub fn deactivate(&mut self, tenant_id: &TenantId) -> AuditResult<()> {
        match self.tenants.get_mut(tenant_id) {
            Some(metadata) => {
                metadata.active = false;
                Ok(())
            }
            None => Err(AuditError::InvalidRecord(format!(
                "tenant '{tenant_id}' is not registered"
            ))),
        }
    }

    /// Marks a tenant active.
    ///
    /// # Errors
    /// Returns [`AuditError::InvalidRecord`] if the tenant is not registered.
    pub fn activate(&mut self, tenant_id: &TenantId) -> AuditResult<()> {
        match self.tenants.get_mut(tenant_id) {
            Some(metadata) => {
                metadata.active = true;
                Ok(())
            }
            None => Err(AuditError::InvalidRecord(format!(
                "tenant '{tenant_id}' is not registered"
            ))),
        }
    }

    /// Builds an active [`TenantContext`] for a registered, active tenant.
    ///
    /// # Errors
    /// Returns [`AuditError::InvalidRecord`] if the tenant is unknown or inactive.
    pub fn context_for(&self, tenant_id: &TenantId) -> AuditResult<TenantContext> {
        match self.tenants.get(tenant_id) {
            Some(metadata) if metadata.active => Ok(TenantContext::new(tenant_id.clone())),
            Some(_) => Err(AuditError::InvalidRecord(format!(
                "tenant '{tenant_id}' is inactive"
            ))),
            None => Err(AuditError::InvalidRecord(format!(
                "tenant '{tenant_id}' is not registered"
            ))),
        }
    }

    /// Lists all registered tenants, sorted by id.
    pub fn list(&self) -> Vec<&TenantMetadata> {
        let mut all: Vec<&TenantMetadata> = self.tenants.values().collect();
        all.sort_by(|a, b| a.tenant_id.cmp(&b.tenant_id));
        all
    }

    /// Returns the ids of all active tenants, sorted.
    pub fn active_tenants(&self) -> Vec<TenantId> {
        let mut ids: Vec<TenantId> = self
            .tenants
            .values()
            .filter(|m| m.active)
            .map(|m| m.tenant_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Number of registered tenants.
    pub fn len(&self) -> usize {
        self.tenants.len()
    }

    /// Returns `true` if no tenants are registered.
    pub fn is_empty(&self) -> bool {
        self.tenants.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;
    use uuid::Uuid;

    pub(crate) fn tid(s: &str) -> TenantId {
        TenantId::new(s).expect("valid tenant id")
    }

    pub(crate) fn sample_record(statute: &str) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "engine".to_string(),
            },
            statute.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_tenant_id_validation() {
        assert!(TenantId::new("acme-corp_1.eu").is_ok());
        assert!(TenantId::new("ns:sub").is_ok());
        assert!(TenantId::new("").is_err());
        assert!(TenantId::new("has space").is_err());
        assert!(TenantId::new("slash/here").is_err());
        assert!(TenantId::new("x".repeat(MAX_TENANT_ID_LEN + 1)).is_err());
    }

    #[test]
    fn test_context_stamp_and_owns() {
        let ctx = TenantContext::new(tid("tenant-a"));
        let mut record = sample_record("s-1");
        assert!(!ctx.owns(&record));
        ctx.stamp(&mut record);
        assert!(ctx.owns(&record));
        assert_eq!(tenant_of(&record), Some(tid("tenant-a")));
    }

    #[test]
    fn test_stamp_preserves_hash() {
        let ctx = TenantContext::new(tid("tenant-a"));
        let mut record = sample_record("s-1");
        let original_hash = record.record_hash.clone();
        ctx.stamp(&mut record);
        // Metadata is not part of the record hash, so the chain stays valid.
        assert_eq!(record.record_hash, original_hash);
        assert!(record.verify());
    }

    #[test]
    fn test_registry_register_and_context() {
        let mut registry = TenantRegistry::new();
        registry
            .register_new(tid("tenant-a"), "Tenant A", TenantTier::Premium)
            .expect("register");
        // Duplicate registration fails.
        assert!(
            registry
                .register_new(tid("tenant-a"), "dup", TenantTier::Free)
                .is_err()
        );
        assert!(registry.is_active(&tid("tenant-a")));
        assert!(registry.context_for(&tid("tenant-a")).is_ok());
        assert!(registry.context_for(&tid("ghost")).is_err());
    }

    #[test]
    fn test_registry_deactivate_blocks_context() {
        let mut registry = TenantRegistry::new();
        registry
            .register_new(tid("tenant-a"), "Tenant A", TenantTier::Standard)
            .expect("register");
        registry.deactivate(&tid("tenant-a")).expect("deactivate");
        assert!(!registry.is_active(&tid("tenant-a")));
        assert!(registry.context_for(&tid("tenant-a")).is_err());
        assert!(registry.active_tenants().is_empty());
        registry.activate(&tid("tenant-a")).expect("activate");
        assert!(registry.context_for(&tid("tenant-a")).is_ok());
    }

    #[test]
    fn test_registry_list_sorted() {
        let mut registry = TenantRegistry::new();
        registry
            .register_new(tid("zeta"), "Z", TenantTier::Free)
            .expect("register");
        registry
            .register_new(tid("alpha"), "A", TenantTier::Free)
            .expect("register");
        let ids: Vec<&str> = registry
            .list()
            .iter()
            .map(|m| m.tenant_id.as_str())
            .collect();
        assert_eq!(ids, vec!["alpha", "zeta"]);
        assert_eq!(registry.len(), 2);
    }
}
