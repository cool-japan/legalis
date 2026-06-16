//! Edge computing support: latency-budgeted diffing and deterministic placement.
//!
//! On an edge node — a gateway, micro-datacentre or constrained handset — a diff
//! must complete within a tight time and memory envelope. [`EdgeDiffer`] computes
//! a structural diff under an explicit [`EdgeBudget`]: it caps the number of
//! emitted changes, truncates oversized values to bound memory, exits early when
//! the budget is exhausted (flagging the result `truncated`), and short-circuits
//! byte-identical statutes via a content fingerprint (`used_fast_path`). For the
//! very lowest latency, [`EdgeDiffer::quick_severity`] returns only a severity
//! estimate without materialising a change list.
//!
//! [`EdgeScheduler`] places an [`EdgeJob`] on the best available [`EdgeNode`]
//! using a deterministic score derived from each node's capacity, current load
//! and network quality; unhealthy, unreachable or under-provisioned nodes are
//! skipped, and ties break by device id so placement is reproducible.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_diff::mobile::edge::{EdgeConfig, EdgeDiffer};
//!
//! let old = Statute::new("s", "Old", Effect::new(EffectType::Grant, "x"));
//! let mut new = old.clone();
//! new.title = "New".to_string();
//!
//! let differ = EdgeDiffer::new(EdgeConfig::default());
//! let result = differ.diff(&old, &new);
//! assert_eq!(result.diff.changes.len(), 1);
//! assert!(!result.truncated);
//! ```

use crate::mobile::{DeviceClass, DeviceProfile, NetworkQuality, statute_fingerprint};
use crate::{Change, ChangeTarget, ChangeType, ImpactAssessment, Severity, StatuteDiff};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Bounds on a single edge diff computation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeConfig {
    /// Maximum number of changes to emit before truncating.
    pub max_changes: usize,
    /// Maximum length (in characters) of any emitted old/new value.
    pub max_value_len: usize,
    /// Maximum number of work units to consume before stopping.
    pub work_budget: u64,
    /// Optional wall-clock deadline in milliseconds (for real-time use).
    pub wall_clock_deadline_ms: Option<u64>,
}

impl Default for EdgeConfig {
    fn default() -> Self {
        // A balanced phone-class default with no wall-clock deadline so behaviour
        // is deterministic in tests.
        Self {
            max_changes: 128,
            max_value_len: 1024,
            work_budget: 1024,
            wall_clock_deadline_ms: None,
        }
    }
}

impl EdgeConfig {
    /// Returns the default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Derives a configuration from a device profile, scaling the change cap,
    /// value-length cap and work budget to the device class and halving them in
    /// battery-saver mode.
    pub fn from_profile(profile: &DeviceProfile) -> Self {
        let (max_changes, max_value_len) = match profile.class {
            DeviceClass::Wearable => (32, 256),
            DeviceClass::Phone => (128, 1024),
            DeviceClass::Tablet => (256, 2048),
            DeviceClass::Desktop | DeviceClass::EdgeNode => (1024, 8192),
            DeviceClass::Server => (4096, 16384),
        };
        let (max_changes, work_budget) = if profile.battery_saver {
            (max_changes / 2, (max_changes as u64 / 2).saturating_mul(8))
        } else {
            (max_changes, (max_changes as u64).saturating_mul(8))
        };
        Self {
            max_changes: max_changes.max(1),
            max_value_len,
            work_budget: work_budget.max(8),
            wall_clock_deadline_ms: None,
        }
    }

    /// Sets a wall-clock deadline in milliseconds.
    #[must_use]
    pub fn with_deadline_ms(mut self, ms: u64) -> Self {
        self.wall_clock_deadline_ms = Some(ms);
        self
    }

    /// Sets the maximum number of emitted changes.
    #[must_use]
    pub fn with_max_changes(mut self, max_changes: usize) -> Self {
        self.max_changes = max_changes.max(1);
        self
    }

    /// Sets the work-unit budget.
    #[must_use]
    pub fn with_work_budget(mut self, work_budget: u64) -> Self {
        self.work_budget = work_budget.max(1);
        self
    }
}

