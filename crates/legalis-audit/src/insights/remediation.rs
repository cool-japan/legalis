//! Remediation suggestions driven by a template catalogue.
//!
//! Every [`FindingKind`] maps to one or more [`RemediationTemplate`]s — curated
//! playbooks describing how to address that class of finding. The catalogue is
//! pre-populated with sensible defaults but is fully extensible: callers can
//! register additional templates for bespoke finding kinds.
//!
//! Templates contain `{placeholder}` tokens that are filled in from the concrete
//! finding (its title, blast-radius counts, and any attached metrics), so the
//! emitted [`RemediationSuggestion`]s are specific rather than generic boiler-
//! plate.

use crate::insights::finding::{AuditFinding, FindingKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The relative effort of carrying out a remediation.
///
/// Declared ascending so the derived [`Ord`] keeps
/// `Trivial < Low < Moderate < High < Significant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RemediationEffort {
    /// Minutes of work.
    Trivial,
    /// A few hours.
    Low,
    /// Up to a couple of days.
    Moderate,
    /// A week or so.
    High,
    /// A major, multi-week undertaking.
    Significant,
}

impl RemediationEffort {
    /// A rough effort estimate in engineer-hours.
    pub fn estimated_hours(self) -> u32 {
        match self {
            RemediationEffort::Trivial => 1,
            RemediationEffort::Low => 4,
            RemediationEffort::Moderate => 16,
            RemediationEffort::High => 40,
            RemediationEffort::Significant => 120,
        }
    }
}

/// A concrete, finding-specific remediation recommendation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemediationSuggestion {
    /// The finding kind this suggestion addresses.
    pub finding_kind: String,
    /// Short headline.
    pub title: String,
    /// Detailed, context-filled description.
    pub description: String,
    /// Ordered remediation steps.
    pub steps: Vec<String>,
    /// Estimated effort.
    pub effort: RemediationEffort,
    /// Estimated proportional risk reduction in `[0, 1]`.
    pub expected_impact: f64,
    /// Supporting references (controls, standards, runbooks).
    pub references: Vec<String>,
}

/// A reusable remediation playbook for a finding kind.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemediationTemplate {
    /// The finding kind this template addresses.
    pub kind: FindingKind,
    /// Headline (may contain `{placeholder}` tokens).
    pub title: String,
    /// Description (may contain `{placeholder}` tokens).
    pub description: String,
    /// Steps (each may contain `{placeholder}` tokens).
    pub steps: Vec<String>,
    /// Estimated effort.
    pub effort: RemediationEffort,
    /// Estimated proportional risk reduction in `[0, 1]`.
    pub expected_impact: f64,
    /// Supporting references.
    pub references: Vec<String>,
}

impl RemediationTemplate {
    /// Fills in `{placeholder}` tokens from a finding's context and produces a
    /// concrete [`RemediationSuggestion`].
    pub fn instantiate(&self, finding: &AuditFinding) -> RemediationSuggestion {
        let ctx = build_context(finding);
        RemediationSuggestion {
            finding_kind: finding.kind.label(),
            title: apply_context(&self.title, &ctx),
            description: apply_context(&self.description, &ctx),
            steps: self.steps.iter().map(|s| apply_context(s, &ctx)).collect(),
            effort: self.effort,
            expected_impact: self.expected_impact,
            references: self.references.clone(),
        }
    }
}

/// A catalogue mapping finding kinds to remediation templates.
#[derive(Debug, Clone)]
pub struct RemediationCatalog {
    templates: HashMap<FindingKind, Vec<RemediationTemplate>>,
}

impl RemediationCatalog {
    /// Creates an empty catalogue.
    pub fn empty() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// Registers a template, appending it to its kind's playbook list.
    pub fn register(&mut self, template: RemediationTemplate) {
        self.templates
            .entry(template.kind.clone())
            .or_default()
            .push(template);
    }

    /// Returns the templates registered for a kind, if any.
    pub fn templates_for(&self, kind: &FindingKind) -> Option<&[RemediationTemplate]> {
        self.templates.get(kind).map(|v| v.as_slice())
    }

