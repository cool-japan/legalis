//! Interactive, graph-based navigation over a [`Scene3d`].
//!
//! [`SceneNavigator`] turns a static scene into something a user can *explore*:
//! focus a node, expand its neighbourhood to a chosen depth, walk shortest
//! paths, compute a level-of-detail set, and step back/forward through a focus
//! history — all without mutating the underlying scene.

use super::Scene3d;
use crate::{DiffError, DiffResult};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

/// One entry in the navigator's focus history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavStep {
    /// The node that became focused.
    pub node_id: String,
    /// The expansion depth in effect at that step.
    pub depth: usize,
}

/// An interactive navigator over a scene graph.
///
/// The navigator keeps a lightweight adjacency snapshot keyed by node id, so it
/// is independent of the scene's storage order and cheap to clone/serialise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneNavigator {
    /// Sorted adjacency list per node id.
    adjacency: BTreeMap<String, Vec<String>>,
    /// Currently focused node.
    focus: String,
    /// Current neighbourhood-expansion depth.
    depth: usize,
    /// Back-stack (most recent last); the current focus is *not* on it.
    history: Vec<NavStep>,
    /// Forward-stack populated by [`SceneNavigator::back`].
    forward: Vec<NavStep>,
}

impl SceneNavigator {
    /// Builds a navigator over `scene`, focusing the highest-degree node.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::Visualization`] if the scene has no nodes.
    pub fn new(scene: &Scene3d) -> DiffResult<Self> {
        if scene.is_empty() {
            return Err(DiffError::Visualization(
                "cannot navigate an empty scene".to_string(),
            ));
        }
        let mut adjacency: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for node in scene.nodes() {
            adjacency.entry(node.id.clone()).or_default();
        }
        for edge in scene.edges() {
            if adjacency.contains_key(&edge.source) && adjacency.contains_key(&edge.target) {
                adjacency
                    .entry(edge.source.clone())
                    .or_default()
                    .push(edge.target.clone());
                adjacency
                    .entry(edge.target.clone())
                    .or_default()
                    .push(edge.source.clone());
            }
        }
        for neighbours in adjacency.values_mut() {
            neighbours.sort();
            neighbours.dedup();
        }
        // Default focus: highest degree, ties broken by id for determinism.
        let focus = adjacency
            .iter()
            .max_by(|a, b| a.1.len().cmp(&b.1.len()).then_with(|| b.0.cmp(a.0)))
            .map(|(id, _)| id.clone())
            .unwrap_or_default();
        Ok(Self {
            adjacency,
            focus,
            depth: 1,
            history: Vec::new(),
            forward: Vec::new(),
        })
    }

    /// The currently focused node id.
    #[must_use]
    pub fn focus(&self) -> &str {
        &self.focus
    }

    /// The current expansion depth.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns `true` if `id` exists in the navigated graph.
    #[must_use]
    pub fn contains(&self, id: &str) -> bool {
        self.adjacency.contains_key(id)
    }

    /// Sorted neighbours of `id` (empty if `id` is unknown).
    #[must_use]
    pub fn neighbors(&self, id: &str) -> Vec<String> {
        self.adjacency.get(id).cloned().unwrap_or_default()
    }

    /// Focuses `id`, recording the previous focus on the back-stack and clearing
    /// the forward-stack.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::Visualization`] if `id` is not in the graph.
    pub fn focus_on(&mut self, id: &str) -> DiffResult<()> {
        if !self.adjacency.contains_key(id) {
            return Err(DiffError::Visualization(format!(
                "cannot focus unknown node '{id}'"
            )));
        }
        if id != self.focus {
            self.history.push(NavStep {
                node_id: self.focus.clone(),
                depth: self.depth,
            });
            self.forward.clear();
            self.focus = id.to_string();
        }
        Ok(())
    }

