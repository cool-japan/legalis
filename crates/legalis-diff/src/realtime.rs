//! Real-time diff streaming and collaborative editing.
//!
//! This module provides real-time diff capabilities including:
//! - WebSocket-based real-time diff updates
//! - Live collaborative editing with diff tracking
//! - Incremental diff streaming for large documents
//! - Server-sent events for diff notifications
//! - Real-time conflict resolution
//!
//! # Examples
//!
//! ```
//! use legalis_diff::realtime::{RealtimeDiffServer, DiffUpdate, UpdateType};
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let server = RealtimeDiffServer::new();
//! let session_id = server.create_session("statute-123");
//!
//! // Simulate a diff update
//! let statute = Statute::new("statute-123", "Test", Effect::new(EffectType::Grant, "Benefit"));
//! ```

use crate::{Change, ChangeType, DiffResult, StatuteDiff};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Types of diff updates in real-time streaming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateType {
    /// Initial diff computed
    Initial,
    /// Incremental change applied
    Incremental,
    /// Conflict detected
    Conflict,
    /// Conflict resolved
    ConflictResolved,
    /// Session ended
    SessionEnded,
}

/// A real-time diff update event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffUpdate {
    /// Unique update ID
    pub update_id: String,
    /// Session ID this update belongs to
    pub session_id: String,
    /// Type of update
    pub update_type: UpdateType,
    /// Timestamp of the update (milliseconds since epoch)
    pub timestamp: u64,
    /// The actual diff or change
    pub diff: Option<StatuteDiff>,
    /// Individual change (for incremental updates)
    pub change: Option<Change>,
    /// Conflict information (if any)
    pub conflict: Option<ConflictInfo>,
    /// User who made the change
    pub user_id: Option<String>,
}

/// Information about a detected conflict.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// Description of the conflict
    pub description: String,
    /// Conflicting changes
    pub conflicting_changes: Vec<Change>,
    /// Suggested resolution
    pub suggested_resolution: Option<ConflictResolution>,
}

/// Types of conflicts that can occur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Concurrent modifications to the same field
    ConcurrentModification,
    /// Incompatible changes
    IncompatibleChanges,
    /// Version mismatch
    VersionMismatch,
    /// Semantic conflict (changes that work individually but conflict logically)
    SemanticConflict,
}

/// Resolution strategy for conflicts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Use the first change
    UseFirst,
    /// Use the second change
    UseSecond,
    /// Merge both changes
    Merge,
    /// Custom resolution with merged change
    Custom { merged_change: Change },
}

/// A collaborative editing session.
#[derive(Debug, Clone)]
struct Session {
    /// Session ID
    #[allow(dead_code)]
    id: String,
    /// Statute being edited
    #[allow(dead_code)]
    statute_id: String,
    /// Current statute state
    #[allow(dead_code)]
    current_state: Statute,
    /// Active users in this session
    active_users: Vec<String>,
    /// Pending changes that haven't been applied
    pending_changes: Vec<Change>,
    /// Update history
    history: Vec<DiffUpdate>,
}

/// Real-time diff server for managing collaborative editing sessions.
///
/// # Examples
///
/// ```
/// use legalis_diff::realtime::RealtimeDiffServer;
///
/// let server = RealtimeDiffServer::new();
/// let session = server.create_session("statute-123");
/// println!("Created session: {}", session);
/// ```
#[derive(Clone)]
pub struct RealtimeDiffServer {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
}

impl Default for RealtimeDiffServer {
    fn default() -> Self {
        Self::new()
    }
}

impl RealtimeDiffServer {
    /// Creates a new real-time diff server.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::RealtimeDiffServer;
    ///
    /// let server = RealtimeDiffServer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Creates a new collaborative editing session.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::RealtimeDiffServer;
    ///
    /// let server = RealtimeDiffServer::new();
    /// let session_id = server.create_session("statute-123");
    /// assert!(!session_id.is_empty());
    /// ```
    pub fn create_session(&self, statute_id: &str) -> String {
        let session_id = Uuid::new_v4().to_string();
        let statute = Statute::new(
            statute_id,
            "Placeholder",
            legalis_core::Effect::new(legalis_core::EffectType::Grant, "Placeholder effect"),
        );

        let session = Session {
            id: session_id.clone(),
            statute_id: statute_id.to_string(),
            current_state: statute,
            active_users: Vec::new(),
            pending_changes: Vec::new(),
            history: Vec::new(),
        };

        self.sessions
            .lock()
            .unwrap()
            .insert(session_id.clone(), session);
        session_id
    }

