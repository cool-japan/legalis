//! Real-Time Statute Synchronization
//!
//! This module provides real-time synchronization of statutes across distributed systems,
//! ensuring consistency and immediate updates.

use crate::Statute;
use std::collections::{HashMap, VecDeque};
use std::fmt;

/// Real-time statute synchronization manager
///
/// # Example
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_core::realtime_sync::RealtimeSync;
///
/// let mut sync = RealtimeSync::new("node-1");
///
/// let statute = Statute::new("statute-001", "Test Statute", Effect::new(EffectType::Grant, "Test"));
/// sync.publish_update(statute);
///
/// assert_eq!(sync.pending_updates(), 1);
/// ```
pub struct RealtimeSync {
    /// Node identifier
    node_id: String,
    /// Local statute cache
    local_cache: HashMap<String, Statute>,
    /// Update queue
    update_queue: VecDeque<StatuteUpdate>,
    /// Subscribers
    subscribers: Vec<String>,
    /// Sync statistics
    stats: SyncStats,
}

impl RealtimeSync {
    /// Create a new real-time sync manager
    pub fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            local_cache: HashMap::new(),
            update_queue: VecDeque::new(),
            subscribers: Vec::new(),
            stats: SyncStats::new(),
        }
    }

    /// Publish a statute update
    pub fn publish_update(&mut self, statute: Statute) {
        let update = StatuteUpdate {
            statute_id: statute.id.clone(),
            statute,
            timestamp: current_timestamp(),
            source_node: self.node_id.clone(),
            update_type: UpdateType::Upsert,
        };

        self.update_queue.push_back(update);
        self.stats.updates_published += 1;
    }

    /// Publish a statute deletion
    pub fn publish_deletion(&mut self, statute_id: String) {
        let update = StatuteUpdate {
            statute_id: statute_id.clone(),
            statute: Statute::new(
                &statute_id,
                "Deleted",
                crate::Effect::new(crate::EffectType::Custom, "Deleted"),
            ),
            timestamp: current_timestamp(),
            source_node: self.node_id.clone(),
            update_type: UpdateType::Delete,
        };

        self.update_queue.push_back(update);
        self.stats.updates_published += 1;
    }

    /// Get next update from queue
    pub fn poll_update(&mut self) -> Option<StatuteUpdate> {
        self.update_queue.pop_front()
    }

    /// Apply an update to local cache
    pub fn apply_update(&mut self, update: StatuteUpdate) {
        match update.update_type {
            UpdateType::Upsert => {
                self.local_cache
                    .insert(update.statute_id.clone(), update.statute);
                self.stats.updates_applied += 1;
            }
            UpdateType::Delete => {
                self.local_cache.remove(&update.statute_id);
                self.stats.deletes_applied += 1;
            }
        }
    }

    /// Get statute from local cache
    pub fn get_statute(&self, statute_id: &str) -> Option<&Statute> {
        self.local_cache.get(statute_id)
    }

    /// Add a subscriber
    pub fn subscribe(&mut self, node_id: String) {
        if !self.subscribers.contains(&node_id) {
            self.subscribers.push(node_id);
        }
    }

    /// Remove a subscriber
    pub fn unsubscribe(&mut self, node_id: &str) {
        self.subscribers.retain(|id| id != node_id);
    }

    /// Get list of subscribers
    pub fn get_subscribers(&self) -> &[String] {
        &self.subscribers
    }

    /// Get number of pending updates
    pub fn pending_updates(&self) -> usize {
        self.update_queue.len()
    }

    /// Get sync statistics
    pub fn get_stats(&self) -> &SyncStats {
        &self.stats
    }

    /// Clear update queue
    pub fn clear_queue(&mut self) {
        self.update_queue.clear();
    }

    /// Get node ID
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}

/// Statute update event
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatuteUpdate {
    /// Statute ID
    pub statute_id: String,
    /// Updated statute
    pub statute: Statute,
    /// Timestamp of update
    pub timestamp: u64,
    /// Source node that created the update
    pub source_node: String,
    /// Type of update
    pub update_type: UpdateType,
}

/// Type of update
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UpdateType {
    /// Insert or update
    Upsert,
    /// Delete
    Delete,
}

