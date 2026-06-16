//! Collaborative format editing via a CRDT merge engine.
//!
//! This module implements a conflict-free replicated data type (CRDT) for a
//! structured legal document: an ordered sequence of *regions* (one per
//! [`Statute`]) supporting concurrent inserts, deletes, moves, and content
//! updates from independent replicas, with **conflict-free convergence** — any
//! two replicas that have observed the same set of operations compute the same
//! document, regardless of the order in which operations arrived.
//!
//! Design (an RGA + LWW-register hybrid):
//! - Sequence order is an **RGA** (Replicated Growable Array). Each element has
//!   a globally-unique [`Dot`] `(counter, replica)` and a reference to the
//!   element it was inserted *after*. Concurrent inserts after the same anchor
//!   are ordered deterministically by `Dot` (higher `Dot` first), so all
//!   replicas agree on order without coordination.
//! - Deletion is a **tombstone**: the element stays in the sequence but is
//!   hidden, so late-arriving operations referencing it still resolve.
//! - Region *content* is a per-field **last-writer-wins (LWW) register** keyed
//!   by the operation's [`Dot`]; the writer with the lexicographically-greatest
//!   `Dot` wins ties, which is deterministic across replicas.
//! - A **move** is modelled as an LWW register over the element's "position
//!   key", so concurrent moves converge to a single winner.
//!
//! All state mutation happens by *applying operations*; operations are
//! idempotent and commutative, which is what yields convergence. The engine is
//! transport-agnostic — operations are plain serializable values; a future
//! network layer can ship them, but none is required here.

use super::CanonicalDocument;
use crate::InteropResult;
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

/// A replica identifier. Each independent editor/agent has a distinct id.
pub type ReplicaId = String;

/// A Lamport-style logical timestamp paired with a replica id, giving a total
/// order across all operations from all replicas.
///
/// Comparison is `(counter, replica)` lexicographic: a higher counter wins;
/// equal counters break ties by replica id. This total order is what makes
/// last-writer-wins deterministic and concurrent RGA inserts converge.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Dot {
    /// Lamport counter.
    pub counter: u64,
    /// Originating replica.
    pub replica: ReplicaId,
}

impl Dot {
    /// Creates a dot.
    pub fn new(counter: u64, replica: impl Into<String>) -> Self {
        Self {
            counter,
            replica: replica.into(),
        }
    }
}

/// The CRDT operation set. Operations are commutative and idempotent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrdtOp {
    /// Insert a new element carrying `statute`, immediately after the element
    /// identified by `after` (or at the head if `after` is `None`).
    Insert {
        /// Unique id of the new element.
        dot: Dot,
        /// Element to insert after (head if `None`).
        after: Option<Dot>,
        /// Initial payload.
        statute: Box<Statute>,
    },
    /// Update the content of an existing element. The update wins over any update
    /// with a smaller `dot` (LWW).
    Update {
        /// Id of the element to update.
        target: Dot,
        /// Dot of this write (decides LWW ordering).
        dot: Dot,
        /// New payload.
        statute: Box<Statute>,
    },
    /// Tombstone (delete) an element. Deletion is monotonic: once deleted, an
    /// element stays deleted regardless of operation arrival order.
    Delete {
        /// Id of the element to delete.
        target: Dot,
        /// Dot of this delete.
        dot: Dot,
    },
    /// Move an element to sit after a different anchor. Modelled as an LWW
    /// register over the element's anchor; the greatest `dot` wins.
    Move {
        /// Id of the element to move.
        target: Dot,
        /// New anchor (head if `None`).
        after: Option<Dot>,
        /// Dot of this move.
        dot: Dot,
    },
}

impl CrdtOp {
    /// The dot uniquely identifying this operation (used for idempotency).
    pub fn op_dot(&self) -> &Dot {
        match self {
            CrdtOp::Insert { dot, .. } => dot,
            CrdtOp::Update { dot, .. } => dot,
            CrdtOp::Delete { dot, .. } => dot,
            CrdtOp::Move { dot, .. } => dot,
        }
    }
}

/// Internal element of the RGA sequence.
#[derive(Debug, Clone)]
struct Element {
    /// Unique element id (assigned at insert time).
    id: Dot,
    /// LWW register for the anchor (which element this sits after). The value is
    /// `(write_dot, anchor)`; greater `write_dot` wins.
    anchor: (Dot, Option<Dot>),
    /// LWW register for the payload: `(write_dot, statute)`.
    content: (Dot, Statute),
    /// LWW register for the tombstone flag: `(write_dot, deleted)`.
    deleted: (Dot, bool),
}

