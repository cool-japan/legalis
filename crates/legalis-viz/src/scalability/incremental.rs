//! Incremental, viewport-windowed rendering for massive graphs.
//!
//! Re-rendering an entire graph on every interaction is wasteful: a pan or zoom
//! only changes which nodes are visible, and an edit only changes a handful of
//! nodes. This module implements a diff-based renderer that compares the
//! previously rendered [`RenderState`] against the current viewport and graph,
//! and emits SVG fragments **only** for nodes that entered the viewport
//! (`added`), changed content (`updated`) or left it (`removed`).
//!
//! Nodes are rendered in world coordinates so that pure pan/zoom can be applied
//! by a single wrapping transform on the client without re-emitting unchanged
//! nodes. A [`Rect`]-based dirty-region mechanism lets callers force re-emission
//! of a sub-region (for example after a localised theme change) even when node
//! content is otherwise unchanged.

use super::grid_layout_positions;
use crate::functions::VizResult;
use crate::types_4::DependencyGraph;
use crate::types_5::VizError;
use crate::types_10::Theme;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// An axis-aligned rectangle in world coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    /// Left edge.
    pub x: f64,
    /// Top edge.
    pub y: f64,
    /// Width.
    pub width: f64,
    /// Height.
    pub height: f64,
}

impl Rect {
    /// Creates a new rectangle.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns `true` when `(px, py)` lies within the rectangle (inclusive).
    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}

/// The currently visible world-space window plus a client zoom hint.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    /// Left edge of the visible region in world coordinates.
    pub x: f64,
    /// Top edge of the visible region in world coordinates.
    pub y: f64,
    /// Width of the visible region in world coordinates.
    pub width: f64,
    /// Height of the visible region in world coordinates.
    pub height: f64,
    /// Client zoom factor (used by the wrapping transform, must be positive).
    pub zoom: f64,
}

impl Viewport {
    /// Creates a validated viewport.
    ///
    /// Returns [`VizError::RenderError`] when the dimensions or zoom are not
    /// strictly positive.
    pub fn new(x: f64, y: f64, width: f64, height: f64, zoom: f64) -> VizResult<Self> {
        if !(width > 0.0 && height > 0.0 && zoom > 0.0) {
            return Err(VizError::RenderError(
                "viewport width, height and zoom must be positive".to_string(),
            ));
        }
        Ok(Self {
            x,
            y,
            width,
            height,
            zoom,
        })
    }

    /// Returns `true` when `(px, py)` is inside the viewport (inclusive).
    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }

    fn validate(&self) -> VizResult<()> {
        if !(self.width > 0.0 && self.height > 0.0 && self.zoom > 0.0) {
            return Err(VizError::RenderError(
                "viewport width, height and zoom must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

/// A rendered node: its position, content hash and ready-to-insert SVG.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeRender {
    /// Statute id.
    pub id: String,
    /// World x coordinate.
    pub x: f64,
    /// World y coordinate.
    pub y: f64,
    /// Hash of the visible content (label + position).
    pub content_hash: u64,
    /// SVG `<g>` fragment for this node, in world coordinates.
    pub svg: String,
}

/// A node previously committed to a [`RenderState`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct RenderedNode {
    x: f64,
    y: f64,
    content_hash: u64,
}

/// Snapshot of what was last rendered, keyed by node id.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RenderState {
    nodes: BTreeMap<String, RenderedNode>,
}

impl RenderState {
    /// Creates an empty render state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of nodes currently committed.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` when no nodes are committed.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns `true` when `id` is currently committed.
    pub fn contains(&self, id: &str) -> bool {
        self.nodes.contains_key(id)
    }

    /// Advances the state by applying a [`RenderDiff`].
    pub fn apply(&mut self, diff: &RenderDiff) {
        for node in diff.added.iter().chain(diff.updated.iter()) {
            self.nodes.insert(
                node.id.clone(),
                RenderedNode {
                    x: node.x,
                    y: node.y,
                    content_hash: node.content_hash,
                },
            );
        }
        for id in &diff.removed {
            self.nodes.remove(id);
        }
    }

    fn get(&self, id: &str) -> Option<&RenderedNode> {
        self.nodes.get(id)
    }

    fn ids(&self) -> impl Iterator<Item = &String> {
        self.nodes.keys()
    }
}

/// The minimal set of changes between two renders.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RenderDiff {
    /// Nodes that entered the viewport.
    pub added: Vec<NodeRender>,
    /// Visible nodes whose content changed.
    pub updated: Vec<NodeRender>,
    /// Ids of nodes that left the viewport.
    pub removed: Vec<String>,
    /// Count of visible nodes that did not change.
    pub unchanged: usize,
}

impl RenderDiff {
    /// Total number of nodes that require client-side changes.
    pub fn changed_count(&self) -> usize {
        self.added.len() + self.updated.len() + self.removed.len()
    }