/// A consumable budget tracking work units and an optional wall-clock deadline.
///
/// Without a deadline the budget is purely work-unit based and therefore
/// deterministic; with a deadline it additionally expires once the elapsed time
/// passes the deadline. Not serializable (it captures an [`Instant`]).
#[derive(Debug)]
pub struct EdgeBudget {
    work_budget: u64,
    consumed: u64,
    start: Instant,
    deadline: Option<Duration>,
}

impl EdgeBudget {
    /// Creates a budget with `work_budget` units and an optional `deadline_ms`.
    pub fn new(work_budget: u64, deadline_ms: Option<u64>) -> Self {
        Self {
            work_budget,
            consumed: 0,
            start: Instant::now(),
            deadline: deadline_ms.map(Duration::from_millis),
        }
    }

    fn from_config(config: &EdgeConfig) -> Self {
        Self::new(config.work_budget, config.wall_clock_deadline_ms)
    }

    /// Records `units` of work as consumed (saturating).
    pub fn consume(&mut self, units: u64) {
        self.consumed = self.consumed.saturating_add(units);
    }

    /// Total work units consumed so far.
    pub fn consumed(&self) -> u64 {
        self.consumed
    }

    /// The work-unit budget this instance was created with.
    pub fn work_budget(&self) -> u64 {
        self.work_budget
    }

    /// Work units remaining (saturating at zero).
    pub fn remaining(&self) -> u64 {
        self.work_budget.saturating_sub(self.consumed)
    }

    /// Returns `true` once the work budget or the wall-clock deadline is reached.
    pub fn is_exhausted(&self) -> bool {
        if self.consumed >= self.work_budget {
            return true;
        }
        match self.deadline {
            Some(d) => self.start.elapsed() >= d,
            None => false,
        }
    }
}

/// The outcome of an [`EdgeDiffer::diff`] call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDiffResult {
    /// The (possibly partial) diff.
    pub diff: StatuteDiff,
    /// Whether the diff was cut short by the change cap or budget.
    pub truncated: bool,
    /// Work units consumed producing the result.
    pub work_units: u64,
    /// Whether the byte-identical fast path was taken.
    pub used_fast_path: bool,
    /// Wall-clock time spent, in microseconds.
    pub elapsed_micros: u64,
}

/// A budgeted differ for edge execution.
#[derive(Debug, Clone)]
pub struct EdgeDiffer {
    config: EdgeConfig,
}

impl EdgeDiffer {
    /// Creates a differ with the given configuration.
    pub fn new(config: EdgeConfig) -> Self {
        Self { config }
    }

    /// Returns the configuration in effect.
    pub fn config(&self) -> &EdgeConfig {
        &self.config
    }

