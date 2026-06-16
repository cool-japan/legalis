//! Inline condition refactoring (roadmap v0.3.3).
//!
//! The inverse of [`super::extract`]: given a document containing reference
//! placeholders (`HAS <ref_key>` attributes) and the definitions they stand for,
//! substitute every placeholder back with its definition.
//!
//! Substitution runs to a fixpoint (bounded to avoid pathological inputs) so that
//! nested extractions — where one extracted definition itself references another
//! placeholder — are fully expanded.

use super::{RefactorChange, RefactorKind, RefactorReport, collect_subconditions};
use crate::ast::{ConditionNode, LegalDocument};
use std::collections::HashMap;

/// Result of an inline operation.
#[derive(Debug, Clone, PartialEq)]
pub struct InlineResult {
    /// The document with placeholders replaced by their definitions.
    pub document: LegalDocument,
    /// Structured report of what changed.
    pub report: RefactorReport,
    /// Total number of placeholder substitutions performed.
    pub substitutions: usize,
}

impl InlineResult {
    /// Returns true when nothing was inlined.
    pub fn is_noop(&self) -> bool {
        self.substitutions == 0
    }
}

/// Inlines the conditions previously produced by [`super::extract_condition`].
///
/// This is the exact inverse of extraction: `inline_condition(&result.document,
/// &result.extracted)` reproduces the original document.
pub fn inline_condition(
    doc: &LegalDocument,
    extracted: &[super::extract::ExtractedCondition],
) -> InlineResult {
    let map = super::extract::extracted_as_map(extracted);
    inline_named_conditions(doc, &map)
}

/// Inlines an arbitrary `ref_key → definition` map. Any `HAS <ref_key>`
/// attribute whose key is present in the map is replaced by the mapped
/// definition; the process repeats until no mapped placeholder remains.
pub fn inline_named_conditions(
    doc: &LegalDocument,
    definitions: &HashMap<String, ConditionNode>,
) -> InlineResult {
    let mut report = RefactorReport::new(RefactorKind::InlineCondition);

    if definitions.is_empty() {
        return InlineResult {
            document: doc.clone(),
            report,
            substitutions: 0,
        };
    }

    let mut current = doc.clone();
    let mut total = 0usize;
    let mut affected = std::collections::BTreeSet::new();

    // Fixpoint loop: an inlined definition may itself contain placeholders.
    // The number of distinct keys bounds the depth of legitimate nesting.
    let max_passes = definitions.len() + 1;
    for _ in 0..max_passes {
        let mut pass_count = 0usize;
        let mut pass_affected = Vec::new();

        let next = {
            let mut out = current.clone();
            out.statutes = current
                .statutes
                .iter()
                .map(|statute| {
                    let mut count_here = 0usize;
                    let rewritten = super::map_statute_conditions(statute, |cond| {
                        let (new_cond, n) = substitute(cond, definitions);
                        count_here += n;
                        new_cond
                    });
                    if count_here > 0 {
                        pass_affected.push(statute.id.clone());
                        pass_count += count_here;
                    }
                    rewritten
                })
                .collect();
            out
        };

        current = next;
        total += pass_count;
        for id in pass_affected {
            affected.insert(id);
        }

        if pass_count == 0 {
            break;
        }
    }

    if total > 0 {
        report.record(RefactorChange::new(
            format!(
                "inlined {} placeholder(s) from {} definition(s)",
                total,
                definitions.len()
            ),
            affected.into_iter().collect(),
        ));
    }

    InlineResult {
        document: current,
        report,
        substitutions: total,
    }
}

/// Replaces every `HAS <key>` node whose key is in `definitions` with the mapped
/// condition, returning the rewritten tree and the count of substitutions.
fn substitute(
    cond: &ConditionNode,
    definitions: &HashMap<String, ConditionNode>,
) -> (ConditionNode, usize) {
    match cond {
        ConditionNode::HasAttribute { key } => {
            if let Some(def) = definitions.get(key) {
                (def.clone(), 1)
            } else {
                (cond.clone(), 0)
            }
        }
        ConditionNode::And(left, right) => {
            let (l, lc) = substitute(left, definitions);
            let (r, rc) = substitute(right, definitions);
            (ConditionNode::And(Box::new(l), Box::new(r)), lc + rc)
        }
        ConditionNode::Or(left, right) => {
            let (l, lc) = substitute(left, definitions);
            let (r, rc) = substitute(right, definitions);
            (ConditionNode::Or(Box::new(l), Box::new(r)), lc + rc)
        }
        ConditionNode::Not(inner) => {
            let (i, c) = substitute(inner, definitions);
            (ConditionNode::Not(Box::new(i)), c)
        }
        other => (other.clone(), 0),
    }
}

/// Returns the set of reference keys still present in a document (placeholders
/// that have not yet been inlined). Useful for callers validating completeness.
pub fn pending_reference_keys(
    doc: &LegalDocument,
    candidate_keys: &std::collections::BTreeSet<String>,
) -> std::collections::BTreeSet<String> {
    let mut subs = Vec::new();
    for statute in &doc.statutes {
        for cond in &statute.conditions {
            collect_subconditions(cond, &mut subs);
        }
        for ex in &statute.exceptions {
            for cond in &ex.conditions {
                collect_subconditions(cond, &mut subs);
            }
        }
    }
    let mut found = std::collections::BTreeSet::new();
    for sub in subs {
        if let ConditionNode::HasAttribute { key } = sub
            && candidate_keys.contains(&key)
        {
            found.insert(key);
        }
    }
    found
}

/// Inlines a single named condition everywhere it is referenced (convenience
/// wrapper used by editor-style "inline this" actions).
pub fn inline_single(
    doc: &LegalDocument,
    ref_key: &str,
    definition: &ConditionNode,
) -> InlineResult {
    let mut map = HashMap::new();
    map.insert(ref_key.to_string(), definition.clone());
    inline_named_conditions(doc, &map)
}
