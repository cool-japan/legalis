//! Visualization change diffing.
//!
//! [`VizDiff::between`] compares two [`VizDocument`]s and reports which nodes and
//! edges were added, removed or modified, deterministically ordered by id. The
//! result renders to plain text or a colour-coded HTML table, and underpins the
//! "diff view for visualization changes" collaboration feature.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::{VizDocument, VizEdge, VizNode};
use crate::data_exchange::escape_xml;
use crate::types_10::Theme;

/// The kind of change a node or edge underwent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeKind {
    /// Present only in the new document.
    Added,
    /// Present only in the old document.
    Removed,
    /// Present in both but altered.
    Modified,
}

impl ChangeKind {
    /// A lower-case label.
    pub fn label(&self) -> &'static str {
        match self {
            ChangeKind::Added => "added",
            ChangeKind::Removed => "removed",
            ChangeKind::Modified => "modified",
        }
    }

    /// A representative colour (green/red/amber).
    pub fn color(&self) -> &'static str {
        match self {
            ChangeKind::Added => "#2e7d32",
            ChangeKind::Removed => "#c62828",
            ChangeKind::Modified => "#f9a825",
        }
    }
}

/// A change to a single node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeChange {
    /// Node id.
    pub id: String,
    /// What kind of change occurred.
    pub kind: ChangeKind,
    /// The node before the change (absent for additions).
    pub before: Option<VizNode>,
    /// The node after the change (absent for removals).
    pub after: Option<VizNode>,
    /// Names of attributes that changed (for modifications); `"label"` is
    /// included when the display label changed.
    pub changed_fields: Vec<String>,
}

/// A change to a single edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeChange {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// What kind of change occurred.
    pub kind: ChangeKind,
    /// The edge label before the change.
    pub before_label: Option<String>,
    /// The edge label after the change.
    pub after_label: Option<String>,
}

/// The computed difference between two [`VizDocument`]s.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VizDiff {
    /// Node changes, ordered by id.
    pub node_changes: Vec<NodeChange>,
    /// Edge changes, ordered by `(from, to)`.
    pub edge_changes: Vec<EdgeChange>,
}

impl VizDiff {
    /// Computes the diff turning `old` into `new`.
    pub fn between(old: &VizDocument, new: &VizDocument) -> Self {
        let mut diff = VizDiff::default();

        let old_nodes: BTreeMap<&str, &VizNode> =
            old.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
        let new_nodes: BTreeMap<&str, &VizNode> =
            new.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
        let node_ids: BTreeSet<&str> = old_nodes.keys().chain(new_nodes.keys()).copied().collect();
        for id in node_ids {
            match (old_nodes.get(id), new_nodes.get(id)) {
                (None, Some(after)) => diff.node_changes.push(NodeChange {
                    id: id.to_string(),
                    kind: ChangeKind::Added,
                    before: None,
                    after: Some((*after).clone()),
                    changed_fields: Vec::new(),
                }),
                (Some(before), None) => diff.node_changes.push(NodeChange {
                    id: id.to_string(),
                    kind: ChangeKind::Removed,
                    before: Some((*before).clone()),
                    after: None,
                    changed_fields: Vec::new(),
                }),
                (Some(before), Some(after)) => {
                    let changed = changed_node_fields(before, after);
                    if !changed.is_empty() {
                        diff.node_changes.push(NodeChange {
                            id: id.to_string(),
                            kind: ChangeKind::Modified,
                            before: Some((*before).clone()),
                            after: Some((*after).clone()),
                            changed_fields: changed,
                        });
                    }
                }
                (None, None) => {}
            }
        }

        let old_edges: BTreeMap<(&str, &str), &VizEdge> = old
            .edges
            .iter()
            .map(|e| ((e.from.as_str(), e.to.as_str()), e))
            .collect();
        let new_edges: BTreeMap<(&str, &str), &VizEdge> = new
            .edges
            .iter()
            .map(|e| ((e.from.as_str(), e.to.as_str()), e))
            .collect();
        let edge_keys: BTreeSet<(&str, &str)> =
            old_edges.keys().chain(new_edges.keys()).copied().collect();
        for (from, to) in edge_keys {
            match (old_edges.get(&(from, to)), new_edges.get(&(from, to))) {
                (None, Some(after)) => diff.edge_changes.push(EdgeChange {
                    from: from.to_string(),
                    to: to.to_string(),
                    kind: ChangeKind::Added,
                    before_label: None,
                    after_label: Some(after.label.clone()),
                }),
                (Some(before), None) => diff.edge_changes.push(EdgeChange {
                    from: from.to_string(),
                    to: to.to_string(),
                    kind: ChangeKind::Removed,
                    before_label: Some(before.label.clone()),
                    after_label: None,
                }),
                (Some(before), Some(after)) if before.label != after.label => {
                    diff.edge_changes.push(EdgeChange {
                        from: from.to_string(),
                        to: to.to_string(),
                        kind: ChangeKind::Modified,
                        before_label: Some(before.label.clone()),
                        after_label: Some(after.label.clone()),
                    });
                }
                _ => {}
            }
        }

        diff
    }

