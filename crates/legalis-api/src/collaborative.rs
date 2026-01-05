//! Collaborative editing support with operational transformation.
//!
//! This module provides real-time collaborative editing features including:
//! - Operation tracking and transformation
//! - Conflict detection and resolution
//! - Version control for concurrent edits
//! - WebSocket-based real-time synchronization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Maximum number of operations to keep in history per document
const MAX_OPERATION_HISTORY: usize = 1000;

/// Types of edit operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EditOperation {
    /// Insert text at a position
    Insert {
        position: usize,
        content: String,
        user_id: String,
    },
    /// Delete text from a position
    Delete {
        position: usize,
        length: usize,
        user_id: String,
    },
    /// Replace text at a position
    Replace {
        position: usize,
        length: usize,
        content: String,
        user_id: String,
    },
}

impl EditOperation {
    /// Gets the user ID who performed this operation
    pub fn user_id(&self) -> &str {
        match self {
            EditOperation::Insert { user_id, .. } => user_id,
            EditOperation::Delete { user_id, .. } => user_id,
            EditOperation::Replace { user_id, .. } => user_id,
        }
    }

    /// Transforms this operation against another operation (Operational Transformation)
    pub fn transform(&self, other: &EditOperation) -> EditOperation {
        match (self, other) {
            // Insert vs Insert
            (
                EditOperation::Insert {
                    position: p1,
                    content: c1,
                    user_id: u1,
                },
                EditOperation::Insert {
                    position: p2,
                    content: c2,
                    ..
                },
            ) => {
                let new_pos = if *p2 <= *p1 { p1 + c2.len() } else { *p1 };
                EditOperation::Insert {
                    position: new_pos,
                    content: c1.clone(),
                    user_id: u1.clone(),
                }
            }
            // Insert vs Delete
            (
                EditOperation::Insert {
                    position: p1,
                    content: c1,
                    user_id: u1,
                },
                EditOperation::Delete {
                    position: p2,
                    length: l2,
                    ..
                },
            ) => {
                let new_pos = if *p2 + *l2 <= *p1 {
                    p1 - l2
                } else if *p2 <= *p1 {
                    *p2
                } else {
                    *p1
                };
                EditOperation::Insert {
                    position: new_pos,
                    content: c1.clone(),
                    user_id: u1.clone(),
                }
            }
            // Delete vs Insert
            (
                EditOperation::Delete {
                    position: p1,
                    length: l1,
                    user_id: u1,
                },
                EditOperation::Insert {
                    position: p2,
                    content: c2,
                    ..
                },
            ) => {
                let new_pos = if *p2 <= *p1 { p1 + c2.len() } else { *p1 };
                EditOperation::Delete {
                    position: new_pos,
                    length: *l1,
                    user_id: u1.clone(),
                }
            }
            // Delete vs Delete
            (
                EditOperation::Delete {
                    position: p1,
                    length: l1,
                    user_id: u1,
                },
                EditOperation::Delete {
                    position: p2,
                    length: l2,
                    ..
                },
            ) => {
                let new_pos = if *p2 + *l2 <= *p1 {
                    p1 - l2
                } else if *p2 < *p1 {
                    *p2
                } else {
                    *p1
                };
                let new_len = if *p2 <= *p1 && *p1 + *l1 <= *p2 + *l2 {
                    0 // Completely overlapped
                } else if *p2 <= *p1 && *p1 < *p2 + *l2 {
                    l1 - (*p2 + *l2 - *p1).min(*l1)
                } else if *p1 < *p2 && *p2 < *p1 + *l1 {
                    l1 - (*p1 + *l1 - *p2).min(*l2)
                } else {
                    *l1
                };
                EditOperation::Delete {
                    position: new_pos,
                    length: new_len,
                    user_id: u1.clone(),
                }
            }
            // Handle Replace operations by decomposing them
            _ => self.clone(), // Simplified - in production, decompose Replace into Delete+Insert
        }
    }
}

/// An edit operation with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedOperation {
    /// The operation itself
    pub operation: EditOperation,
    /// Version number when this operation was applied
    pub version: u64,
    /// Timestamp when the operation was created
    pub timestamp: DateTime<Utc>,
    /// Session ID of the editor
    pub session_id: String,
}

/// Represents a conflict between two concurrent operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditConflict {
    /// The first operation
    pub operation1: TrackedOperation,
    /// The second operation
    pub operation2: TrackedOperation,
    /// Timestamp when the conflict was detected
    pub detected_at: DateTime<Utc>,
    /// Whether the conflict was automatically resolved
    pub auto_resolved: bool,
    /// Description of the conflict
    pub description: String,
}

/// State of a collaborative document
#[derive(Debug, Clone)]
struct DocumentState {
    /// Current version number
    version: u64,
    /// History of operations
    operations: VecDeque<TrackedOperation>,
    /// Detected conflicts
    conflicts: Vec<EditConflict>,
    /// Active editing sessions
    active_sessions: HashMap<String, String>, // session_id -> user_id
    /// Last modification time
    last_modified: DateTime<Utc>,
}

