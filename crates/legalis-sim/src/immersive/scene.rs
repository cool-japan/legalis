//! The [`SimScene`] graph: a 3-D, navigable representation of a simulation.
//!
//! A scene is a set of [`SceneNode`]s (each carrying a 3-D [`Vec3`] position,
//! colour and size) connected by typed [`SceneEdge`]s. It is the shared data
//! model consumed by [`super::xr`], [`super::ar`] and [`super::collab`]. Scenes
//! are built either from a *population* (a slice of [`LegalEntity`]) or from a
//! [`SimulationMetrics`] aggregate, reusing the crate's existing types rather
//! than duplicating them.

use super::{BoundingBox, Color, Vec3, seed_position};
use crate::metrics::{SimulationMetrics, StatuteMetrics};
use crate::{SimResult, SimulationError};
use legalis_core::LegalEntity;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// The semantic role of a [`SceneNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    /// The synthetic root anchoring an aggregate scene.
    Origin,
    /// A single simulated entity / agent of the population.
    Entity,
    /// A statute applied during the simulation.
    Statute,
    /// A cluster grouping entities that share a categorical attribute.
    Cluster,
    /// A derived metric summary node.
    Metric,
}

impl NodeKind {
    /// A short, stable string tag (handy for exporters and tests).
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            NodeKind::Origin => "origin",
            NodeKind::Entity => "entity",
            NodeKind::Statute => "statute",
            NodeKind::Cluster => "cluster",
            NodeKind::Metric => "metric",
        }
    }
}

/// The semantic role of a [`SceneEdge`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeKind {
    /// An entity belongs to a cluster (cluster → entity).
    Member,
    /// A statute is applied within the simulation (origin → statute).
    Applies,
    /// A generic relationship between two entities.
    Relation,
    /// A node aggregates into a summary (origin → metric).
    Aggregates,
}

impl EdgeKind {
    /// A short, stable string tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            EdgeKind::Member => "member",
            EdgeKind::Applies => "applies",
            EdgeKind::Relation => "relation",
            EdgeKind::Aggregates => "aggregates",
        }
    }
}

/// Picks a default colour and size for a node kind.
fn default_style(kind: NodeKind) -> (Color, f64) {
    match kind {
        NodeKind::Origin => (Color::rgb(0x21, 0x25, 0x29), 2.2),
        NodeKind::Entity => (Color::rgb(0x21, 0x96, 0xf3), 0.6),
        NodeKind::Statute => (Color::rgb(0x34, 0x3a, 0x40), 1.4),
        NodeKind::Cluster => (Color::rgb(0x6f, 0x42, 0xc1), 1.2),
        NodeKind::Metric => (Color::rgb(0xfd, 0x7e, 0x14), 1.0),
    }
}

