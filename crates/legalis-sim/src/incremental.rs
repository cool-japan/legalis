//! Incremental simulation support with dirty tracking and checkpointing.
//!
//! This module provides:
//! - Dirty tracking for efficient re-simulation
//! - Delta-based updates
//! - Checkpoint and restore functionality
//! - Simulation replay

use crate::SimulationMetrics;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Tracks which entities have changed and need re-simulation
#[derive(Debug, Clone, Default)]
pub struct DirtyTracker {
    /// Set of entity IDs that have been modified
    dirty_entities: HashSet<Uuid>,
    /// Timestamp of last clean state
    last_clean_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

impl DirtyTracker {
    /// Creates a new dirty tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks an entity as dirty (modified)
    pub fn mark_dirty(&mut self, entity_id: Uuid) {
        self.dirty_entities.insert(entity_id);
    }

    /// Marks multiple entities as dirty
    pub fn mark_many_dirty(&mut self, entity_ids: impl IntoIterator<Item = Uuid>) {
        self.dirty_entities.extend(entity_ids);
    }

    /// Checks if an entity is dirty
    pub fn is_dirty(&self, entity_id: &Uuid) -> bool {
        self.dirty_entities.contains(entity_id)
    }

    /// Returns all dirty entity IDs
    pub fn dirty_entities(&self) -> &HashSet<Uuid> {
        &self.dirty_entities
    }

    /// Returns the number of dirty entities
    pub fn dirty_count(&self) -> usize {
        self.dirty_entities.len()
    }

    /// Clears all dirty flags and updates timestamp
    pub fn clear(&mut self) {
        self.dirty_entities.clear();
        self.last_clean_timestamp = Some(chrono::Utc::now());
    }

    /// Removes specific entity from dirty set
    pub fn mark_clean(&mut self, entity_id: &Uuid) {
        self.dirty_entities.remove(entity_id);
    }

    /// Returns the timestamp of last clean state
    pub fn last_clean_timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.last_clean_timestamp
    }
}

