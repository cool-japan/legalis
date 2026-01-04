//! Schema stitching for microservices.
//!
//! This module provides schema stitching capabilities to combine multiple GraphQL schemas
//! from different microservices into a single unified schema.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A remote GraphQL service that can be stitched into the main schema.
#[derive(Debug, Clone)]
pub struct RemoteService {
    /// Service name
    pub name: String,
    /// Service URL/endpoint
    pub endpoint: String,
    /// GraphQL schema definition (SDL)
    pub schema_sdl: String,
    /// Type definitions owned by this service
    pub owned_types: Vec<String>,
    /// Whether this service is healthy
    pub healthy: bool,
}

impl RemoteService {
    /// Create a new remote service.
    pub fn new(name: String, endpoint: String, schema_sdl: String) -> Self {
        Self {
            name,
            endpoint,
            schema_sdl,
            owned_types: Vec::new(),
            healthy: true,
        }
    }

    /// Add types owned by this service.
    pub fn with_owned_types(mut self, types: Vec<String>) -> Self {
        self.owned_types = types;
        self
    }

    /// Set service health status.
    pub fn set_healthy(&mut self, healthy: bool) {
        self.healthy = healthy;
    }
}

/// Configuration for schema stitching.
#[derive(Debug, Clone)]
pub struct StitchingConfig {
    /// Maximum number of remote services
    pub max_services: usize,
    /// Request timeout for remote services (ms)
    pub request_timeout_ms: u64,
    /// Whether to enable type merging
    pub enable_type_merging: bool,
    /// Whether to validate schemas on registration
    pub validate_on_register: bool,
}

impl Default for StitchingConfig {
    fn default() -> Self {
        Self {
            max_services: 10,
            request_timeout_ms: 5000,
            enable_type_merging: true,
            validate_on_register: true,
        }
    }
}

/// Manager for schema stitching.
pub struct SchemaStitcher {
    /// Registered remote services
    services: Arc<RwLock<HashMap<String, RemoteService>>>,
    /// Stitched schema (SDL)
    stitched_schema: Arc<RwLock<Option<String>>>,
    /// Type to service mapping
    type_owners: Arc<RwLock<HashMap<String, String>>>,
    /// Configuration
    config: StitchingConfig,
}

impl SchemaStitcher {
    /// Create a new schema stitcher.
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            stitched_schema: Arc::new(RwLock::new(None)),
            type_owners: Arc::new(RwLock::new(HashMap::new())),
            config: StitchingConfig::default(),
        }
    }

    /// Create a new schema stitcher with custom configuration.
    pub fn with_config(config: StitchingConfig) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            stitched_schema: Arc::new(RwLock::new(None)),
            type_owners: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Register a remote service.
    pub async fn register_service(&self, service: RemoteService) -> Result<(), StitchingError> {
        let mut services = self.services.write().await;

        // Check service limit
        if services.len() >= self.config.max_services {
            return Err(StitchingError::TooManyServices {
                limit: self.config.max_services,
            });
        }

        // Validate schema if enabled
        if self.config.validate_on_register {
            self.validate_schema(&service.schema_sdl)?;
        }

        // Update type owners
        {
            let mut type_owners = self.type_owners.write().await;
            for type_name in &service.owned_types {
                type_owners.insert(type_name.clone(), service.name.clone());
            }
        }

        // Register service
        services.insert(service.name.clone(), service);

        // Invalidate stitched schema (needs to be regenerated)
        {
            let mut stitched = self.stitched_schema.write().await;
            *stitched = None;
        }

        Ok(())
    }

    /// Unregister a remote service.
    pub async fn unregister_service(&self, service_name: &str) -> Result<(), StitchingError> {
        let mut services = self.services.write().await;

        let service =
            services
                .remove(service_name)
                .ok_or_else(|| StitchingError::ServiceNotFound {
                    name: service_name.to_string(),
                })?;

        // Remove type owners
        {
            let mut type_owners = self.type_owners.write().await;
            for type_name in &service.owned_types {
                type_owners.remove(type_name);
            }
        }

        // Invalidate stitched schema
        {
            let mut stitched = self.stitched_schema.write().await;
            *stitched = None;
        }

        Ok(())
    }

    /// Get the stitched schema SDL.
    pub async fn get_stitched_schema(&self) -> Result<String, StitchingError> {
        // Check if we have a cached version
        {
            let stitched = self.stitched_schema.read().await;
            if let Some(ref schema) = *stitched {
                return Ok(schema.clone());
            }
        }

        // Generate stitched schema
        let schema = self.generate_stitched_schema().await?;

        // Cache it
        {
            let mut stitched = self.stitched_schema.write().await;
            *stitched = Some(schema.clone());
        }

        Ok(schema)
    }

    /// Generate the stitched schema from all registered services.
    async fn generate_stitched_schema(&self) -> Result<String, StitchingError> {
        let services = self.services.read().await;

        if services.is_empty() {
            return Err(StitchingError::NoServices);
        }

        let mut stitched = String::new();

        // Combine schemas (simplified - in production, use proper GraphQL schema merging)
        stitched.push_str("# Stitched Schema\n\n");

        for (name, service) in services.iter() {
            if !service.healthy {
                tracing::warn!("Service {} is unhealthy, skipping from schema", name);
                continue;
            }

            stitched.push_str(&format!("# From service: {}\n", name));
            stitched.push_str(&service.schema_sdl);
            stitched.push('\n');
        }

        Ok(stitched)
    }

    /// Validate a GraphQL schema SDL.
    fn validate_schema(&self, _schema_sdl: &str) -> Result<(), StitchingError> {
        // Simplified validation - in production, parse and validate the SDL
        // For now, just check if it's not empty
        if _schema_sdl.trim().is_empty() {
            return Err(StitchingError::InvalidSchema {
                reason: "Schema is empty".to_string(),
            });
        }

        Ok(())
    }

    /// Route a query to the appropriate service.
    pub async fn route_query(&self, type_name: &str) -> Result<String, StitchingError> {
        let type_owners = self.type_owners.read().await;

        let service_name =
            type_owners
                .get(type_name)
                .ok_or_else(|| StitchingError::TypeNotFound {
                    type_name: type_name.to_string(),
                })?;

        let services = self.services.read().await;
        let service =
            services
                .get(service_name)
                .ok_or_else(|| StitchingError::ServiceNotFound {
                    name: service_name.to_string(),
                })?;

        Ok(service.endpoint.clone())
    }

    /// Get all registered services.
    pub async fn get_services(&self) -> Vec<ServiceInfo> {
        let services = self.services.read().await;

        services
            .values()
            .map(|s| ServiceInfo {
                name: s.name.clone(),
                endpoint: s.endpoint.clone(),
                owned_types: s.owned_types.clone(),
                healthy: s.healthy,
            })
            .collect()
    }

    /// Get statistics about schema stitching.
    pub async fn get_stats(&self) -> StitchingStats {
        let services = self.services.read().await;
        let type_owners = self.type_owners.read().await;
        let stitched = self.stitched_schema.read().await;

        let healthy_services = services.values().filter(|s| s.healthy).count();

        StitchingStats {
            total_services: services.len(),
            healthy_services,
            total_types: type_owners.len(),
            schema_cached: stitched.is_some(),
        }
    }

    /// Execute a health check on all services.
    pub async fn health_check_all(&self) -> HashMap<String, bool> {
        let mut services = self.services.write().await;
        let mut health_map = HashMap::new();

        for (name, service) in services.iter_mut() {
            // In production, make an actual HTTP request to the service
            // For now, just return the current status
            health_map.insert(name.clone(), service.healthy);
        }

        health_map
    }
}

