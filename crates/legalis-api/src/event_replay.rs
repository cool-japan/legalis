//! Event Replay implementation
//!
//! This module provides event replay capabilities including:
//! - Replay events from a specific point in time
//! - Replay events for specific aggregates
//! - Replay events by type
//! - Replay monitoring and control

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

use crate::event_sourcing::{DomainEvent, EventResult, EventSourcingError, EventStore};

/// Error types for event replay
#[derive(Debug, Error)]
pub enum ReplayError {
    #[error("Replay error: {0}")]
    ReplayError(String),

    #[error("Event sourcing error: {0}")]
    EventSourcingError(#[from] EventSourcingError),

    #[error("Invalid replay state: {0}")]
    InvalidState(String),
}

/// Result type for replay operations
pub type ReplayResult<T> = Result<T, ReplayError>;

/// Replay state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayState {
    /// Replay is not started
    NotStarted,

    /// Replay is in progress
    InProgress,

    /// Replay is paused
    Paused,

    /// Replay is completed
    Completed,

    /// Replay failed
    Failed,
}

/// Replay statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayStats {
    /// Total number of events to replay
    pub total_events: usize,

    /// Number of events replayed so far
    pub replayed_events: usize,

    /// Number of events that failed to replay
    pub failed_events: usize,

    /// Start time of the replay
    pub start_time: Option<DateTime<Utc>>,

    /// End time of the replay
    pub end_time: Option<DateTime<Utc>>,

    /// Current replay state
    pub state: ReplayState,
}

impl ReplayStats {
    /// Create new replay stats
    pub fn new(total_events: usize) -> Self {
        Self {
            total_events,
            replayed_events: 0,
            failed_events: 0,
            start_time: None,
            end_time: None,
            state: ReplayState::NotStarted,
        }
    }

    /// Calculate progress percentage
    pub fn progress_percentage(&self) -> f64 {
        if self.total_events == 0 {
            return 100.0;
        }
        (self.replayed_events as f64 / self.total_events as f64) * 100.0
    }

    /// Check if replay is completed
    pub fn is_completed(&self) -> bool {
        self.state == ReplayState::Completed
    }

    /// Check if replay is in progress
    pub fn is_in_progress(&self) -> bool {
        self.state == ReplayState::InProgress
    }
}

/// Replay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayConfig {
    /// Replay ID
    pub replay_id: Uuid,

    /// Start from this timestamp (inclusive)
    pub from_time: Option<DateTime<Utc>>,

    /// Replay until this timestamp (inclusive)
    pub to_time: Option<DateTime<Utc>>,

    /// Filter by aggregate IDs
    pub aggregate_ids: Option<Vec<String>>,

    /// Filter by event types
    pub event_types: Option<Vec<String>>,

    /// Batch size for processing events
    pub batch_size: usize,

    /// Delay between batches (milliseconds)
    pub batch_delay_ms: u64,

    /// Whether to continue on error
    pub continue_on_error: bool,
}

impl Default for ReplayConfig {
    fn default() -> Self {
        Self {
            replay_id: Uuid::new_v4(),
            from_time: None,
            to_time: None,
            aggregate_ids: None,
            event_types: None,
            batch_size: 100,
            batch_delay_ms: 0,
            continue_on_error: false,
        }
    }
}

/// Event handler for replay
#[async_trait]
pub trait ReplayEventHandler: Send + Sync {
    /// Handle a replayed event
    async fn handle(&self, event: &DomainEvent) -> EventResult<()>;
}

/// Replay manager for managing event replays
pub struct ReplayManager {
    event_store: Arc<dyn EventStore>,
    stats: Arc<RwLock<ReplayStats>>,
}

impl ReplayManager {
    /// Create a new replay manager
    pub fn new(event_store: Arc<dyn EventStore>) -> Self {
        Self {
            event_store,
            stats: Arc::new(RwLock::new(ReplayStats::new(0))),
        }
    }