    /// Number of distinct finding kinds covered.
    pub fn covered_kinds(&self) -> usize {
        self.templates.len()
    }

    /// Produces context-filled suggestions for a finding. If no template
    /// matches the finding's kind, a generic fallback suggestion is returned so
    /// callers always receive actionable guidance.
    pub fn suggest(&self, finding: &AuditFinding) -> Vec<RemediationSuggestion> {
        match self.templates.get(&finding.kind) {
            Some(templates) if !templates.is_empty() => {
                templates.iter().map(|t| t.instantiate(finding)).collect()
            }
            _ => vec![generic_fallback(finding)],
        }
    }
}

impl Default for RemediationCatalog {
    /// Returns a catalogue pre-populated with a template for every built-in
    /// finding kind.
    fn default() -> Self {
        let mut catalog = Self::empty();
        for template in default_templates() {
            catalog.register(template);
        }
        catalog
    }
}

/// Builds the placeholder substitution map from a finding.
fn build_context(finding: &AuditFinding) -> HashMap<String, String> {
    let mut ctx = HashMap::new();
    ctx.insert("title".to_string(), finding.title.clone());
    ctx.insert("kind".to_string(), finding.kind.label());
    ctx.insert(
        "affected_records".to_string(),
        finding.blast_radius.affected_records.to_string(),
    );
    ctx.insert(
        "affected_subjects".to_string(),
        finding.blast_radius.affected_subjects.to_string(),
    );
    ctx.insert(
        "affected_statutes".to_string(),
        finding.blast_radius.affected_statutes.to_string(),
    );
    for (key, value) in finding.metrics.iter() {
        ctx.insert(key.clone(), format!("{value:.3}"));
    }
    ctx
}

/// Replaces every `{key}` token in `template` with its mapped value.
fn apply_context(template: &str, ctx: &HashMap<String, String>) -> String {
    let mut out = template.to_string();
    for (key, value) in ctx.iter() {
        out = out.replace(&format!("{{{key}}}"), value);
    }
    out
}

/// A generic suggestion used when no specific template is registered.
fn generic_fallback(finding: &AuditFinding) -> RemediationSuggestion {
    RemediationSuggestion {
        finding_kind: finding.kind.label(),
        title: format!("Investigate: {}", finding.title),
        description: format!(
            "Triage the finding '{}' affecting {} record(s), determine whether it reflects a genuine control gap, and assign an owner.",
            finding.title, finding.blast_radius.affected_records
        ),
        steps: vec![
            "Confirm the finding against the underlying audit records.".to_string(),
            "Classify it as true-positive, false-positive, or accepted risk.".to_string(),
            "If genuine, define and track a corrective action.".to_string(),
        ],
        effort: RemediationEffort::Low,
        expected_impact: 0.3,
        references: Vec::new(),
    }
}

