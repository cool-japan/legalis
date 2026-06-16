//! Autoscaling policy: turning observed load into scaling decisions.
//!
//! [`AutoscalingPolicy`] follows the Horizontal-Pod-Autoscaler pattern: it sizes
//! a [`NodePool`] so that observed utilisation tracks a target, but only acts
//! when utilisation leaves a configurable dead-band, and never changes the node
//! count by more than `max_step` in a single decision (damping oscillation).

use super::NodePool;
use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};

/// A snapshot of the work pressure on a pool.
///
/// `slots_per_node` is how many concurrent work units one node can serve;
/// utilisation is measured against the pool's *current* desired node count.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ClusterLoad {
    /// Work units currently executing.
    pub running_units: usize,
    /// Work units waiting in the queue.
    pub pending_units: usize,
    /// Concurrent work units one node can serve.
    pub slots_per_node: f64,
}

impl ClusterLoad {
    /// Creates a load snapshot (clamping `slots_per_node` to `≥ 0`).
    #[must_use]
    pub fn new(running_units: usize, pending_units: usize, slots_per_node: f64) -> Self {
        Self {
            running_units,
            pending_units,
            slots_per_node: slots_per_node.max(0.0),
        }
    }

    /// Total demand (running plus pending).
    #[must_use]
    pub fn demand_units(&self) -> usize {
        self.running_units + self.pending_units
    }

    /// Utilisation at `nodes` node count: `demand / (slots_per_node * nodes)`.
    ///
    /// Returns `f64::INFINITY` when there is demand but no capacity, and `0.0`
    /// when there is no demand.
    #[must_use]
    pub fn utilization_at(&self, nodes: usize) -> f64 {
        let demand = self.demand_units() as f64;
        if demand <= 0.0 {
            return 0.0;
        }
        let slots = self.slots_per_node * nodes as f64;
        if slots <= f64::EPSILON {
            f64::INFINITY
        } else {
            demand / slots
        }
    }
}

/// The direction of a scaling decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScalingDirection {
    /// Add nodes.
    Out,
    /// Remove nodes.
    In,
    /// Leave the node count unchanged.
    Hold,
}

impl ScalingDirection {
    /// A short, stable string tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            ScalingDirection::Out => "out",
            ScalingDirection::In => "in",
            ScalingDirection::Hold => "hold",
        }
    }
}

/// The outcome of evaluating an [`AutoscalingPolicy`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalingDecision {
    /// Which way to scale.
    pub direction: ScalingDirection,
    /// The node count before the decision.
    pub current_nodes: usize,
    /// The recommended node count after the decision.
    pub target_nodes: usize,
    /// Utilisation at the current node count.
    pub utilization: f64,
    /// Human-readable rationale.
    pub reason: String,
}

impl ScalingDecision {
    /// The signed change in node count (`target - current`).
    #[must_use]
    pub fn delta(&self) -> i64 {
        self.target_nodes as i64 - self.current_nodes as i64
    }
}

/// A target-tracking autoscaling policy with a dead-band and per-step cap.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoscalingPolicy {
    /// Desired steady-state utilisation in `(0, 1]`.
    pub target_utilization: f64,
    /// Utilisation at or above which scale-out is permitted.
    pub scale_out_threshold: f64,
    /// Utilisation at or below which scale-in is permitted.
    pub scale_in_threshold: f64,
    /// Maximum node-count change per decision (`0` = unbounded).
    pub max_step: usize,
}

impl AutoscalingPolicy {
    /// Creates a policy targeting `target_utilization`, deriving a symmetric-ish
    /// dead-band (`±` around the target).
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if `target_utilization` is
    /// not in `(0, 1]`.
    pub fn new(target_utilization: f64) -> SimResult<Self> {
        if !(target_utilization > 0.0 && target_utilization <= 1.0) {
            return Err(SimulationError::InvalidParameter(format!(
                "target utilization must be in (0, 1], got {target_utilization}"
            )));
        }
        Ok(Self {
            target_utilization,
            scale_out_threshold: (target_utilization + 0.1).min(1.0),
            scale_in_threshold: (target_utilization - 0.2).max(0.0),
            max_step: 0,
        })
    }

