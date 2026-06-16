//! Intelligent node clustering for dependency graphs.
//!
//! Three complementary clustering strategies are provided, each useful for a
//! different facet of graph analysis and visualisation:
//!
//! - [`NodeClusterer::connected_components`] groups nodes by reachability using
//!   a classic [`UnionFind`] (path-halving + union-by-rank).
//! - [`NodeClusterer::label_propagation`] performs community detection via
//!   deterministic asynchronous label propagation.
//! - [`NodeClusterer::kmeans_layout`] clusters nodes spatially with Lloyd's
//!   k-means over the deterministic grid layout, seeded by farthest-first
//!   initialisation (no randomness, fully reproducible).
//!
//! Cluster quality can be assessed with [`NodeClusterer::modularity`].

use super::grid_layout_positions;
use crate::functions::VizResult;
use crate::types_4::DependencyGraph;
use crate::types_5::VizError;
use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// Disjoint-set (union-find) structure with path halving and union by rank.
#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
    count: usize,
}

impl UnionFind {
    /// Creates a forest of `n` singleton sets.
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }

    /// Finds the representative of `x`, compressing the path on the way up.
    pub fn find(&mut self, mut x: usize) -> usize {
        if x >= self.parent.len() {
            return x;
        }
        while self.parent[x] != x {
            let grandparent = self.parent[self.parent[x]];
            self.parent[x] = grandparent;
            x = grandparent;
        }
        x
    }

    /// Unions the sets containing `a` and `b`.
    pub fn union(&mut self, a: usize, b: usize) {
        if a >= self.parent.len() || b >= self.parent.len() {
            return;
        }
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a == root_b {
            return;
        }
        match self.rank[root_a].cmp(&self.rank[root_b]) {
            std::cmp::Ordering::Less => self.parent[root_a] = root_b,
            std::cmp::Ordering::Greater => self.parent[root_b] = root_a,
            std::cmp::Ordering::Equal => {
                self.parent[root_b] = root_a;
                self.rank[root_a] = self.rank[root_a].saturating_add(1);
            }
        }
        self.count -= 1;
    }

    /// Returns `true` when `a` and `b` are in the same set.
    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// Returns the number of disjoint sets.
    pub fn component_count(&self) -> usize {
        self.count
    }
}

/// A single cluster of statute nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    /// Cluster identifier (a dense index `0..cluster_count`).
    pub id: usize,
    /// Member statute ids, sorted for determinism.
    pub members: Vec<String>,
    /// Spatial centroid `(x, y)` when produced by a layout-based method.
    pub centroid: Option<(f64, f64)>,
}

impl Cluster {
    /// Number of members in the cluster.
    pub fn size(&self) -> usize {
        self.members.len()
    }
}

/// The result of a clustering run.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ClusterAssignment {
    /// Clusters, ordered by their (smallest) member id.
    pub clusters: Vec<Cluster>,
}

impl ClusterAssignment {
    /// Number of clusters.
    pub fn cluster_count(&self) -> usize {
        self.clusters.len()
    }

    /// Returns the largest cluster, if any.
    pub fn largest_cluster(&self) -> Option<&Cluster> {
        self.clusters.iter().max_by_key(|cluster| cluster.size())
    }

    /// Returns the cluster id containing `node`, if any.
    pub fn cluster_of(&self, node: &str) -> Option<usize> {
        self.clusters.iter().find_map(|cluster| {
            cluster
                .members
                .iter()
                .any(|member| member == node)
                .then_some(cluster.id)
        })
    }

    /// Projects the assignment to a list of member-id groups, suitable for
    /// feeding [`crate::scalability::GraphSimplifier::coarsen_by_communities`].
    pub fn as_communities(&self) -> Vec<Vec<String>> {
        self.clusters
            .iter()
            .map(|cluster| cluster.members.clone())
            .collect()
    }
}

/// Computes clusterings of a [`DependencyGraph`].
#[derive(Debug, Clone, Default)]
pub struct NodeClusterer;

impl NodeClusterer {
    /// Creates a new clusterer.
    pub fn new() -> Self {
        Self
    }

