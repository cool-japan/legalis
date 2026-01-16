//! CQRS (Command Query Responsibility Segregation) implementation
//!
//! This module provides CQRS patterns including:
//! - Command handlers for write operations
//! - Query handlers for read operations
//! - Read model projections
//! - Eventual consistency support

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::event_sourcing::{DomainEvent, EventSourcingError, EventStore};

/// Error types for CQRS operations
#[derive(Debug, Error)]
pub enum CqrsError {
    #[error("Command validation error: {0}")]
    ValidationError(String),

    #[error("Command handler error: {0}")]
    HandlerError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Projection error: {0}")]
    ProjectionError(String),

    #[error("Event sourcing error: {0}")]
    EventSourcingError(#[from] EventSourcingError),
}

/// Result type for CQRS operations
pub type CqrsResult<T> = Result<T, CqrsError>;

/// Base trait for commands
pub trait Command: Send + Sync {
    /// Get command ID
    fn command_id(&self) -> Uuid;

    /// Get aggregate ID that this command targets
    fn aggregate_id(&self) -> &str;

    /// Validate the command
    fn validate(&self) -> CqrsResult<()>;
}

/// Base trait for queries
pub trait Query: Send + Sync {
    /// Query result type
    type Result: Send;

    /// Validate the query
    fn validate(&self) -> CqrsResult<()>;
}

/// Command handler trait
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    /// Handle a command and return generated events
    async fn handle(&self, command: &C) -> CqrsResult<Vec<DomainEvent>>;
}

/// Query handler trait
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    /// Handle a query and return results
    async fn handle(&self, query: &Q) -> CqrsResult<Q::Result>;
}

/// Command bus for dispatching commands
pub struct CommandBus {
    event_store: Arc<dyn EventStore>,
}

impl CommandBus {
    /// Create a new command bus
    pub fn new(event_store: Arc<dyn EventStore>) -> Self {
        Self { event_store }
    }

    /// Dispatch a command
    pub async fn dispatch<C: Command, H: CommandHandler<C>>(
        &self,
        command: &C,
        handler: &H,
    ) -> CqrsResult<Vec<DomainEvent>> {
        // Validate command
        command.validate()?;

        // Handle command
        let events = handler.handle(command).await?;

        // Append events to event store
        if !events.is_empty() {
            let aggregate_id = events[0].metadata.aggregate_id.clone();
            let expected_version = events[0].metadata.version - 1;

            self.event_store
                .append_events(&aggregate_id, expected_version, events.clone())
                .await?;
        }

        Ok(events)
    }
}

/// Query bus for dispatching queries
pub struct QueryBus;

impl QueryBus {
    /// Create a new query bus
    pub fn new() -> Self {
        Self
    }

    /// Dispatch a query
    pub async fn dispatch<Q: Query, H: QueryHandler<Q>>(
        &self,
        query: &Q,
        handler: &H,
    ) -> CqrsResult<Q::Result> {
        // Validate query
        query.validate()?;

        // Handle query
        handler.handle(query).await
    }
}

impl Default for QueryBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Read model trait for projections
#[async_trait]
pub trait ReadModel: Send + Sync {
    /// Get the read model name
    fn name(&self) -> &str;

    /// Apply an event to update the read model
    async fn apply(&mut self, event: &DomainEvent) -> CqrsResult<()>;

    /// Rebuild the read model from events
    async fn rebuild(&mut self, events: Vec<DomainEvent>) -> CqrsResult<()>;
}

/// Projection manager for managing read model projections
pub struct ProjectionManager {
    #[allow(dead_code)]
    event_store: Arc<dyn EventStore>,
    read_models: Arc<RwLock<HashMap<String, Box<dyn ReadModel>>>>,
}