    /// Computes a budgeted diff between `old` and `new`.
    ///
    /// Byte-identical statutes return immediately via the fast path. Otherwise
    /// changes are emitted until the change cap or work budget is reached, at
    /// which point the result is flagged `truncated`. Emitted values are clipped
    /// to [`EdgeConfig::max_value_len`] to bound memory.
    pub fn diff(&self, old: &Statute, new: &Statute) -> EdgeDiffResult {
        let mut budget = EdgeBudget::from_config(&self.config);

        // Fast path: byte-identical content (content-addressed equality).
        if let (Some(fa), Some(fb)) = (statute_fingerprint(old), statute_fingerprint(new)) {
            budget.consume(2);
            if fa == fb {
                return EdgeDiffResult {
                    diff: StatuteDiff {
                        statute_id: old.id.clone(),
                        version_info: None,
                        changes: Vec::new(),
                        impact: ImpactAssessment::default(),
                    },
                    truncated: false,
                    work_units: budget.consumed(),
                    used_fast_path: true,
                    elapsed_micros: budget.start.elapsed().as_micros() as u64,
                };
            }
        }

        let mut changes = Vec::new();
        let mut impact = ImpactAssessment::default();
        let mut truncated = false;
        let max_changes = self.config.max_changes;
        let max_len = self.config.max_value_len;

        // Title.
        budget.consume(1);
        if old.title != new.title {
            if Self::has_room(changes.len(), max_changes, &budget) {
                changes.push(Change {
                    change_type: ChangeType::Modified,
                    target: ChangeTarget::Title,
                    description: "Title was modified".to_string(),
                    old_value: Some(clip_value(old.title.clone(), max_len)),
                    new_value: Some(clip_value(new.title.clone(), max_len)),
                });
                impact.severity = impact.severity.max(Severity::Minor);
            } else {
                truncated = true;
            }
        }

        // Preconditions.
        if !truncated {
            truncated = self.diff_preconditions(old, new, &mut changes, &mut impact, &mut budget);
        }

        // Effect.
        if !truncated {
            budget.consume(1);
            if old.effect != new.effect {
                if Self::has_room(changes.len(), max_changes, &budget) {
                    changes.push(Change {
                        change_type: ChangeType::Modified,
                        target: ChangeTarget::Effect,
                        description: "Effect was modified".to_string(),
                        old_value: Some(clip_value(format!("{:?}", old.effect), max_len)),
                        new_value: Some(clip_value(format!("{:?}", new.effect), max_len)),
                    });
                    impact.affects_outcome = true;
                    impact.severity = impact.severity.max(Severity::Major);
                } else {
                    truncated = true;
                }
            }
        }

        // Discretion logic.
        if !truncated {
            budget.consume(1);
            if old.discretion_logic != new.discretion_logic {
                if Self::has_room(changes.len(), max_changes, &budget) {
                    let (ct, ov, nv) = match (&old.discretion_logic, &new.discretion_logic) {
                        (None, Some(l)) => (ChangeType::Added, None, Some(l.clone())),
                        (Some(l), None) => (ChangeType::Removed, Some(l.clone()), None),
                        (Some(a), Some(b)) => {
                            (ChangeType::Modified, Some(a.clone()), Some(b.clone()))
                        }
                        (None, None) => (ChangeType::Modified, None, None),
                    };
                    changes.push(Change {
                        change_type: ct,
                        target: ChangeTarget::DiscretionLogic,
                        description: "Discretion logic changed".to_string(),
                        old_value: ov.map(|v| clip_value(v, max_len)),
                        new_value: nv.map(|v| clip_value(v, max_len)),
                    });
                    impact.discretion_changed = true;
                    impact.severity = impact.severity.max(Severity::Moderate);
                } else {
                    truncated = true;
                }
            }
        }

        if truncated {
            impact
                .notes
                .push("Edge diff truncated: change cap or budget reached".to_string());
        }

        EdgeDiffResult {
            diff: StatuteDiff {
                statute_id: old.id.clone(),
                version_info: None,
                changes,
                impact,
            },
            truncated,
            work_units: budget.consumed(),
            used_fast_path: false,
            elapsed_micros: budget.start.elapsed().as_micros() as u64,
        }
    }

    /// Returns a cheap severity estimate without building a change list — the
    /// lowest-latency mode, useful for triage on very constrained devices.
    pub fn quick_severity(&self, old: &Statute, new: &Statute) -> Severity {
        let mut severity = Severity::None;
        if old.title != new.title {
            severity = severity.max(Severity::Minor);
        }
        if old.preconditions.len() != new.preconditions.len() {
            severity = severity.max(Severity::Major);
        } else {
            for (a, b) in old.preconditions.iter().zip(new.preconditions.iter()) {
                if a != b {
                    severity = severity.max(Severity::Moderate);
                }
            }
        }
        if old.effect != new.effect {
            severity = severity.max(Severity::Major);
        }
        if old.discretion_logic != new.discretion_logic {
            severity = severity.max(Severity::Moderate);
        }
        severity
    }

