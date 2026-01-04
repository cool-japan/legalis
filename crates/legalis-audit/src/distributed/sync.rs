//! Synchronization protocol for distributed audit trails

use super::{DistributedError, DistributedRecord, NodeId, VectorClock};
use crate::storage::AuditStorage;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Synchronization strategy for audit records
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStrategy {
    /// Push records to other nodes immediately
    Push,
    /// Pull records from other nodes on demand
    Pull,
    /// Hybrid push-pull strategy
    Hybrid,
}

/// Synchronization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub strategy: SyncStrategy,
    pub sync_interval_secs: u64,
    pub batch_size: usize,
    pub max_retries: usize,
    pub enable_compression: bool,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            strategy: SyncStrategy::Hybrid,
            sync_interval_secs: 60,
            batch_size: 100,
            max_retries: 3,
            enable_compression: false,
        }
    }
}

/// Synchronization message between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncMessage {
    /// Request synchronization from a specific timestamp
    SyncRequest {
        from_node: NodeId,
        since: DateTime<Utc>,
        vector_clock: VectorClock,
    },
    /// Response with records to synchronize
    SyncResponse {
        from_node: NodeId,
        records: Vec<DistributedRecord>,
        vector_clock: VectorClock,
        has_more: bool,
    },
    /// Acknowledge receipt of records
    SyncAck {
        from_node: NodeId,
        record_ids: Vec<uuid::Uuid>,
        vector_clock: VectorClock,
    },
    /// Heartbeat to indicate node is alive
    Heartbeat {
        from_node: NodeId,
        vector_clock: VectorClock,
        record_count: usize,
        last_hash: Option<String>,
    },
}

/// Synchronization state for tracking progress
#[derive(Debug, Clone)]
pub struct SyncState {
    pub last_sync: DateTime<Utc>,
    pub synced_records: HashSet<uuid::Uuid>,
    pub pending_records: Vec<DistributedRecord>,
    pub failed_attempts: usize,
}

impl SyncState {
    pub fn new() -> Self {
        Self {
            last_sync: Utc::now(),
            synced_records: HashSet::new(),
            pending_records: Vec::new(),
            failed_attempts: 0,
        }
    }

    pub fn mark_synced(&mut self, record_id: uuid::Uuid) {
        self.synced_records.insert(record_id);
    }

    pub fn is_synced(&self, record_id: &uuid::Uuid) -> bool {
        self.synced_records.contains(record_id)
    }

    pub fn add_pending(&mut self, record: DistributedRecord) {
        if !self.is_synced(&record.record.id) {
            self.pending_records.push(record);
        }
    }

    pub fn clear_pending(&mut self) {
        self.pending_records.clear();
    }
}

impl Default for SyncState {
    fn default() -> Self {
        Self::new()
    }
}

/// Synchronization manager for coordinating record replication
pub struct SyncManager {
    node_id: NodeId,
    config: SyncConfig,
    storage: Arc<dyn AuditStorage>,
    sync_states: HashMap<NodeId, SyncState>,
}

impl SyncManager {
    pub fn new(node_id: NodeId, config: SyncConfig, storage: Arc<dyn AuditStorage>) -> Self {
        Self {
            node_id,
            config,
            storage,
            sync_states: HashMap::new(),
        }
    }

    /// Create a sync request for a specific node
    pub fn create_sync_request(
        &self,
        target_node: &NodeId,
        vector_clock: VectorClock,
    ) -> SyncMessage {
        let since = self
            .sync_states
            .get(target_node)
            .map(|s| s.last_sync)
            .unwrap_or_else(|| Utc::now() - Duration::days(1));

        SyncMessage::SyncRequest {
            from_node: self.node_id.clone(),
            since,
            vector_clock,
        }
    }

    /// Process an incoming sync request
    pub fn process_sync_request(
        &mut self,
        request: SyncMessage,
    ) -> Result<SyncMessage, DistributedError> {
        match request {
            SyncMessage::SyncRequest {
                from_node: _,
                since,
                vector_clock,
            } => {
                // Get records since the requested timestamp
                let records = self.get_records_since(since)?;

                let response = SyncMessage::SyncResponse {
                    from_node: self.node_id.clone(),
                    records,
                    vector_clock,
                    has_more: false,
                };

                Ok(response)
            }
            _ => Err(DistributedError::InvalidState(
                "Expected SyncRequest".to_string(),
            )),
        }
    }

