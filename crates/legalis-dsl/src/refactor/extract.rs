//! Extract condition refactoring (roadmap v0.3.3).
//!
//! Pulls a repeated or complex sub-condition out of the statutes that use it and
//! replaces every occurrence with a *reference placeholder* — a
//! [`ConditionNode::HasAttribute`] whose key is a freshly generated,
//! identifier-safe slug guaranteed not to collide with any existing attribute in
//! the document. The extracted definition(s) are returned alongside the rewritten
//! document so the operation can be reversed exactly by [`super::inline`].
//!
//! Two modes are supported:
//!
//! * **Targeted** — supply [`ExtractOptions::target`] (and optionally a name) to
//!   extract one specific sub-condition.
//! * **Automatic** — leave `target` unset and the most frequently occurring
//!   sub-condition that meets the `min_occurrences`/`min_complexity` thresholds is
//!   chosen (ties broken by larger size, then by structural key for determinism).

use super::{
    RefactorChange, RefactorKind, RefactorReport, collect_subconditions, condition_key,
    count_nodes, document_attribute_keys, is_atom, replace_subcondition, sanitize_ident,
};
use crate::ast::{ConditionNode, LegalDocument};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A condition that was lifted out of the document by [`extract_condition`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractedCondition {
    /// Human-friendly name for the extracted condition.
    pub name: String,
    /// The attribute key used as the reference placeholder in the document.
    pub ref_key: String,
    /// The original condition definition (what the placeholder stands for).
    pub definition: ConditionNode,
    /// How many occurrences were replaced.
    pub occurrences: usize,
}

/// Options controlling [`extract_condition`].
#[derive(Debug, Clone)]
pub struct ExtractOptions {
    /// Explicit name for the extracted condition. When `None` an automatic name
    /// (`extracted_condition`, deduplicated) is generated.
    pub name: Option<String>,
    /// A specific sub-condition to extract. When `None` the most frequent
    /// eligible sub-condition is chosen automatically.
    pub target: Option<ConditionNode>,
    /// Minimum number of occurrences for a sub-condition to be auto-extracted.
    pub min_occurrences: usize,
    /// Minimum node count for a sub-condition to be eligible (skips trivial
    /// atoms by default).
    pub min_complexity: usize,
    /// Prefix for the generated reference attribute key.
    pub ref_prefix: String,
}

impl Default for ExtractOptions {
    fn default() -> Self {
        Self {
            name: None,
            target: None,
            min_occurrences: 2,
            min_complexity: 2,
            ref_prefix: "cond_".to_string(),
        }
    }
}

impl ExtractOptions {
    /// Targets a specific sub-condition for extraction.
    pub fn target(mut self, condition: ConditionNode) -> Self {
        self.target = Some(condition);
        self
    }

    /// Sets the name for the extracted condition.
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the minimum occurrence threshold for automatic extraction.
    pub fn min_occurrences(mut self, n: usize) -> Self {
        self.min_occurrences = n;
        self
    }
}

/// Result of [`extract_condition`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractConditionResult {
    /// The document with occurrences replaced by reference placeholders.
    pub document: LegalDocument,
    /// The condition(s) that were extracted (currently always zero or one).
    pub extracted: Vec<ExtractedCondition>,
    /// Structured report of what changed.
    pub report: RefactorReport,
}

impl ExtractConditionResult {
    /// Returns true when nothing was extracted (the document is unchanged).
    pub fn is_noop(&self) -> bool {
        self.extracted.is_empty()
    }
}

