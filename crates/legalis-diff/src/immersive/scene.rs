//! The [`Scene3d`] graph: a 3-D, navigable representation of statute diffs.
//!
//! A scene is a set of [`SceneNode`]s (each carrying a 3-D [`Vec3`] position,
//! colour and size) connected by typed [`SceneEdge`]s. It is the shared data
//! model consumed by the [`super::layout`], [`super::xr`],
//! [`super::navigation`] and [`super::plugin`] sub-modules.

use super::{Color, Vec3, seed_position};
use crate::{Change, ChangeTarget, ChangeType, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// The semantic role of a [`SceneNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    /// The root node representing a whole statute.
    Statute,
    /// A node representing a single change of the given type.
    Change(ChangeType),
    /// A node grouping changes that touch the same target.
    TargetGroup,
    /// A node summarising the overall impact assessment.
    Impact,
    /// A synthetic root joining several statute sub-graphs.
    Forest,
}

impl NodeKind {
    /// A short, stable string tag (handy for exporters and tests).
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            NodeKind::Statute => "statute",
            NodeKind::Change(ChangeType::Added) => "change-added",
            NodeKind::Change(ChangeType::Removed) => "change-removed",
            NodeKind::Change(ChangeType::Modified) => "change-modified",
            NodeKind::Change(ChangeType::Reordered) => "change-reordered",
            NodeKind::TargetGroup => "target-group",
            NodeKind::Impact => "impact",
            NodeKind::Forest => "forest",
        }
    }
}

/// The semantic role of a [`SceneEdge`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeKind {
    /// Structural containment (statute → change, forest → statute).
    Contains,
    /// A change belongs to a target group.
    Grouped,
    /// Two changes are related (e.g. touch the same target).
    Related,
    /// A node contributes to the impact assessment.
    Impacts,
}

impl EdgeKind {
    /// A short, stable string tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            EdgeKind::Contains => "contains",
            EdgeKind::Grouped => "grouped",
            EdgeKind::Related => "related",
            EdgeKind::Impacts => "impacts",
        }
    }
}

/// A node in a [`Scene3d`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneNode {
    /// Stable, unique identifier within the scene.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Semantic role.
    pub kind: NodeKind,
    /// World-space position (seeded deterministically, refined by layout).
    pub position: Vec3,
    /// Fill colour.
    pub color: Color,
    /// Visual radius / weight.
    pub size: f64,
    /// Arbitrary string metadata.
    pub metadata: BTreeMap<String, String>,
}

impl SceneNode {
    /// Creates a node with a deterministic seed position derived from its id.
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>, kind: NodeKind) -> Self {
        let id = id.into();
        let position = seed_position(&id, 6.0);
        let (color, size) = default_style(kind);
        Self {
            id,
            label: label.into(),
            kind,
            position,
            color,
            size,
            metadata: BTreeMap::new(),
        }
    }

    /// Builder: overrides the colour.
    #[must_use]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Builder: overrides the size.
    #[must_use]
    pub fn with_size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// Builder: attaches a metadata key/value pair.
    #[must_use]
    pub fn with_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// An edge in a [`Scene3d`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneEdge {
    /// Source node id.
    pub source: String,
    /// Target node id.
    pub target: String,
    /// Semantic role.
    pub kind: EdgeKind,
    /// Edge weight (used by force-directed layout; higher = stiffer spring).
    pub weight: f64,
}

impl SceneEdge {
    /// Creates an edge with unit weight.
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>, kind: EdgeKind) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            kind,
            weight: 1.0,
        }
    }

    /// Builder: overrides the weight.
    #[must_use]
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }
}

/// A 3-D scene graph of nodes and typed edges.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Scene3d {
    nodes: Vec<SceneNode>,
    edges: Vec<SceneEdge>,
    #[serde(skip)]
    index: HashMap<String, usize>,
}

