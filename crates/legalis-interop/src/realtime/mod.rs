//! Real-time and collaborative conversion engines (pure-Rust, offline).
//!
//! This module provides a family of *offline* engines for incremental,
//! streaming, collaborative, and multi-view legal-document conversion. None of
//! them require a network transport or a running server: the "real-time" /
//! "collaborative" naming describes the *capability* (incremental, low-latency,
//! conflict-free convergence under concurrent edits) rather than a requirement
//! for live networking. The data structures are transport-agnostic; a network
//! layer can be added on top without changing the engines.
//!
//! The submodules are:
//! - [`live_translate`] — delta-driven incremental translation that
//!   re-translates only the changed regions of a document between two formats,
//!   exposing an `apply_change` API.
//! - [`streaming_convert`] — a chunked/streaming converter that converts a large
//!   document format→format in bounded memory, driven by an explicit state
//!   machine.
//! - [`collab`] — a CRDT-style merge engine (RGA-ordered sequence with
//!   per-field last-writer-wins registers and tombstones) giving conflict-free
//!   convergence for concurrent edits to a structured legal document.
//! - [`sync`] — a bidirectional synchronisation engine that keeps two
//!   different-format representations of the same document in sync over the
//!   collaborative-edit primitives.
//! - [`views`] — multi-format document views that project a single canonical
//!   document into multiple simultaneous, mutually-consistent format views.
//!
//! All engines share the [`CanonicalDocument`] backbone: an ordered list of
//! [`DocumentRegion`]s, each wrapping a single [`Statute`] keyed by its stable
//! id. Content-addressed [`region_fingerprint`]s drive change detection so that
//! work is only performed for regions that actually changed.

use crate::LegalFormat;
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub mod collab;
pub mod live_translate;
pub mod streaming_convert;
pub mod sync;
pub mod views;

/// A stable identifier for a document region.
///
/// A region corresponds to exactly one [`Statute`]; the region key is the
/// statute id. Using the id (rather than a positional index) keeps region
/// identity stable across insertions, deletions, and reorderings, which is what
/// makes incremental and collaborative editing well-behaved.
pub type RegionId = String;

/// A 32-byte content fingerprint of a region's semantic payload.
pub type RegionFingerprint = [u8; 32];

/// Computes a deterministic, content-addressed fingerprint of a statute.
///
/// The fingerprint is derived from the statute's canonical JSON serialization,
/// so two statutes with identical semantic content (regardless of in-memory
/// allocation) produce the same fingerprint, and any change to a field changes
/// the fingerprint. This is the change-detection primitive shared by all
/// real-time engines.
///
/// # Errors
/// Returns [`crate::InteropError::SerializationError`] if the statute cannot be
/// serialized to JSON.
pub fn region_fingerprint(statute: &Statute) -> crate::InteropResult<RegionFingerprint> {
    let json = serde_json::to_vec(statute)
        .map_err(|e| crate::InteropError::SerializationError(e.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(b"legalis.realtime.region.v1");
    hasher.update((json.len() as u64).to_le_bytes());
    hasher.update(&json);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    Ok(out)
}

/// Lower-hex encoding of a [`RegionFingerprint`].
pub fn fingerprint_hex(fingerprint: &RegionFingerprint) -> String {
    let mut s = String::with_capacity(64);
    for byte in fingerprint {
        // Two lowercase hex digits per byte; `write!` to a String is infallible
        // but we avoid `unwrap` by pushing nibbles directly.
        const HEX: &[u8; 16] = b"0123456789abcdef";
        s.push(HEX[(byte >> 4) as usize] as char);
        s.push(HEX[(byte & 0x0f) as usize] as char);
    }
    s
}

/// A single addressable region of a canonical document.
///
/// Each region wraps one [`Statute`] plus its content fingerprint. The
/// fingerprint is recomputed whenever the statute changes so that downstream
/// engines can detect "did this region actually change?" in O(1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRegion {
    /// Stable region identifier (the statute id).
    pub id: RegionId,
    /// The statute payload for this region.
    pub statute: Statute,
    /// Content fingerprint of [`Self::statute`].
    pub fingerprint: RegionFingerprint,
}

impl DocumentRegion {
    /// Creates a region from a statute, computing its fingerprint.
    ///
    /// # Errors
    /// Returns an error if the statute cannot be fingerprinted.
    pub fn new(statute: Statute) -> crate::InteropResult<Self> {
        let id = statute.id.clone();
        let fingerprint = region_fingerprint(&statute)?;
        Ok(Self {
            id,
            statute,
            fingerprint,
        })
    }

    /// Replaces the region payload, recomputing the fingerprint.
    ///
    /// Returns `true` if the fingerprint changed (i.e. the content is different).
    ///
    /// # Errors
    /// Returns an error if the new statute cannot be fingerprinted.
    pub fn set_statute(&mut self, statute: Statute) -> crate::InteropResult<bool> {
        let new_fp = region_fingerprint(&statute)?;
        let changed = new_fp != self.fingerprint;
        self.statute = statute;
        self.fingerprint = new_fp;
        Ok(changed)
    }
}

/// An ordered, keyed collection of [`DocumentRegion`]s forming a single
/// logical legal document.
///
/// The canonical document is the format-neutral source of truth shared by every
/// real-time engine. It preserves region order (so exports are stable) while
/// keeping an id→position index for O(log n) lookups.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanonicalDocument {
    /// Regions in document order.
    regions: Vec<DocumentRegion>,
    /// Index from region id to its position in [`Self::regions`].
    #[serde(skip)]
    index: BTreeMap<RegionId, usize>,
}