    /// Sets the expansion depth (clamped to at least 1).
    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth.max(1);
    }

    /// Maps each node reachable from the focus within `depth` hops to its graph
    /// distance (BFS). The focus itself has distance 0.
    #[must_use]
    pub fn reachable_within(&self, depth: usize) -> BTreeMap<String, usize> {
        let mut dist: BTreeMap<String, usize> = BTreeMap::new();
        dist.insert(self.focus.clone(), 0);
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(self.focus.clone());
        while let Some(cur) = queue.pop_front() {
            let d = *dist.get(&cur).unwrap_or(&0);
            if d >= depth {
                continue;
            }
            for nb in self.neighbors(&cur) {
                if !dist.contains_key(&nb) {
                    dist.insert(nb.clone(), d + 1);
                    queue.push_back(nb);
                }
            }
        }
        dist
    }

    /// The set of node ids visible at the current focus and depth (sorted).
    #[must_use]
    pub fn visible_nodes(&self) -> Vec<String> {
        self.reachable_within(self.depth).into_keys().collect()
    }

    /// Increases the expansion depth by one and returns the new visible set.
    pub fn expand(&mut self) -> Vec<String> {
        self.depth += 1;
        self.visible_nodes()
    }

    /// Resets the expansion depth to 1 (focus + immediate neighbours).
    pub fn collapse(&mut self) -> Vec<String> {
        self.depth = 1;
        self.visible_nodes()
    }

    /// Computes the shortest path (in hops) from the focus to `target`,
    /// inclusive of both endpoints, or `None` if unreachable.
    #[must_use]
    pub fn path_to(&self, target: &str) -> Option<Vec<String>> {
        self.path_between(&self.focus, target)
    }

    /// Computes the shortest path between any two nodes via BFS.
    #[must_use]
    pub fn path_between(&self, from: &str, to: &str) -> Option<Vec<String>> {
        if !self.adjacency.contains_key(from) || !self.adjacency.contains_key(to) {
            return None;
        }
        if from == to {
            return Some(vec![from.to_string()]);
        }
        let mut prev: HashMap<String, String> = HashMap::new();
        let mut visited: BTreeSet<String> = BTreeSet::new();
        visited.insert(from.to_string());
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(from.to_string());
        while let Some(cur) = queue.pop_front() {
            for nb in self.neighbors(&cur) {
                if visited.insert(nb.clone()) {
                    prev.insert(nb.clone(), cur.clone());
                    if nb == to {
                        return Some(reconstruct(&prev, from, to));
                    }
                    queue.push_back(nb);
                }
            }
        }
        None
    }

    /// A deterministic breadth-first traversal order from the focus, capped at
    /// `max_nodes` entries.
    #[must_use]
    pub fn breadth_first(&self, max_nodes: usize) -> Vec<String> {
        let mut order = Vec::new();
        if max_nodes == 0 {
            return order;
        }
        let mut visited: BTreeSet<String> = BTreeSet::new();
        visited.insert(self.focus.clone());
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(self.focus.clone());
        while let Some(cur) = queue.pop_front() {
            order.push(cur.clone());
            if order.len() >= max_nodes {
                break;
            }
            for nb in self.neighbors(&cur) {
                if visited.insert(nb.clone()) {
                    queue.push_back(nb);
                }
            }
        }
        order
    }

    /// A level-of-detail partition of the reachable nodes, grouped by graph
    /// distance from the focus: index 0 holds the focus, index 1 its immediate
    /// neighbours, and so on up to `max_depth`.
    #[must_use]
    pub fn level_of_detail(&self, max_depth: usize) -> Vec<Vec<String>> {
        let dist = self.reachable_within(max_depth);
        let mut levels: Vec<Vec<String>> = vec![Vec::new(); max_depth + 1];
        for (id, d) in dist {
            if let Some(bucket) = levels.get_mut(d) {
                bucket.push(id);
            }
        }
        for bucket in &mut levels {
            bucket.sort();
        }
        levels
    }

    /// Steps back to the previously focused node, if any. The current focus is
    /// pushed onto the forward-stack. Returns the new focus.
    pub fn back(&mut self) -> Option<String> {
        let step = self.history.pop()?;
        self.forward.push(NavStep {
            node_id: self.focus.clone(),
            depth: self.depth,
        });
        self.focus = step.node_id;
        self.depth = step.depth.max(1);
        Some(self.focus.clone())
    }

    /// Steps forward after a [`SceneNavigator::back`], if possible. Returns the
    /// new focus.
    pub fn forward(&mut self) -> Option<String> {
        let step = self.forward.pop()?;
        self.history.push(NavStep {
            node_id: self.focus.clone(),
            depth: self.depth,
        });
        self.focus = step.node_id;
        self.depth = step.depth.max(1);
        Some(self.focus.clone())
    }

    /// Number of entries available to [`SceneNavigator::back`].
    #[must_use]
    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}

