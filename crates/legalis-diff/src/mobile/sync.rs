//! Cross-platform synchronization for diff state across devices.
//!
//! [`SyncEngine`] reconciles statute/diff state between replicas running on
//! different platforms (phone, tablet, desktop, server) using **vector clocks**
//! for causality, **delta sync** to exchange only what a peer is missing, and a
//! **convergent** conflict-resolution rule so that, after exchanging deltas, two
//! replicas reach byte-for-byte identical state (an equal [`state_digest`]).
//!
//! State resolution uses a single total order whose primary key is the Lamport
//! projection of the vector clock (the sum of its components). Because a causally
//! later update always has a strictly larger projection, this order respects
//! causality *and* is a genuine total order, so the per-resource winner is the
//! maximum of a set and therefore independent of receive order — the property
//! that guarantees convergence. The [`ConflictResolution`] policy only chooses
//! the tiebreak among causally-*concurrent* updates (which are additionally
//! recorded as [`SyncConflict`]s for observability).
//!
//! This is distinct from [`crate::realtime`] (live WebSocket streaming /
//! collaborative editing) and [`crate::mobile::offline`] (the local-first
//! operation queue): here the concern is device-to-device causal reconciliation,
//! the natural sync layer beneath an offline-first client.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_diff::mobile::sync::{SyncEngine, SyncPayload, sync_pair};
//!
//! let mut phone = SyncEngine::new("phone");
//! let mut laptop = SyncEngine::new("laptop");
//!
//! let s = Statute::new("law", "V1", Effect::new(EffectType::Grant, "x"));
//! phone.local_change(SyncPayload::Statute { statute: Box::new(s) });
//!
//! // Exchange deltas until both converge.
//! sync_pair(&mut phone, &mut laptop);
//! assert_eq!(phone.state_digest(), laptop.state_digest());
//! ```

use crate::StatuteDiff;
use crate::mobile::{sha256_hex, sha256_parts};
use chrono::{DateTime, Utc};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A vector clock: a per-device monotonically increasing counter map.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorClock {
    entries: BTreeMap<String, u64>,
}

impl VectorClock {
    /// Creates an empty clock.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the counter for `device` (zero if absent).
    pub fn get(&self, device: &str) -> u64 {
        self.entries.get(device).copied().unwrap_or(0)
    }

    /// Increments and returns the counter for `device`.
    pub fn increment(&mut self, device: &str) -> u64 {
        let counter = self.entries.entry(device.to_string()).or_insert(0);
        *counter = counter.saturating_add(1);
        *counter
    }

    /// Componentwise maximum merge of `other` into `self`.
    pub fn merge(&mut self, other: &VectorClock) {
        for (device, &counter) in &other.entries {
            let slot = self.entries.entry(device.clone()).or_insert(0);
            if counter > *slot {
                *slot = counter;
            }
        }
    }

    /// Returns the componentwise-maximum merge of `self` and `other`.
    pub fn merged(&self, other: &VectorClock) -> VectorClock {
        let mut clone = self.clone();
        clone.merge(other);
        clone
    }

    /// Returns `true` if `self` dominates `other` (covers every component:
    /// `self[k] >= other[k]` for all `k`). Equivalent to "`other` happened
    /// before or at `self`".
    pub fn dominates(&self, other: &VectorClock) -> bool {
        other
            .entries
            .iter()
            .all(|(device, &counter)| self.get(device) >= counter)
    }

    /// The Lamport projection: the sum of all components, as a `u128` to avoid
    /// overflow. Monotonic with causal order.
    pub fn total(&self) -> u128 {
        self.entries.values().map(|&v| v as u128).sum()
    }

    /// Returns the causal relationship between `self` and `other`.
    pub fn compare(&self, other: &VectorClock) -> ClockOrder {
        let mut less = false;
        let mut greater = false;
        for device in self.entries.keys().chain(other.entries.keys()) {
            let a = self.get(device);
            let b = other.get(device);
            if a < b {
                less = true;
            }
            if a > b {
                greater = true;
            }
        }
        match (less, greater) {
            (false, false) => ClockOrder::Equal,
            (true, false) => ClockOrder::Before,
            (false, true) => ClockOrder::After,
            (true, true) => ClockOrder::Concurrent,
        }
    }