impl CanonicalDocument {
    /// Creates an empty canonical document.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a canonical document from an ordered slice of statutes.
    ///
    /// Later statutes with a duplicate id overwrite earlier ones (keeping the
    /// earlier position), matching last-writer-wins semantics on construction.
    ///
    /// # Errors
    /// Returns an error if any statute cannot be fingerprinted.
    pub fn from_statutes(statutes: &[Statute]) -> crate::InteropResult<Self> {
        let mut doc = Self::new();
        for statute in statutes {
            doc.upsert(statute.clone())?;
        }
        Ok(doc)
    }

    /// Rebuilds the position index from the region vector.
    fn reindex(&mut self) {
        self.index.clear();
        for (pos, region) in self.regions.iter().enumerate() {
            self.index.insert(region.id.clone(), pos);
        }
    }

    /// Number of regions in the document.
    pub fn len(&self) -> usize {
        self.regions.len()
    }

    /// Returns `true` if the document has no regions.
    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

    /// Returns the regions in document order.
    pub fn regions(&self) -> &[DocumentRegion] {
        &self.regions
    }

    /// Returns the statutes in document order.
    pub fn statutes(&self) -> Vec<Statute> {
        self.regions.iter().map(|r| r.statute.clone()).collect()
    }

    /// Looks up a region by id.
    pub fn region(&self, id: &str) -> Option<&DocumentRegion> {
        // The index may be stale only between mutating ops; all public mutators
        // keep it consistent, so a direct lookup is safe.
        self.index.get(id).and_then(|&pos| self.regions.get(pos))
    }

    /// Returns `true` if a region with the given id exists.
    pub fn contains(&self, id: &str) -> bool {
        self.index.contains_key(id)
    }

    /// Inserts a new statute or replaces an existing one with the same id.
    ///
    /// Returns the [`ChangeKind`] describing what happened (insert vs. an update
    /// that changed content vs. a no-op update with identical content).
    ///
    /// # Errors
    /// Returns an error if the statute cannot be fingerprinted.
    pub fn upsert(&mut self, statute: Statute) -> crate::InteropResult<ChangeKind> {
        let id = statute.id.clone();
        if let Some(&pos) = self.index.get(&id)
            && let Some(region) = self.regions.get_mut(pos)
        {
            let changed = region.set_statute(statute)?;
            return Ok(if changed {
                ChangeKind::Updated
            } else {
                ChangeKind::Unchanged
            });
        }
        let region = DocumentRegion::new(statute)?;
        let pos = self.regions.len();
        self.index.insert(id, pos);
        self.regions.push(region);
        Ok(ChangeKind::Inserted)
    }

