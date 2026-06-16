//! Offline-first diff computation.
//!
//! [`OfflineEngine`] lets a disconnected device keep working: it holds a local
//! **snapshot store** (the latest known version of each statute) and an
//! append-only **operation queue**. Diffs are computed locally and optimistically
//! the moment they are requested — no network round-trip — while the underlying
//! operations are queued for later synchronization. The snapshot store is a
//! materialised view of [`OperationKind::UpsertStatute`] events, so the entire
//! state can be [`replay`](OfflineEngine::replay)ed from the log for recovery,
//! and persisted to / restored from disk to survive process restarts.
//!
//! This is deliberately distinct from [`crate::advanced_cache`] (which caches
//! diff *results* in Redis/Memcached/multi-level hierarchies): here the concern
//! is a durable, replayable local-first operation log that pairs with
//! [`crate::mobile::sync`] for eventual reconciliation.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_diff::mobile::offline::OfflineEngine;
//!
//! let mut engine = OfflineEngine::new("phone-1");
//! let v1 = Statute::new("law", "V1", Effect::new(EffectType::Grant, "x"));
//! let mut v2 = v1.clone();
//! v2.title = "V2".to_string();
//!
//! engine.put_statute(v1.clone());
//! let diff = engine.record_diff(&v1, &v2).unwrap();
//! assert_eq!(diff.changes.len(), 1);
//!
//! // Everything is queued for sync while offline.
//! assert!(engine.pending_count() > 0);
//! assert_eq!(engine.latest("law").map(|s| s.title.clone()), Some("V2".to_string()));
//! ```

use crate::mobile::sha256_parts;
use crate::{DiffError, DiffResult, StatuteDiff, diff};
use chrono::{DateTime, Utc};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// The kind of a queued offline operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum OperationKind {
    /// Insert or replace the latest snapshot of a statute.
    UpsertStatute {
        /// The statute version that became current (boxed to keep the enum small).
        statute: Box<Statute>,
    },
    /// Record a computed diff (audit of an offline computation).
    RecordDiff {
        /// The computed diff (boxed to keep the enum small).
        diff: Box<StatuteDiff>,
    },
}

impl OperationKind {
    /// A short stable label for the operation kind.
    pub fn label(&self) -> &'static str {
        match self {
            Self::UpsertStatute { .. } => "upsert_statute",
            Self::RecordDiff { .. } => "record_diff",
        }
    }
}

/// Whether a queued operation has been synchronized to a peer / server yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpSyncState {
    /// Created locally, not yet synchronized.
    Pending,
    /// Confirmed synchronized.
    Synced,
}

/// A single entry in the offline operation queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingOperation {
    /// Content-addressed operation id (unique within an engine).
    pub op_id: String,
    /// Monotonic per-engine sequence number.
    pub local_seq: u64,
    /// When the operation was created.
    pub created_at: DateTime<Utc>,
    /// The operation payload.
    pub kind: OperationKind,
    /// Synchronization state.
    pub state: OpSyncState,
}

/// A serializable full snapshot of an [`OfflineEngine`] for persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineSnapshot {
    /// Owning device identifier.
    pub device_id: String,
    /// Next sequence number to assign.
    pub next_seq: u64,
    /// Latest known statute per id (materialised view).
    pub statutes: BTreeMap<String, Statute>,
    /// The operation log.
    pub operations: Vec<PendingOperation>,
}

/// An offline-first diff engine: snapshot store plus append-only operation log.
#[derive(Debug, Clone)]
pub struct OfflineEngine {
    device_id: String,
    next_seq: u64,
    statutes: BTreeMap<String, Statute>,
    operations: Vec<PendingOperation>,
}