    /// Returns `true` if `self` strictly happened before `other`.
    pub fn happens_before(&self, other: &VectorClock) -> bool {
        matches!(self.compare(other), ClockOrder::Before)
    }

    /// Returns `true` if `self` and `other` are concurrent (neither precedes).
    pub fn concurrent_with(&self, other: &VectorClock) -> bool {
        matches!(self.compare(other), ClockOrder::Concurrent)
    }

    /// Read-only view of the underlying counters.
    pub fn entries(&self) -> &BTreeMap<String, u64> {
        &self.entries
    }

    /// Returns `true` if the clock has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// The causal relationship between two [`VectorClock`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClockOrder {
    /// The clocks are identical.
    Equal,
    /// The left clock happened strictly before the right.
    Before,
    /// The left clock happened strictly after the right.
    After,
    /// The clocks are concurrent (a conflict).
    Concurrent,
}

/// The payload carried by a [`SyncOp`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "payload")]
pub enum SyncPayload {
    /// A statute snapshot became current.
    Statute {
        /// The statute (boxed to keep the enum small).
        statute: Box<Statute>,
    },
    /// A computed diff to be replicated.
    Diff {
        /// The diff (boxed to keep the enum small).
        diff: Box<StatuteDiff>,
    },
    /// A deletion of `resource_id`.
    Tombstone {
        /// The resource being deleted.
        resource_id: String,
    },
}

impl SyncPayload {
    /// The resource id this payload concerns.
    pub fn resource_id(&self) -> String {
        match self {
            Self::Statute { statute } => statute.id.clone(),
            Self::Diff { diff } => diff.statute_id.clone(),
            Self::Tombstone { resource_id } => resource_id.clone(),
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Statute { .. } => "statute",
            Self::Diff { .. } => "diff",
            Self::Tombstone { .. } => "tombstone",
        }
    }
}

/// A replicated operation: a payload stamped with origin, vector clock and time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOp {
    /// Content-addressed identifier (unique per distinct content).
    pub op_id: String,
    /// The device that originated the operation.
    pub origin: String,
    /// The resource the operation concerns.
    pub resource_id: String,
    /// The vector clock at creation.
    pub clock: VectorClock,
    /// Wall-clock timestamp at creation.
    pub timestamp: DateTime<Utc>,
    /// The payload.
    pub payload: SyncPayload,
}

/// How concurrent (conflicting) updates choose a winner.
///
/// Every variant defines a total order over operations (with the unique content
/// id as the final tiebreak), so every variant is convergent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Later wall-clock timestamp wins.
    LastWriterWins,
    /// Earlier wall-clock timestamp wins.
    EarliestWriterWins,
    /// Purely by content id (clock- and time-agnostic, fully deterministic).
    DeterministicById,
}

/// A recorded concurrent-update conflict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflict {
    /// The contested resource.
    pub resource_id: String,
    /// The op already held in state.
    pub existing_op_id: String,
    /// The incoming op.
    pub incoming_op_id: String,
    /// The op id that won (now current state).
    pub winner_op_id: String,
    /// The resolution policy applied.
    pub resolution: ConflictResolution,
}

/// The classification of a [`SyncEngine::receive`] call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncOutcome {
    /// First operation for the resource; now current.
    NewResource,
    /// Causally supersedes the prior state; now current.
    Applied,
    /// Causally older than current state; ignored for state.
    Stale,
    /// Already known (same content) or an identical clock; no change.
    Duplicate,
    /// Concurrent with current state; resolved per policy (see conflicts).
    Conflict,
}

/// A batch of operations a peer is missing, plus the clock it was computed for.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDelta {
    /// The peer clock the delta was computed against.
    pub from_clock: VectorClock,
    /// The operations the peer has not yet seen, in causal-stable order.
    pub ops: Vec<SyncOp>,
}

impl SyncDelta {
    /// The number of operations in the delta.
    pub fn len(&self) -> usize {
        self.ops.len()
    }

