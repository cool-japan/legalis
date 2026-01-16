//! Digital Twin Modeling for Legal Entities
//!
//! This module provides digital twin functionality for modeling legal entities
//! and their evolution over time, enabling real-time synchronization and scenario simulation.

use crate::Effect;
use std::collections::HashMap;

/// Digital twin representing a legal entity
///
/// # Example
///
/// ```
/// use legalis_core::digital_twin::{LegalDigitalTwin, EntityState};
///
/// let mut twin = LegalDigitalTwin::new("entity-001", "Individual");
///
/// let mut state = EntityState::new();
/// state.set_attribute("age".to_string(), "30".to_string());
/// state.set_attribute("income".to_string(), "45000".to_string());
///
/// twin.update_state(state);
///
/// assert_eq!(twin.get_current_state().get_attribute("age"), Some(&"30".to_string()));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegalDigitalTwin {
    /// Unique identifier for the digital twin
    pub id: String,
    /// Type of entity (Individual, Corporation, Government, etc.)
    pub entity_type: String,
    /// Current state of the entity
    current_state: EntityState,
    /// Historical states (timestamp -> state)
    state_history: Vec<(u64, EntityState)>,
    /// Applied effects history
    effect_history: Vec<AppliedEffect>,
    /// Metadata
    metadata: HashMap<String, String>,
}

impl LegalDigitalTwin {
    /// Create a new digital twin
    pub fn new(id: impl Into<String>, entity_type: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            entity_type: entity_type.into(),
            current_state: EntityState::new(),
            state_history: Vec::new(),
            effect_history: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Update the current state
    pub fn update_state(&mut self, state: EntityState) {
        let timestamp = current_timestamp();
        self.state_history
            .push((timestamp, self.current_state.clone()));
        self.current_state = state;
    }

    /// Get the current state
    pub fn get_current_state(&self) -> &EntityState {
        &self.current_state
    }

    /// Get state at a specific timestamp
    pub fn get_state_at(&self, timestamp: u64) -> Option<&EntityState> {
        self.state_history
            .iter()
            .rev()
            .find(|(ts, _)| *ts <= timestamp)
            .map(|(_, state)| state)
    }

    /// Apply an effect to the digital twin
    pub fn apply_effect(&mut self, statute_id: String, effect: Effect) {
        let applied = AppliedEffect {
            timestamp: current_timestamp(),
            statute_id,
            effect,
        };
        self.effect_history.push(applied);
    }

    /// Get all applied effects
    pub fn get_effect_history(&self) -> &[AppliedEffect] {
        &self.effect_history
    }

    /// Get effects applied in a time range
    pub fn get_effects_in_range(&self, start: u64, end: u64) -> Vec<&AppliedEffect> {
        self.effect_history
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Get state history length
    pub fn history_length(&self) -> usize {
        self.state_history.len()
    }

    /// Clear old history (keep only last N entries)
    pub fn prune_history(&mut self, keep_last: usize) {
        if self.state_history.len() > keep_last {
            let to_remove = self.state_history.len() - keep_last;
            self.state_history.drain(0..to_remove);
        }
    }
}

/// State of a legal entity at a point in time
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityState {
    /// Attributes (key -> value)
    attributes: HashMap<String, String>,
    /// Relationships to other entities
    relationships: HashMap<String, Vec<String>>,
    /// Timestamp when state was recorded
    pub timestamp: u64,
}

impl EntityState {
    /// Create a new entity state
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
            relationships: HashMap::new(),
            timestamp: current_timestamp(),
        }
    }

    /// Set an attribute
    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    /// Get an attribute
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    /// Add a relationship
    pub fn add_relationship(&mut self, relationship_type: String, target_id: String) {
        self.relationships
            .entry(relationship_type)
            .or_default()
            .push(target_id);
    }

    /// Get relationships of a type
    pub fn get_relationships(&self, relationship_type: &str) -> Option<&Vec<String>> {
        self.relationships.get(relationship_type)
    }

    /// Get all attributes
    pub fn all_attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    /// Get all relationships
    pub fn all_relationships(&self) -> &HashMap<String, Vec<String>> {
        &self.relationships
    }
}

impl Default for EntityState {
    fn default() -> Self {
        Self::new()
    }
}

/// An effect that has been applied to a digital twin
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AppliedEffect {
    /// Timestamp when effect was applied
    pub timestamp: u64,
    /// ID of the statute that caused this effect
    pub statute_id: String,
    /// The effect that was applied
    pub effect: Effect,
}

/// Registry for managing multiple digital twins
///
/// # Example
///
/// ```
/// use legalis_core::digital_twin::{DigitalTwinRegistry, LegalDigitalTwin};
///
/// let mut registry = DigitalTwinRegistry::new();
///
/// let twin1 = LegalDigitalTwin::new("entity-001", "Individual");
/// let twin2 = LegalDigitalTwin::new("entity-002", "Corporation");
///
/// registry.register(twin1);
/// registry.register(twin2);
///
/// assert_eq!(registry.len(), 2);
/// assert!(registry.get("entity-001").is_some());
/// ```
pub struct DigitalTwinRegistry {
    twins: HashMap<String, LegalDigitalTwin>,
}