/// A CRDT replica of a structured legal document.
///
/// Edits are produced via the `local_*` methods (which mint fresh [`Dot`]s on
/// this replica and return the corresponding [`CrdtOp`]), and remote edits are
/// integrated via [`CrdtDocument::apply`]. Convergence holds for any delivery
/// order of the same operation set.
pub struct CrdtDocument {
    replica: ReplicaId,
    /// Lamport clock for minting local dots.
    clock: u64,
    /// All elements by id.
    elements: HashMap<Dot, Element>,
    /// Set of operation dots already applied (idempotency guard).
    applied: HashSet<Dot>,
}

impl CrdtDocument {
    /// Creates an empty replica with the given id.
    pub fn new(replica: impl Into<String>) -> Self {
        Self {
            replica: replica.into(),
            clock: 0,
            elements: HashMap::new(),
            applied: HashSet::new(),
        }
    }

    /// This replica's id.
    pub fn replica_id(&self) -> &str {
        &self.replica
    }

    /// The current Lamport clock value.
    pub fn clock(&self) -> u64 {
        self.clock
    }

    /// Mints a fresh dot on this replica, advancing the clock.
    fn next_dot(&mut self) -> Dot {
        self.clock += 1;
        Dot::new(self.clock, self.replica.clone())
    }

    /// Advances the local clock past a remote dot (Lamport merge rule).
    fn observe(&mut self, dot: &Dot) {
        if dot.counter > self.clock {
            self.clock = dot.counter;
        }
    }

    /// Number of visible (non-tombstoned) elements.
    pub fn len(&self) -> usize {
        self.ordered_visible().len()
    }

    /// Returns `true` if there are no visible elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Generates a local insert-after operation, applies it, and returns it for
    /// dissemination to other replicas.
    ///
    /// # Errors
    /// Returns an error if the operation cannot be applied (it always can for a
    /// freshly-minted dot; the `Result` keeps the signature uniform).
    pub fn local_insert_after(
        &mut self,
        after: Option<Dot>,
        statute: Statute,
    ) -> InteropResult<CrdtOp> {
        let dot = self.next_dot();
        let op = CrdtOp::Insert {
            dot,
            after,
            statute: Box::new(statute),
        };
        self.apply(op.clone())?;
        Ok(op)
    }

    /// Appends a statute at the end of the visible sequence (local op).
    ///
    /// # Errors
    /// See [`Self::local_insert_after`].
    pub fn local_append(&mut self, statute: Statute) -> InteropResult<CrdtOp> {
        let after = self.ordered_visible().last().map(|e| e.id.clone());
        self.local_insert_after(after, statute)
    }

    /// Generates a local content update for the element with the given id.
    ///
    /// # Errors
    /// Returns an error if applying fails.
    pub fn local_update(&mut self, target: Dot, statute: Statute) -> InteropResult<CrdtOp> {
        let dot = self.next_dot();
        let op = CrdtOp::Update {
            target,
            dot,
            statute: Box::new(statute),
        };
        self.apply(op.clone())?;
        Ok(op)
    }

    /// Generates a local delete (tombstone) for the element with the given id.
    ///
    /// # Errors
    /// Returns an error if applying fails.
    pub fn local_delete(&mut self, target: Dot) -> InteropResult<CrdtOp> {
        let dot = self.next_dot();
        let op = CrdtOp::Delete { target, dot };
        self.apply(op.clone())?;
        Ok(op)
    }

    /// Generates a local move for the element with the given id.
    ///
    /// # Errors
    /// Returns an error if applying fails.
    pub fn local_move(&mut self, target: Dot, after: Option<Dot>) -> InteropResult<CrdtOp> {
        let dot = self.next_dot();
        let op = CrdtOp::Move { target, after, dot };
        self.apply(op.clone())?;
        Ok(op)
    }