impl fmt::Display for UpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateType::Upsert => write!(f, "Upsert"),
            UpdateType::Delete => write!(f, "Delete"),
        }
    }
}

/// Synchronization statistics
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncStats {
    /// Number of updates published
    pub updates_published: u64,
    /// Number of updates applied
    pub updates_applied: u64,
    /// Number of deletes applied
    pub deletes_applied: u64,
    /// Number of conflicts resolved
    pub conflicts_resolved: u64,
}

impl SyncStats {
    fn new() -> Self {
        Self {
            updates_published: 0,
            updates_applied: 0,
            deletes_applied: 0,
            conflicts_resolved: 0,
        }
    }
}

/// Conflict resolution strategy for synchronization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConflictResolution {
    /// Last write wins (based on timestamp)
    LastWriteWins,
    /// Higher version wins
    HigherVersionWins,
    /// Manual resolution required
    Manual,
}

/// Multi-node synchronization coordinator
///
/// # Example
///
/// ```
/// use legalis_core::realtime_sync::SyncCoordinator;
///
/// let mut coordinator = SyncCoordinator::new();
/// coordinator.add_node("node-1");
/// coordinator.add_node("node-2");
///
/// assert_eq!(coordinator.node_count(), 2);
/// ```
pub struct SyncCoordinator {
    nodes: HashMap<String, RealtimeSync>,
    resolution_strategy: ConflictResolution,
}

impl SyncCoordinator {
    /// Create a new synchronization coordinator
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            resolution_strategy: ConflictResolution::LastWriteWins,
        }
    }

    /// Set conflict resolution strategy
    pub fn with_resolution_strategy(mut self, strategy: ConflictResolution) -> Self {
        self.resolution_strategy = strategy;
        self
    }

    /// Add a node
    pub fn add_node(&mut self, node_id: impl Into<String>) {
        let node_id = node_id.into();
        self.nodes
            .insert(node_id.clone(), RealtimeSync::new(node_id));
    }

    /// Get a node
    pub fn get_node(&self, node_id: &str) -> Option<&RealtimeSync> {
        self.nodes.get(node_id)
    }

    /// Get a mutable node
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut RealtimeSync> {
        self.nodes.get_mut(node_id)
    }

    /// Synchronize updates between two nodes
    pub fn sync_nodes(&mut self, source_id: &str, target_id: &str) -> Result<usize, SyncError> {
        // Get updates from source
        let updates: Vec<_> = {
            let source = self
                .nodes
                .get_mut(source_id)
                .ok_or_else(|| SyncError::NodeNotFound(source_id.to_string()))?;

            let mut updates = Vec::new();
            while let Some(update) = source.poll_update() {
                updates.push(update);
            }
            updates
        };

        // Apply updates to target
        let target = self
            .nodes
            .get_mut(target_id)
            .ok_or_else(|| SyncError::NodeNotFound(target_id.to_string()))?;

        for update in &updates {
            target.apply_update(update.clone());
        }

        Ok(updates.len())
    }

    /// Broadcast updates from one node to all others
    pub fn broadcast_updates(&mut self, source_id: &str) -> Result<usize, SyncError> {
        // Get all updates from source
        let updates: Vec<_> = {
            let source = self
                .nodes
                .get_mut(source_id)
                .ok_or_else(|| SyncError::NodeNotFound(source_id.to_string()))?;

            let mut updates = Vec::new();
            while let Some(update) = source.poll_update() {
                updates.push(update);
            }
            updates
        };

        // Get target node IDs
        let target_ids: Vec<_> = self
            .nodes
            .keys()
            .filter(|id| *id != source_id)
            .cloned()
            .collect();

        // Apply updates to all targets
        let update_count = updates.len();
        for target_id in target_ids {
            let target = self
                .nodes
                .get_mut(&target_id)
                .ok_or_else(|| SyncError::NodeNotFound(target_id.clone()))?;

            for update in &updates {
                target.apply_update(update.clone());
            }
        }

        // Return total updates synced (updates * targets)
        Ok(update_count * (self.nodes.len() - 1))
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get all node IDs
    pub fn list_nodes(&self) -> Vec<&str> {
        self.nodes.keys().map(|s| s.as_str()).collect()
    }

    /// Get conflict resolution strategy
    pub fn get_resolution_strategy(&self) -> ConflictResolution {
        self.resolution_strategy
    }
}

