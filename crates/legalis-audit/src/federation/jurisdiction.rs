//! Multi-jurisdiction compliance evaluation.
//!
//! A multinational system rarely answers to one regulator. The same audit trail
//! may have to satisfy the EU (GDPR), the US (HIPAA / SOX / CCPA), a card-network
//! regime (PCI-DSS), and a corporate security baseline (ISO 27001 / SOC 2 / NIST)
//! simultaneously — and the *applicable* standards differ by jurisdiction.
//!
//! This module models jurisdictions and evaluates a system's *provided audit
//! capabilities* (expressed as [`ControlObjective`]s) against the standards each
//! jurisdiction mandates:
//!
//! - [`Jurisdiction`] names a legal/operational domain and the [`Standard`]s it
//!   requires.
//! - [`MultiJurisdictionEvaluator`] holds a [`StandardMapping`] plus a set of
//!   jurisdictions, and produces a [`MultiJurisdictionReport`] — per-jurisdiction
//!   [`JurisdictionCompliance`] plus a global roll-up and the *minimal set of
//!   additional objectives* that would bring every jurisdiction into
//!   compliance.
//!
//! Optionally the evaluator can *derive* the provided objectives directly from a
//! slice of [`AuditRecord`]s (via [`derive_objectives`]) so a live trail can be
//! scored against the world's regimes in one call.

use crate::federation::standards::{
    Control, ControlObjective, CoverageReport, Standard, StandardMapping,
};
use crate::{Actor, AuditRecord, DecisionResult, EventType};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// A legal / operational jurisdiction and the standards it mandates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// Stable code (e.g. "EU", "US", "US-CA", "GLOBAL").
    pub code: String,
    /// Human-readable name.
    pub name: String,
    /// The standards this jurisdiction requires compliance with.
    pub required_standards: BTreeSet<Standard>,
}

impl Jurisdiction {
    /// Builds a jurisdiction.
    pub fn new(
        code: impl Into<String>,
        name: impl Into<String>,
        standards: impl IntoIterator<Item = Standard>,
    ) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            required_standards: standards.into_iter().collect(),
        }
    }

    /// The European Union (GDPR + ISO 27001 baseline).
    pub fn european_union() -> Self {
        Self::new("EU", "European Union", [Standard::Gdpr, Standard::Iso27001])
    }

    /// The United States (federal): HIPAA, SOX, NIST, SOC 2.
    pub fn united_states() -> Self {
        Self::new(
            "US",
            "United States (Federal)",
            [
                Standard::Hipaa,
                Standard::Sox,
                Standard::Nist80053,
                Standard::Soc2,
            ],
        )
    }

    /// California (adds CCPA on top of US baseline).
    pub fn california() -> Self {
        Self::new(
            "US-CA",
            "California",
            [
                Standard::Ccpa,
                Standard::Hipaa,
                Standard::Nist80053,
                Standard::Soc2,
            ],
        )
    }

    /// A card-payment operating environment (PCI-DSS + ISO 27001).
    pub fn payment_card() -> Self {
        Self::new(
            "PCI",
            "Payment Card Environment",
            [Standard::PciDss, Standard::Iso27001],
        )
    }
}

/// The compliance result for one jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionCompliance {
    /// The jurisdiction code.
    pub jurisdiction: String,
    /// The standards evaluated.
    pub standards: Vec<Standard>,
    /// Per-standard coverage reports.
    pub per_standard: BTreeMap<String, CoverageReport>,
    /// Total controls in scope across the jurisdiction's standards.
    pub total_controls: usize,
    /// Controls fully satisfied.
    pub satisfied_controls: usize,
    /// Whether every control in scope is fully satisfied.
    pub is_compliant: bool,
    /// Objectives that, if added, would satisfy currently-unmet/partial
    /// controls in this jurisdiction.
    pub missing_objectives: Vec<ControlObjective>,
    /// Overall compliance fraction in `[0, 1]`.
    pub compliance_fraction: f64,
}