    /// Inserts a statute at a specific position (clamped to the current length).
    ///
    /// If the id already exists it is updated in place (position unchanged).
    ///
    /// # Errors
    /// Returns an error if the statute cannot be fingerprinted.
    pub fn insert_at(
        &mut self,
        position: usize,
        statute: Statute,
    ) -> crate::InteropResult<ChangeKind> {
        let id = statute.id.clone();
        if self.index.contains_key(&id) {
            return self.upsert(statute);
        }
        let pos = position.min(self.regions.len());
        let region = DocumentRegion::new(statute)?;
        self.regions.insert(pos, region);
        self.reindex();
        Ok(ChangeKind::Inserted)
    }

    /// Removes a region by id, returning the removed statute if present.
    pub fn remove(&mut self, id: &str) -> Option<Statute> {
        let pos = *self.index.get(id)?;
        let region = self.regions.remove(pos);
        self.reindex();
        Some(region.statute)
    }

    /// Returns the fingerprint of a region, if it exists.
    pub fn fingerprint_of(&self, id: &str) -> Option<RegionFingerprint> {
        self.region(id).map(|r| r.fingerprint)
    }

    /// Returns an ordered map of region id → fingerprint, suitable for diffing
    /// two documents cheaply.
    pub fn fingerprint_map(&self) -> BTreeMap<RegionId, RegionFingerprint> {
        self.regions
            .iter()
            .map(|r| (r.id.clone(), r.fingerprint))
            .collect()
    }

    /// Computes a [`RegionDelta`] describing how `self` differs from a previous
    /// snapshot (`previous` → `self`).
    pub fn delta_from(&self, previous: &CanonicalDocument) -> RegionDelta {
        let prev = previous.fingerprint_map();
        let curr = self.fingerprint_map();

        let mut added = Vec::new();
        let mut updated = Vec::new();
        let mut removed = Vec::new();

        for (id, fp) in &curr {
            match prev.get(id) {
                None => added.push(id.clone()),
                Some(prev_fp) if prev_fp != fp => updated.push(id.clone()),
                Some(_) => {}
            }
        }
        for id in prev.keys() {
            if !curr.contains_key(id) {
                removed.push(id.clone());
            }
        }
        // Detect reordering: same id-set but a different order.
        let prev_order: Vec<&String> = previous.regions.iter().map(|r| &r.id).collect();
        let curr_order: Vec<&String> = self.regions.iter().map(|r| &r.id).collect();
        let reordered = added.is_empty()
            && removed.is_empty()
            && prev_order.len() == curr_order.len()
            && prev_order != curr_order;

        RegionDelta {
            added,
            updated,
            removed,
            reordered,
        }
    }
}

/// Classification of what a single region-level mutation did.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeKind {
    /// A new region was inserted.
    Inserted,
    /// An existing region's content changed.
    Updated,
    /// An existing region was overwritten with identical content (no-op).
    Unchanged,
    /// A region was removed.
    Removed,
}

/// A region-level diff between two canonical-document snapshots.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionDelta {
    /// Ids of regions present now but not before.
    pub added: Vec<RegionId>,
    /// Ids of regions whose content changed.
    pub updated: Vec<RegionId>,
    /// Ids of regions present before but not now.
    pub removed: Vec<RegionId>,
    /// Whether the surviving regions were reordered (no add/remove).
    pub reordered: bool,
}

impl RegionDelta {
    /// Returns `true` if there are no structural or content changes.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty()
            && self.updated.is_empty()
            && self.removed.is_empty()
            && !self.reordered
    }

    /// Ids that require (re-)translation: additions and updates.
    pub fn touched(&self) -> Vec<RegionId> {
        let mut out = Vec::with_capacity(self.added.len() + self.updated.len());
        out.extend(self.added.iter().cloned());
        out.extend(self.updated.iter().cloned());
        out
    }

    /// Total number of changed regions (added + updated + removed).
    pub fn change_count(&self) -> usize {
        self.added.len() + self.updated.len() + self.removed.len()
    }
}