impl Default for SyncCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Synchronization errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum SyncError {
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Synchronization conflict: {0}")]
    Conflict(String),

    #[error("Update failed: {0}")]
    UpdateFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),
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
    use crate::{Effect, EffectType};

    #[test]
    fn test_realtime_sync_creation() {
        let sync = RealtimeSync::new("node-1");
        assert_eq!(sync.node_id(), "node-1");
        assert_eq!(sync.pending_updates(), 0);
    }

    #[test]
    fn test_publish_update() {
        let mut sync = RealtimeSync::new("node-1");

        let statute = Statute::new(
            "statute-001",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        );
        sync.publish_update(statute);

        assert_eq!(sync.pending_updates(), 1);
        assert_eq!(sync.get_stats().updates_published, 1);
    }

    #[test]
    fn test_apply_update() {
        let mut sync = RealtimeSync::new("node-1");

        let statute = Statute::new(
            "statute-001",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        );
        let update = StatuteUpdate {
            statute_id: statute.id.clone(),
            statute,
            timestamp: current_timestamp(),
            source_node: "node-1".to_string(),
            update_type: UpdateType::Upsert,
        };

        sync.apply_update(update);

        assert!(sync.get_statute("statute-001").is_some());
        assert_eq!(sync.get_stats().updates_applied, 1);
    }

    #[test]
    fn test_subscription() {
        let mut sync = RealtimeSync::new("node-1");

        sync.subscribe("subscriber-1".to_string());
        sync.subscribe("subscriber-2".to_string());

        assert_eq!(sync.get_subscribers().len(), 2);

        sync.unsubscribe("subscriber-1");
        assert_eq!(sync.get_subscribers().len(), 1);
    }

    #[test]
    fn test_sync_coordinator() {
        let mut coordinator = SyncCoordinator::new();

        coordinator.add_node("node-1");
        coordinator.add_node("node-2");

        assert_eq!(coordinator.node_count(), 2);
    }

    #[test]
    fn test_node_synchronization() {
        let mut coordinator = SyncCoordinator::new();

        coordinator.add_node("node-1");
        coordinator.add_node("node-2");

        // Publish update on node-1
        let statute = Statute::new(
            "statute-001",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        );
        coordinator
            .get_node_mut("node-1")
            .unwrap()
            .publish_update(statute);

        // Sync to node-2
        let synced = coordinator.sync_nodes("node-1", "node-2").unwrap();
        assert_eq!(synced, 1);

        // Verify statute is on node-2
        assert!(
            coordinator
                .get_node("node-2")
                .unwrap()
                .get_statute("statute-001")
                .is_some()
        );
    }

    #[test]
    fn test_broadcast() {
        let mut coordinator = SyncCoordinator::new();

        coordinator.add_node("node-1");
        coordinator.add_node("node-2");
        coordinator.add_node("node-3");

        // Publish update on node-1
        let statute = Statute::new(
            "statute-001",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        );
        coordinator
            .get_node_mut("node-1")
            .unwrap()
            .publish_update(statute);

        // Broadcast to all
        let synced = coordinator.broadcast_updates("node-1").unwrap();
        assert_eq!(synced, 2); // Synced to 2 other nodes

        // Verify all nodes have the statute
        assert!(
            coordinator
                .get_node("node-2")
                .unwrap()
                .get_statute("statute-001")
                .is_some()
        );
        assert!(
            coordinator
                .get_node("node-3")
                .unwrap()
                .get_statute("statute-001")
                .is_some()
        );
    }

    #[test]
    fn test_deletion() {
        let mut sync = RealtimeSync::new("node-1");

        sync.publish_deletion("statute-001".to_string());
        assert_eq!(sync.pending_updates(), 1);

        let update = sync.poll_update().unwrap();
        assert_eq!(update.update_type, UpdateType::Delete);
    }

    #[test]
    fn test_update_type_display() {
        assert_eq!(UpdateType::Upsert.to_string(), "Upsert");
        assert_eq!(UpdateType::Delete.to_string(), "Delete");
    }
}
