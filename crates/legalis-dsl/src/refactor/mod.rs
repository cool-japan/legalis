//! Automated AST refactorings for the legal DSL (roadmap v0.3.3).
//!
//! Every refactoring in this module is a pure AST → AST transformation: it takes
//! an immutable [`crate::ast::LegalDocument`] / [`crate::ast::StatuteNode`] /
//! [`crate::ast::ConditionNode`] by reference and returns a freshly-allocated,
//! transformed value together with a structured [`RefactorReport`] describing
//! exactly what changed. Nothing mutates its input and no global state is
//! touched, so the refactorings compose freely and are trivially testable.
//!
//! The submodules are:
//!
//! * [`extract`] — pull a repeated/complex sub-condition into a named, reusable
//!   condition (replacing each occurrence with a reference placeholder).
//! * [`inline`] — the inverse of [`extract`]: substitute a named condition back
//!   into its uses.
//! * [`merge`] — combine statutes with near-identical structure, factoring the
//!   common conditions and OR-ing the parts that differ.
//! * [`split`] — decompose an over-large statute into coherent smaller statutes.
//! * [`normalize`] — canonicalise the boolean structure of a condition to a
//!   deterministic negation normal form (flatten, push negations, dedupe, sort).
//!
//! The `extract`/`inline` pair round-trips: `inline(extract(doc)) == doc` for any
//! document (the reference key chosen by [`extract`] is guaranteed not to clash
//! with an existing attribute, so the substitution is exact and reversible).

use crate::ast::{ConditionNode, LegalDocument, StatuteNode};
use serde::{Deserialize, Serialize};

pub mod extract;
pub mod inline;
pub mod merge;
pub mod normalize;
pub mod split;

#[cfg(test)]
mod tests;

pub use extract::{ExtractOptions, ExtractedCondition, extract_condition};
pub use inline::{InlineResult, inline_condition, inline_named_conditions};
pub use merge::{MergeOptions, MergeResult, merge_similar_statutes};
pub use normalize::{
    NormalizeReport, normalize_condition_structure, normalize_document_conditions,
    normalize_statute_conditions,
};
pub use split::{SplitOptions, SplitResult, split_complex_statute};

/// Identifies which refactoring produced a [`RefactorReport`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefactorKind {
    /// [`extract::extract_condition`].
    ExtractCondition,
    /// [`inline::inline_condition`].
    InlineCondition,
    /// [`merge::merge_similar_statutes`].
    MergeStatutes,
    /// [`split::split_complex_statute`].
    SplitStatute,
    /// [`normalize::normalize_condition_structure`].
    NormalizeCondition,
}

impl RefactorKind {
    /// A short human-readable label for this refactoring kind.
    pub fn label(self) -> &'static str {
        match self {
            Self::ExtractCondition => "extract-condition",
            Self::InlineCondition => "inline-condition",
            Self::MergeStatutes => "merge-statutes",
            Self::SplitStatute => "split-statute",
            Self::NormalizeCondition => "normalize-condition",
        }
    }
}

/// A single change recorded by a refactoring.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorChange {
    /// Human-readable summary of the change.
    pub summary: String,
    /// Statute IDs (or other identifiers) affected by this change.
    pub affected: Vec<String>,
    /// Optional extended detail.
    pub detail: Option<String>,
}

impl RefactorChange {
    /// Creates a change with a summary and the affected identifiers.
    pub fn new(summary: impl Into<String>, affected: Vec<String>) -> Self {
        Self {
            summary: summary.into(),
            affected,
            detail: None,
        }
    }

    /// Attaches extended detail to this change.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

/// Structured report describing the changes a refactoring applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorReport {
    /// Which refactoring produced this report.
    pub kind: RefactorKind,
    /// The individual changes, in application order.
    pub changes: Vec<RefactorChange>,
}

impl RefactorReport {
    /// Creates an empty report for the given refactoring kind.
    pub fn new(kind: RefactorKind) -> Self {
        Self {
            kind,
            changes: Vec::new(),
        }
    }