/// The complete multi-jurisdiction evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiJurisdictionReport {
    /// Provided capabilities used in the evaluation.
    pub provided_objectives: Vec<ControlObjective>,
    /// Per-jurisdiction results.
    pub jurisdictions: Vec<JurisdictionCompliance>,
    /// Jurisdictions fully compliant.
    pub compliant_jurisdictions: usize,
    /// Whether every evaluated jurisdiction is compliant.
    pub globally_compliant: bool,
    /// The union of objectives missing across *all* jurisdictions — the minimal
    /// remediation set to satisfy the whole world at once.
    pub global_missing_objectives: Vec<ControlObjective>,
}

impl MultiJurisdictionReport {
    /// Fraction of evaluated jurisdictions that are compliant.
    pub fn compliant_fraction(&self) -> f64 {
        if self.jurisdictions.is_empty() {
            return 0.0;
        }
        self.compliant_jurisdictions as f64 / self.jurisdictions.len() as f64
    }

    /// The least-compliant jurisdiction, if any.
    pub fn weakest(&self) -> Option<&JurisdictionCompliance> {
        self.jurisdictions
            .iter()
            .min_by(|a, b| a.compliance_fraction.total_cmp(&b.compliance_fraction))
    }
}

/// Evaluates provided audit capabilities against multiple jurisdictions.
#[derive(Debug, Clone)]
pub struct MultiJurisdictionEvaluator {
    mapping: StandardMapping,
    jurisdictions: Vec<Jurisdiction>,
}

impl MultiJurisdictionEvaluator {
    /// Builds an evaluator over the given mapping and jurisdictions.
    pub fn new(mapping: StandardMapping, jurisdictions: Vec<Jurisdiction>) -> Self {
        Self {
            mapping,
            jurisdictions,
        }
    }

    /// Builds an evaluator with the built-in control mapping and the major
    /// jurisdictions (EU, US, California, payment-card).
    pub fn with_defaults() -> Self {
        Self::new(
            StandardMapping::with_builtin_controls(),
            vec![
                Jurisdiction::european_union(),
                Jurisdiction::united_states(),
                Jurisdiction::california(),
                Jurisdiction::payment_card(),
            ],
        )
    }

