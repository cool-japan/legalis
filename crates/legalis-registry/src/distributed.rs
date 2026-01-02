//! Distributed registry module for multi-node replication and consensus.
//!
//! This module implements the v0.2.0 distributed registry features:
//! - Raft consensus protocol for leader election and log replication
//! - CRDTs (Conflict-Free Replicated Data Types) for statute updates
//! - Vector clocks for partition tolerance and causality tracking
//! - Cross-datacenter synchronization
//! - Leader election with write coordination

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::time::{Duration, Instant};

// ============================================================================
// Vector Clocks - For Partition Tolerance
// ============================================================================

/// A vector clock for tracking causality in distributed systems.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorClock {
    /// Map from node ID to logical timestamp
    pub clocks: BTreeMap<String, u64>,
}

impl VectorClock {
    /// Create a new empty vector clock.
    pub fn new() -> Self {
        Self {
            clocks: BTreeMap::new(),
        }
    }

    /// Increment the clock for a specific node.
    pub fn increment(&mut self, node_id: &str) {
        let counter = self.clocks.entry(node_id.to_string()).or_insert(0);
        *counter += 1;
    }

    /// Merge this clock with another (take max of each component).
    pub fn merge(&mut self, other: &VectorClock) {
        for (node_id, &timestamp) in &other.clocks {
            let current = self.clocks.entry(node_id.clone()).or_insert(0);
            *current = (*current).max(timestamp);
        }
    }

    /// Check if this clock happened before another.
    pub fn happened_before(&self, other: &VectorClock) -> bool {
        let mut strictly_less = false;

        // Check all nodes in self
        for (node_id, &self_time) in &self.clocks {
            let other_time = other.clocks.get(node_id).copied().unwrap_or(0);
            if self_time > other_time {
                return false;
            }
            if self_time < other_time {
                strictly_less = true;
            }
        }

        // Check nodes only in other
        for node_id in other.clocks.keys() {
            if !self.clocks.contains_key(node_id) {
                strictly_less = true;
            }
        }

        strictly_less
    }

    /// Check if two clocks are concurrent (neither happened before the other).
    pub fn is_concurrent(&self, other: &VectorClock) -> bool {
        !self.happened_before(other) && !other.happened_before(self)
    }

    /// Get the timestamp for a specific node.
    pub fn get(&self, node_id: &str) -> u64 {
        self.clocks.get(node_id).copied().unwrap_or(0)
    }

    /// Set the timestamp for a specific node.
    pub fn set(&mut self, node_id: &str, timestamp: u64) {
        self.clocks.insert(node_id.to_string(), timestamp);
    }
}

impl Default for VectorClock {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CRDTs - Conflict-Free Replicated Data Types
// ============================================================================

/// CRDT operation types for statute updates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CrdtOperation {
    /// Add a tag to the statute
    AddTag { tag: String, timestamp: u64 },
    /// Remove a tag from the statute
    RemoveTag { tag: String, timestamp: u64 },
    /// Update statute field (last-write-wins)
    UpdateField {
        field: String,
        value: String,
        timestamp: u64,
    },
    /// Add metadata entry
    AddMetadata {
        key: String,
        value: String,
        timestamp: u64,
    },
    /// Remove metadata entry
    RemoveMetadata { key: String, timestamp: u64 },
}

/// A CRDT-based statute entry for conflict-free replication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdtStatuteEntry {
    /// The statute ID
    pub statute_id: String,
    /// Tags with their addition timestamps (OR-Set CRDT)
    pub tags: HashMap<String, u64>,
    /// Removed tags with their removal timestamps
    pub removed_tags: HashMap<String, u64>,
    /// Field values with timestamps (LWW-Register CRDT)
    pub fields: HashMap<String, (String, u64)>,
    /// Metadata with timestamps
    pub metadata: HashMap<String, (String, u64)>,
    /// Removed metadata keys with timestamps
    pub removed_metadata: HashMap<String, u64>,
    /// Vector clock for causality tracking
    pub vector_clock: VectorClock,
}

impl CrdtStatuteEntry {
    /// Create a new CRDT statute entry.
    pub fn new(statute_id: String) -> Self {
        Self {
            statute_id,
            tags: HashMap::new(),
            removed_tags: HashMap::new(),
            fields: HashMap::new(),
            metadata: HashMap::new(),
            removed_metadata: HashMap::new(),
            vector_clock: VectorClock::new(),
        }
    }

    /// Apply a CRDT operation to this entry.
    pub fn apply_operation(&mut self, op: CrdtOperation) {
        match op {
            CrdtOperation::AddTag { tag, timestamp } => {
                // Add-wins semantics: add if not removed or remove is older
                let removed_ts = self.removed_tags.get(&tag).copied().unwrap_or(0);
                if timestamp > removed_ts {
                    self.tags.insert(tag.clone(), timestamp);
                    self.removed_tags.remove(&tag);
                }
            }
            CrdtOperation::RemoveTag { tag, timestamp } => {
                let added_ts = self.tags.get(&tag).copied().unwrap_or(0);
                if timestamp > added_ts {
                    self.removed_tags.insert(tag.clone(), timestamp);
                    self.tags.remove(&tag);
                }
            }
            CrdtOperation::UpdateField {
                field,
                value,
                timestamp,
            } => {
                // Last-write-wins for fields
                let current_ts = self.fields.get(&field).map(|(_, ts)| *ts).unwrap_or(0);
                if timestamp > current_ts {
                    self.fields.insert(field, (value, timestamp));
                }
            }
            CrdtOperation::AddMetadata {
                key,
                value,
                timestamp,
            } => {
                let removed_ts = self.removed_metadata.get(&key).copied().unwrap_or(0);
                if timestamp > removed_ts {
                    self.metadata.insert(key.clone(), (value, timestamp));
                    self.removed_metadata.remove(&key);
                }
            }
            CrdtOperation::RemoveMetadata { key, timestamp } => {
                let added_ts = self.metadata.get(&key).map(|(_, ts)| *ts).unwrap_or(0);
                if timestamp > added_ts {
                    self.removed_metadata.insert(key.clone(), timestamp);
                    self.metadata.remove(&key);
                }
            }
        }
    }