    /// Groups nodes into weakly-connected components.
    pub fn connected_components(&self, graph: &DependencyGraph) -> ClusterAssignment {
        let labels = node_labels(graph);
        let compact = compact_index(graph);
        let mut union_find = UnionFind::new(labels.len());
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge)
                && let (Some(&src), Some(&dst)) = (compact.get(&source), compact.get(&target))
            {
                union_find.union(src, dst);
            }
        }
        let mut groups: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for (index, label) in labels.iter().enumerate() {
            let root = union_find.find(index);
            groups.entry(root).or_default().push(label.clone());
        }
        assignment_from_groups(groups.into_values())
    }

    /// Detects communities using deterministic asynchronous label propagation.
    ///
    /// Each node adopts the most frequent label among its (undirected)
    /// neighbours; ties are broken towards the smallest label for
    /// reproducibility. Iteration stops at convergence or after `max_iters`.
    pub fn label_propagation(
        &self,
        graph: &DependencyGraph,
        max_iters: usize,
    ) -> ClusterAssignment {
        let labels = node_labels(graph);
        let node_count = labels.len();
        if node_count == 0 {
            return ClusterAssignment::default();
        }
        let adjacency = undirected_adjacency(graph, &labels);
        let mut community: Vec<usize> = (0..node_count).collect();
        for _ in 0..max_iters.max(1) {
            let mut changed = false;
            for node in 0..node_count {
                let neighbours = &adjacency[node];
                if neighbours.is_empty() {
                    continue;
                }
                let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
                for &neighbour in neighbours {
                    *counts.entry(community[neighbour]).or_insert(0) += 1;
                }
                let mut best_label = community[node];
                let mut best_count = 0usize;
                for (&candidate, &count) in &counts {
                    if count > best_count {
                        best_count = count;
                        best_label = candidate;
                    }
                }
                if best_count > 0 && best_label != community[node] {
                    community[node] = best_label;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mut groups: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for (index, label) in labels.iter().enumerate() {
            groups
                .entry(community[index])
                .or_default()
                .push(label.clone());
        }
        assignment_from_groups(groups.into_values())
    }

    /// Clusters nodes spatially into `k` groups with Lloyd's k-means over the
    /// deterministic grid layout.
    ///
    /// Returns [`VizError::InvalidStructure`] when the graph is empty or when
    /// `k` is zero or larger than the node count.
    pub fn kmeans_layout(
        &self,
        graph: &DependencyGraph,
        k: usize,
        max_iters: usize,
    ) -> VizResult<ClusterAssignment> {
        let positions = grid_layout_positions(graph);
        let node_count = positions.len();
        if node_count == 0 {
            return Err(VizError::InvalidStructure(
                "cannot run k-means on an empty graph".to_string(),
            ));
        }
        if k == 0 || k > node_count {
            return Err(VizError::InvalidStructure(format!(
                "k must be in 1..={node_count}, got {k}"
            )));
        }
        let points: Vec<(f64, f64)> = positions.iter().map(|(_, x, y)| (*x, *y)).collect();
        let mut centroids = farthest_first_init(&points, k);
        let mut assignment = vec![0usize; node_count];
        for _ in 0..max_iters.max(1) {
            let mut changed = false;
            for (index, point) in points.iter().enumerate() {
                let nearest = nearest_centroid(point, &centroids);
                if assignment[index] != nearest {
                    assignment[index] = nearest;
                    changed = true;
                }
            }
            recompute_centroids(&points, &assignment, &mut centroids);
            if !changed {
                break;
            }
        }
        let mut groups: BTreeMap<usize, SpatialGroup> = BTreeMap::new();
        for (index, (id, x, y)) in positions.iter().enumerate() {
            let entry = groups.entry(assignment[index]).or_default();
            entry.members.push(id.clone());
            entry.sum_x += x;
            entry.sum_y += y;
            entry.size += 1;
        }
        let mut clusters: Vec<Cluster> = groups
            .into_values()
            .map(|mut group| {
                group.members.sort();
                let centroid = if group.size > 0 {
                    Some((
                        group.sum_x / group.size as f64,
                        group.sum_y / group.size as f64,
                    ))
                } else {
                    None
                };
                Cluster {
                    id: 0,
                    members: group.members,
                    centroid,
                }
            })
            .collect();
        clusters.sort_by(|a, b| a.members.first().cmp(&b.members.first()));
        for (id, cluster) in clusters.iter_mut().enumerate() {
            cluster.id = id;
        }
        Ok(ClusterAssignment { clusters })
    }

    /// Computes the Newman modularity `Q` of an assignment, treating the graph
    /// as undirected. Higher values indicate stronger community structure.
    pub fn modularity(&self, graph: &DependencyGraph, assignment: &ClusterAssignment) -> f64 {
        let edges = graph.graph.edge_count();
        if edges == 0 {
            return 0.0;
        }
        let mut community_of: HashMap<String, usize> = HashMap::new();
        for cluster in &assignment.clusters {
            for member in &cluster.members {
                community_of.insert(member.clone(), cluster.id);
            }
        }
        let mut degree: HashMap<usize, f64> = HashMap::new();
        let mut internal: HashMap<usize, f64> = HashMap::new();
        let two_m = 2.0 * edges as f64;
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
            *degree.entry(cs).or_insert(0.0) += 1.0;
            *degree.entry(ct).or_insert(0.0) += 1.0;
            if cs == ct {
                *internal.entry(cs).or_insert(0.0) += 1.0;
            }
        }
        let mut q = 0.0;
        for cluster in &assignment.clusters {
            let l_c = internal.get(&cluster.id).copied().unwrap_or(0.0);
            let d_c = degree.get(&cluster.id).copied().unwrap_or(0.0);
            q += l_c / edges as f64 - (d_c / two_m).powi(2);
        }
        q
    }
}

/// Running spatial accumulator for a k-means cluster.
#[derive(Debug, Default)]
struct SpatialGroup {
    members: Vec<String>,
    sum_x: f64,
    sum_y: f64,
    size: usize,
}

fn node_labels(graph: &DependencyGraph) -> Vec<String> {
    graph
        .graph
        .node_indices()
        .map(|idx| graph.graph.node_weight(idx).cloned().unwrap_or_default())
        .collect()
}

fn compact_index(graph: &DependencyGraph) -> HashMap<NodeIndex, usize> {
    graph
        .graph
        .node_indices()
        .enumerate()
        .map(|(index, idx)| (idx, index))
        .collect()
}

fn undirected_adjacency(graph: &DependencyGraph, labels: &[String]) -> Vec<Vec<usize>> {
    let compact = compact_index(graph);
    let mut adjacency = vec![Vec::new(); labels.len()];
    for edge in graph.graph.edge_indices() {
        if let Some((source, target)) = graph.graph.edge_endpoints(edge)
            && let (Some(&src), Some(&dst)) = (compact.get(&source), compact.get(&target))
            && src != dst
        {
            adjacency[src].push(dst);
            adjacency[dst].push(src);
        }
    }
    adjacency
}

fn assignment_from_groups<I>(groups: I) -> ClusterAssignment
where
    I: IntoIterator<Item = Vec<String>>,
{
    let mut clusters: Vec<Cluster> = groups
        .into_iter()
        .map(|mut members| {
            members.sort();
            Cluster {
                id: 0,
                members,
                centroid: None,
            }
        })
        .collect();
    clusters.sort_by(|a, b| a.members.first().cmp(&b.members.first()));
    for (id, cluster) in clusters.iter_mut().enumerate() {
        cluster.id = id;
    }
    ClusterAssignment { clusters }
}

fn squared_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}

