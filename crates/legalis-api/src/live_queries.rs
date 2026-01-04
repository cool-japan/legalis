//! Live queries (Subscriptions 2.0) for GraphQL.
//!
//! This module provides advanced subscription capabilities where queries automatically
//! update when the underlying data changes, without requiring explicit subscriptions.

use async_graphql::{Request, Response, Schema};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;

/// A live query that automatically updates when data changes.
#[derive(Debug, Clone)]
pub struct LiveQuery {
    /// Unique ID for this live query
    pub id: String,
    /// The GraphQL query
    pub query: String,
    /// Optional operation name
    pub operation_name: Option<String>,
    /// Optional variables
    pub variables: Option<serde_json::Value>,
    /// Resources this query depends on
    pub dependencies: HashSet<String>,
}

/// Update notification for a live query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveQueryUpdate {
    /// The query ID
    pub query_id: String,
    /// The updated data
    pub data: Option<serde_json::Value>,
    /// Any errors
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<serde_json::Value>,
    /// Update timestamp
    pub timestamp: String,
    /// Update reason
    pub reason: UpdateReason,
}

/// Reason for a live query update.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateReason {
    /// Initial query execution
    Initial,
    /// Data dependency changed
    DataChanged { resources: Vec<String> },
    /// Periodic refresh
    PeriodicRefresh,
    /// Manual refresh requested
    ManualRefresh,
}

/// Event that triggers live query updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataChangeEvent {
    /// A statute was created
    StatuteCreated { id: String },
    /// A statute was updated
    StatuteUpdated { id: String },
    /// A statute was deleted
    StatuteDeleted { id: String },
    /// Multiple statutes changed
    BulkChange { ids: Vec<String> },
    /// All data invalidated
    FullInvalidation,
}

impl DataChangeEvent {
    /// Get the affected resource IDs.
    pub fn affected_resources(&self) -> Vec<String> {
        match self {
            DataChangeEvent::StatuteCreated { id } => vec![id.clone()],
            DataChangeEvent::StatuteUpdated { id } => vec![id.clone()],
            DataChangeEvent::StatuteDeleted { id } => vec![id.clone()],
            DataChangeEvent::BulkChange { ids } => ids.clone(),
            DataChangeEvent::FullInvalidation => vec!["*".to_string()],
        }
    }
}

/// Configuration for live queries.
#[derive(Debug, Clone)]
pub struct LiveQueryConfig {
    /// Maximum number of concurrent live queries per client
    pub max_queries_per_client: usize,
    /// Automatic refresh interval (None = no automatic refresh)
    pub auto_refresh_interval: Option<std::time::Duration>,
    /// Maximum query execution time
    pub max_execution_time: std::time::Duration,
}

impl Default for LiveQueryConfig {
    fn default() -> Self {
        Self {
            max_queries_per_client: 100,
            auto_refresh_interval: None,
            max_execution_time: std::time::Duration::from_secs(30),
        }
    }
}

/// Manager for live queries.
pub struct LiveQueryManager {
    /// Active live queries indexed by query ID
    queries: Arc<RwLock<HashMap<String, LiveQuery>>>,
    /// Mapping from resource ID to query IDs that depend on it
    resource_dependencies: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    /// Broadcast channel for sending updates
    update_tx: broadcast::Sender<LiveQueryUpdate>,
    /// Broadcast channel for data change events
    event_tx: broadcast::Sender<DataChangeEvent>,
    /// Configuration
    #[allow(dead_code)]
    config: LiveQueryConfig,
}

impl LiveQueryManager {
    /// Create a new live query manager.
    pub fn new() -> Self {
        let (update_tx, _) = broadcast::channel(1000);
        let (event_tx, _) = broadcast::channel(1000);

        Self {
            queries: Arc::new(RwLock::new(HashMap::new())),
            resource_dependencies: Arc::new(RwLock::new(HashMap::new())),
            update_tx,
            event_tx,
            config: LiveQueryConfig::default(),
        }
    }