/// Reconstructs a BFS path from the `prev` predecessor map.
fn reconstruct(prev: &HashMap<String, String>, from: &str, to: &str) -> Vec<String> {
    let mut path = vec![to.to_string()];
    let mut cur = to.to_string();
    while cur != from {
        match prev.get(&cur) {
            Some(p) => {
                cur = p.clone();
                path.push(cur.clone());
            }
            None => break,
        }
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::immersive::{scene_from_diff, scene_from_diffs};
    use legalis_core::{Effect, EffectType, Statute};

    fn diff_with_changes(id: &str) -> crate::StatuteDiff {
        let old = Statute::new(id, "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.title = "New".to_string();
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        crate::diff(&old, &new).expect("diff")
    }

    fn navigator() -> SceneNavigator {
        let scene = scene_from_diff(&diff_with_changes("law-1"));
        SceneNavigator::new(&scene).expect("navigator")
    }

    #[test]
    fn test_new_focuses_highest_degree() {
        let nav = navigator();
        // The statute root has the highest degree.
        assert_eq!(nav.focus(), "law-1");
        assert_eq!(nav.depth(), 1);
    }

    #[test]
    fn test_new_rejects_empty_scene() {
        let scene = Scene3d::new();
        assert!(SceneNavigator::new(&scene).is_err());
    }

    #[test]
    fn test_focus_records_history_and_clears_forward() {
        let mut nav = navigator();
        let neighbour = nav.neighbors("law-1")[0].clone();
        nav.focus_on(&neighbour).unwrap();
        assert_eq!(nav.focus(), neighbour);
        assert_eq!(nav.history_len(), 1);
        // Unknown node is rejected.
        assert!(nav.focus_on("ghost").is_err());
    }

    #[test]
    fn test_expand_and_collapse_change_visibility() {
        let mut nav = navigator();
        let near = nav.visible_nodes().len();
        let far = nav.expand().len();
        assert!(far >= near);
        let collapsed = nav.collapse().len();
        assert_eq!(collapsed, near);
    }

    #[test]
    fn test_visible_nodes_includes_focus() {
        let nav = navigator();
        assert!(nav.visible_nodes().contains(&"law-1".to_string()));
    }

    #[test]
    fn test_path_to_reaches_change_node() {
        let nav = navigator();
        // A change node id is namespaced "law-1::change::N".
        let target = nav
            .neighbors("law-1")
            .into_iter()
            .find(|id| id.contains("::change::"))
            .expect("a change neighbour exists");
        let path = nav.path_to(&target).expect("path exists");
        assert_eq!(path.first().map(String::as_str), Some("law-1"));
        assert_eq!(path.last(), Some(&target));
    }

    #[test]
    fn test_path_between_unreachable_components() {
        // Two separate statutes with no forest root => disconnected.
        let mut scene = scene_from_diff(&diff_with_changes("law-1"));
        let other = scene_from_diff(&diff_with_changes("law-2"));
        for node in other.nodes() {
            scene.add_node(node.clone());
        }
        for edge in other.edges() {
            scene.add_edge(edge.clone());
        }
        let nav = SceneNavigator::new(&scene).expect("nav");
        assert!(nav.path_between("law-1", "law-2").is_none());
    }

    #[test]
    fn test_breadth_first_is_capped_and_starts_at_focus() {
        let nav = navigator();
        let order = nav.breadth_first(3);
        assert_eq!(order.len(), 3);
        assert_eq!(order.first().map(String::as_str), Some("law-1"));
    }

    #[test]
    fn test_level_of_detail_buckets_by_distance() {
        let nav = navigator();
        let lod = nav.level_of_detail(2);
        assert_eq!(lod.len(), 3);
        assert_eq!(lod[0], vec!["law-1".to_string()]);
        assert!(!lod[1].is_empty());
    }

    #[test]
    fn test_back_and_forward_navigation() {
        let mut nav = navigator();
        let n1 = nav.neighbors("law-1")[0].clone();
        nav.focus_on(&n1).unwrap();
        assert_eq!(nav.back(), Some("law-1".to_string()));
        assert_eq!(nav.focus(), "law-1");
        assert_eq!(nav.forward(), Some(n1.clone()));
        assert_eq!(nav.focus(), n1);
    }

    #[test]
    fn test_forest_navigation_connects_statutes() {
        let scene = scene_from_diffs(&[diff_with_changes("law-1"), diff_with_changes("law-2")]);
        let nav = SceneNavigator::new(&scene).expect("nav");
        let path = nav
            .path_between("law-1", "law-2")
            .expect("connected via forest");
        assert!(path.contains(&"forest:root".to_string()));
    }
}
