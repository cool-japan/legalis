//! Real-time bidirectional format synchronisation.
//!
//! [`FormatSyncEngine`] keeps two *different-format* representations of the same
//! legal document in sync. Endpoint A holds the document in format X, endpoint B
//! in format Y. When either side edits its representation, the engine:
//!
//! 1. imports the edited side's text into the shared canonical model,
//! 2. diffs it against the last-known canonical state to derive a [`RegionDelta`],
//! 3. replays the delta as CRDT operations on a shared [`CrdtDocument`] so the
//!    two sides converge conflict-free even under (offline) concurrent edits,
//! 4. re-exports the merged canonical document to *both* formats, returning the
//!    refreshed text for the opposite side (and, on demand, both sides).
//!
//! Because synchronisation is built on the [`crate::realtime::collab`] CRDT,
//! concurrent edits on both endpoints converge deterministically; there is no
//! "last save wins" data loss. The engine is fully offline: it requires no
//! network transport. (Shipping ops between physically-separate endpoints would
//! add a transport, but the convergence logic — the hard part — lives here.)

use super::collab::{CrdtDocument, CrdtOp, Dot};
use super::{CanonicalDocument, RegionDelta, RegionId};
use crate::{ConversionReport, InteropResult, LegalConverter, LegalFormat};
use legalis_core::Statute;
use std::collections::HashMap;

/// Which endpoint of a sync pair an edit originated from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endpoint {
    /// The "A" side (format [`FormatSyncEngine::format_a`]).
    A,
    /// The "B" side (format [`FormatSyncEngine::format_b`]).
    B,
}

impl Endpoint {
    /// The opposite endpoint.
    pub fn other(self) -> Endpoint {
        match self {
            Endpoint::A => Endpoint::B,
            Endpoint::B => Endpoint::A,
        }
    }
}

/// The result of a synchronisation step.
#[derive(Debug, Clone)]
pub struct SyncResult {
    /// Refreshed text for endpoint A in format A.
    pub text_a: String,
    /// Refreshed text for endpoint B in format B.
    pub text_b: String,
    /// The region-level delta that was propagated.
    pub delta: RegionDelta,
    /// Aggregate conversion report for the re-export.
    pub report: ConversionReport,
}

/// A bidirectional, CRDT-backed synchroniser between two formats.
pub struct FormatSyncEngine {
    format_a: LegalFormat,
    format_b: LegalFormat,
    converter: LegalConverter,
    /// Shared conflict-free document state.
    crdt: CrdtDocument,
    /// Last canonical snapshot, used to diff incoming edits.
    last_canonical: CanonicalDocument,
    /// Map from region id (statute id) to its CRDT element handle.
    handles: HashMap<RegionId, Dot>,
}

impl FormatSyncEngine {
    /// Creates a synchroniser between two formats, starting empty.
    ///
    /// Uses a default replica id (`"sync-engine"`). When two engines may edit
    /// concurrently and relay operations to each other, give each a **distinct**
    /// replica id via [`Self::with_replica_id`] so their CRDT identities never
    /// collide; otherwise convergence under concurrent edits is not guaranteed.
    pub fn new(format_a: LegalFormat, format_b: LegalFormat) -> Self {
        Self::with_replica_id(format_a, format_b, "sync-engine")
    }

    /// Creates a synchroniser with an explicit replica id for the underlying
    /// CRDT. Distinct ids are required for two engines that exchange operations.
    pub fn with_replica_id(
        format_a: LegalFormat,
        format_b: LegalFormat,
        replica_id: impl Into<String>,
    ) -> Self {
        Self {
            format_a,
            format_b,
            converter: LegalConverter::new(),
            crdt: CrdtDocument::new(replica_id),
            last_canonical: CanonicalDocument::new(),
            handles: HashMap::new(),
        }
    }

    /// Format held by endpoint A.
    pub fn format_a(&self) -> LegalFormat {
        self.format_a
    }

    /// Format held by endpoint B.
    pub fn format_b(&self) -> LegalFormat {
        self.format_b
    }

    /// The current shared canonical document.
    pub fn canonical(&self) -> InteropResult<CanonicalDocument> {
        self.crdt.to_canonical()
    }