    /// Returns `true` if there is nothing to send.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}

/// A per-device synchronization engine.
#[derive(Debug, Clone)]
pub struct SyncEngine {
    device_id: String,
    clock: VectorClock,
    resolution: ConflictResolution,
    applied: BTreeMap<String, SyncOp>,
    state: BTreeMap<String, SyncOp>,
    conflicts: Vec<SyncConflict>,
}

impl SyncEngine {
    /// Creates an engine for `device_id` using [`ConflictResolution::LastWriterWins`].
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
            clock: VectorClock::new(),
            resolution: ConflictResolution::LastWriterWins,
            applied: BTreeMap::new(),
            state: BTreeMap::new(),
            conflicts: Vec::new(),
        }
    }

    /// Sets the conflict-resolution policy.
    #[must_use]
    pub fn with_resolution(mut self, resolution: ConflictResolution) -> Self {
        self.resolution = resolution;
        self
    }

    /// The owning device id.
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// The current vector clock.
    pub fn clock(&self) -> &VectorClock {
        &self.clock
    }

    /// The current winning op for `resource_id`, if any.
    pub fn current(&self, resource_id: &str) -> Option<&SyncOp> {
        self.state.get(resource_id)
    }

    /// The number of distinct resources in state.
    pub fn resource_count(&self) -> usize {
        self.state.len()
    }

    /// The number of operations known (the dedup/delta set).
    pub fn applied_count(&self) -> usize {
        self.applied.len()
    }

    /// The recorded conflicts.
    pub fn conflicts(&self) -> &[SyncConflict] {
        &self.conflicts
    }

    /// Creates, applies and returns a local operation for `payload`.
    pub fn local_change(&mut self, payload: SyncPayload) -> SyncOp {
        self.clock.increment(&self.device_id);
        let resource_id = payload.resource_id();
        let timestamp = Utc::now();
        let op_id = Self::compute_op_id(
            &self.device_id,
            &resource_id,
            &self.clock,
            timestamp,
            &payload,
        );
        let op = SyncOp {
            op_id,
            origin: self.device_id.clone(),
            resource_id,
            clock: self.clock.clone(),
            timestamp,
            payload,
        };
        self.integrate(op.clone());
        op
    }

    /// Integrates a remote operation, returning how it was classified.
    pub fn receive(&mut self, op: SyncOp) -> SyncOutcome {
        if self.applied.contains_key(&op.op_id) {
            return SyncOutcome::Duplicate;
        }
        self.clock.merge(&op.clock);
        self.integrate(op)
    }

    /// Returns the operations `their_clock` has not yet seen.
    pub fn delta_since(&self, their_clock: &VectorClock) -> SyncDelta {
        let mut ops: Vec<SyncOp> = self
            .applied
            .values()
            .filter(|op| !their_clock.dominates(&op.clock))
            .cloned()
            .collect();
        ops.sort_by(|a, b| {
            (a.clock.total(), a.op_id.as_str()).cmp(&(b.clock.total(), b.op_id.as_str()))
        });
        SyncDelta {
            from_clock: their_clock.clone(),
            ops,
        }
    }

    /// A digest of the current state. Two converged replicas produce equal
    /// digests.
    pub fn state_digest(&self) -> String {
        let mut parts: Vec<String> = self
            .state
            .iter()
            .map(|(resource, op)| format!("{}={}", resource, op.op_id))
            .collect();
        parts.sort();
        sha256_hex(parts.join(";").as_bytes())
    }

    fn integrate(&mut self, op: SyncOp) -> SyncOutcome {
        let resource = op.resource_id.clone();
        let outcome = match self.state.get(&resource) {
            None => {
                self.state.insert(resource, op.clone());
                SyncOutcome::NewResource
            }
            Some(existing) => match existing.clock.compare(&op.clock) {
                ClockOrder::Before => {
                    self.state.insert(resource, op.clone());
                    SyncOutcome::Applied
                }
                ClockOrder::After => SyncOutcome::Stale,
                ClockOrder::Equal => SyncOutcome::Duplicate,
                ClockOrder::Concurrent => {
                    let incoming_wins = self.beats(&op, existing);
                    let existing_id = existing.op_id.clone();
                    let winner_op_id = if incoming_wins {
                        op.op_id.clone()
                    } else {
                        existing_id.clone()
                    };
                    self.conflicts.push(SyncConflict {
                        resource_id: resource.clone(),
                        existing_op_id: existing_id,
                        incoming_op_id: op.op_id.clone(),
                        winner_op_id,
                        resolution: self.resolution,
                    });
                    if incoming_wins {
                        self.state.insert(resource, op.clone());
                    }
                    SyncOutcome::Conflict
                }
            },
        };
        self.applied.insert(op.op_id.clone(), op);
        outcome
    }

    /// Returns `true` if `incoming` should beat `existing` under the policy.
    ///
    /// The Lamport projection is the primary key (so causally later always
    /// wins); the policy provides the tiebreak among concurrent operations, with
    /// the unique op id as a final, deterministic tiebreak.
    fn beats(&self, incoming: &SyncOp, existing: &SyncOp) -> bool {
        let (li, le) = (incoming.clock.total(), existing.clock.total());
        if li != le {
            return li > le;
        }
        match self.resolution {
            ConflictResolution::LastWriterWins => {
                (incoming.timestamp, incoming.op_id.as_str())
                    > (existing.timestamp, existing.op_id.as_str())
            }
            ConflictResolution::EarliestWriterWins => {
                if incoming.timestamp != existing.timestamp {
                    incoming.timestamp < existing.timestamp
                } else {
                    incoming.op_id.as_str() > existing.op_id.as_str()
                }
            }
            ConflictResolution::DeterministicById => {
                incoming.op_id.as_str() > existing.op_id.as_str()
            }
        }
    }

    fn compute_op_id(
        origin: &str,
        resource_id: &str,
        clock: &VectorClock,
        timestamp: DateTime<Utc>,
        payload: &SyncPayload,
    ) -> String {
        let payload_bytes = serde_json::to_vec(payload).unwrap_or_default();
        let clock_bytes = serde_json::to_vec(clock).unwrap_or_default();
        let ts = timestamp.to_rfc3339();
        sha256_parts(&[
            origin.as_bytes(),
            resource_id.as_bytes(),
            payload.label().as_bytes(),
            &clock_bytes,
            ts.as_bytes(),
            &payload_bytes,
        ])
    }
}