impl Default for SchemaStitcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a registered service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service name
    pub name: String,
    /// Service endpoint
    pub endpoint: String,
    /// Types owned by this service
    pub owned_types: Vec<String>,
    /// Health status
    pub healthy: bool,
}

/// Statistics about schema stitching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StitchingStats {
    /// Total number of registered services
    pub total_services: usize,
    /// Number of healthy services
    pub healthy_services: usize,
    /// Total number of types
    pub total_types: usize,
    /// Whether stitched schema is cached
    pub schema_cached: bool,
}

/// Errors that can occur during schema stitching.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum StitchingError {
    #[error("Too many services registered (limit: {limit})")]
    TooManyServices { limit: usize },

    #[error("Service not found: {name}")]
    ServiceNotFound { name: String },

    #[error("Type not found: {type_name}")]
    TypeNotFound { type_name: String },

    #[error("Invalid schema: {reason}")]
    InvalidSchema { reason: String },

    #[error("No services registered")]
    NoServices,

    #[error("Service is unhealthy: {name}")]
    ServiceUnhealthy { name: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_service_creation() {
        let service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! name: String! }".to_string(),
        );

        assert_eq!(service.name, "users");
        assert_eq!(service.endpoint, "http://localhost:4001/graphql");
        assert!(service.healthy);
    }

    #[test]
    fn test_remote_service_with_owned_types() {
        let service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! }".to_string(),
        )
        .with_owned_types(vec!["User".to_string(), "UserQuery".to_string()]);

        assert_eq!(service.owned_types.len(), 2);
        assert!(service.owned_types.contains(&"User".to_string()));
    }

    #[test]
    fn test_stitching_config_default() {
        let config = StitchingConfig::default();
        assert_eq!(config.max_services, 10);
        assert_eq!(config.request_timeout_ms, 5000);
        assert!(config.enable_type_merging);
        assert!(config.validate_on_register);
    }

    #[tokio::test]
    async fn test_schema_stitcher_creation() {
        let stitcher = SchemaStitcher::new();
        let stats = stitcher.get_stats().await;
        assert_eq!(stats.total_services, 0);
        assert_eq!(stats.total_types, 0);
    }

    #[tokio::test]
    async fn test_register_service() {
        let stitcher = SchemaStitcher::new();

        let service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! }".to_string(),
        )
        .with_owned_types(vec!["User".to_string()]);

        let result = stitcher.register_service(service).await;
        assert!(result.is_ok());

        let stats = stitcher.get_stats().await;
        assert_eq!(stats.total_services, 1);
        assert_eq!(stats.total_types, 1);
    }

    #[tokio::test]
    async fn test_register_multiple_services() {
        let stitcher = SchemaStitcher::new();

        let users_service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! }".to_string(),
        )
        .with_owned_types(vec!["User".to_string()]);

        let posts_service = RemoteService::new(
            "posts".to_string(),
            "http://localhost:4002/graphql".to_string(),
            "type Post { id: ID! }".to_string(),
        )
        .with_owned_types(vec!["Post".to_string()]);

        stitcher.register_service(users_service).await.unwrap();
        stitcher.register_service(posts_service).await.unwrap();

        let stats = stitcher.get_stats().await;
        assert_eq!(stats.total_services, 2);
        assert_eq!(stats.total_types, 2);
    }

    #[tokio::test]
    async fn test_unregister_service() {
        let stitcher = SchemaStitcher::new();

        let service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! }".to_string(),
        )
        .with_owned_types(vec!["User".to_string()]);

        stitcher.register_service(service).await.unwrap();

        let result = stitcher.unregister_service("users").await;
        assert!(result.is_ok());

        let stats = stitcher.get_stats().await;
        assert_eq!(stats.total_services, 0);
        assert_eq!(stats.total_types, 0);
    }

    #[tokio::test]
    async fn test_unregister_nonexistent_service() {
        let stitcher = SchemaStitcher::new();

        let result = stitcher.unregister_service("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_stitched_schema() {
        let stitcher = SchemaStitcher::new();

        let service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! name: String! }".to_string(),
        );

        stitcher.register_service(service).await.unwrap();

        let schema = stitcher.get_stitched_schema().await;
        assert!(schema.is_ok());

        let schema_str = schema.unwrap();
        assert!(schema_str.contains("User"));
        assert!(schema_str.contains("users"));
    }

    #[tokio::test]
    async fn test_get_stitched_schema_no_services() {
        let stitcher = SchemaStitcher::new();

        let result = stitcher.get_stitched_schema().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_route_query() {
        let stitcher = SchemaStitcher::new();

        let service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! }".to_string(),
        )
        .with_owned_types(vec!["User".to_string()]);

        stitcher.register_service(service).await.unwrap();

        let endpoint = stitcher.route_query("User").await;
        assert!(endpoint.is_ok());
        assert_eq!(endpoint.unwrap(), "http://localhost:4001/graphql");
    }

    #[tokio::test]
    async fn test_route_query_type_not_found() {
        let stitcher = SchemaStitcher::new();

        let result = stitcher.route_query("NonexistentType").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_services() {
        let stitcher = SchemaStitcher::new();

        let service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! }".to_string(),
        )
        .with_owned_types(vec!["User".to_string()]);

        stitcher.register_service(service).await.unwrap();

        let services = stitcher.get_services().await;
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].name, "users");
    }

    #[tokio::test]
    async fn test_health_check_all() {
        let stitcher = SchemaStitcher::new();

        let service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! }".to_string(),
        );

        stitcher.register_service(service).await.unwrap();

        let health = stitcher.health_check_all().await;
        assert_eq!(health.len(), 1);
        assert_eq!(health.get("users"), Some(&true));
    }

    #[tokio::test]
    async fn test_too_many_services() {
        let config = StitchingConfig {
            max_services: 2,
            ..Default::default()
        };
        let stitcher = SchemaStitcher::with_config(config);

        // Register 2 services (should work)
        for i in 0..2 {
            let service = RemoteService::new(
                format!("service{}", i),
                format!("http://localhost:400{}/graphql", i),
                format!("type Type{} {{ id: ID! }}", i),
            );
            stitcher.register_service(service).await.unwrap();
        }

        // Try to register 3rd service (should fail)
        let service = RemoteService::new(
            "service3".to_string(),
            "http://localhost:4003/graphql".to_string(),
            "type Type3 { id: ID! }".to_string(),
        );
        let result = stitcher.register_service(service).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_schema_caching() {
        let stitcher = SchemaStitcher::new();

        let service = RemoteService::new(
            "users".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "type User { id: ID! }".to_string(),
        );

        stitcher.register_service(service).await.unwrap();

        // First call generates schema
        let stats_before = stitcher.get_stats().await;
        assert!(!stats_before.schema_cached);

        let _schema = stitcher.get_stitched_schema().await.unwrap();

        // Second call uses cached schema
        let stats_after = stitcher.get_stats().await;
        assert!(stats_after.schema_cached);
    }

    #[tokio::test]
    async fn test_validate_empty_schema() {
        let config = StitchingConfig {
            validate_on_register: true,
            ..Default::default()
        };
        let stitcher = SchemaStitcher::with_config(config);

        let service = RemoteService::new(
            "empty".to_string(),
            "http://localhost:4001/graphql".to_string(),
            "   ".to_string(), // Empty schema
        );

        let result = stitcher.register_service(service).await;
        assert!(result.is_err());
    }
}