    /// A convergence digest of the shared state (equal across synced engines).
    pub fn digest(&self) -> InteropResult<String> {
        self.crdt.state_digest()
    }

    /// Seeds the engine from an initial document supplied in format A.
    ///
    /// # Errors
    /// Returns an error if the source cannot be imported or re-exported.
    pub fn initialize_from_a(&mut self, text: &str) -> InteropResult<SyncResult> {
        self.initialize_from(Endpoint::A, text)
    }

    /// Seeds the engine from an initial document supplied in format B.
    ///
    /// # Errors
    /// Returns an error if the source cannot be imported or re-exported.
    pub fn initialize_from_b(&mut self, text: &str) -> InteropResult<SyncResult> {
        self.initialize_from(Endpoint::B, text)
    }

    fn format_of(&self, endpoint: Endpoint) -> LegalFormat {
        match endpoint {
            Endpoint::A => self.format_a,
            Endpoint::B => self.format_b,
        }
    }

    /// Seeds the engine from `text` interpreted in `endpoint`'s format.
    fn initialize_from(&mut self, endpoint: Endpoint, text: &str) -> InteropResult<SyncResult> {
        let format = self.format_of(endpoint);
        let (statutes, _report) = self.converter.import(text, format)?;
        let incoming = CanonicalDocument::from_statutes(&statutes)?;

        // Replay as fresh appends into the (empty) CRDT.
        let mut last_handle: Option<Dot> = None;
        for region in incoming.regions() {
            let op = self
                .crdt
                .local_insert_after(last_handle.clone(), region.statute.clone())?;
            let dot = op.op_dot().clone();
            self.handles.insert(region.id.clone(), dot.clone());
            last_handle = Some(dot);
        }
        self.refresh_snapshot()?;
        self.reexport(incoming.delta_from(&CanonicalDocument::new()))
    }

    /// Applies an edit made on `endpoint` (its full updated text) and propagates
    /// it to the other side. Returns refreshed text for both sides.
    ///
    /// # Errors
    /// Returns an error if import/export fails.
    pub fn apply_edit(&mut self, endpoint: Endpoint, text: &str) -> InteropResult<SyncResult> {
        let format = self.format_of(endpoint);
        let (statutes, _report) = self.converter.import(text, format)?;
        let incoming = CanonicalDocument::from_statutes(&statutes)?;
        let delta = incoming.delta_from(&self.last_canonical);
        self.propagate(&incoming, &delta)?;
        self.refresh_snapshot()?;
        self.reexport(delta)
    }

    /// Applies a set of externally-produced CRDT operations (e.g. relayed from a
    /// peer engine) and re-exports both sides. This is the hook a future network
    /// transport would call; it keeps convergence guarantees intact.
    ///
    /// # Errors
    /// Returns an error if applying or re-exporting fails.
    pub fn apply_remote_ops(
        &mut self,
        ops: impl IntoIterator<Item = CrdtOp>,
    ) -> InteropResult<SyncResult> {
        let before = self.last_canonical.clone();
        for op in ops {
            // Keep the handle map in step with inserts so later edits can target
            // these regions by id.
            if let CrdtOp::Insert { dot, statute, .. } = &op {
                self.handles.insert(statute.id.clone(), dot.clone());
            }
            self.crdt.apply(op)?;
        }
        let after = self.crdt.to_canonical()?;
        let delta = after.delta_from(&before);
        self.refresh_snapshot()?;
        self.reexport(delta)
    }

