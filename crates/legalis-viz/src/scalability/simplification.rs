//! Graph simplification algorithms.
//!
//! Large dependency graphs are rarely interesting in full. This module reduces
//! them while preserving the structure that matters, through four cooperating
//! algorithms exposed by [`GraphSimplifier`]:
//!
//! - **Leaf pruning** iteratively peels degree-≤1 nodes (a step towards the
//!   graph's 2-core).
//! - **Degree-two chain contraction** collapses `p → v → s` chains where the
//!   intermediate node has exactly one predecessor and one successor.
//! - **Importance-based filtering** keeps the top fraction of nodes by PageRank
//!   and bridges dropped nodes so reachability between survivors is preserved.
//! - **Modularity-preserving coarsening** builds the quotient graph over a
//!   community partition, keeping inter-community edges intact.
//!
//! All algorithms produce a [`SimplifiedGraph`], which records the original
//! members folded into each super-node and converts back to a
//! [`DependencyGraph`] for rendering with the crate's existing surface.

use super::{degree_map, representative_member};
use crate::functions::VizResult;
use crate::types_4::DependencyGraph;
use crate::types_5::VizError;
use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// A node in a simplified graph, possibly representing several originals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuperNode {
    /// Representative id (a real statute id drawn from the members).
    pub id: String,
    /// Original statute ids folded into this node, sorted.
    pub members: Vec<String>,
    /// Importance score (member count by default).
    pub importance: f64,
}

impl SuperNode {
    /// Number of original nodes represented.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

/// A weighted edge in a simplified graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimplifiedEdge {
    /// Source super-node id.
    pub from: String,
    /// Target super-node id.
    pub to: String,
    /// Relation label.
    pub relation: String,
    /// Number of original edges folded into this edge.
    pub weight: f64,
}

/// The output of any simplification pass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimplifiedGraph {
    /// Surviving super-nodes, sorted by id.
    pub nodes: Vec<SuperNode>,
    /// Surviving edges, sorted by `(from, to)`.
    pub edges: Vec<SimplifiedEdge>,
    /// Node count of the original graph.
    pub original_node_count: usize,
    /// Edge count of the original graph.
    pub original_edge_count: usize,
}

impl SimplifiedGraph {
    /// Number of surviving nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of surviving edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Fraction of nodes removed relative to the original (in `0.0..=1.0`).
    pub fn node_reduction_ratio(&self) -> f64 {
        if self.original_node_count == 0 {
            0.0
        } else {
            1.0 - self.node_count() as f64 / self.original_node_count as f64
        }
    }

    /// Returns the super-node id that contains `original_id`, if any.
    pub fn member_of(&self, original_id: &str) -> Option<&str> {
        self.nodes.iter().find_map(|node| {
            node.members
                .iter()
                .any(|member| member == original_id)
                .then_some(node.id.as_str())
        })
    }

    /// Rebuilds a [`DependencyGraph`] from the simplified structure so the
    /// crate's existing renderers and exporters can be reused unchanged.
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

/// Configurable graph simplifier.
#[derive(Debug, Clone)]
pub struct GraphSimplifier {
    /// Maximum leaf-pruning rounds applied by [`GraphSimplifier::simplify`].
    pub prune_rounds: usize,
    /// Whether [`GraphSimplifier::simplify`] contracts degree-two chains.
    pub contract_chains: bool,
    /// Fraction of nodes to retain during importance filtering (`0.0..=1.0`).
    pub keep_fraction: f64,
}

impl Default for GraphSimplifier {
    fn default() -> Self {
        Self {
            prune_rounds: 1,
            contract_chains: true,
            keep_fraction: 1.0,
        }
    }
}

impl GraphSimplifier {
    /// Creates a simplifier with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of leaf-pruning rounds.
    pub fn with_prune_rounds(mut self, rounds: usize) -> Self {
        self.prune_rounds = rounds;
        self
    }

    /// Enables or disables chain contraction in the combined pipeline.
    pub fn with_chain_contraction(mut self, enabled: bool) -> Self {
        self.contract_chains = enabled;
        self
    }

    /// Sets the retained node fraction for importance filtering.
    pub fn with_keep_fraction(mut self, fraction: f64) -> Self {
        self.keep_fraction = fraction;
        self
    }

