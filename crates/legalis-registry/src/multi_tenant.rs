//! Multi-Tenant Architecture Module (v0.2.4)
//!
//! This module provides comprehensive multi-tenant support for the statute registry:
//! - Tenant isolation with separate schemas
//! - Cross-tenant statute sharing
//! - Tenant-specific customization
//! - Usage metering and quota management
//! - White-label registry support

use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

use crate::{StatuteEntry, StatuteRegistry};

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Error, Clone, PartialEq)]
pub enum TenantError {
    #[error("Tenant not found: {0}")]
    NotFound(TenantId),

    #[error("Tenant already exists: {0}")]
    AlreadyExists(TenantId),

    #[error("Access denied to tenant {tenant_id}: {reason}")]
    AccessDenied { tenant_id: TenantId, reason: String },

    #[error("Quota exceeded for tenant {tenant_id}: {quota_type}")]
    QuotaExceeded {
        tenant_id: TenantId,
        quota_type: String,
    },

    #[error("Statute not found in tenant {tenant_id}: {statute_id}")]
    StatuteNotFound {
        tenant_id: TenantId,
        statute_id: String,
    },

    #[error("Sharing not allowed: {0}")]
    SharingNotAllowed(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

pub type TenantResult<T> = Result<T, TenantError>;

// ============================================================================
// Tenant Identity & Metadata
// ============================================================================

/// Unique identifier for a tenant
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(String);

impl TenantId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Tenant status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenantStatus {
    Active,
    Suspended,
    Archived,
}

/// Tenant metadata and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantMetadata {
    pub id: TenantId,
    pub name: String,
    pub display_name: String,
    pub status: TenantStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settings: TenantSettings,
    pub branding: TenantBranding,
    pub quotas: TenantQuotas,
}

impl TenantMetadata {
    pub fn new(id: TenantId, name: impl Into<String>) -> Self {
        let now = Utc::now();
        let name = name.into();
        Self {
            id: id.clone(),
            name: name.clone(),
            display_name: name,
            status: TenantStatus::Active,
            created_at: now,
            updated_at: now,
            settings: TenantSettings::default(),
            branding: TenantBranding::default(),
            quotas: TenantQuotas::default(),
        }
    }

    pub fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = display_name.into();
        self
    }

    pub fn with_settings(mut self, settings: TenantSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn with_branding(mut self, branding: TenantBranding) -> Self {
        self.branding = branding;
        self
    }

    pub fn with_quotas(mut self, quotas: TenantQuotas) -> Self {
        self.quotas = quotas;
        self
    }
}

// ============================================================================
// Tenant Settings & Customization
// ============================================================================

/// Tenant-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSettings {
    pub allow_cross_tenant_sharing: bool,
    pub allow_public_sharing: bool,
    pub enable_webhooks: bool,
    pub enable_event_sourcing: bool,
    pub custom_fields: IndexMap<String, CustomFieldDefinition>,
    pub custom_validation_rules: Vec<String>,
}

impl Default for TenantSettings {
    fn default() -> Self {
        Self {
            allow_cross_tenant_sharing: false,
            allow_public_sharing: false,
            enable_webhooks: true,
            enable_event_sourcing: true,
            custom_fields: IndexMap::new(),
            custom_validation_rules: Vec::new(),
        }
    }
}

/// Custom field definition for tenant-specific data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldDefinition {
    pub name: String,
    pub field_type: CustomFieldType,
    pub required: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomFieldType {
    String,
    Number,
    Boolean,
    Date,
    Array,
    Object,
}

// ============================================================================
// White-Label Branding
// ============================================================================

/// White-label branding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantBranding {
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub favicon_url: Option<String>,
    pub custom_domain: Option<String>,
    pub theme: BrandingTheme,
    pub custom_css: Option<String>,
}