    /// Translates a region-level delta into CRDT operations and applies them.
    fn propagate(
        &mut self,
        incoming: &CanonicalDocument,
        delta: &RegionDelta,
    ) -> InteropResult<()> {
        // Updates: LWW content writes on existing handles.
        for id in &delta.updated {
            if let (Some(handle), Some(region)) =
                (self.handles.get(id).cloned(), incoming.region(id))
            {
                self.crdt.local_update(handle, region.statute.clone())?;
            }
        }
        // Removals: tombstone the element.
        for id in &delta.removed {
            if let Some(handle) = self.handles.get(id).cloned() {
                self.crdt.local_delete(handle)?;
                self.handles.remove(id);
            }
        }
        // Additions: insert after the element preceding them in the incoming doc
        // so order is preserved.
        let order: Vec<&Statute> = incoming.regions().iter().map(|r| &r.statute).collect();
        for id in &delta.added {
            // Find the predecessor (in incoming order) that already has a handle.
            let mut anchor: Option<Dot> = None;
            if let Some(pos) = order.iter().position(|s| &s.id == id) {
                for prev in order[..pos].iter().rev() {
                    if let Some(h) = self.handles.get(&prev.id) {
                        anchor = Some(h.clone());
                        break;
                    }
                }
            }
            if let Some(region) = incoming.region(id) {
                let op = self
                    .crdt
                    .local_insert_after(anchor, region.statute.clone())?;
                self.handles.insert(id.clone(), op.op_dot().clone());
            }
        }
        // Reordering (no add/remove): apply moves to match incoming order.
        if delta.reordered {
            let mut prev_handle: Option<Dot> = None;
            for region in incoming.regions() {
                if let Some(handle) = self.handles.get(&region.id).cloned() {
                    self.crdt.local_move(handle.clone(), prev_handle.clone())?;
                    prev_handle = Some(handle);
                }
            }
        }
        Ok(())
    }

    /// Refreshes the cached canonical snapshot from the CRDT.
    fn refresh_snapshot(&mut self) -> InteropResult<()> {
        self.last_canonical = self.crdt.to_canonical()?;
        Ok(())
    }

    /// Re-exports the merged document to both formats.
    fn reexport(&mut self, delta: RegionDelta) -> InteropResult<SyncResult> {
        let statutes = self.last_canonical.statutes();
        let (text_a, report_a) = self.converter.export(&statutes, self.format_a)?;
        let (text_b, report_b) = self.converter.export(&statutes, self.format_b)?;

        let mut report = ConversionReport::new(self.format_a, self.format_b);
        report.statutes_converted = statutes.len();
        report
            .unsupported_features
            .extend(report_a.unsupported_features);
        report
            .unsupported_features
            .extend(report_b.unsupported_features);
        report.warnings.extend(report_a.warnings);
        report.warnings.extend(report_b.warnings);
        report.confidence = (report_a.confidence * report_b.confidence).max(0.0);

        Ok(SyncResult {
            text_a,
            text_b,
            delta,
            report,
        })
    }