    /// Returns true if the documents are identical.
    pub fn is_empty(&self) -> bool {
        self.node_changes.is_empty() && self.edge_changes.is_empty()
    }

    /// Returns `(added, removed, modified)` counts across nodes and edges.
    pub fn summary(&self) -> (usize, usize, usize) {
        let mut added = 0;
        let mut removed = 0;
        let mut modified = 0;
        for kind in self
            .node_changes
            .iter()
            .map(|c| c.kind)
            .chain(self.edge_changes.iter().map(|c| c.kind))
        {
            match kind {
                ChangeKind::Added => added += 1,
                ChangeKind::Removed => removed += 1,
                ChangeKind::Modified => modified += 1,
            }
        }
        (added, removed, modified)
    }

    /// Renders the diff as plain text.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        let (added, removed, modified) = self.summary();
        out.push_str(&format!(
            "Visualization diff: +{} added, -{} removed, ~{} modified\n",
            added, removed, modified
        ));
        if self.is_empty() {
            out.push_str("  (no changes)\n");
            return out;
        }
        for change in &self.node_changes {
            let marker = match change.kind {
                ChangeKind::Added => "+",
                ChangeKind::Removed => "-",
                ChangeKind::Modified => "~",
            };
            out.push_str(&format!("{} node {}", marker, change.id));
            if change.kind == ChangeKind::Modified && !change.changed_fields.is_empty() {
                out.push_str(&format!(" ({})", change.changed_fields.join(", ")));
            }
            out.push('\n');
        }
        for change in &self.edge_changes {
            let marker = match change.kind {
                ChangeKind::Added => "+",
                ChangeKind::Removed => "-",
                ChangeKind::Modified => "~",
            };
            out.push_str(&format!("{} edge {} -> {}", marker, change.from, change.to));
            if change.kind == ChangeKind::Modified {
                out.push_str(&format!(
                    " ({} => {})",
                    change.before_label.as_deref().unwrap_or(""),
                    change.after_label.as_deref().unwrap_or("")
                ));
            }
            out.push('\n');
        }
        out
    }

    /// Renders the diff as a colour-coded HTML table.
    pub fn to_html(&self, theme: &Theme) -> String {
        let mut html = String::new();
        let (added, removed, modified) = self.summary();
        html.push_str(&format!(
            "<div class=\"viz-diff\" style=\"background:{};color:{};font-family:sans-serif;\">\n",
            escape_xml(&theme.background_color),
            escape_xml(&theme.text_color)
        ));
        html.push_str(&format!(
            "  <p class=\"viz-diff-summary\">+{} added, -{} removed, ~{} modified</p>\n",
            added, removed, modified
        ));
        html.push_str("  <table class=\"viz-diff-table\">\n");
        html.push_str("    <thead><tr><th>Kind</th><th>Element</th><th>Detail</th></tr></thead>\n");
        html.push_str("    <tbody>\n");
        for change in &self.node_changes {
            let detail = if change.changed_fields.is_empty() {
                String::new()
            } else {
                change.changed_fields.join(", ")
            };
            html.push_str(&row_html(
                change.kind,
                &format!("node {}", change.id),
                &detail,
            ));
        }
        for change in &self.edge_changes {
            let detail = match (&change.before_label, &change.after_label) {
                (Some(b), Some(a)) if change.kind == ChangeKind::Modified => {
                    format!("{} => {}", b, a)
                }
                _ => String::new(),
            };
            html.push_str(&row_html(
                change.kind,
                &format!("edge {} -> {}", change.from, change.to),
                &detail,
            ));
        }
        html.push_str("    </tbody>\n  </table>\n</div>\n");
        html
    }
}