    /// Merge this CRDT entry with another, resolving conflicts automatically.
    pub fn merge(&mut self, other: &CrdtStatuteEntry) {
        // Merge tags (OR-Set)
        for (tag, &timestamp) in &other.tags {
            let current_ts = self.tags.get(tag).copied().unwrap_or(0);
            if timestamp > current_ts {
                self.tags.insert(tag.clone(), timestamp);
            }
        }
        for (tag, &timestamp) in &other.removed_tags {
            let current_ts = self.removed_tags.get(tag).copied().unwrap_or(0);
            if timestamp > current_ts {
                self.removed_tags.insert(tag.clone(), timestamp);
            }
        }

        // Merge fields (LWW)
        for (field, (value, timestamp)) in &other.fields {
            let current_ts = self.fields.get(field).map(|(_, ts)| *ts).unwrap_or(0);
            if *timestamp > current_ts {
                self.fields
                    .insert(field.clone(), (value.clone(), *timestamp));
            }
        }

        // Merge metadata
        for (key, (value, timestamp)) in &other.metadata {
            let current_ts = self.metadata.get(key).map(|(_, ts)| *ts).unwrap_or(0);
            if *timestamp > current_ts {
                self.metadata
                    .insert(key.clone(), (value.clone(), *timestamp));
            }
        }
        for (key, &timestamp) in &other.removed_metadata {
            let current_ts = self.removed_metadata.get(key).copied().unwrap_or(0);
            if timestamp > current_ts {
                self.removed_metadata.insert(key.clone(), timestamp);
            }
        }

        // Merge vector clocks
        self.vector_clock.merge(&other.vector_clock);
    }

    /// Get the current active tags.
    pub fn active_tags(&self) -> HashSet<String> {
        self.tags.keys().cloned().collect()
    }

    /// Get the current active metadata.
    pub fn active_metadata(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|(k, (v, _))| (k.clone(), v.clone()))
            .collect()
    }
}

// ============================================================================
// Raft Consensus - Node Roles and State
// ============================================================================

/// Node role in the Raft consensus protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaftRole {
    /// Follower node
    Follower,
    /// Candidate during election
    Candidate,
    /// Leader node
    Leader,
}

/// Raft log entry for replicated state machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaftLogEntry {
    /// Log entry index
    pub index: u64,
    /// Term when entry was received by leader
    pub term: u64,
    /// Command to apply to state machine
    pub command: ReplicationCommand,
}

/// Commands that can be replicated via Raft.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationCommand {
    /// Register a new statute
    RegisterStatute { entry: StatuteEntry },
    /// Update an existing statute
    UpdateStatute {
        statute_id: String,
        entry: StatuteEntry,
    },
    /// Delete a statute
    DeleteStatute { statute_id: String },
    /// Apply CRDT operation
    ApplyCrdtOperation {
        statute_id: String,
        operation: CrdtOperation,
    },
}

/// Raft state for a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaftState {
    /// Current node ID
    pub node_id: String,
    /// Current role
    pub role: RaftRole,
    /// Current term
    pub current_term: u64,
    /// Node voted for in current term
    pub voted_for: Option<String>,
    /// Log entries
    pub log: Vec<RaftLogEntry>,
    /// Index of highest log entry known to be committed
    pub commit_index: u64,
    /// Index of highest log entry applied to state machine
    pub last_applied: u64,
    /// Leader-specific state
    pub leader_state: Option<LeaderState>,
}

/// State maintained by leader nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderState {
    /// For each server, index of next log entry to send
    pub next_index: HashMap<String, u64>,
    /// For each server, index of highest log entry known to be replicated
    pub match_index: HashMap<String, u64>,
}

impl RaftState {
    /// Create a new Raft state for a node.
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            role: RaftRole::Follower,
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            leader_state: None,
        }
    }

    /// Transition to candidate and start election.
    pub fn become_candidate(&mut self) {
        self.role = RaftRole::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.node_id.clone());
    }

    /// Transition to leader.
    pub fn become_leader(&mut self, peer_ids: &[String]) {
        self.role = RaftRole::Leader;
        let next_log_index = self.log.len() as u64 + 1;

        let mut next_index = HashMap::new();
        let mut match_index = HashMap::new();

        for peer_id in peer_ids {
            next_index.insert(peer_id.clone(), next_log_index);
            match_index.insert(peer_id.clone(), 0);
        }

        self.leader_state = Some(LeaderState {
            next_index,
            match_index,
        });
    }

    /// Transition to follower.
    pub fn become_follower(&mut self, term: u64) {
        self.role = RaftRole::Follower;
        self.current_term = term;
        self.voted_for = None;
        self.leader_state = None;
    }

    /// Append a new log entry (leader only).
    pub fn append_log_entry(&mut self, command: ReplicationCommand) -> u64 {
        let index = self.log.len() as u64 + 1;
        let entry = RaftLogEntry {
            index,
            term: self.current_term,
            command,
        };
        self.log.push(entry);
        index
    }

    /// Get the last log index.
    pub fn last_log_index(&self) -> u64 {
        self.log.len() as u64
    }

    /// Get the last log term.
    pub fn last_log_term(&self) -> u64 {
        self.log.last().map(|e| e.term).unwrap_or(0)
    }
}