impl DigitalTwinRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            twins: HashMap::new(),
        }
    }

    /// Register a digital twin
    pub fn register(&mut self, twin: LegalDigitalTwin) -> Result<(), RegistryError> {
        if self.twins.contains_key(&twin.id) {
            return Err(RegistryError::DuplicateId(twin.id.clone()));
        }
        self.twins.insert(twin.id.clone(), twin);
        Ok(())
    }

    /// Get a digital twin by ID
    pub fn get(&self, id: &str) -> Option<&LegalDigitalTwin> {
        self.twins.get(id)
    }

    /// Get a mutable reference to a digital twin
    pub fn get_mut(&mut self, id: &str) -> Option<&mut LegalDigitalTwin> {
        self.twins.get_mut(id)
    }

    /// Remove a digital twin
    pub fn unregister(&mut self, id: &str) -> Option<LegalDigitalTwin> {
        self.twins.remove(id)
    }

    /// Get all twin IDs
    pub fn list_ids(&self) -> Vec<&str> {
        self.twins.keys().map(|s| s.as_str()).collect()
    }

    /// Get number of registered twins
    pub fn len(&self) -> usize {
        self.twins.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.twins.is_empty()
    }

    /// Find twins by entity type
    pub fn find_by_type(&self, entity_type: &str) -> Vec<&LegalDigitalTwin> {
        self.twins
            .values()
            .filter(|twin| twin.entity_type == entity_type)
            .collect()
    }

    /// Synchronize state from one twin to another
    pub fn sync_state(&mut self, source_id: &str, target_id: &str) -> Result<(), RegistryError> {
        let source_state = self
            .get(source_id)
            .ok_or_else(|| RegistryError::TwinNotFound(source_id.to_string()))?
            .get_current_state()
            .clone();

        let target = self
            .get_mut(target_id)
            .ok_or_else(|| RegistryError::TwinNotFound(target_id.to_string()))?;

        target.update_state(source_state);
        Ok(())
    }
}

impl Default for DigitalTwinRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Twin synchronization event
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncEvent {
    /// Source twin ID
    pub source_id: String,
    /// Target twin ID
    pub target_id: String,
    /// Fields that were synchronized
    pub synced_fields: Vec<String>,
    /// Timestamp of sync
    pub timestamp: u64,
}

/// Digital twin synchronizer for real-time updates
///
/// # Example
///
/// ```
/// use legalis_core::digital_twin::{TwinSynchronizer, LegalDigitalTwin, EntityState};
///
/// let mut synchronizer = TwinSynchronizer::new();
///
/// let mut twin1 = LegalDigitalTwin::new("source", "Individual");
/// let mut twin2 = LegalDigitalTwin::new("target", "Individual");
///
/// let mut state = EntityState::new();
/// state.set_attribute("status".to_string(), "active".to_string());
/// twin1.update_state(state);
///
/// synchronizer.add_twin(twin1);
/// synchronizer.add_twin(twin2);
/// synchronizer.link("source", "target");
///
/// assert!(synchronizer.are_linked("source", "target"));
/// ```
pub struct TwinSynchronizer {
    twins: HashMap<String, LegalDigitalTwin>,
    links: HashMap<String, Vec<String>>,
    sync_history: Vec<SyncEvent>,
}

impl TwinSynchronizer {
    /// Create a new synchronizer
    pub fn new() -> Self {
        Self {
            twins: HashMap::new(),
            links: HashMap::new(),
            sync_history: Vec::new(),
        }
    }

    /// Add a twin to the synchronizer
    pub fn add_twin(&mut self, twin: LegalDigitalTwin) {
        self.twins.insert(twin.id.clone(), twin);
    }

    /// Link two twins for synchronization
    pub fn link(&mut self, source_id: &str, target_id: &str) {
        self.links
            .entry(source_id.to_string())
            .or_default()
            .push(target_id.to_string());
    }

    /// Unlink two twins
    pub fn unlink(&mut self, source_id: &str, target_id: &str) {
        if let Some(targets) = self.links.get_mut(source_id) {
            targets.retain(|id| id != target_id);
        }
    }

    /// Check if two twins are linked
    pub fn are_linked(&self, source_id: &str, target_id: &str) -> bool {
        self.links
            .get(source_id)
            .map(|targets| targets.contains(&target_id.to_string()))
            .unwrap_or(false)
    }

