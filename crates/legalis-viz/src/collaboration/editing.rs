//! Collaborative editing sessions.
//!
//! An [`EditSession`] applies an ordered log of [`EditOperation`]s to a
//! [`VizDocument`], tracking a monotonically increasing revision number. Edits
//! can be gated by an [`AccessControlList`] (only users with
//! [`Capability::Edit`] may mutate) and submitted against a base revision for
//! optimistic-concurrency conflict detection
//! ([`EditSession::apply_at`]).
//!
//! ## Runtime boundary
//!
//! The live transport that distributes operations between participants (a
//! WebSocket server) is a deployment concern. This type provides the document
//! model, the operation log, conflict detection and a *client* script
//! ([`EditSession::to_javascript`]) that speaks to a caller-supplied endpoint;
//! it does not open sockets or run a server.

use serde::{Deserialize, Serialize};

use super::{AccessControlList, Capability, VizDocument, VizEdge, VizNode};
use crate::{VizError, VizResult};

/// A single mutation applied to a [`VizDocument`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditOperation {
    /// Insert a new node (fails if the id already exists).
    AddNode(VizNode),
    /// Remove a node and its incident edges (fails if absent).
    RemoveNode {
        /// Id of the node to remove.
        id: String,
    },
    /// Change a node's display label (fails if absent).
    UpdateNodeLabel {
        /// Target node id.
        id: String,
        /// New label.
        label: String,
    },
    /// Set an attribute on a node (fails if the node is absent).
    SetNodeAttribute {
        /// Target node id.
        id: String,
        /// Attribute key.
        key: String,
        /// Attribute value.
        value: String,
    },
    /// Remove an attribute from a node (fails if the node is absent).
    RemoveNodeAttribute {
        /// Target node id.
        id: String,
        /// Attribute key.
        key: String,
    },
    /// Add an edge (fails if an endpoint is missing or the edge already exists).
    AddEdge(VizEdge),
    /// Remove an edge (fails if absent).
    RemoveEdge {
        /// Source node id.
        from: String,
        /// Target node id.
        to: String,
    },
}

impl EditOperation {
    /// Applies the operation to a document, mutating it only on success.
    pub fn apply(&self, document: &mut VizDocument) -> VizResult<()> {
        match self {
            EditOperation::AddNode(node) => {
                if document.has_node(&node.id) {
                    return Err(VizError::InvalidStructure(format!(
                        "add node: id '{}' already exists",
                        node.id
                    )));
                }
                document.nodes.push(node.clone());
            }
            EditOperation::RemoveNode { id } => {
                if !document.remove_node(id) {
                    return Err(VizError::InvalidStructure(format!(
                        "remove node: id '{}' not found",
                        id
                    )));
                }
            }
            EditOperation::UpdateNodeLabel { id, label } => {
                let node = document.node_mut(id).ok_or_else(|| {
                    VizError::InvalidStructure(format!("update label: node '{}' not found", id))
                })?;
                node.label = label.clone();
            }
            EditOperation::SetNodeAttribute { id, key, value } => {
                let node = document.node_mut(id).ok_or_else(|| {
                    VizError::InvalidStructure(format!("set attribute: node '{}' not found", id))
                })?;
                node.attributes.insert(key.clone(), value.clone());
            }
            EditOperation::RemoveNodeAttribute { id, key } => {
                let node = document.node_mut(id).ok_or_else(|| {
                    VizError::InvalidStructure(format!("remove attribute: node '{}' not found", id))
                })?;
                node.attributes.remove(key);
            }
            EditOperation::AddEdge(edge) => {
                if !document.has_node(&edge.from) || !document.has_node(&edge.to) {
                    return Err(VizError::InvalidStructure(format!(
                        "add edge: endpoint missing for {} -> {}",
                        edge.from, edge.to
                    )));
                }
                if document.has_edge(&edge.from, &edge.to) {
                    return Err(VizError::InvalidStructure(format!(
                        "add edge: {} -> {} already exists",
                        edge.from, edge.to
                    )));
                }
                document.edges.push(edge.clone());
            }
            EditOperation::RemoveEdge { from, to } => {
                if !document.remove_edge(from, to) {
                    return Err(VizError::InvalidStructure(format!(
                        "remove edge: {} -> {} not found",
                        from, to
                    )));
                }
            }
        }
        Ok(())
    }
}

/// One recorded edit in a session's log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditEntry {
    /// Revision this edit produced.
    pub revision: u64,
    /// Author who applied the edit.
    pub author: String,
    /// Caller-supplied timestamp.
    pub timestamp: u64,
    /// The operation applied.
    pub operation: EditOperation,
}

/// A revision-tracked collaborative editing session over one document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditSession {
    /// Session identifier.
    pub session_id: String,
    /// The current document state.
    document: VizDocument,
    /// The ordered edit log.
    log: Vec<EditEntry>,
    /// The current revision (number of applied edits).
    revision: u64,
    /// Optional access control gating who may edit.
    acl: Option<AccessControlList>,
}

