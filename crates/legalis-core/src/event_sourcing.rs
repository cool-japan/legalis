//! Event Sourcing for Legal State Changes
//!
//! This module provides event sourcing capabilities for tracking legal state changes
//! as a series of immutable events, enabling full audit trails and state reconstruction.

use crate::Effect;
use std::collections::HashMap;
use std::fmt;

/// Legal event representing a state change
///
/// # Example
///
/// ```
/// use legalis_core::event_sourcing::{LegalEvent, EventType};
///
/// let event = LegalEvent::new(
///     "event-001",
///     "entity-123",
///     EventType::StatuteApplied { statute_id: "statute-001".to_string() },
/// );
///
/// assert_eq!(event.entity_id, "entity-123");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegalEvent {
    /// Event ID
    pub id: String,
    /// Entity ID this event applies to
    pub entity_id: String,
    /// Event type and data
    pub event_type: EventType,
    /// Timestamp when event occurred
    pub timestamp: u64,
    /// Event version for ordering
    pub version: u64,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl LegalEvent {
    /// Create a new legal event
    pub fn new(id: impl Into<String>, entity_id: impl Into<String>, event_type: EventType) -> Self {
        Self {
            id: id.into(),
            entity_id: entity_id.into(),
            event_type,
            timestamp: current_timestamp(),
            version: 0,
            metadata: HashMap::new(),
        }
    }

    /// Set version
    pub fn with_version(mut self, version: u64) -> Self {
        self.version = version;
        self
    }

    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Type of legal event
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EventType {
    /// Statute was applied to entity
    StatuteApplied { statute_id: String },
    /// Attribute changed
    AttributeChanged {
        key: String,
        old_value: Option<String>,
        new_value: String,
    },
    /// Relationship established
    RelationshipEstablished {
        relationship_type: String,
        target_id: String,
    },
    /// Relationship dissolved
    RelationshipDissolved {
        relationship_type: String,
        target_id: String,
    },
    /// Effect granted
    EffectGranted { effect: Effect },
    /// Effect revoked
    EffectRevoked { effect: Effect },
    /// State snapshot taken
    SnapshotCreated { snapshot_id: String },
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::StatuteApplied { statute_id } => {
                write!(f, "StatuteApplied({})", statute_id)
            }
            EventType::AttributeChanged { key, .. } => write!(f, "AttributeChanged({})", key),
            EventType::RelationshipEstablished {
                relationship_type, ..
            } => write!(f, "RelationshipEstablished({})", relationship_type),
            EventType::RelationshipDissolved {
                relationship_type, ..
            } => write!(f, "RelationshipDissolved({})", relationship_type),
            EventType::EffectGranted { .. } => write!(f, "EffectGranted"),
            EventType::EffectRevoked { .. } => write!(f, "EffectRevoked"),
            EventType::SnapshotCreated { snapshot_id } => {
                write!(f, "SnapshotCreated({})", snapshot_id)
            }
        }
    }
}

/// Event store for persisting and retrieving events
///
/// # Example
///
/// ```
/// use legalis_core::event_sourcing::{EventStore, LegalEvent, EventType};
///
/// let mut store = EventStore::new();
///
/// let event = LegalEvent::new(
///     "event-001",
///     "entity-123",
///     EventType::AttributeChanged {
///         key: "age".to_string(),
///         old_value: Some("25".to_string()),
///         new_value: "26".to_string(),
///     },
/// );
///
/// store.append(event);
///
/// assert_eq!(store.event_count(), 1);
/// assert_eq!(store.entity_event_count("entity-123"), 1);
/// ```
pub struct EventStore {
    events: Vec<LegalEvent>,
    entity_index: HashMap<String, Vec<usize>>,
    next_version: HashMap<String, u64>,
}

