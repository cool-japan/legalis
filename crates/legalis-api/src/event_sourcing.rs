//! Event Sourcing implementation for Legalis API
//!
//! This module provides event sourcing capabilities including:
//! - Event store for persisting domain events
//! - Event stream for reading events
//! - Snapshot support for performance
//! - Event versioning and migration

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

/// Error types for event sourcing
#[derive(Debug, Error)]
pub enum EventSourcingError {
    #[error("Event store error: {0}")]
    StoreError(String),

    #[error("Aggregate not found: {0}")]
    AggregateNotFound(String),

    #[error("Concurrency conflict: expected version {expected}, got {actual}")]
    ConcurrencyConflict { expected: i64, actual: i64 },

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Event replay error: {0}")]
    ReplayError(String),
}

/// Result type for event sourcing operations
pub type EventResult<T> = Result<T, EventSourcingError>;

/// Event metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// Event ID
    pub event_id: Uuid,

    /// Aggregate ID that this event belongs to
    pub aggregate_id: String,

    /// Aggregate type
    pub aggregate_type: String,

    /// Event type/name
    pub event_type: String,

    /// Event version in the aggregate stream
    pub version: i64,

    /// Timestamp when the event occurred
    pub timestamp: DateTime<Utc>,

    /// User/actor who caused this event
    pub caused_by: Option<String>,

    /// Correlation ID for tracking related events
    pub correlation_id: Option<Uuid>,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Domain event with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    /// Event metadata
    pub metadata: EventMetadata,

    /// Event payload (JSON-serialized)
    pub payload: serde_json::Value,
}

impl DomainEvent {
    /// Create a new domain event
    pub fn new(
        aggregate_id: String,
        aggregate_type: String,
        event_type: String,
        version: i64,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            metadata: EventMetadata {
                event_id: Uuid::new_v4(),
                aggregate_id,
                aggregate_type,
                event_type,
                version,
                timestamp: Utc::now(),
                caused_by: None,
                correlation_id: None,
                metadata: HashMap::new(),
            },
            payload,
        }
    }

    /// Set the actor who caused this event
    pub fn with_caused_by(mut self, caused_by: String) -> Self {
        self.metadata.caused_by = Some(caused_by);
        self
    }

    /// Set correlation ID
    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.metadata.correlation_id = Some(correlation_id);
        self
    }

    /// Add custom metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.metadata.insert(key, value);
        self
    }
}

/// Snapshot of aggregate state at a specific version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Aggregate ID
    pub aggregate_id: String,

    /// Aggregate type
    pub aggregate_type: String,

    /// Version at which this snapshot was taken
    pub version: i64,

    /// Timestamp when snapshot was created
    pub timestamp: DateTime<Utc>,

    /// Serialized aggregate state
    pub state: serde_json::Value,
}

/// Event stream for reading events
#[derive(Debug, Clone)]
pub struct EventStream {
    /// Events in the stream
    pub events: Vec<DomainEvent>,

    /// Version of the last event in the stream
    pub version: i64,
}

/// Trait for event store implementations
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Append events to an aggregate stream
    /// Returns the new version of the aggregate
    async fn append_events(
        &self,
        aggregate_id: &str,
        expected_version: i64,
        events: Vec<DomainEvent>,
    ) -> EventResult<i64>;

    /// Load all events for an aggregate
    async fn load_events(&self, aggregate_id: &str) -> EventResult<EventStream>;

    /// Load events from a specific version
    async fn load_events_from(
        &self,
        aggregate_id: &str,
        from_version: i64,
    ) -> EventResult<EventStream>;

    /// Save a snapshot
    async fn save_snapshot(&self, snapshot: Snapshot) -> EventResult<()>;

    /// Load the latest snapshot for an aggregate
    async fn load_snapshot(&self, aggregate_id: &str) -> EventResult<Option<Snapshot>>;

    /// Load all events of a specific type
    async fn load_events_by_type(&self, event_type: &str) -> EventResult<Vec<DomainEvent>>;

    /// Load all events in a time range
    async fn load_events_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> EventResult<Vec<DomainEvent>>;
}