    /// Create a new live query manager with custom configuration.
    pub fn with_config(config: LiveQueryConfig) -> Self {
        let (update_tx, _) = broadcast::channel(1000);
        let (event_tx, _) = broadcast::channel(1000);

        Self {
            queries: Arc::new(RwLock::new(HashMap::new())),
            resource_dependencies: Arc::new(RwLock::new(HashMap::new())),
            update_tx,
            event_tx,
            config,
        }
    }

    /// Register a new live query.
    pub async fn register_query(
        &self,
        query: String,
        operation_name: Option<String>,
        variables: Option<serde_json::Value>,
    ) -> Result<String, String> {
        let query_id = Uuid::new_v4().to_string();

        // Extract dependencies from query (simplified - in production use AST analysis)
        let dependencies = self.extract_dependencies(&query);

        let live_query = LiveQuery {
            id: query_id.clone(),
            query,
            operation_name,
            variables,
            dependencies: dependencies.clone(),
        };

        // Store query
        {
            let mut queries = self.queries.write().await;
            queries.insert(query_id.clone(), live_query);
        }

        // Update resource dependencies
        {
            let mut resource_deps = self.resource_dependencies.write().await;
            for dep in dependencies {
                resource_deps
                    .entry(dep)
                    .or_insert_with(HashSet::new)
                    .insert(query_id.clone());
            }
        }

        Ok(query_id)
    }

    /// Unregister a live query.
    pub async fn unregister_query(&self, query_id: &str) -> Result<(), String> {
        // Get the query to find its dependencies
        let query = {
            let mut queries = self.queries.write().await;
            queries.remove(query_id)
        };

        if let Some(query) = query {
            // Remove from resource dependencies
            let mut resource_deps = self.resource_dependencies.write().await;
            for dep in &query.dependencies {
                if let Some(dep_set) = resource_deps.get_mut(dep) {
                    dep_set.remove(query_id);
                    if dep_set.is_empty() {
                        resource_deps.remove(dep);
                    }
                }
            }
            Ok(())
        } else {
            Err(format!("Query {} not found", query_id))
        }
    }