impl Scene3d {
    /// Creates an empty scene.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebuilds the internal id → index map (after deserialisation).
    fn reindex(&mut self) {
        self.index = self
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (n.id.clone(), i))
            .collect();
    }

    /// Adds a node. Re-adding an id replaces the existing node in place and
    /// returns `false`; a fresh id returns `true`.
    pub fn add_node(&mut self, node: SceneNode) -> bool {
        if let Some(&i) = self.index.get(&node.id) {
            self.nodes[i] = node;
            false
        } else {
            self.index.insert(node.id.clone(), self.nodes.len());
            self.nodes.push(node);
            true
        }
    }

    /// Adds an edge **only if** both endpoints already exist and it is not a
    /// self-loop. Returns `true` when the edge was added.
    pub fn add_edge(&mut self, edge: SceneEdge) -> bool {
        if edge.source == edge.target {
            return false;
        }
        if self.index.contains_key(&edge.source) && self.index.contains_key(&edge.target) {
            self.edges.push(edge);
            true
        } else {
            false
        }
    }

    /// All nodes.
    #[must_use]
    pub fn nodes(&self) -> &[SceneNode] {
        &self.nodes
    }

    /// All edges.
    #[must_use]
    pub fn edges(&self) -> &[SceneEdge] {
        &self.edges
    }

    /// Number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns `true` if the scene has no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Looks up a node by id.
    #[must_use]
    pub fn node(&self, id: &str) -> Option<&SceneNode> {
        self.index.get(id).and_then(|&i| self.nodes.get(i))
    }

    /// Mutable access to a node's position by id.
    pub fn set_position(&mut self, id: &str, position: Vec3) -> bool {
        if let Some(&i) = self.index.get(id) {
            self.nodes[i].position = position;
            true
        } else {
            false
        }
    }

    /// Ids of the nodes adjacent to `id` (edges in either direction), sorted
    /// for determinism.
    #[must_use]
    pub fn neighbors(&self, id: &str) -> Vec<String> {
        let mut out: Vec<String> = self
            .edges
            .iter()
            .filter_map(|e| {
                if e.source == id {
                    Some(e.target.clone())
                } else if e.target == id {
                    Some(e.source.clone())
                } else {
                    None
                }
            })
            .collect();
        out.sort();
        out.dedup();
        out
    }

    /// The number of edges incident to `id`.
    #[must_use]
    pub fn degree(&self, id: &str) -> usize {
        self.edges
            .iter()
            .filter(|e| e.source == id || e.target == id)
            .count()
    }

    /// The bounding box enclosing every node position.
    #[must_use]
    pub fn bounds(&self) -> super::BoundingBox {
        super::BoundingBox::from_points(self.nodes.iter().map(|n| n.position))
    }
}

/// Picks a default colour and size for a node kind.
fn default_style(kind: NodeKind) -> (Color, f64) {
    match kind {
        NodeKind::Statute => (Color::rgb(0x34, 0x3a, 0x40), 1.6),
        NodeKind::Forest => (Color::rgb(0x21, 0x25, 0x29), 2.0),
        NodeKind::Change(ct) => (Color::for_change_type(ct), 1.0),
        NodeKind::TargetGroup => (Color::rgb(0x6f, 0x42, 0xc1), 1.2),
        NodeKind::Impact => (Color::rgb(0xfd, 0x7e, 0x14), 1.4),
    }
}

/// A stable string key for a [`ChangeTarget`] (used to group related changes).
fn target_key(target: &ChangeTarget) -> String {
    match target {
        ChangeTarget::Title => "title".to_string(),
        ChangeTarget::Precondition { index } => format!("precondition:{index}"),
        ChangeTarget::Effect => "effect".to_string(),
        ChangeTarget::DiscretionLogic => "discretion".to_string(),
        ChangeTarget::Metadata { key } => format!("metadata:{key}"),
    }
}

/// A per-change importance weight that drives node size.
fn change_weight(change: &Change) -> f64 {
    let base = match change.target {
        ChangeTarget::Effect => 1.6,
        ChangeTarget::DiscretionLogic => 1.5,
        ChangeTarget::Precondition { .. } => 1.3,
        ChangeTarget::Title => 1.0,
        ChangeTarget::Metadata { .. } => 0.9,
    };
    let modifier = match change.change_type {
        ChangeType::Removed => 1.15,
        ChangeType::Added => 1.1,
        ChangeType::Modified => 1.0,
        ChangeType::Reordered => 0.85,
    };
    base * modifier
}

/// Builds a [`Scene3d`] for a single [`StatuteDiff`].
///
/// The graph has a [`NodeKind::Statute`] root, one [`NodeKind::Change`] node per
/// change, one [`NodeKind::TargetGroup`] node per distinct change target, and a
/// single [`NodeKind::Impact`] node. Changes are linked to their statute and
/// their target group; the impact node is linked from the statute.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::diff;
/// use legalis_diff::immersive::scene_from_diff;
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
///     .with_precondition(Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 65 });
/// let mut new = old.clone();
/// new.preconditions[0] = Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 60 };
///
/// let d = diff(&old, &new).unwrap();
/// let scene = scene_from_diff(&d);
/// assert!(scene.node("law").is_some());
/// assert!(scene.node_count() >= 2);
/// ```
#[must_use]
pub fn scene_from_diff(diff: &StatuteDiff) -> Scene3d {
    let mut scene = Scene3d::new();
    add_diff_subgraph(&mut scene, diff, None);
    scene
}