    /// Returns `true` when no client-side changes are required.
    pub fn is_empty(&self) -> bool {
        self.changed_count() == 0
    }

    /// Emits an SVG patch containing only added and updated node fragments,
    /// annotated with removal comments for nodes the client should delete.
    pub fn to_svg_patch(&self) -> String {
        let mut patch = String::new();
        for node in &self.added {
            patch.push_str("<!-- add -->\n");
            patch.push_str(&node.svg);
            patch.push('\n');
        }
        for node in &self.updated {
            patch.push_str("<!-- update -->\n");
            patch.push_str(&node.svg);
            patch.push('\n');
        }
        for id in &self.removed {
            patch.push_str(&format!("<!-- remove: {} -->\n", xml_escape(id)));
        }
        patch
    }
}

/// Diff-based incremental SVG renderer.
#[derive(Debug, Clone)]
pub struct IncrementalRenderer {
    theme: Theme,
    node_radius: f64,
    label_limit: usize,
}

impl Default for IncrementalRenderer {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            node_radius: 10.0,
            label_limit: 16,
        }
    }
}

impl IncrementalRenderer {
    /// Creates a renderer with the default light theme.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the theme used for node fills, strokes and text.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the node radius in world units.
    pub fn with_node_radius(mut self, radius: f64) -> Self {
        self.node_radius = radius;
        self
    }

    /// Sets the maximum number of label characters rendered before truncation.
    pub fn with_label_limit(mut self, limit: usize) -> Self {
        self.label_limit = limit;
        self
    }

    /// Computes the diff between `previous` and the current `viewport`/`graph`.
    pub fn compute_diff(
        &self,
        graph: &DependencyGraph,
        viewport: &Viewport,
        previous: &RenderState,
    ) -> VizResult<RenderDiff> {
        self.compute_diff_with_dirty(graph, viewport, previous, &[])
    }

    /// Computes the diff while forcing re-emission of any visible node whose
    /// position falls inside one of the `dirty` regions.
    pub fn compute_diff_with_dirty(
        &self,
        graph: &DependencyGraph,
        viewport: &Viewport,
        previous: &RenderState,
        dirty: &[Rect],
    ) -> VizResult<RenderDiff> {
        viewport.validate()?;
        let positions = grid_layout_positions(graph);
        let mut diff = RenderDiff::default();
        let mut visible: BTreeSet<String> = BTreeSet::new();
        for (id, x, y) in &positions {
            if !viewport.contains(*x, *y) {
                continue;
            }
            visible.insert(id.clone());
            let hash = content_hash(id, *x, *y);
            let in_dirty = dirty.iter().any(|rect| rect.contains(*x, *y));
            match previous.get(id) {
                None => diff.added.push(self.node_render(id, *x, *y, hash)),
                Some(prev) if prev.content_hash != hash || in_dirty => {
                    diff.updated.push(self.node_render(id, *x, *y, hash));
                }
                Some(_) => diff.unchanged += 1,
            }
        }
        for id in previous.ids() {
            if !visible.contains(id) {
                diff.removed.push(id.clone());
            }
        }
        diff.removed.sort();
        Ok(diff)
    }

    /// Renders the full visible window from scratch, returning the patch and the
    /// resulting [`RenderState`].
    pub fn render_full(
        &self,
        graph: &DependencyGraph,
        viewport: &Viewport,
    ) -> VizResult<(String, RenderState)> {
        let mut state = RenderState::new();
        let diff = self.compute_diff(graph, viewport, &state)?;
        state.apply(&diff);
        Ok((diff.to_svg_patch(), state))
    }

    fn node_render(&self, id: &str, x: f64, y: f64, hash: u64) -> NodeRender {
        NodeRender {
            id: id.to_string(),
            x,
            y,
            content_hash: hash,
            svg: self.render_node_svg(id, x, y),
        }
    }

    fn render_node_svg(&self, id: &str, x: f64, y: f64) -> String {
        let display = if id.chars().count() > self.label_limit {
            let truncated: String = id
                .chars()
                .take(self.label_limit.saturating_sub(1))
                .collect();
            format!("{truncated}\u{2026}")
        } else {
            id.to_string()
        };
        format!(
            "<g class=\"node\" data-id=\"{id}\">\
<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{r:.1}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"2\"/>\
<text x=\"{x:.1}\" y=\"{ty:.1}\" font-size=\"10\" text-anchor=\"middle\" fill=\"{text}\">{label}</text>\
</g>",
            id = xml_escape(id),
            x = x,
            y = y,
            r = self.node_radius,
            fill = self.theme.condition_color,
            stroke = self.theme.text_color,
            ty = y + 4.0,
            text = self.theme.text_color,
            label = xml_escape(&display),
        )
    }
}