    /// Returns the CRDT operations needed to bring a fresh peer up to date
    /// (snapshot of current state). A network transport would ship these.
    pub fn snapshot_ops(&self) -> Vec<CrdtOp> {
        self.crdt.snapshot_ops()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    fn statute(id: &str, title: &str, desc: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, desc))
    }

    // Formats chosen for lossless multi-statute round-trips (so `apply_edit`,
    // which re-imports the edited side, sees the expected region counts *and*
    // detects field-level content changes): LLM-native on side A, neural-document
    // on side B. Both are genuinely different serializations sharing the
    // provenance backbone, so this exercises real cross-format synchronisation
    // without conflating engine behaviour with a target format's lossy mapping.
    const FMT_A: LegalFormat = LegalFormat::LlmNative;
    const FMT_B: LegalFormat = LegalFormat::NeuralDocument;

    /// Exports statutes through the converter into `fmt`, so the sync engine can
    /// later import them. Keeps tests independent of hand-written DSL syntax.
    fn as_format(statutes: &[Statute], fmt: LegalFormat) -> String {
        let mut c = LegalConverter::new();
        c.export(statutes, fmt).expect("export").0
    }

    #[test]
    fn initialize_populates_both_sides() {
        let mut eng = FormatSyncEngine::new(FMT_A, FMT_B);
        let src = as_format(&[statute("a", "A", "x"), statute("b", "B", "y")], FMT_A);
        let res = eng.initialize_from_a(&src).expect("init");
        assert!(!res.text_a.is_empty());
        assert!(!res.text_b.is_empty());
        assert_eq!(eng.canonical().expect("canon").len(), 2);
    }

    #[test]
    fn edit_on_a_propagates_to_b() {
        let mut eng = FormatSyncEngine::new(FMT_A, FMT_B);
        let src = as_format(&[statute("a", "A", "x")], FMT_A);
        eng.initialize_from_a(&src).expect("init");

        // Edit on A: add a region.
        let edited = as_format(&[statute("a", "A", "x"), statute("b", "B", "new")], FMT_A);
        let res = eng.apply_edit(Endpoint::A, &edited).expect("edit");
        assert_eq!(res.delta.added, vec!["b".to_string()]);
        // Canonical now has both regions; B side reflects them.
        assert_eq!(eng.canonical().expect("canon").len(), 2);
        assert!(!res.text_b.is_empty());
    }

    #[test]
    fn edit_on_b_propagates_to_a() {
        let mut eng = FormatSyncEngine::new(FMT_A, FMT_B);
        let src = as_format(&[statute("a", "A", "x"), statute("b", "B", "y")], FMT_A);
        eng.initialize_from_a(&src).expect("init");

        // Round-trip the canonical doc into B's format, edit (remove a region),
        // and apply on B.
        let canon = eng.canonical().expect("canon");
        let mut statutes = canon.statutes();
        statutes.retain(|s| s.id != "b"); // remove region b on the B side
        let edited_b = as_format(&statutes, FMT_B);
        let res = eng.apply_edit(Endpoint::B, &edited_b).expect("edit b");
        assert_eq!(res.delta.removed, vec!["b".to_string()]);
        assert_eq!(eng.canonical().expect("c").len(), 1);
    }

    #[test]
    fn update_propagates_and_converges() {
        let mut eng = FormatSyncEngine::new(FMT_A, FMT_B);
        let src = as_format(&[statute("a", "A", "orig")], FMT_A);
        eng.initialize_from_a(&src).expect("init");

        let edited = as_format(&[statute("a", "A", "updated")], FMT_A);
        let res = eng.apply_edit(Endpoint::A, &edited).expect("edit");
        assert_eq!(res.delta.updated, vec!["a".to_string()]);
        let canon = eng.canonical().expect("canon");
        assert_eq!(canon.statutes()[0].effect.description, "updated");
    }

    #[test]
    fn two_engines_converge_via_op_relay() {
        // Two independent engines edit concurrently, then exchange ops. They must
        // converge to the same canonical digest (offline CRDT convergence).
        let mut e1 = FormatSyncEngine::with_replica_id(FMT_A, FMT_B, "e1");
        let mut e2 = FormatSyncEngine::with_replica_id(FMT_A, FMT_B, "e2");

        let seed = as_format(&[statute("a", "A", "x")], FMT_A);
        e1.initialize_from_a(&seed).expect("e1 init");
        // Bring e2 to the same starting point via op relay.
        e2.apply_remote_ops(e1.snapshot_ops()).expect("e2 seed");
        assert_eq!(e1.digest().expect("d1"), e2.digest().expect("d2"));

        // Concurrent edits: e1 adds "b", e2 adds "c".
        let e1_edit = as_format(&[statute("a", "A", "x"), statute("b", "B", "from1")], FMT_A);
        let e2_edit = as_format(&[statute("a", "A", "x"), statute("c", "C", "from2")], FMT_A);
        e1.apply_edit(Endpoint::A, &e1_edit).expect("e1 edit");
        e2.apply_edit(Endpoint::A, &e2_edit).expect("e2 edit");

        // Exchange the new ops both directions.
        let ops1 = e1.snapshot_ops();
        let ops2 = e2.snapshot_ops();
        e1.apply_remote_ops(ops2).expect("e1<-ops2");
        e2.apply_remote_ops(ops1).expect("e2<-ops1");

        // Converged: same digest, both have all three regions.
        assert_eq!(e1.digest().expect("d1"), e2.digest().expect("d2"));
        assert_eq!(e1.canonical().expect("c1").len(), 3);
        assert_eq!(e2.canonical().expect("c2").len(), 3);
    }

    #[test]
    fn endpoint_other_is_involution() {
        assert_eq!(Endpoint::A.other(), Endpoint::B);
        assert_eq!(Endpoint::B.other(), Endpoint::A);
        assert_eq!(Endpoint::A.other().other(), Endpoint::A);
    }
}