/// Builds a single [`Scene3d`] spanning several diffs, joined under one
/// synthetic [`NodeKind::Forest`] root so multi-statute relationships can be
/// explored together.
#[must_use]
pub fn scene_from_diffs(diffs: &[StatuteDiff]) -> Scene3d {
    let mut scene = Scene3d::new();
    if diffs.is_empty() {
        return scene;
    }
    if diffs.len() == 1 {
        add_diff_subgraph(&mut scene, &diffs[0], None);
        return scene;
    }
    let forest_id = "forest:root";
    scene.add_node(
        SceneNode::new(forest_id, "All Statutes", NodeKind::Forest)
            .with_meta("statutes", diffs.len().to_string()),
    );
    for diff in diffs {
        add_diff_subgraph(&mut scene, diff, Some(forest_id));
    }
    scene
}

/// Adds the sub-graph for one diff, optionally linking its statute root to a
/// parent (forest) node. Node ids are namespaced by statute id so multiple
/// diffs never collide.
fn add_diff_subgraph(scene: &mut Scene3d, diff: &StatuteDiff, parent: Option<&str>) {
    let statute_id = &diff.statute_id;
    scene.add_node(
        SceneNode::new(statute_id.clone(), statute_id.clone(), NodeKind::Statute)
            .with_size(1.6 + (diff.changes.len() as f64).sqrt() * 0.25)
            .with_meta("changes", diff.changes.len().to_string())
            .with_meta("severity", format!("{:?}", diff.impact.severity)),
    );
    if let Some(parent_id) = parent {
        scene.add_edge(SceneEdge::new(
            parent_id,
            statute_id.clone(),
            EdgeKind::Contains,
        ));
    }

    // Impact node.
    let impact_id = format!("{statute_id}::impact");
    scene.add_node(
        SceneNode::new(
            impact_id.clone(),
            format!("Impact: {:?}", diff.impact.severity),
            NodeKind::Impact,
        )
        .with_color(Color::for_severity(diff.impact.severity))
        .with_size(1.0 + severity_rank(diff.impact.severity) as f64 * 0.2)
        .with_meta("notes", diff.impact.notes.len().to_string()),
    );
    scene.add_edge(
        SceneEdge::new(statute_id.clone(), impact_id.clone(), EdgeKind::Impacts).with_weight(0.6),
    );

    // Target-group nodes (deduplicated, deterministic order via BTreeMap).
    let mut group_ids: BTreeMap<String, String> = BTreeMap::new();
    for change in &diff.changes {
        let key = target_key(&change.target);
        group_ids.entry(key.clone()).or_insert_with(|| {
            let gid = format!("{statute_id}::group::{key}");
            scene.add_node(
                SceneNode::new(
                    gid.clone(),
                    change.target.to_string(),
                    NodeKind::TargetGroup,
                )
                .with_meta("target", change.target.to_string()),
            );
            scene.add_edge(SceneEdge::new(
                statute_id.clone(),
                gid.clone(),
                EdgeKind::Contains,
            ));
            gid
        });
    }

    // Change nodes.
    for (i, change) in diff.changes.iter().enumerate() {
        let cid = format!("{statute_id}::change::{i}");
        scene.add_node(
            SceneNode::new(
                cid.clone(),
                change.description.clone(),
                NodeKind::Change(change.change_type),
            )
            .with_size(change_weight(change))
            .with_meta("type", format!("{:?}", change.change_type))
            .with_meta("target", change.target.to_string()),
        );
        scene.add_edge(SceneEdge::new(
            statute_id.clone(),
            cid.clone(),
            EdgeKind::Contains,
        ));
        if let Some(gid) = group_ids.get(&target_key(&change.target)) {
            scene.add_edge(
                SceneEdge::new(gid.clone(), cid.clone(), EdgeKind::Grouped).with_weight(1.2),
            );
        }
    }

    // "Related" edges between consecutive changes sharing a target group.
    let mut by_group: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (i, change) in diff.changes.iter().enumerate() {
        let cid = format!("{statute_id}::change::{i}");
        by_group
            .entry(target_key(&change.target))
            .or_default()
            .push(cid);
    }
    for ids in by_group.values() {
        for pair in ids.windows(2) {
            if let [a, b] = pair {
                scene.add_edge(
                    SceneEdge::new(a.clone(), b.clone(), EdgeKind::Related).with_weight(0.5),
                );
            }
        }
    }
}

/// Numeric rank for a severity (None = 0 … Breaking = 4).
fn severity_rank(severity: Severity) -> u8 {
    match severity {
        Severity::None => 0,
        Severity::Minor => 1,
        Severity::Moderate => 2,
        Severity::Major => 3,
        Severity::Breaking => 4,
    }
}