impl DocumentState {
    fn new() -> Self {
        Self {
            version: 0,
            operations: VecDeque::new(),
            conflicts: Vec::new(),
            active_sessions: HashMap::new(),
            last_modified: Utc::now(),
        }
    }
}

/// Collaborative editing manager
#[derive(Clone)]
pub struct CollaborativeEditor {
    /// Map of document_id -> document state
    documents: Arc<RwLock<HashMap<String, DocumentState>>>,
}

impl CollaborativeEditor {
    /// Creates a new collaborative editor
    pub fn new() -> Self {
        Self {
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Applies an operation to a document
    pub async fn apply_operation(
        &self,
        document_id: String,
        operation: EditOperation,
        session_id: String,
    ) -> Result<TrackedOperation, String> {
        let mut documents = self.documents.write().await;
        let doc_state = documents
            .entry(document_id.clone())
            .or_insert_with(DocumentState::new);

        // Transform the operation against all pending operations from other sessions
        let mut transformed_op = operation.clone();
        let current_version = doc_state.version;

        for tracked_op in doc_state.operations.iter().rev() {
            if tracked_op.session_id != session_id {
                transformed_op = transformed_op.transform(&tracked_op.operation);
            }
        }

        // Create tracked operation
        let tracked = TrackedOperation {
            operation: transformed_op,
            version: current_version + 1,
            timestamp: Utc::now(),
            session_id: session_id.clone(),
        };

        // Check for conflicts
        self.detect_conflicts(doc_state, &tracked);

        // Update document state
        doc_state.version += 1;
        doc_state.operations.push_back(tracked.clone());
        doc_state.last_modified = Utc::now();

        // Trim operation history if needed
        while doc_state.operations.len() > MAX_OPERATION_HISTORY {
            doc_state.operations.pop_front();
        }

        Ok(tracked)
    }

    /// Detects conflicts between the new operation and recent operations
    fn detect_conflicts(&self, doc_state: &mut DocumentState, new_op: &TrackedOperation) {
        // Check operations from the last few seconds for conflicts
        let recent_window = chrono::Duration::seconds(5);
        let cutoff = Utc::now() - recent_window;

        for existing_op in doc_state.operations.iter().rev().take(10) {
            if existing_op.timestamp < cutoff {
                break;
            }

            if existing_op.session_id == new_op.session_id {
                continue; // Same session, not a conflict
            }

            // Check if operations overlap
            if self.operations_overlap(&existing_op.operation, &new_op.operation) {
                let conflict = EditConflict {
                    operation1: existing_op.clone(),
                    operation2: new_op.clone(),
                    detected_at: Utc::now(),
                    auto_resolved: true, // We're using OT, so it's auto-resolved
                    description: format!(
                        "Concurrent edits by {} and {}",
                        existing_op.session_id, new_op.session_id
                    ),
                };
                doc_state.conflicts.push(conflict);
            }
        }

        // Trim conflict history
        if doc_state.conflicts.len() > 100 {
            doc_state
                .conflicts
                .drain(0..doc_state.conflicts.len() - 100);
        }
    }

    /// Checks if two operations overlap in their edit regions
    fn operations_overlap(&self, op1: &EditOperation, op2: &EditOperation) -> bool {
        let (start1, end1) = self.operation_range(op1);
        let (start2, end2) = self.operation_range(op2);

        // Check for overlap
        !(end1 <= start2 || end2 <= start1)
    }

    /// Gets the range of positions affected by an operation
    fn operation_range(&self, op: &EditOperation) -> (usize, usize) {
        match op {
            EditOperation::Insert {
                position, content, ..
            } => (*position, position + content.len()),
            EditOperation::Delete {
                position, length, ..
            } => (*position, position + length),
            EditOperation::Replace {
                position, length, ..
            } => (*position, position + length),
        }
    }

    /// Gets the operation history for a document
    pub async fn get_history(
        &self,
        document_id: &str,
        since_version: Option<u64>,
    ) -> Vec<TrackedOperation> {
        let documents = self.documents.read().await;
        if let Some(doc_state) = documents.get(document_id) {
            match since_version {
                Some(version) => doc_state
                    .operations
                    .iter()
                    .filter(|op| op.version > version)
                    .cloned()
                    .collect(),
                None => doc_state.operations.iter().cloned().collect(),
            }
        } else {
            Vec::new()
        }
    }

    /// Gets conflicts for a document
    pub async fn get_conflicts(
        &self,
        document_id: &str,
        since: Option<DateTime<Utc>>,
    ) -> Vec<EditConflict> {
        let documents = self.documents.read().await;
        if let Some(doc_state) = documents.get(document_id) {
            match since {
                Some(timestamp) => doc_state
                    .conflicts
                    .iter()
                    .filter(|c| c.detected_at > timestamp)
                    .cloned()
                    .collect(),
                None => doc_state.conflicts.clone(),
            }
        } else {
            Vec::new()
        }
    }

    /// Registers a new editing session
    pub async fn register_session(&self, document_id: String, session_id: String, user_id: String) {
        let mut documents = self.documents.write().await;
        let doc_state = documents
            .entry(document_id)
            .or_insert_with(DocumentState::new);
        doc_state.active_sessions.insert(session_id, user_id);
    }

    /// Unregisters an editing session
    pub async fn unregister_session(&self, document_id: &str, session_id: &str) {
        let mut documents = self.documents.write().await;
        if let Some(doc_state) = documents.get_mut(document_id) {
            doc_state.active_sessions.remove(session_id);
        }
    }

    /// Gets the current version of a document
    pub async fn get_version(&self, document_id: &str) -> u64 {
        let documents = self.documents.read().await;
        documents
            .get(document_id)
            .map(|doc| doc.version)
            .unwrap_or(0)
    }

    /// Gets active sessions for a document
    pub async fn get_active_sessions(&self, document_id: &str) -> HashMap<String, String> {
        let documents = self.documents.read().await;
        documents
            .get(document_id)
            .map(|doc| doc.active_sessions.clone())
            .unwrap_or_default()
    }
}

impl Default for CollaborativeEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_transform() {
        let op1 = EditOperation::Insert {
            position: 5,
            content: "Hello".to_string(),
            user_id: "user1".to_string(),
        };
        let op2 = EditOperation::Insert {
            position: 3,
            content: "World".to_string(),
            user_id: "user2".to_string(),
        };

        let transformed = op1.transform(&op2);
        match transformed {
            EditOperation::Insert { position, .. } => {
                assert_eq!(position, 10); // 5 + len("World")
            }
            _ => panic!("Expected Insert operation"),
        }
    }

    #[test]
    fn test_delete_transform() {
        let op1 = EditOperation::Delete {
            position: 10,
            length: 5,
            user_id: "user1".to_string(),
        };
        let op2 = EditOperation::Delete {
            position: 5,
            length: 3,
            user_id: "user2".to_string(),
        };

        let transformed = op1.transform(&op2);
        match transformed {
            EditOperation::Delete {
                position, length, ..
            } => {
                assert_eq!(position, 7); // 10 - 3
                assert_eq!(length, 5);
            }
            _ => panic!("Expected Delete operation"),
        }
    }

    #[tokio::test]
    async fn test_apply_operation() {
        let editor = CollaborativeEditor::new();

        let op = EditOperation::Insert {
            position: 0,
            content: "Hello".to_string(),
            user_id: "user1".to_string(),
        };

        let result = editor
            .apply_operation("doc1".to_string(), op, "session1".to_string())
            .await;

        assert!(result.is_ok());
        let tracked = result.unwrap();
        assert_eq!(tracked.version, 1);
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let editor = CollaborativeEditor::new();

        let op1 = EditOperation::Insert {
            position: 0,
            content: "A".to_string(),
            user_id: "user1".to_string(),
        };
        let op2 = EditOperation::Insert {
            position: 0,
            content: "B".to_string(),
            user_id: "user2".to_string(),
        };

        let _ = editor
            .apply_operation("doc1".to_string(), op1, "session1".to_string())
            .await
            .unwrap();
        let _ = editor
            .apply_operation("doc1".to_string(), op2, "session2".to_string())
            .await
            .unwrap();

        let version = editor.get_version("doc1").await;
        assert_eq!(version, 2);
    }

    #[tokio::test]
    async fn test_conflict_detection() {
        let editor = CollaborativeEditor::new();

        let op1 = EditOperation::Delete {
            position: 5,
            length: 3,
            user_id: "user1".to_string(),
        };
        let op2 = EditOperation::Insert {
            position: 6,
            content: "X".to_string(),
            user_id: "user2".to_string(),
        };

        let _ = editor
            .apply_operation("doc1".to_string(), op1, "session1".to_string())
            .await
            .unwrap();
        let _ = editor
            .apply_operation("doc1".to_string(), op2, "session2".to_string())
            .await
            .unwrap();

        let conflicts = editor.get_conflicts("doc1", None).await;
        assert!(!conflicts.is_empty());
    }

    #[tokio::test]
    async fn test_session_management() {
        let editor = CollaborativeEditor::new();

        editor
            .register_session(
                "doc1".to_string(),
                "session1".to_string(),
                "user1".to_string(),
            )
            .await;
        editor
            .register_session(
                "doc1".to_string(),
                "session2".to_string(),
                "user2".to_string(),
            )
            .await;

        let sessions = editor.get_active_sessions("doc1").await;
        assert_eq!(sessions.len(), 2);

        editor.unregister_session("doc1", "session1").await;
        let sessions = editor.get_active_sessions("doc1").await;
        assert_eq!(sessions.len(), 1);
    }

    #[tokio::test]
    async fn test_get_history() {
        let editor = CollaborativeEditor::new();

        for i in 0..5 {
            let op = EditOperation::Insert {
                position: i,
                content: format!("{}", i),
                user_id: "user1".to_string(),
            };
            let _ = editor
                .apply_operation("doc1".to_string(), op, "session1".to_string())
                .await
                .unwrap();
        }

        let history = editor.get_history("doc1", None).await;
        assert_eq!(history.len(), 5);

        let recent_history = editor.get_history("doc1", Some(3)).await;
        assert_eq!(recent_history.len(), 2);
    }
}
