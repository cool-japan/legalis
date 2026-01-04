//! Enterprise knowledge graph deployment framework.
//!
//! This module provides tools for deploying and managing knowledge graphs in
//! enterprise environments:
//! - Multi-tenant graph management
//! - Scalability configuration
//! - High availability setup
//! - Performance monitoring
//! - Backup and recovery

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Deployment environment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentEnvironment {
    /// Development environment
    Development,
    /// Staging environment
    Staging,
    /// Production environment
    Production,
}

/// Deployment configuration for a knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Environment type
    pub environment: DeploymentEnvironment,
    /// Number of replicas for high availability
    pub replicas: usize,
    /// Enable sharding for horizontal scaling
    pub enable_sharding: bool,
    /// Number of shards (if sharding enabled)
    pub shard_count: usize,
    /// Enable caching
    pub enable_cache: bool,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Enable compression
    pub enable_compression: bool,
    /// Backup frequency in hours
    pub backup_frequency_hours: usize,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Query timeout in seconds
    pub query_timeout_seconds: u64,
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl DeploymentConfig {
    /// Creates a development configuration.
    pub fn development() -> Self {
        Self {
            environment: DeploymentEnvironment::Development,
            replicas: 1,
            enable_sharding: false,
            shard_count: 1,
            enable_cache: true,
            cache_size_mb: 256,
            enable_compression: false,
            backup_frequency_hours: 24,
            max_connections: 10,
            query_timeout_seconds: 30,
        }
    }

    /// Creates a production configuration.
    pub fn production() -> Self {
        Self {
            environment: DeploymentEnvironment::Production,
            replicas: 3,
            enable_sharding: true,
            shard_count: 4,
            enable_cache: true,
            cache_size_mb: 4096,
            enable_compression: true,
            backup_frequency_hours: 6,
            max_connections: 1000,
            query_timeout_seconds: 60,
        }
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), String> {
        if self.replicas == 0 {
            return Err("Replicas must be at least 1".to_string());
        }

        if self.enable_sharding && self.shard_count == 0 {
            return Err("Shard count must be at least 1 when sharding is enabled".to_string());
        }

        if self.max_connections == 0 {
            return Err("Max connections must be at least 1".to_string());
        }

        Ok(())
    }
}

/// Tenant information for multi-tenant deployments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// Unique tenant ID
    pub id: String,
    /// Tenant name
    pub name: String,
    /// Tenant-specific graph URI
    pub graph_uri: String,
    /// Resource quota
    pub quota: ResourceQuota,
    /// Tenant status
    pub active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Tenant {
    /// Creates a new tenant.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        graph_uri: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            graph_uri: graph_uri.into(),
            quota: ResourceQuota::default(),
            active: true,
            created_at: Utc::now(),
        }
    }

    /// Sets the resource quota.
    pub fn with_quota(mut self, quota: ResourceQuota) -> Self {
        self.quota = quota;
        self
    }
}

/// Resource quota for a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuota {
    /// Maximum number of triples
    pub max_triples: Option<usize>,
    /// Maximum storage size in MB
    pub max_storage_mb: Option<usize>,
    /// Maximum queries per minute
    pub max_queries_per_minute: Option<usize>,
}

impl Default for ResourceQuota {
    fn default() -> Self {
        Self {
            max_triples: Some(1_000_000),
            max_storage_mb: Some(1024),
            max_queries_per_minute: Some(100),
        }
    }
}

impl ResourceQuota {
    /// Creates an unlimited quota.
    pub fn unlimited() -> Self {
        Self {
            max_triples: None,
            max_storage_mb: None,
            max_queries_per_minute: None,
        }
    }

    /// Checks if the quota allows adding more triples.
    pub fn can_add_triples(&self, current_count: usize, to_add: usize) -> bool {
        if let Some(max) = self.max_triples {
            current_count + to_add <= max
        } else {
            true
        }
    }
}

/// Deployment instance representing a deployed knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentInstance {
    /// Instance ID
    pub id: String,
    /// Instance name
    pub name: String,
    /// Configuration
    pub config: DeploymentConfig,
    /// Tenants (for multi-tenant deployments)
    pub tenants: Vec<Tenant>,
    /// Deployment timestamp
    pub deployed_at: DateTime<Utc>,
    /// Current status
    pub status: DeploymentStatus,
    /// Health metrics
    pub metrics: HealthMetrics,
}

/// Deployment status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentStatus {
    /// Deployment in progress
    Deploying,
    /// Running and healthy
    Running,
    /// Degraded but operational
    Degraded,
    /// Stopped
    Stopped,
    /// Failed
    Failed,
}

/// Health metrics for a deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage in MB
    pub memory_usage_mb: usize,
    /// Total triples stored
    pub total_triples: usize,
    /// Queries per second
    pub queries_per_second: f64,
    /// Average query latency in ms
    pub avg_query_latency_ms: f64,
    /// Number of active connections
    pub active_connections: usize,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for HealthMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0,
            total_triples: 0,
            queries_per_second: 0.0,
            avg_query_latency_ms: 0.0,
            active_connections: 0,
            last_updated: Utc::now(),
        }
    }
}