    /// Builder: sets explicit dead-band thresholds.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if `scale_in >= scale_out`.
    pub fn with_thresholds(mut self, scale_in: f64, scale_out: f64) -> SimResult<Self> {
        if !scale_in.is_finite() || !scale_out.is_finite() || scale_in >= scale_out {
            return Err(SimulationError::InvalidParameter(format!(
                "scale-in threshold ({scale_in}) must be below scale-out ({scale_out})"
            )));
        }
        self.scale_in_threshold = scale_in;
        self.scale_out_threshold = scale_out;
        Ok(self)
    }

    /// Builder: caps the per-decision node-count change.
    #[must_use]
    pub fn with_max_step(mut self, max_step: usize) -> Self {
        self.max_step = max_step;
        self
    }

    /// The node count needed to bring `load` to the target utilisation.
    fn needed_nodes(&self, load: &ClusterLoad, pool: &NodePool) -> usize {
        let demand = load.demand_units() as f64;
        if demand <= 0.0 {
            return pool.min_nodes;
        }
        if load.slots_per_node <= f64::EPSILON {
            // Cannot size without per-node capacity: request the maximum.
            return pool.max_nodes;
        }
        let raw = (demand / (self.target_utilization * load.slots_per_node)).ceil();
        // `raw` is finite and positive here.
        (raw as usize).clamp(pool.min_nodes, pool.max_nodes)
    }

    /// Caps `target` to within `max_step` of `current` (no-op when `max_step` is
    /// zero).
    fn apply_step(&self, current: usize, target: usize) -> usize {
        if self.max_step == 0 {
            return target;
        }
        if target > current {
            current + (target - current).min(self.max_step)
        } else {
            current - (current - target).min(self.max_step)
        }
    }

