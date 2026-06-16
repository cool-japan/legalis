//! Normalize condition structure (roadmap v0.3.3).
//!
//! Canonicalises the boolean structure of a [`ConditionNode`] to a deterministic
//! *negation normal form* (NNF):
//!
//! 1. **Push negations inward** via De Morgan's laws and double-negation
//!    elimination, so `NOT` only ever wraps an atom. `NOT (IN_RANGE)` and
//!    `NOT (NOT_IN_RANGE)` are folded into their negated atom counterparts.
//! 2. **Flatten** nested `AND`/`OR` chains into flat operand lists.
//! 3. **Deduplicate** structurally-identical operands within a connective.
//! 4. **Order** operands deterministically by their structural key, then rebuild
//!    a left-leaning tree.
//!
//! The result is canonical and **idempotent**: `normalize(normalize(c)) ==
//! normalize(c)` for every condition. The transformation is purely structural —
//! it never distributes `AND` over `OR` (avoiding the exponential blow-up of full
//! DNF/CNF) and never invents truth constants, so it always returns a single
//! equivalent [`ConditionNode`].

use super::{RefactorKind, RefactorReport, condition_key, count_nodes};
use crate::ast::{ConditionNode, LegalDocument, StatuteNode};
use serde::{Deserialize, Serialize};

/// Per-condition statistics produced by [`normalize_condition_structure`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizeReport {
    /// Node count before normalization.
    pub nodes_before: usize,
    /// Node count after normalization.
    pub nodes_after: usize,
    /// Number of negations rewritten via De Morgan / double-negation.
    pub negations_pushed: usize,
    /// Number of duplicate operands removed from `AND`/`OR` connectives.
    pub duplicates_removed: usize,
    /// Number of nested same-operator connectives flattened.
    pub flattened: usize,
}

impl NormalizeReport {
    /// Returns true if normalization left the condition structurally unchanged.
    pub fn is_noop(&self) -> bool {
        self.negations_pushed == 0
            && self.duplicates_removed == 0
            && self.flattened == 0
            && self.nodes_before == self.nodes_after
    }
}

/// Mutable accumulator threaded through the recursion.
#[derive(Default)]
struct Stats {
    negations_pushed: usize,
    duplicates_removed: usize,
    flattened: usize,
}

/// Normalizes a single condition to canonical negation normal form, returning the
/// rewritten condition and a [`NormalizeReport`]. Deterministic and idempotent.
pub fn normalize_condition_structure(cond: &ConditionNode) -> (ConditionNode, NormalizeReport) {
    let mut stats = Stats::default();
    let nodes_before = count_nodes(cond);
    let nnf = to_nnf(cond, false, &mut stats);
    let canonical = canon(&nnf, &mut stats);
    let nodes_after = count_nodes(&canonical);
    let report = NormalizeReport {
        nodes_before,
        nodes_after,
        negations_pushed: stats.negations_pushed,
        duplicates_removed: stats.duplicates_removed,
        flattened: stats.flattened,
    };
    (canonical, report)
}

/// Pushes negations to the leaves. `negated` tracks whether the current subtree
/// sits under an odd number of `NOT`s.
///
/// Counting is deliberately *transformation-based* rather than per-`NOT`: only
/// genuine rewrites (double-negation elimination, a De Morgan distribution, or a
/// range fold) increment `negations_pushed`. A negated atom that stays as
/// `NOT atom` is already canonical and does not count — this is what makes
/// re-normalization a reported no-op (idempotence).
fn to_nnf(cond: &ConditionNode, negated: bool, stats: &mut Stats) -> ConditionNode {
    match cond {
        ConditionNode::Not(inner) => {
            // `NOT (NOT x)` cancels: count the double-negation elimination.
            if matches!(inner.as_ref(), ConditionNode::Not(_)) {
                stats.negations_pushed += 1;
            }
            to_nnf(inner, !negated, stats)
        }
        ConditionNode::And(left, right) => {
            if negated {
                // NOT (a AND b) => (NOT a) OR (NOT b)
                stats.negations_pushed += 1;
                ConditionNode::Or(
                    Box::new(to_nnf(left, true, stats)),
                    Box::new(to_nnf(right, true, stats)),
                )
            } else {
                ConditionNode::And(
                    Box::new(to_nnf(left, false, stats)),
                    Box::new(to_nnf(right, false, stats)),
                )
            }
        }
        ConditionNode::Or(left, right) => {
            if negated {
                // NOT (a OR b) => (NOT a) AND (NOT b)
                stats.negations_pushed += 1;
                ConditionNode::And(
                    Box::new(to_nnf(left, true, stats)),
                    Box::new(to_nnf(right, true, stats)),
                )
            } else {
                ConditionNode::Or(
                    Box::new(to_nnf(left, false, stats)),
                    Box::new(to_nnf(right, false, stats)),
                )
            }
        }
        atom => {
            if negated {
                // A range atom has an exact negated counterpart (a real rewrite);
                // every other atom stays as a canonical `NOT atom`.
                if matches!(
                    atom,
                    ConditionNode::InRange { .. } | ConditionNode::NotInRange { .. }
                ) {
                    stats.negations_pushed += 1;
                }
                negate_atom(atom)
            } else {
                atom.clone()
            }
        }
    }
}