    /// Execute a live query and get initial results.
    pub async fn execute_query<Q, M, S>(
        &self,
        schema: &Schema<Q, M, S>,
        query_id: &str,
    ) -> Result<LiveQueryUpdate, String>
    where
        Q: async_graphql::ObjectType + 'static,
        M: async_graphql::ObjectType + 'static,
        S: async_graphql::SubscriptionType + 'static,
    {
        let query = {
            let queries = self.queries.read().await;
            queries
                .get(query_id)
                .cloned()
                .ok_or_else(|| format!("Query {} not found", query_id))?
        };

        let response = self.execute_graphql_query(schema, &query).await;

        let update = LiveQueryUpdate {
            query_id: query_id.to_string(),
            data: response.data.into_json().ok(),
            errors: response
                .errors
                .into_iter()
                .map(|e| {
                    serde_json::json!({
                        "message": e.message,
                        "locations": e.locations,
                        "path": e.path,
                    })
                })
                .collect(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            reason: UpdateReason::Initial,
        };

        Ok(update)
    }

    /// Notify about a data change event.
    pub async fn notify_change(&self, event: DataChangeEvent) {
        let affected = event.affected_resources();

        // Find all queries that depend on affected resources
        let affected_queries = {
            let resource_deps = self.resource_dependencies.read().await;
            let mut query_ids = HashSet::new();

            for resource in &affected {
                if resource == "*" {
                    // Full invalidation - all queries affected
                    let queries = self.queries.read().await;
                    query_ids.extend(queries.keys().cloned());
                    break;
                } else if let Some(deps) = resource_deps.get(resource) {
                    query_ids.extend(deps.iter().cloned());
                }
            }

            query_ids
        };

        // Broadcast the event
        let _ = self.event_tx.send(event);

        // Note: Actual re-execution happens in the background task or when client polls
        tracing::debug!(
            "Data change affects {} live queries: {:?}",
            affected_queries.len(),
            affected_queries
        );
    }

    /// Subscribe to live query updates.
    pub fn subscribe_updates(&self) -> broadcast::Receiver<LiveQueryUpdate> {
        self.update_tx.subscribe()
    }

    /// Subscribe to data change events.
    pub fn subscribe_events(&self) -> broadcast::Receiver<DataChangeEvent> {
        self.event_tx.subscribe()
    }

    /// Broadcast an update to all subscribers.
    pub fn broadcast_update(&self, update: LiveQueryUpdate) {
        let _ = self.update_tx.send(update);
    }

    /// Get all active query IDs.
    pub async fn get_active_queries(&self) -> Vec<String> {
        let queries = self.queries.read().await;
        queries.keys().cloned().collect()
    }

    /// Get statistics about live queries.
    pub async fn get_stats(&self) -> LiveQueryStats {
        let queries = self.queries.read().await;
        let resource_deps = self.resource_dependencies.read().await;

        LiveQueryStats {
            total_queries: queries.len(),
            total_resources: resource_deps.len(),
            avg_dependencies_per_query: if queries.is_empty() {
                0.0
            } else {
                queries
                    .values()
                    .map(|q| q.dependencies.len())
                    .sum::<usize>() as f64
                    / queries.len() as f64
            },
        }
    }

    /// Execute a GraphQL query.
    async fn execute_graphql_query<Q, M, S>(
        &self,
        schema: &Schema<Q, M, S>,
        query: &LiveQuery,
    ) -> Response
    where
        Q: async_graphql::ObjectType + 'static,
        M: async_graphql::ObjectType + 'static,
        S: async_graphql::SubscriptionType + 'static,
    {
        let mut request = Request::new(&query.query);

        if let Some(ref operation_name) = query.operation_name {
            request = request.operation_name(operation_name);
        }

        if let Some(ref variables) = query.variables {
            if let Ok(vars) = serde_json::from_value(variables.clone()) {
                request = request.variables(vars);
            }
        }

        schema.execute(request).await
    }

    /// Extract dependencies from a GraphQL query (simplified).
    fn extract_dependencies(&self, query: &str) -> HashSet<String> {
        let mut dependencies = HashSet::new();

        // Simple heuristic: extract field names that might be resource types
        if query.contains("statute") {
            dependencies.insert("statutes".to_string());
        }
        if query.contains("verification") {
            dependencies.insert("verifications".to_string());
        }
        if query.contains("simulation") {
            dependencies.insert("simulations".to_string());
        }

        // If no specific dependencies found, depend on everything
        if dependencies.is_empty() {
            dependencies.insert("*".to_string());
        }

        dependencies
    }
}

impl Default for LiveQueryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about live queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveQueryStats {
    /// Total number of active live queries
    pub total_queries: usize,
    /// Total number of tracked resources
    pub total_resources: usize,
    /// Average dependencies per query
    pub avg_dependencies_per_query: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_query_config_default() {
        let config = LiveQueryConfig::default();
        assert_eq!(config.max_queries_per_client, 100);
        assert!(config.auto_refresh_interval.is_none());
    }

    #[tokio::test]
    async fn test_live_query_manager_creation() {
        let manager = LiveQueryManager::new();
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_queries, 0);
        assert_eq!(stats.total_resources, 0);
    }