/// An edit operation against a [`CanonicalDocument`].
///
/// These describe *intent* (insert / update / remove / move a region) and are
/// the unit of change shared by the live translator, the collaborative engine,
/// and the synchroniser. They are serializable so they can be journalled or, in
/// a future networked deployment, shipped over a transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentChange {
    /// Insert a new region at a position (clamped). If the id already exists this
    /// behaves like [`DocumentChange::Update`].
    Insert {
        /// Desired position in document order.
        position: usize,
        /// The statute to insert.
        statute: Box<Statute>,
    },
    /// Replace the content of an existing region (or insert if absent).
    Update {
        /// The replacement statute (its id selects the region).
        statute: Box<Statute>,
    },
    /// Remove the region with the given id.
    Remove {
        /// Region id to remove.
        id: RegionId,
    },
    /// Move an existing region to a new position.
    Move {
        /// Region id to move.
        id: RegionId,
        /// New position in document order (clamped).
        position: usize,
    },
}

impl DocumentChange {
    /// Convenience constructor for an [`DocumentChange::Update`].
    pub fn update(statute: Statute) -> Self {
        DocumentChange::Update {
            statute: Box::new(statute),
        }
    }

    /// Convenience constructor for an [`DocumentChange::Insert`] appended at the
    /// end (using `usize::MAX`, which is clamped to the document length).
    pub fn append(statute: Statute) -> Self {
        DocumentChange::Insert {
            position: usize::MAX,
            statute: Box::new(statute),
        }
    }

    /// Convenience constructor for a [`DocumentChange::Remove`].
    pub fn remove(id: impl Into<String>) -> Self {
        DocumentChange::Remove { id: id.into() }
    }

    /// The region id this change targets.
    pub fn target_id(&self) -> &str {
        match self {
            DocumentChange::Insert { statute, .. } => &statute.id,
            DocumentChange::Update { statute } => &statute.id,
            DocumentChange::Remove { id } => id,
            DocumentChange::Move { id, .. } => id,
        }
    }

    /// Applies this change to a canonical document, returning the resulting
    /// [`ChangeKind`].
    ///
    /// # Errors
    /// Returns an error if a contained statute cannot be fingerprinted.
    pub fn apply_to(&self, doc: &mut CanonicalDocument) -> crate::InteropResult<ChangeKind> {
        match self {
            DocumentChange::Insert { position, statute } => {
                doc.insert_at(*position, (**statute).clone())
            }
            DocumentChange::Update { statute } => doc.upsert((**statute).clone()),
            DocumentChange::Remove { id } => Ok(doc
                .remove(id)
                .map(|_| ChangeKind::Removed)
                .unwrap_or(ChangeKind::Unchanged)),
            DocumentChange::Move { id, position } => {
                if let Some(statute) = doc.remove(id) {
                    doc.insert_at(*position, statute)?;
                    Ok(ChangeKind::Updated)
                } else {
                    Ok(ChangeKind::Unchanged)
                }
            }
        }
    }
}

/// Identifies a format pairing for translation (source → target).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FormatPair {
    /// Source format.
    pub from: LegalFormat,
    /// Target format.
    pub to: LegalFormat,
}