impl EditSession {
    /// Creates an empty session.
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            document: VizDocument::new(),
            log: Vec::new(),
            revision: 0,
            acl: None,
        }
    }

    /// Sets the initial document (builder style).
    pub fn with_document(mut self, document: VizDocument) -> Self {
        self.document = document;
        self
    }

    /// Attaches an access control list (builder style).
    pub fn with_access_control(mut self, acl: AccessControlList) -> Self {
        self.acl = Some(acl);
        self
    }

    /// The current document.
    pub fn document(&self) -> &VizDocument {
        &self.document
    }

    /// The current revision number.
    pub fn current_revision(&self) -> u64 {
        self.revision
    }

    /// The edit log.
    pub fn log(&self) -> &[EditEntry] {
        &self.log
    }

    /// Whether an edit based on `base_revision` would conflict with the session
    /// (i.e. the session has advanced past it).
    pub fn would_conflict(&self, base_revision: u64) -> bool {
        base_revision != self.revision
    }

    /// Applies an operation, returning the new revision number.
    ///
    /// If an access control list is attached, `author` must hold
    /// [`Capability::Edit`]. On any failure the document and log are unchanged.
    pub fn apply(
        &mut self,
        author: &str,
        timestamp: u64,
        operation: EditOperation,
    ) -> VizResult<u64> {
        if let Some(acl) = &self.acl {
            acl.require(author, Capability::Edit)?;
        }
        operation.apply(&mut self.document)?;
        self.revision += 1;
        self.log.push(EditEntry {
            revision: self.revision,
            author: author.to_string(),
            timestamp,
            operation,
        });
        Ok(self.revision)
    }

    /// Applies an operation only if it is based on the current revision,
    /// otherwise reports a conflict.
    pub fn apply_at(
        &mut self,
        base_revision: u64,
        author: &str,
        timestamp: u64,
        operation: EditOperation,
    ) -> VizResult<u64> {
        if self.would_conflict(base_revision) {
            return Err(VizError::InvalidStructure(format!(
                "edit conflict: based on r{} but session is at r{}",
                base_revision, self.revision
            )));
        }
        self.apply(author, timestamp, operation)
    }

    /// Generates a client-side script that relays operations to a
    /// caller-supplied WebSocket endpoint and applies inbound ones.
    ///
    /// The transport itself (the server) is out of scope — see the module-level
    /// runtime-boundary note.
    pub fn to_javascript(&self, websocket_url: &str) -> String {
        let url = js_string_escape(websocket_url);
        let session = js_string_escape(&self.session_id);
        let mut js = String::new();
        js.push_str("// Collaborative edit client (transport endpoint supplied by caller)\n");
        js.push_str(&format!("(function(){{\n  const url = \"{}\";\n", url));
        js.push_str(&format!("  const sessionId = \"{}\";\n", session));
        js.push_str("  let revision = ");
        js.push_str(&self.revision.to_string());
        js.push_str(";\n");
        js.push_str("  const socket = new WebSocket(url);\n");
        js.push_str("  socket.addEventListener('open', () => socket.send(JSON.stringify({ type: 'join', sessionId, revision })));\n");
        js.push_str("  socket.addEventListener('message', (ev) => {\n");
        js.push_str("    try {\n");
        js.push_str("      const msg = JSON.parse(ev.data);\n");
        js.push_str("      if (msg.type === 'op' && typeof applyRemoteEdit === 'function') { applyRemoteEdit(msg.operation); revision = msg.revision; }\n");
        js.push_str("    } catch (e) { console.error('bad edit message', e); }\n");
        js.push_str("  });\n");
        js.push_str("  window.submitEdit = function(operation) {\n");
        js.push_str("    socket.send(JSON.stringify({ type: 'op', sessionId, baseRevision: revision, operation }));\n");
        js.push_str("  };\n");
        js.push_str("})();\n");
        js
    }

    /// Serializes the session to pretty JSON.
    pub fn to_json(&self) -> VizResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| VizError::ExportError(format!("edit session to JSON: {}", e)))
    }

    /// Parses a session from JSON.
    pub fn from_json(json: &str) -> VizResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VizError::InvalidStructure(format!("edit session from JSON: {}", e)))
    }
}