/// Delta representing a change to an entity attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeDelta {
    /// Entity ID
    pub entity_id: Uuid,
    /// Attribute name
    pub attribute_name: String,
    /// Old value (None if attribute didn't exist)
    pub old_value: Option<String>,
    /// New value (None if attribute was deleted)
    pub new_value: Option<String>,
    /// Timestamp of change
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AttributeDelta {
    /// Creates a new attribute delta
    pub fn new(
        entity_id: Uuid,
        attribute_name: String,
        old_value: Option<String>,
        new_value: Option<String>,
    ) -> Self {
        Self {
            entity_id,
            attribute_name,
            old_value,
            new_value,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Reverses this delta (for undo operations)
    pub fn reverse(&self) -> Self {
        Self {
            entity_id: self.entity_id,
            attribute_name: self.attribute_name.clone(),
            old_value: self.new_value.clone(),
            new_value: self.old_value.clone(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Delta log for tracking all changes
#[derive(Debug, Clone, Default)]
pub struct DeltaLog {
    deltas: Vec<AttributeDelta>,
    max_size: Option<usize>,
}

impl DeltaLog {
    /// Creates a new delta log
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a delta log with a maximum size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            deltas: Vec::new(),
            max_size: Some(max_size),
        }
    }

    /// Records a delta
    pub fn record(&mut self, delta: AttributeDelta) {
        self.deltas.push(delta);

        // Trim if exceeds max size
        if let Some(max) = self.max_size {
            if self.deltas.len() > max {
                self.deltas.drain(0..self.deltas.len() - max);
            }
        }
    }

    /// Returns all deltas
    pub fn deltas(&self) -> &[AttributeDelta] {
        &self.deltas
    }

    /// Returns deltas for a specific entity
    pub fn deltas_for_entity(&self, entity_id: &Uuid) -> Vec<&AttributeDelta> {
        self.deltas
            .iter()
            .filter(|d| d.entity_id == *entity_id)
            .collect()
    }

    /// Returns deltas since a specific timestamp
    pub fn deltas_since(&self, timestamp: chrono::DateTime<chrono::Utc>) -> Vec<&AttributeDelta> {
        self.deltas
            .iter()
            .filter(|d| d.timestamp > timestamp)
            .collect()
    }

    /// Clears the delta log
    pub fn clear(&mut self) {
        self.deltas.clear();
    }

    /// Returns the number of deltas
    pub fn len(&self) -> usize {
        self.deltas.len()
    }

    /// Checks if the delta log is empty
    pub fn is_empty(&self) -> bool {
        self.deltas.is_empty()
    }
}

/// Simulation checkpoint for save/restore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Checkpoint ID
    pub id: String,
    /// Checkpoint timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Simulation metrics at checkpoint
    pub metrics: SimulationMetrics,
    /// Entity states (entity_id -> attributes)
    pub entity_states: HashMap<Uuid, HashMap<String, String>>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Checkpoint {
    /// Creates a new checkpoint
    pub fn new(id: String, metrics: SimulationMetrics) -> Self {
        Self {
            id,
            timestamp: chrono::Utc::now(),
            metrics,
            entity_states: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Adds entity state to checkpoint
    pub fn add_entity_state(&mut self, entity_id: Uuid, attributes: HashMap<String, String>) {
        self.entity_states.insert(entity_id, attributes);
    }

    /// Adds metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Serializes checkpoint to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes checkpoint from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Checkpoint manager for handling multiple checkpoints
#[derive(Debug, Default)]
pub struct CheckpointManager {
    checkpoints: IndexMap<String, Checkpoint>,
    max_checkpoints: Option<usize>,
}

impl CheckpointManager {
    /// Creates a new checkpoint manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a checkpoint manager with a maximum number of checkpoints
    pub fn with_max_checkpoints(max: usize) -> Self {
        Self {
            checkpoints: IndexMap::new(),
            max_checkpoints: Some(max),
        }
    }

    /// Saves a checkpoint
    pub fn save(&mut self, checkpoint: Checkpoint) {
        // Remove oldest checkpoint if at max capacity
        if let Some(max) = self.max_checkpoints {
            if self.checkpoints.len() >= max {
                if let Some(oldest_id) = self.oldest_checkpoint_id() {
                    self.checkpoints.shift_remove(&oldest_id);
                }
            }
        }

        self.checkpoints.insert(checkpoint.id.clone(), checkpoint);
    }

    /// Loads a checkpoint by ID
    pub fn load(&self, id: &str) -> Option<&Checkpoint> {
        self.checkpoints.get(id)
    }

    /// Deletes a checkpoint
    pub fn delete(&mut self, id: &str) -> Option<Checkpoint> {
        self.checkpoints.shift_remove(id)
    }

    /// Lists all checkpoint IDs
    pub fn list_ids(&self) -> Vec<&String> {
        self.checkpoints.keys().collect()
    }

    /// Returns the number of checkpoints
    pub fn count(&self) -> usize {
        self.checkpoints.len()
    }

    /// Clears all checkpoints
    pub fn clear(&mut self) {
        self.checkpoints.clear();
    }

    /// Finds the oldest checkpoint ID (by insertion order)
    fn oldest_checkpoint_id(&self) -> Option<String> {
        self.checkpoints.keys().next().cloned()
    }

    /// Returns the most recent checkpoint (by insertion order)
    pub fn latest(&self) -> Option<&Checkpoint> {
        self.checkpoints.values().last()
    }
}

/// Simulation replay for debugging and analysis
#[derive(Debug)]
pub struct SimulationReplay {
    /// Initial checkpoint
    #[allow(dead_code)]
    initial_state: Checkpoint,
    /// Sequence of deltas to replay
    deltas: Vec<AttributeDelta>,
    /// Current replay position
    current_position: usize,
}

impl SimulationReplay {
    /// Creates a new simulation replay
    pub fn new(initial_state: Checkpoint, deltas: Vec<AttributeDelta>) -> Self {
        Self {
            initial_state,
            deltas,
            current_position: 0,
        }
    }

    /// Steps forward by one delta
    pub fn step_forward(&mut self) -> Option<&AttributeDelta> {
        if self.current_position < self.deltas.len() {
            let delta = &self.deltas[self.current_position];
            self.current_position += 1;
            Some(delta)
        } else {
            None
        }
    }

    /// Steps backward by one delta
    pub fn step_backward(&mut self) -> Option<AttributeDelta> {
        if self.current_position > 0 {
            self.current_position -= 1;
            Some(self.deltas[self.current_position].reverse())
        } else {
            None
        }
    }

    /// Resets replay to initial state
    pub fn reset(&mut self) {
        self.current_position = 0;
    }

    /// Returns the current position
    pub fn position(&self) -> usize {
        self.current_position
    }

    /// Returns total number of deltas
    pub fn total_deltas(&self) -> usize {
        self.deltas.len()
    }

    /// Checks if replay is complete
    pub fn is_complete(&self) -> bool {
        self.current_position >= self.deltas.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirty_tracker() {
        let mut tracker = DirtyTracker::new();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        assert_eq!(tracker.dirty_count(), 0);

        tracker.mark_dirty(id1);
        assert!(tracker.is_dirty(&id1));
        assert!(!tracker.is_dirty(&id2));
        assert_eq!(tracker.dirty_count(), 1);

        tracker.mark_many_dirty(vec![id2]);
        assert_eq!(tracker.dirty_count(), 2);

        tracker.mark_clean(&id1);
        assert!(!tracker.is_dirty(&id1));
        assert_eq!(tracker.dirty_count(), 1);

        tracker.clear();
        assert_eq!(tracker.dirty_count(), 0);
        assert!(tracker.last_clean_timestamp().is_some());
    }

    #[test]
    fn test_attribute_delta() {
        let id = Uuid::new_v4();
        let delta = AttributeDelta::new(
            id,
            "age".to_string(),
            Some("25".to_string()),
            Some("26".to_string()),
        );

        assert_eq!(delta.entity_id, id);
        assert_eq!(delta.attribute_name, "age");
        assert_eq!(delta.old_value, Some("25".to_string()));
        assert_eq!(delta.new_value, Some("26".to_string()));

        let reversed = delta.reverse();
        assert_eq!(reversed.old_value, Some("26".to_string()));
        assert_eq!(reversed.new_value, Some("25".to_string()));
    }

    #[test]
    fn test_delta_log() {
        let mut log = DeltaLog::with_max_size(3);
        let id = Uuid::new_v4();

        log.record(AttributeDelta::new(
            id,
            "age".to_string(),
            None,
            Some("25".to_string()),
        ));
        assert_eq!(log.len(), 1);

        log.record(AttributeDelta::new(
            id,
            "age".to_string(),
            Some("25".to_string()),
            Some("26".to_string()),
        ));
        assert_eq!(log.len(), 2);

        let deltas = log.deltas_for_entity(&id);
        assert_eq!(deltas.len(), 2);

        // Add more deltas to test max size
        log.record(AttributeDelta::new(
            id,
            "age".to_string(),
            Some("26".to_string()),
            Some("27".to_string()),
        ));
        log.record(AttributeDelta::new(
            id,
            "age".to_string(),
            Some("27".to_string()),
            Some("28".to_string()),
        ));

        // Should only keep last 3
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn test_checkpoint() {
        let metrics = SimulationMetrics::new();
        let mut checkpoint = Checkpoint::new("test-1".to_string(), metrics);

        let id = Uuid::new_v4();
        let mut attrs = HashMap::new();
        attrs.insert("name".to_string(), "Alice".to_string());

        checkpoint.add_entity_state(id, attrs);
        checkpoint.add_metadata("version".to_string(), "1.0".to_string());

        assert!(checkpoint.entity_states.contains_key(&id));
        assert_eq!(checkpoint.metadata.get("version").unwrap(), "1.0");

        // Test serialization
        let json = checkpoint.to_json().unwrap();
        let restored = Checkpoint::from_json(&json).unwrap();
        assert_eq!(restored.id, "test-1");
    }

    #[test]
    fn test_checkpoint_manager() {
        let mut manager = CheckpointManager::with_max_checkpoints(2);

        let checkpoint1 = Checkpoint::new("cp1".to_string(), SimulationMetrics::new());
        let checkpoint2 = Checkpoint::new("cp2".to_string(), SimulationMetrics::new());
        let checkpoint3 = Checkpoint::new("cp3".to_string(), SimulationMetrics::new());

        manager.save(checkpoint1);
        manager.save(checkpoint2);
        assert_eq!(manager.count(), 2);

        manager.save(checkpoint3);
        assert_eq!(manager.count(), 2); // Should have removed oldest

        assert!(manager.load("cp1").is_none()); // Oldest was removed
        assert!(manager.load("cp2").is_some());
        assert!(manager.load("cp3").is_some());
    }

    #[test]
    fn test_simulation_replay() {
        let checkpoint = Checkpoint::new("start".to_string(), SimulationMetrics::new());
        let id = Uuid::new_v4();

        let deltas = vec![
            AttributeDelta::new(id, "age".to_string(), None, Some("25".to_string())),
            AttributeDelta::new(
                id,
                "age".to_string(),
                Some("25".to_string()),
                Some("26".to_string()),
            ),
        ];

        let mut replay = SimulationReplay::new(checkpoint, deltas);

        assert_eq!(replay.position(), 0);
        assert!(!replay.is_complete());

        let delta1 = replay.step_forward();
        assert!(delta1.is_some());
        assert_eq!(replay.position(), 1);

        let delta2 = replay.step_forward();
        assert!(delta2.is_some());
        assert_eq!(replay.position(), 2);
        assert!(replay.is_complete());

        let reversed = replay.step_backward();
        assert!(reversed.is_some());
        assert_eq!(replay.position(), 1);

        replay.reset();
        assert_eq!(replay.position(), 0);
    }
}
