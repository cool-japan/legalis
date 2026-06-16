//! Cloud-native scaling abstraction (provider-agnostic).
//!
//! This module provides a **pure-Rust, provider-agnostic** model of elastic
//! cloud capacity for large-scale simulations. It is built from four cooperating
//! pieces, each in its own focused submodule, plus a simulated in-memory backend:
//!
//! - **Node pool** (this module) — [`InstanceType`], [`NodeSpec`], [`CloudNode`]
//!   and [`NodePool`] describe heterogeneous compute capacity, and the
//!   [`CloudProvider`] trait abstracts provisioning/termination across any
//!   backend.
//! - **Autoscaling policy** ([`autoscaling`]) — an HPA-style
//!   [`autoscaling::AutoscalingPolicy`] turns observed [`autoscaling::ClusterLoad`]
//!   into a [`autoscaling::ScalingDecision`] with a dead-band and a per-step cap.
//! - **Work distribution plan** ([`distribution`]) — partitions weighted
//!   [`distribution::WorkUnit`]s across nodes (round-robin, hash, or greedy
//!   longest-processing-time bin-packing) into a balanced
//!   [`distribution::WorkDistributionPlan`].
//! - **Cost model** ([`cost`]) — a [`cost::CostModel`] with per-instance price
//!   overrides and a discount factor (spot/committed-use) producing a
//!   [`cost::CostEstimate`].
//!
//! The [`simulated::SimulatedCloud`] backend implements [`CloudProvider`] entirely
//! in memory so the whole pipeline — provision → distribute → autoscale → cost —
//! is exercisable and testable offline.
//!
//! # Deferred: live provider SDKs (AWS / GCP / Azure)
//!
//! Binding the live AWS / GCP / Azure SDKs (EC2 Auto Scaling Groups, GKE / GCE
//! managed instance groups, Azure VM Scale Sets) requires network access and
//! cloud credentials this offline workspace does not have. Those bindings are
//! intentionally **deferred**: a production deployment implements [`CloudProvider`]
//! over the relevant SDK (e.g. an `Ec2Provider`, `GceProvider`, `AzureVmssProvider`)
//! without changing any caller — the autoscaler, distribution planner and cost
//! model all operate against the trait and the pure data types defined here. The
//! [`simulated::SimulatedCloud`] backend stands in for them in this build.
//!
//! # Example
//!
//! ```
//! use legalis_sim::cloud::{
//!     CloudProvider, InstanceType, NodePool, SimulatedCloud,
//! };
//!
//! let mut cloud = SimulatedCloud::new("sim");
//! let mut pool = NodePool::new("workers", InstanceType::medium(), 1, 8).unwrap();
//! pool.set_desired(4);
//! let report = cloud.reconcile(&pool).unwrap();
//! assert_eq!(report.final_count, 4);
//! assert_eq!(cloud.node_count(), 4);
//! ```

pub mod autoscaling;
pub mod cost;
pub mod distribution;
pub mod simulated;

pub use autoscaling::{AutoscalingPolicy, ClusterLoad, ScalingDecision, ScalingDirection};
pub use cost::{CostEstimate, CostModel};
pub use distribution::{
    DistributionStrategy, NodeAssignment, WorkDistributionPlan, WorkUnit, distribute,
};
pub use simulated::SimulatedCloud;

use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A cloud node identifier (opaque, provider-assigned).
pub type CloudNodeId = String;

/// The [`NodeSpec`] label key used to associate a node with its [`NodePool`].
pub const POOL_LABEL: &str = "legalis.pool";

/// A provider-agnostic compute instance type (shape + price).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceType {
    /// Instance type name (e.g. "medium").
    pub name: String,
    /// Number of virtual CPUs.
    pub vcpus: u32,
    /// Memory, in gibibytes.
    pub memory_gib: f64,
    /// Representative on-demand price, in USD per hour.
    pub price_per_hour_usd: f64,
}

impl InstanceType {
    /// Creates an instance type, validating that resources/price are sensible.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if vCPUs are zero or
    /// memory/price are negative or non-finite.
    pub fn new(
        name: impl Into<String>,
        vcpus: u32,
        memory_gib: f64,
        price_per_hour_usd: f64,
    ) -> SimResult<Self> {
        if vcpus == 0 {
            return Err(SimulationError::InvalidParameter(
                "instance type must have at least one vCPU".to_string(),
            ));
        }
        if !memory_gib.is_finite() || memory_gib < 0.0 {
            return Err(SimulationError::InvalidParameter(
                "instance memory must be finite and non-negative".to_string(),
            ));
        }
        if !price_per_hour_usd.is_finite() || price_per_hour_usd < 0.0 {
            return Err(SimulationError::InvalidParameter(
                "instance price must be finite and non-negative".to_string(),
            ));
        }
        Ok(Self {
            name: name.into(),
            vcpus,
            memory_gib,
            price_per_hour_usd,
        })
    }

    /// A 2-vCPU / 4 GiB small instance.
    #[must_use]
    pub fn small() -> Self {
        Self {
            name: "small".to_string(),
            vcpus: 2,
            memory_gib: 4.0,
            price_per_hour_usd: 0.05,
        }
    }

