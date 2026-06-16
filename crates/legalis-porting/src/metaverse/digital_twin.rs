//! Digital-twin legal systems.
//!
//! A [`LegalDigitalTwin`] is a virtual mirror of a *physical* jurisdiction's rule
//! set. Like an engineering digital twin, it holds a synchronised copy of each
//! source rule together with a [`SyncState`] recording the version it was last
//! synced from and a content fingerprint of that version. When the physical
//! original changes, the twin can detect the drift ([`DivergenceKind`]) per rule
//! and produce a [`TwinSyncPlan`] describing the re-synchronisation needed to
//! bring the mirror back in line.
//!
//! The twin is deliberately conservative: detecting drift never mutates the
//! mirror. Mutation only happens through [`LegalDigitalTwin::apply_sync_plan`],
//! so an operator can review divergence before accepting it — the legal analogue
//! of reviewing a diff before merging.

use super::{current_timestamp, sha256_parts};
use crate::PortingError;
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type TwinResult<T> = Result<T, PortingError>;

/// A fingerprint of a statute's legally significant content.
///
/// Two statutes with the same id, version, effect description and precondition
/// count produce the same fingerprint; a change to any of those changes it. This
/// is intentionally coarse — it tracks *legal* drift, not byte-level edits to,
/// say, a comment.
fn fingerprint(statute: &Statute) -> String {
    sha256_parts(&[
        statute.id.as_bytes(),
        &statute.version.to_le_bytes(),
        statute.effect.description.as_bytes(),
        &(statute.preconditions.len() as u64).to_le_bytes(),
        &(statute.exceptions.len() as u64).to_le_bytes(),
    ])
}

/// The synchronisation status of a single mirrored rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncState {
    /// Mirror matches the physical original at the recorded version.
    InSync,
    /// The physical original changed since the last sync (mirror is stale).
    Drifted,
    /// The rule exists only in the physical original (never mirrored).
    MissingInTwin,
    /// The rule exists only in the mirror (removed upstream).
    OrphanInTwin,
}

/// A rule held in the twin: a snapshot of the source statute plus sync metadata.
///
/// `PartialEq` is intentionally not derived because [`legalis_core::Statute`]
/// does not implement it; compare via serde where structural equality is needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirroredRule {
    /// Snapshot of the source statute at last sync.
    pub statute: Statute,
    /// Source statute version this snapshot was taken from.
    pub synced_version: u32,
    /// Fingerprint of the snapshot.
    pub fingerprint: String,
    /// UNIX timestamp (seconds) of the last sync.
    pub synced_at: u64,
}

impl MirroredRule {
    /// Creates a mirrored rule from a source statute (treated as in-sync).
    fn snapshot(statute: &Statute) -> Self {
        Self {
            statute: statute.clone(),
            synced_version: statute.version,
            fingerprint: fingerprint(statute),
            synced_at: current_timestamp(),
        }
    }
}

/// The nature of a divergence between the physical original and the mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivergenceKind {
    /// Same id, but the physical content fingerprint changed.
    ContentChanged,
    /// Physical version is newer than the mirrored version.
    VersionAdvanced,
    /// Present physically, absent from the mirror.
    AddedUpstream,
    /// Absent physically, still present in the mirror.
    RemovedUpstream,
}

/// One detected divergence for a single rule id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivergenceReport {
    /// The rule id that diverged.
    pub rule_id: String,
    /// What kind of divergence was detected.
    pub kind: DivergenceKind,
    /// Mirror's version (if the rule exists in the mirror).
    pub twin_version: Option<u32>,
    /// Physical original's version (if the rule exists physically).
    pub physical_version: Option<u32>,
    /// Human-readable explanation.
    pub detail: String,
}

/// What happened when a sync plan was applied to one rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncOutcome {
    /// Mirror snapshot was refreshed from the physical original.
    Updated,
    /// A new rule was added to the mirror.
    Added,
    /// A rule was removed from the mirror.
    Removed,
}

/// A reviewable plan to bring the mirror back in sync, one action per rule.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwinSyncPlan {
    /// Rule ids whose mirror snapshot should be refreshed.
    pub to_update: Vec<String>,
    /// Rule ids that should be added to the mirror.
    pub to_add: Vec<String>,
    /// Rule ids that should be removed from the mirror.
    pub to_remove: Vec<String>,
}

