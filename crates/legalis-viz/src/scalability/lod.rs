//! Adaptive level-of-detail (LOD) for dependency graphs.
//!
//! When a graph exceeds the on-screen node budget, showing every node produces
//! an unreadable hairball. The [`LevelOfDetailEngine`] adapts the rendered
//! detail to the current zoom and a node budget:
//!
//! - **Full** detail (`zoom` above threshold, or the graph already fits the
//!   budget): every node is shown as its own representative.
//! - **Reduced** detail: the graph is clustered into `budget` spatial groups
//!   and each group is aggregated into a single [`RepresentativeNode`].
//! - **Overview** detail (very large graphs): the same aggregation with a
//!   tighter cluster count for a bird's-eye view.
//!
//! Representative selection reuses the crate's shared degree-based heuristic,
//! and edges are aggregated between representatives. The result reuses
//! [`SimplifiedEdge`] and converts back to a [`DependencyGraph`] for rendering.

use super::clustering::NodeClusterer;
use super::simplification::SimplifiedEdge;
use super::{degree_map, representative_member};
use crate::functions::VizResult;
use crate::types_4::DependencyGraph;
use crate::types_5::VizError;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// The detail level chosen for a given zoom and graph size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetailLevel {
    /// Every node is shown individually.
    Full,
    /// Nodes are aggregated to fit the budget.
    Reduced,
    /// Aggressive aggregation for a high-level overview.
    Overview,
}

/// Configuration controlling adaptive level-of-detail.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LodConfig {
    /// Maximum number of nodes to show at reduced detail.
    pub node_budget: usize,
    /// Current client zoom factor.
    pub zoom: f64,
    /// Zoom at or above which full detail is always shown.
    pub detail_zoom_threshold: f64,
    /// Graphs larger than `node_budget * overview_multiplier` switch to
    /// [`DetailLevel::Overview`].
    pub overview_multiplier: usize,
}

impl LodConfig {
    /// Creates a configuration with the given node budget.
    ///
    /// Returns [`VizError::InvalidStructure`] when `node_budget` is zero.
    pub fn new(node_budget: usize) -> VizResult<Self> {
        if node_budget == 0 {
            return Err(VizError::InvalidStructure(
                "node_budget must be greater than zero".to_string(),
            ));
        }
        Ok(Self {
            node_budget,
            zoom: 1.0,
            detail_zoom_threshold: 2.0,
            overview_multiplier: 8,
        })
    }

    /// Sets the current zoom factor.
    pub fn with_zoom(mut self, zoom: f64) -> Self {
        self.zoom = zoom;
        self
    }

    /// Sets the zoom threshold above which full detail is shown.
    pub fn with_detail_threshold(mut self, threshold: f64) -> Self {
        self.detail_zoom_threshold = threshold;
        self
    }

    /// Sets the overview multiplier.
    pub fn with_overview_multiplier(mut self, multiplier: usize) -> Self {
        self.overview_multiplier = multiplier;
        self
    }
}

/// A node in a level-of-detail view, representing one or more originals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepresentativeNode {
    /// Representative statute id.
    pub id: String,
    /// Original ids represented (sorted, includes `id`).
    pub represents: Vec<String>,
    /// Importance score used to pick the representative.
    pub importance: f64,
}

impl RepresentativeNode {
    /// Number of original nodes folded into this representative.
    pub fn represented_count(&self) -> usize {
        self.represents.len()
    }
}

/// The adaptive view computed for a given [`LodConfig`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LodView {
    /// The detail level applied.
    pub level: DetailLevel,
    /// Visible representative nodes.
    pub nodes: Vec<RepresentativeNode>,
    /// Aggregated edges between representatives.
    pub edges: Vec<SimplifiedEdge>,
    /// Number of original nodes hidden behind representatives.
    pub hidden_node_count: usize,
    /// The node budget that produced this view.
    pub budget: usize,
}

impl LodView {
    /// Number of visible representative nodes.
    pub fn visible_node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Rebuilds a [`DependencyGraph`] from the representatives so the crate's
    /// existing renderers can draw the LOD view directly.
    pub fn to_dependency_graph(&self) -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        for node in &self.nodes {
            graph.add_statute(&node.id);
        }
        for edge in &self.edges {
            graph.add_dependency(&edge.from, &edge.to, &edge.relation);
        }
        graph
    }
}