    /// Process an incoming sync response
    pub fn process_sync_response(
        &mut self,
        response: SyncMessage,
    ) -> Result<SyncMessage, DistributedError> {
        match response {
            SyncMessage::SyncResponse {
                from_node,
                records,
                vector_clock,
                has_more: _,
            } => {
                let mut synced_ids = Vec::new();

                for dist_record in records {
                    // Note: Actual storage is handled externally
                    // This manager only tracks synchronization state
                    synced_ids.push(dist_record.record.id);

                    // Update sync state
                    let state = self.sync_states.entry(from_node.clone()).or_default();
                    state.mark_synced(dist_record.record.id);
                    state.add_pending(dist_record);
                }

                // Update last sync time
                if let Some(state) = self.sync_states.get_mut(&from_node) {
                    state.last_sync = Utc::now();
                }

                let ack = SyncMessage::SyncAck {
                    from_node: self.node_id.clone(),
                    record_ids: synced_ids,
                    vector_clock,
                };

                Ok(ack)
            }
            _ => Err(DistributedError::InvalidState(
                "Expected SyncResponse".to_string(),
            )),
        }
    }

    /// Process an acknowledgment message
    pub fn process_sync_ack(&mut self, ack: SyncMessage) -> Result<(), DistributedError> {
        match ack {
            SyncMessage::SyncAck {
                from_node,
                record_ids,
                vector_clock: _,
            } => {
                let state = self.sync_states.entry(from_node).or_default();

                for record_id in record_ids {
                    state.mark_synced(record_id);
                }

                state.last_sync = Utc::now();
                state.failed_attempts = 0;
                Ok(())
            }
            _ => Err(DistributedError::InvalidState(
                "Expected SyncAck".to_string(),
            )),
        }
    }

    /// Create a heartbeat message
    pub fn create_heartbeat(
        &self,
        vector_clock: VectorClock,
    ) -> Result<SyncMessage, DistributedError> {
        let record_count = self
            .storage
            .count()
            .map_err(|e| DistributedError::StorageError(e.to_string()))?;

        let last_hash = self
            .storage
            .get_last_hash()
            .map_err(|e| DistributedError::StorageError(e.to_string()))?;

        Ok(SyncMessage::Heartbeat {
            from_node: self.node_id.clone(),
            vector_clock,
            record_count,
            last_hash,
        })
    }

    /// Process a heartbeat message
    pub fn process_heartbeat(
        &mut self,
        heartbeat: SyncMessage,
    ) -> Result<Option<SyncMessage>, DistributedError> {
        match heartbeat {
            SyncMessage::Heartbeat {
                from_node,
                vector_clock,
                record_count,
                last_hash: _,
            } => {
                let state = self.sync_states.entry(from_node.clone()).or_default();

                state.last_sync = Utc::now();

                // Check if we need to sync
                let local_count = self
                    .storage
                    .count()
                    .map_err(|e| DistributedError::StorageError(e.to_string()))?;

                if record_count > local_count {
                    // Remote node has more records, request sync
                    let sync_request = self.create_sync_request(&from_node, vector_clock);
                    Ok(Some(sync_request))
                } else {
                    Ok(None)
                }
            }
            _ => Err(DistributedError::InvalidState(
                "Expected Heartbeat".to_string(),
            )),
        }
    }

    /// Get sync state for a specific node
    pub fn get_sync_state(&self, node_id: &NodeId) -> Option<&SyncState> {
        self.sync_states.get(node_id)
    }

    /// Get all sync states
    pub fn get_all_sync_states(&self) -> &HashMap<NodeId, SyncState> {
        &self.sync_states
    }

    /// Check if synchronization is needed with a node
    pub fn needs_sync(&self, node_id: &NodeId) -> bool {
        if let Some(state) = self.sync_states.get(node_id) {
            let elapsed = Utc::now().signed_duration_since(state.last_sync);
            elapsed.num_seconds() > self.config.sync_interval_secs as i64
                || !state.pending_records.is_empty()
        } else {
            true
        }
    }

    /// Get records since a specific timestamp
    fn get_records_since(
        &self,
        since: DateTime<Utc>,
    ) -> Result<Vec<DistributedRecord>, DistributedError> {
        // Get all records and filter by timestamp
        let all_records = self
            .storage
            .get_all()
            .map_err(|e| DistributedError::StorageError(e.to_string()))?;

        let mut clock = VectorClock::new();
        clock.increment(&self.node_id);

        let filtered: Vec<DistributedRecord> = all_records
            .into_iter()
            .filter(|r| r.timestamp >= since)
            .take(self.config.batch_size)
            .map(|r| DistributedRecord::new(r, self.node_id.clone(), clock.clone()))
            .collect();

        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, AuditRecord, EventType, storage::memory::MemoryStorage};