impl TwinSyncPlan {
    /// Whether the plan has no actions (mirror already in sync).
    pub fn is_empty(&self) -> bool {
        self.to_update.is_empty() && self.to_add.is_empty() && self.to_remove.is_empty()
    }

    /// Total number of actions in the plan.
    pub fn action_count(&self) -> usize {
        self.to_update.len() + self.to_add.len() + self.to_remove.len()
    }
}

/// A virtual legal mirror of a physical jurisdiction's rule set.
///
/// `PartialEq` is intentionally not derived because the mirrored
/// [`legalis_core::Statute`] snapshots do not implement it; compare via serde
/// where structural equality is needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalDigitalTwin {
    /// Identifier of the physical jurisdiction being mirrored.
    pub physical_jurisdiction: String,
    /// Identifier of the virtual twin.
    pub twin_id: String,
    /// Mirrored rules keyed by source statute id.
    rules: BTreeMap<String, MirroredRule>,
}

impl LegalDigitalTwin {
    /// Creates an empty twin for a physical jurisdiction.
    pub fn new(physical_jurisdiction: impl Into<String>, twin_id: impl Into<String>) -> Self {
        Self {
            physical_jurisdiction: physical_jurisdiction.into(),
            twin_id: twin_id.into(),
            rules: BTreeMap::new(),
        }
    }

    /// Builds a twin from an initial physical rule set (all mirrored in-sync).
    pub fn from_physical(
        physical_jurisdiction: impl Into<String>,
        twin_id: impl Into<String>,
        physical_rules: &[Statute],
    ) -> Self {
        let mut twin = Self::new(physical_jurisdiction, twin_id);
        for rule in physical_rules {
            twin.rules
                .insert(rule.id.clone(), MirroredRule::snapshot(rule));
        }
        twin
    }

    /// Number of rules currently mirrored.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Whether the twin mirrors no rules.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Looks up a mirrored rule by id.
    pub fn mirrored_rule(&self, id: &str) -> Option<&MirroredRule> {
        self.rules.get(id)
    }

    /// The sync state of a single rule id against a current physical rule set.
    pub fn sync_state(&self, id: &str, physical_rules: &[Statute]) -> SyncState {
        let physical = physical_rules.iter().find(|s| s.id == id);
        match (self.rules.get(id), physical) {
            (Some(mirror), Some(phys)) => {
                if phys.version > mirror.synced_version || fingerprint(phys) != mirror.fingerprint {
                    SyncState::Drifted
                } else {
                    SyncState::InSync
                }
            }
            (None, Some(_)) => SyncState::MissingInTwin,
            (Some(_), None) => SyncState::OrphanInTwin,
            (None, None) => SyncState::InSync,
        }
    }

    /// Detects every divergence between the mirror and a current physical rule
    /// set. The report is deterministic (sorted by rule id).
    pub fn detect_divergence(&self, physical_rules: &[Statute]) -> Vec<DivergenceReport> {
        let physical: BTreeMap<&str, &Statute> =
            physical_rules.iter().map(|s| (s.id.as_str(), s)).collect();
        let mut reports = Vec::new();

        // Rules present physically: changed, advanced, or missing in twin.
        for (id, phys) in &physical {
            match self.rules.get(*id) {
                Some(mirror) => {
                    if phys.version > mirror.synced_version {
                        reports.push(DivergenceReport {
                            rule_id: (*id).to_string(),
                            kind: DivergenceKind::VersionAdvanced,
                            twin_version: Some(mirror.synced_version),
                            physical_version: Some(phys.version),
                            detail: format!(
                                "physical version {} > mirrored version {}",
                                phys.version, mirror.synced_version
                            ),
                        });
                    } else if fingerprint(phys) != mirror.fingerprint {
                        reports.push(DivergenceReport {
                            rule_id: (*id).to_string(),
                            kind: DivergenceKind::ContentChanged,
                            twin_version: Some(mirror.synced_version),
                            physical_version: Some(phys.version),
                            detail: "physical content changed without a version bump".to_string(),
                        });
                    }
                }
                None => reports.push(DivergenceReport {
                    rule_id: (*id).to_string(),
                    kind: DivergenceKind::AddedUpstream,
                    twin_version: None,
                    physical_version: Some(phys.version),
                    detail: "rule added in the physical jurisdiction".to_string(),
                }),
            }
        }

        // Rules present only in the mirror: removed upstream.
        for (id, mirror) in &self.rules {
            if !physical.contains_key(id.as_str()) {
                reports.push(DivergenceReport {
                    rule_id: id.clone(),
                    kind: DivergenceKind::RemovedUpstream,
                    twin_version: Some(mirror.synced_version),
                    physical_version: None,
                    detail: "rule removed in the physical jurisdiction".to_string(),
                });
            }
        }

        reports.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
        reports
    }