/// Produces the NNF representation of `NOT atom`. Range atoms have an exact
/// negated counterpart; every other atom is wrapped in a single `NOT`.
fn negate_atom(atom: &ConditionNode) -> ConditionNode {
    match atom {
        ConditionNode::InRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => ConditionNode::NotInRange {
            field: field.clone(),
            min: min.clone(),
            max: max.clone(),
            inclusive_min: *inclusive_min,
            inclusive_max: *inclusive_max,
        },
        ConditionNode::NotInRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => ConditionNode::InRange {
            field: field.clone(),
            min: min.clone(),
            max: max.clone(),
            inclusive_min: *inclusive_min,
            inclusive_max: *inclusive_max,
        },
        other => ConditionNode::Not(Box::new(other.clone())),
    }
}

/// Flattens, deduplicates and deterministically orders an NNF condition.
fn canon(cond: &ConditionNode, stats: &mut Stats) -> ConditionNode {
    match cond {
        ConditionNode::And(_, _) => {
            let parts = gather(cond, true, stats);
            let parts = canon_parts(parts, stats);
            super::fold_and(parts).unwrap_or_else(|| cond.clone())
        }
        ConditionNode::Or(_, _) => {
            let parts = gather(cond, false, stats);
            let parts = canon_parts(parts, stats);
            super::fold_or(parts).unwrap_or_else(|| cond.clone())
        }
        // After NNF a `Not` only ever wraps an atom; nothing further to flatten.
        ConditionNode::Not(inner) => ConditionNode::Not(Box::new(canon(inner, stats))),
        atom => atom.clone(),
    }
}

/// Collects the operands of a same-operator connective, recursing through nested
/// connectives of the same kind (counting each flattening) and canonicalising
/// operands of a different kind.
fn gather(cond: &ConditionNode, is_and: bool, stats: &mut Stats) -> Vec<ConditionNode> {
    let mut out = Vec::new();
    match (cond, is_and) {
        (ConditionNode::And(left, right), true) => {
            // A nested AND inside an AND is flattened away.
            if matches!(left.as_ref(), ConditionNode::And(_, _))
                || matches!(right.as_ref(), ConditionNode::And(_, _))
            {
                stats.flattened += 1;
            }
            out.extend(gather(left, true, stats));
            out.extend(gather(right, true, stats));
        }
        (ConditionNode::Or(left, right), false) => {
            if matches!(left.as_ref(), ConditionNode::Or(_, _))
                || matches!(right.as_ref(), ConditionNode::Or(_, _))
            {
                stats.flattened += 1;
            }
            out.extend(gather(left, false, stats));
            out.extend(gather(right, false, stats));
        }
        (other, _) => out.push(canon(other, stats)),
    }
    out
}

/// Deduplicates (by structural key) and deterministically sorts a list of
/// already-canonicalised operands.
fn canon_parts(parts: Vec<ConditionNode>, stats: &mut Stats) -> Vec<ConditionNode> {
    let mut seen = std::collections::BTreeSet::new();
    let mut unique = Vec::with_capacity(parts.len());
    for part in parts {
        let key = condition_key(&part);
        if seen.insert(key) {
            unique.push(part);
        } else {
            stats.duplicates_removed += 1;
        }
    }
    unique.sort_by_cached_key(condition_key);
    unique
}

/// Normalizes every condition tree stored on a statute, returning the rewritten
/// statute and an aggregate refactor report (one change entry per condition that
/// actually changed).
pub fn normalize_statute_conditions(statute: &StatuteNode) -> (StatuteNode, RefactorReport) {
    let mut report = RefactorReport::new(RefactorKind::NormalizeCondition);
    let mut changed_fields: Vec<String> = Vec::new();

    let rewritten = super::map_statute_conditions(statute, |cond| {
        let (normalized, sub) = normalize_condition_structure(cond);
        if !sub.is_noop() && normalized != *cond {
            changed_fields.push(format!("{}→{} nodes", sub.nodes_before, sub.nodes_after));
        }
        normalized
    });

    if !changed_fields.is_empty() {
        report.record(super::RefactorChange::new(
            format!(
                "normalized {} condition(s) in statute '{}'",
                changed_fields.len(),
                statute.id
            ),
            vec![statute.id.clone()],
        ));
    }
    (rewritten, report)
}

/// Normalizes every condition across an entire document.
pub fn normalize_document_conditions(doc: &LegalDocument) -> (LegalDocument, RefactorReport) {
    let mut report = RefactorReport::new(RefactorKind::NormalizeCondition);
    let mut out = doc.clone();
    let mut new_statutes = Vec::with_capacity(doc.statutes.len());
    for statute in &doc.statutes {
        let (rewritten, sub) = normalize_statute_conditions(statute);
        for change in sub.changes {
            report.record(change);
        }
        new_statutes.push(rewritten);
    }
    out.statutes = new_statutes;
    (out, report)
}