    #[test]
    fn test_sync_config_default() {
        let config = SyncConfig::default();
        assert_eq!(config.strategy, SyncStrategy::Hybrid);
        assert_eq!(config.sync_interval_secs, 60);
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.max_retries, 3);
        assert!(!config.enable_compression);
    }

    #[test]
    fn test_sync_state() {
        let mut state = SyncState::new();
        let record_id = uuid::Uuid::new_v4();

        assert!(!state.is_synced(&record_id));

        state.mark_synced(record_id);
        assert!(state.is_synced(&record_id));
    }

    #[test]
    fn test_sync_manager_creation() {
        let node_id = NodeId::new("node-1");
        let config = SyncConfig::default();
        let storage = Arc::new(MemoryStorage::new());
        let manager = SyncManager::new(node_id, config, storage);

        assert_eq!(manager.sync_states.len(), 0);
    }

    #[test]
    fn test_create_sync_request() {
        let node_id = NodeId::new("node-1");
        let config = SyncConfig::default();
        let storage = Arc::new(MemoryStorage::new());
        let manager = SyncManager::new(node_id.clone(), config, storage);

        let target = NodeId::new("node-2");
        let clock = VectorClock::new();
        let request = manager.create_sync_request(&target, clock.clone());

        match request {
            SyncMessage::SyncRequest {
                from_node,
                since: _,
                vector_clock,
            } => {
                assert_eq!(from_node, node_id);
                assert_eq!(vector_clock, clock);
            }
            _ => panic!("Expected SyncRequest"),
        }
    }

    #[test]
    fn test_process_sync_request() {
        let node_id = NodeId::new("node-1");
        let config = SyncConfig::default();
        let storage = Arc::new(MemoryStorage::new());
        let mut manager = SyncManager::new(node_id, config, storage);

        let request = SyncMessage::SyncRequest {
            from_node: NodeId::new("node-2"),
            since: Utc::now() - Duration::hours(1),
            vector_clock: VectorClock::new(),
        };

        let response = manager.process_sync_request(request).unwrap();

        match response {
            SyncMessage::SyncResponse {
                from_node: _,
                records,
                vector_clock: _,
                has_more,
            } => {
                assert_eq!(records.len(), 0);
                assert!(!has_more);
            }
            _ => panic!("Expected SyncResponse"),
        }
    }

    #[test]
    fn test_create_heartbeat() {
        let node_id = NodeId::new("node-1");
        let config = SyncConfig::default();
        let storage = Arc::new(MemoryStorage::new());
        let manager = SyncManager::new(node_id.clone(), config, storage);

        let clock = VectorClock::new();
        let heartbeat = manager.create_heartbeat(clock).unwrap();

        match heartbeat {
            SyncMessage::Heartbeat {
                from_node,
                vector_clock: _,
                record_count,
                last_hash: _,
            } => {
                assert_eq!(from_node, node_id);
                assert_eq!(record_count, 0);
            }
            _ => panic!("Expected Heartbeat"),
        }
    }

    #[test]
    fn test_process_heartbeat() {
        let node_id = NodeId::new("node-1");
        let config = SyncConfig::default();
        let storage = Arc::new(MemoryStorage::new());
        let mut manager = SyncManager::new(node_id, config, storage);

        let heartbeat = SyncMessage::Heartbeat {
            from_node: NodeId::new("node-2"),
            vector_clock: VectorClock::new(),
            record_count: 0,
            last_hash: None,
        };

        let result = manager.process_heartbeat(heartbeat).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_needs_sync() {
        let node_id = NodeId::new("node-1");
        let config = SyncConfig::default();
        let storage = Arc::new(MemoryStorage::new());
        let manager = SyncManager::new(node_id, config, storage);

        let target = NodeId::new("node-2");
        assert!(manager.needs_sync(&target));
    }

    #[test]
    fn test_process_sync_ack() {
        let node_id = NodeId::new("node-1");
        let config = SyncConfig::default();
        let storage = Arc::new(MemoryStorage::new());
        let mut manager = SyncManager::new(node_id, config, storage);

        let record_ids = vec![uuid::Uuid::new_v4()];
        let ack = SyncMessage::SyncAck {
            from_node: NodeId::new("node-2"),
            record_ids: record_ids.clone(),
            vector_clock: VectorClock::new(),
        };

        manager.process_sync_ack(ack).unwrap();

        let state = manager.get_sync_state(&NodeId::new("node-2")).unwrap();
        assert!(state.is_synced(&record_ids[0]));
        assert_eq!(state.failed_attempts, 0);
    }

    #[test]
    fn test_sync_state_pending_records() {
        let mut state = SyncState::new();

        let record = AuditRecord {
            id: uuid::Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::AutomaticDecision,
            statute_id: "TEST-1".to_string(),
            subject_id: uuid::Uuid::new_v4(),
            actor: Actor::System {
                component: "test".to_string(),
            },
            context: crate::DecisionContext::default(),
            result: crate::DecisionResult::Deterministic {
                effect_applied: "allowed".to_string(),
                parameters: std::collections::HashMap::new(),
            },
            previous_hash: None,
            record_hash: "hash123".to_string(),
        };

        let dist_record =
            DistributedRecord::new(record.clone(), NodeId::new("node-1"), VectorClock::new());

        state.add_pending(dist_record.clone());
        assert_eq!(state.pending_records.len(), 1);

        state.mark_synced(record.id);
        state.add_pending(dist_record);
        assert_eq!(state.pending_records.len(), 1); // Shouldn't add duplicate
    }
}