    /// Evaluates the policy for `pool` under `load`, returning a decision.
    ///
    /// The current node count is the pool's `desired_nodes`. Scale-out happens
    /// only when utilisation is at/above [`AutoscalingPolicy::scale_out_threshold`]
    /// and the needed count exceeds the current; scale-in only below
    /// [`AutoscalingPolicy::scale_in_threshold`] and when fewer nodes suffice.
    #[must_use]
    pub fn evaluate(&self, pool: &NodePool, load: &ClusterLoad) -> ScalingDecision {
        let current = pool.desired_nodes;
        let utilization = load.utilization_at(current);
        let needed = self.needed_nodes(load, pool);

        let (direction, target, reason) =
            if utilization >= self.scale_out_threshold && needed > current {
                let capped = self.apply_step(current, needed);
                if capped > current {
                    (
                        ScalingDirection::Out,
                        capped,
                        format!(
                            "utilization {:.2} >= scale-out {:.2}; need {} nodes",
                            utilization, self.scale_out_threshold, needed
                        ),
                    )
                } else {
                    (
                        ScalingDirection::Hold,
                        current,
                        "scale-out capped at current by max_step".to_string(),
                    )
                }
            } else if utilization <= self.scale_in_threshold && needed < current {
                let capped = self.apply_step(current, needed);
                if capped < current {
                    (
                        ScalingDirection::In,
                        capped,
                        format!(
                            "utilization {:.2} <= scale-in {:.2}; {} nodes suffice",
                            utilization, self.scale_in_threshold, needed
                        ),
                    )
                } else {
                    (
                        ScalingDirection::Hold,
                        current,
                        "scale-in capped at current by max_step".to_string(),
                    )
                }
            } else {
                (
                    ScalingDirection::Hold,
                    current,
                    format!("utilization {utilization:.2} within dead-band"),
                )
            };

        ScalingDecision {
            direction,
            current_nodes: current,
            target_nodes: target,
            utilization,
            reason,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud::InstanceType;

    fn pool(min: usize, max: usize, desired: usize) -> NodePool {
        let mut p = NodePool::new("p", InstanceType::medium(), min, max).unwrap();
        p.set_desired(desired);
        p
    }

    #[test]
    fn test_policy_validation_and_thresholds() {
        assert!(AutoscalingPolicy::new(0.0).is_err());
        assert!(AutoscalingPolicy::new(1.5).is_err());
        let p = AutoscalingPolicy::new(0.7).unwrap();
        assert!(p.scale_out_threshold > p.scale_in_threshold);
        assert!(p.with_thresholds(0.8, 0.5).is_err()); // in >= out
    }

    #[test]
    fn test_utilization_edge_cases() {
        let load = ClusterLoad::new(0, 0, 10.0);
        assert_eq!(load.utilization_at(3), 0.0);
        let busy = ClusterLoad::new(50, 0, 10.0);
        assert!((busy.utilization_at(5) - 1.0).abs() < 1e-9);
        // Demand with zero nodes → infinite utilisation.
        assert!(busy.utilization_at(0).is_infinite());
    }

    #[test]
    fn test_scale_out_when_saturated() {
        let policy = AutoscalingPolicy::new(0.7).unwrap();
        // 5 nodes * 10 slots = 50 capacity, demand 90 → util 1.8.
        let load = ClusterLoad::new(70, 20, 10.0);
        let decision = policy.evaluate(&pool(1, 20, 5), &load);
        assert_eq!(decision.direction, ScalingDirection::Out);
        // need ceil(90 / (0.7*10)) = ceil(12.86) = 13.
        assert_eq!(decision.target_nodes, 13);
        assert!(decision.delta() > 0);
    }

    #[test]
    fn test_scale_in_when_idle() {
        let policy = AutoscalingPolicy::new(0.7).unwrap();
        // 10 nodes * 10 = 100 capacity, demand 20 → util 0.2 <= scale-in.
        let load = ClusterLoad::new(20, 0, 10.0);
        let decision = policy.evaluate(&pool(1, 20, 10), &load);
        assert_eq!(decision.direction, ScalingDirection::In);
        // need ceil(20 / 7) = 3.
        assert_eq!(decision.target_nodes, 3);
    }

    #[test]
    fn test_hold_within_deadband_and_step_cap() {
        let policy = AutoscalingPolicy::new(0.7).unwrap();
        // util ~0.7 within dead-band → hold.
        let load = ClusterLoad::new(49, 0, 10.0);
        let hold = policy.evaluate(&pool(1, 20, 7), &load);
        assert_eq!(hold.direction, ScalingDirection::Hold);
        assert_eq!(hold.target_nodes, 7);

        // With max_step=2, a big jump is capped.
        let capped_policy = AutoscalingPolicy::new(0.7).unwrap().with_max_step(2);
        let busy = ClusterLoad::new(200, 0, 10.0);
        let decision = capped_policy.evaluate(&pool(1, 50, 5), &busy);
        assert_eq!(decision.direction, ScalingDirection::Out);
        assert_eq!(decision.target_nodes, 7); // 5 + max_step(2)
    }

    #[test]
    fn test_respects_pool_bounds() {
        let policy = AutoscalingPolicy::new(0.7).unwrap();
        // Huge demand but max is 4.
        let load = ClusterLoad::new(1000, 0, 10.0);
        let decision = policy.evaluate(&pool(2, 4, 3), &load);
        assert_eq!(decision.target_nodes, 4); // clamped to max
        // Zero demand never goes below min.
        let idle = ClusterLoad::new(0, 0, 10.0);
        let in_decision = policy.evaluate(&pool(2, 4, 3), &idle);
        assert!(in_decision.target_nodes >= 2);
    }
}