    /// A 4-vCPU / 16 GiB medium instance.
    #[must_use]
    pub fn medium() -> Self {
        Self {
            name: "medium".to_string(),
            vcpus: 4,
            memory_gib: 16.0,
            price_per_hour_usd: 0.10,
        }
    }

    /// An 8-vCPU / 32 GiB large instance.
    #[must_use]
    pub fn large() -> Self {
        Self {
            name: "large".to_string(),
            vcpus: 8,
            memory_gib: 32.0,
            price_per_hour_usd: 0.20,
        }
    }

    /// A 16-vCPU / 64 GiB extra-large instance.
    #[must_use]
    pub fn xlarge() -> Self {
        Self {
            name: "xlarge".to_string(),
            vcpus: 16,
            memory_gib: 64.0,
            price_per_hour_usd: 0.40,
        }
    }

    /// A representative provider-agnostic catalog (small → xlarge).
    #[must_use]
    pub fn standard_catalog() -> Vec<Self> {
        vec![Self::small(), Self::medium(), Self::large(), Self::xlarge()]
    }
}

/// A specification used to provision a node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSpec {
    /// The instance type to provision.
    pub instance_type: InstanceType,
    /// The region / availability zone.
    pub region: String,
    /// Free-form labels (used to associate nodes with a pool, etc.).
    pub labels: BTreeMap<String, String>,
}

impl NodeSpec {
    /// Creates a spec for `instance_type` in `region`.
    #[must_use]
    pub fn new(instance_type: InstanceType, region: impl Into<String>) -> Self {
        Self {
            instance_type,
            region: region.into(),
            labels: BTreeMap::new(),
        }
    }

    /// Builder: attaches a label.
    #[must_use]
    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }
}

/// The lifecycle state of a [`CloudNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeState {
    /// Requested, not yet ready.
    Pending,
    /// Ready and accepting work.
    Running,
    /// Draining / shutting down.
    Terminating,
    /// Gone.
    Terminated,
}

impl NodeState {
    /// Whether a node in this state contributes usable capacity.
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(self, NodeState::Pending | NodeState::Running)
    }
}

/// A provisioned cloud node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudNode {
    /// Provider-assigned id.
    pub id: CloudNodeId,
    /// The spec the node was provisioned from.
    pub spec: NodeSpec,
    /// Lifecycle state.
    pub state: NodeState,
}

impl CloudNode {
    /// Creates a node in the given state.
    #[must_use]
    pub fn new(id: impl Into<CloudNodeId>, spec: NodeSpec, state: NodeState) -> Self {
        Self {
            id: id.into(),
            spec,
            state,
        }
    }

    /// Whether the node currently contributes usable capacity.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// The vCPU capacity this node contributes (0 if inactive).
    #[must_use]
    pub fn vcpus(&self) -> u32 {
        if self.is_active() {
            self.spec.instance_type.vcpus
        } else {
            0
        }
    }

    /// The id of the pool this node belongs to, if labelled.
    #[must_use]
    pub fn pool(&self) -> Option<&str> {
        self.spec.labels.get(POOL_LABEL).map(String::as_str)
    }
}

/// A managed, bounded group of identical nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodePool {
    /// Pool name (used as the node pool label).
    pub name: String,
    /// The instance type every node in the pool uses.
    pub instance_type: InstanceType,
    /// Region the pool provisions into.
    pub region: String,
    /// Minimum node count.
    pub min_nodes: usize,
    /// Maximum node count.
    pub max_nodes: usize,
    /// Currently desired node count (always within `[min, max]`).
    pub desired_nodes: usize,
}

impl NodePool {
    /// Creates a pool with `min_nodes` desired, validating that `min <= max`.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidConfiguration`] if `min_nodes >
    /// max_nodes`.
    pub fn new(
        name: impl Into<String>,
        instance_type: InstanceType,
        min_nodes: usize,
        max_nodes: usize,
    ) -> SimResult<Self> {
        if min_nodes > max_nodes {
            return Err(SimulationError::InvalidConfiguration(format!(
                "node pool min ({min_nodes}) exceeds max ({max_nodes})"
            )));
        }
        Ok(Self {
            name: name.into(),
            instance_type,
            region: "default".to_string(),
            min_nodes,
            max_nodes,
            desired_nodes: min_nodes,
        })
    }

    /// Builder: sets the region.
    #[must_use]
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = region.into();
        self
    }

    /// Sets the desired node count, clamping to `[min_nodes, max_nodes]`.
    /// Returns the clamped value.
    pub fn set_desired(&mut self, desired: usize) -> usize {
        self.desired_nodes = desired.clamp(self.min_nodes, self.max_nodes);
        self.desired_nodes
    }

    /// The total vCPU capacity at the desired node count.
    #[must_use]
    pub fn capacity_vcpus(&self) -> u32 {
        self.instance_type.vcpus * self.desired_nodes as u32
    }

    /// The [`NodeSpec`] used to provision nodes for this pool (with the pool
    /// label attached).
    #[must_use]
    pub fn node_spec(&self) -> NodeSpec {
        NodeSpec::new(self.instance_type.clone(), self.region.clone())
            .with_label(POOL_LABEL, self.name.clone())
    }
}