impl DeploymentInstance {
    /// Creates a new deployment instance.
    pub fn new(id: impl Into<String>, name: impl Into<String>, config: DeploymentConfig) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            config,
            tenants: Vec::new(),
            deployed_at: Utc::now(),
            status: DeploymentStatus::Deploying,
            metrics: HealthMetrics::default(),
        }
    }

    /// Adds a tenant to the deployment.
    pub fn add_tenant(&mut self, tenant: Tenant) -> Result<(), String> {
        if self.tenants.iter().any(|t| t.id == tenant.id) {
            return Err(format!("Tenant {} already exists", tenant.id));
        }
        self.tenants.push(tenant);
        Ok(())
    }

    /// Gets a tenant by ID.
    pub fn get_tenant(&self, tenant_id: &str) -> Option<&Tenant> {
        self.tenants.iter().find(|t| t.id == tenant_id)
    }

    /// Checks if deployment is healthy.
    pub fn is_healthy(&self) -> bool {
        self.status == DeploymentStatus::Running
            && self.metrics.cpu_usage_percent < 90.0
            && self.metrics.avg_query_latency_ms < 1000.0
    }
}

/// Manager for enterprise deployments.
pub struct DeploymentManager {
    /// All deployments
    deployments: HashMap<String, DeploymentInstance>,
}

impl DeploymentManager {
    /// Creates a new deployment manager.
    pub fn new() -> Self {
        Self {
            deployments: HashMap::new(),
        }
    }

    /// Deploys a new knowledge graph instance.
    pub fn deploy(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        config: DeploymentConfig,
    ) -> Result<String, String> {
        config.validate()?;

        let id_str = id.into();
        if self.deployments.contains_key(&id_str) {
            return Err(format!("Deployment {} already exists", id_str));
        }

        let instance = DeploymentInstance::new(id_str.clone(), name, config);
        self.deployments.insert(id_str.clone(), instance);

        Ok(id_str)
    }

    /// Gets a deployment by ID.
    pub fn get_deployment(&self, id: &str) -> Option<&DeploymentInstance> {
        self.deployments.get(id)
    }

    /// Gets a mutable reference to a deployment.
    pub fn get_deployment_mut(&mut self, id: &str) -> Option<&mut DeploymentInstance> {
        self.deployments.get_mut(id)
    }

    /// Updates deployment status.
    pub fn update_status(&mut self, id: &str, status: DeploymentStatus) -> Result<(), String> {
        let deployment = self
            .deployments
            .get_mut(id)
            .ok_or_else(|| format!("Deployment {} not found", id))?;

        deployment.status = status;
        Ok(())
    }

    /// Updates health metrics.
    pub fn update_metrics(&mut self, id: &str, metrics: HealthMetrics) -> Result<(), String> {
        let deployment = self
            .deployments
            .get_mut(id)
            .ok_or_else(|| format!("Deployment {} not found", id))?;

        deployment.metrics = metrics;
        Ok(())
    }

    /// Gets all deployments.
    pub fn list_deployments(&self) -> Vec<&DeploymentInstance> {
        self.deployments.values().collect()
    }

    /// Gets deployments by status.
    pub fn list_by_status(&self, status: DeploymentStatus) -> Vec<&DeploymentInstance> {
        self.deployments
            .values()
            .filter(|d| d.status == status)
            .collect()
    }

    /// Scales a deployment (updates replica count).
    pub fn scale(&mut self, id: &str, new_replicas: usize) -> Result<(), String> {
        if new_replicas == 0 {
            return Err("Replicas must be at least 1".to_string());
        }

        let deployment = self
            .deployments
            .get_mut(id)
            .ok_or_else(|| format!("Deployment {} not found", id))?;

        deployment.config.replicas = new_replicas;
        Ok(())
    }
}

impl Default for DeploymentManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Backup configuration and management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Backup storage location
    pub storage_location: String,
    /// Retention period in days
    pub retention_days: usize,
    /// Enable incremental backups
    pub incremental: bool,
    /// Compression enabled
    pub compress: bool,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            storage_location: "/backups".to_string(),
            retention_days: 30,
            incremental: true,
            compress: true,
        }
    }
}

/// Represents a backup of a knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    /// Backup ID
    pub id: String,
    /// Deployment ID
    pub deployment_id: String,
    /// Backup timestamp
    pub created_at: DateTime<Utc>,
    /// Size in MB
    pub size_mb: usize,
    /// Number of triples
    pub triple_count: usize,
    /// Is incremental backup
    pub incremental: bool,
    /// Storage path
    pub storage_path: String,
}

impl Backup {
    /// Creates a new backup record.
    pub fn new(
        id: impl Into<String>,
        deployment_id: impl Into<String>,
        triple_count: usize,
    ) -> Self {
        Self {
            id: id.into(),
            deployment_id: deployment_id.into(),
            created_at: Utc::now(),
            size_mb: 0,
            triple_count,
            incremental: false,
            storage_path: String::new(),
        }
    }