/// The built-in template set, one playbook per finding kind.
fn default_templates() -> Vec<RemediationTemplate> {
    vec![
        RemediationTemplate {
            kind: FindingKind::VolumeSpike,
            title: "Investigate decision volume spike".to_string(),
            description:
                "A bucket recorded {observed} decisions against an expected {expected}. Confirm whether this reflects legitimate demand or an upstream fault."
                    .to_string(),
            steps: vec![
                "Inspect the intake pipeline for batch jobs, retries, or replays.".to_string(),
                "Validate that upstream systems are healthy and not double-submitting.".to_string(),
                "If demand is genuine, review capacity and rate-limiting headroom.".to_string(),
            ],
            effort: RemediationEffort::Low,
            expected_impact: 0.5,
            references: vec!["ops/runbooks/volume-spike".to_string()],
        },
        RemediationTemplate {
            kind: FindingKind::VolumeDrop,
            title: "Investigate decision volume drop".to_string(),
            description:
                "Volume fell to {observed} against an expected {expected}, which often signals an ingestion outage."
                    .to_string(),
            steps: vec![
                "Check ingestion connectors and message queues for stalls.".to_string(),
                "Verify that no upstream filter is silently dropping events.".to_string(),
                "Confirm scheduler / cron health for batch decisioning.".to_string(),
            ],
            effort: RemediationEffort::Low,
            expected_impact: 0.5,
            references: vec!["ops/runbooks/ingestion".to_string()],
        },
        RemediationTemplate {
            kind: FindingKind::FrequencySpike,
            title: "Review statute usage surge".to_string(),
            description:
                "{title}: a single statute was applied {observed} times versus a baseline of {expected}."
                    .to_string(),
            steps: vec![
                "Examine recent changes to the surging rule and its triggers.".to_string(),
                "Rule out mis-routing where unrelated matters hit this statute.".to_string(),
                "Validate the inputs feeding the rule's conditions.".to_string(),
            ],
            effort: RemediationEffort::Moderate,
            expected_impact: 0.55,
            references: vec!["legal/rule-review".to_string()],
        },
        RemediationTemplate {
            kind: FindingKind::RareEvent,
            title: "Validate rarely-exercised rule".to_string(),
            description:
                "{title}. Seldom-used rules accumulate latent defects because they are rarely tested in production."
                    .to_string(),
            steps: vec![
                "Add or refresh automated test coverage for the rule.".to_string(),
                "Schedule a periodic manual review of the rule's correctness.".to_string(),
                "Document the expected trigger conditions for future auditors.".to_string(),
            ],
            effort: RemediationEffort::Low,
            expected_impact: 0.4,
            references: vec!["qa/coverage".to_string()],
        },
        RemediationTemplate {
            kind: FindingKind::BaselineDrift,
            title: "Recalibrate decision baseline".to_string(),
            description:
                "Decision volume drifted from {expected} to {observed}; determine whether this is an intentional regime change."
                    .to_string(),
            steps: vec![
                "Correlate the drift with policy, rule, or population changes.".to_string(),
                "Update monitoring baselines once the new regime is confirmed.".to_string(),
                "Communicate the shift to compliance stakeholders.".to_string(),
            ],
            effort: RemediationEffort::Moderate,
            expected_impact: 0.5,
            references: vec!["monitoring/baselines".to_string()],
        },
        RemediationTemplate {
            kind: FindingKind::OutcomeDrift,
            title: "Investigate outcome distribution shift".to_string(),
            description:
                "The outcome mix diverged from the learned baseline (TVD {total_variation_distance}); investigate the cause of the behavioural shift."
                    .to_string(),
            steps: vec![
                "Diff the active rule versions against the prior baseline period.".to_string(),
                "Check for data drift in the attributes driving outcomes.".to_string(),
                "Sample affected decisions to confirm correctness.".to_string(),
            ],
            effort: RemediationEffort::High,
            expected_impact: 0.6,
            references: vec!["legal/outcome-review".to_string()],
        },
        RemediationTemplate {
            kind: FindingKind::ImprobableTransition,
            title: "Audit unexpected decision pathway".to_string(),
            description:
                "{title} (probability {transition_probability}). Such pathways can indicate logic errors, data issues, or tampering."
                    .to_string(),
            steps: vec![
                "Trace the implicated records end-to-end.".to_string(),
                "Verify hash-chain integrity for the affected segment.".to_string(),
                "Review the override or void rationale, if present.".to_string(),
            ],
            effort: RemediationEffort::Moderate,
            expected_impact: 0.55,
            references: vec!["forensics/pathway-audit".to_string()],
        },
        RemediationTemplate {
            kind: FindingKind::OverrideCluster,
            title: "Review human-override cluster".to_string(),
            description:
                "An elevated cluster of {affected_records} overrides was detected; clustered overrides often reveal a systematic rule gap."
                    .to_string(),
            steps: vec![
                "Sample overrides for justification quality and consistency.".to_string(),
                "Refine the underlying rule so the common case no longer needs an override.".to_string(),
                "Coach reviewers where overrides appear inconsistent.".to_string(),
            ],
            effort: RemediationEffort::Moderate,
            expected_impact: 0.6,
            references: vec!["legal/override-policy".to_string()],
        },
        RemediationTemplate {
            kind: FindingKind::ElevatedVoidRate,
            title: "Fix logic errors causing voided decisions".to_string(),
            description:
                "An elevated rate of voided decisions ({affected_records} records) indicates defective rule logic."
                    .to_string(),
            steps: vec![
                "Identify the conditions that evaluate inconsistently.".to_string(),
                "Patch the rule logic and add regression tests.".to_string(),
                "Re-run the affected matters once the fix is deployed.".to_string(),
            ],
            effort: RemediationEffort::High,
            expected_impact: 0.65,
            references: vec!["eng/rule-defects".to_string()],
        },
        RemediationTemplate {
            kind: FindingKind::IntegrityRisk,
            title: "Restore audit-trail integrity".to_string(),
            description:
                "An integrity risk was raised over {affected_records} record(s); the tamper-evidence guarantees of the log may be compromised."
                    .to_string(),
            steps: vec![
                "Run a full hash-chain and Merkle verification.".to_string(),
                "Isolate and quarantine the affected log segment.".to_string(),
                "Escalate to the security team and preserve evidence.".to_string(),
            ],
            effort: RemediationEffort::Significant,
            expected_impact: 0.8,
            references: vec!["security/incident-response".to_string()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insights::finding::{BlastRadius, Likelihood, Severity};

    fn finding(kind: FindingKind) -> AuditFinding {
        AuditFinding::new(
            kind,
            "Volume spike in volume",
            Severity::Medium,
            Likelihood::Likely,
            BlastRadius::from_counts(40, 30, 1),
        )
        .with_metric("observed", 40.0)
        .with_metric("expected", 2.0)
    }

    #[test]
    fn test_default_catalog_covers_all_builtin_kinds() {
        let catalog = RemediationCatalog::default();
        // Ten built-in kinds (Custom is intentionally handled by fallback).
        assert_eq!(catalog.covered_kinds(), 10);
        for kind in [
            FindingKind::VolumeSpike,
            FindingKind::VolumeDrop,
            FindingKind::FrequencySpike,
            FindingKind::RareEvent,
            FindingKind::BaselineDrift,
            FindingKind::OutcomeDrift,
            FindingKind::ImprobableTransition,
            FindingKind::OverrideCluster,
            FindingKind::ElevatedVoidRate,
            FindingKind::IntegrityRisk,
        ] {
            assert!(catalog.templates_for(&kind).is_some(), "{kind:?}");
        }
    }

    #[test]
    fn test_placeholder_substitution() {
        let catalog = RemediationCatalog::default();
        let suggestions = catalog.suggest(&finding(FindingKind::VolumeSpike));
        assert_eq!(suggestions.len(), 1);
        let s = &suggestions[0];
        // {observed}/{expected} must be replaced with the metric values.
        assert!(s.description.contains("40.000"));
        assert!(s.description.contains("2.000"));
        assert!(!s.description.contains("{observed}"));
        assert_eq!(s.effort, RemediationEffort::Low);
    }

    #[test]
    fn test_fallback_for_unregistered_kind() {
        let catalog = RemediationCatalog::default();
        let suggestions = catalog.suggest(&finding(FindingKind::Custom("bespoke".to_string())));
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].title.starts_with("Investigate:"));
    }

    #[test]
    fn test_register_custom_template() {
        let mut catalog = RemediationCatalog::empty();
        catalog.register(RemediationTemplate {
            kind: FindingKind::Custom("bespoke".to_string()),
            title: "Custom fix for {title}".to_string(),
            description: "do the thing".to_string(),
            steps: vec!["step one".to_string()],
            effort: RemediationEffort::Trivial,
            expected_impact: 0.9,
            references: vec![],
        });
        let suggestions = catalog.suggest(&finding(FindingKind::Custom("bespoke".to_string())));
        assert_eq!(
            suggestions[0].title,
            "Custom fix for Volume spike in volume"
        );
        assert_eq!(suggestions[0].effort, RemediationEffort::Trivial);
    }

    #[test]
    fn test_effort_ordering_and_hours() {
        assert!(RemediationEffort::Significant > RemediationEffort::Trivial);
        assert!(
            RemediationEffort::High.estimated_hours() > RemediationEffort::Low.estimated_hours()
        );
    }
}
