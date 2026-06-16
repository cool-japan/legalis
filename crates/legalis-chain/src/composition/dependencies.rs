//! Inter-contract dependency tracking and topological ordering.
//!
//! Part of the `composition` module. A [`DependencyGraph`] records directed
//! "depends-on" edges between named contracts and answers the two questions a
//! multi-contract deployer needs: *in what order must these be deployed?* and
//! *is the dependency set acyclic?*
//!
//! The ordering is a deterministic Kahn topological sort: ties between otherwise
//! independent nodes are broken by insertion order so that repeated runs over the
//! same graph always emit byte-identical deployment plans (important for
//! reproducible builds and for on-chain/off-chain parity of `CREATE2` salts).

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::functions::ChainResult;
use crate::types_19::ChainError;

/// Maximum number of nodes accepted in one dependency graph.
///
/// Bounds the cost of the topological sort and the size of any generated
/// deployment manifest.
pub const MAX_DEPENDENCY_NODES: usize = 1024;

/// A directed acyclic graph of inter-contract "depends-on" relationships.
///
/// Insertion order of nodes is preserved so that the topological sort is
/// deterministic: among nodes whose dependencies are all already satisfied, the
/// one added earliest is emitted first.
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    /// Node names in insertion order. Indices into this vector are the stable
    /// node ids used by the adjacency structures.
    order: Vec<String>,
    /// Maps a node name to its stable id.
    index: BTreeMap<String, usize>,
    /// For each node id, the set of node ids it directly depends on.
    dependencies: Vec<BTreeSet<usize>>,
}

impl DependencyGraph {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of nodes in the graph.
    pub fn len(&self) -> usize {
        self.order.len()
    }

    /// Returns whether the graph contains no nodes.
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    /// Returns whether a node with `name` exists.
    pub fn contains(&self, name: &str) -> bool {
        self.index.contains_key(name)
    }

