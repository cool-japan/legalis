//! Time-Travel Debugging for Legal Histories
//!
//! This module provides time-travel debugging capabilities for exploring legal entity
//! states at different points in time, enabling historical analysis and debugging.

use crate::digital_twin::{EntityState, LegalDigitalTwin};
use crate::event_sourcing::{EventStore, LegalEvent, SnapshotStore};
use std::collections::HashMap;

/// Time-travel debugger for legal entities
///
/// # Example
///
/// ```
/// use legalis_core::time_travel::TimeTravelDebugger;
/// use legalis_core::digital_twin::{LegalDigitalTwin, EntityState};
///
/// let mut debugger = TimeTravelDebugger::new();
///
/// let mut twin = LegalDigitalTwin::new("entity-001", "Individual");
/// let mut state = EntityState::new();
/// state.set_attribute("status".to_string(), "active".to_string());
/// twin.update_state(state);
///
/// debugger.track(twin);
///
/// assert_eq!(debugger.tracked_count(), 1);
/// ```
pub struct TimeTravelDebugger {
    twins: HashMap<String, LegalDigitalTwin>,
    event_store: EventStore,
    snapshot_store: SnapshotStore,
    checkpoints: HashMap<String, Vec<Checkpoint>>,
}

impl TimeTravelDebugger {
    /// Create a new time-travel debugger
    pub fn new() -> Self {
        Self {
            twins: HashMap::new(),
            event_store: EventStore::new(),
            snapshot_store: SnapshotStore::new(),
            checkpoints: HashMap::new(),
        }
    }

    /// Track a digital twin
    pub fn track(&mut self, twin: LegalDigitalTwin) {
        self.twins.insert(twin.id.clone(), twin);
    }

    /// Record an event
    pub fn record_event(&mut self, event: LegalEvent) {
        self.event_store.append(event);
    }

    /// Create a checkpoint for an entity
    pub fn create_checkpoint(
        &mut self,
        entity_id: &str,
        label: impl Into<String>,
    ) -> Result<String, TimeTravelError> {
        let twin = self
            .twins
            .get(entity_id)
            .ok_or_else(|| TimeTravelError::EntityNotFound(entity_id.to_string()))?;

        let checkpoint = Checkpoint {
            id: format!("checkpoint-{}-{}", entity_id, current_timestamp()),
            label: label.into(),
            entity_id: entity_id.to_string(),
            state: twin.get_current_state().clone(),
            timestamp: current_timestamp(),
        };

        let checkpoint_id = checkpoint.id.clone();

        self.checkpoints
            .entry(entity_id.to_string())
            .or_default()
            .push(checkpoint);

        Ok(checkpoint_id)
    }

    /// Get state at a specific timestamp
    pub fn get_state_at(
        &self,
        entity_id: &str,
        timestamp: u64,
    ) -> Result<EntityState, TimeTravelError> {
        let twin = self
            .twins
            .get(entity_id)
            .ok_or_else(|| TimeTravelError::EntityNotFound(entity_id.to_string()))?;

        // Try to get state from twin's history first
        if let Some(state) = twin.get_state_at(timestamp) {
            return Ok(state.clone());
        }

        // Fall back to reconstructing from events
        self.reconstruct_state_at(entity_id, timestamp)
    }

    /// Reconstruct state from events at a given timestamp
    fn reconstruct_state_at(
        &self,
        entity_id: &str,
        timestamp: u64,
    ) -> Result<EntityState, TimeTravelError> {
        // Get all events up to the timestamp
        let events = self
            .event_store
            .get_events_in_range(entity_id, 0, timestamp);

        // Start with empty state
        let mut state = EntityState::new();

        // Apply each event
        for event in events {
            match &event.event_type {
                crate::event_sourcing::EventType::AttributeChanged { key, new_value, .. } => {
                    state.set_attribute(key.clone(), new_value.clone());
                }
                crate::event_sourcing::EventType::RelationshipEstablished {
                    relationship_type,
                    target_id,
                } => {
                    state.add_relationship(relationship_type.clone(), target_id.clone());
                }
                _ => {
                    // Other event types don't directly modify state
                }
            }
        }

        Ok(state)
    }

    /// Get checkpoint by ID
    pub fn get_checkpoint(&self, checkpoint_id: &str) -> Option<&Checkpoint> {
        self.checkpoints
            .values()
            .flat_map(|v| v.iter())
            .find(|c| c.id == checkpoint_id)
    }