// ============================================================================
// Replication Messages
// ============================================================================

/// Request vote message for leader election.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestVoteRequest {
    /// Candidate's term
    pub term: u64,
    /// Candidate requesting vote
    pub candidate_id: String,
    /// Index of candidate's last log entry
    pub last_log_index: u64,
    /// Term of candidate's last log entry
    pub last_log_term: u64,
}

/// Response to vote request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestVoteResponse {
    /// Current term, for candidate to update itself
    pub term: u64,
    /// True if candidate received vote
    pub vote_granted: bool,
}

/// Append entries RPC for log replication and heartbeat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendEntriesRequest {
    /// Leader's term
    pub term: u64,
    /// Leader ID
    pub leader_id: String,
    /// Index of log entry immediately preceding new ones
    pub prev_log_index: u64,
    /// Term of prev_log_index entry
    pub prev_log_term: u64,
    /// Log entries to store (empty for heartbeat)
    pub entries: Vec<RaftLogEntry>,
    /// Leader's commit index
    pub leader_commit: u64,
}

/// Response to append entries RPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    /// Current term, for leader to update itself
    pub term: u64,
    /// True if follower contained entry matching prev_log_index and prev_log_term
    pub success: bool,
    /// The index of the last log entry that was successfully replicated
    pub match_index: u64,
}

// ============================================================================
// Cluster Configuration
// ============================================================================

/// Configuration for a distributed registry cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// This node's ID
    pub node_id: String,
    /// All node IDs in the cluster
    pub nodes: Vec<String>,
    /// Election timeout range (milliseconds)
    pub election_timeout_ms: (u64, u64),
    /// Heartbeat interval (milliseconds)
    pub heartbeat_interval_ms: u64,
    /// RPC timeout (milliseconds)
    pub rpc_timeout_ms: u64,
}

impl ClusterConfig {
    /// Create a default cluster configuration.
    pub fn new(node_id: String, nodes: Vec<String>) -> Self {
        Self {
            node_id,
            nodes,
            election_timeout_ms: (150, 300),
            heartbeat_interval_ms: 50,
            rpc_timeout_ms: 100,
        }
    }

    /// Get peer node IDs (all nodes except this one).
    pub fn peer_ids(&self) -> Vec<String> {
        self.nodes
            .iter()
            .filter(|id| *id != &self.node_id)
            .cloned()
            .collect()
    }

    /// Calculate a random election timeout.
    pub fn random_election_timeout(&self) -> Duration {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        let (min, max) = self.election_timeout_ms;
        let range = max - min;

        // Simple pseudo-random based on current time
        let hasher = RandomState::new().build_hasher();
        let seed = hasher.finish();
        let offset = seed % range;

        Duration::from_millis(min + offset)
    }
}

// ============================================================================
// Cross-Datacenter Synchronization
// ============================================================================

/// Datacenter identifier.
pub type DatacenterId = String;

/// Cross-datacenter synchronization state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDcSyncState {
    /// Local datacenter ID
    pub local_dc: DatacenterId,
    /// Remote datacenters
    pub remote_dcs: HashMap<DatacenterId, RemoteDcState>,
    /// Last sync timestamp for each remote DC
    pub last_sync: HashMap<DatacenterId, DateTime<Utc>>,
}

/// State of a remote datacenter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteDcState {
    /// Remote datacenter ID
    pub dc_id: DatacenterId,
    /// Endpoint URL for sync
    pub endpoint: String,
    /// Last known vector clock
    pub vector_clock: VectorClock,
    /// Connection status
    pub status: DcConnectionStatus,
    /// Round-trip latency
    pub latency_ms: Option<u64>,
}

/// Datacenter connection status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DcConnectionStatus {
    /// Connected and healthy
    Connected,
    /// Connection degraded
    Degraded,
    /// Disconnected
    Disconnected,
}

impl CrossDcSyncState {
    /// Create a new cross-DC sync state.
    pub fn new(local_dc: DatacenterId) -> Self {
        Self {
            local_dc,
            remote_dcs: HashMap::new(),
            last_sync: HashMap::new(),
        }
    }

    /// Add a remote datacenter.
    pub fn add_remote_dc(&mut self, dc_id: DatacenterId, endpoint: String) {
        let state = RemoteDcState {
            dc_id: dc_id.clone(),
            endpoint,
            vector_clock: VectorClock::new(),
            status: DcConnectionStatus::Disconnected,
            latency_ms: None,
        };
        self.remote_dcs.insert(dc_id, state);
    }

    /// Update remote DC status.
    pub fn update_status(
        &mut self,
        dc_id: &DatacenterId,
        status: DcConnectionStatus,
        latency_ms: Option<u64>,
    ) {
        if let Some(dc_state) = self.remote_dcs.get_mut(dc_id) {
            dc_state.status = status;
            dc_state.latency_ms = latency_ms;
        }
    }

    /// Record successful sync with a remote DC.
    pub fn record_sync(&mut self, dc_id: &DatacenterId, remote_clock: VectorClock) {
        self.last_sync.insert(dc_id.clone(), Utc::now());
        if let Some(dc_state) = self.remote_dcs.get_mut(dc_id) {
            dc_state.vector_clock = remote_clock;
        }
    }