impl FormatPair {
    /// Creates a new format pair.
    pub fn new(from: LegalFormat, to: LegalFormat) -> Self {
        Self { from, to }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn statute(id: &str, title: &str, desc: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, desc))
    }

    #[test]
    fn fingerprint_is_deterministic_and_content_sensitive() {
        let a = statute("s1", "Title", "grant something");
        let b = statute("s1", "Title", "grant something");
        let c = statute("s1", "Title", "grant something else");

        let fa = region_fingerprint(&a).expect("fingerprint a");
        let fb = region_fingerprint(&b).expect("fingerprint b");
        let fc = region_fingerprint(&c).expect("fingerprint c");

        assert_eq!(fa, fb, "identical content => identical fingerprint");
        assert_ne!(fa, fc, "different content => different fingerprint");
        assert_eq!(fingerprint_hex(&fa).len(), 64);
    }

    #[test]
    fn upsert_classifies_changes() {
        let mut doc = CanonicalDocument::new();
        assert_eq!(
            doc.upsert(statute("a", "A", "x")).expect("ins"),
            ChangeKind::Inserted
        );
        assert_eq!(
            doc.upsert(statute("a", "A", "x")).expect("noop"),
            ChangeKind::Unchanged
        );
        assert_eq!(
            doc.upsert(statute("a", "A", "y")).expect("upd"),
            ChangeKind::Updated
        );
        assert_eq!(doc.len(), 1);
    }

    #[test]
    fn insert_at_preserves_order_and_index() {
        let mut doc = CanonicalDocument::new();
        doc.insert_at(0, statute("a", "A", "x")).expect("a");
        doc.insert_at(0, statute("b", "B", "x")).expect("b");
        doc.insert_at(1, statute("c", "C", "x")).expect("c");
        let order: Vec<&str> = doc.regions().iter().map(|r| r.id.as_str()).collect();
        assert_eq!(order, vec!["b", "c", "a"]);
        // Index must agree with positions.
        assert!(doc.contains("c"));
        assert_eq!(doc.region("c").map(|r| r.id.as_str()), Some("c"));
    }

    #[test]
    fn remove_updates_index() {
        let mut doc = CanonicalDocument::new();
        doc.upsert(statute("a", "A", "x")).expect("a");
        doc.upsert(statute("b", "B", "x")).expect("b");
        doc.upsert(statute("c", "C", "x")).expect("c");
        let removed = doc.remove("b").expect("removed b");
        assert_eq!(removed.id, "b");
        assert!(!doc.contains("b"));
        // Remaining ids still resolve correctly.
        assert_eq!(doc.region("c").map(|r| r.id.clone()), Some("c".to_string()));
        assert_eq!(doc.len(), 2);
    }

    #[test]
    fn delta_detects_add_update_remove() {
        let prev =
            CanonicalDocument::from_statutes(&[statute("a", "A", "x"), statute("b", "B", "x")])
                .expect("prev");
        let curr = CanonicalDocument::from_statutes(&[
            statute("a", "A", "x"),       // unchanged
            statute("b", "B", "changed"), // updated
            statute("c", "C", "x"),       // added
        ])
        .expect("curr");

        let delta = curr.delta_from(&prev);
        assert_eq!(delta.added, vec!["c".to_string()]);
        assert_eq!(delta.updated, vec!["b".to_string()]);
        assert!(delta.removed.is_empty());
        assert!(!delta.reordered);
        assert_eq!(delta.change_count(), 2);
        assert_eq!(delta.touched().len(), 2);
    }

    #[test]
    fn delta_detects_reordering() {
        let prev =
            CanonicalDocument::from_statutes(&[statute("a", "A", "x"), statute("b", "B", "x")])
                .expect("prev");
        let mut curr = prev.clone();
        let a = curr.remove("a").expect("remove a");
        curr.insert_at(usize::MAX, a).expect("reinsert a");
        let delta = curr.delta_from(&prev);
        assert!(delta.reordered);
        assert!(delta.added.is_empty() && delta.removed.is_empty());
    }

    #[test]
    fn document_change_apply_roundtrip() {
        let mut doc = CanonicalDocument::new();
        DocumentChange::append(statute("a", "A", "x"))
            .apply_to(&mut doc)
            .expect("append");
        assert_eq!(doc.len(), 1);
        let kind = DocumentChange::update(statute("a", "A", "y"))
            .apply_to(&mut doc)
            .expect("update");
        assert_eq!(kind, ChangeKind::Updated);
        let kind = DocumentChange::remove("a")
            .apply_to(&mut doc)
            .expect("remove");
        assert_eq!(kind, ChangeKind::Removed);
        assert!(doc.is_empty());
    }

    #[test]
    fn document_change_move_changes_order() {
        let mut doc = CanonicalDocument::from_statutes(&[
            statute("a", "A", "x"),
            statute("b", "B", "x"),
            statute("c", "C", "x"),
        ])
        .expect("doc");
        DocumentChange::Move {
            id: "c".to_string(),
            position: 0,
        }
        .apply_to(&mut doc)
        .expect("move");
        let order: Vec<&str> = doc.regions().iter().map(|r| r.id.as_str()).collect();
        assert_eq!(order, vec!["c", "a", "b"]);
    }

    #[test]
    fn condition_change_is_detected_by_fingerprint() {
        let base = statute("s", "S", "x");
        let with_cond = base.clone().with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });
        let fa = region_fingerprint(&base).expect("fa");
        let fb = region_fingerprint(&with_cond).expect("fb");
        assert_ne!(fa, fb);
    }
}