    /// List all checkpoints for an entity
    pub fn list_checkpoints(&self, entity_id: &str) -> Vec<&Checkpoint> {
        self.checkpoints
            .get(entity_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Restore entity to a checkpoint
    pub fn restore_checkpoint(&mut self, checkpoint_id: &str) -> Result<(), TimeTravelError> {
        let checkpoint = self
            .checkpoints
            .values()
            .flat_map(|v| v.iter())
            .find(|c| c.id == checkpoint_id)
            .ok_or_else(|| TimeTravelError::CheckpointNotFound(checkpoint_id.to_string()))?
            .clone();

        let twin = self
            .twins
            .get_mut(&checkpoint.entity_id)
            .ok_or_else(|| TimeTravelError::EntityNotFound(checkpoint.entity_id.clone()))?;

        twin.update_state(checkpoint.state);

        Ok(())
    }

    /// Compare state at two different timestamps
    pub fn compare_states(
        &self,
        entity_id: &str,
        timestamp1: u64,
        timestamp2: u64,
    ) -> Result<StateComparison, TimeTravelError> {
        let state1 = self.get_state_at(entity_id, timestamp1)?;
        let state2 = self.get_state_at(entity_id, timestamp2)?;

        Ok(StateComparison::compare(
            state1, state2, timestamp1, timestamp2,
        ))
    }

    /// Get timeline of events for an entity
    pub fn get_timeline(&self, entity_id: &str) -> Timeline {
        let events = self.event_store.get_events(entity_id);
        let checkpoints = self.list_checkpoints(entity_id);

        Timeline {
            entity_id: entity_id.to_string(),
            events: events.into_iter().cloned().collect(),
            checkpoints: checkpoints.into_iter().cloned().collect(),
        }
    }

    /// Get number of tracked entities
    pub fn tracked_count(&self) -> usize {
        self.twins.len()
    }

    /// Get event store
    pub fn event_store(&self) -> &EventStore {
        &self.event_store
    }

    /// Get snapshot store
    pub fn snapshot_store(&self) -> &SnapshotStore {
        &self.snapshot_store
    }
}

impl Default for TimeTravelDebugger {
    fn default() -> Self {
        Self::new()
    }
}

/// Checkpoint in time for an entity
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Checkpoint {
    /// Checkpoint ID
    pub id: String,
    /// Label/description
    pub label: String,
    /// Entity ID
    pub entity_id: String,
    /// State at checkpoint
    pub state: EntityState,
    /// Timestamp
    pub timestamp: u64,
}

/// Timeline showing events and checkpoints
#[derive(Debug)]
pub struct Timeline {
    /// Entity ID
    pub entity_id: String,
    /// Events in timeline
    pub events: Vec<LegalEvent>,
    /// Checkpoints in timeline
    pub checkpoints: Vec<Checkpoint>,
}

impl Timeline {
    /// Get number of events
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Get number of checkpoints
    pub fn checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }

    /// Get events in time range
    pub fn events_in_range(&self, start: u64, end: u64) -> Vec<&LegalEvent> {
        self.events
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }
}

/// Comparison of two states
#[derive(Debug)]
pub struct StateComparison {
    /// Earlier timestamp
    pub timestamp1: u64,
    /// Later timestamp
    pub timestamp2: u64,
    /// Attributes that changed
    pub changed_attributes: HashMap<String, (Option<String>, Option<String>)>,
    /// Relationships that changed
    pub changed_relationships: Vec<RelationshipChange>,
}

impl StateComparison {
    /// Compare two states
    pub fn compare(state1: EntityState, state2: EntityState, ts1: u64, ts2: u64) -> Self {
        let mut changed_attributes = HashMap::new();

        // Compare attributes
        let all_keys: std::collections::HashSet<_> = state1
            .all_attributes()
            .keys()
            .chain(state2.all_attributes().keys())
            .collect();

        for key in all_keys {
            let val1 = state1.get_attribute(key).cloned();
            let val2 = state2.get_attribute(key).cloned();

            if val1 != val2 {
                changed_attributes.insert(key.clone(), (val1, val2));
            }
        }

        Self {
            timestamp1: ts1,
            timestamp2: ts2,
            changed_attributes,
            changed_relationships: Vec::new(),
        }
    }

    /// Get number of changes
    pub fn change_count(&self) -> usize {
        self.changed_attributes.len() + self.changed_relationships.len()
    }

    /// Check if states are identical
    pub fn is_identical(&self) -> bool {
        self.change_count() == 0
    }
}

/// Change in a relationship
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RelationshipChange {
    /// Relationship was added
    Added {
        relationship_type: String,
        target_id: String,
    },
    /// Relationship was removed
    Removed {
        relationship_type: String,
        target_id: String,
    },
}

/// Time range for queries
#[derive(Debug, Clone, Copy)]
pub struct TimeRange {
    /// Start timestamp
    pub start: u64,
    /// End timestamp
    pub end: u64,
}

impl TimeRange {
    /// Create a new time range
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    /// Check if timestamp is in range
    pub fn contains(&self, timestamp: u64) -> bool {
        timestamp >= self.start && timestamp <= self.end
    }