/// In-memory event store implementation (for testing/demo)
#[derive(Debug, Clone)]
pub struct InMemoryEventStore {
    events: Arc<RwLock<HashMap<String, Vec<DomainEvent>>>>,
    snapshots: Arc<RwLock<HashMap<String, Snapshot>>>,
}

impl InMemoryEventStore {
    /// Create a new in-memory event store
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
            snapshots: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn append_events(
        &self,
        aggregate_id: &str,
        expected_version: i64,
        events: Vec<DomainEvent>,
    ) -> EventResult<i64> {
        let mut store = self.events.write().map_err(|e| {
            EventSourcingError::StoreError(format!("Failed to acquire write lock: {}", e))
        })?;

        let aggregate_events = store
            .entry(aggregate_id.to_string())
            .or_insert_with(Vec::new);

        let current_version = aggregate_events.len() as i64;

        // Check for concurrency conflicts
        if expected_version != current_version {
            return Err(EventSourcingError::ConcurrencyConflict {
                expected: expected_version,
                actual: current_version,
            });
        }

        // Append events
        aggregate_events.extend(events);

        Ok(aggregate_events.len() as i64)
    }

    async fn load_events(&self, aggregate_id: &str) -> EventResult<EventStream> {
        let store = self.events.read().map_err(|e| {
            EventSourcingError::StoreError(format!("Failed to acquire read lock: {}", e))
        })?;

        let events = store.get(aggregate_id).cloned().unwrap_or_default();

        let version = events.len() as i64;

        Ok(EventStream { events, version })
    }

    async fn load_events_from(
        &self,
        aggregate_id: &str,
        from_version: i64,
    ) -> EventResult<EventStream> {
        let store = self.events.read().map_err(|e| {
            EventSourcingError::StoreError(format!("Failed to acquire read lock: {}", e))
        })?;

        let all_events = store.get(aggregate_id).cloned().unwrap_or_default();

        let events: Vec<DomainEvent> = all_events.into_iter().skip(from_version as usize).collect();

        let version = (from_version as usize + events.len()) as i64;

        Ok(EventStream { events, version })
    }

    async fn save_snapshot(&self, snapshot: Snapshot) -> EventResult<()> {
        let mut snapshots = self.snapshots.write().map_err(|e| {
            EventSourcingError::StoreError(format!("Failed to acquire write lock: {}", e))
        })?;

        snapshots.insert(snapshot.aggregate_id.clone(), snapshot);

        Ok(())
    }