/// Computes adaptive level-of-detail views of a [`DependencyGraph`].
#[derive(Debug, Clone, Default)]
pub struct LevelOfDetailEngine {
    clusterer: NodeClusterer,
}

impl LevelOfDetailEngine {
    /// Creates a new engine.
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines the detail level for a node count under a configuration.
    pub fn level_for(&self, node_count: usize, config: &LodConfig) -> DetailLevel {
        if config.zoom >= config.detail_zoom_threshold || node_count <= config.node_budget {
            DetailLevel::Full
        } else if node_count
            > config
                .node_budget
                .saturating_mul(config.overview_multiplier.max(1))
        {
            DetailLevel::Overview
        } else {
            DetailLevel::Reduced
        }
    }

    /// Applies adaptive level-of-detail, returning the resulting [`LodView`].
    pub fn apply(&self, graph: &DependencyGraph, config: &LodConfig) -> VizResult<LodView> {
        let node_count = graph.node_count();
        if node_count == 0 {
            return Ok(LodView {
                level: DetailLevel::Overview,
                nodes: Vec::new(),
                edges: Vec::new(),
                hidden_node_count: 0,
                budget: config.node_budget,
            });
        }
        let level = self.level_for(node_count, config);
        let degrees = degree_map(graph);
        match level {
            DetailLevel::Full => Ok(self.full_view(graph, &degrees, config.node_budget)),
            DetailLevel::Reduced | DetailLevel::Overview => {
                let target_clusters = if level == DetailLevel::Overview {
                    (config.node_budget / 4).max(1)
                } else {
                    config.node_budget
                };
                let k = target_clusters.min(node_count).max(1);
                let assignment = self.clusterer.kmeans_layout(graph, k, 50)?;
                Ok(self.aggregate_view(
                    graph,
                    &assignment.as_communities(),
                    &degrees,
                    level,
                    config,
                ))
            }
        }
    }

    fn full_view(
        &self,
        graph: &DependencyGraph,
        degrees: &HashMap<String, usize>,
        budget: usize,
    ) -> LodView {
        let mut nodes: Vec<RepresentativeNode> = graph
            .graph
            .node_indices()
            .filter_map(|idx| graph.graph.node_weight(idx))
            .map(|id| RepresentativeNode {
                id: id.clone(),
                represents: vec![id.clone()],
                importance: degrees.get(id).copied().unwrap_or(0) as f64,
            })
            .collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        let mut edge_acc: BTreeMap<(String, String), (f64, String)> = BTreeMap::new();
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge)
                && let (Some(sid), Some(tid)) = (
                    graph.graph.node_weight(source),
                    graph.graph.node_weight(target),
                )
            {
                let relation = graph.graph.edge_weight(edge).cloned().unwrap_or_default();
                let entry = edge_acc
                    .entry((sid.clone(), tid.clone()))
                    .or_insert((0.0, relation));
                entry.0 += 1.0;
            }
        }
        LodView {
            level: DetailLevel::Full,
            nodes,
            edges: edges_from_accumulator(edge_acc),
            hidden_node_count: 0,
            budget,
        }
    }

    fn aggregate_view(
        &self,
        graph: &DependencyGraph,
        communities: &[Vec<String>],
        degrees: &HashMap<String, usize>,
        level: DetailLevel,
        config: &LodConfig,
    ) -> LodView {
        let mut id_to_rep: HashMap<String, String> = HashMap::new();
        let mut nodes: Vec<RepresentativeNode> = Vec::with_capacity(communities.len());
        for members in communities {
            if members.is_empty() {
                continue;
            }
            let mut sorted = members.clone();
            sorted.sort();
            let rep = representative_member(&sorted, degrees)
                .cloned()
                .unwrap_or_else(|| sorted[0].clone());
            for member in &sorted {
                id_to_rep.insert(member.clone(), rep.clone());
            }
            let importance = degrees.get(&rep).copied().unwrap_or(0) as f64;
            nodes.push(RepresentativeNode {
                id: rep,
                represents: sorted,
                importance,
            });
        }
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        let mut edge_acc: BTreeMap<(String, String), (f64, String)> = BTreeMap::new();
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge)
                && let (Some(sid), Some(tid)) = (
                    graph.graph.node_weight(source),
                    graph.graph.node_weight(target),
                )
                && let (Some(rep_source), Some(rep_target)) =
                    (id_to_rep.get(sid), id_to_rep.get(tid))
                && rep_source != rep_target
            {
                let relation = graph.graph.edge_weight(edge).cloned().unwrap_or_default();
                let entry = edge_acc
                    .entry((rep_source.clone(), rep_target.clone()))
                    .or_insert((0.0, relation));
                entry.0 += 1.0;
            }
        }
        let visible = nodes.len();
        LodView {
            level,
            nodes,
            edges: edges_from_accumulator(edge_acc),
            hidden_node_count: graph.node_count().saturating_sub(visible),
            budget: config.node_budget,
        }
    }
}