    /// Records a change.
    pub fn record(&mut self, change: RefactorChange) {
        self.changes.push(change);
    }

    /// Returns true if the refactoring changed nothing.
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Returns the number of recorded changes.
    pub fn len(&self) -> usize {
        self.changes.len()
    }

    /// Returns a one-line textual summary of all changes.
    pub fn summary(&self) -> String {
        if self.changes.is_empty() {
            return format!("{}: no changes", self.kind.label());
        }
        let parts: Vec<String> = self.changes.iter().map(|c| c.summary.clone()).collect();
        format!("{}: {}", self.kind.label(), parts.join("; "))
    }
}

// ---------------------------------------------------------------------------
// Shared AST utilities used across the refactorings.
// ---------------------------------------------------------------------------

/// Returns a stable structural key for a condition (used for equality grouping,
/// deduplication and deterministic ordering). Two conditions share a key iff they
/// are structurally equal, matching the convention already used by
/// [`crate::ast::transform::remove_duplicate_conditions`].
pub(crate) fn condition_key(cond: &ConditionNode) -> String {
    format!("{cond:?}")
}

/// Counts the total number of nodes in a condition tree (atoms count as one,
/// each `AND`/`OR`/`NOT` adds one for itself plus its children).
pub(crate) fn count_nodes(cond: &ConditionNode) -> usize {
    match cond {
        ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
            1 + count_nodes(left) + count_nodes(right)
        }
        ConditionNode::Not(inner) => 1 + count_nodes(inner),
        _ => 1,
    }
}

/// Returns true when a condition is a single atom (no boolean connectives).
pub(crate) fn is_atom(cond: &ConditionNode) -> bool {
    !matches!(
        cond,
        ConditionNode::And(_, _) | ConditionNode::Or(_, _) | ConditionNode::Not(_)
    )
}

/// Collects every distinct sub-condition (including the root) into `out`,
/// largest-first is *not* guaranteed; callers that need ordering should sort.
pub(crate) fn collect_subconditions(cond: &ConditionNode, out: &mut Vec<ConditionNode>) {
    out.push(cond.clone());
    match cond {
        ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
            collect_subconditions(left, out);
            collect_subconditions(right, out);
        }
        ConditionNode::Not(inner) => collect_subconditions(inner, out),
        _ => {}
    }
}

/// Replaces every maximal occurrence of `target` within `cond` by `replacement`,
/// returning the rewritten tree and the number of replacements performed. A
/// matched node is replaced wholesale (the walk does not descend into it again).
pub(crate) fn replace_subcondition(
    cond: &ConditionNode,
    target: &ConditionNode,
    replacement: &ConditionNode,
) -> (ConditionNode, usize) {
    if cond == target {
        return (replacement.clone(), 1);
    }
    match cond {
        ConditionNode::And(left, right) => {
            let (l, lc) = replace_subcondition(left, target, replacement);
            let (r, rc) = replace_subcondition(right, target, replacement);
            (ConditionNode::And(Box::new(l), Box::new(r)), lc + rc)
        }
        ConditionNode::Or(left, right) => {
            let (l, lc) = replace_subcondition(left, target, replacement);
            let (r, rc) = replace_subcondition(right, target, replacement);
            (ConditionNode::Or(Box::new(l), Box::new(r)), lc + rc)
        }
        ConditionNode::Not(inner) => {
            let (i, c) = replace_subcondition(inner, target, replacement);
            (ConditionNode::Not(Box::new(i)), c)
        }
        other => (other.clone(), 0),
    }
}

/// Flattens a left/right-nested `AND` chain into its conjuncts. A non-`AND`
/// condition yields a single-element vector.
pub(crate) fn flatten_conjuncts(cond: &ConditionNode) -> Vec<ConditionNode> {
    let mut out = Vec::new();
    fn go(c: &ConditionNode, out: &mut Vec<ConditionNode>) {
        match c {
            ConditionNode::And(left, right) => {
                go(left, out);
                go(right, out);
            }
            other => out.push(other.clone()),
        }
    }
    go(cond, &mut out);
    out
}