    /// Diffs preconditions under budget; returns `true` if truncated.
    fn diff_preconditions(
        &self,
        old: &Statute,
        new: &Statute,
        changes: &mut Vec<Change>,
        impact: &mut ImpactAssessment,
        budget: &mut EdgeBudget,
    ) -> bool {
        let max_changes = self.config.max_changes;
        let max_len = self.config.max_value_len;
        let (old_len, new_len) = (old.preconditions.len(), new.preconditions.len());

        if new_len > old_len {
            for (i, cond) in new.preconditions.iter().enumerate().skip(old_len) {
                budget.consume(1);
                if !Self::has_room(changes.len(), max_changes, budget) {
                    return true;
                }
                changes.push(Change {
                    change_type: ChangeType::Added,
                    target: ChangeTarget::Precondition { index: i },
                    description: format!("New precondition added at position {}", i + 1),
                    old_value: None,
                    new_value: Some(clip_value(format!("{:?}", cond), max_len)),
                });
            }
            impact.affects_eligibility = true;
            impact.severity = impact.severity.max(Severity::Major);
        } else if old_len > new_len {
            for (i, cond) in old.preconditions.iter().enumerate().skip(new_len) {
                budget.consume(1);
                if !Self::has_room(changes.len(), max_changes, budget) {
                    return true;
                }
                changes.push(Change {
                    change_type: ChangeType::Removed,
                    target: ChangeTarget::Precondition { index: i },
                    description: format!("Precondition removed from position {}", i + 1),
                    old_value: Some(clip_value(format!("{:?}", cond), max_len)),
                    new_value: None,
                });
            }
            impact.affects_eligibility = true;
            impact.severity = impact.severity.max(Severity::Major);
        }

        for i in 0..old_len.min(new_len) {
            budget.consume(1);
            if old.preconditions[i] != new.preconditions[i] {
                if !Self::has_room(changes.len(), max_changes, budget) {
                    return true;
                }
                changes.push(Change {
                    change_type: ChangeType::Modified,
                    target: ChangeTarget::Precondition { index: i },
                    description: format!("Precondition {} was modified", i + 1),
                    old_value: Some(clip_value(format!("{:?}", old.preconditions[i]), max_len)),
                    new_value: Some(clip_value(format!("{:?}", new.preconditions[i]), max_len)),
                });
                impact.affects_eligibility = true;
                impact.severity = impact.severity.max(Severity::Moderate);
            }
        }

        false
    }

    fn has_room(current: usize, max_changes: usize, budget: &EdgeBudget) -> bool {
        current < max_changes && !budget.is_exhausted()
    }
}

/// Clips a string to at most `max` characters, appending a truncation marker.
fn clip_value(value: String, max: usize) -> String {
    if value.chars().count() <= max {
        return value;
    }
    // Byte offset of the `max`-th character; `Some` because count > max above.
    let boundary = value
        .char_indices()
        .nth(max)
        .map_or(value.len(), |(idx, _)| idx);
    let mut clipped = value;
    clipped.truncate(boundary);
    clipped.push_str("…[truncated]");
    clipped
}

/// A diff job to be placed on an edge node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeJob {
    /// Stable job identifier.
    pub job_id: String,
    /// Estimated peak memory the job requires, in bytes.
    pub estimated_memory_bytes: u64,
    /// Scheduling priority (higher runs first when a caller sorts jobs).
    pub priority: u8,
}

impl EdgeJob {
    /// Creates a job with the given identifier and memory estimate.
    pub fn new(job_id: impl Into<String>, estimated_memory_bytes: u64) -> Self {
        Self {
            job_id: job_id.into(),
            estimated_memory_bytes,
            priority: 0,
        }
    }

    /// Sets the scheduling priority.
    #[must_use]
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

/// A schedulable edge node: a device profile plus live load and health.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeNode {
    /// Static device characteristics.
    pub profile: DeviceProfile,
    /// Number of jobs currently assigned to the node.
    pub active_jobs: u32,
    /// Whether the node is currently healthy and accepting work.
    pub healthy: bool,
}

impl EdgeNode {
    /// Creates a healthy, idle node from a profile.
    pub fn new(profile: DeviceProfile) -> Self {
        Self {
            profile,
            active_jobs: 0,
            healthy: true,
        }
    }

    /// Sets the active job count.
    #[must_use]
    pub fn with_active_jobs(mut self, active_jobs: u32) -> Self {
        self.active_jobs = active_jobs;
        self
    }

    /// Sets the health flag.
    #[must_use]
    pub fn with_healthy(mut self, healthy: bool) -> Self {
        self.healthy = healthy;
        self
    }