fn farthest_first_init(points: &[(f64, f64)], k: usize) -> Vec<(f64, f64)> {
    let mut centroids = Vec::with_capacity(k);
    if let Some(&first) = points.first() {
        centroids.push(first);
    }
    while centroids.len() < k {
        let mut best_index = 0;
        let mut best_distance = -1.0;
        for (index, point) in points.iter().enumerate() {
            let nearest = centroids
                .iter()
                .map(|centroid| squared_distance(*point, *centroid))
                .fold(f64::INFINITY, f64::min);
            if nearest > best_distance {
                best_distance = nearest;
                best_index = index;
            }
        }
        if let Some(&point) = points.get(best_index) {
            centroids.push(point);
        } else {
            break;
        }
    }
    centroids
}

fn nearest_centroid(point: &(f64, f64), centroids: &[(f64, f64)]) -> usize {
    let mut best_index = 0;
    let mut best_distance = f64::INFINITY;
    for (index, centroid) in centroids.iter().enumerate() {
        let distance = squared_distance(*point, *centroid);
        if distance < best_distance {
            best_distance = distance;
            best_index = index;
        }
    }
    best_index
}

fn recompute_centroids(points: &[(f64, f64)], assignment: &[usize], centroids: &mut [(f64, f64)]) {
    let mut sums = vec![(0.0f64, 0.0f64, 0usize); centroids.len()];
    for (point, &cluster) in points.iter().zip(assignment.iter()) {
        if let Some(slot) = sums.get_mut(cluster) {
            slot.0 += point.0;
            slot.1 += point.1;
            slot.2 += 1;
        }
    }
    for (centroid, &(sum_x, sum_y, count)) in centroids.iter_mut().zip(sums.iter()) {
        if count > 0 {
            centroid.0 = sum_x / count as f64;
            centroid.1 = sum_y / count as f64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_component_graph() -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        // Component 1: a-b-c
        graph.add_dependency("a", "b", "depends_on");
        graph.add_dependency("b", "c", "depends_on");
        // Component 2: x-y
        graph.add_dependency("x", "y", "depends_on");
        graph
    }

    #[test]
    fn union_find_tracks_components() {
        let mut union_find = UnionFind::new(5);
        assert_eq!(union_find.component_count(), 5);
        union_find.union(0, 1);
        union_find.union(1, 2);
        assert!(union_find.connected(0, 2));
        assert!(!union_find.connected(0, 3));
        assert_eq!(union_find.component_count(), 3);
    }

    #[test]
    fn union_find_ignores_out_of_range() {
        let mut union_find = UnionFind::new(2);
        union_find.union(0, 99);
        assert_eq!(union_find.component_count(), 2);
        assert_eq!(union_find.find(99), 99);
    }

    #[test]
    fn connected_components_splits_disjoint_subgraphs() {
        let clusterer = NodeClusterer::new();
        let assignment = clusterer.connected_components(&two_component_graph());
        assert_eq!(assignment.cluster_count(), 2);
        let largest = assignment.largest_cluster().expect("a cluster");
        assert_eq!(largest.size(), 3);
        assert_eq!(assignment.cluster_of("x"), assignment.cluster_of("y"));
        assert_ne!(assignment.cluster_of("a"), assignment.cluster_of("x"));
    }

    #[test]
    fn label_propagation_detects_communities() {
        // Two dense, disjoint triangles: label propagation collapses each
        // triangle to a single shared label, recovering both communities.
        let mut graph = DependencyGraph::new();
        for (a, b) in [("a", "b"), ("b", "c"), ("c", "a")] {
            graph.add_dependency(a, b, "rel");
        }
        for (a, b) in [("x", "y"), ("y", "z"), ("z", "x")] {
            graph.add_dependency(a, b, "rel");
        }
        let clusterer = NodeClusterer::new();
        let assignment = clusterer.label_propagation(&graph, 50);
        assert_eq!(assignment.cluster_count(), 2);
        // Each triangle is consolidated, never split into singletons.
        assert!(
            assignment
                .clusters
                .iter()
                .all(|cluster| cluster.size() == 3)
        );
        // Determinism: repeated runs are identical.
        let again = clusterer.label_propagation(&graph, 50);
        assert_eq!(assignment, again);
    }

    #[test]
    fn kmeans_partitions_into_k_clusters() {
        let mut graph = DependencyGraph::new();
        for i in 0..16 {
            graph.add_statute(&format!("n{i:02}"));
        }
        let clusterer = NodeClusterer::new();
        let assignment = clusterer.kmeans_layout(&graph, 4, 50).expect("kmeans");
        assert!(assignment.cluster_count() <= 4);
        let total: usize = assignment.clusters.iter().map(Cluster::size).sum();
        assert_eq!(total, 16);
        assert!(assignment.clusters.iter().all(|c| c.centroid.is_some()));
        // Deterministic.
        let again = clusterer.kmeans_layout(&graph, 4, 50).expect("kmeans");
        assert_eq!(assignment, again);
    }

    #[test]
    fn kmeans_rejects_invalid_k() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("only");
        let clusterer = NodeClusterer::new();
        assert!(clusterer.kmeans_layout(&graph, 0, 10).is_err());
        assert!(clusterer.kmeans_layout(&graph, 5, 10).is_err());
        let empty = DependencyGraph::new();
        assert!(clusterer.kmeans_layout(&empty, 1, 10).is_err());
    }

    #[test]
    fn modularity_is_higher_for_good_partition() {
        let graph = two_component_graph();
        let clusterer = NodeClusterer::new();
        let good = clusterer.connected_components(&graph);
        let good_q = clusterer.modularity(&graph, &good);
        // A trivial all-in-one-community partition.
        let mut all = good.clone();
        let merged: Vec<String> = all
            .clusters
            .iter()
            .flat_map(|c| c.members.clone())
            .collect();
        all.clusters = vec![Cluster {
            id: 0,
            members: merged,
            centroid: None,
        }];
        let all_q = clusterer.modularity(&graph, &all);
        assert!(good_q > all_q);
    }

    #[test]
    fn modularity_empty_graph_is_zero() {
        let clusterer = NodeClusterer::new();
        let graph = DependencyGraph::new();
        let assignment = clusterer.connected_components(&graph);
        assert_eq!(clusterer.modularity(&graph, &assignment), 0.0);
    }
}