/// A report describing the effect of a [`CloudProvider::reconcile`] call.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconcileReport {
    /// Nodes newly provisioned.
    pub provisioned: usize,
    /// Nodes terminated.
    pub terminated: usize,
    /// Active node count for the pool after reconciliation.
    pub final_count: usize,
}

/// A provider-agnostic interface to elastic cloud capacity.
///
/// Implement this over a live SDK (AWS/GCP/Azure) for production; the
/// [`SimulatedCloud`] in-memory implementation is used offline. The autoscaler,
/// distribution planner and cost model all operate against this trait and the
/// pure data types in this module, so swapping backends changes no callers.
pub trait CloudProvider {
    /// A human-readable backend name.
    fn provider_name(&self) -> &str;

    /// The instance types this provider offers.
    fn instance_catalog(&self) -> Vec<InstanceType>;

    /// Provisions `count` nodes from `spec`, returning their ids.
    ///
    /// # Errors
    ///
    /// Returns a [`SimulationError`] if provisioning fails (e.g. a quota the
    /// backend enforces).
    fn provision(&mut self, spec: &NodeSpec, count: usize) -> SimResult<Vec<CloudNodeId>>;

    /// Terminates the node with `node_id`.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if no such node exists.
    fn terminate(&mut self, node_id: &str) -> SimResult<()>;

    /// A snapshot of all nodes known to the provider.
    fn nodes(&self) -> Vec<CloudNode>;

    /// The number of currently active (pending or running) nodes.
    fn node_count(&self) -> usize {
        self.nodes().iter().filter(|n| n.is_active()).count()
    }

    /// The total active vCPU capacity across all nodes.
    fn total_vcpus(&self) -> u32 {
        self.nodes().iter().map(CloudNode::vcpus).sum()
    }

    /// Active node ids belonging to `pool`.
    fn pool_nodes(&self, pool: &str) -> Vec<CloudNodeId> {
        self.nodes()
            .into_iter()
            .filter(|n| n.is_active() && n.pool() == Some(pool))
            .map(|n| n.id)
            .collect()
    }

    /// Brings the active node count for `pool` to `pool.desired_nodes`,
    /// provisioning or terminating as needed.
    ///
    /// # Errors
    ///
    /// Propagates any [`SimulationError`] from [`CloudProvider::provision`] or
    /// [`CloudProvider::terminate`].
    fn reconcile(&mut self, pool: &NodePool) -> SimResult<ReconcileReport> {
        let current = self.pool_nodes(&pool.name);
        let mut report = ReconcileReport::default();
        if current.len() < pool.desired_nodes {
            let add = pool.desired_nodes - current.len();
            self.provision(&pool.node_spec(), add)?;
            report.provisioned = add;
        } else if current.len() > pool.desired_nodes {
            let remove = current.len() - pool.desired_nodes;
            for id in current.iter().take(remove) {
                self.terminate(id)?;
                report.terminated += 1;
            }
        }
        report.final_count = self.pool_nodes(&pool.name).len();
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_type_validation_and_catalog() {
        assert!(InstanceType::new("bad", 0, 4.0, 0.05).is_err());
        assert!(InstanceType::new("bad", 2, -1.0, 0.05).is_err());
        assert!(InstanceType::new("bad", 2, 4.0, -0.05).is_err());
        let ok = InstanceType::new("custom", 4, 8.0, 0.12).unwrap();
        assert_eq!(ok.vcpus, 4);
        let catalog = InstanceType::standard_catalog();
        assert_eq!(catalog.len(), 4);
        assert!(catalog.iter().any(|i| i.name == "medium"));
    }

    #[test]
    fn test_node_pool_bounds_and_desired_clamp() {
        assert!(NodePool::new("p", InstanceType::small(), 5, 2).is_err());
        let mut pool = NodePool::new("workers", InstanceType::medium(), 2, 6).unwrap();
        assert_eq!(pool.desired_nodes, 2);
        assert_eq!(pool.set_desired(100), 6); // clamped to max
        assert_eq!(pool.set_desired(0), 2); // clamped to min
        pool.set_desired(4);
        assert_eq!(pool.capacity_vcpus(), 4 * 4);
        let spec = pool.node_spec();
        assert_eq!(
            spec.labels.get(POOL_LABEL).map(String::as_str),
            Some("workers")
        );
    }

    #[test]
    fn test_node_state_and_active_capacity() {
        let spec = NodeSpec::new(InstanceType::large(), "us-east");
        let running = CloudNode::new("n1", spec.clone(), NodeState::Running);
        let terminated = CloudNode::new("n2", spec, NodeState::Terminated);
        assert!(running.is_active());
        assert_eq!(running.vcpus(), 8);
        assert!(!terminated.is_active());
        assert_eq!(terminated.vcpus(), 0);
    }
}