    async fn load_snapshot(&self, aggregate_id: &str) -> EventResult<Option<Snapshot>> {
        let snapshots = self.snapshots.read().map_err(|e| {
            EventSourcingError::StoreError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(snapshots.get(aggregate_id).cloned())
    }

    async fn load_events_by_type(&self, event_type: &str) -> EventResult<Vec<DomainEvent>> {
        let store = self.events.read().map_err(|e| {
            EventSourcingError::StoreError(format!("Failed to acquire read lock: {}", e))
        })?;

        let events: Vec<DomainEvent> = store
            .values()
            .flat_map(|events| events.iter())
            .filter(|event| event.metadata.event_type == event_type)
            .cloned()
            .collect();

        Ok(events)
    }

    async fn load_events_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> EventResult<Vec<DomainEvent>> {
        let store = self.events.read().map_err(|e| {
            EventSourcingError::StoreError(format!("Failed to acquire read lock: {}", e))
        })?;

        let events: Vec<DomainEvent> = store
            .values()
            .flat_map(|events| events.iter())
            .filter(|event| event.metadata.timestamp >= start && event.metadata.timestamp <= end)
            .cloned()
            .collect();

        Ok(events)
    }
}

/// Aggregate root trait for event-sourced aggregates
pub trait AggregateRoot: Sized {
    /// Get the aggregate ID
    fn aggregate_id(&self) -> &str;

    /// Get current version
    fn version(&self) -> i64;

    /// Apply an event to update the aggregate state
    fn apply_event(&mut self, event: &DomainEvent) -> EventResult<()>;

    /// Load aggregate from event stream
    fn load_from_events(events: Vec<DomainEvent>) -> EventResult<Self>;
}

/// Event publisher for publishing events to external systems
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish an event
    async fn publish(&self, event: &DomainEvent) -> EventResult<()>;

    /// Publish multiple events
    async fn publish_batch(&self, events: &[DomainEvent]) -> EventResult<()>;
}

/// In-memory event publisher (for testing)
#[derive(Debug, Clone)]
pub struct InMemoryEventPublisher {
    published_events: Arc<RwLock<Vec<DomainEvent>>>,
}

impl InMemoryEventPublisher {
    /// Create a new in-memory event publisher
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get all published events (for testing)
    pub fn get_published_events(&self) -> Vec<DomainEvent> {
        self.published_events.read().unwrap().clone()
    }
}

impl Default for InMemoryEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&self, event: &DomainEvent) -> EventResult<()> {
        let mut events = self.published_events.write().map_err(|e| {
            EventSourcingError::StoreError(format!("Failed to acquire write lock: {}", e))
        })?;

        events.push(event.clone());

        Ok(())
    }

    async fn publish_batch(&self, events: &[DomainEvent]) -> EventResult<()> {
        let mut published = self.published_events.write().map_err(|e| {
            EventSourcingError::StoreError(format!("Failed to acquire write lock: {}", e))
        })?;

        published.extend_from_slice(events);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_append_and_load_events() {
        let store = InMemoryEventStore::new();

        let event1 = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"title": "Test Statute"}),
        );

        let event2 = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteUpdated".to_string(),
            2,
            serde_json::json!({"title": "Updated Statute"}),
        );

        // Append events
        let version = store
            .append_events("statute-1", 0, vec![event1.clone(), event2.clone()])
            .await
            .unwrap();

        assert_eq!(version, 2);

        // Load events
        let stream = store.load_events("statute-1").await.unwrap();
        assert_eq!(stream.events.len(), 2);
        assert_eq!(stream.version, 2);
    }

    #[tokio::test]
    async fn test_concurrency_conflict() {
        let store = InMemoryEventStore::new();

        let event1 = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"title": "Test"}),
        );

        // First append succeeds
        store
            .append_events("statute-1", 0, vec![event1.clone()])
            .await
            .unwrap();

        let event2 = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteUpdated".to_string(),
            2,
            serde_json::json!({"title": "Updated"}),
        );

        // Second append with wrong expected version fails
        let result = store.append_events("statute-1", 0, vec![event2]).await;

        assert!(matches!(
            result,
            Err(EventSourcingError::ConcurrencyConflict { .. })
        ));
    }

    #[tokio::test]
    async fn test_snapshot() {
        let store = InMemoryEventStore::new();

        let snapshot = Snapshot {
            aggregate_id: "statute-1".to_string(),
            aggregate_type: "Statute".to_string(),
            version: 10,
            timestamp: Utc::now(),
            state: serde_json::json!({"title": "Snapshot State"}),
        };

        // Save snapshot
        store.save_snapshot(snapshot.clone()).await.unwrap();

        // Load snapshot
        let loaded = store.load_snapshot("statute-1").await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().version, 10);
    }

    #[tokio::test]
    async fn test_event_publisher() {
        let publisher = InMemoryEventPublisher::new();

        let event = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"title": "Test"}),
        );

        publisher.publish(&event).await.unwrap();

        let published = publisher.get_published_events();
        assert_eq!(published.len(), 1);
    }
}