impl ProjectionManager {
    /// Create a new projection manager
    pub fn new(event_store: Arc<dyn EventStore>) -> Self {
        Self {
            event_store,
            read_models: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a read model
    pub async fn register_read_model(&self, read_model: Box<dyn ReadModel>) -> CqrsResult<()> {
        let name = read_model.name().to_string();
        let mut models = self.read_models.write().await;

        models.insert(name, read_model);

        Ok(())
    }

    /// Project events to all registered read models
    pub async fn project_events(&self, events: Vec<DomainEvent>) -> CqrsResult<()> {
        let mut models = self.read_models.write().await;

        for event in &events {
            for read_model in models.values_mut() {
                read_model.apply(event).await?;
            }
        }

        Ok(())
    }

    /// Rebuild all read models from the event store
    pub async fn rebuild_all(&self) -> CqrsResult<()> {
        // Load all events from the event store
        // For simplicity, we'll implement this when we have a way to get all events
        Ok(())
    }
}

/// Example: Statute read model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteReadModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Example: Statute list read model (for queries)
pub struct StatuteListReadModel {
    statutes: Arc<RwLock<HashMap<String, StatuteReadModel>>>,
}

impl StatuteListReadModel {
    /// Create a new statute list read model
    pub fn new() -> Self {
        Self {
            statutes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get all statutes
    pub async fn get_all(&self) -> Vec<StatuteReadModel> {
        self.statutes.read().await.values().cloned().collect()
    }

    /// Get statute by ID
    pub async fn get_by_id(&self, id: &str) -> Option<StatuteReadModel> {
        self.statutes.read().await.get(id).cloned()
    }

    /// Search statutes by title
    pub async fn search_by_title(&self, query: &str) -> Vec<StatuteReadModel> {
        self.statutes
            .read()
            .await
            .values()
            .filter(|s| s.title.contains(query))
            .cloned()
            .collect()
    }
}

impl Default for StatuteListReadModel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReadModel for StatuteListReadModel {
    fn name(&self) -> &str {
        "StatuteListReadModel"
    }

    async fn apply(&mut self, event: &DomainEvent) -> CqrsResult<()> {
        let mut statutes = self.statutes.write().await;

        match event.metadata.event_type.as_str() {
            "StatuteCreated" => {
                let id = event.metadata.aggregate_id.clone();
                let title = event.payload["title"]
                    .as_str()
                    .unwrap_or("Untitled")
                    .to_string();
                let content = event.payload["content"].as_str().unwrap_or("").to_string();

                statutes.insert(
                    id.clone(),
                    StatuteReadModel {
                        id: id.clone(),
                        title,
                        content,
                        version: event.metadata.version,
                        created_at: event.metadata.timestamp,
                        updated_at: event.metadata.timestamp,
                    },
                );
            }
            "StatuteUpdated" => {
                let id = event.metadata.aggregate_id.clone();
                if let Some(statute) = statutes.get_mut(&id) {
                    if let Some(title) = event.payload["title"].as_str() {
                        statute.title = title.to_string();
                    }
                    if let Some(content) = event.payload["content"].as_str() {
                        statute.content = content.to_string();
                    }
                    statute.version = event.metadata.version;
                    statute.updated_at = event.metadata.timestamp;
                }
            }
            "StatuteDeleted" => {
                let id = event.metadata.aggregate_id.clone();
                statutes.remove(&id);
            }
            _ => {
                // Ignore unknown events
            }
        }

        Ok(())
    }

    async fn rebuild(&mut self, events: Vec<DomainEvent>) -> CqrsResult<()> {
        // Clear existing data
        {
            let mut statutes = self.statutes.write().await;
            statutes.clear();
        }

        // Apply all events
        for event in events {
            self.apply(&event).await?;
        }

        Ok(())
    }
}

/// Example command: Create statute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStatuteCommand {
    pub command_id: Uuid,
    pub statute_id: String,
    pub title: String,
    pub content: String,
}

impl Command for CreateStatuteCommand {
    fn command_id(&self) -> Uuid {
        self.command_id
    }

    fn aggregate_id(&self) -> &str {
        &self.statute_id
    }

    fn validate(&self) -> CqrsResult<()> {
        if self.title.is_empty() {
            return Err(CqrsError::ValidationError(
                "Title cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// Example query: Get all statutes
#[derive(Debug, Clone)]
pub struct GetAllStatutesQuery;

impl Query for GetAllStatutesQuery {
    type Result = Vec<StatuteReadModel>;

    fn validate(&self) -> CqrsResult<()> {
        Ok(())
    }
}

/// Example query: Search statutes
#[derive(Debug, Clone)]
pub struct SearchStatutesQuery {
    pub query: String,
}

impl Query for SearchStatutesQuery {
    type Result = Vec<StatuteReadModel>;

    fn validate(&self) -> CqrsResult<()> {
        if self.query.is_empty() {
            return Err(CqrsError::ValidationError(
                "Query cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_sourcing::{DomainEvent, InMemoryEventStore};

    #[tokio::test]
    async fn test_command_validation() {
        let command = CreateStatuteCommand {
            command_id: Uuid::new_v4(),
            statute_id: "statute-1".to_string(),
            title: "".to_string(),
            content: "Test content".to_string(),
        };

        let result = command.validate();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_read_model_apply_event() {
        let mut read_model = StatuteListReadModel::new();

        let event = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({
                "title": "Test Statute",
                "content": "Test content"
            }),
        );

        read_model.apply(&event).await.unwrap();

        let statutes = read_model.get_all().await;
        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].title, "Test Statute");
    }

    #[tokio::test]
    async fn test_query_bus() {
        let query_bus = QueryBus::new();

        let query = GetAllStatutesQuery;

        // Create a simple handler
        struct TestHandler {
            read_model: Arc<StatuteListReadModel>,
        }

        #[async_trait]
        impl QueryHandler<GetAllStatutesQuery> for TestHandler {
            async fn handle(
                &self,
                _query: &GetAllStatutesQuery,
            ) -> CqrsResult<Vec<StatuteReadModel>> {
                Ok(self.read_model.get_all().await)
            }
        }

        let read_model = Arc::new(StatuteListReadModel::new());
        let handler = TestHandler {
            read_model: read_model.clone(),
        };

        let result = query_bus.dispatch(&query, &handler).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_projection_manager() {
        let event_store = Arc::new(InMemoryEventStore::new());
        let projection_manager = ProjectionManager::new(event_store);

        let read_model = Box::new(StatuteListReadModel::new());
        projection_manager
            .register_read_model(read_model)
            .await
            .unwrap();

        let event = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({
                "title": "Test Statute",
                "content": "Test content"
            }),
        );

        projection_manager
            .project_events(vec![event])
            .await
            .unwrap();
    }
}
