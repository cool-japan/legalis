//! Cost modelling for cloud capacity.
//!
//! [`CostModel`] prices capacity from each [`InstanceType`]'s hourly rate, with
//! optional per-instance price overrides (e.g. negotiated rates) and a global
//! discount factor (spot / committed-use). It produces a [`CostEstimate`] for a
//! [`NodePool`] or a concrete set of [`CloudNode`]s over a time horizon.

use super::{CloudNode, InstanceType, NodePool};
use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Approximate hours in a 30.4-day month (for monthly projections).
pub const HOURS_PER_MONTH: f64 = 730.0;

/// A computed cost estimate over a time horizon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostEstimate {
    /// The horizon the estimate covers, in hours.
    pub hours: f64,
    /// Total cost over the horizon, in USD.
    pub total_usd: f64,
    /// Cumulative node-hours (active nodes × hours).
    pub node_hours: f64,
    /// Cost broken down by instance type name.
    pub per_instance_usd: BTreeMap<String, f64>,
}

impl CostEstimate {
    /// The average cost per hour over the horizon.
    #[must_use]
    pub fn cost_per_hour(&self) -> f64 {
        if self.hours <= f64::EPSILON {
            0.0
        } else {
            self.total_usd / self.hours
        }
    }

    /// Projects the same hourly rate over a full month.
    #[must_use]
    pub fn monthly_usd(&self) -> f64 {
        self.cost_per_hour() * HOURS_PER_MONTH
    }
}

/// A pricing model: per-instance overrides plus a global discount multiplier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostModel {
    /// Per-instance-type hourly price overrides (USD/hour).
    pub overrides: BTreeMap<String, f64>,
    /// Global price multiplier in `(0, 1]` (e.g. `0.7` for spot capacity).
    pub discount: f64,
}

impl Default for CostModel {
    fn default() -> Self {
        Self {
            overrides: BTreeMap::new(),
            discount: 1.0,
        }
    }
}

impl CostModel {
    /// Creates an on-demand cost model (no discount, no overrides).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a discounted (e.g. spot) cost model.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if `discount` is not in
    /// `(0, 1]`.
    pub fn spot(discount: f64) -> SimResult<Self> {
        if !(discount > 0.0 && discount <= 1.0) {
            return Err(SimulationError::InvalidParameter(format!(
                "discount must be in (0, 1], got {discount}"
            )));
        }
        Ok(Self {
            overrides: BTreeMap::new(),
            discount,
        })
    }

    /// Builder: overrides the hourly price for one instance type.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if `price` is negative or
    /// non-finite.
    pub fn with_override(
        mut self,
        instance_name: impl Into<String>,
        price: f64,
    ) -> SimResult<Self> {
        if !price.is_finite() || price < 0.0 {
            return Err(SimulationError::InvalidParameter(
                "price override must be finite and non-negative".to_string(),
            ));
        }
        self.overrides.insert(instance_name.into(), price);
        Ok(self)
    }

    /// The effective hourly price for `instance_type` (override or catalog price,
    /// times the discount).
    #[must_use]
    pub fn price_per_hour(&self, instance_type: &InstanceType) -> f64 {
        let base = self
            .overrides
            .get(&instance_type.name)
            .copied()
            .unwrap_or(instance_type.price_per_hour_usd);
        base * self.discount
    }

    /// Validates a time horizon.
    fn check_hours(hours: f64) -> SimResult<()> {
        if !hours.is_finite() || hours < 0.0 {
            return Err(SimulationError::InvalidParameter(format!(
                "hours must be finite and non-negative, got {hours}"
            )));
        }
        Ok(())
    }

    /// Estimates the cost of running `pool` at its desired size for `hours`.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if `hours` is invalid.
    pub fn estimate_pool(&self, pool: &NodePool, hours: f64) -> SimResult<CostEstimate> {
        Self::check_hours(hours)?;
        let count = pool.desired_nodes as f64;
        let price = self.price_per_hour(&pool.instance_type);
        let total = price * count * hours;
        let mut per_instance = BTreeMap::new();
        if count > 0.0 {
            per_instance.insert(pool.instance_type.name.clone(), total);
        }
        Ok(CostEstimate {
            hours,
            total_usd: total,
            node_hours: count * hours,
            per_instance_usd: per_instance,
        })
    }

