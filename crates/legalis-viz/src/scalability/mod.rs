//! Performance and scalability tooling for very large dependency graphs.
//!
//! Rendering, exporting and laying out statute dependency graphs becomes
//! prohibitively expensive once the node count grows into the tens of
//! thousands. This module bundles a set of cooperating, dependency-free
//! algorithms that keep such graphs interactive:
//!
//! - [`IncrementalRenderer`] performs viewport-windowed rendering with a
//!   dirty-region / diff model so that only nodes that actually changed are
//!   re-emitted (see [`incremental`]).
//! - [`GraphSimplifier`] reduces a graph through leaf pruning, degree-two chain
//!   contraction, importance-based filtering and modularity-preserving
//!   coarsening (see [`simplification`]).
//! - [`NodeClusterer`] groups nodes via union-find connected components, label
//!   propagation community detection and spatial k-means (see [`clustering`]).
//! - [`LevelOfDetailEngine`] applies an adaptive, budget- and zoom-driven
//!   level-of-detail with representative aggregation (see [`lod`]).
//! - [`CompactGraph`] / [`StringInterner`] provide an arena and CSR based
//!   memory-optimised representation with streaming emit (see [`memory`]).
//!
//! Every algorithm operates directly on the crate's existing
//! [`DependencyGraph`](crate::types_4::DependencyGraph) and reuses its existing
//! rendering surface through [`SimplifiedGraph::to_dependency_graph`].

mod clustering;
mod incremental;
mod lod;
mod memory;
mod simplification;

pub use clustering::{Cluster, ClusterAssignment, NodeClusterer, UnionFind};
pub use incremental::{IncrementalRenderer, NodeRender, Rect, RenderDiff, RenderState, Viewport};
pub use lod::{DetailLevel, LevelOfDetailEngine, LodConfig, LodView, RepresentativeNode};
pub use memory::{CompactGraph, CompactGraphBuilder, MemoryFootprint, StringInterner, Symbol};
pub use simplification::{GraphSimplifier, SimplifiedEdge, SimplifiedGraph, SuperNode};

use crate::types_4::DependencyGraph;
use std::collections::HashMap;

/// Default spacing (in world units) used by the deterministic grid layout when
/// the graph's [`LayoutConfig`](crate::types_10::LayoutConfig) leaves it unset.
pub(crate) const DEFAULT_GRID_SPACING: f64 = 80.0;

/// Computes deterministic 2D layout positions for every node using a square
/// grid sized from the node count.
///
/// The layout is purely a function of node order and the graph's configured
/// `node_spacing`, which makes it stable across calls. Stability is essential
/// for the incremental renderer's diff model: a node only counts as "moved"
/// when the underlying graph actually changes, never because of layout jitter.
///
/// Returns `(id, x, y)` triples in graph node order.
pub(crate) fn grid_layout_positions(graph: &DependencyGraph) -> Vec<(String, f64, f64)> {
    let count = graph.graph.node_count();
    if count == 0 {
        return Vec::new();
    }
    let spacing = if graph.layout_config.node_spacing == 0 {
        DEFAULT_GRID_SPACING
    } else {
        graph.layout_config.node_spacing as f64
    };
    let cols = ((count as f64).sqrt().ceil() as usize).max(1);
    let mut positions = Vec::with_capacity(count);
    for (order, idx) in graph.graph.node_indices().enumerate() {
        let col = order % cols;
        let row = order / cols;
        let x = (col as f64 + 1.0) * spacing;
        let y = (row as f64 + 1.0) * spacing;
        let id = graph.graph.node_weight(idx).cloned().unwrap_or_default();
        positions.push((id, x, y));
    }
    positions
}

/// Computes the total incident-edge degree (in + out) for every node id.
///
/// Isolated nodes are included with a degree of zero so that callers can rely
/// on the map covering the entire vertex set.
pub(crate) fn degree_map(graph: &DependencyGraph) -> HashMap<String, usize> {
    let mut degrees: HashMap<String, usize> = HashMap::with_capacity(graph.graph.node_count());
    for idx in graph.graph.node_indices() {
        if let Some(id) = graph.graph.node_weight(idx) {
            degrees.entry(id.clone()).or_insert(0);
        }
    }
    for edge in graph.graph.edge_indices() {
        if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
            if let Some(sid) = graph.graph.node_weight(source) {
                *degrees.entry(sid.clone()).or_insert(0) += 1;
            }
            if let Some(tid) = graph.graph.node_weight(target) {
                *degrees.entry(tid.clone()).or_insert(0) += 1;
            }
        }
    }
    degrees
}

/// Picks a representative id from `members`, preferring the highest-degree
/// member and breaking ties by smallest id for determinism.
///
/// Shared by clustering aggregation, level-of-detail and coarsening so that the
/// chosen representative is consistent regardless of which feature requested it.
pub(crate) fn representative_member<'a>(
    members: &'a [String],
    degrees: &HashMap<String, usize>,
) -> Option<&'a String> {
    members.iter().reduce(|best, candidate| {
        let best_degree = degrees.get(best).copied().unwrap_or(0);
        let candidate_degree = degrees.get(candidate).copied().unwrap_or(0);
        if candidate_degree > best_degree || (candidate_degree == best_degree && candidate < best) {
            candidate
        } else {
            best
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types_4::DependencyGraph;

    fn line_graph() -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "depends_on");
        graph.add_dependency("b", "c", "depends_on");
        graph
    }

    #[test]
    fn grid_layout_is_deterministic_and_covers_all_nodes() {
        let graph = line_graph();
        let first = grid_layout_positions(&graph);
        let second = grid_layout_positions(&graph);
        assert_eq!(first, second);
        assert_eq!(first.len(), graph.node_count());
        // Positions are strictly positive and spread across the grid.
        assert!(first.iter().all(|(_, x, y)| *x > 0.0 && *y > 0.0));
    }

    #[test]
    fn grid_layout_empty_graph_is_empty() {
        let graph = DependencyGraph::new();
        assert!(grid_layout_positions(&graph).is_empty());
    }

    #[test]
    fn degree_map_counts_incident_edges_and_includes_isolated() {
        let mut graph = line_graph();
        graph.add_statute("isolated");
        let degrees = degree_map(&graph);
        assert_eq!(degrees.get("a"), Some(&1));
        assert_eq!(degrees.get("b"), Some(&2));
        assert_eq!(degrees.get("c"), Some(&1));
        assert_eq!(degrees.get("isolated"), Some(&0));
    }

    #[test]
    fn representative_prefers_highest_degree_then_smallest_id() {
        let mut degrees = HashMap::new();
        degrees.insert("a".to_string(), 1usize);
        degrees.insert("b".to_string(), 3usize);
        degrees.insert("c".to_string(), 3usize);
        let members = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        // b and c tie on degree, b wins on smaller id.
        assert_eq!(
            representative_member(&members, &degrees),
            Some(&"b".to_string())
        );
    }
}
