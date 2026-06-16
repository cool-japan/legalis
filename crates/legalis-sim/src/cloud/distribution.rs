//! Work-distribution planning: partitioning weighted work across nodes.
//!
//! Given a set of weighted [`WorkUnit`]s (e.g. population shards, each weighted by
//! entity count) and a set of node ids, [`distribute`] produces a balanced
//! [`WorkDistributionPlan`] under one of several strategies — round-robin, hash,
//! or greedy longest-processing-time (LPT) bin-packing.

use super::CloudNodeId;
use crate::immersive::fnv1a;
use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};

/// A single unit of work with a relative cost / weight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkUnit {
    /// Stable unit id.
    pub id: String,
    /// Relative cost (e.g. number of entities); clamped to `≥ 0`.
    pub cost: f64,
}

impl WorkUnit {
    /// Creates a work unit, clamping `cost` to `≥ 0` (non-finite → `0`).
    #[must_use]
    pub fn new(id: impl Into<String>, cost: f64) -> Self {
        let cost = if cost.is_finite() { cost.max(0.0) } else { 0.0 };
        Self {
            id: id.into(),
            cost,
        }
    }
}

/// The strategy used to assign work units to nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributionStrategy {
    /// Cycle units across nodes in order, ignoring cost.
    RoundRobin,
    /// Assign each unit to a node chosen by a hash of its id (sticky placement).
    Hash,
    /// Greedy longest-processing-time bin-packing (best balance).
    GreedyLpt,
}

impl DistributionStrategy {
    /// A short, stable string tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            DistributionStrategy::RoundRobin => "round-robin",
            DistributionStrategy::Hash => "hash",
            DistributionStrategy::GreedyLpt => "greedy-lpt",
        }
    }
}

/// The set of work units assigned to a single node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeAssignment {
    /// The node this assignment is for.
    pub node_id: CloudNodeId,
    /// Ids of the units assigned to this node.
    pub unit_ids: Vec<String>,
    /// Total cost assigned to this node.
    pub load: f64,
}

impl NodeAssignment {
    /// Creates an empty assignment for `node_id`.
    #[must_use]
    fn empty(node_id: CloudNodeId) -> Self {
        Self {
            node_id,
            unit_ids: Vec::new(),
            load: 0.0,
        }
    }

    /// Number of units assigned.
    #[must_use]
    pub fn len(&self) -> usize {
        self.unit_ids.len()
    }

    /// Whether the node has no units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.unit_ids.is_empty()
    }
}

/// A complete plan mapping every work unit to a node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkDistributionPlan {
    /// Per-node assignments (one entry per node, in the input order).
    pub assignments: Vec<NodeAssignment>,
    /// The strategy that produced the plan.
    pub strategy: DistributionStrategy,
}

impl WorkDistributionPlan {
    /// The number of nodes in the plan.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.assignments.len()
    }

    /// The total number of assigned units.
    #[must_use]
    pub fn total_units(&self) -> usize {
        self.assignments.iter().map(NodeAssignment::len).sum()
    }

    /// The total cost across all nodes.
    #[must_use]
    pub fn total_load(&self) -> f64 {
        self.assignments.iter().map(|a| a.load).sum()
    }

    /// The maximum per-node load (the plan's makespan).
    #[must_use]
    pub fn makespan(&self) -> f64 {
        self.assignments
            .iter()
            .map(|a| a.load)
            .fold(0.0_f64, f64::max)
    }

    /// The minimum per-node load.
    #[must_use]
    pub fn min_load(&self) -> f64 {
        self.assignments
            .iter()
            .map(|a| a.load)
            .fold(f64::INFINITY, f64::min)
    }

    /// A balance ratio in `[0, 1]`: `min_load / makespan` (`1.0` is perfectly
    /// balanced; defined as `1.0` when every node is empty).
    #[must_use]
    pub fn balance_ratio(&self) -> f64 {
        let makespan = self.makespan();
        if makespan <= f64::EPSILON {
            1.0
        } else {
            (self.min_load() / makespan).clamp(0.0, 1.0)
        }
    }

    /// The node a given unit was assigned to, if any.
    #[must_use]
    pub fn node_for_unit(&self, unit_id: &str) -> Option<&str> {
        self.assignments
            .iter()
            .find(|a| a.unit_ids.iter().any(|u| u == unit_id))
            .map(|a| a.node_id.as_str())
    }

    /// Serialises the plan to pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::Serialization`] if serialisation fails.
    pub fn to_json(&self) -> SimResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// Builds a [`WorkDistributionPlan`] assigning `units` across `node_ids`.