impl EventStore {
    /// Create a new event store
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            entity_index: HashMap::new(),
            next_version: HashMap::new(),
        }
    }

    /// Append an event to the store
    pub fn append(&mut self, mut event: LegalEvent) {
        // Assign version
        let version = self
            .next_version
            .entry(event.entity_id.clone())
            .or_insert(0);
        event.version = *version;
        *version += 1;

        // Store event
        let event_index = self.events.len();
        self.events.push(event.clone());

        // Update index
        self.entity_index
            .entry(event.entity_id.clone())
            .or_default()
            .push(event_index);
    }

    /// Get all events for an entity
    pub fn get_events(&self, entity_id: &str) -> Vec<&LegalEvent> {
        self.entity_index
            .get(entity_id)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&idx| self.events.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get events for an entity since a specific version
    pub fn get_events_since(&self, entity_id: &str, since_version: u64) -> Vec<&LegalEvent> {
        self.get_events(entity_id)
            .into_iter()
            .filter(|e| e.version >= since_version)
            .collect()
    }

    /// Get events in a time range
    pub fn get_events_in_range(&self, entity_id: &str, start: u64, end: u64) -> Vec<&LegalEvent> {
        self.get_events(entity_id)
            .into_iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    /// Get total event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Get event count for specific entity
    pub fn entity_event_count(&self, entity_id: &str) -> usize {
        self.entity_index
            .get(entity_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// Get all entity IDs
    pub fn list_entities(&self) -> Vec<&str> {
        self.entity_index.keys().map(|s| s.as_str()).collect()
    }

    /// Get latest version for entity
    pub fn get_latest_version(&self, entity_id: &str) -> Option<u64> {
        self.next_version
            .get(entity_id)
            .map(|v| v.saturating_sub(1))
    }

    /// Clear all events (use with caution!)
    pub fn clear(&mut self) {
        self.events.clear();
        self.entity_index.clear();
        self.next_version.clear();
    }
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for event subscriber callback
type EventSubscriber = Box<dyn Fn(&LegalEvent)>;

/// Event stream for subscribing to events
pub struct EventStream {
    store: EventStore,
    subscribers: HashMap<String, Vec<EventSubscriber>>,
}

impl EventStream {
    /// Create a new event stream
    pub fn new() -> Self {
        Self {
            store: EventStore::new(),
            subscribers: HashMap::new(),
        }
    }

    /// Publish an event
    pub fn publish(&mut self, event: LegalEvent) {
        // Store the event
        self.store.append(event.clone());

        // Notify subscribers
        if let Some(callbacks) = self.subscribers.get(&event.entity_id) {
            for callback in callbacks {
                callback(&event);
            }
        }
    }

    /// Get the underlying store
    pub fn store(&self) -> &EventStore {
        &self.store
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.store.event_count()
    }
}

impl Default for EventStream {
    fn default() -> Self {
        Self::new()
    }
}

/// State snapshot for optimization
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StateSnapshot {
    /// Snapshot ID
    pub id: String,
    /// Entity ID
    pub entity_id: String,
    /// Version of last applied event
    pub version: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Attributes at this point
    pub attributes: HashMap<String, String>,
    /// Relationships at this point
    pub relationships: HashMap<String, Vec<String>>,
}

impl StateSnapshot {
    /// Create a new snapshot
    pub fn new(id: impl Into<String>, entity_id: impl Into<String>, version: u64) -> Self {
        Self {
            id: id.into(),
            entity_id: entity_id.into(),
            version,
            timestamp: current_timestamp(),
            attributes: HashMap::new(),
            relationships: HashMap::new(),
        }
    }

    /// Set an attribute
    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    /// Add a relationship
    pub fn add_relationship(&mut self, relationship_type: String, target_id: String) {
        self.relationships
            .entry(relationship_type)
            .or_default()
            .push(target_id);
    }
}

/// Snapshot store for managing snapshots
pub struct SnapshotStore {
    snapshots: HashMap<String, Vec<StateSnapshot>>,
}

impl SnapshotStore {
    /// Create a new snapshot store
    pub fn new() -> Self {
        Self {
            snapshots: HashMap::new(),
        }
    }

    /// Save a snapshot
    pub fn save(&mut self, snapshot: StateSnapshot) {
        self.snapshots
            .entry(snapshot.entity_id.clone())
            .or_default()
            .push(snapshot);
    }

    /// Get latest snapshot for entity
    pub fn get_latest(&self, entity_id: &str) -> Option<&StateSnapshot> {
        self.snapshots.get(entity_id).and_then(|v| v.last())
    }

    /// Get snapshot at or before version
    pub fn get_at_version(&self, entity_id: &str, version: u64) -> Option<&StateSnapshot> {
        self.snapshots
            .get(entity_id)
            .and_then(|snapshots| snapshots.iter().rev().find(|s| s.version <= version))
    }

    /// Get snapshot count for entity
    pub fn snapshot_count(&self, entity_id: &str) -> usize {
        self.snapshots.get(entity_id).map(|v| v.len()).unwrap_or(0)
    }

    /// Prune old snapshots (keep only last N)
    pub fn prune(&mut self, entity_id: &str, keep_last: usize) {
        if let Some(snapshots) = self.snapshots.get_mut(entity_id)
            && snapshots.len() > keep_last
        {
            let to_remove = snapshots.len() - keep_last;
            snapshots.drain(0..to_remove);
        }
    }
}

impl Default for SnapshotStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EffectType;

    #[test]
    fn test_legal_event_creation() {
        let event = LegalEvent::new(
            "event-001",
            "entity-123",
            EventType::StatuteApplied {
                statute_id: "statute-001".to_string(),
            },
        );

        assert_eq!(event.id, "event-001");
        assert_eq!(event.entity_id, "entity-123");
    }

    #[test]
    fn test_event_store_append() {
        let mut store = EventStore::new();

        let event = LegalEvent::new(
            "event-001",
            "entity-123",
            EventType::AttributeChanged {
                key: "age".to_string(),
                old_value: None,
                new_value: "25".to_string(),
            },
        );

        store.append(event);

        assert_eq!(store.event_count(), 1);
        assert_eq!(store.entity_event_count("entity-123"), 1);
    }

    #[test]
    fn test_event_versioning() {
        let mut store = EventStore::new();

        let event1 = LegalEvent::new(
            "event-001",
            "entity-123",
            EventType::AttributeChanged {
                key: "age".to_string(),
                old_value: None,
                new_value: "25".to_string(),
            },
        );

        let event2 = LegalEvent::new(
            "event-002",
            "entity-123",
            EventType::AttributeChanged {
                key: "age".to_string(),
                old_value: Some("25".to_string()),
                new_value: "26".to_string(),
            },
        );

        store.append(event1);
        store.append(event2);

        let events = store.get_events("entity-123");
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].version, 0);
        assert_eq!(events[1].version, 1);
    }

    #[test]
    fn test_get_events_since() {
        let mut store = EventStore::new();

        for i in 0..5 {
            let event = LegalEvent::new(
                format!("event-{}", i),
                "entity-123",
                EventType::AttributeChanged {
                    key: "counter".to_string(),
                    old_value: None,
                    new_value: i.to_string(),
                },
            );
            store.append(event);
        }

        let recent = store.get_events_since("entity-123", 3);
        assert_eq!(recent.len(), 2); // Versions 3 and 4
    }

    #[test]
    fn test_event_stream() {
        let mut stream = EventStream::new();

        let event = LegalEvent::new(
            "event-001",
            "entity-123",
            EventType::EffectGranted {
                effect: Effect::new(EffectType::Grant, "Tax benefit"),
            },
        );

        stream.publish(event);

        assert_eq!(stream.event_count(), 1);
    }

    #[test]
    fn test_state_snapshot() {
        let mut snapshot = StateSnapshot::new("snap-001", "entity-123", 10);

        snapshot.set_attribute("age".to_string(), "30".to_string());
        snapshot.add_relationship("employee_of".to_string(), "company-001".to_string());

        assert_eq!(snapshot.version, 10);
        assert_eq!(snapshot.attributes.len(), 1);
        assert_eq!(snapshot.relationships.len(), 1);
    }

    #[test]
    fn test_snapshot_store() {
        let mut store = SnapshotStore::new();

        let snapshot1 = StateSnapshot::new("snap-001", "entity-123", 5);
        let snapshot2 = StateSnapshot::new("snap-002", "entity-123", 10);

        store.save(snapshot1);
        store.save(snapshot2);

        assert_eq!(store.snapshot_count("entity-123"), 2);

        let latest = store.get_latest("entity-123").unwrap();
        assert_eq!(latest.version, 10);
    }

    #[test]
    fn test_snapshot_at_version() {
        let mut store = SnapshotStore::new();

        let snapshot1 = StateSnapshot::new("snap-001", "entity-123", 5);
        let snapshot2 = StateSnapshot::new("snap-002", "entity-123", 10);

        store.save(snapshot1);
        store.save(snapshot2);

        let snapshot = store.get_at_version("entity-123", 7).unwrap();
        assert_eq!(snapshot.version, 5);
    }

    #[test]
    fn test_snapshot_pruning() {
        let mut store = SnapshotStore::new();

        for i in 0..10 {
            let snapshot = StateSnapshot::new(format!("snap-{}", i), "entity-123", i);
            store.save(snapshot);
        }

        assert_eq!(store.snapshot_count("entity-123"), 10);

        store.prune("entity-123", 5);
        assert_eq!(store.snapshot_count("entity-123"), 5);
    }

    #[test]
    fn test_event_type_display() {
        let event_type = EventType::StatuteApplied {
            statute_id: "statute-001".to_string(),
        };

        assert_eq!(event_type.to_string(), "StatuteApplied(statute-001)");
    }

    #[test]
    fn test_list_entities() {
        let mut store = EventStore::new();

        let event1 = LegalEvent::new(
            "e1",
            "entity-001",
            EventType::SnapshotCreated {
                snapshot_id: "s1".to_string(),
            },
        );
        let event2 = LegalEvent::new(
            "e2",
            "entity-002",
            EventType::SnapshotCreated {
                snapshot_id: "s2".to_string(),
            },
        );

        store.append(event1);
        store.append(event2);

        let entities = store.list_entities();
        assert_eq!(entities.len(), 2);
    }
}