    /// Registers a contract node, returning its stable id.
    ///
    /// Idempotent: registering an existing name returns the existing id without
    /// duplicating it.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if `name` is empty or if adding a
    /// new node would exceed [`MAX_DEPENDENCY_NODES`].
    pub fn add_node(&mut self, name: &str) -> ChainResult<usize> {
        if name.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "dependency node name must not be empty".to_string(),
            ));
        }
        if let Some(existing) = self.index.get(name) {
            return Ok(*existing);
        }
        if self.order.len() >= MAX_DEPENDENCY_NODES {
            return Err(ChainError::GenerationError(format!(
                "dependency graph exceeds the {MAX_DEPENDENCY_NODES}-node limit"
            )));
        }
        let id = self.order.len();
        self.order.push(name.to_string());
        self.index.insert(name.to_string(), id);
        self.dependencies.push(BTreeSet::new());
        Ok(id)
    }

    /// Records that `dependent` depends on `dependency` (an edge
    /// `dependency -> dependent` in deployment order).
    ///
    /// Both endpoints are auto-registered if not already present. Self-edges are
    /// rejected eagerly because a contract can never depend on itself.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if either name is empty, if the two
    /// names are equal (a trivial self-cycle), or if auto-registration would
    /// exceed [`MAX_DEPENDENCY_NODES`].
    pub fn add_dependency(&mut self, dependent: &str, dependency: &str) -> ChainResult<()> {
        if dependent == dependency {
            return Err(ChainError::GenerationError(format!(
                "contract '{dependent}' cannot depend on itself"
            )));
        }
        let dependent_id = self.add_node(dependent)?;
        let dependency_id = self.add_node(dependency)?;
        if let Some(set) = self.dependencies.get_mut(dependent_id) {
            set.insert(dependency_id);
        }
        Ok(())
    }

    /// Returns the direct dependencies of `name`, in insertion order, if the node
    /// exists.
    pub fn direct_dependencies(&self, name: &str) -> Option<Vec<String>> {
        let id = *self.index.get(name)?;
        let set = self.dependencies.get(id)?;
        Some(
            set.iter()
                .filter_map(|dep| self.order.get(*dep).cloned())
                .collect(),
        )
    }

    /// Computes the unmet-prerequisite count of every node, keyed by id.
    ///
    /// A node's count is simply the number of contracts it directly depends on;
    /// Kahn's algorithm decrements these as dependencies are deployed.
    fn prerequisite_counts(&self) -> Vec<usize> {
        self.dependencies.iter().map(BTreeSet::len).collect()
    }

    /// Produces a deterministic topological deployment order.
    ///
    /// The returned vector lists every node such that each contract appears only
    /// *after* all of its dependencies. Among ready nodes (those whose remaining
    /// prerequisite count has reached zero) the earliest-inserted is chosen,
    /// guaranteeing a stable, reproducible plan.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] naming the participants of a cycle
    /// if the graph is not acyclic.
    pub fn topological_order(&self) -> ChainResult<Vec<String>> {
        let node_count = self.order.len();
        let mut remaining = self.prerequisite_counts();

        // Build the reverse adjacency: for each dependency, which nodes wait on it.
        let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); node_count];
        for (node, deps) in self.dependencies.iter().enumerate() {
            for &dep in deps {
                if let Some(list) = dependents.get_mut(dep) {
                    list.push(node);
                }
            }
        }

        // Seed the queue with every node that has no unmet prerequisites,
        // preserving insertion order.
        let mut ready: VecDeque<usize> = VecDeque::new();
        for (id, &count) in remaining.iter().enumerate() {
            if count == 0 {
                ready.push_back(id);
            }
        }

        let mut ordered: Vec<String> = Vec::with_capacity(node_count);
        while let Some(node) = ready.pop_front() {
            if let Some(name) = self.order.get(node) {
                ordered.push(name.clone());
            }
            // Collect newly-ready dependents and re-insert them in ascending id
            // order so the overall ordering stays insertion-stable.
            let mut freed: Vec<usize> = Vec::new();
            if let Some(list) = dependents.get(node) {
                for &dependent in list {
                    if let Some(count) = remaining.get_mut(dependent) {
                        *count = count.saturating_sub(1);
                        if *count == 0 {
                            freed.push(dependent);
                        }
                    }
                }
            }
            freed.sort_unstable();
            for dependent in freed {
                ready.push_back(dependent);
            }
        }

        if ordered.len() != node_count {
            let cycle = self.find_cycle_members(&ordered);
            return Err(ChainError::GenerationError(format!(
                "dependency cycle detected among: {}",
                cycle.join(", ")
            )));
        }
        Ok(ordered)
    }

    /// Returns whether the dependency graph is acyclic.
    pub fn is_acyclic(&self) -> bool {
        self.topological_order().is_ok()
    }

    /// Returns the names that could not be ordered (i.e. participate in or are
    /// dominated by a cycle), used to build a helpful error message.
    fn find_cycle_members(&self, ordered: &[String]) -> Vec<String> {
        let placed: BTreeSet<&String> = ordered.iter().collect();
        self.order
            .iter()
            .filter(|name| !placed.contains(name))
            .cloned()
            .collect()
    }

    /// Computes the transitive closure of dependencies for `name`: every contract
    /// that must be deployed before it, directly or indirectly.
    ///
    /// The result is returned in deterministic topological order (dependencies of
    /// dependencies first). The node itself is excluded.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if `name` is unknown or if a cycle
    /// makes the closure ill-defined.
    pub fn transitive_dependencies(&self, name: &str) -> ChainResult<Vec<String>> {
        let start = *self.index.get(name).ok_or_else(|| {
            ChainError::GenerationError(format!("unknown dependency node: '{name}'"))
        })?;

        // Breadth-first collect the reachable dependency set.
        let mut reachable: BTreeSet<usize> = BTreeSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        if let Some(deps) = self.dependencies.get(start) {
            for &dep in deps {
                queue.push_back(dep);
            }
        }
        while let Some(node) = queue.pop_front() {
            if !reachable.insert(node) {
                continue;
            }
            if let Some(deps) = self.dependencies.get(node) {
                for &dep in deps {
                    if !reachable.contains(&dep) {
                        queue.push_back(dep);
                    }
                }
            }
        }

        // Order the closure consistently with the full topological order.
        let full = self.topological_order()?;
        let reachable_names: BTreeSet<String> = reachable
            .iter()
            .filter_map(|id| self.order.get(*id).cloned())
            .collect();
        Ok(full
            .into_iter()
            .filter(|candidate| reachable_names.contains(candidate))
            .collect())
    }
}
