//! Memory-usage optimisation for large dependency graphs.
//!
//! A petgraph `DiGraph<String, String>` is convenient but heavy: every node and
//! every edge owns a heap-allocated `String`, and repeated labels (relations
//! such as `"depends_on"` in particular) are duplicated thousands of times.
//!
//! This module provides a compact, immutable representation built around two
//! ideas that are standard in high-performance graph engines:
//!
//! - A [`StringInterner`] stores each distinct label exactly once and hands out
//!   small [`Symbol`] handles, eliminating duplicate string storage.
//! - A [`CompactGraph`] keeps adjacency in *compressed sparse row* (CSR) form —
//!   a single flat `edge_targets` array indexed by a per-node `edge_offsets`
//!   table — which is dramatically more cache-friendly and compact than a map
//!   of per-node adjacency lists.
//!
//! Construction reserves capacity up front (avoiding incremental reallocation),
//! and [`CompactGraph::write_edge_list`] / [`CompactGraph::stream_edges`] emit
//! the graph without materialising a second copy in memory.

use crate::functions::VizResult;
use crate::types_4::DependencyGraph;
use crate::types_5::VizError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A compact handle to an interned string.
///
/// Occupies four bytes regardless of the underlying string length.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Symbol(u32);

impl Symbol {
    /// Returns the symbol's raw index into the owning [`StringInterner`].
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Deduplicating string store mapping distinct strings to compact [`Symbol`]s.
#[derive(Debug, Clone, Default)]
pub struct StringInterner {
    lookup: HashMap<String, u32>,
    storage: Vec<String>,
}

impl StringInterner {
    /// Creates an empty interner.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an interner with capacity reserved for `capacity` distinct
    /// strings, avoiding reallocation while filling.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            lookup: HashMap::with_capacity(capacity),
            storage: Vec::with_capacity(capacity),
        }
    }

    /// Interns `value`, returning the existing handle when already present.
    pub fn intern(&mut self, value: &str) -> Symbol {
        if let Some(&id) = self.lookup.get(value) {
            return Symbol(id);
        }
        let id = self.storage.len() as u32;
        self.storage.push(value.to_string());
        self.lookup.insert(value.to_string(), id);
        Symbol(id)
    }

    /// Resolves a [`Symbol`] back to its string, if it belongs to this interner.
    pub fn resolve(&self, symbol: Symbol) -> Option<&str> {
        self.storage.get(symbol.index()).map(String::as_str)
    }

    /// Returns the number of distinct interned strings.
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` when no strings have been interned.
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Total number of bytes occupied by the interned string contents.
    pub fn interned_bytes(&self) -> usize {
        self.storage.iter().map(String::len).sum()
    }
}

/// A compressed-sparse-row, arena-backed view of a [`DependencyGraph`].
///
/// Node labels and edge relations are interned. Out-edges are stored as a flat
/// `edge_targets` array; the targets of node `i` are
/// `edge_targets[edge_offsets[i]..edge_offsets[i + 1]]`.
#[derive(Debug, Clone)]
pub struct CompactGraph {
    interner: StringInterner,
    node_labels: Vec<Symbol>,
    edge_offsets: Vec<u32>,
    edge_targets: Vec<u32>,
    edge_relations: Vec<Symbol>,
}

impl CompactGraph {
    /// Builds a compact representation from a [`DependencyGraph`].
    ///
    /// Capacity is reserved up front for nodes and edges. Out-edges are sorted
    /// by target so that traversal order is deterministic.
    pub fn from_dependency_graph(graph: &DependencyGraph) -> Self {
        let node_count = graph.graph.node_count();
        let edge_count = graph.graph.edge_count();
        let mut interner = StringInterner::with_capacity(node_count);
        let mut node_labels = Vec::with_capacity(node_count);
        let mut compact_of = HashMap::with_capacity(node_count);
        for (compact, idx) in graph.graph.node_indices().enumerate() {
            let label = graph
                .graph
                .node_weight(idx)
                .map(String::as_str)
                .unwrap_or("");
            node_labels.push(interner.intern(label));
            compact_of.insert(idx, compact as u32);
        }
        let mut per_source: Vec<Vec<(u32, Symbol)>> = vec![Vec::new(); node_count];
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge)
                && let (Some(&src), Some(&dst)) = (compact_of.get(&source), compact_of.get(&target))
            {
                let relation = graph
                    .graph
                    .edge_weight(edge)
                    .map(String::as_str)
                    .unwrap_or("");
                let symbol = interner.intern(relation);
                per_source[src as usize].push((dst, symbol));
            }
        }
        let (edge_offsets, edge_targets, edge_relations) =
            build_csr(per_source, node_count, edge_count);
        Self {
            interner,
            node_labels,
            edge_offsets,
            edge_targets,
            edge_relations,
        }
    }

    /// Returns the number of nodes.
    pub fn node_count(&self) -> usize {
        self.node_labels.len()
    }

    /// Returns the number of edges.
    pub fn edge_count(&self) -> usize {
        self.edge_targets.len()
    }

    /// Returns `true` when the graph has no nodes.
    pub fn is_empty(&self) -> bool {
        self.node_labels.is_empty()
    }

    /// Resolves the label of node `node`, if it exists.
    pub fn label(&self, node: usize) -> Option<&str> {
        self.node_labels
            .get(node)
            .and_then(|&symbol| self.interner.resolve(symbol))
    }

    /// Returns the compact target indices of `node`'s out-edges.
    pub fn neighbors(&self, node: usize) -> &[u32] {
        match self.edge_range(node) {
            Some((start, end)) => self.edge_targets.get(start..end).unwrap_or(&[]),
            None => &[],
        }
    }

    /// Returns the relation symbols of `node`'s out-edges, aligned with
    /// [`CompactGraph::neighbors`].
    pub fn neighbor_relations(&self, node: usize) -> &[Symbol] {
        match self.edge_range(node) {
            Some((start, end)) => self.edge_relations.get(start..end).unwrap_or(&[]),
            None => &[],
        }
    }

    /// Returns the out-degree of `node`.
    pub fn degree(&self, node: usize) -> usize {
        self.neighbors(node).len()
    }

    /// Borrows the underlying string interner.
    pub fn interner(&self) -> &StringInterner {
        &self.interner
    }

    /// Computes a [`MemoryFootprint`] comparing the compact layout against an
    /// estimate of the equivalent naive `DiGraph<String, String>`.
    pub fn memory_footprint(&self) -> MemoryFootprint {
        let symbol_bytes = std::mem::size_of::<u32>();
        let string_header = std::mem::size_of::<String>();
        let index_bytes = (self.node_labels.len()
            + self.edge_offsets.len()
            + self.edge_targets.len()
            + self.edge_relations.len())
            * symbol_bytes;
        let interner_bytes = self.interner.interned_bytes() + self.interner.len() * string_header;
        let compact_bytes = index_bytes + interner_bytes;

        let mut naive_bytes = 0usize;
        for &symbol in &self.node_labels {
            naive_bytes += string_header + self.interner.resolve(symbol).map_or(0, str::len);
        }
        for &symbol in &self.edge_relations {
            naive_bytes += string_header
                + self.interner.resolve(symbol).map_or(0, str::len)
                + 2 * symbol_bytes;
        }
        MemoryFootprint {
            node_count: self.node_count(),
            edge_count: self.edge_count(),
            compact_bytes,
            naive_bytes,
        }
    }

    /// Streams every edge as `(source_label, target_label, relation)` triples
    /// without allocating an intermediate collection.
    pub fn stream_edges<F: FnMut(&str, &str, &str)>(&self, mut visit: F) {
        for node in 0..self.node_count() {
            let Some(source) = self.label(node) else {
                continue;
            };
            let targets = self.neighbors(node);
            let relations = self.neighbor_relations(node);
            for (target, relation) in targets.iter().zip(relations.iter()) {
                if let (Some(dst), Some(rel)) = (
                    self.label(*target as usize),
                    self.interner.resolve(*relation),
                ) {
                    visit(source, dst, rel);
                }
            }
        }
    }

    /// Writes the edge list directly to `writer`, streaming one
    /// `source -> target [relation]` line per edge.
    pub fn write_edge_list<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        for node in 0..self.node_count() {
            let Some(source) = self.label(node) else {
                continue;
            };
            let targets = self.neighbors(node);
            let relations = self.neighbor_relations(node);
            for (target, relation) in targets.iter().zip(relations.iter()) {
                if let (Some(dst), Some(rel)) = (
                    self.label(*target as usize),
                    self.interner.resolve(*relation),
                ) {
                    writeln!(writer, "{source} -> {dst} [{rel}]")?;
                }
            }
        }
        Ok(())
    }

    /// Materialises the edge list as a single string via streaming emit.
    pub fn to_edge_list(&self) -> String {
        let mut out = String::with_capacity(self.edge_count() * 24);
        // Writing into a `String` is infallible.
        let _ = self.write_edge_list(&mut out);
        out
    }

    fn edge_range(&self, node: usize) -> Option<(usize, usize)> {
        let start = *self.edge_offsets.get(node)? as usize;
        let end = *self.edge_offsets.get(node + 1)? as usize;
        Some((start, end))
    }
}

/// Incremental builder for a [`CompactGraph`] supporting capacity reservation
/// and streaming construction.
#[derive(Debug, Clone, Default)]
pub struct CompactGraphBuilder {
    interner: StringInterner,
    node_labels: Vec<Symbol>,
    index_of: HashMap<String, u32>,
    pending: Vec<(u32, u32, Symbol)>,
}

impl CompactGraphBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a builder with capacity reserved for `nodes` nodes and `edges`
    /// edges.
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Self {
            interner: StringInterner::with_capacity(nodes),
            node_labels: Vec::with_capacity(nodes),
            index_of: HashMap::with_capacity(nodes),
            pending: Vec::with_capacity(edges),
        }
    }

    /// Reserves additional capacity for `additional` nodes.
    pub fn reserve_nodes(&mut self, additional: usize) {
        self.node_labels.reserve(additional);
        self.index_of.reserve(additional);
    }

    /// Reserves additional capacity for `additional` edges.
    pub fn reserve_edges(&mut self, additional: usize) {
        self.pending.reserve(additional);
    }

    /// Adds (or finds) a node by id and returns its compact index.
    pub fn add_node(&mut self, id: &str) -> u32 {
        if let Some(&index) = self.index_of.get(id) {
            return index;
        }
        let index = self.node_labels.len() as u32;
        self.node_labels.push(self.interner.intern(id));
        self.index_of.insert(id.to_string(), index);
        index
    }

    /// Adds an edge between two previously added nodes.
    ///
    /// Returns [`VizError::InvalidStructure`] if either endpoint is unknown.
    pub fn add_edge(&mut self, from: &str, to: &str, relation: &str) -> VizResult<()> {
        let source =
            self.index_of.get(from).copied().ok_or_else(|| {
                VizError::InvalidStructure(format!("unknown source node '{from}'"))
            })?;
        let target = self
            .index_of
            .get(to)
            .copied()
            .ok_or_else(|| VizError::InvalidStructure(format!("unknown target node '{to}'")))?;
        let symbol = self.interner.intern(relation);
        self.pending.push((source, target, symbol));
        Ok(())
    }

    /// Number of nodes added so far.
    pub fn node_count(&self) -> usize {
        self.node_labels.len()
    }

    /// Number of edges queued so far.
    pub fn edge_count(&self) -> usize {
        self.pending.len()
    }

    /// Consumes the builder and produces the immutable [`CompactGraph`].
    pub fn build(self) -> CompactGraph {
        let node_count = self.node_labels.len();
        let edge_count = self.pending.len();
        let mut per_source: Vec<Vec<(u32, Symbol)>> = vec![Vec::new(); node_count];
        for (source, target, relation) in self.pending {
            if let Some(bucket) = per_source.get_mut(source as usize) {
                bucket.push((target, relation));
            }
        }
        let (edge_offsets, edge_targets, edge_relations) =
            build_csr(per_source, node_count, edge_count);
        CompactGraph {
            interner: self.interner,
            node_labels: self.node_labels,
            edge_offsets,
            edge_targets,
            edge_relations,
        }
    }
}

/// Reports the memory characteristics of a [`CompactGraph`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryFootprint {
    /// Number of nodes.
    pub node_count: usize,
    /// Number of edges.
    pub edge_count: usize,
    /// Estimated bytes used by the compact representation.
    pub compact_bytes: usize,
    /// Estimated bytes used by an equivalent naive `DiGraph<String, String>`.
    pub naive_bytes: usize,
}

impl MemoryFootprint {
    /// Ratio of naive to compact bytes; values above `1.0` indicate savings.
    pub fn compression_ratio(&self) -> f64 {
        if self.compact_bytes == 0 {
            0.0
        } else {
            self.naive_bytes as f64 / self.compact_bytes as f64
        }
    }

    /// Estimated bytes saved relative to the naive representation.
    pub fn bytes_saved(&self) -> usize {
        self.naive_bytes.saturating_sub(self.compact_bytes)
    }
}

/// Builds CSR arrays from per-source adjacency buckets, sorting each bucket by
/// target for deterministic traversal.
fn build_csr(
    mut per_source: Vec<Vec<(u32, Symbol)>>,
    node_count: usize,
    edge_count: usize,
) -> (Vec<u32>, Vec<u32>, Vec<Symbol>) {
    let mut edge_offsets = Vec::with_capacity(node_count + 1);
    let mut edge_targets = Vec::with_capacity(edge_count);
    let mut edge_relations = Vec::with_capacity(edge_count);
    edge_offsets.push(0u32);
    for bucket in per_source.iter_mut() {
        bucket.sort_by_key(|&(target, _)| target);
        for &(target, relation) in bucket.iter() {
            edge_targets.push(target);
            edge_relations.push(relation);
        }
        edge_offsets.push(edge_targets.len() as u32);
    }
    (edge_offsets, edge_targets, edge_relations)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-a", "statute-b", "depends_on");
        graph.add_dependency("statute-a", "statute-c", "depends_on");
        graph.add_dependency("statute-b", "statute-c", "references");
        graph
    }

    #[test]
    fn interner_deduplicates() {
        let mut interner = StringInterner::new();
        let first = interner.intern("depends_on");
        let second = interner.intern("depends_on");
        let third = interner.intern("references");
        assert_eq!(first, second);
        assert_ne!(first, third);
        assert_eq!(interner.len(), 2);
        assert_eq!(interner.resolve(first), Some("depends_on"));
        assert!(!interner.is_empty());
    }

    #[test]
    fn interner_resolve_unknown_is_none() {
        let interner = StringInterner::new();
        assert_eq!(interner.resolve(Symbol(7)), None);
    }

    #[test]
    fn compact_graph_preserves_structure() {
        let graph = sample_graph();
        let compact = CompactGraph::from_dependency_graph(&graph);
        assert_eq!(compact.node_count(), 3);
        assert_eq!(compact.edge_count(), 3);
        assert!(!compact.is_empty());
        // statute-a (compact index 0) has two out-edges.
        assert_eq!(compact.degree(0), 2);
        assert_eq!(compact.label(0), Some("statute-a"));
        let targets: Vec<&str> = compact
            .neighbors(0)
            .iter()
            .filter_map(|&t| compact.label(t as usize))
            .collect();
        assert!(targets.contains(&"statute-b"));
        assert!(targets.contains(&"statute-c"));
    }

    #[test]
    fn compact_graph_neighbors_out_of_range_is_empty() {
        let compact = CompactGraph::from_dependency_graph(&sample_graph());
        assert!(compact.neighbors(999).is_empty());
        assert!(compact.neighbor_relations(999).is_empty());
        assert_eq!(compact.label(999), None);
    }

    #[test]
    fn memory_footprint_shows_savings_on_repeated_relations() {
        let mut graph = DependencyGraph::new();
        // Many edges sharing the same relation string maximise interning gains.
        for i in 0..200 {
            graph.add_dependency(&format!("n{i}"), &format!("n{}", i + 1), "depends_on");
        }
        let compact = CompactGraph::from_dependency_graph(&graph);
        let footprint = compact.memory_footprint();
        assert!(footprint.compact_bytes < footprint.naive_bytes);
        assert!(footprint.compression_ratio() > 1.0);
        assert!(footprint.bytes_saved() > 0);
    }

    #[test]
    fn streaming_emit_matches_edge_count() {
        let compact = CompactGraph::from_dependency_graph(&sample_graph());
        let mut count = 0usize;
        compact.stream_edges(|_, _, _| count += 1);
        assert_eq!(count, compact.edge_count());

        let listing = compact.to_edge_list();
        assert_eq!(listing.lines().count(), compact.edge_count());
        assert!(listing.contains("statute-a -> statute-b [depends_on]"));
    }

    #[test]
    fn builder_constructs_equivalent_graph() {
        let mut builder = CompactGraphBuilder::with_capacity(3, 2);
        builder.reserve_nodes(1);
        builder.reserve_edges(1);
        builder.add_node("x");
        builder.add_node("y");
        builder.add_node("z");
        builder
            .add_edge("x", "y", "depends_on")
            .expect("known nodes");
        builder
            .add_edge("y", "z", "references")
            .expect("known nodes");
        assert_eq!(builder.node_count(), 3);
        assert_eq!(builder.edge_count(), 2);
        let compact = builder.build();
        assert_eq!(compact.node_count(), 3);
        assert_eq!(compact.edge_count(), 2);
        assert_eq!(compact.degree(0), 1);
    }

    #[test]
    fn builder_rejects_unknown_endpoints() {
        let mut builder = CompactGraphBuilder::new();
        builder.add_node("only");
        let result = builder.add_edge("only", "missing", "depends_on");
        assert!(matches!(result, Err(VizError::InvalidStructure(_))));
    }
}