/// Bidirectionally synchronizes two engines until they converge, returning the
/// number of exchange rounds performed.
///
/// Each round computes the delta each side is missing and applies it; the loop
/// stops once both deltas are empty (bounded to avoid pathological loops).
pub fn sync_pair(a: &mut SyncEngine, b: &mut SyncEngine) -> usize {
    let mut rounds = 0;
    loop {
        let to_b = a.delta_since(b.clock());
        let to_a = b.delta_since(a.clock());
        if to_b.is_empty() && to_a.is_empty() {
            break;
        }
        for op in to_b.ops {
            b.receive(op);
        }
        for op in to_a.ops {
            a.receive(op);
        }
        rounds += 1;
        if rounds >= 100 {
            break;
        }
    }
    rounds
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    fn statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Benefit"))
    }

    #[test]
    fn test_vector_clock_compare() {
        let mut a = VectorClock::new();
        a.increment("x");
        let mut b = a.clone();
        b.increment("x");
        assert_eq!(a.compare(&b), ClockOrder::Before);
        assert_eq!(b.compare(&a), ClockOrder::After);
        assert_eq!(a.compare(&a), ClockOrder::Equal);
        assert!(a.happens_before(&b));

        let mut c = VectorClock::new();
        c.increment("y");
        assert_eq!(a.compare(&c), ClockOrder::Concurrent);
        assert!(a.concurrent_with(&c));
    }

    #[test]
    fn test_vector_clock_merge_and_dominates() {
        let mut a = VectorClock::new();
        a.increment("x");
        a.increment("x");
        let mut b = VectorClock::new();
        b.increment("y");
        let merged = a.merged(&b);
        assert_eq!(merged.get("x"), 2);
        assert_eq!(merged.get("y"), 1);
        assert!(merged.dominates(&a));
        assert!(merged.dominates(&b));
        assert!(!a.dominates(&b));
        assert_eq!(merged.total(), 3);
    }

    #[test]
    fn test_local_change_advances_clock_and_state() {
        let mut engine = SyncEngine::new("dev");
        let op = engine.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law", "V1")),
        });
        assert_eq!(engine.clock().get("dev"), 1);
        assert_eq!(engine.resource_count(), 1);
        assert_eq!(
            engine.current("law").map(|o| o.op_id.clone()),
            Some(op.op_id)
        );
    }

    #[test]
    fn test_receive_duplicate_is_idempotent() {
        let mut a = SyncEngine::new("a");
        let op = a.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law", "V1")),
        });
        let mut b = SyncEngine::new("b");
        assert_eq!(b.receive(op.clone()), SyncOutcome::NewResource);
        assert_eq!(b.receive(op.clone()), SyncOutcome::Duplicate);
        assert_eq!(b.applied_count(), 1);
    }

    #[test]
    fn test_causal_supersede_and_stale() {
        // a makes V1 then V2 (causally ordered). b receiving them in either order
        // must end at V2.
        let mut a = SyncEngine::new("a");
        let op1 = a.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law", "V1")),
        });
        let op2 = a.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law", "V2")),
        });

        let mut forward = SyncEngine::new("b");
        assert_eq!(forward.receive(op1.clone()), SyncOutcome::NewResource);
        assert_eq!(forward.receive(op2.clone()), SyncOutcome::Applied);
        assert_eq!(
            forward.current("law").map(|o| o.op_id.clone()),
            Some(op2.op_id.clone())
        );

        let mut backward = SyncEngine::new("c");
        assert_eq!(backward.receive(op2.clone()), SyncOutcome::NewResource);
        assert_eq!(backward.receive(op1.clone()), SyncOutcome::Stale);
        assert_eq!(
            backward.current("law").map(|o| o.op_id.clone()),
            Some(op2.op_id)
        );
    }

    #[test]
    fn test_concurrent_conflict_recorded_and_deterministic() {
        // Two independent engines edit the same resource concurrently.
        let mut a = SyncEngine::new("a");
        let mut b = SyncEngine::new("b");
        let op_a = a.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law", "from-a")),
        });
        let op_b = b.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law", "from-b")),
        });

        // Apply the other's op to each: both should detect a conflict and pick the
        // same winner deterministically.
        let mut left = a.clone();
        let mut right = b.clone();
        assert_eq!(left.receive(op_b.clone()), SyncOutcome::Conflict);
        assert_eq!(right.receive(op_a.clone()), SyncOutcome::Conflict);
        assert_eq!(left.conflicts().len(), 1);
        assert_eq!(
            left.current("law").map(|o| o.op_id.clone()),
            right.current("law").map(|o| o.op_id.clone())
        );
    }

    #[test]
    fn test_delta_since() {
        let mut a = SyncEngine::new("a");
        a.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law1", "V1")),
        });
        a.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law2", "V1")),
        });
        // A fresh peer clock has seen nothing.
        let delta = a.delta_since(&VectorClock::new());
        assert_eq!(delta.len(), 2);
        // After seeing a's clock, there is nothing to send.
        let delta_seen = a.delta_since(a.clock());
        assert!(delta_seen.is_empty());
    }

    #[test]
    fn test_sync_pair_converges() {
        let mut phone = SyncEngine::new("phone");
        let mut laptop = SyncEngine::new("laptop");

        // Independent edits to different resources plus one concurrent conflict.
        phone.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law-a", "phone-a")),
        });
        laptop.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law-b", "laptop-b")),
        });
        phone.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law-x", "phone-x")),
        });
        laptop.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law-x", "laptop-x")),
        });

        let rounds = sync_pair(&mut phone, &mut laptop);
        assert!(rounds >= 1);
        assert_eq!(phone.state_digest(), laptop.state_digest());
        assert_eq!(phone.resource_count(), 3);
        assert_eq!(phone.applied_count(), laptop.applied_count());
    }

    #[test]
    fn test_convergence_under_alternate_resolution() {
        let mut a = SyncEngine::new("a").with_resolution(ConflictResolution::DeterministicById);
        let mut b = SyncEngine::new("b").with_resolution(ConflictResolution::DeterministicById);
        a.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law", "a")),
        });
        b.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law", "b")),
        });
        sync_pair(&mut a, &mut b);
        assert_eq!(a.state_digest(), b.state_digest());
    }

    #[test]
    fn test_tombstone_replicates() {
        let mut a = SyncEngine::new("a");
        a.local_change(SyncPayload::Statute {
            statute: Box::new(statute("law", "V1")),
        });
        a.local_change(SyncPayload::Tombstone {
            resource_id: "law".to_string(),
        });
        let mut b = SyncEngine::new("b");
        sync_pair(&mut a, &mut b);
        assert!(matches!(
            b.current("law").map(|o| &o.payload),
            Some(SyncPayload::Tombstone { .. })
        ));
        assert_eq!(a.state_digest(), b.state_digest());
    }
}