    /// Adds a jurisdiction (builder style).
    pub fn add_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdictions.push(jurisdiction);
        self
    }

    /// The underlying standard mapping.
    pub fn mapping(&self) -> &StandardMapping {
        &self.mapping
    }

    /// The configured jurisdictions.
    pub fn jurisdictions(&self) -> &[Jurisdiction] {
        &self.jurisdictions
    }

    /// Evaluates the supplied provided-objective set against every jurisdiction.
    pub fn evaluate(&self, provided: &BTreeSet<ControlObjective>) -> MultiJurisdictionReport {
        let mut results = Vec::new();
        let mut global_missing: BTreeSet<ControlObjective> = BTreeSet::new();

        for jurisdiction in &self.jurisdictions {
            let result = self.evaluate_jurisdiction(jurisdiction, provided);
            for obj in &result.missing_objectives {
                global_missing.insert(*obj);
            }
            results.push(result);
        }

        let compliant = results.iter().filter(|r| r.is_compliant).count();
        MultiJurisdictionReport {
            provided_objectives: provided.iter().copied().collect(),
            compliant_jurisdictions: compliant,
            globally_compliant: !results.is_empty() && compliant == results.len(),
            global_missing_objectives: global_missing.into_iter().collect(),
            jurisdictions: results,
        }
    }

    /// Evaluates the provided capabilities derived from a live audit trail
    /// against every jurisdiction.
    pub fn evaluate_records(&self, records: &[AuditRecord]) -> MultiJurisdictionReport {
        let provided = derive_objectives(records);
        self.evaluate(&provided)
    }

    /// Evaluates one jurisdiction.
    fn evaluate_jurisdiction(
        &self,
        jurisdiction: &Jurisdiction,
        provided: &BTreeSet<ControlObjective>,
    ) -> JurisdictionCompliance {
        // Collect controls in scope: those whose standard is required here.
        let in_scope: Vec<&Control> = self
            .mapping
            .controls()
            .filter(|c| jurisdiction.required_standards.contains(&c.standard))
            .collect();

        let total_controls = in_scope.len();
        let mut satisfied_controls = 0usize;
        let mut missing: BTreeSet<ControlObjective> = BTreeSet::new();

        for control in &in_scope {
            if control.objectives.is_empty() {
                continue;
            }
            let uncovered: Vec<ControlObjective> =
                control.objectives.difference(provided).copied().collect();
            if uncovered.is_empty() {
                satisfied_controls += 1;
            } else {
                for obj in uncovered {
                    missing.insert(obj);
                }
            }
        }

        // Per-standard coverage (scoped to this jurisdiction's standards).
        let mut per_standard = BTreeMap::new();
        let mut standards: Vec<Standard> =
            jurisdiction.required_standards.iter().copied().collect();
        standards.sort();
        for standard in &standards {
            let scoped = self.scoped_mapping(*standard);
            per_standard.insert(standard.code().to_string(), scoped.coverage(provided));
        }

        let compliance_fraction = if total_controls == 0 {
            1.0
        } else {
            satisfied_controls as f64 / total_controls as f64
        };

        JurisdictionCompliance {
            jurisdiction: jurisdiction.code.clone(),
            standards,
            per_standard,
            total_controls,
            satisfied_controls,
            is_compliant: total_controls > 0 && satisfied_controls == total_controls,
            missing_objectives: missing.into_iter().collect(),
            compliance_fraction,
        }
    }

    /// Builds a sub-mapping containing only one standard's controls (for
    /// per-standard coverage reporting).
    fn scoped_mapping(&self, standard: Standard) -> StandardMapping {
        let mut m = StandardMapping::new();
        for control in self.mapping.controls_for(standard) {
            m.insert_control(control.clone());
        }
        m
    }
}