fn row_html(kind: ChangeKind, element: &str, detail: &str) -> String {
    format!(
        "      <tr><td style=\"color:{};font-weight:bold;\">{}</td><td>{}</td><td>{}</td></tr>\n",
        kind.color(),
        kind.label(),
        escape_xml(element),
        escape_xml(detail)
    )
}

/// Returns the changed field names between two nodes (`"label"` plus any
/// attribute whose value was added, removed or altered), sorted.
fn changed_node_fields(before: &VizNode, after: &VizNode) -> Vec<String> {
    let mut changed: Vec<String> = Vec::new();
    if before.label != after.label {
        changed.push("label".to_string());
    }
    let keys: BTreeSet<&str> = before
        .attributes
        .keys()
        .chain(after.attributes.keys())
        .map(String::as_str)
        .collect();
    for key in keys {
        if before.attributes.get(key) != after.attributes.get(key) {
            changed.push(key.to_string());
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_a() -> VizDocument {
        VizDocument::new()
            .with_node(VizNode::new("a", "A"))
            .with_node(VizNode::new("b", "B").with_attribute("color", "red"))
            .with_edge(VizEdge::new("a", "b", "requires"))
    }

    #[test]
    fn identical_documents_have_empty_diff() {
        let diff = VizDiff::between(&doc_a(), &doc_a());
        assert!(diff.is_empty());
        assert_eq!(diff.summary(), (0, 0, 0));
        assert!(diff.to_text().contains("no changes"));
    }

    #[test]
    fn detects_added_removed_modified_nodes() {
        let new = VizDocument::new()
            .with_node(VizNode::new("a", "A renamed"))
            .with_node(VizNode::new("c", "C"))
            .with_edge(VizEdge::new("a", "b", "requires"));
        let diff = VizDiff::between(&doc_a(), &new);
        // a modified (label), b removed, c added.
        let a = diff.node_changes.iter().find(|c| c.id == "a").expect("a");
        assert_eq!(a.kind, ChangeKind::Modified);
        assert!(a.changed_fields.contains(&"label".to_string()));
        assert!(
            diff.node_changes
                .iter()
                .any(|c| c.id == "b" && c.kind == ChangeKind::Removed)
        );
        assert!(
            diff.node_changes
                .iter()
                .any(|c| c.id == "c" && c.kind == ChangeKind::Added)
        );
    }

    #[test]
    fn detects_edge_relabel_and_removal() {
        let new = VizDocument::new()
            .with_node(VizNode::new("a", "A"))
            .with_node(VizNode::new("b", "B").with_attribute("color", "red"))
            .with_edge(VizEdge::new("a", "b", "amends"));
        let diff = VizDiff::between(&doc_a(), &new);
        let edge = diff
            .edge_changes
            .iter()
            .find(|c| c.from == "a" && c.to == "b")
            .expect("edge");
        assert_eq!(edge.kind, ChangeKind::Modified);
        assert_eq!(edge.before_label.as_deref(), Some("requires"));
        assert_eq!(edge.after_label.as_deref(), Some("amends"));
    }

    #[test]
    fn diff_html_is_colour_coded_and_escaped() {
        let new = VizDocument::new().with_node(VizNode::new("<x>", "X"));
        let diff = VizDiff::between(&VizDocument::new(), &new);
        let html = diff.to_html(&Theme::light());
        assert!(html.contains("viz-diff-table"));
        assert!(html.contains(ChangeKind::Added.color()));
        assert!(html.contains("node &lt;x&gt;"));
    }

    #[test]
    fn modified_attribute_is_reported() {
        let new = VizDocument::new()
            .with_node(VizNode::new("a", "A"))
            .with_node(VizNode::new("b", "B").with_attribute("color", "blue"))
            .with_edge(VizEdge::new("a", "b", "requires"));
        let diff = VizDiff::between(&doc_a(), &new);
        let b = diff.node_changes.iter().find(|c| c.id == "b").expect("b");
        assert_eq!(b.kind, ChangeKind::Modified);
        assert_eq!(b.changed_fields, vec!["color".to_string()]);
    }
}
