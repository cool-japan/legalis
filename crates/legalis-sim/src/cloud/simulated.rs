//! An in-memory [`CloudProvider`] backend.
//!
//! [`SimulatedCloud`] stands in for a live cloud provider entirely in memory:
//! provisioning creates `Running` nodes immediately and termination marks them
//! `Terminated` (history is retained). It honours an optional active-node quota,
//! so quota-exhaustion paths are exercisable offline. The live AWS/GCP/Azure
//! backends are deferred (see the [cloud module overview](super)).

use super::{CloudNode, CloudNodeId, CloudProvider, InstanceType, NodeSpec, NodeState};
use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};

/// A simulated, in-memory cloud provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedCloud {
    name: String,
    nodes: Vec<CloudNode>,
    next_id: u64,
    catalog: Vec<InstanceType>,
    quota: Option<usize>,
}

impl SimulatedCloud {
    /// Creates an empty simulated cloud with the standard instance catalog and no
    /// quota.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: Vec::new(),
            next_id: 0,
            catalog: InstanceType::standard_catalog(),
            quota: None,
        }
    }

    /// Builder: caps the number of simultaneously active nodes.
    #[must_use]
    pub fn with_quota(mut self, max_active_nodes: usize) -> Self {
        self.quota = Some(max_active_nodes);
        self
    }

    /// The number of currently active nodes.
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.is_active()).count()
    }

    /// The total number of node records ever created (including terminated).
    #[must_use]
    pub fn history_len(&self) -> usize {
        self.nodes.len()
    }

    /// Allocates a fresh node id.
    fn alloc_id(&mut self) -> CloudNodeId {
        let id = format!("{}-node-{}", self.name, self.next_id);
        self.next_id += 1;
        id
    }
}

impl CloudProvider for SimulatedCloud {
    fn provider_name(&self) -> &str {
        &self.name
    }

    fn instance_catalog(&self) -> Vec<InstanceType> {
        self.catalog.clone()
    }

    fn provision(&mut self, spec: &NodeSpec, count: usize) -> SimResult<Vec<CloudNodeId>> {
        if let Some(quota) = self.quota
            && self.active_count() + count > quota
        {
            return Err(SimulationError::InvalidConfiguration(format!(
                "provisioning {count} nodes would exceed active quota {quota} \
                 (currently {} active)",
                self.active_count()
            )));
        }
        let mut ids = Vec::with_capacity(count);
        for _ in 0..count {
            let id = self.alloc_id();
            self.nodes
                .push(CloudNode::new(id.clone(), spec.clone(), NodeState::Running));
            ids.push(id);
        }
        Ok(ids)
    }

    fn terminate(&mut self, node_id: &str) -> SimResult<()> {
        match self
            .nodes
            .iter_mut()
            .find(|n| n.id == node_id && n.is_active())
        {
            Some(node) => {
                node.state = NodeState::Terminated;
                Ok(())
            }
            None => Err(SimulationError::InvalidParameter(format!(
                "no active node with id '{node_id}'"
            ))),
        }
    }