fn edges_from_accumulator(
    accumulator: BTreeMap<(String, String), (f64, String)>,
) -> Vec<SimplifiedEdge> {
    accumulator
        .into_iter()
        .map(|((from, to), (weight, relation))| SimplifiedEdge {
            from,
            to,
            relation,
            weight,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph_with(count: usize) -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        for i in 0..count {
            graph.add_statute(&format!("n{i:03}"));
        }
        // Chain the nodes so there are edges to aggregate.
        for i in 0..count.saturating_sub(1) {
            graph.add_dependency(&format!("n{i:03}"), &format!("n{:03}", i + 1), "depends_on");
        }
        graph
    }

    #[test]
    fn config_rejects_zero_budget() {
        assert!(LodConfig::new(0).is_err());
        assert!(LodConfig::new(10).is_ok());
    }

    #[test]
    fn small_graph_uses_full_detail() {
        let engine = LevelOfDetailEngine::new();
        let config = LodConfig::new(50).expect("config");
        let view = engine.apply(&graph_with(20), &config).expect("view");
        assert_eq!(view.level, DetailLevel::Full);
        assert_eq!(view.visible_node_count(), 20);
        assert_eq!(view.hidden_node_count, 0);
    }

    #[test]
    fn high_zoom_forces_full_detail() {
        let engine = LevelOfDetailEngine::new();
        let config = LodConfig::new(10).expect("config").with_zoom(3.0);
        // 40 nodes would normally reduce, but high zoom keeps full detail.
        let view = engine.apply(&graph_with(40), &config).expect("view");
        assert_eq!(view.level, DetailLevel::Full);
        assert_eq!(view.visible_node_count(), 40);
    }

    #[test]
    fn large_graph_reduces_to_budget() {
        let engine = LevelOfDetailEngine::new();
        let config = LodConfig::new(8).expect("config").with_zoom(1.0);
        let view = engine.apply(&graph_with(60), &config).expect("view");
        assert_eq!(view.level, DetailLevel::Reduced);
        assert!(view.visible_node_count() <= 8);
        assert!(view.hidden_node_count > 0);
        // Representatives cover the whole graph.
        let covered: usize = view
            .nodes
            .iter()
            .map(RepresentativeNode::represented_count)
            .sum();
        assert_eq!(covered, 60);
    }

    #[test]
    fn very_large_graph_uses_overview() {
        let engine = LevelOfDetailEngine::new();
        let config = LodConfig::new(5).expect("config").with_zoom(1.0);
        // 5 * 8 = 40 threshold; 100 nodes -> Overview.
        let view = engine.apply(&graph_with(100), &config).expect("view");
        assert_eq!(view.level, DetailLevel::Overview);
        assert!(view.visible_node_count() <= 5);
    }

    #[test]
    fn empty_graph_yields_empty_view() {
        let engine = LevelOfDetailEngine::new();
        let config = LodConfig::new(10).expect("config");
        let view = engine
            .apply(&DependencyGraph::new(), &config)
            .expect("view");
        assert_eq!(view.visible_node_count(), 0);
        assert_eq!(view.hidden_node_count, 0);
    }

    #[test]
    fn lod_view_round_trips_to_dependency_graph() {
        let engine = LevelOfDetailEngine::new();
        let config = LodConfig::new(6).expect("config");
        let view = engine.apply(&graph_with(50), &config).expect("view");
        let rebuilt = view.to_dependency_graph();
        assert_eq!(rebuilt.node_count(), view.visible_node_count());
    }

    #[test]
    fn level_for_thresholds() {
        let engine = LevelOfDetailEngine::new();
        let config = LodConfig::new(10).expect("config");
        assert_eq!(engine.level_for(5, &config), DetailLevel::Full);
        assert_eq!(engine.level_for(50, &config), DetailLevel::Reduced);
        assert_eq!(engine.level_for(500, &config), DetailLevel::Overview);
    }
}