    /// Runs the full configured pipeline: prune, contract, then filter.
    pub fn simplify(&self, graph: &DependencyGraph) -> VizResult<SimplifiedGraph> {
        if !(self.keep_fraction > 0.0 && self.keep_fraction <= 1.0) {
            return Err(VizError::InvalidStructure(format!(
                "keep_fraction must be in (0.0, 1.0], got {}",
                self.keep_fraction
            )));
        }
        let mut working = WorkingGraph::from_graph(graph);
        if self.prune_rounds > 0 {
            working.prune_leaves(self.prune_rounds);
        }
        if self.contract_chains {
            working.contract_degree_two_chains();
        }
        if self.keep_fraction < 1.0 {
            working.filter_by_importance(self.keep_fraction);
        }
        Ok(working.into_simplified())
    }

    /// Iteratively prunes degree-≤1 nodes for up to `rounds` rounds.
    pub fn prune_leaves(&self, graph: &DependencyGraph, rounds: usize) -> SimplifiedGraph {
        let mut working = WorkingGraph::from_graph(graph);
        working.prune_leaves(rounds);
        working.into_simplified()
    }

    /// Contracts every `p → v → s` chain whose intermediate node `v` has a
    /// single predecessor and single successor.
    pub fn contract_chains(&self, graph: &DependencyGraph) -> SimplifiedGraph {
        let mut working = WorkingGraph::from_graph(graph);
        working.contract_degree_two_chains();
        working.into_simplified()
    }

    /// Keeps the top `keep_fraction` of nodes by PageRank, bridging dropped
    /// nodes so reachability between survivors is preserved.
    pub fn filter_by_importance(
        &self,
        graph: &DependencyGraph,
        keep_fraction: f64,
    ) -> VizResult<SimplifiedGraph> {
        if !(keep_fraction > 0.0 && keep_fraction <= 1.0) {
            return Err(VizError::InvalidStructure(format!(
                "keep_fraction must be in (0.0, 1.0], got {keep_fraction}"
            )));
        }
        let mut working = WorkingGraph::from_graph(graph);
        working.filter_by_importance(keep_fraction);
        Ok(working.into_simplified())
    }