    /// Start a replay
    pub async fn replay(
        &self,
        config: ReplayConfig,
        handler: Arc<dyn ReplayEventHandler>,
    ) -> ReplayResult<()> {
        // Load events based on config
        let events = self.load_events(&config).await?;

        // Initialize stats
        {
            let mut stats = self.stats.write().map_err(|e| {
                ReplayError::ReplayError(format!("Failed to acquire write lock: {}", e))
            })?;

            *stats = ReplayStats::new(events.len());
            stats.state = ReplayState::InProgress;
            stats.start_time = Some(Utc::now());
        }

        // Replay events in batches
        for chunk in events.chunks(config.batch_size) {
            // Check if replay is paused
            {
                let stats = self.stats.read().map_err(|e| {
                    ReplayError::ReplayError(format!("Failed to acquire read lock: {}", e))
                })?;

                if stats.state == ReplayState::Paused {
                    return Ok(());
                }
            }

            // Process batch
            for event in chunk {
                match handler.handle(event).await {
                    Ok(_) => {
                        let mut stats = self.stats.write().map_err(|e| {
                            ReplayError::ReplayError(format!("Failed to acquire write lock: {}", e))
                        })?;

                        stats.replayed_events += 1;
                    }
                    Err(e) => {
                        let mut stats = self.stats.write().map_err(|err| {
                            ReplayError::ReplayError(format!(
                                "Failed to acquire write lock: {}",
                                err
                            ))
                        })?;

                        stats.failed_events += 1;

                        if !config.continue_on_error {
                            stats.state = ReplayState::Failed;
                            stats.end_time = Some(Utc::now());
                            return Err(ReplayError::EventSourcingError(e));
                        }
                    }
                }
            }

            // Delay between batches
            if config.batch_delay_ms > 0 {
                tokio::time::sleep(tokio::time::Duration::from_millis(config.batch_delay_ms)).await;
            }
        }

        // Mark replay as completed
        {
            let mut stats = self.stats.write().map_err(|e| {
                ReplayError::ReplayError(format!("Failed to acquire write lock: {}", e))
            })?;

            stats.state = ReplayState::Completed;
            stats.end_time = Some(Utc::now());
        }

        Ok(())
    }

    /// Pause the replay
    pub fn pause(&self) -> ReplayResult<()> {
        let mut stats = self.stats.write().map_err(|e| {
            ReplayError::ReplayError(format!("Failed to acquire write lock: {}", e))
        })?;

        if stats.state != ReplayState::InProgress {
            return Err(ReplayError::InvalidState(
                "Replay is not in progress".to_string(),
            ));
        }

        stats.state = ReplayState::Paused;

        Ok(())
    }

    /// Resume the replay
    pub fn resume(&self) -> ReplayResult<()> {
        let mut stats = self.stats.write().map_err(|e| {
            ReplayError::ReplayError(format!("Failed to acquire write lock: {}", e))
        })?;

        if stats.state != ReplayState::Paused {
            return Err(ReplayError::InvalidState(
                "Replay is not paused".to_string(),
            ));
        }

        stats.state = ReplayState::InProgress;

        Ok(())
    }

    /// Get current replay stats
    pub fn get_stats(&self) -> ReplayStats {
        self.stats.read().unwrap().clone()
    }

    /// Load events based on config
    async fn load_events(&self, config: &ReplayConfig) -> ReplayResult<Vec<DomainEvent>> {
        // If time range is specified, use that
        if let (Some(from), Some(to)) = (config.from_time, config.to_time) {
            let mut events = self.event_store.load_events_by_time_range(from, to).await?;

            // Filter by aggregate IDs if specified
            if let Some(aggregate_ids) = &config.aggregate_ids {
                events.retain(|e| aggregate_ids.contains(&e.metadata.aggregate_id));
            }

            // Filter by event types if specified
            if let Some(event_types) = &config.event_types {
                events.retain(|e| event_types.contains(&e.metadata.event_type));
            }

            return Ok(events);
        }

        // If event types are specified, load by type
        if let Some(event_types) = &config.event_types {
            let mut events = Vec::new();

            for event_type in event_types {
                let type_events = self.event_store.load_events_by_type(event_type).await?;
                events.extend(type_events);
            }

            // Filter by aggregate IDs if specified
            if let Some(aggregate_ids) = &config.aggregate_ids {
                events.retain(|e| aggregate_ids.contains(&e.metadata.aggregate_id));
            }

            return Ok(events);
        }

        // If only aggregate IDs are specified, load by aggregate
        if let Some(aggregate_ids) = &config.aggregate_ids {
            let mut events = Vec::new();

            for aggregate_id in aggregate_ids {
                let stream = self.event_store.load_events(aggregate_id).await?;
                events.extend(stream.events);
            }

            return Ok(events);
        }

        // Default: no events to replay
        Ok(Vec::new())
    }
}

