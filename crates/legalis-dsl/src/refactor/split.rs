//! Split complex statute refactoring (roadmap v0.3.3).
//!
//! Decomposes an over-large statute into a set of coherent smaller statutes that,
//! taken together, are semantically equivalent to the original. Two
//! complementary, semantics-preserving decompositions are applied:
//!
//! 1. **Per-effect split** — a statute that grants/revokes/obligates several
//!    effects under the same guard becomes one statute per effect (the union of
//!    the effects under the shared condition is unchanged).
//! 2. **Per-disjunct split** — a guard of the form `C ∧ (A ∨ B ∨ …)` becomes one
//!    statute per disjunct (`C ∧ A`, `C ∧ B`, …), exploiting distributivity so
//!    the union of the cases reproduces the original guard.
//!
//! Splitting only happens when it genuinely reduces complexity; otherwise the
//! original statute is returned unchanged with an empty report.

use super::{RefactorChange, RefactorKind, RefactorReport, flatten_disjuncts, sanitize_ident};
use crate::ast::{ConditionNode, StatuteNode};
use std::collections::BTreeSet;

/// Options controlling [`split_complex_statute`].
#[derive(Debug, Clone)]
pub struct SplitOptions {
    /// Maximum number of effects a statute may keep before a per-effect split is
    /// triggered.
    pub max_effects: usize,
    /// Whether to split a top-level `OR` guard into one statute per disjunct.
    pub split_or_disjunctions: bool,
    /// Minimum number of disjuncts required for a per-disjunct split.
    pub min_disjuncts: usize,
}

impl Default for SplitOptions {
    fn default() -> Self {
        Self {
            max_effects: 1,
            split_or_disjunctions: true,
            min_disjuncts: 2,
        }
    }
}

/// Result of [`split_complex_statute`].
#[derive(Debug, Clone, PartialEq)]
pub struct SplitResult {
    /// The resulting statute(s). Contains a single element (the original clone)
    /// when no split was warranted.
    pub statutes: Vec<StatuteNode>,
    /// Structured report of the decomposition.
    pub report: RefactorReport,
}

impl SplitResult {
    /// Returns true when the statute was left as-is.
    pub fn is_noop(&self) -> bool {
        self.statutes.len() <= 1
    }
}

/// Splits `statute` into coherent smaller statutes per `options`.
pub fn split_complex_statute(statute: &StatuteNode, options: &SplitOptions) -> SplitResult {
    let mut report = RefactorReport::new(RefactorKind::SplitStatute);

    // Phase 1 — per-effect decomposition.
    let phase1 = split_by_effect(statute, options);
    let effect_split = phase1.len() > 1;

    // Phase 2 — per-disjunct decomposition of each phase-1 statute.
    let mut phase2: Vec<StatuteNode> = Vec::new();
    let mut or_split = false;
    for s in &phase1 {
        if options.split_or_disjunctions {
            let parts = split_by_disjunction(s, options);
            if parts.len() > 1 {
                or_split = true;
            }
            phase2.extend(parts);
        } else {
            phase2.push(s.clone());
        }
    }

    if phase2.len() <= 1 {
        // Nothing was decomposed.
        return SplitResult {
            statutes: vec![statute.clone()],
            report,
        };
    }

    // Assign deterministic, unique, identifier-safe ids.
    let assigned = assign_unique_ids(&statute.id, phase2);

    let mut detail = Vec::new();
    if effect_split {
        detail.push(format!("by-effect ×{}", statute.effects.len()));
    }
    if or_split {
        detail.push("by-disjunction".to_string());
    }

    report.record(
        RefactorChange::new(
            format!(
                "split statute '{}' into {} statutes",
                statute.id,
                assigned.len()
            ),
            assigned.iter().map(|s| s.id.clone()).collect(),
        )
        .with_detail(detail.join(", ")),
    );

    SplitResult {
        statutes: assigned,
        report,
    }
}

/// Produces one statute per effect when the statute has more effects than
/// allowed; otherwise returns a single clone.
fn split_by_effect(statute: &StatuteNode, options: &SplitOptions) -> Vec<StatuteNode> {
    if statute.effects.len() <= options.max_effects || statute.effects.len() <= 1 {
        return vec![statute.clone()];
    }
    statute
        .effects
        .iter()
        .map(|effect| {
            let mut s = statute.clone();
            s.effects = vec![effect.clone()];
            s
        })
        .collect()
}

/// Splits the first top-level `OR` guard into one statute per disjunct, keeping
/// every other condition entry shared (distributivity). Returns a single clone
/// when no eligible `OR` guard is present.
fn split_by_disjunction(statute: &StatuteNode, options: &SplitOptions) -> Vec<StatuteNode> {
    let target_index = statute.conditions.iter().position(|cond| {
        matches!(cond, ConditionNode::Or(_, _))
            && flatten_disjuncts(cond).len() >= options.min_disjuncts
    });

    let Some(index) = target_index else {
        return vec![statute.clone()];
    };

    let disjuncts = flatten_disjuncts(&statute.conditions[index]);
    disjuncts
        .into_iter()
        .map(|disjunct| {
            let mut s = statute.clone();
            s.conditions[index] = disjunct;
            s
        })
        .collect()
}

/// Gives every produced statute a unique id derived from the base id, keeping the
/// first one's id stable so existing references survive.
fn assign_unique_ids(base: &str, statutes: Vec<StatuteNode>) -> Vec<StatuteNode> {
    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut out = Vec::with_capacity(statutes.len());
    for (i, mut statute) in statutes.into_iter().enumerate() {
        let candidate = if i == 0 {
            base.to_string()
        } else {
            // Derive a readable suffix from the (single) effect type when
            // available, falling back to a positional `partN`.
            statute
                .effects
                .first()
                .map(|e| sanitize_ident(&e.effect_type))
                .filter(|s| !s.is_empty())
                .map(|s| format!("{base}_{s}_{i}"))
                .unwrap_or_else(|| format!("{base}_part{i}"))
        };
        let unique = dedup_id(candidate, &mut used);
        statute.id = unique;
        out.push(statute);
    }
    out
}

/// Ensures `candidate` is unique within `used`, appending a numeric suffix if
/// necessary, and records the chosen id.
fn dedup_id(candidate: String, used: &mut BTreeSet<String>) -> String {
    if used.insert(candidate.clone()) {
        return candidate;
    }
    let mut n = 2;
    loop {
        let alt = format!("{candidate}_{n}");
        if used.insert(alt.clone()) {
            return alt;
        }
        n += 1;
    }
}