    fn nodes(&self) -> Vec<CloudNode> {
        self.nodes.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud::{
        AutoscalingPolicy, ClusterLoad, CostModel, DistributionStrategy, NodePool,
        ScalingDirection, WorkUnit, distribute,
    };

    #[test]
    fn test_provision_and_terminate() {
        let mut cloud = SimulatedCloud::new("sim");
        let spec = NodeSpec::new(InstanceType::large(), "us-east");
        let ids = cloud.provision(&spec, 3).unwrap();
        assert_eq!(ids.len(), 3);
        assert_eq!(cloud.node_count(), 3);
        assert_eq!(cloud.total_vcpus(), 3 * 8);
        // Terminate one.
        cloud.terminate(&ids[0]).unwrap();
        assert_eq!(cloud.node_count(), 2);
        // Terminating again / unknown errors.
        assert!(cloud.terminate(&ids[0]).is_err());
        assert!(cloud.terminate("ghost").is_err());
        // History retains the terminated node.
        assert_eq!(cloud.history_len(), 3);
    }

    #[test]
    fn test_quota_enforced() {
        let mut cloud = SimulatedCloud::new("sim").with_quota(2);
        let spec = NodeSpec::new(InstanceType::small(), "r");
        assert!(cloud.provision(&spec, 2).is_ok());
        assert!(cloud.provision(&spec, 1).is_err()); // would exceed quota
        assert_eq!(cloud.node_count(), 2);
    }

    #[test]
    fn test_reconcile_scales_pool_up_and_down() {
        let mut cloud = SimulatedCloud::new("sim");
        let mut pool = NodePool::new("workers", InstanceType::medium(), 1, 10).unwrap();
        pool.set_desired(5);
        let up = cloud.reconcile(&pool).unwrap();
        assert_eq!(up.provisioned, 5);
        assert_eq!(up.final_count, 5);
        assert_eq!(cloud.pool_nodes("workers").len(), 5);

        // Scale down to 2.
        pool.set_desired(2);
        let down = cloud.reconcile(&pool).unwrap();
        assert_eq!(down.terminated, 3);
        assert_eq!(down.final_count, 2);
        assert_eq!(cloud.node_count(), 2);

        // Idempotent reconcile is a no-op.
        let same = cloud.reconcile(&pool).unwrap();
        assert_eq!(same.provisioned, 0);
        assert_eq!(same.terminated, 0);
        assert_eq!(same.final_count, 2);
    }

    #[test]
    fn test_pool_isolation_by_label() {
        let mut cloud = SimulatedCloud::new("sim");
        let mut a = NodePool::new("pool-a", InstanceType::small(), 0, 10).unwrap();
        let mut b = NodePool::new("pool-b", InstanceType::large(), 0, 10).unwrap();
        a.set_desired(2);
        b.set_desired(3);
        cloud.reconcile(&a).unwrap();
        cloud.reconcile(&b).unwrap();
        assert_eq!(cloud.pool_nodes("pool-a").len(), 2);
        assert_eq!(cloud.pool_nodes("pool-b").len(), 3);
        assert_eq!(cloud.node_count(), 5);
    }

    #[test]
    fn test_end_to_end_scaling_pipeline() {
        // Provision -> observe load -> autoscale -> reconcile -> distribute -> cost.
        let mut cloud = SimulatedCloud::new("prod");
        let mut pool = NodePool::new("sim-workers", InstanceType::medium(), 1, 20).unwrap();
        pool.set_desired(2);
        cloud.reconcile(&pool).unwrap();
        assert_eq!(cloud.node_count(), 2);

        // Heavy backlog: 2 nodes * 10 slots = 20 capacity, demand 80.
        let policy = AutoscalingPolicy::new(0.7).unwrap();
        let load = ClusterLoad::new(40, 40, 10.0);
        let decision = policy.evaluate(&pool, &load);
        assert_eq!(decision.direction, ScalingDirection::Out);
        pool.set_desired(decision.target_nodes);
        cloud.reconcile(&pool).unwrap();
        assert_eq!(cloud.node_count(), decision.target_nodes);

        // Distribute 100 weighted shards across the scaled-out pool.
        let units: Vec<WorkUnit> = (0..100)
            .map(|i| WorkUnit::new(format!("shard-{i}"), 1.0 + (i % 5) as f64))
            .collect();
        let node_ids = cloud.pool_nodes("sim-workers");
        let plan = distribute(&units, &node_ids, DistributionStrategy::GreedyLpt).unwrap();
        assert_eq!(plan.total_units(), 100);
        assert!(plan.balance_ratio() > 0.9); // well balanced

        // Cost of the running fleet for an 8-hour batch (spot pricing).
        let model = CostModel::spot(0.4).unwrap();
        let est = model.estimate_nodes(&cloud.nodes(), 8.0).unwrap();
        assert!(est.total_usd > 0.0);
        assert!((est.node_hours - decision.target_nodes as f64 * 8.0).abs() < 1e-9);
    }
}