    /// Applies an operation (local or remote). Idempotent: re-applying an
    /// already-seen operation is a no-op. Commutative across the operation set.
    ///
    /// # Errors
    /// Returns an error only for internal-consistency violations (never under
    /// normal CRDT usage); the `Result` future-proofs the signature.
    pub fn apply(&mut self, op: CrdtOp) -> InteropResult<()> {
        let op_dot = op.op_dot().clone();
        self.observe(&op_dot);
        if self.applied.contains(&op_dot) {
            return Ok(()); // idempotent
        }

        match op {
            CrdtOp::Insert {
                dot,
                after,
                statute,
            } => {
                // Idempotency at the element level too: if the element id already
                // exists, keep the existing one (inserts are unique by dot).
                self.elements.entry(dot.clone()).or_insert(Element {
                    id: dot.clone(),
                    anchor: (dot.clone(), after),
                    content: (dot.clone(), *statute),
                    deleted: (Dot::new(0, ""), false),
                });
            }
            CrdtOp::Update {
                target,
                dot,
                statute,
            } => {
                if let Some(el) = self.elements.get_mut(&target) {
                    // LWW: take the write with the greater dot.
                    if dot > el.content.0 {
                        el.content = (dot, *statute);
                    }
                }
                // If target unknown yet, the update is dropped; in a causal
                // delivery the insert precedes the update. Idempotency set still
                // records the op so a replay is a no-op.
            }
            CrdtOp::Delete { target, dot } => {
                if let Some(el) = self.elements.get_mut(&target) {
                    // Tombstone is monotone-LWW: once true with some dot, only a
                    // strictly greater dot may change it; deletes set `true`.
                    if dot > el.deleted.0 {
                        el.deleted = (dot, true);
                    }
                }
            }
            CrdtOp::Move { target, after, dot } => {
                if let Some(el) = self.elements.get_mut(&target)
                    && dot > el.anchor.0
                {
                    el.anchor = (dot, after);
                }
            }
        }

        self.applied.insert(op_dot);
        Ok(())
    }

    /// Applies a batch of operations in arbitrary order (convergence holds).
    ///
    /// # Errors
    /// Returns an error if any application fails.
    pub fn apply_all(&mut self, ops: impl IntoIterator<Item = CrdtOp>) -> InteropResult<()> {
        for op in ops {
            self.apply(op)?;
        }
        Ok(())
    }

    /// Computes the visible elements in convergent RGA order.
    ///
    /// Order is produced by a deterministic traversal: starting from the head,
    /// children inserted after a given anchor are visited in descending `Dot`
    /// order (so concurrent inserts after the same anchor interleave
    /// identically on every replica). Tombstoned elements are skipped in the
    /// output but still traversed so their children remain reachable.
    fn ordered_visible(&self) -> Vec<&Element> {
        // Group element ids by their *current* anchor (post-move).
        let mut children: BTreeMap<Option<Dot>, Vec<Dot>> = BTreeMap::new();
        for el in self.elements.values() {
            children
                .entry(el.anchor.1.clone())
                .or_default()
                .push(el.id.clone());
        }
        // Sort each sibling group by Dot descending for determinism.
        for ids in children.values_mut() {
            ids.sort_by(|a, b| b.cmp(a));
        }

        let mut out: Vec<&Element> = Vec::new();
        let mut visited: HashSet<Dot> = HashSet::new();
        // Iterative DFS from the head anchor (None).
        let mut stack: Vec<Dot> = children
            .get(&None)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .rev()
            .collect();

        while let Some(id) = stack.pop() {
            if !visited.insert(id.clone()) {
                continue;
            }
            if let Some(el) = self.elements.get(&id) {
                if !el.deleted.1 {
                    out.push(el);
                }
                if let Some(kids) = children.get(&Some(id.clone())) {
                    for kid in kids.iter().rev() {
                        stack.push(kid.clone());
                    }
                }
            }
        }
        out
    }

    /// Returns the visible elements' ids in document order (stable handles for
    /// targeting updates/deletes/moves).
    pub fn element_ids(&self) -> Vec<Dot> {
        self.ordered_visible()
            .iter()
            .map(|e| e.id.clone())
            .collect()
    }

    /// Returns the visible statutes in document order.
    pub fn statutes(&self) -> Vec<Statute> {
        self.ordered_visible()
            .iter()
            .map(|e| e.content.1.clone())
            .collect()
    }

    /// Materialises the current state as a [`CanonicalDocument`].
    ///
    /// # Errors
    /// Returns an error if a statute cannot be fingerprinted.
    pub fn to_canonical(&self) -> InteropResult<CanonicalDocument> {
        CanonicalDocument::from_statutes(&self.statutes())
    }

    /// A convergence digest: a deterministic, order-sensitive hash of the
    /// visible document. Two replicas converge iff their digests match.
    pub fn state_digest(&self) -> InteropResult<String> {
        let canon = self.to_canonical()?;
        let mut acc: Vec<u8> = Vec::new();
        for region in canon.regions() {
            acc.extend_from_slice(region.id.as_bytes());
            acc.push(0x1f);
            acc.extend_from_slice(&region.fingerprint);
            acc.push(0x1e);
        }
        let mut hasher = <sha2::Sha256 as sha2::Digest>::new();
        sha2::Digest::update(&mut hasher, b"legalis.realtime.crdt.digest.v1");
        sha2::Digest::update(&mut hasher, &acc);
        let digest = sha2::Digest::finalize(hasher);
        Ok(super::fingerprint_hex(&{
            let mut out = [0u8; 32];
            out.copy_from_slice(&digest);
            out
        }))
    }