/// Flattens a left/right-nested `OR` chain into its disjuncts.
pub(crate) fn flatten_disjuncts(cond: &ConditionNode) -> Vec<ConditionNode> {
    let mut out = Vec::new();
    fn go(c: &ConditionNode, out: &mut Vec<ConditionNode>) {
        match c {
            ConditionNode::Or(left, right) => {
                go(left, out);
                go(right, out);
            }
            other => out.push(other.clone()),
        }
    }
    go(cond, &mut out);
    out
}

/// Folds a list of conjuncts back into a left-leaning `AND` tree. Returns `None`
/// for an empty list (a conjunction of nothing is vacuously true and cannot be
/// represented by a single [`ConditionNode`]).
pub(crate) fn fold_and(parts: Vec<ConditionNode>) -> Option<ConditionNode> {
    parts
        .into_iter()
        .reduce(|acc, item| ConditionNode::And(Box::new(acc), Box::new(item)))
}

/// Folds a list of disjuncts back into a left-leaning `OR` tree.
pub(crate) fn fold_or(parts: Vec<ConditionNode>) -> Option<ConditionNode> {
    parts
        .into_iter()
        .reduce(|acc, item| ConditionNode::Or(Box::new(acc), Box::new(item)))
}

/// Turns an arbitrary string into an identifier-safe slug (lowercase ASCII
/// alphanumerics and underscores), suitable for use as an attribute key that
/// round-trips through the tokenizer and pretty-printer.
pub(crate) fn sanitize_ident(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if ch == '_' || ch == '-' || ch == ' ' {
            out.push('_');
        }
    }
    let trimmed = out.trim_matches('_').to_string();
    if trimmed.is_empty() {
        "condition".to_string()
    } else {
        trimmed
    }
}

/// Visits every condition tree stored anywhere in a statute (preconditions,
/// exception carve-outs, delegate guards, scope guards and constraint
/// invariants), applying `f` and collecting the rewritten statute.
pub(crate) fn map_statute_conditions<F>(statute: &StatuteNode, mut f: F) -> StatuteNode
where
    F: FnMut(&ConditionNode) -> ConditionNode,
{
    let mut out = statute.clone();
    out.conditions = statute.conditions.iter().map(&mut f).collect();
    out.exceptions = statute
        .exceptions
        .iter()
        .map(|ex| {
            let mut ex = ex.clone();
            ex.conditions = ex.conditions.iter().map(&mut f).collect();
            ex
        })
        .collect();
    out.delegates = statute
        .delegates
        .iter()
        .map(|d| {
            let mut d = d.clone();
            d.conditions = d.conditions.iter().map(&mut f).collect();
            d
        })
        .collect();
    out.scope = statute.scope.as_ref().map(|s| {
        let mut s = s.clone();
        s.conditions = s.conditions.iter().map(&mut f).collect();
        s
    });
    out.constraints = statute
        .constraints
        .iter()
        .map(|c| {
            let mut c = c.clone();
            c.condition = f(&c.condition);
            c
        })
        .collect();
    out
}

/// Collects every attribute key referenced via `HAS`/bare-identifier conditions
/// across an entire document. Used by [`extract`] to avoid choosing a reference
/// key that would clash with an existing attribute (which would make the
/// extraction non-reversible).
pub(crate) fn document_attribute_keys(doc: &LegalDocument) -> std::collections::BTreeSet<String> {
    let mut keys = std::collections::BTreeSet::new();
    let mut conds = Vec::new();
    for statute in &doc.statutes {
        for cond in &statute.conditions {
            collect_subconditions(cond, &mut conds);
        }
        for ex in &statute.exceptions {
            for cond in &ex.conditions {
                collect_subconditions(cond, &mut conds);
            }
        }
    }
    for cond in &conds {
        if let ConditionNode::HasAttribute { key } = cond {
            keys.insert(key.clone());
        }
    }
    keys
}