    /// Get all healthy remote DCs.
    pub fn healthy_dcs(&self) -> Vec<&RemoteDcState> {
        self.remote_dcs
            .values()
            .filter(|dc| dc.status == DcConnectionStatus::Connected)
            .collect()
    }

    /// Get stale DCs (not synced recently).
    pub fn stale_dcs(&self, threshold: Duration) -> Vec<&DatacenterId> {
        let now = Utc::now();
        self.remote_dcs
            .keys()
            .filter(|dc_id| match self.last_sync.get(*dc_id) {
                Some(last) => {
                    now.signed_duration_since(*last)
                        .to_std()
                        .unwrap_or(Duration::MAX)
                        > threshold
                }
                None => true,
            })
            .collect()
    }
}

// ============================================================================
// Leader Election Tracker
// ============================================================================

/// Tracks leader election state and history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderElection {
    /// Current leader node ID
    pub current_leader: Option<String>,
    /// Current term
    pub current_term: u64,
    /// Election history
    pub history: VecDeque<ElectionRecord>,
    /// Last election timestamp
    pub last_election: Option<DateTime<Utc>>,
    /// Election timeout tracker
    #[serde(skip)]
    pub last_heartbeat: Option<Instant>,
}

/// Record of a past election.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionRecord {
    /// Term number
    pub term: u64,
    /// Elected leader
    pub leader: String,
    /// Election timestamp
    pub timestamp: DateTime<Utc>,
    /// Number of votes received
    pub votes: usize,
    /// Total number of nodes
    pub total_nodes: usize,
}

impl LeaderElection {
    /// Create a new leader election tracker.
    pub fn new() -> Self {
        Self {
            current_leader: None,
            current_term: 0,
            history: VecDeque::new(),
            last_election: None,
            last_heartbeat: None,
        }
    }

    /// Record a new election.
    pub fn record_election(&mut self, term: u64, leader: String, votes: usize, total_nodes: usize) {
        self.current_leader = Some(leader.clone());
        self.current_term = term;
        self.last_election = Some(Utc::now());
        self.last_heartbeat = Some(Instant::now());

        let record = ElectionRecord {
            term,
            leader,
            timestamp: Utc::now(),
            votes,
            total_nodes,
        };

        self.history.push_back(record);

        // Keep only last 100 elections
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    /// Record heartbeat from leader.
    pub fn record_heartbeat(&mut self) {
        self.last_heartbeat = Some(Instant::now());
    }

    /// Check if election timeout has occurred.
    pub fn is_timeout(&self, timeout: Duration) -> bool {
        match self.last_heartbeat {
            Some(last) => last.elapsed() > timeout,
            None => true,
        }
    }

    /// Get the current leader.
    pub fn leader(&self) -> Option<&String> {
        self.current_leader.as_ref()
    }

    /// Check if a specific node is the leader.
    pub fn is_leader(&self, node_id: &str) -> bool {
        self.current_leader.as_deref() == Some(node_id)
    }

    /// Get election history.
    pub fn election_history(&self) -> &VecDeque<ElectionRecord> {
        &self.history
    }

    /// Get the number of leader changes.
    pub fn leader_changes(&self) -> usize {
        let mut changes: usize = 0;
        let mut prev_leader: Option<&String> = None;

        for record in &self.history {
            if prev_leader != Some(&record.leader) {
                changes += 1;
            }
            prev_leader = Some(&record.leader);
        }

        changes.saturating_sub(1) // Don't count initial election
    }
}

impl Default for LeaderElection {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Distributed Registry Manager
// ============================================================================

/// Manager for distributed registry operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedRegistry {
    /// Raft consensus state
    pub raft_state: RaftState,
    /// Cluster configuration
    pub cluster_config: ClusterConfig,
    /// CRDT entries for each statute
    pub crdt_entries: HashMap<String, CrdtStatuteEntry>,
    /// Cross-datacenter sync state
    pub cross_dc_sync: CrossDcSyncState,
    /// Leader election tracker
    pub leader_election: LeaderElection,
    /// Local node vector clock
    pub local_clock: VectorClock,
}

impl DistributedRegistry {
    /// Create a new distributed registry.
    pub fn new(node_id: String, cluster_nodes: Vec<String>, datacenter_id: String) -> Self {
        let cluster_config = ClusterConfig::new(node_id.clone(), cluster_nodes);
        let raft_state = RaftState::new(node_id.clone());
        let cross_dc_sync = CrossDcSyncState::new(datacenter_id);
        let mut local_clock = VectorClock::new();
        local_clock.increment(&node_id);

        Self {
            raft_state,
            cluster_config,
            crdt_entries: HashMap::new(),
            cross_dc_sync,
            leader_election: LeaderElection::new(),
            local_clock,
        }
    }

    /// Apply a CRDT operation to a statute.
    pub fn apply_crdt_operation(&mut self, statute_id: &str, operation: CrdtOperation) {
        let entry = self
            .crdt_entries
            .entry(statute_id.to_string())
            .or_insert_with(|| CrdtStatuteEntry::new(statute_id.to_string()));

        entry.apply_operation(operation.clone());

        // Increment local vector clock
        self.local_clock.increment(&self.cluster_config.node_id);
        entry.vector_clock = self.local_clock.clone();

        // If leader, replicate to followers
        if self.raft_state.role == RaftRole::Leader {
            let command = ReplicationCommand::ApplyCrdtOperation {
                statute_id: statute_id.to_string(),
                operation,
            };
            self.raft_state.append_log_entry(command);
        }
    }

