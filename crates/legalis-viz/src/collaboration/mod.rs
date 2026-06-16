//! Collaboration 2.0: version control, change diffing, threaded comments,
//! collaborative-edit operations and role-based access control for
//! visualizations.
//!
//! This module models the *data and operations* behind collaborative work on a
//! visualization, built on a neutral [`VizDocument`] (nodes + edges +
//! attributes) that any of the crate's models can be projected into (for
//! example via [`VizDocument::from_dependency_graph`]). It complements — rather
//! than replaces — the existing real-time *viewing* primitives
//! ([`CollaborativeSession`](crate::CollaborativeSession),
//! [`SharedAnnotation`](crate::SharedAnnotation)) by adding the editing-side
//! model:
//!
//! - [`version`] — [`VizVersionControl`] keeps an ordered history of
//!   [`VizSnapshot`]s with parent links, content hashing and revert.
//! - [`diff`] — [`VizDiff`] computes the added/removed/modified nodes and edges
//!   between two documents and renders them as text or HTML.
//! - [`comments`] — [`CommentThread`] / [`CommentBoard`] provide *threaded*
//!   (nested-reply) comments anchored to nodes, with resolution.
//! - [`editing`] — [`EditOperation`] / [`EditSession`] apply an ordered,
//!   revision-tracked operation log to a document with conflict detection.
//! - [`permissions`] — [`Role`] / [`Capability`] / [`AccessControlList`] gate
//!   who may view, comment, edit or administer.
//!
//! ## Runtime boundary
//!
//! Live network transport (a WebSocket server distributing operations between
//! clients) is a deployment concern, not a library one. As with the existing
//! [`CollaborativeSession`](crate::CollaborativeSession), this module provides
//! the document model, the operation log and conflict detection, plus a
//! client-side script ([`EditSession::to_javascript`]) that speaks to a
//! caller-supplied endpoint — it does not open sockets or run a server itself.

mod comments;
mod diff;
mod editing;
mod permissions;
mod version;

pub use comments::{Comment, CommentBoard, CommentThread};
pub use diff::{ChangeKind, EdgeChange, NodeChange, VizDiff};
pub use editing::{EditEntry, EditOperation, EditSession};
pub use permissions::{AccessControlList, Capability, Role};
pub use version::{VizSnapshot, VizVersionControl};

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types_4::DependencyGraph;
use crate::{VizError, VizResult};

/// A node in a [`VizDocument`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VizNode {
    /// Stable node identifier.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Arbitrary key/value attributes (kept ordered for stable hashing).
    pub attributes: BTreeMap<String, String>,
}

impl VizNode {
    /// Creates a node with an id and label.
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            attributes: BTreeMap::new(),
        }
    }

    /// Adds an attribute (builder style).
    pub fn with_attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
}

/// A directed edge in a [`VizDocument`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VizEdge {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// Edge label / relation.
    pub label: String,
}

impl VizEdge {
    /// Creates an edge.
    pub fn new(from: &str, to: &str, label: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            label: label.to_string(),
        }
    }
}

/// A neutral, serializable visualization document: the unit that is versioned,
/// diffed, commented on and edited.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VizDocument {
    /// Nodes, in document order.
    pub nodes: Vec<VizNode>,
    /// Edges, in document order.
    pub edges: Vec<VizEdge>,
}

impl VizDocument {
    /// Creates an empty document.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node (builder style).
    pub fn with_node(mut self, node: VizNode) -> Self {
        self.nodes.push(node);
        self
    }

    /// Adds an edge (builder style).
    pub fn with_edge(mut self, edge: VizEdge) -> Self {
        self.edges.push(edge);
        self
    }

    /// Returns the node with the given id, if any.
    pub fn node(&self, id: &str) -> Option<&VizNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Returns a mutable reference to the node with the given id, if any.
    pub fn node_mut(&mut self, id: &str) -> Option<&mut VizNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Returns true if a node with the id exists.
    pub fn has_node(&self, id: &str) -> bool {
        self.nodes.iter().any(|n| n.id == id)
    }

    /// Returns true if an edge between `from` and `to` exists.
    pub fn has_edge(&self, from: &str, to: &str) -> bool {
        self.edges.iter().any(|e| e.from == from && e.to == to)
    }