/// Simple replay event handler that prints events
pub struct PrintReplayHandler;

#[async_trait]
impl ReplayEventHandler for PrintReplayHandler {
    async fn handle(&self, event: &DomainEvent) -> EventResult<()> {
        tracing::info!(
            "Replaying event: {} - {} at {}",
            event.metadata.event_type,
            event.metadata.aggregate_id,
            event.metadata.timestamp
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_sourcing::InMemoryEventStore;

    struct TestReplayHandler {
        processed: Arc<RwLock<Vec<Uuid>>>,
    }

    impl TestReplayHandler {
        fn new() -> Self {
            Self {
                processed: Arc::new(RwLock::new(Vec::new())),
            }
        }

        fn get_processed_count(&self) -> usize {
            self.processed.read().unwrap().len()
        }
    }

    #[async_trait]
    impl ReplayEventHandler for TestReplayHandler {
        async fn handle(&self, event: &DomainEvent) -> EventResult<()> {
            let mut processed = self.processed.write().unwrap();
            processed.push(event.metadata.event_id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_replay_all_events() {
        let event_store = Arc::new(InMemoryEventStore::new());

        // Add some events
        let event1 = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"title": "Test 1"}),
        );

        let event2 = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteUpdated".to_string(),
            2,
            serde_json::json!({"title": "Test 2"}),
        );

        event_store
            .append_events("statute-1", 0, vec![event1, event2])
            .await
            .unwrap();

        // Create replay manager
        let replay_manager = ReplayManager::new(event_store);

        // Create handler
        let handler = Arc::new(TestReplayHandler::new());

        // Configure replay
        let config = ReplayConfig {
            aggregate_ids: Some(vec!["statute-1".to_string()]),
            ..Default::default()
        };

        // Start replay
        replay_manager
            .replay(config, handler.clone())
            .await
            .unwrap();

        // Verify
        assert_eq!(handler.get_processed_count(), 2);

        let stats = replay_manager.get_stats();
        assert_eq!(stats.replayed_events, 2);
        assert_eq!(stats.failed_events, 0);
        assert!(stats.is_completed());
    }

    #[tokio::test]
    async fn test_replay_by_event_type() {
        let event_store = Arc::new(InMemoryEventStore::new());

        // Add some events
        let event1 = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteCreated".to_string(),
            1,
            serde_json::json!({"title": "Test 1"}),
        );

        let event2 = DomainEvent::new(
            "statute-1".to_string(),
            "Statute".to_string(),
            "StatuteUpdated".to_string(),
            2,
            serde_json::json!({"title": "Test 2"}),
        );

        event_store
            .append_events("statute-1", 0, vec![event1, event2])
            .await
            .unwrap();

        // Create replay manager
        let replay_manager = ReplayManager::new(event_store);

        // Create handler
        let handler = Arc::new(TestReplayHandler::new());

        // Configure replay (only StatuteCreated events)
        let config = ReplayConfig {
            event_types: Some(vec!["StatuteCreated".to_string()]),
            ..Default::default()
        };

        // Start replay
        replay_manager
            .replay(config, handler.clone())
            .await
            .unwrap();

        // Verify
        assert_eq!(handler.get_processed_count(), 1);
    }

    #[tokio::test]
    async fn test_replay_stats() {
        let stats = ReplayStats::new(100);

        assert_eq!(stats.total_events, 100);
        assert_eq!(stats.replayed_events, 0);
        assert_eq!(stats.progress_percentage(), 0.0);
        assert!(!stats.is_completed());
    }
}