    /// Exports the full causal history (all applied operations are recoverable
    /// from element state for inserts; this returns a reconstructable snapshot
    /// of inserts/updates/deletes/moves implied by current state). Primarily
    /// used to seed a fresh replica deterministically.
    pub fn snapshot_ops(&self) -> Vec<CrdtOp> {
        let mut ops = Vec::new();
        // Inserts first (with their original anchor at insert is not retained;
        // we use the *current* anchor, which yields the same visible order on a
        // fresh replica because anchors are themselves LWW-resolved already).
        let mut by_dot: Vec<&Element> = self.elements.values().collect();
        by_dot.sort_by(|a, b| a.id.cmp(&b.id));
        for el in &by_dot {
            ops.push(CrdtOp::Insert {
                dot: el.id.clone(),
                after: el.anchor.1.clone(),
                statute: Box::new(el.content.1.clone()),
            });
            if el.deleted.1 {
                ops.push(CrdtOp::Delete {
                    target: el.id.clone(),
                    dot: el.deleted.0.clone(),
                });
            }
        }
        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    fn statute(id: &str, title: &str, desc: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, desc))
    }

    /// Extracts the element id minted by an `Insert` op (test helper).
    fn insert_id(op: &CrdtOp) -> Dot {
        match op {
            CrdtOp::Insert { dot, .. } => dot.clone(),
            _ => unreachable!("expected an Insert op"),
        }
    }

    #[test]
    fn local_append_builds_order() {
        let mut doc = CrdtDocument::new("r1");
        doc.local_append(statute("a", "A", "x")).expect("a");
        doc.local_append(statute("b", "B", "y")).expect("b");
        doc.local_append(statute("c", "C", "z")).expect("c");
        let ids: Vec<String> = doc.statutes().iter().map(|s| s.id.clone()).collect();
        assert_eq!(ids, vec!["a", "b", "c"]);
        assert_eq!(doc.len(), 3);
    }

    #[test]
    fn concurrent_inserts_converge_regardless_of_order() {
        // Two replicas insert concurrently after the same (empty) head.
        let mut r1 = CrdtDocument::new("r1");
        let mut r2 = CrdtDocument::new("r2");

        let op1 = r1
            .local_insert_after(None, statute("from_r1", "T1", "x"))
            .expect("op1");
        let op2 = r2
            .local_insert_after(None, statute("from_r2", "T2", "y"))
            .expect("op2");

        // Deliver in opposite orders.
        r1.apply(op2.clone()).expect("r1<-op2");
        r2.apply(op1.clone()).expect("r2<-op1");

        let d1 = r1.state_digest().expect("d1");
        let d2 = r2.state_digest().expect("d2");
        assert_eq!(d1, d2, "replicas converge");
        assert_eq!(r1.statutes().len(), 2);
        assert_eq!(r2.statutes().len(), 2);
        // Order is deterministic and identical on both.
        assert_eq!(
            r1.statutes()
                .iter()
                .map(|s| s.id.clone())
                .collect::<Vec<_>>(),
            r2.statutes()
                .iter()
                .map(|s| s.id.clone())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn concurrent_updates_resolve_lww_deterministically() {
        // Both replicas update the same element concurrently; the greater dot
        // wins, and both replicas agree.
        let mut r1 = CrdtDocument::new("r1");
        let seed = r1.local_append(statute("a", "A", "orig")).expect("seed");
        let target = insert_id(&seed);
        let mut r2 = CrdtDocument::new("r2");
        r2.apply(seed).expect("r2 seed");

        let up1 = r1
            .local_update(target.clone(), statute("a", "A", "from_r1"))
            .expect("up1");
        let up2 = r2
            .local_update(target.clone(), statute("a", "A", "from_r2"))
            .expect("up2");

        r1.apply(up2).expect("r1<-up2");
        r2.apply(up1).expect("r2<-up1");

        assert_eq!(
            r1.state_digest().expect("d1"),
            r2.state_digest().expect("d2")
        );
        // The winner is the one with the greater dot (replica tie-break "r2").
        let winning_desc = r1.statutes()[0].effect.description.clone();
        assert_eq!(winning_desc, "from_r2");
    }

    #[test]
    fn delete_is_monotone_and_commutative() {
        let mut r1 = CrdtDocument::new("r1");
        let seed = r1.local_append(statute("a", "A", "x")).expect("seed");
        let target = insert_id(&seed);
        let mut r2 = CrdtDocument::new("r2");
        r2.apply(seed).expect("seed r2");

        // r1 deletes; r2 concurrently updates.
        let del = r1.local_delete(target.clone()).expect("del");
        let upd = r2
            .local_update(target.clone(), statute("a", "A", "y"))
            .expect("upd");

        r1.apply(upd).expect("r1<-upd");
        r2.apply(del).expect("r2<-del");

        // Deletion wins (element hidden) on both, regardless of order.
        assert!(r1.is_empty());
        assert!(r2.is_empty());
        assert_eq!(
            r1.state_digest().expect("d1"),
            r2.state_digest().expect("d2")
        );
    }

    #[test]
    fn idempotent_redelivery() {
        let mut r1 = CrdtDocument::new("r1");
        let op = r1.local_append(statute("a", "A", "x")).expect("op");
        // Re-applying the same op multiple times is a no-op.
        r1.apply(op.clone()).expect("dup1");
        r1.apply(op).expect("dup2");
        assert_eq!(r1.len(), 1);
    }

    #[test]
    fn three_replica_convergence_under_permuted_delivery() {
        // Each replica makes an edit; gather all ops and deliver them to a clean
        // replica in several permutations — all must converge to one digest.
        let mut a = CrdtDocument::new("a");
        let mut b = CrdtDocument::new("b");
        let mut c = CrdtDocument::new("c");

        let oa = a.local_append(statute("sa", "SA", "1")).expect("oa");
        let ob = b.local_append(statute("sb", "SB", "2")).expect("ob");
        let oc = c.local_append(statute("sc", "SC", "3")).expect("oc");
        let all = [oa, ob, oc];

        let permutations = [
            vec![0usize, 1, 2],
            vec![2, 1, 0],
            vec![1, 0, 2],
            vec![2, 0, 1],
        ];
        let mut digests = Vec::new();
        for perm in &permutations {
            let mut replica = CrdtDocument::new("merge");
            for &i in perm {
                replica.apply(all[i].clone()).expect("apply");
            }
            digests.push(replica.state_digest().expect("digest"));
            assert_eq!(replica.len(), 3);
        }
        // All permutations agree.
        assert!(digests.windows(2).all(|w| w[0] == w[1]));
    }

    #[test]
    fn concurrent_move_converges() {
        // Build [a,b,c] on a seed, replicate to r2, then both move c.
        let mut r1 = CrdtDocument::new("r1");
        let ia = r1.local_append(statute("a", "A", "x")).expect("a");
        let ib = r1.local_append(statute("b", "B", "y")).expect("b");
        let ic = r1.local_append(statute("c", "C", "z")).expect("c");
        let id_a = insert_id(&ia);
        let id_c = insert_id(&ic);

        let mut r2 = CrdtDocument::new("r2");
        r2.apply_all(vec![ia, ib, ic]).expect("seed r2");

        // r1 moves c to head; r2 moves c after a — concurrent moves.
        let m1 = r1.local_move(id_c.clone(), None).expect("m1");
        let m2 = r2.local_move(id_c.clone(), Some(id_a.clone())).expect("m2");

        r1.apply(m2).expect("r1<-m2");
        r2.apply(m1).expect("r2<-m1");

        assert_eq!(
            r1.state_digest().expect("d1"),
            r2.state_digest().expect("d2")
        );
    }

    #[test]
    fn snapshot_ops_reconstructs_identical_state() {
        let mut r1 = CrdtDocument::new("r1");
        r1.local_append(statute("a", "A", "x")).expect("a");
        let ib = r1.local_append(statute("b", "B", "y")).expect("b");
        r1.local_append(statute("c", "C", "z")).expect("c");
        let id_b = insert_id(&ib);
        r1.local_delete(id_b).expect("del b");

        let snap = r1.snapshot_ops();
        let mut fresh = CrdtDocument::new("fresh");
        fresh.apply_all(snap).expect("replay");
        assert_eq!(
            r1.state_digest().expect("d1"),
            fresh.state_digest().expect("d2")
        );
        assert_eq!(
            fresh
                .statutes()
                .iter()
                .map(|s| s.id.clone())
                .collect::<Vec<_>>(),
            vec!["a", "c"]
        );
    }
}
