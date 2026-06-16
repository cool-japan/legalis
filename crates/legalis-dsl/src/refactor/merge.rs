//! Merge similar statutes refactoring (roadmap v0.3.3).
//!
//! Combines statutes that have *near-identical structure* — identical effects,
//! discretion, exceptions, metadata and dependencies, differing only in their
//! `WHEN` conditions — into a single statute. The common conjuncts shared by
//! every member are factored out and the parts that differ are OR-ed together,
//! exploiting the distributive law
//!
//! ```text
//! (common ∧ rest₁) ∨ (common ∧ rest₂) ∨ … ≡ common ∧ (rest₁ ∨ rest₂ ∨ …)
//! ```
//!
//! so the merged statute fires the same effects under exactly the same overall
//! condition as the union of the originals.

use super::{
    RefactorChange, RefactorKind, RefactorReport, condition_key, flatten_conjuncts, fold_and,
    fold_or,
};
use crate::ast::{ConditionNode, LegalDocument, StatuteNode};
use std::collections::BTreeMap;

/// Strategy for choosing the merged statute's identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeIdStrategy {
    /// Keep the first group member's id (references to it remain valid).
    KeepFirst,
    /// Join all member ids with an underscore.
    JoinIds,
}

/// Options controlling [`merge_similar_statutes`].
#[derive(Debug, Clone)]
pub struct MergeOptions {
    /// Minimum number of structurally-identical statutes required to merge.
    pub min_group_size: usize,
    /// How to name the merged statute.
    pub id_strategy: MergeIdStrategy,
}

impl Default for MergeOptions {
    fn default() -> Self {
        Self {
            min_group_size: 2,
            id_strategy: MergeIdStrategy::KeepFirst,
        }
    }
}

/// Result of [`merge_similar_statutes`].
#[derive(Debug, Clone, PartialEq)]
pub struct MergeResult {
    /// The document with merge groups collapsed.
    pub document: LegalDocument,
    /// Structured report of what merged.
    pub report: RefactorReport,
    /// Number of merge groups that were collapsed.
    pub merged_groups: usize,
}

impl MergeResult {
    /// Returns true when nothing was merged.
    pub fn is_noop(&self) -> bool {
        self.merged_groups == 0
    }
}

/// Merges structurally-similar statutes in `doc`.
pub fn merge_similar_statutes(doc: &LegalDocument, options: &MergeOptions) -> MergeResult {
    let mut report = RefactorReport::new(RefactorKind::MergeStatutes);

    // Group statute indices by a signature covering everything except id, title
    // and conditions. First-appearance order is preserved for deterministic
    // output.
    let mut group_of: BTreeMap<String, usize> = BTreeMap::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (idx, statute) in doc.statutes.iter().enumerate() {
        let sig = structural_signature(statute);
        match group_of.get(&sig) {
            Some(&gid) => groups[gid].push(idx),
            None => {
                let gid = groups.len();
                group_of.insert(sig, gid);
                groups.push(vec![idx]);
            }
        }
    }

    // Map each statute index to its group, and which index is the group leader.
    let mut leader_group: BTreeMap<usize, usize> = BTreeMap::new();
    for (gid, members) in groups.iter().enumerate() {
        if members.len() >= options.min_group_size {
            for &idx in members {
                leader_group.insert(idx, gid);
            }
        }
    }

    let mut merged_groups = 0usize;
    let mut out_statutes = Vec::with_capacity(doc.statutes.len());
    for (idx, statute) in doc.statutes.iter().enumerate() {
        match leader_group.get(&idx) {
            Some(&gid) => {
                let members = &groups[gid];
                if members.first() == Some(&idx) {
                    // Leader: emit the merged statute.
                    let member_statutes: Vec<&StatuteNode> =
                        members.iter().map(|&i| &doc.statutes[i]).collect();
                    let merged = merge_group(&member_statutes, options);
                    let member_ids: Vec<String> =
                        member_statutes.iter().map(|s| s.id.clone()).collect();
                    report.record(
                        RefactorChange::new(
                            format!("merged {} statutes into '{}'", member_ids.len(), merged.id),
                            member_ids.clone(),
                        )
                        .with_detail(format!("members: {}", member_ids.join(", "))),
                    );
                    merged_groups += 1;
                    out_statutes.push(merged);
                }
                // Non-leader members are dropped (folded into the leader).
            }
            None => out_statutes.push(statute.clone()),
        }
    }

    let mut document = doc.clone();
    document.statutes = out_statutes;

    MergeResult {
        document,
        report,
        merged_groups,
    }
}