    /// Merge a remote CRDT entry.
    pub fn merge_crdt_entry(&mut self, remote_entry: CrdtStatuteEntry) {
        let statute_id = remote_entry.statute_id.clone();
        let entry = self
            .crdt_entries
            .entry(statute_id)
            .or_insert_with(|| CrdtStatuteEntry::new(remote_entry.statute_id.clone()));

        entry.merge(&remote_entry);
        self.local_clock.merge(&remote_entry.vector_clock);
    }

    /// Start an election (become candidate).
    pub fn start_election(&mut self) -> RequestVoteRequest {
        self.raft_state.become_candidate();

        RequestVoteRequest {
            term: self.raft_state.current_term,
            candidate_id: self.raft_state.node_id.clone(),
            last_log_index: self.raft_state.last_log_index(),
            last_log_term: self.raft_state.last_log_term(),
        }
    }

    /// Handle vote request.
    pub fn handle_vote_request(&mut self, request: RequestVoteRequest) -> RequestVoteResponse {
        // Grant vote if:
        // 1. Candidate's term >= our term
        // 2. We haven't voted in this term, or already voted for this candidate
        // 3. Candidate's log is at least as up-to-date as ours

        let vote_granted = if request.term < self.raft_state.current_term {
            false
        } else {
            if request.term > self.raft_state.current_term {
                self.raft_state.become_follower(request.term);
            }

            let can_vote = self.raft_state.voted_for.is_none()
                || self.raft_state.voted_for.as_ref() == Some(&request.candidate_id);

            let log_ok = request.last_log_term > self.raft_state.last_log_term()
                || (request.last_log_term == self.raft_state.last_log_term()
                    && request.last_log_index >= self.raft_state.last_log_index());

            if can_vote && log_ok {
                self.raft_state.voted_for = Some(request.candidate_id.clone());
                true
            } else {
                false
            }
        };

        RequestVoteResponse {
            term: self.raft_state.current_term,
            vote_granted,
        }
    }

    /// Handle winning an election.
    pub fn become_leader(&mut self) {
        let peer_ids = self.cluster_config.peer_ids();
        self.raft_state.become_leader(&peer_ids);
        self.leader_election.record_election(
            self.raft_state.current_term,
            self.raft_state.node_id.clone(),
            peer_ids.len().div_ceil(2) + 1,
            peer_ids.len() + 1,
        );
    }

    /// Create an append entries request (heartbeat or replication).
    pub fn create_append_entries(&self, peer_id: &str) -> Option<AppendEntriesRequest> {
        if self.raft_state.role != RaftRole::Leader {
            return None;
        }

        let leader_state = self.raft_state.leader_state.as_ref()?;
        let next_index = leader_state.next_index.get(peer_id)?;

        let prev_log_index = next_index.saturating_sub(1);
        let prev_log_term = if prev_log_index > 0 {
            self.raft_state
                .log
                .get((prev_log_index - 1) as usize)
                .map(|e| e.term)
                .unwrap_or(0)
        } else {
            0
        };

        let entries = self
            .raft_state
            .log
            .iter()
            .skip(*next_index as usize)
            .cloned()
            .collect();

        Some(AppendEntriesRequest {
            term: self.raft_state.current_term,
            leader_id: self.raft_state.node_id.clone(),
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit: self.raft_state.commit_index,
        })
    }

    /// Handle append entries request.
    pub fn handle_append_entries(
        &mut self,
        request: AppendEntriesRequest,
    ) -> AppendEntriesResponse {
        self.leader_election.record_heartbeat();

        if request.term < self.raft_state.current_term {
            return AppendEntriesResponse {
                term: self.raft_state.current_term,
                success: false,
                match_index: 0,
            };
        }

        if request.term > self.raft_state.current_term {
            self.raft_state.become_follower(request.term);
        }

        // Update leader
        if self.leader_election.current_leader.as_ref() != Some(&request.leader_id) {
            self.leader_election.current_leader = Some(request.leader_id.clone());
        }

        // Check if log contains entry at prev_log_index with matching term
        let log_ok = if request.prev_log_index == 0 {
            true
        } else if let Some(entry) = self
            .raft_state
            .log
            .get((request.prev_log_index - 1) as usize)
        {
            entry.term == request.prev_log_term
        } else {
            false
        };

        if !log_ok {
            return AppendEntriesResponse {
                term: self.raft_state.current_term,
                success: false,
                match_index: 0,
            };
        }

        // Append new entries
        let mut match_index = request.prev_log_index;
        for entry in request.entries {
            let idx = (entry.index - 1) as usize;
            if idx < self.raft_state.log.len() {
                if self.raft_state.log[idx].term != entry.term {
                    self.raft_state.log.truncate(idx);
                    self.raft_state.log.push(entry.clone());
                }
            } else {
                self.raft_state.log.push(entry.clone());
            }
            match_index = entry.index;
        }

        // Update commit index
        if request.leader_commit > self.raft_state.commit_index {
            self.raft_state.commit_index = request.leader_commit.min(match_index);
        }

        AppendEntriesResponse {
            term: self.raft_state.current_term,
            success: true,
            match_index,
        }
    }

    /// Add a remote datacenter for cross-DC sync.
    pub fn add_remote_datacenter(&mut self, dc_id: String, endpoint: String) {
        self.cross_dc_sync.add_remote_dc(dc_id, endpoint);
    }

    /// Check if this node is the leader.
    pub fn is_leader(&self) -> bool {
        self.raft_state.role == RaftRole::Leader
    }