    #[tokio::test]
    async fn test_register_query() {
        let manager = LiveQueryManager::new();

        let query_id = manager
            .register_query("{ statutes { id title } }".to_string(), None, None)
            .await
            .unwrap();

        assert!(!query_id.is_empty());

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_queries, 1);
        assert!(stats.total_resources > 0);
    }

    #[tokio::test]
    async fn test_unregister_query() {
        let manager = LiveQueryManager::new();

        let query_id = manager
            .register_query("{ statutes { id title } }".to_string(), None, None)
            .await
            .unwrap();

        let result = manager.unregister_query(&query_id).await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_queries, 0);
    }

    #[tokio::test]
    async fn test_unregister_nonexistent_query() {
        let manager = LiveQueryManager::new();

        let result = manager.unregister_query("nonexistent-id").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_data_change_event_affected_resources() {
        let event = DataChangeEvent::StatuteCreated {
            id: "statute-1".to_string(),
        };
        let affected = event.affected_resources();
        assert_eq!(affected.len(), 1);
        assert_eq!(affected[0], "statute-1");

        let bulk_event = DataChangeEvent::BulkChange {
            ids: vec!["s1".to_string(), "s2".to_string(), "s3".to_string()],
        };
        let bulk_affected = bulk_event.affected_resources();
        assert_eq!(bulk_affected.len(), 3);

        let full_event = DataChangeEvent::FullInvalidation;
        let full_affected = full_event.affected_resources();
        assert_eq!(full_affected.len(), 1);
        assert_eq!(full_affected[0], "*");
    }

    #[tokio::test]
    async fn test_notify_change() {
        let manager = LiveQueryManager::new();

        // Register a query
        let _query_id = manager
            .register_query("{ statutes { id title } }".to_string(), None, None)
            .await
            .unwrap();

        // Subscribe to events
        let mut event_rx = manager.subscribe_events();

        // Notify a change
        manager
            .notify_change(DataChangeEvent::StatuteCreated {
                id: "statute-1".to_string(),
            })
            .await;

        // Should receive the event
        let event =
            tokio::time::timeout(std::time::Duration::from_millis(100), event_rx.recv()).await;
        assert!(event.is_ok());
    }

    #[tokio::test]
    async fn test_get_active_queries() {
        let manager = LiveQueryManager::new();

        let query_id_1 = manager
            .register_query("{ statutes { id } }".to_string(), None, None)
            .await
            .unwrap();

        let query_id_2 = manager
            .register_query("{ statuteCount }".to_string(), None, None)
            .await
            .unwrap();

        let active = manager.get_active_queries().await;
        assert_eq!(active.len(), 2);
        assert!(active.contains(&query_id_1));
        assert!(active.contains(&query_id_2));
    }

    #[tokio::test]
    async fn test_execute_query() {
        use crate::graphql::GraphQLState;
        use crate::graphql::create_schema;

        let state = GraphQLState::new();
        let schema = create_schema(state);
        let manager = LiveQueryManager::new();

        let query_id = manager
            .register_query("{ statuteCount }".to_string(), None, None)
            .await
            .unwrap();

        let result = manager.execute_query(&schema, &query_id).await;
        assert!(result.is_ok());

        let update = result.unwrap();
        assert_eq!(update.query_id, query_id);
        assert!(update.data.is_some());
        assert!(update.errors.is_empty());
    }

    #[tokio::test]
    async fn test_extract_dependencies() {
        let manager = LiveQueryManager::new();

        let deps = manager.extract_dependencies("{ statutes { id } }");
        assert!(deps.contains("statutes"));

        let deps = manager.extract_dependencies("{ verification { passed } }");
        assert!(deps.contains("verifications"));

        let deps = manager.extract_dependencies("{ simulation { id } }");
        assert!(deps.contains("simulations"));

        let deps = manager.extract_dependencies("{ unknown }");
        assert!(deps.contains("*"));
    }

    #[tokio::test]
    async fn test_subscribe_updates() {
        let manager = LiveQueryManager::new();
        let mut rx = manager.subscribe_updates();

        let update = LiveQueryUpdate {
            query_id: "test-id".to_string(),
            data: Some(serde_json::json!({"test": "data"})),
            errors: vec![],
            timestamp: chrono::Utc::now().to_rfc3339(),
            reason: UpdateReason::Initial,
        };

        manager.broadcast_update(update.clone());

        let received = tokio::time::timeout(std::time::Duration::from_millis(100), rx.recv()).await;
        assert!(received.is_ok());
    }
}