///
/// # Errors
///
/// Returns [`SimulationError::InvalidConfiguration`] if `node_ids` is empty.
pub fn distribute(
    units: &[WorkUnit],
    node_ids: &[CloudNodeId],
    strategy: DistributionStrategy,
) -> SimResult<WorkDistributionPlan> {
    if node_ids.is_empty() {
        return Err(SimulationError::InvalidConfiguration(
            "cannot distribute work across zero nodes".to_string(),
        ));
    }
    let mut assignments: Vec<NodeAssignment> = node_ids
        .iter()
        .map(|id| NodeAssignment::empty(id.clone()))
        .collect();
    let n = assignments.len();

    match strategy {
        DistributionStrategy::RoundRobin => {
            for (i, unit) in units.iter().enumerate() {
                let slot = i % n;
                assign(&mut assignments[slot], unit);
            }
        }
        DistributionStrategy::Hash => {
            for unit in units {
                let slot = (fnv1a(&unit.id) % n as u64) as usize;
                assign(&mut assignments[slot], unit);
            }
        }
        DistributionStrategy::GreedyLpt => {
            // Sort units by descending cost, tie-broken by id for determinism.
            let mut order: Vec<&WorkUnit> = units.iter().collect();
            order.sort_by(|a, b| {
                b.cost
                    .partial_cmp(&a.cost)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.id.cmp(&b.id))
            });
            for unit in order {
                let slot = least_loaded(&assignments);
                assign(&mut assignments[slot], unit);
            }
        }
    }

    Ok(WorkDistributionPlan {
        assignments,
        strategy,
    })
}

/// Adds `unit` to `assignment`, updating its load.
fn assign(assignment: &mut NodeAssignment, unit: &WorkUnit) {
    assignment.unit_ids.push(unit.id.clone());
    assignment.load += unit.cost;
}

/// Index of the least-loaded assignment (tie-broken by the lowest index).
fn least_loaded(assignments: &[NodeAssignment]) -> usize {
    let mut best = 0;
    let mut best_load = f64::INFINITY;
    for (i, a) in assignments.iter().enumerate() {
        if a.load < best_load {
            best_load = a.load;
            best = i;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nodes(n: usize) -> Vec<CloudNodeId> {
        (0..n).map(|i| format!("node-{i}")).collect()
    }

    fn units(costs: &[f64]) -> Vec<WorkUnit> {
        costs
            .iter()
            .enumerate()
            .map(|(i, &c)| WorkUnit::new(format!("u{i}"), c))
            .collect()
    }

    #[test]
    fn test_distribute_requires_nodes() {
        let result = distribute(&units(&[1.0]), &[], DistributionStrategy::RoundRobin);
        assert!(result.is_err());
    }

    #[test]
    fn test_round_robin_cycles_units() {
        let plan = distribute(
            &units(&[1.0, 1.0, 1.0, 1.0]),
            &nodes(2),
            DistributionStrategy::RoundRobin,
        )
        .unwrap();
        assert_eq!(plan.total_units(), 4);
        assert_eq!(plan.assignments[0].len(), 2);
        assert_eq!(plan.assignments[1].len(), 2);
        // u0,u2 → node-0 ; u1,u3 → node-1.
        assert_eq!(plan.node_for_unit("u0"), Some("node-0"));
        assert_eq!(plan.node_for_unit("u1"), Some("node-1"));
    }

    #[test]
    fn test_hash_is_deterministic_and_sticky() {
        let plan_a = distribute(&units(&[1.0; 8]), &nodes(3), DistributionStrategy::Hash).unwrap();
        let plan_b = distribute(&units(&[1.0; 8]), &nodes(3), DistributionStrategy::Hash).unwrap();
        // Same inputs → identical placement.
        for i in 0..8 {
            let id = format!("u{i}");
            assert_eq!(plan_a.node_for_unit(&id), plan_b.node_for_unit(&id));
        }
        assert_eq!(plan_a.total_units(), 8);
    }

    #[test]
    fn test_greedy_lpt_balances_better_than_round_robin() {
        // Skewed costs: LPT should balance much better.
        let costs = [10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let lpt = distribute(&units(&costs), &nodes(3), DistributionStrategy::GreedyLpt).unwrap();
        let rr = distribute(&units(&costs), &nodes(3), DistributionStrategy::RoundRobin).unwrap();
        assert!((lpt.total_load() - 55.0).abs() < 1e-9);
        assert!(lpt.balance_ratio() >= rr.balance_ratio());
        // LPT makespan is near the optimal lower bound (ceil(55/3)=19).
        assert!(lpt.makespan() <= 20.0);
    }

    #[test]
    fn test_balance_ratio_and_json_roundtrip() {
        let plan = distribute(
            &units(&[1.0, 1.0, 1.0]),
            &nodes(3),
            DistributionStrategy::GreedyLpt,
        )
        .unwrap();
        // Perfectly balanced: each node one unit of cost 1.
        assert!((plan.balance_ratio() - 1.0).abs() < 1e-9);
        let empty = distribute(&[], &nodes(2), DistributionStrategy::GreedyLpt).unwrap();
        assert_eq!(empty.total_units(), 0);
        assert!((empty.balance_ratio() - 1.0).abs() < 1e-9); // empty defined as balanced
        let json = plan.to_json().expect("json");
        let restored: WorkDistributionPlan = serde_json::from_str(&json).expect("roundtrip");
        assert_eq!(restored.node_count(), plan.node_count());
        assert_eq!(restored.strategy, DistributionStrategy::GreedyLpt);
    }
}