/// Escapes a string for inclusion inside a double-quoted JavaScript literal.
fn js_string_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '<' => out.push_str("\\u003c"),
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collaboration::Role;

    fn seeded_session() -> EditSession {
        let doc = VizDocument::new().with_node(VizNode::new("a", "A"));
        EditSession::new("s-1").with_document(doc)
    }

    #[test]
    fn apply_sequence_tracks_revision_and_log() {
        let mut session = seeded_session();
        let r1 = session
            .apply("alice", 1, EditOperation::AddNode(VizNode::new("b", "B")))
            .expect("add b");
        let r2 = session
            .apply(
                "alice",
                2,
                EditOperation::AddEdge(VizEdge::new("a", "b", "requires")),
            )
            .expect("add edge");
        assert_eq!((r1, r2), (1, 2));
        assert_eq!(session.current_revision(), 2);
        assert_eq!(session.log().len(), 2);
        assert!(session.document().has_edge("a", "b"));
    }

    #[test]
    fn add_duplicate_node_fails_without_advancing() {
        let mut session = seeded_session();
        let err = session
            .apply("alice", 1, EditOperation::AddNode(VizNode::new("a", "dup")))
            .unwrap_err();
        assert!(matches!(err, VizError::InvalidStructure(_)));
        assert_eq!(session.current_revision(), 0);
        assert!(session.log().is_empty());
    }

    #[test]
    fn add_edge_requires_existing_endpoints() {
        let mut session = seeded_session();
        let err = session
            .apply(
                "alice",
                1,
                EditOperation::AddEdge(VizEdge::new("a", "ghost", "x")),
            )
            .unwrap_err();
        assert!(matches!(err, VizError::InvalidStructure(_)));
    }

    #[test]
    fn remove_node_drops_edges_and_updates_label() {
        let mut session = seeded_session();
        session
            .apply("alice", 1, EditOperation::AddNode(VizNode::new("b", "B")))
            .expect("add b");
        session
            .apply(
                "alice",
                2,
                EditOperation::AddEdge(VizEdge::new("a", "b", "r")),
            )
            .expect("edge");
        session
            .apply(
                "alice",
                3,
                EditOperation::UpdateNodeLabel {
                    id: "a".to_string(),
                    label: "Alpha".to_string(),
                },
            )
            .expect("relabel");
        assert_eq!(
            session.document().node("a").map(|n| n.label.as_str()),
            Some("Alpha")
        );
        session
            .apply(
                "alice",
                4,
                EditOperation::RemoveNode {
                    id: "b".to_string(),
                },
            )
            .expect("remove b");
        assert!(!session.document().has_node("b"));
        assert!(!session.document().has_edge("a", "b"));
    }

    #[test]
    fn set_and_remove_attribute() {
        let mut session = seeded_session();
        session
            .apply(
                "alice",
                1,
                EditOperation::SetNodeAttribute {
                    id: "a".to_string(),
                    key: "color".to_string(),
                    value: "red".to_string(),
                },
            )
            .expect("set attr");
        assert_eq!(
            session
                .document()
                .node("a")
                .and_then(|n| n.attributes.get("color"))
                .map(String::as_str),
            Some("red")
        );
        session
            .apply(
                "alice",
                2,
                EditOperation::RemoveNodeAttribute {
                    id: "a".to_string(),
                    key: "color".to_string(),
                },
            )
            .expect("remove attr");
        assert!(
            session
                .document()
                .node("a")
                .map(|n| n.attributes.is_empty())
                .unwrap_or(false)
        );
    }

    #[test]
    fn conflict_detection_blocks_stale_base_revision() {
        let mut session = seeded_session();
        session
            .apply("alice", 1, EditOperation::AddNode(VizNode::new("b", "B")))
            .expect("add b");
        // Session is now at r1; an edit based on r0 conflicts.
        assert!(session.would_conflict(0));
        let err = session
            .apply_at(0, "bob", 2, EditOperation::AddNode(VizNode::new("c", "C")))
            .unwrap_err();
        assert!(matches!(err, VizError::InvalidStructure(_)));
        // Based on the current revision it succeeds.
        let r2 = session
            .apply_at(1, "bob", 3, EditOperation::AddNode(VizNode::new("c", "C")))
            .expect("apply at r1");
        assert_eq!(r2, 2);
    }

    #[test]
    fn access_control_gates_edits() {
        let mut acl = AccessControlList::new();
        acl.grant("editor", Role::Editor);
        acl.grant("viewer", Role::Viewer);
        let mut session = seeded_session().with_access_control(acl);
        assert!(
            session
                .apply("viewer", 1, EditOperation::AddNode(VizNode::new("b", "B")))
                .is_err()
        );
        assert!(
            session
                .apply("editor", 2, EditOperation::AddNode(VizNode::new("b", "B")))
                .is_ok()
        );
        assert_eq!(session.current_revision(), 1);
    }

    #[test]
    fn javascript_escapes_endpoint_and_carries_revision() {
        let mut session = seeded_session();
        session
            .apply("alice", 1, EditOperation::AddNode(VizNode::new("b", "B")))
            .expect("add");
        let js = session.to_javascript("wss://example.test/\"<script>");
        assert!(js.contains("\\u003c"));
        assert!(!js.contains("\"<script>"));
        assert!(js.contains("let revision = 1;"));
        assert!(js.contains("window.submitEdit"));
    }

    #[test]
    fn edit_session_json_round_trip() {
        let mut session = seeded_session();
        session
            .apply("alice", 1, EditOperation::AddNode(VizNode::new("b", "B")))
            .expect("add");
        let json = session.to_json().expect("to_json");
        let restored = EditSession::from_json(&json).expect("from_json");
        assert_eq!(session, restored);
    }
}