    /// A deterministic placement score for `job`, or `None` if the node is
    /// unhealthy, unreachable (offline) or lacks the required memory.
    ///
    /// Higher is better. The score is the node's capacity divided by its load,
    /// scaled by a network-quality factor.
    pub fn placement_score(&self, job: &EdgeJob) -> Option<f64> {
        if !self.healthy {
            return None;
        }
        if job.estimated_memory_bytes > self.profile.memory_budget_bytes {
            return None;
        }
        let network_factor = match self.profile.network_quality {
            NetworkQuality::Excellent => 1.0,
            NetworkQuality::Good => 0.9,
            NetworkQuality::Moderate => 0.7,
            NetworkQuality::Poor => 0.4,
            NetworkQuality::Offline => return None,
        };
        Some(self.profile.capacity_score() / (self.active_jobs as f64 + 1.0) * network_factor)
    }
}

/// A deterministic scheduler placing [`EdgeJob`]s onto [`EdgeNode`]s.
#[derive(Debug, Clone, Default)]
pub struct EdgeScheduler {
    nodes: Vec<EdgeNode>,
}

impl EdgeScheduler {
    /// Creates an empty scheduler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node to the pool.
    pub fn add_node(&mut self, node: EdgeNode) {
        self.nodes.push(node);
    }