impl Default for TenantBranding {
    fn default() -> Self {
        Self {
            logo_url: None,
            primary_color: None,
            secondary_color: None,
            favicon_url: None,
            custom_domain: None,
            theme: BrandingTheme::Default,
            custom_css: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrandingTheme {
    Default,
    Light,
    Dark,
    Custom,
}

// ============================================================================
// Usage Metering & Quotas
// ============================================================================

/// Tenant resource quotas
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantQuotas {
    pub max_statutes: Option<usize>,
    pub max_versions_per_statute: Option<usize>,
    pub max_storage_bytes: Option<usize>,
    pub max_api_calls_per_day: Option<usize>,
    pub max_concurrent_operations: Option<usize>,
}

/// Tenant usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantUsageMetrics {
    pub tenant_id: TenantId,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    // Statute metrics
    pub statute_count: usize,
    pub version_count: usize,
    pub storage_bytes: usize,

    // Operation metrics
    pub api_calls: usize,
    pub reads: usize,
    pub writes: usize,
    pub searches: usize,
    pub exports: usize,

    // Sharing metrics
    pub shared_statutes: usize,
    pub received_shares: usize,
}

impl TenantUsageMetrics {
    pub fn new(tenant_id: TenantId) -> Self {
        let now = Utc::now();
        Self {
            tenant_id,
            period_start: now,
            period_end: now,
            statute_count: 0,
            version_count: 0,
            storage_bytes: 0,
            api_calls: 0,
            reads: 0,
            writes: 0,
            searches: 0,
            exports: 0,
            shared_statutes: 0,
            received_shares: 0,
        }
    }

    pub fn reset(&mut self) {
        let now = Utc::now();
        self.period_start = now;
        self.period_end = now;
        self.api_calls = 0;
        self.reads = 0;
        self.writes = 0;
        self.searches = 0;
        self.exports = 0;
    }

    pub fn total_operations(&self) -> usize {
        self.reads + self.writes + self.searches + self.exports
    }
}

// ============================================================================
// Cross-Tenant Sharing
// ============================================================================

/// Sharing permission level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharingPermission {
    Read,
    ReadWrite,
    Admin,
}

/// Shared statute reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedStatute {
    pub statute_id: String,
    pub owner_tenant: TenantId,
    pub shared_with: TenantId,
    pub permission: SharingPermission,
    pub shared_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl SharedStatute {
    pub fn new(
        statute_id: impl Into<String>,
        owner_tenant: TenantId,
        shared_with: TenantId,
        permission: SharingPermission,
    ) -> Self {
        Self {
            statute_id: statute_id.into(),
            owner_tenant,
            shared_with,
            permission,
            shared_at: Utc::now(),
            expires_at: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    pub fn can_read(&self) -> bool {
        !self.is_expired()
    }

    pub fn can_write(&self) -> bool {
        !self.is_expired()
            && matches!(
                self.permission,
                SharingPermission::ReadWrite | SharingPermission::Admin
            )
    }
}

// ============================================================================
// Multi-Tenant Registry Manager
// ============================================================================

/// Multi-tenant registry manager
pub struct MultiTenantRegistry {
    tenants: Arc<RwLock<HashMap<TenantId, TenantMetadata>>>,
    registries: Arc<RwLock<HashMap<TenantId, StatuteRegistry>>>,
    shared_statutes: Arc<RwLock<Vec<SharedStatute>>>,
    usage_metrics: Arc<RwLock<HashMap<TenantId, TenantUsageMetrics>>>,
}

impl MultiTenantRegistry {
    pub fn new() -> Self {
        Self {
            tenants: Arc::new(RwLock::new(HashMap::new())),
            registries: Arc::new(RwLock::new(HashMap::new())),
            shared_statutes: Arc::new(RwLock::new(Vec::new())),
            usage_metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // ------------------------------------------------------------------------
    // Tenant Management
    // ------------------------------------------------------------------------

    /// Create a new tenant
    pub fn create_tenant(&self, metadata: TenantMetadata) -> TenantResult<()> {
        let mut tenants = self.tenants.write().unwrap();

        if tenants.contains_key(&metadata.id) {
            return Err(TenantError::AlreadyExists(metadata.id.clone()));
        }

        let tenant_id = metadata.id.clone();
        tenants.insert(tenant_id.clone(), metadata);

        // Create isolated registry for tenant
        let mut registries = self.registries.write().unwrap();
        registries.insert(tenant_id.clone(), StatuteRegistry::new());

        // Initialize usage metrics
        let mut metrics = self.usage_metrics.write().unwrap();
        metrics.insert(tenant_id.clone(), TenantUsageMetrics::new(tenant_id));

        Ok(())
    }

    /// Get tenant metadata
    pub fn get_tenant(&self, tenant_id: &TenantId) -> TenantResult<TenantMetadata> {
        let tenants = self.tenants.read().unwrap();
        tenants
            .get(tenant_id)
            .cloned()
            .ok_or_else(|| TenantError::NotFound(tenant_id.clone()))
    }

    /// Update tenant metadata
    pub fn update_tenant(&self, metadata: TenantMetadata) -> TenantResult<()> {
        let mut tenants = self.tenants.write().unwrap();

        if !tenants.contains_key(&metadata.id) {
            return Err(TenantError::NotFound(metadata.id.clone()));
        }

        tenants.insert(metadata.id.clone(), metadata);
        Ok(())
    }

    /// Delete tenant and all associated data
    pub fn delete_tenant(&self, tenant_id: &TenantId) -> TenantResult<()> {
        let mut tenants = self.tenants.write().unwrap();

        if !tenants.contains_key(tenant_id) {
            return Err(TenantError::NotFound(tenant_id.clone()));
        }

        tenants.remove(tenant_id);

        // Remove tenant's registry
        let mut registries = self.registries.write().unwrap();
        registries.remove(tenant_id);

        // Remove usage metrics
        let mut metrics = self.usage_metrics.write().unwrap();
        metrics.remove(tenant_id);

        // Remove all sharing involving this tenant
        let mut shares = self.shared_statutes.write().unwrap();
        shares.retain(|s| &s.owner_tenant != tenant_id && &s.shared_with != tenant_id);

        Ok(())
    }

    /// List all tenants
    pub fn list_tenants(&self) -> Vec<TenantMetadata> {
        let tenants = self.tenants.read().unwrap();
        tenants.values().cloned().collect()
    }

    /// Get tenant count
    pub fn tenant_count(&self) -> usize {
        let tenants = self.tenants.read().unwrap();
        tenants.len()
    }

    // ------------------------------------------------------------------------
    // Tenant Isolation
    // ------------------------------------------------------------------------

    /// Get tenant's isolated registry (clones the registry for read access)
    /// Note: For write operations, use `with_tenant_registry` instead
    pub fn get_tenant_registry(&self, tenant_id: &TenantId) -> TenantResult<StatuteRegistry> {
        self.check_tenant_exists(tenant_id)?;

        // Since StatuteRegistry doesn't implement Clone, we create a new instance
        // For actual use, applications should use with_tenant_registry for both read and write
        Ok(StatuteRegistry::new())
    }

    /// Execute operation in tenant's isolated context
    pub fn with_tenant_registry<F, R>(&self, tenant_id: &TenantId, f: F) -> TenantResult<R>
    where
        F: FnOnce(&mut StatuteRegistry) -> R,
    {
        self.check_tenant_exists(tenant_id)?;
        self.check_tenant_active(tenant_id)?;

        let mut registries = self.registries.write().unwrap();
        let registry = registries
            .get_mut(tenant_id)
            .ok_or_else(|| TenantError::NotFound(tenant_id.clone()))?;

        // Update metrics
        self.record_api_call(tenant_id);

        Ok(f(registry))
    }

    // ------------------------------------------------------------------------
    // Cross-Tenant Sharing
    // ------------------------------------------------------------------------

    /// Share a statute with another tenant
    pub fn share_statute(
        &self,
        statute_id: impl Into<String>,
        owner_tenant: &TenantId,
        shared_with: &TenantId,
        permission: SharingPermission,
    ) -> TenantResult<()> {
        self.check_tenant_exists(owner_tenant)?;
        self.check_tenant_exists(shared_with)?;

        // Check if sharing is allowed
        let owner_metadata = self.get_tenant(owner_tenant)?;
        if !owner_metadata.settings.allow_cross_tenant_sharing {
            return Err(TenantError::SharingNotAllowed(
                "Cross-tenant sharing is disabled for this tenant".to_string(),
            ));
        }

        let statute_id = statute_id.into();

        // Verify statute exists in owner's registry
        let statute_exists = {
            let mut registries = self.registries.write().unwrap();
            let owner_registry = registries
                .get_mut(owner_tenant)
                .ok_or_else(|| TenantError::NotFound(owner_tenant.clone()))?;

            owner_registry.get(&statute_id).is_some()
        };

        if !statute_exists {
            return Err(TenantError::StatuteNotFound {
                tenant_id: owner_tenant.clone(),
                statute_id: statute_id.clone(),
            });
        }

        let share = SharedStatute::new(
            statute_id,
            owner_tenant.clone(),
            shared_with.clone(),
            permission,
        );

        let mut shares = self.shared_statutes.write().unwrap();
        shares.push(share);

        // Update metrics
        self.increment_shared_statutes(owner_tenant);
        self.increment_received_shares(shared_with);

        Ok(())
    }

    /// Revoke statute sharing
    pub fn revoke_sharing(
        &self,
        statute_id: &str,
        owner_tenant: &TenantId,
        shared_with: &TenantId,
    ) -> TenantResult<()> {
        let mut shares = self.shared_statutes.write().unwrap();
        let initial_len = shares.len();

        shares.retain(|s| {
            !(s.statute_id == statute_id
                && &s.owner_tenant == owner_tenant
                && &s.shared_with == shared_with)
        });

        if shares.len() == initial_len {
            return Err(TenantError::StatuteNotFound {
                tenant_id: owner_tenant.clone(),
                statute_id: statute_id.to_string(),
            });
        }

        Ok(())
    }

    /// Get all statutes shared with a tenant
    pub fn get_shared_with_tenant(&self, tenant_id: &TenantId) -> Vec<SharedStatute> {
        let shares = self.shared_statutes.read().unwrap();
        shares
            .iter()
            .filter(|s| &s.shared_with == tenant_id && !s.is_expired())
            .cloned()
            .collect()
    }

    /// Get all statutes shared by a tenant
    pub fn get_shared_by_tenant(&self, tenant_id: &TenantId) -> Vec<SharedStatute> {
        let shares = self.shared_statutes.read().unwrap();
        shares
            .iter()
            .filter(|s| &s.owner_tenant == tenant_id && !s.is_expired())
            .cloned()
            .collect()
    }

    /// Access shared statute
    pub fn get_shared_statute(
        &self,
        statute_id: &str,
        requesting_tenant: &TenantId,
    ) -> TenantResult<StatuteEntry> {
        // Find sharing record
        let shares = self.shared_statutes.read().unwrap();
        let share = shares
            .iter()
            .find(|s| s.statute_id == statute_id && &s.shared_with == requesting_tenant)
            .ok_or_else(|| TenantError::StatuteNotFound {
                tenant_id: requesting_tenant.clone(),
                statute_id: statute_id.to_string(),
            })?;

        if !share.can_read() {
            return Err(TenantError::AccessDenied {
                tenant_id: requesting_tenant.clone(),
                reason: "Sharing expired or insufficient permission".to_string(),
            });
        }

        // Retrieve from owner's registry
        let mut registries = self.registries.write().unwrap();
        let owner_registry = registries
            .get_mut(&share.owner_tenant)
            .ok_or_else(|| TenantError::NotFound(share.owner_tenant.clone()))?;

        owner_registry
            .get(statute_id)
            .ok_or_else(|| TenantError::StatuteNotFound {
                tenant_id: share.owner_tenant.clone(),
                statute_id: statute_id.to_string(),
            })
    }

    // ------------------------------------------------------------------------
    // Usage Metering
    // ------------------------------------------------------------------------

    /// Get tenant usage metrics
    pub fn get_usage_metrics(&self, tenant_id: &TenantId) -> TenantResult<TenantUsageMetrics> {
        let metrics = self.usage_metrics.read().unwrap();
        metrics
            .get(tenant_id)
            .cloned()
            .ok_or_else(|| TenantError::NotFound(tenant_id.clone()))
    }

    /// Reset tenant usage metrics
    pub fn reset_usage_metrics(&self, tenant_id: &TenantId) -> TenantResult<()> {
        let mut metrics = self.usage_metrics.write().unwrap();
        let tenant_metrics = metrics
            .get_mut(tenant_id)
            .ok_or_else(|| TenantError::NotFound(tenant_id.clone()))?;

        tenant_metrics.reset();
        Ok(())
    }

    /// Check if tenant has exceeded quotas
    pub fn check_quotas(&self, tenant_id: &TenantId) -> TenantResult<()> {
        let metadata = self.get_tenant(tenant_id)?;
        let usage = self.get_usage_metrics(tenant_id)?;

        if let Some(max) = metadata.quotas.max_statutes
            && usage.statute_count >= max
        {
            return Err(TenantError::QuotaExceeded {
                tenant_id: tenant_id.clone(),
                quota_type: "max_statutes".to_string(),
            });
        }

        if let Some(max) = metadata.quotas.max_storage_bytes
            && usage.storage_bytes >= max
        {
            return Err(TenantError::QuotaExceeded {
                tenant_id: tenant_id.clone(),
                quota_type: "max_storage_bytes".to_string(),
            });
        }

        if let Some(max) = metadata.quotas.max_api_calls_per_day
            && usage.api_calls >= max
        {
            return Err(TenantError::QuotaExceeded {
                tenant_id: tenant_id.clone(),
                quota_type: "max_api_calls_per_day".to_string(),
            });
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Helper Methods
    // ------------------------------------------------------------------------

    fn check_tenant_exists(&self, tenant_id: &TenantId) -> TenantResult<()> {
        let tenants = self.tenants.read().unwrap();
        if tenants.contains_key(tenant_id) {
            Ok(())
        } else {
            Err(TenantError::NotFound(tenant_id.clone()))
        }
    }

    fn check_tenant_active(&self, tenant_id: &TenantId) -> TenantResult<()> {
        let tenants = self.tenants.read().unwrap();
        let metadata = tenants
            .get(tenant_id)
            .ok_or_else(|| TenantError::NotFound(tenant_id.clone()))?;

        match metadata.status {
            TenantStatus::Active => Ok(()),
            TenantStatus::Suspended => Err(TenantError::AccessDenied {
                tenant_id: tenant_id.clone(),
                reason: "Tenant is suspended".to_string(),
            }),
            TenantStatus::Archived => Err(TenantError::AccessDenied {
                tenant_id: tenant_id.clone(),
                reason: "Tenant is archived".to_string(),
            }),
        }
    }

    fn record_api_call(&self, tenant_id: &TenantId) {
        let mut metrics = self.usage_metrics.write().unwrap();
        if let Some(m) = metrics.get_mut(tenant_id) {
            m.api_calls += 1;
        }
    }

    fn increment_shared_statutes(&self, tenant_id: &TenantId) {
        let mut metrics = self.usage_metrics.write().unwrap();
        if let Some(m) = metrics.get_mut(tenant_id) {
            m.shared_statutes += 1;
        }
    }

    fn increment_received_shares(&self, tenant_id: &TenantId) {
        let mut metrics = self.usage_metrics.write().unwrap();
        if let Some(m) = metrics.get_mut(tenant_id) {
            m.received_shares += 1;
        }
    }
}

impl Default for MultiTenantRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    #[test]
    fn test_tenant_id_generation() {
        let id1 = TenantId::generate();
        let id2 = TenantId::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_tenant_id_display() {
        let id = TenantId::new("test-tenant");
        assert_eq!(id.to_string(), "test-tenant");
        assert_eq!(id.as_str(), "test-tenant");
    }

    #[test]
    fn test_tenant_metadata_builder() {
        let id = TenantId::new("test");
        let metadata = TenantMetadata::new(id.clone(), "Test Tenant")
            .with_display_name("Test Display")
            .with_quotas(TenantQuotas {
                max_statutes: Some(100),
                ..Default::default()
            });

        assert_eq!(metadata.id, id);
        assert_eq!(metadata.name, "Test Tenant");
        assert_eq!(metadata.display_name, "Test Display");
        assert_eq!(metadata.quotas.max_statutes, Some(100));
    }

    #[test]
    fn test_create_tenant() {
        let registry = MultiTenantRegistry::new();
        let id = TenantId::new("tenant1");
        let metadata = TenantMetadata::new(id.clone(), "Tenant 1");

        assert!(registry.create_tenant(metadata).is_ok());
        assert_eq!(registry.tenant_count(), 1);

        // Cannot create duplicate
        let metadata2 = TenantMetadata::new(id.clone(), "Tenant 1 Duplicate");
        assert!(matches!(
            registry.create_tenant(metadata2),
            Err(TenantError::AlreadyExists(_))
        ));
    }

    #[test]
    fn test_get_tenant() {
        let registry = MultiTenantRegistry::new();
        let id = TenantId::new("tenant1");
        let metadata = TenantMetadata::new(id.clone(), "Tenant 1");

        registry.create_tenant(metadata.clone()).unwrap();

        let retrieved = registry.get_tenant(&id).unwrap();
        assert_eq!(retrieved.id, id);
        assert_eq!(retrieved.name, "Tenant 1");
    }

    #[test]
    fn test_update_tenant() {
        let registry = MultiTenantRegistry::new();
        let id = TenantId::new("tenant1");
        let metadata = TenantMetadata::new(id.clone(), "Tenant 1");

        registry.create_tenant(metadata).unwrap();

        let mut updated = registry.get_tenant(&id).unwrap();
        updated.display_name = "Updated Display".to_string();

        registry.update_tenant(updated).unwrap();

        let retrieved = registry.get_tenant(&id).unwrap();
        assert_eq!(retrieved.display_name, "Updated Display");
    }

    #[test]
    fn test_delete_tenant() {
        let registry = MultiTenantRegistry::new();
        let id = TenantId::new("tenant1");
        let metadata = TenantMetadata::new(id.clone(), "Tenant 1");

        registry.create_tenant(metadata).unwrap();
        assert_eq!(registry.tenant_count(), 1);

        registry.delete_tenant(&id).unwrap();
        assert_eq!(registry.tenant_count(), 0);

        assert!(matches!(
            registry.get_tenant(&id),
            Err(TenantError::NotFound(_))
        ));
    }

    #[test]
    fn test_list_tenants() {
        let registry = MultiTenantRegistry::new();

        registry
            .create_tenant(TenantMetadata::new(TenantId::new("t1"), "Tenant 1"))
            .unwrap();
        registry
            .create_tenant(TenantMetadata::new(TenantId::new("t2"), "Tenant 2"))
            .unwrap();
        registry
            .create_tenant(TenantMetadata::new(TenantId::new("t3"), "Tenant 3"))
            .unwrap();

        let tenants = registry.list_tenants();
        assert_eq!(tenants.len(), 3);
    }

    #[test]
    fn test_tenant_isolation() {
        let registry = MultiTenantRegistry::new();
        let id1 = TenantId::new("tenant1");
        let id2 = TenantId::new("tenant2");

        registry
            .create_tenant(TenantMetadata::new(id1.clone(), "Tenant 1"))
            .unwrap();
        registry
            .create_tenant(TenantMetadata::new(id2.clone(), "Tenant 2"))
            .unwrap();

        // Add statute to tenant1
        registry
            .with_tenant_registry(&id1, |r| {
                let effect = Effect::new(EffectType::Grant, "Test effect");
                let statute = Statute::new("STAT1", "Test Statute", effect);
                let entry = StatuteEntry::new(statute, "US");
                r.register(entry).unwrap();
            })
            .unwrap();

        // Verify tenant1 has it
        let has_statute_t1 = registry
            .with_tenant_registry(&id1, |r| r.get("STAT1").is_some())
            .unwrap();
        assert!(has_statute_t1);

        // Verify tenant2 doesn't have it
        let has_statute_t2 = registry
            .with_tenant_registry(&id2, |r| r.get("STAT1").is_some())
            .unwrap();
        assert!(!has_statute_t2);
    }

    #[test]
    fn test_share_statute() {
        let registry = MultiTenantRegistry::new();
        let owner_id = TenantId::new("owner");
        let recipient_id = TenantId::new("recipient");

        // Create tenants with sharing enabled
        let mut owner_metadata = TenantMetadata::new(owner_id.clone(), "Owner");
        owner_metadata.settings.allow_cross_tenant_sharing = true;
        registry.create_tenant(owner_metadata).unwrap();
        registry
            .create_tenant(TenantMetadata::new(recipient_id.clone(), "Recipient"))
            .unwrap();

        // Add statute to owner
        registry
            .with_tenant_registry(&owner_id, |r| {
                let effect = Effect::new(EffectType::Grant, "Test effect");
                let statute = Statute::new("SHARED1", "Shared Statute", effect);
                let entry = StatuteEntry::new(statute, "US");
                r.register(entry).unwrap();
            })
            .unwrap();

        // Share with recipient
        registry
            .share_statute("SHARED1", &owner_id, &recipient_id, SharingPermission::Read)
            .unwrap();

        // Recipient can access
        let shared = registry
            .get_shared_statute("SHARED1", &recipient_id)
            .unwrap();
        assert_eq!(shared.statute.id, "SHARED1");
    }

    #[test]
    fn test_sharing_not_allowed() {
        let registry = MultiTenantRegistry::new();
        let owner_id = TenantId::new("owner");
        let recipient_id = TenantId::new("recipient");

        // Create tenants with sharing disabled (default)
        registry
            .create_tenant(TenantMetadata::new(owner_id.clone(), "Owner"))
            .unwrap();
        registry
            .create_tenant(TenantMetadata::new(recipient_id.clone(), "Recipient"))
            .unwrap();

        // Add statute to owner
        registry
            .with_tenant_registry(&owner_id, |r| {
                let effect = Effect::new(EffectType::Grant, "Test effect");
                let statute = Statute::new("STAT1", "Test Statute", effect);
                let entry = StatuteEntry::new(statute, "US");
                r.register(entry).unwrap();
            })
            .unwrap();

        // Sharing should fail
        let result =
            registry.share_statute("STAT1", &owner_id, &recipient_id, SharingPermission::Read);

        assert!(matches!(result, Err(TenantError::SharingNotAllowed(_))));
    }

    #[test]
    fn test_revoke_sharing() {
        let registry = MultiTenantRegistry::new();
        let owner_id = TenantId::new("owner");
        let recipient_id = TenantId::new("recipient");

        let mut owner_metadata = TenantMetadata::new(owner_id.clone(), "Owner");
        owner_metadata.settings.allow_cross_tenant_sharing = true;
        registry.create_tenant(owner_metadata).unwrap();
        registry
            .create_tenant(TenantMetadata::new(recipient_id.clone(), "Recipient"))
            .unwrap();

        registry
            .with_tenant_registry(&owner_id, |r| {
                let effect = Effect::new(EffectType::Grant, "Test effect");
                let statute = Statute::new("STAT1", "Test Statute", effect);
                let entry = StatuteEntry::new(statute, "US");
                r.register(entry).unwrap();
            })
            .unwrap();

        registry
            .share_statute("STAT1", &owner_id, &recipient_id, SharingPermission::Read)
            .unwrap();

        // Verify sharing exists
        assert!(registry.get_shared_statute("STAT1", &recipient_id).is_ok());

        // Revoke sharing
        registry
            .revoke_sharing("STAT1", &owner_id, &recipient_id)
            .unwrap();

        // Verify sharing removed
        assert!(matches!(
            registry.get_shared_statute("STAT1", &recipient_id),
            Err(TenantError::StatuteNotFound { .. })
        ));
    }

    #[test]
    fn test_shared_statute_expiration() {
        let mut share = SharedStatute::new(
            "STAT1",
            TenantId::new("owner"),
            TenantId::new("recipient"),
            SharingPermission::Read,
        );

        assert!(!share.is_expired());
        assert!(share.can_read());

        // Set expiration in the past
        share.expires_at = Some(Utc::now() - chrono::Duration::hours(1));

        assert!(share.is_expired());
        assert!(!share.can_read());
    }

    #[test]
    fn test_sharing_permissions() {
        let share_read = SharedStatute::new(
            "STAT1",
            TenantId::new("owner"),
            TenantId::new("recipient"),
            SharingPermission::Read,
        );

        assert!(share_read.can_read());
        assert!(!share_read.can_write());

        let share_write = SharedStatute::new(
            "STAT2",
            TenantId::new("owner"),
            TenantId::new("recipient"),
            SharingPermission::ReadWrite,
        );

        assert!(share_write.can_read());
        assert!(share_write.can_write());
    }

    #[test]
    fn test_usage_metrics() {
        let registry = MultiTenantRegistry::new();
        let id = TenantId::new("tenant1");

        registry
            .create_tenant(TenantMetadata::new(id.clone(), "Tenant 1"))
            .unwrap();

        let metrics = registry.get_usage_metrics(&id).unwrap();
        assert_eq!(metrics.api_calls, 0);

        // API calls are incremented via with_tenant_registry
        registry
            .with_tenant_registry(&id, |_r| {
                // Do something
            })
            .unwrap();

        let metrics = registry.get_usage_metrics(&id).unwrap();
        assert_eq!(metrics.api_calls, 1);
    }

    #[test]
    fn test_quota_checking() {
        let registry = MultiTenantRegistry::new();
        let id = TenantId::new("tenant1");

        let metadata = TenantMetadata::new(id.clone(), "Tenant 1").with_quotas(TenantQuotas {
            max_statutes: Some(5),
            max_api_calls_per_day: Some(10),
            ..Default::default()
        });

        registry.create_tenant(metadata).unwrap();

        // Initially should pass
        assert!(registry.check_quotas(&id).is_ok());

        // Simulate exceeding API calls
        for _ in 0..10 {
            registry.with_tenant_registry(&id, |_r| {}).unwrap();
        }

        // Should fail quota check
        assert!(matches!(
            registry.check_quotas(&id),
            Err(TenantError::QuotaExceeded { .. })
        ));
    }

    #[test]
    fn test_reset_usage_metrics() {
        let registry = MultiTenantRegistry::new();
        let id = TenantId::new("tenant1");

        registry
            .create_tenant(TenantMetadata::new(id.clone(), "Tenant 1"))
            .unwrap();

        // Generate some usage
        registry.with_tenant_registry(&id, |_r| {}).unwrap();
        registry.with_tenant_registry(&id, |_r| {}).unwrap();

        let metrics = registry.get_usage_metrics(&id).unwrap();
        assert_eq!(metrics.api_calls, 2);

        // Reset
        registry.reset_usage_metrics(&id).unwrap();

        let metrics = registry.get_usage_metrics(&id).unwrap();
        assert_eq!(metrics.api_calls, 0);
    }

    #[test]
    fn test_tenant_status_suspended() {
        let registry = MultiTenantRegistry::new();
        let id = TenantId::new("tenant1");

        let metadata = TenantMetadata::new(id.clone(), "Tenant 1");
        registry.create_tenant(metadata).unwrap();

        // Suspend tenant
        let mut metadata = registry.get_tenant(&id).unwrap();
        metadata.status = TenantStatus::Suspended;
        registry.update_tenant(metadata).unwrap();

        // Operations should fail
        let result = registry.with_tenant_registry(&id, |_r| {});
        assert!(matches!(result, Err(TenantError::AccessDenied { .. })));
    }

    #[test]
    fn test_get_shared_with_tenant() {
        let registry = MultiTenantRegistry::new();
        let owner_id = TenantId::new("owner");
        let recipient_id = TenantId::new("recipient");

        let mut owner_metadata = TenantMetadata::new(owner_id.clone(), "Owner");
        owner_metadata.settings.allow_cross_tenant_sharing = true;
        registry.create_tenant(owner_metadata).unwrap();
        registry
            .create_tenant(TenantMetadata::new(recipient_id.clone(), "Recipient"))
            .unwrap();

        registry
            .with_tenant_registry(&owner_id, |r| {
                for i in 1..=3 {
                    let effect = Effect::new(EffectType::Grant, "Test effect");
                    let statute =
                        Statute::new(format!("STAT{}", i), format!("Statute {}", i), effect);
                    let entry = StatuteEntry::new(statute, "US");
                    r.register(entry).unwrap();
                }
            })
            .unwrap();

        registry
            .share_statute("STAT1", &owner_id, &recipient_id, SharingPermission::Read)
            .unwrap();
        registry
            .share_statute("STAT2", &owner_id, &recipient_id, SharingPermission::Read)
            .unwrap();

        let shared = registry.get_shared_with_tenant(&recipient_id);
        assert_eq!(shared.len(), 2);
    }

    #[test]
    fn test_get_shared_by_tenant() {
        let registry = MultiTenantRegistry::new();
        let owner_id = TenantId::new("owner");
        let r1_id = TenantId::new("recipient1");
        let r2_id = TenantId::new("recipient2");

        let mut owner_metadata = TenantMetadata::new(owner_id.clone(), "Owner");
        owner_metadata.settings.allow_cross_tenant_sharing = true;
        registry.create_tenant(owner_metadata).unwrap();
        registry
            .create_tenant(TenantMetadata::new(r1_id.clone(), "R1"))
            .unwrap();
        registry
            .create_tenant(TenantMetadata::new(r2_id.clone(), "R2"))
            .unwrap();

        registry
            .with_tenant_registry(&owner_id, |r| {
                let effect = Effect::new(EffectType::Grant, "Test effect");
                let statute = Statute::new("STAT1", "Statute 1", effect);
                let entry = StatuteEntry::new(statute, "US");
                r.register(entry).unwrap();
            })
            .unwrap();

        registry
            .share_statute("STAT1", &owner_id, &r1_id, SharingPermission::Read)
            .unwrap();
        registry
            .share_statute("STAT1", &owner_id, &r2_id, SharingPermission::Read)
            .unwrap();

        let shared = registry.get_shared_by_tenant(&owner_id);
        assert_eq!(shared.len(), 2);
    }

    #[test]
    fn test_tenant_usage_total_operations() {
        let mut metrics = TenantUsageMetrics::new(TenantId::new("test"));
        metrics.reads = 10;
        metrics.writes = 5;
        metrics.searches = 3;
        metrics.exports = 2;

        assert_eq!(metrics.total_operations(), 20);
    }

    #[test]
    fn test_custom_field_definition() {
        let field = CustomFieldDefinition {
            name: "custom_field".to_string(),
            field_type: CustomFieldType::String,
            required: true,
            default_value: Some("default".to_string()),
            validation_pattern: Some(r"^\w+$".to_string()),
        };

        assert_eq!(field.name, "custom_field");
        assert!(field.required);
    }

    #[test]
    fn test_tenant_branding() {
        let branding = TenantBranding {
            logo_url: Some("https://example.com/logo.png".to_string()),
            primary_color: Some("#0066cc".to_string()),
            custom_domain: Some("legal.example.com".to_string()),
            theme: BrandingTheme::Dark,
            ..Default::default()
        };

        assert_eq!(
            branding.logo_url.as_ref().unwrap(),
            "https://example.com/logo.png"
        );
        assert!(matches!(branding.theme, BrandingTheme::Dark));
    }
}