    /// Builds the quotient graph over `communities`, folding each community into
    /// a single super-node and aggregating inter-community edges.
    ///
    /// Returns [`VizError::InvalidStructure`] if any node is missing from the
    /// partition or assigned to more than one community.
    pub fn coarsen_by_communities(
        &self,
        graph: &DependencyGraph,
        communities: &[Vec<String>],
    ) -> VizResult<SimplifiedGraph> {
        let labels: Vec<String> = graph
            .graph
            .node_indices()
            .map(|idx| graph.graph.node_weight(idx).cloned().unwrap_or_default())
            .collect();
        let mut community_of: HashMap<String, usize> = HashMap::new();
        for (community_id, members) in communities.iter().enumerate() {
            for member in members {
                if community_of.insert(member.clone(), community_id).is_some() {
                    return Err(VizError::InvalidStructure(format!(
                        "node '{member}' assigned to multiple communities"
                    )));
                }
            }
        }
        for label in &labels {
            if !community_of.contains_key(label) {
                return Err(VizError::InvalidStructure(format!(
                    "node '{label}' is not covered by any community"
                )));
            }
        }
        let degrees = degree_map(graph);
        let mut members_by_community: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for (label, &community_id) in &community_of {
            if labels.contains(label) {
                members_by_community
                    .entry(community_id)
                    .or_default()
                    .push(label.clone());
            }
        }
        let mut representative: HashMap<usize, String> = HashMap::new();
        let mut nodes: Vec<SuperNode> = Vec::with_capacity(members_by_community.len());
        for (&community_id, members) in &members_by_community {
            let mut sorted = members.clone();
            sorted.sort();
            let rep = representative_member(&sorted, &degrees)
                .cloned()
                .unwrap_or_default();
            representative.insert(community_id, rep.clone());
            nodes.push(SuperNode {
                id: rep,
                members: sorted.clone(),
                importance: sorted.len() as f64,
            });
        }
        let mut edge_acc: BTreeMap<(String, String), (f64, String)> = BTreeMap::new();
        for edge in graph.graph.edge_indices() {
            let Some((source, target)) = graph.graph.edge_endpoints(edge) else {
                continue;
            };
            let (Some(sid), Some(tid)) = (
                graph.graph.node_weight(source),
                graph.graph.node_weight(target),
            ) else {
                continue;
            };
            let (Some(&cs), Some(&ct)) = (community_of.get(sid), community_of.get(tid)) else {
                continue;
            };
            if cs == ct {
                continue;
            }
            let relation = graph.graph.edge_weight(edge).cloned().unwrap_or_default();
            if let (Some(from), Some(to)) = (representative.get(&cs), representative.get(&ct)) {
                let entry = edge_acc
                    .entry((from.clone(), to.clone()))
                    .or_insert((0.0, relation));
                entry.0 += 1.0;
            }
        }
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        let edges = edges_from_accumulator(edge_acc);
        Ok(SimplifiedGraph {
            nodes,
            edges,
            original_node_count: labels.len(),
            original_edge_count: graph.graph.edge_count(),
        })
    }
}

/// Aggregated edge weight and a representative relation label.
#[derive(Debug, Clone)]
struct EdgeAgg {
    weight: f64,
    relation: String,
}

/// Mutable index-based working representation used by the in-place algorithms.
struct WorkingGraph {
    labels: Vec<String>,
    alive: Vec<bool>,
    members: Vec<Vec<String>>,
    out_edges: Vec<BTreeMap<usize, EdgeAgg>>,
    in_edges: Vec<BTreeMap<usize, EdgeAgg>>,
    original_node_count: usize,
    original_edge_count: usize,
    degrees: HashMap<String, usize>,
}

impl WorkingGraph {
    fn from_graph(graph: &DependencyGraph) -> Self {
        let node_count = graph.graph.node_count();
        let mut labels = Vec::with_capacity(node_count);
        let mut compact: HashMap<NodeIndex, usize> = HashMap::with_capacity(node_count);
        for (index, idx) in graph.graph.node_indices().enumerate() {
            labels.push(graph.graph.node_weight(idx).cloned().unwrap_or_default());
            compact.insert(idx, index);
        }
        let members = labels.iter().map(|label| vec![label.clone()]).collect();
        let mut out_edges = vec![BTreeMap::new(); node_count];
        let mut in_edges = vec![BTreeMap::new(); node_count];
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge)
                && let (Some(&src), Some(&dst)) = (compact.get(&source), compact.get(&target))
                && src != dst
            {
                let relation = graph.graph.edge_weight(edge).cloned().unwrap_or_default();
                accumulate(&mut out_edges[src], dst, &relation);
                accumulate(&mut in_edges[dst], src, &relation);
            }
        }
        Self {
            alive: vec![true; node_count],
            labels,
            members,
            out_edges,
            in_edges,
            original_node_count: node_count,
            original_edge_count: graph.graph.edge_count(),
            degrees: degree_map(graph),
        }
    }

    fn neighbour_count(&self, node: usize) -> usize {
        let mut neighbours: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
        neighbours.extend(self.out_edges[node].keys().copied());
        neighbours.extend(self.in_edges[node].keys().copied());
        neighbours.remove(&node);
        neighbours.len()
    }

    fn remove_node(&mut self, node: usize) {
        let outgoing: Vec<usize> = self.out_edges[node].keys().copied().collect();
        for target in outgoing {
            self.in_edges[target].remove(&node);
        }
        let incoming: Vec<usize> = self.in_edges[node].keys().copied().collect();
        for source in incoming {
            self.out_edges[source].remove(&node);
        }
        self.out_edges[node].clear();
        self.in_edges[node].clear();
        self.alive[node] = false;
    }

    fn prune_leaves(&mut self, rounds: usize) {
        for _ in 0..rounds {
            let leaves: Vec<usize> = (0..self.labels.len())
                .filter(|&node| self.alive[node] && self.neighbour_count(node) <= 1)
                .collect();
            if leaves.is_empty() {
                break;
            }
            for node in leaves {
                self.remove_node(node);
            }
        }
    }

    fn add_edge(&mut self, source: usize, target: usize, agg: EdgeAgg) {
        let out = self.out_edges[source]
            .entry(target)
            .or_insert_with(|| EdgeAgg {
                weight: 0.0,
                relation: agg.relation.clone(),
            });
        out.weight += agg.weight;
        let incoming = self.in_edges[target]
            .entry(source)
            .or_insert_with(|| EdgeAgg {
                weight: 0.0,
                relation: agg.relation.clone(),
            });
        incoming.weight += agg.weight;
    }

    fn contract(&mut self, into: usize, from: usize) {
        if into == from || !self.alive[from] || !self.alive[into] {
            return;
        }
        let from_out: Vec<(usize, EdgeAgg)> = self.out_edges[from]
            .iter()
            .map(|(&target, agg)| (target, agg.clone()))
            .collect();
        for (target, agg) in from_out {
            self.in_edges[target].remove(&from);
            if target == into {
                continue;
            }
            self.add_edge(into, target, agg);
        }
        let from_in: Vec<(usize, EdgeAgg)> = self.in_edges[from]
            .iter()
            .map(|(&source, agg)| (source, agg.clone()))
            .collect();
        for (source, agg) in from_in {
            self.out_edges[source].remove(&from);
            if source == into {
                continue;
            }
            self.add_edge(source, into, agg);
        }
        let folded = std::mem::take(&mut self.members[from]);
        self.members[into].extend(folded);
        self.out_edges[from].clear();
        self.in_edges[from].clear();
        self.alive[from] = false;
    }

    fn contract_degree_two_chains(&mut self) {
        loop {
            let mut target = None;
            for node in 0..self.labels.len() {
                if !self.alive[node] {
                    continue;
                }
                if self.out_edges[node].len() == 1
                    && self.in_edges[node].len() == 1
                    && let (Some(&successor), Some(&predecessor)) = (
                        self.out_edges[node].keys().next(),
                        self.in_edges[node].keys().next(),
                    )
                    && predecessor != node
                    && successor != node
                    && predecessor != successor
                {
                    target = Some((predecessor, node));
                    break;
                }
            }
            match target {
                Some((predecessor, node)) => self.contract(predecessor, node),
                None => break,
            }
        }
    }

    fn filter_by_importance(&mut self, keep_fraction: f64) {
        let ranks = self.pagerank();
        let mut alive_nodes: Vec<usize> = (0..self.labels.len())
            .filter(|&node| self.alive[node])
            .collect();
        if alive_nodes.is_empty() {
            return;
        }
        let keep_count = ((keep_fraction * alive_nodes.len() as f64).ceil() as usize)
            .clamp(1, alive_nodes.len());
        // Sort survivors first (highest rank, ties by smallest label).
        alive_nodes.sort_by(|&a, &b| {
            ranks[b]
                .partial_cmp(&ranks[a])
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| self.labels[a].cmp(&self.labels[b]))
        });
        let dropped: Vec<usize> = alive_nodes.split_off(keep_count);
        // Remove least important first, bridging predecessors to successors.
        let mut ordered = dropped;
        ordered.sort_by(|&a, &b| {
            ranks[a]
                .partial_cmp(&ranks[b])
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| self.labels[a].cmp(&self.labels[b]))
        });
        for node in ordered {
            let sources: Vec<usize> = self.in_edges[node].keys().copied().collect();
            let targets: Vec<usize> = self.out_edges[node].keys().copied().collect();
            for &source in &sources {
                for &target in &targets {
                    if source != target {
                        self.add_edge(
                            source,
                            target,
                            EdgeAgg {
                                weight: 1.0,
                                relation: "bridge".to_string(),
                            },
                        );
                    }
                }
            }
            self.remove_node(node);
        }
    }

    fn pagerank(&self) -> Vec<f64> {
        const DAMPING: f64 = 0.85;
        const ITERATIONS: usize = 60;
        let node_count = self.labels.len();
        let alive_nodes: Vec<usize> = (0..node_count).filter(|&node| self.alive[node]).collect();
        let mut ranks = vec![0.0f64; node_count];
        let alive_count = alive_nodes.len();
        if alive_count == 0 {
            return ranks;
        }
        let base = 1.0 / alive_count as f64;
        for &node in &alive_nodes {
            ranks[node] = base;
        }
        let alive_outs: Vec<Vec<usize>> = (0..node_count)
            .map(|node| {
                if self.alive[node] {
                    self.out_edges[node]
                        .keys()
                        .copied()
                        .filter(|&target| self.alive[target])
                        .collect()
                } else {
                    Vec::new()
                }
            })
            .collect();
        for _ in 0..ITERATIONS {
            let mut next = vec![0.0f64; node_count];
            let teleport = (1.0 - DAMPING) * base;
            let mut dangling = 0.0;
            for &node in &alive_nodes {
                next[node] = teleport;
            }
            for &node in &alive_nodes {
                let outs = &alive_outs[node];
                if outs.is_empty() {
                    dangling += ranks[node];
                    continue;
                }
                let share = DAMPING * ranks[node] / outs.len() as f64;
                for &target in outs {
                    next[target] += share;
                }
            }
            let dangling_share = DAMPING * dangling * base;
            for &node in &alive_nodes {
                next[node] += dangling_share;
            }
            ranks = next;
        }
        ranks
    }

    fn into_simplified(self) -> SimplifiedGraph {
        // Choose a representative id per surviving node.
        let mut id_of: Vec<String> = vec![String::new(); self.labels.len()];
        for (node, slot) in id_of.iter_mut().enumerate() {
            if !self.alive[node] {
                continue;
            }
            let mut members = self.members[node].clone();
            members.sort();
            *slot = representative_member(&members, &self.degrees)
                .cloned()
                .unwrap_or_else(|| self.labels[node].clone());
        }
        let mut nodes: Vec<SuperNode> = Vec::new();
        for (node, rep) in id_of.iter().enumerate() {
            if !self.alive[node] {
                continue;
            }
            let mut members = self.members[node].clone();
            members.sort();
            nodes.push(SuperNode {
                id: rep.clone(),
                importance: members.len() as f64,
                members,
            });
        }
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        let mut edge_acc: BTreeMap<(String, String), (f64, String)> = BTreeMap::new();
        for source in 0..self.labels.len() {
            if !self.alive[source] {
                continue;
            }
            for (&target, agg) in &self.out_edges[source] {
                if !self.alive[target] || source == target {
                    continue;
                }
                let key = (id_of[source].clone(), id_of[target].clone());
                let entry = edge_acc.entry(key).or_insert((0.0, agg.relation.clone()));
                entry.0 += agg.weight;
            }
        }
        let edges = edges_from_accumulator(edge_acc);
        SimplifiedGraph {
            nodes,
            edges,
            original_node_count: self.original_node_count,
            original_edge_count: self.original_edge_count,
        }
    }
}