    /// Estimates the cost of running a concrete set of `nodes` for `hours`.
    ///
    /// Only active (pending/running) nodes are billed; costs are grouped by
    /// instance type.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if `hours` is invalid.
    pub fn estimate_nodes(&self, nodes: &[CloudNode], hours: f64) -> SimResult<CostEstimate> {
        Self::check_hours(hours)?;
        let mut per_instance: BTreeMap<String, f64> = BTreeMap::new();
        let mut active = 0.0;
        for node in nodes.iter().filter(|n| n.is_active()) {
            active += 1.0;
            let price = self.price_per_hour(&node.spec.instance_type);
            *per_instance
                .entry(node.spec.instance_type.name.clone())
                .or_insert(0.0) += price * hours;
        }
        let total = per_instance.values().sum();
        Ok(CostEstimate {
            hours,
            total_usd: total,
            node_hours: active * hours,
            per_instance_usd: per_instance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud::{CloudNode, NodeSpec, NodeState};

    #[test]
    fn test_price_with_override_and_discount() {
        let model = CostModel::new().with_override("medium", 0.08).unwrap();
        // Override beats catalog price; no discount.
        assert!((model.price_per_hour(&InstanceType::medium()) - 0.08).abs() < 1e-9);
        // Non-overridden uses catalog price.
        assert!((model.price_per_hour(&InstanceType::large()) - 0.20).abs() < 1e-9);
        // Spot discount applies multiplicatively.
        let spot = CostModel::spot(0.5).unwrap();
        assert!((spot.price_per_hour(&InstanceType::large()) - 0.10).abs() < 1e-9);
        assert!(CostModel::spot(0.0).is_err());
        assert!(CostModel::spot(1.5).is_err());
        assert!(CostModel::new().with_override("x", -1.0).is_err());
    }

    #[test]
    fn test_estimate_pool() {
        let mut pool = NodePool::new("p", InstanceType::medium(), 1, 10).unwrap();
        pool.set_desired(5);
        let est = CostModel::new().estimate_pool(&pool, 10.0).unwrap();
        // 5 nodes * $0.10 * 10h = $5.00.
        assert!((est.total_usd - 5.0).abs() < 1e-9);
        assert!((est.node_hours - 50.0).abs() < 1e-9);
        assert!((est.cost_per_hour() - 0.5).abs() < 1e-9);
        assert!((est.monthly_usd() - 0.5 * HOURS_PER_MONTH).abs() < 1e-9);
        assert!(est.per_instance_usd.contains_key("medium"));
        assert!(CostModel::new().estimate_pool(&pool, -1.0).is_err());
    }

    #[test]
    fn test_estimate_nodes_groups_and_skips_inactive() {
        let mk = |id: &str, it: InstanceType, st: NodeState| {
            CloudNode::new(id, NodeSpec::new(it, "r"), st)
        };
        let nodes = vec![
            mk("a", InstanceType::small(), NodeState::Running),
            mk("b", InstanceType::small(), NodeState::Running),
            mk("c", InstanceType::large(), NodeState::Running),
            mk("d", InstanceType::large(), NodeState::Terminated), // not billed
        ];
        let est = CostModel::new().estimate_nodes(&nodes, 2.0).unwrap();
        // 2 small * 0.05 * 2 = 0.20 ; 1 large * 0.20 * 2 = 0.40 ; total 0.60.
        assert!((est.total_usd - 0.60).abs() < 1e-9);
        assert!((est.node_hours - 6.0).abs() < 1e-9); // 3 active * 2h
        assert!((est.per_instance_usd["small"] - 0.20).abs() < 1e-9);
        assert!((est.per_instance_usd["large"] - 0.40).abs() < 1e-9);
    }
}