impl OfflineEngine {
    /// Creates an empty engine owned by `device_id`.
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
            next_seq: 0,
            statutes: BTreeMap::new(),
            operations: Vec::new(),
        }
    }

    /// The owning device identifier.
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    fn enqueue(&mut self, kind: OperationKind) -> String {
        let seq = self.next_seq;
        self.next_seq += 1;
        // Content-addressed id; falls back to the (unique) seq if serialization
        // somehow fails, so the id is always well-defined and unique.
        let payload = serde_json::to_vec(&kind).unwrap_or_default();
        let op_id = sha256_parts(&[
            self.device_id.as_bytes(),
            &seq.to_le_bytes(),
            kind.label().as_bytes(),
            &payload,
        ]);
        self.operations.push(PendingOperation {
            op_id: op_id.clone(),
            local_seq: seq,
            created_at: Utc::now(),
            kind,
            state: OpSyncState::Pending,
        });
        op_id
    }

    /// Stores `statute` as the latest snapshot and queues an upsert operation.
    /// Returns the queued operation id.
    pub fn put_statute(&mut self, statute: Statute) -> String {
        self.statutes.insert(statute.id.clone(), statute.clone());
        self.enqueue(OperationKind::UpsertStatute {
            statute: Box::new(statute),
        })
    }

    /// Computes the diff between `old` and `new` locally, makes `new` the latest
    /// snapshot and queues the resulting record. Returns the computed diff.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::IdMismatch`] if the two statutes have different ids.
    pub fn record_diff(&mut self, old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
        let computed = diff(old, new)?;
        // `new` becomes current (upsert event keeps the snapshot replayable)...
        self.put_statute(new.clone());
        // ...and the diff itself is recorded as an auditable operation.
        self.enqueue(OperationKind::RecordDiff {
            diff: Box::new(computed.clone()),
        });
        Ok(computed)
    }

    /// Returns the latest known snapshot for `id`, if any.
    pub fn latest(&self, id: &str) -> Option<&Statute> {
        self.statutes.get(id)
    }

    /// The number of distinct statutes tracked.
    pub fn statute_count(&self) -> usize {
        self.statutes.len()
    }

    /// The total number of operations in the log (pending and synced).
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns `true` if the log is empty.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// All operations in the log, in creation order.
    pub fn operations(&self) -> &[PendingOperation] {
        &self.operations
    }

    /// The pending (unsynced) operations, in creation order.
    pub fn pending(&self) -> Vec<&PendingOperation> {
        self.operations
            .iter()
            .filter(|op| op.state == OpSyncState::Pending)
            .collect()
    }

    /// The number of pending operations.
    pub fn pending_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|op| op.state == OpSyncState::Pending)
            .count()
    }

    /// Returns `true` if any pending operation concerns statute `id`.
    pub fn has_pending_for(&self, id: &str) -> bool {
        self.operations.iter().any(|op| {
            op.state == OpSyncState::Pending
                && match &op.kind {
                    OperationKind::UpsertStatute { statute } => statute.id == id,
                    OperationKind::RecordDiff { diff } => diff.statute_id == id,
                }
        })
    }

    /// Marks the operations whose ids are in `op_ids` as synced. Returns how many
    /// operations changed state.
    pub fn mark_synced(&mut self, op_ids: &[String]) -> usize {
        let mut changed = 0;
        for op in &mut self.operations {
            if op.state == OpSyncState::Pending && op_ids.contains(&op.op_id) {
                op.state = OpSyncState::Synced;
                changed += 1;
            }
        }
        changed
    }

    /// Marks every pending operation as synced. Returns how many changed.
    pub fn mark_all_synced(&mut self) -> usize {
        let ids: Vec<String> = self.pending().iter().map(|op| op.op_id.clone()).collect();
        self.mark_synced(&ids)
    }

    /// Removes synced operations from the log (compaction). Returns how many were
    /// removed. Snapshots are unaffected.
    pub fn drain_synced(&mut self) -> usize {
        let before = self.operations.len();
        self.operations.retain(|op| op.state != OpSyncState::Synced);
        before - self.operations.len()
    }

    /// Reconstructs the snapshot map purely from the operation log, by folding
    /// every [`OperationKind::UpsertStatute`] in order. Used for disaster
    /// recovery and to verify materialised-view consistency.
    ///
    /// # Errors
    ///
    /// This implementation is infallible but returns [`DiffResult`] for symmetry
    /// with the rest of the API and forward compatibility.
    pub fn replay(&self) -> DiffResult<BTreeMap<String, Statute>> {
        let mut map = BTreeMap::new();
        for op in &self.operations {
            if let OperationKind::UpsertStatute { statute } = &op.kind {
                map.insert(statute.id.clone(), statute.as_ref().clone());
            }
        }
        Ok(map)
    }

    /// Produces a serializable snapshot of the entire engine state.
    pub fn snapshot(&self) -> OfflineSnapshot {
        OfflineSnapshot {
            device_id: self.device_id.clone(),
            next_seq: self.next_seq,
            statutes: self.statutes.clone(),
            operations: self.operations.clone(),
        }
    }

    /// Restores an engine from a previously produced snapshot.
    pub fn from_snapshot(snapshot: OfflineSnapshot) -> Self {
        Self {
            device_id: snapshot.device_id,
            next_seq: snapshot.next_seq,
            statutes: snapshot.statutes,
            operations: snapshot.operations,
        }
    }

    /// Persists the engine state to `path` as JSON.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the state cannot be encoded
    /// or the file cannot be written.
    pub fn save_to_path(&self, path: impl AsRef<Path>) -> DiffResult<()> {
        let bytes = serde_json::to_vec_pretty(&self.snapshot()).map_err(|e| {
            DiffError::SerializationError(format!("failed to encode offline state: {}", e))
        })?;
        std::fs::write(path.as_ref(), bytes).map_err(|e| {
            DiffError::SerializationError(format!("failed to write offline state: {}", e))
        })
    }

    /// Restores an engine from a JSON file written by [`save_to_path`](Self::save_to_path).
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the file cannot be read or
    /// parsed.
    pub fn load_from_path(path: impl AsRef<Path>) -> DiffResult<Self> {
        let bytes = std::fs::read(path.as_ref()).map_err(|e| {
            DiffError::SerializationError(format!("failed to read offline state: {}", e))
        })?;
        let snapshot: OfflineSnapshot = serde_json::from_slice(&bytes).map_err(|e| {
            DiffError::SerializationError(format!("failed to parse offline state: {}", e))
        })?;
        Ok(Self::from_snapshot(snapshot))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Benefit"))
    }

    #[test]
    fn test_put_statute_tracks_snapshot_and_op() {
        let mut engine = OfflineEngine::new("dev");
        let op_id = engine.put_statute(statute("a", "A"));
        assert_eq!(engine.statute_count(), 1);
        assert_eq!(engine.operation_count(), 1);
        assert!(!op_id.is_empty());
        assert_eq!(
            engine.latest("a").map(|s| s.title.clone()),
            Some("A".to_string())
        );
    }

    #[test]
    fn test_record_diff_updates_snapshot_and_queues() {
        let mut engine = OfflineEngine::new("dev");
        let v1 = statute("law", "V1");
        let mut v2 = v1.clone();
        v2.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });
        engine.put_statute(v1.clone());
        let d = engine.record_diff(&v1, &v2).expect("diff");
        assert!(!d.changes.is_empty());
        // put(v1) + put(v2 via record) + RecordDiff = 3 ops.
        assert_eq!(engine.operation_count(), 3);
        assert_eq!(engine.latest("law").map(|s| s.preconditions.len()), Some(1));
    }

    #[test]
    fn test_record_diff_id_mismatch_errors() {
        let mut engine = OfflineEngine::new("dev");
        let a = statute("a", "A");
        let b = statute("b", "B");
        assert!(matches!(
            engine.record_diff(&a, &b),
            Err(DiffError::IdMismatch(_, _))
        ));
    }

    #[test]
    fn test_sync_lifecycle() {
        let mut engine = OfflineEngine::new("dev");
        engine.put_statute(statute("a", "A"));
        engine.put_statute(statute("b", "B"));
        assert_eq!(engine.pending_count(), 2);
        assert!(engine.has_pending_for("a"));

        let first_id = engine.pending()[0].op_id.clone();
        assert_eq!(engine.mark_synced(&[first_id]), 1);
        assert_eq!(engine.pending_count(), 1);

        assert_eq!(engine.mark_all_synced(), 1);
        assert_eq!(engine.pending_count(), 0);

        let removed = engine.drain_synced();
        assert_eq!(removed, 2);
        assert_eq!(engine.operation_count(), 0);
        // Snapshots survive compaction.
        assert_eq!(engine.statute_count(), 2);
    }

    #[test]
    fn test_replay_matches_materialized_view() {
        let mut engine = OfflineEngine::new("dev");
        let v1 = statute("law", "V1");
        let mut v2 = v1.clone();
        v2.title = "V2".to_string();
        engine.put_statute(v1.clone());
        engine.record_diff(&v1, &v2).expect("diff");

        let replayed = engine.replay().expect("replay");
        assert_eq!(replayed.len(), 1);
        assert_eq!(
            replayed.get("law").map(|s| s.title.clone()),
            Some("V2".to_string())
        );
        // Replayed view equals the live materialised view.
        assert_eq!(
            replayed.get("law").map(|s| s.title.clone()),
            engine.latest("law").map(|s| s.title.clone())
        );
    }

    #[test]
    fn test_persistence_round_trip() {
        let mut engine = OfflineEngine::new("dev");
        let v1 = statute("law", "V1");
        let mut v2 = v1.clone();
        v2.title = "V2".to_string();
        engine.put_statute(v1.clone());
        engine.record_diff(&v1, &v2).expect("diff");

        let mut path = std::env::temp_dir();
        path.push(format!("legalis_offline_{}.json", std::process::id()));
        engine.save_to_path(&path).expect("save");

        let restored = OfflineEngine::load_from_path(&path).expect("load");
        assert_eq!(restored.device_id(), "dev");
        assert_eq!(restored.operation_count(), engine.operation_count());
        assert_eq!(
            restored.latest("law").map(|s| s.title.clone()),
            Some("V2".to_string())
        );

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_load_missing_file_errors() {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "legalis_offline_missing_{}.json",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        assert!(matches!(
            OfflineEngine::load_from_path(&path),
            Err(DiffError::SerializationError(_))
        ));
    }
}