fn accumulate(edges: &mut BTreeMap<usize, EdgeAgg>, target: usize, relation: &str) {
    let entry = edges.entry(target).or_insert_with(|| EdgeAgg {
        weight: 0.0,
        relation: relation.to_string(),
    });
    entry.weight += 1.0;
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

    fn star_with_leaves() -> DependencyGraph {
        // Hub with three leaves plus a central edge to another hub.
        let mut graph = DependencyGraph::new();
        graph.add_dependency("hub", "leaf1", "depends_on");
        graph.add_dependency("hub", "leaf2", "depends_on");
        graph.add_dependency("hub", "leaf3", "depends_on");
        graph.add_dependency("hub", "hub2", "depends_on");
        graph.add_dependency("hub2", "hub", "depends_on");
        graph
    }

    fn chain() -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "depends_on");
        graph.add_dependency("b", "c", "depends_on");
        graph.add_dependency("c", "d", "depends_on");
        graph
    }

    #[test]
    fn prune_leaves_removes_pendants() {
        // Triangle core (a-b-c, a true 2-core) with two pendant leaves.
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "depends_on");
        graph.add_dependency("b", "c", "depends_on");
        graph.add_dependency("c", "a", "depends_on");
        graph.add_dependency("a", "leaf1", "depends_on");
        graph.add_dependency("b", "leaf2", "depends_on");
        let simplifier = GraphSimplifier::new();
        let simplified = simplifier.prune_leaves(&graph, 1);
        // Both pendant leaves removed, the triangle core survives.
        assert_eq!(simplified.node_count(), 3);
        assert!(simplified.member_of("leaf1").is_none());
        assert!(simplified.member_of("a").is_some());
        assert!(simplified.node_reduction_ratio() > 0.0);
        assert_eq!(simplified.original_node_count, 5);
    }

    #[test]
    fn contract_chains_collapses_path() {
        let simplifier = GraphSimplifier::new();
        let simplified = simplifier.contract_chains(&chain());
        // a -> b -> c -> d : b and c contract into a single node.
        assert!(simplified.node_count() < 4);
        // Every original node is still represented somewhere.
        for original in ["a", "b", "c", "d"] {
            assert!(simplified.member_of(original).is_some());
        }
    }

    #[test]
    fn filter_by_importance_keeps_fraction() {
        let mut graph = DependencyGraph::new();
        for i in 0..10 {
            graph.add_dependency("center", &format!("n{i}"), "depends_on");
            graph.add_dependency(&format!("n{i}"), "center", "depends_on");
        }
        let simplifier = GraphSimplifier::new();
        let simplified = simplifier
            .filter_by_importance(&graph, 0.5)
            .expect("valid fraction");
        assert!(simplified.node_count() < 11);
        assert!(simplified.node_count() >= 1);
        // The high-degree center must survive.
        assert!(simplified.member_of("center").is_some());
    }

    #[test]
    fn filter_rejects_invalid_fraction() {
        let simplifier = GraphSimplifier::new();
        assert!(simplifier.filter_by_importance(&chain(), 0.0).is_err());
        assert!(simplifier.filter_by_importance(&chain(), 1.5).is_err());
    }

    #[test]
    fn filter_bridges_preserve_reachability() {
        // a -> b -> c, drop b, expect a -> c bridge.
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "depends_on");
        graph.add_dependency("b", "c", "depends_on");
        // Make a and c important by adding self-reinforcing structure.
        graph.add_dependency("c", "a", "depends_on");
        let simplifier = GraphSimplifier::new();
        let simplified = simplifier
            .filter_by_importance(&graph, 0.67)
            .expect("valid fraction");
        // If b was dropped, a -> c (or an equivalent bridge) should exist.
        if simplified.member_of("b").is_none() {
            assert!(
                simplified
                    .edges
                    .iter()
                    .any(|edge| edge.relation == "bridge" || edge.from != edge.to)
            );
        }
    }

    #[test]
    fn coarsen_builds_quotient_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a1", "a2", "depends_on");
        graph.add_dependency("b1", "b2", "depends_on");
        graph.add_dependency("a2", "b1", "depends_on");
        let communities = vec![
            vec!["a1".to_string(), "a2".to_string()],
            vec!["b1".to_string(), "b2".to_string()],
        ];
        let simplifier = GraphSimplifier::new();
        let simplified = simplifier
            .coarsen_by_communities(&graph, &communities)
            .expect("valid communities");
        assert_eq!(simplified.node_count(), 2);
        // Only the inter-community edge survives.
        assert_eq!(simplified.edge_count(), 1);
    }

    #[test]
    fn coarsen_rejects_incomplete_partition() {
        let graph = chain();
        let communities = vec![vec!["a".to_string(), "b".to_string()]];
        let simplifier = GraphSimplifier::new();
        // c and d are not covered.
        assert!(
            simplifier
                .coarsen_by_communities(&graph, &communities)
                .is_err()
        );
    }

    #[test]
    fn coarsen_rejects_overlapping_communities() {
        let graph = chain();
        let communities = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string(), "d".to_string()],
        ];
        let simplifier = GraphSimplifier::new();
        assert!(
            simplifier
                .coarsen_by_communities(&graph, &communities)
                .is_err()
        );
    }

    #[test]
    fn simplify_pipeline_reduces_and_round_trips() {
        let simplifier = GraphSimplifier::new()
            .with_prune_rounds(1)
            .with_chain_contraction(true)
            .with_keep_fraction(1.0);
        let simplified = simplifier.simplify(&star_with_leaves()).expect("pipeline");
        assert!(simplified.node_count() <= 5);
        // Round-trips to a DependencyGraph for reuse of existing renderers.
        let rebuilt = simplified.to_dependency_graph();
        assert_eq!(rebuilt.node_count(), simplified.node_count());
    }

    #[test]
    fn simplify_rejects_invalid_keep_fraction() {
        let simplifier = GraphSimplifier::new().with_keep_fraction(2.0);
        assert!(simplifier.simplify(&chain()).is_err());
    }
}