/// FNV-1a hash over a node's stable visible content (label + world position).
fn content_hash(label: &str, x: f64, y: f64) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = OFFSET;
    for &byte in label.as_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    for value in [x.round() as i64, y.round() as i64] {
        hash ^= value as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

/// Minimal XML/SVG text escaping for ids and labels.
fn xml_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid_graph(count: usize) -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        for i in 0..count {
            graph.add_statute(&format!("node-{i:03}"));
        }
        graph
    }

    #[test]
    fn viewport_validation_rejects_non_positive() {
        assert!(Viewport::new(0.0, 0.0, 100.0, 100.0, 1.0).is_ok());
        assert!(Viewport::new(0.0, 0.0, 0.0, 100.0, 1.0).is_err());
        assert!(Viewport::new(0.0, 0.0, 100.0, 100.0, 0.0).is_err());
    }

    #[test]
    fn full_render_emits_only_visible_nodes() {
        let graph = grid_graph(50);
        let renderer = IncrementalRenderer::new();
        // A small window near the origin should not contain all 50 nodes.
        let viewport = Viewport::new(0.0, 0.0, 200.0, 200.0, 1.0).expect("valid viewport");
        let (patch, state) = renderer.render_full(&graph, &viewport).expect("render");
        assert!(state.node_count() > 0);
        assert!(state.node_count() < 50);
        assert!(patch.contains("<g class=\"node\""));
    }

    #[test]
    fn unchanged_viewport_produces_empty_diff() {
        let graph = grid_graph(30);
        let renderer = IncrementalRenderer::new();
        let viewport = Viewport::new(0.0, 0.0, 400.0, 400.0, 1.0).expect("valid viewport");
        let (_, state) = renderer.render_full(&graph, &viewport).expect("render");
        let diff = renderer
            .compute_diff(&graph, &viewport, &state)
            .expect("diff");
        assert!(diff.is_empty());
        assert_eq!(diff.unchanged, state.node_count());
    }

    #[test]
    fn panning_adds_and_removes_nodes() {
        let graph = grid_graph(64);
        let renderer = IncrementalRenderer::new();
        let first = Viewport::new(0.0, 0.0, 200.0, 200.0, 1.0).expect("viewport");
        let (_, mut state) = renderer.render_full(&graph, &first).expect("render");
        let before = state.node_count();
        // Pan far to the right/bottom into a disjoint region.
        let second = Viewport::new(400.0, 400.0, 200.0, 200.0, 1.0).expect("viewport");
        let diff = renderer
            .compute_diff(&graph, &second, &state)
            .expect("diff");
        assert!(!diff.is_empty());
        assert!(!diff.removed.is_empty());
        state.apply(&diff);
        // State now reflects the new window, distinct from the first.
        assert_ne!(state.node_count(), 0);
        assert!(before > 0);
    }

    #[test]
    fn dirty_region_forces_update_of_unchanged_nodes() {
        let graph = grid_graph(16);
        let renderer = IncrementalRenderer::new();
        let viewport = Viewport::new(0.0, 0.0, 1000.0, 1000.0, 1.0).expect("viewport");
        let (_, state) = renderer.render_full(&graph, &viewport).expect("render");
        // Without dirty regions the diff is empty.
        let clean = renderer
            .compute_diff(&graph, &viewport, &state)
            .expect("diff");
        assert!(clean.is_empty());
        // A dirty rectangle covering everything forces all visible nodes to update.
        let dirty = [Rect::new(0.0, 0.0, 1000.0, 1000.0)];
        let dirtied = renderer
            .compute_diff_with_dirty(&graph, &viewport, &state, &dirty)
            .expect("diff");
        assert_eq!(dirtied.updated.len(), state.node_count());
        assert!(dirtied.added.is_empty());
    }

    #[test]
    fn patch_lists_removed_nodes_as_comments() {
        let graph = grid_graph(64);
        let renderer = IncrementalRenderer::new();
        let first = Viewport::new(0.0, 0.0, 300.0, 300.0, 1.0).expect("viewport");
        let (_, state) = renderer.render_full(&graph, &first).expect("render");
        let second = Viewport::new(700.0, 700.0, 200.0, 200.0, 1.0).expect("viewport");
        let diff = renderer
            .compute_diff(&graph, &second, &state)
            .expect("diff");
        let patch = diff.to_svg_patch();
        if !diff.removed.is_empty() {
            assert!(patch.contains("<!-- remove:"));
        }
    }

    #[test]
    fn content_hash_changes_with_position_not_constant() {
        let a = content_hash("node", 10.0, 20.0);
        let b = content_hash("node", 10.0, 21.0);
        let c = content_hash("other", 10.0, 20.0);
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_eq!(a, content_hash("node", 10.0, 20.0));
    }

    #[test]
    fn xml_escape_handles_special_characters() {
        assert_eq!(xml_escape("a&b<c>\"d'"), "a&amp;b&lt;c&gt;&quot;d&apos;");
    }
}