    /// Get duration in seconds
    pub fn duration(&self) -> u64 {
        self.end.saturating_sub(self.start)
    }
}

/// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Time-travel errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum TimeTravelError {
    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Checkpoint not found: {0}")]
    CheckpointNotFound(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(u64),

    #[error("State reconstruction failed: {0}")]
    ReconstructionFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digital_twin::LegalDigitalTwin;

    #[test]
    fn test_debugger_creation() {
        let debugger = TimeTravelDebugger::new();
        assert_eq!(debugger.tracked_count(), 0);
    }

    #[test]
    fn test_track_twin() {
        let mut debugger = TimeTravelDebugger::new();

        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        debugger.track(twin);

        assert_eq!(debugger.tracked_count(), 1);
    }

    #[test]
    fn test_create_checkpoint() {
        let mut debugger = TimeTravelDebugger::new();

        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        debugger.track(twin);

        let checkpoint_id = debugger
            .create_checkpoint("entity-001", "Initial state")
            .unwrap();

        assert!(checkpoint_id.contains("checkpoint-entity-001"));
    }

    #[test]
    fn test_list_checkpoints() {
        let mut debugger = TimeTravelDebugger::new();

        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        debugger.track(twin);

        debugger
            .create_checkpoint("entity-001", "Checkpoint 1")
            .unwrap();
        debugger
            .create_checkpoint("entity-001", "Checkpoint 2")
            .unwrap();

        let checkpoints = debugger.list_checkpoints("entity-001");
        assert_eq!(checkpoints.len(), 2);
    }

    #[test]
    fn test_restore_checkpoint() {
        let mut debugger = TimeTravelDebugger::new();

        let mut twin = LegalDigitalTwin::new("entity-001", "Individual");
        let mut state = EntityState::new();
        state.set_attribute("status".to_string(), "active".to_string());
        twin.update_state(state);

        debugger.track(twin);

        let checkpoint_id = debugger
            .create_checkpoint("entity-001", "Active state")
            .unwrap();

        // Modify state
        let mut new_state = EntityState::new();
        new_state.set_attribute("status".to_string(), "inactive".to_string());
        debugger
            .twins
            .get_mut("entity-001")
            .unwrap()
            .update_state(new_state);

        // Restore checkpoint
        debugger.restore_checkpoint(&checkpoint_id).unwrap();

        let current_state = debugger
            .twins
            .get("entity-001")
            .unwrap()
            .get_current_state();
        assert_eq!(
            current_state.get_attribute("status"),
            Some(&"active".to_string())
        );
    }

    #[test]
    fn test_state_comparison() {
        let mut state1 = EntityState::new();
        state1.set_attribute("age".to_string(), "25".to_string());

        let mut state2 = EntityState::new();
        state2.set_attribute("age".to_string(), "26".to_string());

        let comparison = StateComparison::compare(state1, state2, 1000, 2000);

        assert_eq!(comparison.change_count(), 1);
        assert!(!comparison.is_identical());
    }

    #[test]
    fn test_timeline() {
        let debugger = TimeTravelDebugger::new();
        let timeline = debugger.get_timeline("entity-001");

        assert_eq!(timeline.event_count(), 0);
        assert_eq!(timeline.checkpoint_count(), 0);
    }

    #[test]
    fn test_time_range() {
        let range = TimeRange::new(1000, 2000);

        assert!(range.contains(1500));
        assert!(!range.contains(500));
        assert!(!range.contains(2500));
        assert_eq!(range.duration(), 1000);
    }

    #[test]
    fn test_get_checkpoint() {
        let mut debugger = TimeTravelDebugger::new();

        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        debugger.track(twin);

        let checkpoint_id = debugger
            .create_checkpoint("entity-001", "Test checkpoint")
            .unwrap();

        let checkpoint = debugger.get_checkpoint(&checkpoint_id).unwrap();
        assert_eq!(checkpoint.label, "Test checkpoint");
    }

    #[test]
    fn test_reconstruct_state() {
        let mut debugger = TimeTravelDebugger::new();

        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        debugger.track(twin);

        // Record some events
        let event = LegalEvent::new(
            "event-001",
            "entity-001",
            crate::event_sourcing::EventType::AttributeChanged {
                key: "age".to_string(),
                old_value: None,
                new_value: "25".to_string(),
            },
        );

        debugger.record_event(event);

        let state = debugger
            .reconstruct_state_at("entity-001", current_timestamp())
            .unwrap();
        assert_eq!(state.get_attribute("age"), Some(&"25".to_string()));
    }

    #[test]
    fn test_checkpoint_not_found() {
        let mut debugger = TimeTravelDebugger::new();

        let result = debugger.restore_checkpoint("nonexistent");
        assert!(result.is_err());
    }
}