    /// Estimates backup size based on triple count.
    pub fn estimate_size(&mut self) {
        // Rough estimate: ~200 bytes per triple when compressed
        self.size_mb = (self.triple_count * 200) / (1024 * 1024);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_config_development() {
        let config = DeploymentConfig::development();
        assert_eq!(config.environment, DeploymentEnvironment::Development);
        assert_eq!(config.replicas, 1);
        assert!(!config.enable_sharding);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_deployment_config_production() {
        let config = DeploymentConfig::production();
        assert_eq!(config.environment, DeploymentEnvironment::Production);
        assert!(config.replicas > 1);
        assert!(config.enable_sharding);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_deployment_config_validation() {
        let mut config = DeploymentConfig::development();
        config.replicas = 0;
        assert!(config.validate().is_err());

        config.replicas = 1;
        config.enable_sharding = true;
        config.shard_count = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_tenant_creation() {
        let tenant = Tenant::new("tenant1", "Test Tenant", "http://example.org/tenant1");
        assert_eq!(tenant.id, "tenant1");
        assert_eq!(tenant.name, "Test Tenant");
        assert!(tenant.active);
    }

    #[test]
    fn test_resource_quota() {
        let quota = ResourceQuota::default();
        assert!(quota.can_add_triples(500_000, 100_000));
        assert!(!quota.can_add_triples(950_000, 100_000));

        let unlimited = ResourceQuota::unlimited();
        assert!(unlimited.can_add_triples(10_000_000, 1_000_000));
    }

    #[test]
    fn test_deployment_instance() {
        let config = DeploymentConfig::development();
        let mut instance = DeploymentInstance::new("deploy1", "Test Deployment", config);

        assert_eq!(instance.id, "deploy1");
        assert_eq!(instance.status, DeploymentStatus::Deploying);

        let tenant = Tenant::new("tenant1", "Tenant 1", "http://example.org/t1");
        assert!(instance.add_tenant(tenant).is_ok());
        assert_eq!(instance.tenants.len(), 1);

        // Try to add duplicate
        let tenant2 = Tenant::new("tenant1", "Tenant 1 Dup", "http://example.org/t1");
        assert!(instance.add_tenant(tenant2).is_err());
    }

    #[test]
    fn test_deployment_manager() {
        let mut manager = DeploymentManager::new();

        let config = DeploymentConfig::development();
        let deploy_id = manager.deploy("deploy1", "Test Deploy", config).unwrap();

        assert_eq!(deploy_id, "deploy1");
        assert!(manager.get_deployment("deploy1").is_some());

        // Try to deploy duplicate
        let config2 = DeploymentConfig::development();
        assert!(manager.deploy("deploy1", "Duplicate", config2).is_err());
    }

    #[test]
    fn test_deployment_status_update() {
        let mut manager = DeploymentManager::new();
        let config = DeploymentConfig::development();
        manager.deploy("deploy1", "Test", config).unwrap();

        manager
            .update_status("deploy1", DeploymentStatus::Running)
            .unwrap();

        let deployment = manager.get_deployment("deploy1").unwrap();
        assert_eq!(deployment.status, DeploymentStatus::Running);
    }

    #[test]
    fn test_deployment_scaling() {
        let mut manager = DeploymentManager::new();
        let config = DeploymentConfig::development();
        manager.deploy("deploy1", "Test", config).unwrap();

        manager.scale("deploy1", 5).unwrap();

        let deployment = manager.get_deployment("deploy1").unwrap();
        assert_eq!(deployment.config.replicas, 5);

        // Try to scale to 0
        assert!(manager.scale("deploy1", 0).is_err());
    }

    #[test]
    fn test_list_deployments_by_status() {
        let mut manager = DeploymentManager::new();

        manager
            .deploy("deploy1", "Test1", DeploymentConfig::development())
            .unwrap();
        manager
            .deploy("deploy2", "Test2", DeploymentConfig::development())
            .unwrap();

        manager
            .update_status("deploy1", DeploymentStatus::Running)
            .unwrap();

        let running = manager.list_by_status(DeploymentStatus::Running);
        assert_eq!(running.len(), 1);

        let deploying = manager.list_by_status(DeploymentStatus::Deploying);
        assert_eq!(deploying.len(), 1);
    }

    #[test]
    fn test_health_check() {
        let config = DeploymentConfig::development();
        let mut instance = DeploymentInstance::new("deploy1", "Test", config);

        instance.status = DeploymentStatus::Running;
        instance.metrics.cpu_usage_percent = 50.0;
        instance.metrics.avg_query_latency_ms = 100.0;

        assert!(instance.is_healthy());

        instance.metrics.cpu_usage_percent = 95.0;
        assert!(!instance.is_healthy());
    }

    #[test]
    fn test_backup_creation() {
        let mut backup = Backup::new("backup1", "deploy1", 1_000_000);
        assert_eq!(backup.id, "backup1");
        assert_eq!(backup.triple_count, 1_000_000);

        backup.estimate_size();
        assert!(backup.size_mb > 0);
    }

    #[test]
    fn test_backup_config() {
        let config = BackupConfig::default();
        assert_eq!(config.retention_days, 30);
        assert!(config.incremental);
        assert!(config.compress);
    }
}