/// Extracts a sub-condition from `doc` according to `options`.
///
/// On success the returned document has every occurrence of the chosen condition
/// replaced by a `HAS <ref_key>` placeholder, and `extracted` records the
/// definition. When no eligible condition is found (automatic mode) or the target
/// does not occur (targeted mode), the original document is returned unchanged
/// with an empty `extracted` list.
pub fn extract_condition(doc: &LegalDocument, options: &ExtractOptions) -> ExtractConditionResult {
    let existing = document_attribute_keys(doc);

    let target = match &options.target {
        Some(t) => t.clone(),
        None => match pick_auto_target(doc, options) {
            Some(t) => t,
            None => {
                return ExtractConditionResult {
                    document: doc.clone(),
                    extracted: Vec::new(),
                    report: RefactorReport::new(RefactorKind::ExtractCondition),
                };
            }
        },
    };

    // Choose a collision-free reference key.
    let base_name = options
        .name
        .clone()
        .unwrap_or_else(|| "extracted_condition".to_string());
    let slug = sanitize_ident(&base_name);
    let ref_key = unique_ref_key(&options.ref_prefix, &slug, &existing);

    let placeholder = ConditionNode::HasAttribute {
        key: ref_key.clone(),
    };

    // Replace occurrences across the whole document, counting them and noting
    // which statutes were touched.
    let mut total = 0usize;
    let mut affected: Vec<String> = Vec::new();
    let new_doc = {
        let mut out = doc.clone();
        out.statutes = doc
            .statutes
            .iter()
            .map(|statute| {
                let mut count_here = 0usize;
                let rewritten = super::map_statute_conditions(statute, |cond| {
                    let (new_cond, n) = replace_subcondition(cond, &target, &placeholder);
                    count_here += n;
                    new_cond
                });
                if count_here > 0 {
                    affected.push(statute.id.clone());
                    total += count_here;
                }
                rewritten
            })
            .collect();
        out
    };

    let mut report = RefactorReport::new(RefactorKind::ExtractCondition);
    if total == 0 {
        return ExtractConditionResult {
            document: doc.clone(),
            extracted: Vec::new(),
            report,
        };
    }

    report.record(
        RefactorChange::new(
            format!(
                "extracted condition into '{}' ({} occurrence(s))",
                ref_key, total
            ),
            affected,
        )
        .with_detail(condition_key(&target)),
    );

    ExtractConditionResult {
        document: new_doc,
        extracted: vec![ExtractedCondition {
            name: base_name,
            ref_key,
            definition: target,
            occurrences: total,
        }],
        report,
    }
}

/// Generates a reference key of the form `<prefix><slug>` that is not already an
/// attribute in the document, appending a numeric suffix on collision.
fn unique_ref_key(
    prefix: &str,
    slug: &str,
    existing: &std::collections::BTreeSet<String>,
) -> String {
    let candidate = format!("{prefix}{slug}");
    if !existing.contains(&candidate) {
        return candidate;
    }
    let mut n = 2;
    loop {
        let candidate = format!("{prefix}{slug}_{n}");
        if !existing.contains(&candidate) {
            return candidate;
        }
        n += 1;
    }
}

/// Picks the best sub-condition to auto-extract: the most frequent eligible
/// sub-condition meeting the thresholds, ties broken by larger size then key.
fn pick_auto_target(doc: &LegalDocument, options: &ExtractOptions) -> Option<ConditionNode> {
    // Count occurrences of every eligible sub-condition across the document.
    let mut counts: BTreeMap<String, (usize, ConditionNode)> = BTreeMap::new();
    let mut all_subs = Vec::new();
    for statute in &doc.statutes {
        for cond in &statute.conditions {
            collect_subconditions(cond, &mut all_subs);
        }
        for ex in &statute.exceptions {
            for cond in &ex.conditions {
                collect_subconditions(cond, &mut all_subs);
            }
        }
    }

    for sub in all_subs {
        // Skip trivial atoms and placeholders we might extract into.
        if is_atom(&sub) && count_nodes(&sub) < options.min_complexity {
            continue;
        }
        if count_nodes(&sub) < options.min_complexity {
            continue;
        }
        let key = condition_key(&sub);
        let entry = counts.entry(key).or_insert((0, sub.clone()));
        entry.0 += 1;
    }

    counts
        .into_iter()
        .filter(|(_, (count, _))| *count >= options.min_occurrences)
        .max_by(|(ka, (ca, na)), (kb, (cb, nb))| {
            ca.cmp(cb)
                .then_with(|| count_nodes(na).cmp(&count_nodes(nb)))
                .then_with(|| ka.cmp(kb))
        })
        .map(|(_, (_, node))| node)
}

/// Builds the `ref_key → definition` substitution map consumed by
/// [`super::inline::inline_condition`] to reverse the extraction.
pub(crate) fn extracted_as_map(
    extracted: &[ExtractedCondition],
) -> std::collections::HashMap<String, ConditionNode> {
    extracted
        .iter()
        .map(|e| (e.ref_key.clone(), e.definition.clone()))
        .collect()
}