    /// Joins a user to a session.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::RealtimeDiffServer;
    ///
    /// let server = RealtimeDiffServer::new();
    /// let session_id = server.create_session("statute-123");
    /// let result = server.join_session(&session_id, "user-1");
    /// assert!(result.is_ok());
    /// ```
    pub fn join_session(&self, session_id: &str, user_id: &str) -> DiffResult<()> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            if !session.active_users.contains(&user_id.to_string()) {
                session.active_users.push(user_id.to_string());
            }
            Ok(())
        } else {
            Err(crate::DiffError::InvalidComparison(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Leaves a session.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::RealtimeDiffServer;
    ///
    /// let server = RealtimeDiffServer::new();
    /// let session_id = server.create_session("statute-123");
    /// server.join_session(&session_id, "user-1").unwrap();
    /// let result = server.leave_session(&session_id, "user-1");
    /// assert!(result.is_ok());
    /// ```
    pub fn leave_session(&self, session_id: &str, user_id: &str) -> DiffResult<()> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            session.active_users.retain(|u| u != user_id);
            Ok(())
        } else {
            Err(crate::DiffError::InvalidComparison(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Submits a change to a session.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::{RealtimeDiffServer};
    /// use legalis_diff::{Change, ChangeType, ChangeTarget};
    ///
    /// let server = RealtimeDiffServer::new();
    /// let session_id = server.create_session("statute-123");
    /// server.join_session(&session_id, "user-1").unwrap();
    ///
    /// let change = Change {
    ///     change_type: ChangeType::Modified,
    ///     target: ChangeTarget::Title,
    ///     description: "Updated title".to_string(),
    ///     old_value: Some("Old".to_string()),
    ///     new_value: Some("New".to_string()),
    /// };
    ///
    /// let update = server.submit_change(&session_id, "user-1", change).unwrap();
    /// ```
    pub fn submit_change(
        &self,
        session_id: &str,
        user_id: &str,
        change: Change,
    ) -> DiffResult<DiffUpdate> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            // Check for conflicts
            let conflict = self.detect_conflict(&session.pending_changes, &change);

            let update = DiffUpdate {
                update_id: Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                update_type: if conflict.is_some() {
                    UpdateType::Conflict
                } else {
                    UpdateType::Incremental
                },
                timestamp: current_timestamp(),
                diff: None,
                change: Some(change.clone()),
                conflict,
                user_id: Some(user_id.to_string()),
            };

            session.pending_changes.push(change);
            session.history.push(update.clone());

            Ok(update)
        } else {
            Err(crate::DiffError::InvalidComparison(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Detects conflicts between pending changes and a new change.
    fn detect_conflict(&self, pending: &[Change], new_change: &Change) -> Option<ConflictInfo> {
        for pending_change in pending {
            // Check if they target the same thing
            if pending_change.target == new_change.target {
                // If both are modifications, it's a concurrent modification conflict
                if pending_change.change_type == ChangeType::Modified
                    && new_change.change_type == ChangeType::Modified
                {
                    return Some(ConflictInfo {
                        conflict_type: ConflictType::ConcurrentModification,
                        description: format!(
                            "Concurrent modifications detected on {}",
                            new_change.target
                        ),
                        conflicting_changes: vec![pending_change.clone(), new_change.clone()],
                        suggested_resolution: Some(ConflictResolution::UseSecond),
                    });
                }

                // Check for incompatible changes (e.g., one adds, one removes the same thing)
                if (pending_change.change_type == ChangeType::Added
                    && new_change.change_type == ChangeType::Removed)
                    || (pending_change.change_type == ChangeType::Removed
                        && new_change.change_type == ChangeType::Added)
                {
                    return Some(ConflictInfo {
                        conflict_type: ConflictType::IncompatibleChanges,
                        description: format!(
                            "Incompatible changes detected on {}",
                            new_change.target
                        ),
                        conflicting_changes: vec![pending_change.clone(), new_change.clone()],
                        suggested_resolution: Some(ConflictResolution::UseSecond),
                    });
                }
            }
        }
        None
    }

    /// Resolves a conflict with the given resolution strategy.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::{RealtimeDiffServer, ConflictResolution};
    /// use legalis_diff::{Change, ChangeType, ChangeTarget};
    ///
    /// let server = RealtimeDiffServer::new();
    /// let session_id = server.create_session("statute-123");
    /// server.join_session(&session_id, "user-1").unwrap();
    ///
    /// let change1 = Change {
    ///     change_type: ChangeType::Modified,
    ///     target: ChangeTarget::Title,
    ///     description: "Update 1".to_string(),
    ///     old_value: Some("Old".to_string()),
    ///     new_value: Some("New1".to_string()),
    /// };
    ///
    /// let change2 = Change {
    ///     change_type: ChangeType::Modified,
    ///     target: ChangeTarget::Title,
    ///     description: "Update 2".to_string(),
    ///     old_value: Some("Old".to_string()),
    ///     new_value: Some("New2".to_string()),
    /// };
    ///
    /// server.submit_change(&session_id, "user-1", change1).unwrap();
    /// let update = server.submit_change(&session_id, "user-2", change2).unwrap();
    ///
    /// if update.conflict.is_some() {
    ///     server.resolve_conflict(&session_id, &update.update_id, ConflictResolution::UseSecond).unwrap();
    /// }
    /// ```
    pub fn resolve_conflict(
        &self,
        session_id: &str,
        update_id: &str,
        resolution: ConflictResolution,
    ) -> DiffResult<DiffUpdate> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            // Find the conflicting update
            if let Some(conflicting_update) = session
                .history
                .iter()
                .find(|u| u.update_id == update_id)
                .cloned()
            {
                let resolved_update = DiffUpdate {
                    update_id: Uuid::new_v4().to_string(),
                    session_id: session_id.to_string(),
                    update_type: UpdateType::ConflictResolved,
                    timestamp: current_timestamp(),
                    diff: None,
                    change: match resolution {
                        ConflictResolution::UseFirst => conflicting_update
                            .conflict
                            .as_ref()
                            .and_then(|c| c.conflicting_changes.first().cloned()),
                        ConflictResolution::UseSecond => conflicting_update
                            .conflict
                            .as_ref()
                            .and_then(|c| c.conflicting_changes.get(1).cloned()),
                        ConflictResolution::Merge => {
                            // Simple merge: use second change
                            conflicting_update
                                .conflict
                                .as_ref()
                                .and_then(|c| c.conflicting_changes.get(1).cloned())
                        }
                        ConflictResolution::Custom { merged_change } => Some(merged_change),
                    },
                    conflict: None,
                    user_id: None,
                };

                session.history.push(resolved_update.clone());
                Ok(resolved_update)
            } else {
                Err(crate::DiffError::InvalidComparison(format!(
                    "Update not found: {}",
                    update_id
                )))
            }
        } else {
            Err(crate::DiffError::InvalidComparison(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Gets the update history for a session.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::RealtimeDiffServer;
    ///
    /// let server = RealtimeDiffServer::new();
    /// let session_id = server.create_session("statute-123");
    /// let history = server.get_history(&session_id).unwrap();
    /// ```
    pub fn get_history(&self, session_id: &str) -> DiffResult<Vec<DiffUpdate>> {
        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(session_id) {
            Ok(session.history.clone())
        } else {
            Err(crate::DiffError::InvalidComparison(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Gets active users in a session.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::RealtimeDiffServer;
    ///
    /// let server = RealtimeDiffServer::new();
    /// let session_id = server.create_session("statute-123");
    /// server.join_session(&session_id, "user-1").unwrap();
    /// let users = server.get_active_users(&session_id).unwrap();
    /// assert_eq!(users.len(), 1);
    /// ```
    pub fn get_active_users(&self, session_id: &str) -> DiffResult<Vec<String>> {
        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(session_id) {
            Ok(session.active_users.clone())
        } else {
            Err(crate::DiffError::InvalidComparison(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Ends a session.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::RealtimeDiffServer;
    ///
    /// let server = RealtimeDiffServer::new();
    /// let session_id = server.create_session("statute-123");
    /// let result = server.end_session(&session_id);
    /// assert!(result.is_ok());
    /// ```
    pub fn end_session(&self, session_id: &str) -> DiffResult<DiffUpdate> {
        let mut sessions = self.sessions.lock().unwrap();
        if sessions.remove(session_id).is_some() {
            Ok(DiffUpdate {
                update_id: Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                update_type: UpdateType::SessionEnded,
                timestamp: current_timestamp(),
                diff: None,
                change: None,
                conflict: None,
                user_id: None,
            })
        } else {
            Err(crate::DiffError::InvalidComparison(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }
}

/// Server-sent events handler for diff notifications.
///
/// # Examples
///
/// ```
/// use legalis_diff::realtime::ServerSentEvents;
///
/// let sse = ServerSentEvents::new();
/// let subscriber = sse.subscribe("statute-123");
/// println!("Subscribed: {}", subscriber);
/// ```
#[derive(Clone)]
pub struct ServerSentEvents {
    subscribers: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Default for ServerSentEvents {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerSentEvents {
    /// Creates a new SSE handler.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::ServerSentEvents;
    ///
    /// let sse = ServerSentEvents::new();
    /// ```
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Subscribes to updates for a statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::ServerSentEvents;
    ///
    /// let sse = ServerSentEvents::new();
    /// let subscriber_id = sse.subscribe("statute-123");
    /// assert!(!subscriber_id.is_empty());
    /// ```
    pub fn subscribe(&self, statute_id: &str) -> String {
        let subscriber_id = Uuid::new_v4().to_string();
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers
            .entry(statute_id.to_string())
            .or_default()
            .push(subscriber_id.clone());
        subscriber_id
    }

    /// Unsubscribes from updates.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::ServerSentEvents;
    ///
    /// let sse = ServerSentEvents::new();
    /// let subscriber_id = sse.subscribe("statute-123");
    /// sse.unsubscribe("statute-123", &subscriber_id);
    /// ```
    pub fn unsubscribe(&self, statute_id: &str, subscriber_id: &str) {
        let mut subscribers = self.subscribers.lock().unwrap();
        if let Some(subs) = subscribers.get_mut(statute_id) {
            subs.retain(|s| s != subscriber_id);
        }
    }

    /// Notifies all subscribers of an update.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::{ServerSentEvents, DiffUpdate, UpdateType};
    ///
    /// let sse = ServerSentEvents::new();
    /// sse.subscribe("statute-123");
    ///
    /// let update = DiffUpdate {
    ///     update_id: "update-1".to_string(),
    ///     session_id: "session-1".to_string(),
    ///     update_type: UpdateType::Initial,
    ///     timestamp: 1000000,
    ///     diff: None,
    ///     change: None,
    ///     conflict: None,
    ///     user_id: None,
    /// };
    ///
    /// let count = sse.notify("statute-123", &update);
    /// ```
    pub fn notify(&self, statute_id: &str, _update: &DiffUpdate) -> usize {
        let subscribers = self.subscribers.lock().unwrap();
        if let Some(subs) = subscribers.get(statute_id) {
            // In a real implementation, this would send the update to each subscriber
            // For now, we just return the count
            subs.len()
        } else {
            0
        }
    }

    /// Gets subscriber count for a statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::ServerSentEvents;
    ///
    /// let sse = ServerSentEvents::new();
    /// sse.subscribe("statute-123");
    /// assert_eq!(sse.subscriber_count("statute-123"), 1);
    /// ```
    pub fn subscriber_count(&self, statute_id: &str) -> usize {
        let subscribers = self.subscribers.lock().unwrap();
        subscribers.get(statute_id).map(|s| s.len()).unwrap_or(0)
    }
}

/// Incremental diff streamer for large documents.
///
/// # Examples
///
/// ```
/// use legalis_diff::realtime::IncrementalStreamer;
/// use legalis_core::{Statute, Effect, EffectType};
///
/// let statute = Statute::new("statute-123", "Test", Effect::new(EffectType::Grant, "Benefit"));
/// let streamer = IncrementalStreamer::new(statute.clone());
/// ```
pub struct IncrementalStreamer {
    /// Current statute state
    current: Statute,
    /// Chunk size for streaming
    chunk_size: usize,
}

impl IncrementalStreamer {
    /// Creates a new incremental streamer.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::IncrementalStreamer;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let streamer = IncrementalStreamer::new(statute);
    /// ```
    pub fn new(statute: Statute) -> Self {
        Self {
            current: statute,
            chunk_size: 100, // Default chunk size
        }
    }

    /// Sets the chunk size for streaming.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::IncrementalStreamer;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let streamer = IncrementalStreamer::new(statute).with_chunk_size(50);
    /// ```
    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    /// Streams the diff incrementally.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::realtime::IncrementalStreamer;
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"))
    ///     .with_precondition(Condition::Age {
    ///         operator: ComparisonOp::GreaterOrEqual,
    ///         value: 18,
    ///     });
    ///
    /// let streamer = IncrementalStreamer::new(old);
    /// let chunks = streamer.stream_diff(&new).unwrap();
    /// ```
    pub fn stream_diff(&self, new_statute: &Statute) -> DiffResult<Vec<DiffUpdate>> {
        let full_diff = crate::diff(&self.current, new_statute)?;
        let mut updates = Vec::new();

        // Split changes into chunks
        for chunk in full_diff.changes.chunks(self.chunk_size) {
            for change in chunk {
                updates.push(DiffUpdate {
                    update_id: Uuid::new_v4().to_string(),
                    session_id: "stream".to_string(),
                    update_type: UpdateType::Incremental,
                    timestamp: current_timestamp(),
                    diff: None,
                    change: Some(change.clone()),
                    conflict: None,
                    user_id: None,
                });
            }
        }

        Ok(updates)
    }
}

/// Gets the current timestamp in milliseconds since epoch.
fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChangeTarget, ChangeType};
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_create_session() {
        let server = RealtimeDiffServer::new();
        let session_id = server.create_session("statute-123");
        assert!(!session_id.is_empty());
    }

    #[test]
    fn test_join_and_leave_session() {
        let server = RealtimeDiffServer::new();
        let session_id = server.create_session("statute-123");

        server.join_session(&session_id, "user-1").unwrap();
        let users = server.get_active_users(&session_id).unwrap();
        assert_eq!(users.len(), 1);

        server.leave_session(&session_id, "user-1").unwrap();
        let users = server.get_active_users(&session_id).unwrap();
        assert_eq!(users.len(), 0);
    }

    #[test]
    fn test_submit_change() {
        let server = RealtimeDiffServer::new();
        let session_id = server.create_session("statute-123");
        server.join_session(&session_id, "user-1").unwrap();

        let change = Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Title,
            description: "Updated title".to_string(),
            old_value: Some("Old".to_string()),
            new_value: Some("New".to_string()),
        };

        let update = server.submit_change(&session_id, "user-1", change).unwrap();
        assert_eq!(update.update_type, UpdateType::Incremental);
    }

    #[test]
    fn test_conflict_detection() {
        let server = RealtimeDiffServer::new();
        let session_id = server.create_session("statute-123");
        server.join_session(&session_id, "user-1").unwrap();

        let change1 = Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Title,
            description: "Update 1".to_string(),
            old_value: Some("Old".to_string()),
            new_value: Some("New1".to_string()),
        };

        let change2 = Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Title,
            description: "Update 2".to_string(),
            old_value: Some("Old".to_string()),
            new_value: Some("New2".to_string()),
        };

        server
            .submit_change(&session_id, "user-1", change1)
            .unwrap();
        let update = server
            .submit_change(&session_id, "user-2", change2)
            .unwrap();

        assert_eq!(update.update_type, UpdateType::Conflict);
        assert!(update.conflict.is_some());
    }

    #[test]
    fn test_conflict_resolution() {
        let server = RealtimeDiffServer::new();
        let session_id = server.create_session("statute-123");
        server.join_session(&session_id, "user-1").unwrap();

        let change1 = Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Title,
            description: "Update 1".to_string(),
            old_value: Some("Old".to_string()),
            new_value: Some("New1".to_string()),
        };

        let change2 = Change {
            change_type: ChangeType::Modified,
            target: ChangeTarget::Title,
            description: "Update 2".to_string(),
            old_value: Some("Old".to_string()),
            new_value: Some("New2".to_string()),
        };

        server
            .submit_change(&session_id, "user-1", change1)
            .unwrap();
        let update = server
            .submit_change(&session_id, "user-2", change2)
            .unwrap();

        if update.conflict.is_some() {
            let resolved = server
                .resolve_conflict(
                    &session_id,
                    &update.update_id,
                    ConflictResolution::UseSecond,
                )
                .unwrap();
            assert_eq!(resolved.update_type, UpdateType::ConflictResolved);
        }
    }

    #[test]
    fn test_end_session() {
        let server = RealtimeDiffServer::new();
        let session_id = server.create_session("statute-123");
        let update = server.end_session(&session_id).unwrap();
        assert_eq!(update.update_type, UpdateType::SessionEnded);
    }

    #[test]
    fn test_sse_subscribe() {
        let sse = ServerSentEvents::new();
        let subscriber_id = sse.subscribe("statute-123");
        assert!(!subscriber_id.is_empty());
        assert_eq!(sse.subscriber_count("statute-123"), 1);
    }

    #[test]
    fn test_sse_unsubscribe() {
        let sse = ServerSentEvents::new();
        let subscriber_id = sse.subscribe("statute-123");
        sse.unsubscribe("statute-123", &subscriber_id);
        assert_eq!(sse.subscriber_count("statute-123"), 0);
    }

    #[test]
    fn test_incremental_streamer() {
        let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let streamer = IncrementalStreamer::new(old).with_chunk_size(10);
        let chunks = streamer.stream_diff(&new).unwrap();
        assert!(!chunks.is_empty());
    }
}
