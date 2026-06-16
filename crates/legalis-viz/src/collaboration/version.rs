//! Version control for visualizations.
//!
//! [`VizVersionControl`] keeps an append-only history of [`VizSnapshot`]s, each
//! pinned to a monotonically increasing revision number with a parent link, an
//! author, a timestamp and a content hash. It supports committing, checking out
//! a past revision, reverting (which records a *new* revision restoring an old
//! document) and diffing any two revisions.

use serde::{Deserialize, Serialize};

use super::{VizDiff, VizDocument};
use crate::{VizError, VizResult};

/// One committed revision of a [`VizDocument`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VizSnapshot {
    /// Revision number (unique, increasing within a history).
    pub revision: u64,
    /// Short human-readable label / commit message.
    pub label: String,
    /// Author identifier.
    pub author: String,
    /// Caller-supplied timestamp (e.g. Unix epoch millis).
    pub timestamp: u64,
    /// Parent revision, or `None` for the initial commit.
    pub parent: Option<u64>,
    /// Content hash of the document at this revision.
    pub hash: String,
    /// The document state at this revision.
    pub document: VizDocument,
}

/// An append-only revision history for a single visualization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VizVersionControl {
    /// Committed snapshots, in revision order.
    snapshots: Vec<VizSnapshot>,
    /// The revision number the next commit will receive.
    next_revision: u64,
}

impl Default for VizVersionControl {
    fn default() -> Self {
        Self {
            snapshots: Vec::new(),
            next_revision: 1,
        }
    }
}

impl VizVersionControl {
    /// Creates an empty history (first revision will be `1`).
    pub fn new() -> Self {
        Self::default()
    }

    /// Commits `document` as a new revision and returns its revision number.
    pub fn commit(
        &mut self,
        label: &str,
        author: &str,
        timestamp: u64,
        document: VizDocument,
    ) -> u64 {
        let revision = self.next_revision;
        let parent = self.snapshots.last().map(|s| s.revision);
        let hash = document.content_hash();
        self.snapshots.push(VizSnapshot {
            revision,
            label: label.to_string(),
            author: author.to_string(),
            timestamp,
            parent,
            hash,
            document,
        });
        self.next_revision += 1;
        revision
    }

    /// Commits only if the document differs from the latest revision; returns
    /// the new revision number, or `None` if there was nothing to commit.
    pub fn commit_if_changed(
        &mut self,
        label: &str,
        author: &str,
        timestamp: u64,
        document: VizDocument,
    ) -> Option<u64> {
        if self
            .snapshots
            .last()
            .is_some_and(|latest| latest.hash == document.content_hash())
        {
            return None;
        }
        Some(self.commit(label, author, timestamp, document))
    }

    /// The full history, in revision order.
    pub fn history(&self) -> &[VizSnapshot] {
        &self.snapshots
    }

    /// The most recent snapshot, if any.
    pub fn latest(&self) -> Option<&VizSnapshot> {
        self.snapshots.last()
    }

    /// Looks up a snapshot by revision number.
    pub fn get(&self, revision: u64) -> Option<&VizSnapshot> {
        self.snapshots.iter().find(|s| s.revision == revision)
    }

    /// The number of committed revisions.
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }

    /// Whether the history is empty.
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }

    /// Returns the document at a revision, or an error if it does not exist.
    pub fn checkout(&self, revision: u64) -> VizResult<&VizDocument> {
        self.get(revision)
            .map(|s| &s.document)
            .ok_or_else(|| VizError::InvalidStructure(format!("no such revision: r{}", revision)))
    }

    /// Reverts to a past revision by committing a *new* revision whose document
    /// equals that of `revision`. Returns the new revision number.
    pub fn revert_to(&mut self, revision: u64, author: &str, timestamp: u64) -> VizResult<u64> {
        let document = self.checkout(revision)?.clone();
        let label = format!("Revert to r{}", revision);
        Ok(self.commit(&label, author, timestamp, document))
    }

    /// Computes the diff between two revisions (`from` -> `to`).
    pub fn diff_revisions(&self, from: u64, to: u64) -> VizResult<VizDiff> {
        let old = self.checkout(from)?;
        let new = self.checkout(to)?;
        Ok(VizDiff::between(old, new))
    }

    /// Serializes the history to pretty JSON.
    pub fn to_json(&self) -> VizResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| VizError::ExportError(format!("version history to JSON: {}", e)))
    }

    /// Parses a history from JSON.
    pub fn from_json(json: &str) -> VizResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VizError::InvalidStructure(format!("version history from JSON: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collaboration::{VizDocument, VizNode};

    fn doc(label: &str) -> VizDocument {
        VizDocument::new().with_node(VizNode::new("a", label))
    }

    #[test]
    fn commit_assigns_increasing_revisions_with_parents() {
        let mut vc = VizVersionControl::new();
        assert!(vc.is_empty());
        let r1 = vc.commit("init", "alice", 1, doc("A"));
        let r2 = vc.commit("edit", "bob", 2, doc("B"));
        assert_eq!((r1, r2), (1, 2));
        assert_eq!(vc.len(), 2);
        assert_eq!(vc.get(1).and_then(|s| s.parent), None);
        assert_eq!(vc.get(2).and_then(|s| s.parent), Some(1));
        assert_eq!(vc.latest().map(|s| s.revision), Some(2));
    }

    #[test]
    fn commit_if_changed_skips_identical_documents() {
        let mut vc = VizVersionControl::new();
        vc.commit("init", "alice", 1, doc("A"));
        assert_eq!(vc.commit_if_changed("again", "alice", 2, doc("A")), None);
        assert_eq!(
            vc.commit_if_changed("change", "alice", 3, doc("B")),
            Some(2)
        );
        assert_eq!(vc.len(), 2);
    }

    #[test]
    fn checkout_unknown_revision_errors() {
        let vc = VizVersionControl::new();
        assert!(matches!(
            vc.checkout(99),
            Err(VizError::InvalidStructure(_))
        ));
    }

    #[test]
    fn revert_creates_new_revision_restoring_old_document() {
        let mut vc = VizVersionControl::new();
        vc.commit("init", "alice", 1, doc("A"));
        vc.commit("edit", "bob", 2, doc("B"));
        let r3 = vc.revert_to(1, "carol", 3).expect("revert");
        assert_eq!(r3, 3);
        assert_eq!(
            vc.checkout(3)
                .expect("doc")
                .node("a")
                .map(|n| n.label.as_str()),
            Some("A")
        );
        assert!(
            vc.get(3)
                .map(|s| s.label.contains("Revert to r1"))
                .unwrap_or(false)
        );
    }

    #[test]
    fn diff_revisions_reports_changes() {
        let mut vc = VizVersionControl::new();
        vc.commit("init", "alice", 1, doc("A"));
        vc.commit("edit", "bob", 2, doc("B"));
        let diff = vc.diff_revisions(1, 2).expect("diff");
        assert!(!diff.is_empty());
        assert!(vc.diff_revisions(1, 99).is_err());
    }

    #[test]
    fn version_history_json_round_trip() {
        let mut vc = VizVersionControl::new();
        vc.commit("init", "alice", 1, doc("A"));
        let json = vc.to_json().expect("to_json");
        let restored = VizVersionControl::from_json(&json).expect("from_json");
        assert_eq!(vc, restored);
        // Next commit continues numbering correctly after restore.
        let mut restored = restored;
        assert_eq!(restored.commit("next", "bob", 2, doc("B")), 2);
    }
}