/// Derives the set of [`ControlObjective`]s a system *demonstrably provides*
/// from the evidence in an audit trail.
///
/// The inference is conservative and evidence-based:
/// - Any records at all → [`ControlObjective::AuditLogging`].
/// - Records whose hash chain verifies → [`ControlObjective::LogIntegrity`].
/// - Presence of non-system actors with roles → [`ControlObjective::AccessControl`]
///   and [`ControlObjective::SegregationOfDuties`].
/// - Override / appeal events → [`ControlObjective::Monitoring`] and
///   [`ControlObjective::IncidentResponse`].
/// - Statute-modification events → [`ControlObjective::ChangeManagement`].
/// - Subjects present (so subject-scoped export/erasure is possible) →
///   [`ControlObjective::DataSubjectRights`] and [`ControlObjective::Retention`].
///
/// Encryption is *not* inferred from records alone (it is a storage-layer
/// property), so callers that encrypt at rest should add
/// [`ControlObjective::Encryption`] explicitly.
pub fn derive_objectives(records: &[AuditRecord]) -> BTreeSet<ControlObjective> {
    use ControlObjective::*;
    let mut set = BTreeSet::new();
    if records.is_empty() {
        return set;
    }
    set.insert(AuditLogging);

    // Chain integrity.
    let mut chain_ok = true;
    let mut expected_prev: Option<String> = None;
    for r in records {
        if !r.verify() || r.previous_hash != expected_prev {
            chain_ok = false;
        }
        expected_prev = Some(r.record_hash.clone());
    }
    if chain_ok {
        set.insert(LogIntegrity);
        // Retention is demonstrable once an intact, time-ordered record set
        // exists that can be retained and queried.
        set.insert(Retention);
    }

    let mut has_user = false;
    let mut distinct_roles: BTreeSet<String> = BTreeSet::new();
    let mut has_review = false;
    let mut has_change = false;
    let mut has_subjects = false;

    for r in records {
        match &r.actor {
            Actor::User { role, .. } => {
                has_user = true;
                distinct_roles.insert(role.clone());
            }
            Actor::External { .. } => {
                has_user = true;
            }
            Actor::System { .. } => {}
        }
        match r.event_type {
            EventType::HumanOverride | EventType::Appeal | EventType::DiscretionaryReview => {
                has_review = true;
            }
            EventType::StatuteModified => has_change = true,
            _ => {}
        }
        if matches!(r.result, DecisionResult::Overridden { .. }) {
            has_review = true;
        }
        if !r.subject_id.is_nil() {
            has_subjects = true;
        }
    }

    if has_user {
        set.insert(AccessControl);
    }
    if distinct_roles.len() >= 2 {
        set.insert(SegregationOfDuties);
    }
    if has_review {
        set.insert(Monitoring);
        set.insert(IncidentResponse);
    }
    if has_change {
        set.insert(ChangeManagement);
    }
    if has_subjects {
        set.insert(DataSubjectRights);
    }

    set
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecisionContext, EventType};
    use std::collections::HashMap as StdHashMap;
    use uuid::Uuid;

    fn det() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "engine".to_string(),
            },
            "s".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    fn chain(records: &mut [AuditRecord]) {
        let mut prev: Option<String> = None;
        for r in records.iter_mut() {
            r.relink(prev.clone());
            prev = Some(r.record_hash.clone());
        }
    }

    #[test]
    fn test_full_capabilities_globally_compliant() {
        use ControlObjective::*;
        // Provide every objective -> all controls satisfied everywhere.
        let provided: BTreeSet<ControlObjective> = [
            AuditLogging,
            LogIntegrity,
            AccessControl,
            Retention,
            DataSubjectRights,
            Encryption,
            Monitoring,
            IncidentResponse,
            SegregationOfDuties,
            ChangeManagement,
        ]
        .into_iter()
        .collect();

        let evaluator = MultiJurisdictionEvaluator::with_defaults();
        let report = evaluator.evaluate(&provided);
        assert!(report.globally_compliant);
        assert!(report.global_missing_objectives.is_empty());
        assert_eq!(report.compliant_fraction(), 1.0);
    }

    #[test]
    fn test_partial_capabilities_not_compliant() {
        use ControlObjective::*;
        let provided: BTreeSet<ControlObjective> = [AuditLogging].into_iter().collect();
        let evaluator = MultiJurisdictionEvaluator::with_defaults();
        let report = evaluator.evaluate(&provided);
        assert!(!report.globally_compliant);
        assert!(!report.global_missing_objectives.is_empty());
        // EU requires LogIntegrity etc., so it should be missing objectives.
        let eu = report
            .jurisdictions
            .iter()
            .find(|j| j.jurisdiction == "EU")
            .expect("EU");
        assert!(!eu.is_compliant);
        assert!(eu.missing_objectives.contains(&LogIntegrity));
    }

    #[test]
    fn test_global_missing_is_union() {
        use ControlObjective::*;
        let provided: BTreeSet<ControlObjective> =
            [AuditLogging, LogIntegrity].into_iter().collect();
        let evaluator = MultiJurisdictionEvaluator::with_defaults();
        let report = evaluator.evaluate(&provided);
        // The global missing set must be a superset of each jurisdiction's.
        for j in &report.jurisdictions {
            for obj in &j.missing_objectives {
                assert!(report.global_missing_objectives.contains(obj));
            }
        }
    }

    #[test]
    fn test_weakest_jurisdiction() {
        use ControlObjective::*;
        let provided: BTreeSet<ControlObjective> = [AuditLogging].into_iter().collect();
        let evaluator = MultiJurisdictionEvaluator::with_defaults();
        let report = evaluator.evaluate(&provided);
        let weakest = report.weakest().expect("weakest");
        for j in &report.jurisdictions {
            assert!(weakest.compliance_fraction <= j.compliance_fraction + 1e-9);
        }
    }

    #[test]
    fn test_derive_objectives_basic() {
        use ControlObjective::*;
        let mut records = vec![det(), det(), det()];
        chain(&mut records);
        let derived = derive_objectives(&records);
        assert!(derived.contains(&AuditLogging));
        assert!(derived.contains(&LogIntegrity));
        assert!(derived.contains(&Retention));
        assert!(derived.contains(&DataSubjectRights));
        // System-only, no roles -> no segregation.
        assert!(!derived.contains(&SegregationOfDuties));
    }

    #[test]
    fn test_derive_objectives_with_roles_and_review() {
        use ControlObjective::*;
        let mut records = vec![det(), det()];
        records[0].actor = Actor::User {
            user_id: "u1".to_string(),
            role: "reviewer".to_string(),
        };
        records[1].actor = Actor::User {
            user_id: "u2".to_string(),
            role: "approver".to_string(),
        };
        records[1].event_type = EventType::HumanOverride;
        chain(&mut records);
        let derived = derive_objectives(&records);
        assert!(derived.contains(&AccessControl));
        assert!(derived.contains(&SegregationOfDuties));
        assert!(derived.contains(&Monitoring));
        assert!(derived.contains(&IncidentResponse));
    }

    #[test]
    fn test_derive_objectives_broken_chain_no_integrity() {
        use ControlObjective::*;
        let mut records = vec![det(), det()];
        chain(&mut records);
        records[1].record_hash = "tampered".to_string();
        let derived = derive_objectives(&records);
        assert!(derived.contains(&AuditLogging));
        assert!(!derived.contains(&LogIntegrity));
    }

    #[test]
    fn test_derive_objectives_empty() {
        assert!(derive_objectives(&[]).is_empty());
    }

    #[test]
    fn test_evaluate_records_end_to_end() {
        let mut records = vec![det(), det(), det()];
        records[1].actor = Actor::User {
            user_id: "u1".to_string(),
            role: "reviewer".to_string(),
        };
        records[2].actor = Actor::User {
            user_id: "u2".to_string(),
            role: "approver".to_string(),
        };
        records[2].event_type = EventType::HumanOverride;
        chain(&mut records);
        let evaluator = MultiJurisdictionEvaluator::with_defaults();
        let report = evaluator.evaluate_records(&records);
        // Derived from records (no encryption) -> not globally compliant, but
        // some jurisdictions partially covered.
        assert!(!report.provided_objectives.is_empty());
        assert!(report.jurisdictions.iter().all(|j| j.total_controls > 0));
    }

    #[test]
    fn test_custom_jurisdiction() {
        use ControlObjective::*;
        let evaluator = MultiJurisdictionEvaluator::new(
            StandardMapping::with_builtin_controls(),
            vec![Jurisdiction::new("JP", "Japan", [Standard::Iso27001])],
        );
        let provided: BTreeSet<ControlObjective> = [
            AuditLogging,
            LogIntegrity,
            AccessControl,
            Retention,
            Encryption,
            Monitoring,
            IncidentResponse,
            SegregationOfDuties,
        ]
        .into_iter()
        .collect();
        let report = evaluator.evaluate(&provided);
        let jp = &report.jurisdictions[0];
        assert_eq!(jp.jurisdiction, "JP");
        assert!(jp.is_compliant);
    }

    #[test]
    fn test_report_serializes() {
        use ControlObjective::*;
        let provided: BTreeSet<ControlObjective> =
            [AuditLogging, LogIntegrity].into_iter().collect();
        let report = MultiJurisdictionEvaluator::with_defaults().evaluate(&provided);
        let json = serde_json::to_string(&report).expect("serialize");
        assert!(json.contains("jurisdictions"));
    }
}