    /// Synchronize state from source to all linked targets
    pub fn sync(&mut self, source_id: &str) -> Result<usize, RegistryError> {
        let source_state = self
            .twins
            .get(source_id)
            .ok_or_else(|| RegistryError::TwinNotFound(source_id.to_string()))?
            .get_current_state()
            .clone();

        let targets = self.links.get(source_id).cloned().unwrap_or_default();

        let mut synced_count = 0;

        for target_id in &targets {
            if let Some(target) = self.twins.get_mut(target_id) {
                target.update_state(source_state.clone());

                let event = SyncEvent {
                    source_id: source_id.to_string(),
                    target_id: target_id.clone(),
                    synced_fields: source_state.all_attributes().keys().cloned().collect(),
                    timestamp: current_timestamp(),
                };
                self.sync_history.push(event);

                synced_count += 1;
            }
        }

        Ok(synced_count)
    }

    /// Get sync history
    pub fn get_sync_history(&self) -> &[SyncEvent] {
        &self.sync_history
    }

    /// Get a twin by ID
    pub fn get_twin(&self, id: &str) -> Option<&LegalDigitalTwin> {
        self.twins.get(id)
    }

    /// Get number of twins
    pub fn twin_count(&self) -> usize {
        self.twins.len()
    }
}

impl Default for TwinSynchronizer {
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

/// Registry errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum RegistryError {
    #[error("Duplicate twin ID: {0}")]
    DuplicateId(String),

    #[error("Twin not found: {0}")]
    TwinNotFound(String),

    #[error("Synchronization failed: {0}")]
    SyncFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_twin_creation() {
        let twin = LegalDigitalTwin::new("entity-001", "Individual");
        assert_eq!(twin.id, "entity-001");
        assert_eq!(twin.entity_type, "Individual");
    }

    #[test]
    fn test_state_update() {
        let mut twin = LegalDigitalTwin::new("entity-001", "Individual");

        let mut state = EntityState::new();
        state.set_attribute("age".to_string(), "30".to_string());

        twin.update_state(state);

        assert_eq!(
            twin.get_current_state().get_attribute("age"),
            Some(&"30".to_string())
        );
        assert_eq!(twin.history_length(), 1);
    }

    #[test]
    fn test_effect_application() {
        let mut twin = LegalDigitalTwin::new("entity-001", "Individual");

        let effect = Effect::new(crate::EffectType::Grant, "Tax benefit");
        twin.apply_effect("statute-001".to_string(), effect);

        assert_eq!(twin.get_effect_history().len(), 1);
        assert_eq!(twin.get_effect_history()[0].statute_id, "statute-001");
    }

    #[test]
    fn test_registry() {
        let mut registry = DigitalTwinRegistry::new();

        let twin1 = LegalDigitalTwin::new("entity-001", "Individual");
        let twin2 = LegalDigitalTwin::new("entity-002", "Corporation");

        assert!(registry.register(twin1).is_ok());
        assert!(registry.register(twin2).is_ok());

        assert_eq!(registry.len(), 2);
        assert!(registry.get("entity-001").is_some());
    }

    #[test]
    fn test_find_by_type() {
        let mut registry = DigitalTwinRegistry::new();

        registry
            .register(LegalDigitalTwin::new("e1", "Individual"))
            .unwrap();
        registry
            .register(LegalDigitalTwin::new("e2", "Individual"))
            .unwrap();
        registry
            .register(LegalDigitalTwin::new("e3", "Corporation"))
            .unwrap();

        let individuals = registry.find_by_type("Individual");
        assert_eq!(individuals.len(), 2);
    }

    #[test]
    fn test_synchronizer() {
        let mut synchronizer = TwinSynchronizer::new();

        let mut twin1 = LegalDigitalTwin::new("source", "Individual");
        let twin2 = LegalDigitalTwin::new("target", "Individual");

        let mut state = EntityState::new();
        state.set_attribute("status".to_string(), "active".to_string());
        twin1.update_state(state);

        synchronizer.add_twin(twin1);
        synchronizer.add_twin(twin2);
        synchronizer.link("source", "target");

        assert!(synchronizer.are_linked("source", "target"));

        let synced = synchronizer.sync("source").unwrap();
        assert_eq!(synced, 1);
    }

    #[test]
    fn test_history_pruning() {
        let mut twin = LegalDigitalTwin::new("entity-001", "Individual");

        for i in 0..10 {
            let mut state = EntityState::new();
            state.set_attribute("counter".to_string(), i.to_string());
            twin.update_state(state);
        }

        assert_eq!(twin.history_length(), 10);

        twin.prune_history(5);
        assert_eq!(twin.history_length(), 5);
    }

    #[test]
    fn test_entity_state_relationships() {
        let mut state = EntityState::new();

        state.add_relationship("employee_of".to_string(), "company-001".to_string());
        state.add_relationship("employee_of".to_string(), "company-002".to_string());

        let relationships = state.get_relationships("employee_of").unwrap();
        assert_eq!(relationships.len(), 2);
    }
}