/// A node in a [`SimScene`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneNode {
    /// Stable, unique identifier within the scene.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Semantic role.
    pub kind: NodeKind,
    /// World-space position.
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
        let position = seed_position(&id, 8.0);
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

    /// Builder: overrides the position.
    #[must_use]
    pub fn at(mut self, position: Vec3) -> Self {
        self.position = position;
        self
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

/// An edge in a [`SimScene`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneEdge {
    /// Source node id.
    pub source: String,
    /// Target node id.
    pub target: String,
    /// Semantic role.
    pub kind: EdgeKind,
    /// Edge weight (higher = stronger relationship).
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
pub struct SimScene {
    nodes: Vec<SceneNode>,
    edges: Vec<SceneEdge>,
    #[serde(skip)]
    index: HashMap<String, usize>,
}

impl SimScene {
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

    /// Overwrites a node's position by id. Returns `true` if the node existed.
    pub fn set_position(&mut self, id: &str, position: Vec3) -> bool {
        if let Some(&i) = self.index.get(id) {
            self.nodes[i].position = position;
            true
        } else {
            false
        }
    }

    /// Ids of the nodes adjacent to `id` (edges in either direction), sorted for
    /// determinism.
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

    /// The number of nodes of a given kind.
    #[must_use]
    pub fn count_kind(&self, kind: NodeKind) -> usize {
        self.nodes.iter().filter(|n| n.kind == kind).count()
    }

    /// The bounding box enclosing every node position.
    #[must_use]
    pub fn bounds(&self) -> BoundingBox {
        BoundingBox::from_points(self.nodes.iter().map(|n| n.position))
    }

    /// Serialises the scene to pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::Serialization`] if serialisation fails.
    pub fn to_json(&self) -> SimResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Deserialises a scene from JSON, restoring its internal index.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::Serialization`] if the JSON is invalid.
    pub fn from_json(json: &str) -> SimResult<Self> {
        let mut scene: Self = serde_json::from_str(json)?;
        scene.reindex();
        Ok(scene)
    }
}

/// One mapped axis: an entity attribute linearly normalised to `[0, 1]` over the
/// inclusive range `[min, max]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeAxis {
    /// The attribute name to read from each entity.
    pub attribute: String,
    /// Lower bound of the attribute's expected range.
    pub min: f64,
    /// Upper bound of the attribute's expected range.
    pub max: f64,
}

impl AttributeAxis {
    /// Creates an axis, validating that `min < max`.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if the range is degenerate.
    pub fn new(attribute: impl Into<String>, min: f64, max: f64) -> SimResult<Self> {
        if !min.is_finite() || !max.is_finite() || min >= max {
            return Err(SimulationError::InvalidParameter(format!(
                "axis range must satisfy finite min < max, got [{min}, {max}]"
            )));
        }
        Ok(Self {
            attribute: attribute.into(),
            min,
            max,
        })
    }

    /// Normalises `value` into `[0, 1]` (clamped).
    #[must_use]
    pub fn normalize(&self, value: f64) -> f64 {
        ((value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }
}

/// Describes how a population's attributes drive a [`SimScene`].
///
/// Each spatial axis (`x`/`y`/`z`) may be bound to an [`AttributeAxis`]; an
/// unbound axis falls back to a deterministic per-entity seed so the population
/// still scatters legibly. An optional colour and size attribute grade nodes by
/// intensity, and an optional categorical `cluster_attribute` groups entities
/// under [`NodeKind::Cluster`] hubs linked by [`EdgeKind::Member`] edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationMapping {
    /// Attribute bound to the X axis.
    pub x: Option<AttributeAxis>,
    /// Attribute bound to the Y axis.
    pub y: Option<AttributeAxis>,
    /// Attribute bound to the Z axis.
    pub z: Option<AttributeAxis>,
    /// Attribute graded onto a heat colour.
    pub color: Option<AttributeAxis>,
    /// Attribute graded onto node size.
    pub size: Option<AttributeAxis>,
    /// Categorical attribute used to cluster entities.
    pub cluster_attribute: Option<String>,
    /// World-space half-extent each normalised axis is mapped into.
    pub extent: f64,
}

impl Default for PopulationMapping {
    fn default() -> Self {
        Self {
            x: AttributeAxis::new("age", 0.0, 100.0).ok(),
            y: AttributeAxis::new("income", 0.0, 200_000.0).ok(),
            z: None,
            color: None,
            size: None,
            cluster_attribute: None,
            extent: 10.0,
        }
    }
}

impl PopulationMapping {
    /// A mapping with no axes bound (pure deterministic scatter).
    #[must_use]
    pub fn scatter() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            color: None,
            size: None,
            cluster_attribute: None,
            extent: 10.0,
        }
    }

    /// Builder: binds the X axis.
    #[must_use]
    pub fn with_x(mut self, axis: AttributeAxis) -> Self {
        self.x = Some(axis);
        self
    }

    /// Builder: binds the Y axis.
    #[must_use]
    pub fn with_y(mut self, axis: AttributeAxis) -> Self {
        self.y = Some(axis);
        self
    }

    /// Builder: binds the Z axis.
    #[must_use]
    pub fn with_z(mut self, axis: AttributeAxis) -> Self {
        self.z = Some(axis);
        self
    }

    /// Builder: grades node colour by an attribute.
    #[must_use]
    pub fn with_color(mut self, axis: AttributeAxis) -> Self {
        self.color = Some(axis);
        self
    }

    /// Builder: grades node size by an attribute.
    #[must_use]
    pub fn with_size(mut self, axis: AttributeAxis) -> Self {
        self.size = Some(axis);
        self
    }

    /// Builder: clusters entities by a categorical attribute.
    #[must_use]
    pub fn with_cluster(mut self, attribute: impl Into<String>) -> Self {
        self.cluster_attribute = Some(attribute.into());
        self
    }

    /// The effective half-extent (never below 1.0 to keep scenes legible).
    fn effective_extent(&self) -> f64 {
        self.extent.max(1.0)
    }
}

/// Reads `attribute` from `entity` and parses it as `f64`, if present and numeric.
fn read_f64<E: LegalEntity>(entity: &E, attribute: &str) -> Option<f64> {
    entity
        .get_attribute(attribute)
        .and_then(|s| s.trim().parse::<f64>().ok())
        .filter(|v| v.is_finite())
}

/// Computes a single world coordinate for one axis of one entity.
///
/// When `axis` is bound and the attribute is numeric, the value is normalised and
/// mapped into `[-extent, extent]`. Otherwise a deterministic seed derived from
/// the entity id (and axis salt) is used so unmapped/missing values still scatter.
fn axis_coordinate(axis: Option<&AttributeAxis>, value: Option<f64>, seed: f64) -> f64 {
    match (axis, value) {
        (Some(axis), Some(v)) => (axis.normalize(v) - 0.5) * 2.0 * seed.abs().max(1.0),
        _ => seed,
    }
}

/// Builds a [`SimScene`] from a population of entities.
///
/// Each entity becomes a [`NodeKind::Entity`] node positioned by `mapping`. If
/// the mapping has a `cluster_attribute`, one [`NodeKind::Cluster`] hub is created
/// per distinct categorical value (positioned at its members' centroid) and each
/// entity is linked to its cluster.
///
/// # Examples
///
/// ```
/// use legalis_sim::immersive::{scene_from_entities, PopulationMapping};
/// use legalis_core::{BasicEntity, LegalEntity};
///
/// let mut a = BasicEntity::new();
/// a.set_attribute("age", "30".to_string());
/// a.set_attribute("region", "urban".to_string());
/// let mut b = BasicEntity::new();
/// b.set_attribute("age", "70".to_string());
/// b.set_attribute("region", "rural".to_string());
///
/// let mapping = PopulationMapping::default().with_cluster("region");
/// let scene = scene_from_entities(&[a, b], &mapping);
/// assert_eq!(scene.node_count(), 4); // 2 entities + 2 clusters
/// ```
#[must_use]
pub fn scene_from_entities<E: LegalEntity>(
    entities: &[E],
    mapping: &PopulationMapping,
) -> SimScene {
    let mut scene = SimScene::new();
    let extent = mapping.effective_extent();
    // Accumulate cluster member positions for centroid placement.
    let mut cluster_members: BTreeMap<String, Vec<Vec3>> = BTreeMap::new();

    for entity in entities {
        let id = entity.id().to_string();
        let seed = seed_position(&id, extent);
        let position = Vec3::new(
            axis_coordinate(
                mapping.x.as_ref(),
                mapping
                    .x
                    .as_ref()
                    .and_then(|a| read_f64(entity, &a.attribute)),
                seed.x,
            ),
            axis_coordinate(
                mapping.y.as_ref(),
                mapping
                    .y
                    .as_ref()
                    .and_then(|a| read_f64(entity, &a.attribute)),
                seed.y,
            ),
            axis_coordinate(
                mapping.z.as_ref(),
                mapping
                    .z
                    .as_ref()
                    .and_then(|a| read_f64(entity, &a.attribute)),
                seed.z,
            ),
        );

        let color = mapping
            .color
            .as_ref()
            .and_then(|axis| {
                read_f64(entity, &axis.attribute).map(|v| Color::heat(axis.normalize(v)))
            })
            .unwrap_or_else(|| default_style(NodeKind::Entity).0);

        let size = mapping
            .size
            .as_ref()
            .and_then(|axis| {
                read_f64(entity, &axis.attribute).map(|v| 0.4 + axis.normalize(v) * 1.6)
            })
            .unwrap_or_else(|| default_style(NodeKind::Entity).1);

        let mut node = SceneNode::new(id.clone(), short_label(&id), NodeKind::Entity)
            .at(position)
            .with_color(color)
            .with_size(size);

        if let Some(cluster_attr) = &mapping.cluster_attribute
            && let Some(value) = entity.get_attribute(cluster_attr)
        {
            node = node.with_meta(cluster_attr.clone(), value.clone());
            cluster_members.entry(value).or_default().push(position);
        }
        scene.add_node(node);
    }

    // Build cluster hubs and member edges.
    if let Some(cluster_attr) = &mapping.cluster_attribute {
        for (value, positions) in &cluster_members {
            let centroid = centroid(positions);
            let cluster_id = format!("cluster::{cluster_attr}::{value}");
            scene.add_node(
                SceneNode::new(cluster_id.clone(), value.clone(), NodeKind::Cluster)
                    .at(centroid)
                    .with_size(1.0 + (positions.len() as f64).sqrt() * 0.3)
                    .with_meta("members", positions.len().to_string()),
            );
        }
        for entity in entities {
            if let Some(value) = entity.get_attribute(cluster_attr) {
                let cluster_id = format!("cluster::{cluster_attr}::{value}");
                scene.add_edge(SceneEdge::new(
                    cluster_id,
                    entity.id().to_string(),
                    EdgeKind::Member,
                ));
            }
        }
    }

    scene
}

/// Builds a [`SimScene`] from aggregate [`SimulationMetrics`].
///
/// An [`NodeKind::Origin`] root is always created. Each statute becomes a
/// [`NodeKind::Statute`] node placed by `(effectiveness, ambiguity, volume)`,
/// coloured by its ambiguity (heat), sized by application volume, and linked to
/// the origin by an [`EdgeKind::Applies`] edge.
#[must_use]
pub fn scene_from_metrics(metrics: &SimulationMetrics) -> SimScene {
    let mut scene = SimScene::new();
    scene.add_node(
        SceneNode::new("origin", "Simulation", NodeKind::Origin)
            .at(Vec3::zero())
            .with_meta("applications", metrics.total_applications.to_string())
            .with_meta("statutes", metrics.statute_metrics.len().to_string()),
    );

    let max_total = metrics
        .statute_metrics
        .values()
        .map(|m| m.total)
        .max()
        .unwrap_or(0)
        .max(1) as f64;

    // Sort by id for deterministic placement.
    let mut entries: Vec<(&String, &StatuteMetrics)> = metrics.statute_metrics.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    for (statute_id, sm) in entries {
        let effectiveness = sm.effectiveness();
        let ambiguity = sm.ambiguity();
        let volume = sm.total as f64 / max_total;
        let position = Vec3::new(
            (effectiveness - 0.5) * 16.0,
            (volume - 0.5) * 12.0,
            (ambiguity - 0.5) * 16.0,
        );
        let node_id = format!("statute::{statute_id}");
        scene.add_node(
            SceneNode::new(node_id.clone(), statute_id.clone(), NodeKind::Statute)
                .at(position)
                .with_color(Color::heat(ambiguity))
                .with_size(0.8 + volume * 1.8)
                .with_meta("total", sm.total.to_string())
                .with_meta("deterministic", sm.deterministic.to_string())
                .with_meta("discretion", sm.discretion.to_string())
                .with_meta("void", sm.void.to_string()),
        );
        scene.add_edge(
            SceneEdge::new("origin", node_id, EdgeKind::Applies).with_weight(0.5 + volume),
        );
    }

    scene
}

/// Mean of a set of positions (origin if empty).
fn centroid(points: &[Vec3]) -> Vec3 {
    if points.is_empty() {
        return Vec3::zero();
    }
    let sum = points.iter().fold(Vec3::zero(), |acc, &p| acc + p);
    sum.scale(1.0 / points.len() as f64)
}

/// A compact label for a long id (e.g. a UUID) — first 8 characters.
fn short_label(id: &str) -> String {
    id.chars().take(8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{BasicEntity, EffectType, LegalResult};

    fn entity(age: &str, region: &str) -> BasicEntity {
        let mut e = BasicEntity::new();
        e.set_attribute("age", age.to_string());
        e.set_attribute("region", region.to_string());
        e
    }

    fn sample_metrics() -> SimulationMetrics {
        use crate::engine::LawApplicationResult;
        use legalis_core::Effect;
        let mut m = SimulationMetrics::new();
        for _ in 0..6 {
            m.record_result(&LawApplicationResult {
                agent_id: uuid::Uuid::new_v4(),
                statute_id: "tax-credit".to_string(),
                result: LegalResult::Deterministic(Effect::new(EffectType::Grant, "ok")),
            });
        }
        for _ in 0..2 {
            m.record_result(&LawApplicationResult {
                agent_id: uuid::Uuid::new_v4(),
                statute_id: "vague-rule".to_string(),
                result: LegalResult::JudicialDiscretion {
                    issue: "ambiguous".to_string(),
                    context_id: uuid::Uuid::new_v4(),
                    narrative_hint: None,
                },
            });
        }
        m
    }

    #[test]
    fn test_scene_from_entities_scatter() {
        let pop = vec![
            entity("30", "urban"),
            entity("65", "rural"),
            entity("45", "urban"),
        ];
        let scene = scene_from_entities(&pop, &PopulationMapping::default());
        assert_eq!(scene.node_count(), 3);
        assert_eq!(scene.count_kind(NodeKind::Entity), 3);
        // Bound axes keep coordinates within the mapped extent.
        for node in scene.nodes() {
            assert!(node.position.x.abs() <= 10.0 + 1e-9);
        }
    }

    #[test]
    fn test_scene_from_entities_clusters() {
        let pop = vec![
            entity("30", "urban"),
            entity("65", "rural"),
            entity("45", "urban"),
        ];
        let mapping = PopulationMapping::default().with_cluster("region");
        let scene = scene_from_entities(&pop, &mapping);
        // 3 entities + 2 cluster hubs (urban, rural).
        assert_eq!(scene.node_count(), 5);
        assert_eq!(scene.count_kind(NodeKind::Cluster), 2);
        // Member edges link each entity to a cluster.
        assert_eq!(scene.edge_count(), 3);
        let urban = scene.node("cluster::region::urban").expect("urban cluster");
        assert_eq!(urban.metadata.get("members").map(String::as_str), Some("2"));
    }

    #[test]
    fn test_color_and_size_axes_grade_nodes() {
        let mut young = BasicEntity::new();
        young.set_attribute("age", "10".to_string());
        let mut old = BasicEntity::new();
        old.set_attribute("age", "90".to_string());
        let mapping = PopulationMapping::scatter()
            .with_color(AttributeAxis::new("age", 0.0, 100.0).unwrap())
            .with_size(AttributeAxis::new("age", 0.0, 100.0).unwrap());
        let scene = scene_from_entities(&[young, old], &mapping);
        let nodes = scene.nodes();
        // Higher age → larger, warmer (redder) node.
        let young_node = &nodes[0];
        let old_node = &nodes[1];
        assert!(old_node.size > young_node.size);
        assert!(old_node.color.r >= young_node.color.r);
    }

    #[test]
    fn test_scene_from_metrics_builds_origin_and_statutes() {
        let scene = scene_from_metrics(&sample_metrics());
        assert!(scene.node("origin").is_some());
        assert_eq!(scene.count_kind(NodeKind::Statute), 2);
        // Both statutes connect to the origin.
        assert_eq!(scene.degree("origin"), 2);
        let vague = scene.node("statute::vague-rule").expect("vague statute");
        // Fully discretionary statute is graded toward the hot end.
        assert!(vague.color.r > vague.color.b);
    }

    #[test]
    fn test_empty_metrics_scene_has_only_origin() {
        let scene = scene_from_metrics(&SimulationMetrics::new());
        assert_eq!(scene.node_count(), 1);
        assert_eq!(scene.count_kind(NodeKind::Origin), 1);
        assert_eq!(scene.edge_count(), 0);
    }

    #[test]
    fn test_scene_graph_invariants_and_json_roundtrip() {
        let scene = scene_from_metrics(&sample_metrics());
        // Self-loops and dangling edges rejected.
        let mut s = scene.clone();
        assert!(!s.add_edge(SceneEdge::new("origin", "origin", EdgeKind::Applies)));
        assert!(!s.add_edge(SceneEdge::new("origin", "ghost", EdgeKind::Applies)));
        // Re-adding an id replaces in place.
        assert!(!s.add_node(SceneNode::new("origin", "Renamed", NodeKind::Origin)));
        assert_eq!(
            s.node("origin").map(|n| n.label.clone()),
            Some("Renamed".to_string())
        );
        // JSON round-trip restores the index (lookups work).
        let json = scene.to_json().expect("serialise");
        let restored = SimScene::from_json(&json).expect("deserialise");
        assert_eq!(restored.node_count(), scene.node_count());
        assert!(restored.node("origin").is_some());
    }

    #[test]
    fn test_axis_validation_and_normalize() {
        assert!(AttributeAxis::new("a", 1.0, 0.0).is_err());
        assert!(AttributeAxis::new("a", 0.0, 0.0).is_err());
        let axis = AttributeAxis::new("a", 0.0, 10.0).unwrap();
        assert!((axis.normalize(5.0) - 0.5).abs() < 1e-9);
        assert!((axis.normalize(-3.0) - 0.0).abs() < 1e-9);
        assert!((axis.normalize(99.0) - 1.0).abs() < 1e-9);
    }
}