    /// Removes the node with the id and any incident edges; returns whether a
    /// node was removed.
    pub fn remove_node(&mut self, id: &str) -> bool {
        let before = self.nodes.len();
        self.nodes.retain(|n| n.id != id);
        let removed = self.nodes.len() != before;
        if removed {
            self.edges.retain(|e| e.from != id && e.to != id);
        }
        removed
    }

    /// Removes the first edge between `from` and `to`; returns whether one was
    /// removed.
    pub fn remove_edge(&mut self, from: &str, to: &str) -> bool {
        if let Some(pos) = self.edges.iter().position(|e| e.from == from && e.to == to) {
            self.edges.remove(pos);
            true
        } else {
            false
        }
    }

    /// The node ids, in document order.
    pub fn node_ids(&self) -> Vec<&str> {
        self.nodes.iter().map(|n| n.id.as_str()).collect()
    }

    /// A stable FNV-1a content hash (hex) of the canonical serialization.
    pub fn content_hash(&self) -> String {
        let canonical = serde_json::to_string(self).unwrap_or_else(|_| format!("{:?}", self));
        format!("{:016x}", fnv1a_64(canonical.as_bytes()))
    }

    /// Projects a [`DependencyGraph`] into a document (node label == id, edge
    /// label == relation).
    pub fn from_dependency_graph(graph: &DependencyGraph) -> Self {
        let inner = &graph.graph;
        let mut document = VizDocument::new();
        for idx in inner.node_indices() {
            let id = &inner[idx];
            document.nodes.push(VizNode::new(id, id));
        }
        for edge in inner.edge_indices() {
            if let Some((source, target)) = inner.edge_endpoints(edge) {
                document
                    .edges
                    .push(VizEdge::new(&inner[source], &inner[target], &inner[edge]));
            }
        }
        document
    }

    /// Serializes the document to pretty JSON.
    pub fn to_json(&self) -> VizResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| VizError::ExportError(format!("document to JSON: {}", e)))
    }

    /// Parses a document from JSON.
    pub fn from_json(json: &str) -> VizResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VizError::InvalidStructure(format!("document from JSON: {}", e)))
    }
}

/// 64-bit FNV-1a hash of a byte slice.
pub(crate) fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_node_and_edge_lookup() {
        let doc = VizDocument::new()
            .with_node(VizNode::new("a", "A"))
            .with_node(VizNode::new("b", "B"))
            .with_edge(VizEdge::new("a", "b", "requires"));
        assert!(doc.has_node("a"));
        assert!(doc.has_edge("a", "b"));
        assert!(!doc.has_edge("b", "a"));
        assert_eq!(doc.node("b").map(|n| n.label.as_str()), Some("B"));
        assert_eq!(doc.node_ids(), vec!["a", "b"]);
    }

    #[test]
    fn remove_node_drops_incident_edges() {
        let mut doc = VizDocument::new()
            .with_node(VizNode::new("a", "A"))
            .with_node(VizNode::new("b", "B"))
            .with_edge(VizEdge::new("a", "b", "x"));
        assert!(doc.remove_node("a"));
        assert!(!doc.has_node("a"));
        assert!(doc.edges.is_empty());
        assert!(!doc.remove_node("a"));
    }

    #[test]
    fn content_hash_is_stable_and_sensitive() {
        let doc1 = VizDocument::new().with_node(VizNode::new("a", "A"));
        let doc2 = VizDocument::new().with_node(VizNode::new("a", "A"));
        let doc3 = VizDocument::new().with_node(VizNode::new("a", "Changed"));
        assert_eq!(doc1.content_hash(), doc2.content_hash());
        assert_ne!(doc1.content_hash(), doc3.content_hash());
    }

    #[test]
    fn from_dependency_graph_projects_nodes_and_edges() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "requires");
        let doc = VizDocument::from_dependency_graph(&graph);
        assert_eq!(doc.nodes.len(), 2);
        assert!(doc.has_edge("a", "b"));
    }

    #[test]
    fn document_json_round_trip() {
        let doc = VizDocument::new()
            .with_node(VizNode::new("a", "A").with_attribute("k", "v"))
            .with_edge(VizEdge::new("a", "a", "self"));
        let json = doc.to_json().expect("to_json");
        let restored = VizDocument::from_json(&json).expect("from_json");
        assert_eq!(doc, restored);
    }
}