/// Computes a signature string that is equal for two statutes iff they share the
/// same structure apart from id, title and conditions.
fn structural_signature(statute: &StatuteNode) -> String {
    let skeleton = StatuteNode {
        id: String::new(),
        title: String::new(),
        conditions: Vec::new(),
        ..statute.clone()
    };
    format!("{skeleton:?}")
}

/// Merges a homogeneous group of statutes, factoring common conjuncts.
fn merge_group(members: &[&StatuteNode], options: &MergeOptions) -> StatuteNode {
    // Base everything except id/title/conditions on the first member (they all
    // share the same skeleton by construction).
    let mut merged = members[0].clone();

    merged.id = match options.id_strategy {
        MergeIdStrategy::KeepFirst => members[0].id.clone(),
        MergeIdStrategy::JoinIds => members
            .iter()
            .map(|s| s.id.clone())
            .collect::<Vec<_>>()
            .join("_"),
    };
    merged.title = members[0].title.clone();
    merged.conditions = factor_conditions(members);
    merged
}

/// Factors the common conjuncts out of every member and combines the remainders.
fn factor_conditions(members: &[&StatuteNode]) -> Vec<ConditionNode> {
    // Each member's full conjunct list (the whole `conditions` Vec is an implicit
    // conjunction).
    let member_conjuncts: Vec<Vec<ConditionNode>> = members
        .iter()
        .map(|s| {
            let mut all = Vec::new();
            for cond in &s.conditions {
                all.extend(flatten_conjuncts(cond));
            }
            all
        })
        .collect();

    // Keys present in every member (intersection), preserving the first member's
    // order.
    let key_sets: Vec<std::collections::BTreeSet<String>> = member_conjuncts
        .iter()
        .map(|cs| cs.iter().map(condition_key).collect())
        .collect();

    let mut common: Vec<ConditionNode> = Vec::new();
    let mut seen_common = std::collections::BTreeSet::new();
    if let Some(first) = member_conjuncts.first() {
        for cond in first {
            let key = condition_key(cond);
            if seen_common.contains(&key) {
                continue;
            }
            if key_sets.iter().all(|set| set.contains(&key)) {
                seen_common.insert(key);
                common.push(cond.clone());
            }
        }
    }

    // Each member's remainder = its conjuncts minus the common keys.
    let common_keys: std::collections::BTreeSet<String> =
        common.iter().map(condition_key).collect();
    let mut rests: Vec<Vec<ConditionNode>> = Vec::new();
    let mut any_empty_rest = false;
    for conjuncts in &member_conjuncts {
        let mut rest = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for cond in conjuncts {
            let key = condition_key(cond);
            if common_keys.contains(&key) {
                continue;
            }
            if seen.insert(key) {
                rest.push(cond.clone());
            }
        }
        if rest.is_empty() {
            any_empty_rest = true;
        }
        rests.push(rest);
    }

    let mut result = common;

    // If any member's remainder is empty, the disjunction of remainders is
    // vacuously true (that member applies whenever the common part holds), so the
    // OR contributes nothing.
    if any_empty_rest {
        return result;
    }

    // Build the OR of each member's remainder conjunction, deduped + ordered.
    let mut rest_conds: Vec<ConditionNode> = Vec::new();
    let mut seen_rest = std::collections::BTreeSet::new();
    for rest in rests {
        if let Some(folded) = fold_and(rest) {
            let key = condition_key(&folded);
            if seen_rest.insert(key) {
                rest_conds.push(folded);
            }
        }
    }
    rest_conds.sort_by_cached_key(condition_key);

    if let Some(disjunction) = fold_or(rest_conds) {
        result.push(disjunction);
    }

    result
}