    /// The total number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if there are no nodes.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// The number of healthy nodes.
    pub fn healthy_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.healthy).count()
    }

    /// Read-only view of the node pool.
    pub fn nodes(&self) -> &[EdgeNode] {
        &self.nodes
    }

    /// Returns the index of the best node for `job`, if any can host it. Ties
    /// break deterministically by ascending device id.
    pub fn place_index(&self, job: &EdgeJob) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for (i, node) in self.nodes.iter().enumerate() {
            let Some(score) = node.placement_score(job) else {
                continue;
            };
            match best {
                None => best = Some((i, score)),
                Some((bi, bs)) => {
                    let better_score = score > bs + f64::EPSILON;
                    let tie_break = (score - bs).abs() <= f64::EPSILON
                        && node.profile.device_id < self.nodes[bi].profile.device_id;
                    if better_score || tie_break {
                        best = Some((i, score));
                    }
                }
            }
        }
        best.map(|(i, _)| i)
    }

    /// Returns the best node for `job`, if any.
    pub fn place(&self, job: &EdgeJob) -> Option<&EdgeNode> {
        self.place_index(job).and_then(|i| self.nodes.get(i))
    }

    /// Places `job` on the best node, increments that node's load and returns its
    /// device id, or `None` if no node can host the job.
    pub fn assign(&mut self, job: &EdgeJob) -> Option<String> {
        let index = self.place_index(job)?;
        let node = self.nodes.get_mut(index)?;
        node.active_jobs = node.active_jobs.saturating_add(1);
        Some(node.profile.device_id.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mobile::DeviceClass;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn base() -> Statute {
        Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
    }

    #[test]
    fn test_fast_path_identical() {
        let differ = EdgeDiffer::new(EdgeConfig::default());
        let result = differ.diff(&base(), &base());
        assert!(result.used_fast_path);
        assert!(result.diff.changes.is_empty());
        assert!(!result.truncated);
    }

    #[test]
    fn test_basic_diff_not_truncated() {
        let old = base();
        let mut new = old.clone();
        new.title = "Changed".to_string();
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        let differ = EdgeDiffer::new(EdgeConfig::default());
        let result = differ.diff(&old, &new);
        assert!(!result.used_fast_path);
        assert!(!result.truncated);
        assert_eq!(result.diff.changes.len(), 2);
        assert!(result.work_units > 0);
    }

    #[test]
    fn test_truncation_by_max_changes() {
        let old = base();
        let mut new = old.clone();
        new.title = "Changed".to_string();
        for i in 0..10 {
            new.preconditions.push(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: i,
            });
        }
        let config = EdgeConfig::default().with_max_changes(3);
        let result = EdgeDiffer::new(config).diff(&old, &new);
        assert!(result.truncated);
        assert_eq!(result.diff.changes.len(), 3);
        assert!(
            result
                .diff
                .impact
                .notes
                .iter()
                .any(|n| n.contains("truncated"))
        );
    }

    #[test]
    fn test_truncation_by_work_budget() {
        let old = base();
        let mut new = old.clone();
        new.title = "Changed".to_string();
        for i in 0..20 {
            new.preconditions.push(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: i,
            });
        }
        // Budget allows only a couple of work units beyond the fast-path probe.
        let config = EdgeConfig::default()
            .with_work_budget(4)
            .with_max_changes(1000);
        let result = EdgeDiffer::new(config).diff(&old, &new);
        assert!(result.truncated);
        assert!(result.diff.changes.len() < 21);
    }

    #[test]
    fn test_value_clipping_bounds_memory() {
        let old = base();
        let mut new = old.clone();
        new.title = "x".repeat(10_000);
        let config = EdgeConfig::default().with_max_changes(10);
        let result = EdgeDiffer::new(config).diff(&old, &new);
        let change = &result.diff.changes[0];
        let new_value = change.new_value.as_ref().expect("new value present");
        assert!(new_value.chars().count() <= 1024 + "…[truncated]".chars().count());
        assert!(new_value.contains("truncated"));
    }

    #[test]
    fn test_quick_severity() {
        let differ = EdgeDiffer::new(EdgeConfig::default());
        let old = base();
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        assert_eq!(differ.quick_severity(&old, &new), Severity::Major);
        assert_eq!(differ.quick_severity(&old, &old), Severity::None);
    }

    #[test]
    fn test_config_from_profile() {
        let wearable = EdgeConfig::from_profile(&DeviceProfile::new("w", DeviceClass::Wearable));
        let server = EdgeConfig::from_profile(&DeviceProfile::new("s", DeviceClass::Server));
        assert!(wearable.max_changes < server.max_changes);
        let saver = EdgeConfig::from_profile(
            &DeviceProfile::new("p", DeviceClass::Phone).with_battery_saver(true),
        );
        let normal = EdgeConfig::from_profile(&DeviceProfile::new("p", DeviceClass::Phone));
        assert!(saver.max_changes < normal.max_changes);
    }

    #[test]
    fn test_scheduler_picks_highest_capacity() {
        let mut scheduler = EdgeScheduler::new();
        scheduler.add_node(EdgeNode::new(DeviceProfile::new(
            "phone",
            DeviceClass::Phone,
        )));
        scheduler.add_node(EdgeNode::new(DeviceProfile::new(
            "server",
            DeviceClass::Server,
        )));
        let job = EdgeJob::new("j1", 1024);
        let chosen = scheduler.place(&job).expect("a node");
        assert_eq!(chosen.profile.device_id, "server");
    }

    #[test]
    fn test_scheduler_skips_unhealthy_offline_and_oversized() {
        let mut scheduler = EdgeScheduler::new();
        scheduler.add_node(
            EdgeNode::new(DeviceProfile::new("down", DeviceClass::Server)).with_healthy(false),
        );
        scheduler.add_node(EdgeNode::new(
            DeviceProfile::new("nonet", DeviceClass::Server).with_network(NetworkQuality::Offline),
        ));
        scheduler.add_node(EdgeNode::new(
            DeviceProfile::new("tiny", DeviceClass::Wearable).with_memory_budget_bytes(1000),
        ));
        // Only "tiny" is healthy+online but the job needs more memory than it has.
        let big_job = EdgeJob::new("big", 1_000_000);
        assert!(scheduler.place(&big_job).is_none());
        // A small job fits on "tiny".
        let small_job = EdgeJob::new("small", 500);
        assert_eq!(
            scheduler
                .place(&small_job)
                .map(|n| n.profile.device_id.clone()),
            Some("tiny".to_string())
        );
    }

    #[test]
    fn test_scheduler_assign_increments_load() {
        let mut scheduler = EdgeScheduler::new();
        scheduler.add_node(EdgeNode::new(DeviceProfile::new("a", DeviceClass::Server)));
        scheduler.add_node(EdgeNode::new(DeviceProfile::new("b", DeviceClass::Server)));
        // First two assignments should spread across the two equal nodes because
        // load reduces the score of an already-loaded node.
        let first = scheduler.assign(&EdgeJob::new("j1", 10)).expect("assigned");
        let second = scheduler.assign(&EdgeJob::new("j2", 10)).expect("assigned");
        assert_ne!(first, second);
        assert!(scheduler.nodes().iter().all(|n| n.active_jobs == 1));
    }
}