    /// Whether the mirror is fully in sync with a current physical rule set.
    pub fn is_in_sync(&self, physical_rules: &[Statute]) -> bool {
        self.detect_divergence(physical_rules).is_empty()
    }

    /// Derives a re-synchronisation plan from a current physical rule set.
    pub fn plan_sync(&self, physical_rules: &[Statute]) -> TwinSyncPlan {
        let mut plan = TwinSyncPlan::default();
        for report in self.detect_divergence(physical_rules) {
            match report.kind {
                DivergenceKind::ContentChanged | DivergenceKind::VersionAdvanced => {
                    plan.to_update.push(report.rule_id)
                }
                DivergenceKind::AddedUpstream => plan.to_add.push(report.rule_id),
                DivergenceKind::RemovedUpstream => plan.to_remove.push(report.rule_id),
            }
        }
        plan
    }

    /// Applies a sync plan, mutating the mirror to match `physical_rules`.
    ///
    /// Returns one [`SyncOutcome`] per applied action, in (rule id) order.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if an `update`/`add` action names a
    /// rule absent from `physical_rules` (the plan and the supplied physical set
    /// disagree).
    pub fn apply_sync_plan(
        &mut self,
        plan: &TwinSyncPlan,
        physical_rules: &[Statute],
    ) -> TwinResult<Vec<(String, SyncOutcome)>> {
        let physical: BTreeMap<&str, &Statute> =
            physical_rules.iter().map(|s| (s.id.as_str(), s)).collect();
        let mut outcomes = Vec::new();

        for id in plan.to_add.iter().chain(plan.to_update.iter()) {
            let phys = physical.get(id.as_str()).ok_or_else(|| {
                PortingError::InvalidInput(format!(
                    "digital twin '{}': plan references rule '{id}' absent from physical set",
                    self.twin_id
                ))
            })?;
            let outcome = if self.rules.contains_key(id) {
                SyncOutcome::Updated
            } else {
                SyncOutcome::Added
            };
            self.rules.insert(id.clone(), MirroredRule::snapshot(phys));
            outcomes.push((id.clone(), outcome));
        }

        for id in &plan.to_remove {
            if self.rules.remove(id).is_some() {
                outcomes.push((id.clone(), SyncOutcome::Removed));
            }
        }

        outcomes.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(outcomes)
    }

    /// Convenience: detect, plan and apply in one step, returning the outcomes.
    pub fn resync(&mut self, physical_rules: &[Statute]) -> TwinResult<Vec<(String, SyncOutcome)>> {
        let plan = self.plan_sync(physical_rules);
        self.apply_sync_plan(&plan, physical_rules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn rule(id: &str, version: u32, effect_text: &str) -> Statute {
        Statute::new(id, id, Effect::new(EffectType::Grant, effect_text)).with_version(version)
    }

    fn base_set() -> Vec<Statute> {
        vec![
            rule("r1", 1, "Right one"),
            rule("r2", 1, "Right two"),
            rule("r3", 1, "Right three"),
        ]
    }

    #[test]
    fn test_from_physical_starts_in_sync() {
        let physical = base_set();
        let twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        assert_eq!(twin.rule_count(), 3);
        assert!(twin.is_in_sync(&physical));
        assert!(twin.plan_sync(&physical).is_empty());
    }

    #[test]
    fn test_empty_twin() {
        let twin = LegalDigitalTwin::new("US", "twin-us");
        assert!(twin.is_empty());
        assert_eq!(twin.rule_count(), 0);
    }

    #[test]
    fn test_detect_version_advance() {
        let physical = base_set();
        let twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let mut updated = physical.clone();
        updated[0].version = 2;
        let reports = twin.detect_divergence(&updated);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].rule_id, "r1");
        assert_eq!(reports[0].kind, DivergenceKind::VersionAdvanced);
        assert_eq!(twin.sync_state("r1", &updated), SyncState::Drifted);
    }