    /// Get the current leader.
    pub fn current_leader(&self) -> Option<&String> {
        self.leader_election.leader()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_clock_new() {
        let clock = VectorClock::new();
        assert!(clock.clocks.is_empty());
    }

    #[test]
    fn test_vector_clock_increment() {
        let mut clock = VectorClock::new();
        clock.increment("node1");
        assert_eq!(clock.get("node1"), 1);
        clock.increment("node1");
        assert_eq!(clock.get("node1"), 2);
        clock.increment("node2");
        assert_eq!(clock.get("node2"), 1);
    }

    #[test]
    fn test_vector_clock_merge() {
        let mut clock1 = VectorClock::new();
        clock1.set("node1", 5);
        clock1.set("node2", 3);

        let mut clock2 = VectorClock::new();
        clock2.set("node1", 3);
        clock2.set("node2", 7);
        clock2.set("node3", 2);

        clock1.merge(&clock2);

        assert_eq!(clock1.get("node1"), 5); // max(5, 3)
        assert_eq!(clock1.get("node2"), 7); // max(3, 7)
        assert_eq!(clock1.get("node3"), 2); // max(0, 2)
    }

    #[test]
    fn test_vector_clock_happened_before() {
        let mut clock1 = VectorClock::new();
        clock1.set("node1", 1);
        clock1.set("node2", 2);

        let mut clock2 = VectorClock::new();
        clock2.set("node1", 1);
        clock2.set("node2", 3);

        assert!(clock1.happened_before(&clock2));
        assert!(!clock2.happened_before(&clock1));
    }

    #[test]
    fn test_vector_clock_concurrent() {
        let mut clock1 = VectorClock::new();
        clock1.set("node1", 2);
        clock1.set("node2", 1);

        let mut clock2 = VectorClock::new();
        clock2.set("node1", 1);
        clock2.set("node2", 2);

        assert!(clock1.is_concurrent(&clock2));
        assert!(clock2.is_concurrent(&clock1));
    }

    #[test]
    fn test_crdt_statute_entry_new() {
        let entry = CrdtStatuteEntry::new("statute-1".to_string());
        assert_eq!(entry.statute_id, "statute-1");
        assert!(entry.tags.is_empty());
        assert!(entry.fields.is_empty());
    }

    #[test]
    fn test_crdt_add_tag() {
        let mut entry = CrdtStatuteEntry::new("statute-1".to_string());
        let op = CrdtOperation::AddTag {
            tag: "tax".to_string(),
            timestamp: 100,
        };
        entry.apply_operation(op);

        assert!(entry.tags.contains_key("tax"));
        assert_eq!(entry.tags.get("tax"), Some(&100));
        assert_eq!(entry.active_tags().len(), 1);
    }

    #[test]
    fn test_crdt_remove_tag() {
        let mut entry = CrdtStatuteEntry::new("statute-1".to_string());

        // Add tag first
        entry.apply_operation(CrdtOperation::AddTag {
            tag: "tax".to_string(),
            timestamp: 100,
        });

        // Remove tag
        entry.apply_operation(CrdtOperation::RemoveTag {
            tag: "tax".to_string(),
            timestamp: 200,
        });

        assert!(!entry.tags.contains_key("tax"));
        assert!(entry.removed_tags.contains_key("tax"));
        assert_eq!(entry.active_tags().len(), 0);
    }

    #[test]
    fn test_crdt_add_wins_semantics() {
        let mut entry = CrdtStatuteEntry::new("statute-1".to_string());

        // Remove at timestamp 100
        entry.apply_operation(CrdtOperation::RemoveTag {
            tag: "tax".to_string(),
            timestamp: 100,
        });

        // Add at timestamp 200 (later) - should win
        entry.apply_operation(CrdtOperation::AddTag {
            tag: "tax".to_string(),
            timestamp: 200,
        });

        assert!(entry.tags.contains_key("tax"));
        assert_eq!(entry.active_tags().len(), 1);
    }

    #[test]
    fn test_crdt_update_field_lww() {
        let mut entry = CrdtStatuteEntry::new("statute-1".to_string());

        entry.apply_operation(CrdtOperation::UpdateField {
            field: "title".to_string(),
            value: "Old Title".to_string(),
            timestamp: 100,
        });

        entry.apply_operation(CrdtOperation::UpdateField {
            field: "title".to_string(),
            value: "New Title".to_string(),
            timestamp: 200,
        });

        let (value, _) = entry.fields.get("title").unwrap();
        assert_eq!(value, "New Title");
    }

    #[test]
    fn test_crdt_merge_entries() {
        let mut entry1 = CrdtStatuteEntry::new("statute-1".to_string());
        entry1.apply_operation(CrdtOperation::AddTag {
            tag: "tax".to_string(),
            timestamp: 100,
        });
        entry1.apply_operation(CrdtOperation::UpdateField {
            field: "title".to_string(),
            value: "Title 1".to_string(),
            timestamp: 100,
        });

        let mut entry2 = CrdtStatuteEntry::new("statute-1".to_string());
        entry2.apply_operation(CrdtOperation::AddTag {
            tag: "finance".to_string(),
            timestamp: 150,
        });
        entry2.apply_operation(CrdtOperation::UpdateField {
            field: "title".to_string(),
            value: "Title 2".to_string(),
            timestamp: 200,
        });

        entry1.merge(&entry2);

        assert_eq!(entry1.active_tags().len(), 2);
        assert!(entry1.tags.contains_key("tax"));
        assert!(entry1.tags.contains_key("finance"));

        let (title, _) = entry1.fields.get("title").unwrap();
        assert_eq!(title, "Title 2"); // LWW - entry2 has later timestamp
    }

    #[test]
    fn test_raft_state_new() {
        let state = RaftState::new("node1".to_string());
        assert_eq!(state.node_id, "node1");
        assert_eq!(state.role, RaftRole::Follower);
        assert_eq!(state.current_term, 0);
        assert!(state.voted_for.is_none());
        assert_eq!(state.log.len(), 0);
    }

    #[test]
    fn test_raft_become_candidate() {
        let mut state = RaftState::new("node1".to_string());
        state.become_candidate();

        assert_eq!(state.role, RaftRole::Candidate);
        assert_eq!(state.current_term, 1);
        assert_eq!(state.voted_for, Some("node1".to_string()));
    }

    #[test]
    fn test_raft_become_leader() {
        let mut state = RaftState::new("node1".to_string());
        state.become_candidate();

        let peers = vec!["node2".to_string(), "node3".to_string()];
        state.become_leader(&peers);

        assert_eq!(state.role, RaftRole::Leader);
        assert!(state.leader_state.is_some());

        let leader_state = state.leader_state.as_ref().unwrap();
        assert_eq!(leader_state.next_index.len(), 2);
        assert_eq!(leader_state.match_index.len(), 2);
    }

    #[test]
    fn test_raft_become_follower() {
        let mut state = RaftState::new("node1".to_string());
        state.become_candidate();
        state.become_follower(5);

        assert_eq!(state.role, RaftRole::Follower);
        assert_eq!(state.current_term, 5);
        assert!(state.voted_for.is_none());
        assert!(state.leader_state.is_none());
    }

    #[test]
    fn test_raft_append_log_entry() {
        let mut state = RaftState::new("node1".to_string());
        state.current_term = 1;

        let command = ReplicationCommand::DeleteStatute {
            statute_id: "statute-1".to_string(),
        };

        let index = state.append_log_entry(command);

        assert_eq!(index, 1);
        assert_eq!(state.log.len(), 1);
        assert_eq!(state.log[0].index, 1);
        assert_eq!(state.log[0].term, 1);
    }

    #[test]
    fn test_cluster_config_peer_ids() {
        let nodes = vec![
            "node1".to_string(),
            "node2".to_string(),
            "node3".to_string(),
        ];
        let config = ClusterConfig::new("node1".to_string(), nodes);

        let peers = config.peer_ids();
        assert_eq!(peers.len(), 2);
        assert!(peers.contains(&"node2".to_string()));
        assert!(peers.contains(&"node3".to_string()));
        assert!(!peers.contains(&"node1".to_string()));
    }

    #[test]
    fn test_cross_dc_sync_state_new() {
        let sync = CrossDcSyncState::new("dc1".to_string());
        assert_eq!(sync.local_dc, "dc1");
        assert!(sync.remote_dcs.is_empty());
        assert!(sync.last_sync.is_empty());
    }

    #[test]
    fn test_cross_dc_add_remote() {
        let mut sync = CrossDcSyncState::new("dc1".to_string());
        sync.add_remote_dc("dc2".to_string(), "https://dc2.example.com".to_string());

        assert_eq!(sync.remote_dcs.len(), 1);
        let dc2 = sync.remote_dcs.get("dc2").unwrap();
        assert_eq!(dc2.dc_id, "dc2");
        assert_eq!(dc2.endpoint, "https://dc2.example.com");
        assert_eq!(dc2.status, DcConnectionStatus::Disconnected);
    }

    #[test]
    fn test_cross_dc_update_status() {
        let mut sync = CrossDcSyncState::new("dc1".to_string());
        sync.add_remote_dc("dc2".to_string(), "https://dc2.example.com".to_string());

        sync.update_status(&"dc2".to_string(), DcConnectionStatus::Connected, Some(50));

        let dc2 = sync.remote_dcs.get("dc2").unwrap();
        assert_eq!(dc2.status, DcConnectionStatus::Connected);
        assert_eq!(dc2.latency_ms, Some(50));
    }

    #[test]
    fn test_cross_dc_healthy_dcs() {
        let mut sync = CrossDcSyncState::new("dc1".to_string());
        sync.add_remote_dc("dc2".to_string(), "https://dc2.example.com".to_string());
        sync.add_remote_dc("dc3".to_string(), "https://dc3.example.com".to_string());

        sync.update_status(&"dc2".to_string(), DcConnectionStatus::Connected, Some(50));
        sync.update_status(&"dc3".to_string(), DcConnectionStatus::Disconnected, None);

        let healthy = sync.healthy_dcs();
        assert_eq!(healthy.len(), 1);
        assert_eq!(healthy[0].dc_id, "dc2");
    }

    #[test]
    fn test_leader_election_new() {
        let election = LeaderElection::new();
        assert!(election.current_leader.is_none());
        assert_eq!(election.current_term, 0);
        assert_eq!(election.history.len(), 0);
    }

    #[test]
    fn test_leader_election_record() {
        let mut election = LeaderElection::new();
        election.record_election(1, "node1".to_string(), 2, 3);

        assert_eq!(election.current_leader, Some("node1".to_string()));
        assert_eq!(election.current_term, 1);
        assert_eq!(election.history.len(), 1);
        assert!(election.last_election.is_some());
        assert!(election.last_heartbeat.is_some());
    }

    #[test]
    fn test_leader_election_is_leader() {
        let mut election = LeaderElection::new();
        election.record_election(1, "node1".to_string(), 2, 3);

        assert!(election.is_leader("node1"));
        assert!(!election.is_leader("node2"));
    }

    #[test]
    fn test_leader_election_timeout() {
        let mut election = LeaderElection::new();
        election.record_heartbeat();

        // Should not be timeout immediately
        assert!(!election.is_timeout(Duration::from_secs(1)));

        // Should timeout if we wait (simulated by removing heartbeat)
        election.last_heartbeat = None;
        assert!(election.is_timeout(Duration::from_secs(1)));
    }

    #[test]
    fn test_leader_changes() {
        let mut election = LeaderElection::new();

        election.record_election(1, "node1".to_string(), 2, 3);
        election.record_election(2, "node2".to_string(), 2, 3);
        election.record_election(3, "node1".to_string(), 2, 3);
        election.record_election(4, "node1".to_string(), 2, 3);

        // node1 -> node2 -> node1 = 2 leader changes
        assert_eq!(election.leader_changes(), 2);
    }

    #[test]
    fn test_distributed_registry_new() {
        let nodes = vec![
            "node1".to_string(),
            "node2".to_string(),
            "node3".to_string(),
        ];
        let registry = DistributedRegistry::new("node1".to_string(), nodes, "dc1".to_string());

        assert_eq!(registry.raft_state.node_id, "node1");
        assert_eq!(registry.cluster_config.node_id, "node1");
        assert_eq!(registry.cross_dc_sync.local_dc, "dc1");
        assert!(registry.crdt_entries.is_empty());
    }

    #[test]
    fn test_distributed_registry_apply_crdt_operation() {
        let nodes = vec!["node1".to_string()];
        let mut registry = DistributedRegistry::new("node1".to_string(), nodes, "dc1".to_string());

        let op = CrdtOperation::AddTag {
            tag: "tax".to_string(),
            timestamp: 100,
        };

        registry.apply_crdt_operation("statute-1", op);

        let entry = registry.crdt_entries.get("statute-1").unwrap();
        assert!(entry.tags.contains_key("tax"));
    }

    #[test]
    fn test_distributed_registry_merge_crdt_entry() {
        let nodes = vec!["node1".to_string()];
        let mut registry = DistributedRegistry::new("node1".to_string(), nodes, "dc1".to_string());

        let mut remote_entry = CrdtStatuteEntry::new("statute-1".to_string());
        remote_entry.apply_operation(CrdtOperation::AddTag {
            tag: "finance".to_string(),
            timestamp: 150,
        });

        registry.merge_crdt_entry(remote_entry);

        let entry = registry.crdt_entries.get("statute-1").unwrap();
        assert!(entry.tags.contains_key("finance"));
    }

    #[test]
    fn test_distributed_registry_start_election() {
        let nodes = vec!["node1".to_string(), "node2".to_string()];
        let mut registry = DistributedRegistry::new("node1".to_string(), nodes, "dc1".to_string());

        let request = registry.start_election();

        assert_eq!(request.term, 1);
        assert_eq!(request.candidate_id, "node1");
        assert_eq!(registry.raft_state.role, RaftRole::Candidate);
    }

    #[test]
    fn test_distributed_registry_handle_vote_request() {
        let nodes = vec!["node1".to_string(), "node2".to_string()];
        let mut registry = DistributedRegistry::new("node1".to_string(), nodes, "dc1".to_string());

        let request = RequestVoteRequest {
            term: 1,
            candidate_id: "node2".to_string(),
            last_log_index: 0,
            last_log_term: 0,
        };

        let response = registry.handle_vote_request(request);

        assert!(response.vote_granted);
        assert_eq!(response.term, 1);
        assert_eq!(registry.raft_state.voted_for, Some("node2".to_string()));
    }

    #[test]
    fn test_distributed_registry_become_leader() {
        let nodes = vec!["node1".to_string(), "node2".to_string()];
        let mut registry = DistributedRegistry::new("node1".to_string(), nodes, "dc1".to_string());

        registry.start_election();
        registry.become_leader();

        assert_eq!(registry.raft_state.role, RaftRole::Leader);
        assert!(registry.is_leader());
        assert_eq!(registry.current_leader(), Some(&"node1".to_string()));
    }

    #[test]
    fn test_distributed_registry_append_entries() {
        let nodes = vec!["node1".to_string(), "node2".to_string()];
        let mut registry = DistributedRegistry::new("node1".to_string(), nodes, "dc1".to_string());

        registry.start_election();
        registry.become_leader();

        let request = registry.create_append_entries("node2");
        assert!(request.is_some());

        let req = request.unwrap();
        assert_eq!(req.leader_id, "node1");
        assert_eq!(req.term, 1);
    }

    #[test]
    fn test_distributed_registry_add_remote_datacenter() {
        let nodes = vec!["node1".to_string()];
        let mut registry = DistributedRegistry::new("node1".to_string(), nodes, "dc1".to_string());

        registry.add_remote_datacenter("dc2".to_string(), "https://dc2.example.com".to_string());

        assert_eq!(registry.cross_dc_sync.remote_dcs.len(), 1);
    }

    #[test]
    fn test_crdt_metadata_operations() {
        let mut entry = CrdtStatuteEntry::new("statute-1".to_string());

        // Add metadata
        entry.apply_operation(CrdtOperation::AddMetadata {
            key: "author".to_string(),
            value: "John Doe".to_string(),
            timestamp: 100,
        });

        let metadata = entry.active_metadata();
        assert_eq!(metadata.get("author"), Some(&"John Doe".to_string()));

        // Remove metadata
        entry.apply_operation(CrdtOperation::RemoveMetadata {
            key: "author".to_string(),
            timestamp: 200,
        });

        let metadata = entry.active_metadata();
        assert_eq!(metadata.len(), 0);
    }
}