/// Deserialises a [`Scene3d`] from JSON, restoring its internal index.
///
/// # Errors
///
/// Returns [`crate::DiffError::SerializationError`] if the JSON is invalid.
pub fn scene_from_json(json: &str) -> crate::DiffResult<Scene3d> {
    let mut scene: Scene3d = serde_json::from_str(json)
        .map_err(|e| crate::DiffError::SerializationError(e.to_string()))?;
    scene.reindex();
    Ok(scene)
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn sample_diff() -> StatuteDiff {
        let old = Statute::new(
            "law-1",
            "Old Title",
            Effect::new(EffectType::Grant, "Benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        });
        let mut new = old.clone();
        new.title = "New Title".to_string();
        new.preconditions[0] = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 60,
        };
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        crate::diff(&old, &new).expect("diff should succeed")
    }

    #[test]
    fn test_scene_has_statute_and_change_nodes() {
        let scene = scene_from_diff(&sample_diff());
        assert!(scene.node("law-1").is_some());
        assert_eq!(scene.node("law-1").map(|n| n.kind), Some(NodeKind::Statute));
        let change_nodes = scene
            .nodes()
            .iter()
            .filter(|n| matches!(n.kind, NodeKind::Change(_)))
            .count();
        assert_eq!(change_nodes, 3);
    }

    #[test]
    fn test_scene_edges_link_changes_to_statute() {
        let scene = scene_from_diff(&sample_diff());
        let contains = scene
            .edges()
            .iter()
            .filter(|e| e.kind == EdgeKind::Contains && e.source == "law-1")
            .count();
        // 3 change nodes + 3 target groups (title, precondition:0, effect).
        assert!(contains >= 3);
        assert!(scene.degree("law-1") >= 4);
    }

    #[test]
    fn test_add_node_dedupes_by_id() {
        let mut scene = Scene3d::new();
        assert!(scene.add_node(SceneNode::new("a", "A", NodeKind::Statute)));
        assert!(!scene.add_node(SceneNode::new("a", "A2", NodeKind::Statute)));
        assert_eq!(scene.node_count(), 1);
        assert_eq!(
            scene.node("a").map(|n| n.label.clone()),
            Some("A2".to_string())
        );
    }

    #[test]
    fn test_add_edge_requires_endpoints_and_rejects_self_loop() {
        let mut scene = Scene3d::new();
        scene.add_node(SceneNode::new("a", "A", NodeKind::Statute));
        scene.add_node(SceneNode::new("b", "B", NodeKind::Impact));
        assert!(!scene.add_edge(SceneEdge::new("a", "missing", EdgeKind::Contains)));
        assert!(!scene.add_edge(SceneEdge::new("a", "a", EdgeKind::Contains)));
        assert!(scene.add_edge(SceneEdge::new("a", "b", EdgeKind::Contains)));
        assert_eq!(scene.edge_count(), 1);
    }

    #[test]
    fn test_neighbors_are_sorted_and_deduped() {
        let scene = scene_from_diff(&sample_diff());
        let neigh = scene.neighbors("law-1");
        let mut sorted = neigh.clone();
        sorted.sort();
        assert_eq!(neigh, sorted);
        assert!(!neigh.is_empty());
    }

    #[test]
    fn test_scene_from_diffs_builds_forest() {
        let d1 = sample_diff();
        let mut d2 = sample_diff();
        d2.statute_id = "law-2".to_string();
        let scene = scene_from_diffs(&[d1, d2]);
        assert!(scene.node("forest:root").is_some());
        assert!(scene.node("law-1").is_some());
        assert!(scene.node("law-2").is_some());
        // Forest root contains both statutes.
        let forest_children = scene
            .edges()
            .iter()
            .filter(|e| e.source == "forest:root" && e.kind == EdgeKind::Contains)
            .count();
        assert_eq!(forest_children, 2);
    }

    #[test]
    fn test_scene_roundtrips_json() {
        let scene = scene_from_diff(&sample_diff());
        let json = serde_json::to_string(&scene).expect("serialise");
        let restored = scene_from_json(&json).expect("deserialise");
        assert_eq!(restored.node_count(), scene.node_count());
        // Index restored: lookup works after round-trip.
        assert!(restored.node("law-1").is_some());
    }

    #[test]
    fn test_node_kind_and_edge_kind_tags() {
        assert_eq!(NodeKind::Statute.tag(), "statute");
        assert_eq!(NodeKind::Change(ChangeType::Added).tag(), "change-added");
        assert_eq!(EdgeKind::Related.tag(), "related");
    }

    #[test]
    fn test_set_position_updates_node() {
        let mut scene = scene_from_diff(&sample_diff());
        assert!(scene.set_position("law-1", Vec3::new(1.0, 2.0, 3.0)));
        assert_eq!(
            scene.node("law-1").map(|n| n.position),
            Some(Vec3::new(1.0, 2.0, 3.0))
        );
        assert!(!scene.set_position("nope", Vec3::zero()));
    }
}