    #[test]
    fn test_detect_content_change_without_version_bump() {
        let physical = base_set();
        let twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let mut updated = physical.clone();
        // Same version, but the effect text (legal content) changed.
        updated[1].effect = Effect::new(EffectType::Grant, "Right two amended");
        let reports = twin.detect_divergence(&updated);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].kind, DivergenceKind::ContentChanged);
    }

    #[test]
    fn test_detect_precondition_change() {
        let physical = base_set();
        let twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let mut updated = physical.clone();
        updated[2] = updated[2].clone().with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });
        let reports = twin.detect_divergence(&updated);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].rule_id, "r3");
        assert_eq!(reports[0].kind, DivergenceKind::ContentChanged);
    }

    #[test]
    fn test_detect_added_and_removed_upstream() {
        let physical = base_set();
        let twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let mut updated = base_set();
        updated.remove(2); // r3 removed upstream
        updated.push(rule("r4", 1, "Right four")); // r4 added upstream
        let reports = twin.detect_divergence(&updated);
        let kinds: BTreeMap<&str, DivergenceKind> = reports
            .iter()
            .map(|r| (r.rule_id.as_str(), r.kind))
            .collect();
        assert_eq!(kinds.get("r4"), Some(&DivergenceKind::AddedUpstream));
        assert_eq!(kinds.get("r3"), Some(&DivergenceKind::RemovedUpstream));
        assert_eq!(twin.sync_state("r4", &updated), SyncState::MissingInTwin);
        assert_eq!(twin.sync_state("r3", &updated), SyncState::OrphanInTwin);
    }

    #[test]
    fn test_plan_sync_classifies_actions() {
        let physical = base_set();
        let twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let mut updated = base_set();
        updated[0].version = 2; // update
        updated.remove(2); // remove r3
        updated.push(rule("r4", 1, "Right four")); // add r4
        let plan = twin.plan_sync(&updated);
        assert_eq!(plan.to_update, vec!["r1"]);
        assert_eq!(plan.to_add, vec!["r4"]);
        assert_eq!(plan.to_remove, vec!["r3"]);
        assert_eq!(plan.action_count(), 3);
    }

    #[test]
    fn test_apply_sync_plan_brings_back_in_sync() {
        let physical = base_set();
        let mut twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let mut updated = base_set();
        updated[0].version = 2;
        updated.remove(2);
        updated.push(rule("r4", 1, "Right four"));
        let outcomes = twin.resync(&updated).expect("resync");
        assert_eq!(outcomes.len(), 3);
        assert!(twin.is_in_sync(&updated));
        assert_eq!(twin.rule_count(), 3);
        assert!(twin.mirrored_rule("r4").is_some());
        assert!(twin.mirrored_rule("r3").is_none());
    }

    #[test]
    fn test_apply_sync_plan_rejects_inconsistent_plan() {
        let physical = base_set();
        let mut twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let bad_plan = TwinSyncPlan {
            to_update: Vec::new(),
            to_add: vec!["ghost".to_string()],
            to_remove: Vec::new(),
        };
        // "ghost" is not in the physical set -> error.
        assert!(twin.apply_sync_plan(&bad_plan, &physical).is_err());
    }

    #[test]
    fn test_resync_is_idempotent() {
        let physical = base_set();
        let mut twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let mut updated = base_set();
        updated[0].version = 3;
        twin.resync(&updated).expect("first");
        let second = twin.resync(&updated).expect("second");
        assert!(second.is_empty());
    }

    #[test]
    fn test_mirrored_rule_outcome_is_updated_not_added() {
        let physical = base_set();
        let mut twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let mut updated = base_set();
        updated[0].version = 2;
        let outcomes = twin.resync(&updated).expect("resync");
        assert_eq!(outcomes, vec![("r1".to_string(), SyncOutcome::Updated)]);
    }

    #[test]
    fn test_twin_serde_roundtrip() {
        let physical = base_set();
        let twin = LegalDigitalTwin::from_physical("US", "twin-us", &physical);
        let json = serde_json::to_string(&twin).expect("ser");
        let back: LegalDigitalTwin = serde_json::from_str(&json).expect("de");
        // Statute lacks PartialEq, so compare via re-serialization.
        assert_eq!(json, serde_json::to_string(&back).expect("reser"));
        assert_eq!(back.rule_count(), twin.rule_count());
        assert!(back.is_in_sync(&physical));
    }
}
