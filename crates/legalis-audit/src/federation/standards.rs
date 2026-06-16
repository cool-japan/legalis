//! International standard mapping.
//!
//! Real-world compliance programmes must satisfy *many* overlapping frameworks
//! at once — ISO 27001, SOC 2, NIST SP 800-53, GDPR, HIPAA, PCI-DSS, … — whose
//! individual controls frequently address the *same underlying capability*
//! (e.g. "tamper-evident audit logging") under different names and numbering.
//!
//! This module models that landscape as structured, queryable data:
//!
//! - [`Standard`] enumerates the supported frameworks.
//! - [`Control`] is one control within a standard (its native id, title, and the
//!   abstract [`ControlObjective`]s it satisfies).
//! - [`ControlObjective`] is the *cross-standard pivot*: a normalized capability
//!   that controls from different standards map onto, which is what makes
//!   "implement once, satisfy many" analysis possible.
//! - [`StandardMapping`] is the registry of standards + controls and answers
//!   cross-mapping queries: given a control, which controls in another standard
//!   are equivalent? Given the audit capabilities a system provides, which
//!   controls are satisfied?
//!
//! A [`StandardMapping::with_builtin_controls`] catalogue seeds the registry with
//! the audit-relevant controls of the major frameworks so the crate is useful
//! out of the box, while remaining fully extensible.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// A supported compliance/security framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Standard {
    /// ISO/IEC 27001 — Information security management.
    Iso27001,
    /// AICPA SOC 2 — Trust services criteria.
    Soc2,
    /// NIST SP 800-53 — Security and privacy controls.
    Nist80053,
    /// EU General Data Protection Regulation.
    Gdpr,
    /// US Health Insurance Portability and Accountability Act.
    Hipaa,
    /// US Sarbanes-Oxley Act.
    Sox,
    /// California Consumer Privacy Act.
    Ccpa,
    /// Payment Card Industry Data Security Standard.
    PciDss,
}

impl Standard {
    /// Stable short code.
    pub fn code(self) -> &'static str {
        match self {
            Standard::Iso27001 => "ISO27001",
            Standard::Soc2 => "SOC2",
            Standard::Nist80053 => "NIST-800-53",
            Standard::Gdpr => "GDPR",
            Standard::Hipaa => "HIPAA",
            Standard::Sox => "SOX",
            Standard::Ccpa => "CCPA",
            Standard::PciDss => "PCI-DSS",
        }
    }

    /// Human-readable full name.
    pub fn full_name(self) -> &'static str {
        match self {
            Standard::Iso27001 => "ISO/IEC 27001 Information Security Management",
            Standard::Soc2 => "AICPA SOC 2 Trust Services Criteria",
            Standard::Nist80053 => "NIST SP 800-53 Security and Privacy Controls",
            Standard::Gdpr => "EU General Data Protection Regulation",
            Standard::Hipaa => "Health Insurance Portability and Accountability Act",
            Standard::Sox => "Sarbanes-Oxley Act",
            Standard::Ccpa => "California Consumer Privacy Act",
            Standard::PciDss => "Payment Card Industry Data Security Standard",
        }
    }

    /// All supported standards.
    pub fn all() -> [Standard; 8] {
        [
            Standard::Iso27001,
            Standard::Soc2,
            Standard::Nist80053,
            Standard::Gdpr,
            Standard::Hipaa,
            Standard::Sox,
            Standard::Ccpa,
            Standard::PciDss,
        ]
    }
}

/// A normalized, cross-standard control objective — the pivot that lets controls
/// from different frameworks be recognised as addressing the same capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ControlObjective {
    /// Audit events are logged with sufficient detail.
    AuditLogging,
    /// Audit logs are protected from tampering / cryptographically verifiable.
    LogIntegrity,
    /// Access to records is controlled and authenticated.
    AccessControl,
    /// Logs / records are retained for the required period.
    Retention,
    /// Personal data subjects can exercise access / erasure rights.
    DataSubjectRights,
    /// Data is encrypted at rest and/or in transit.
    Encryption,
    /// Logs are reviewed / monitored for anomalies.
    Monitoring,
    /// Incidents are detected, recorded, and responded to.
    IncidentResponse,
    /// Duties are segregated to prevent fraud / error.
    SegregationOfDuties,
    /// Changes to systems / data are tracked.
    ChangeManagement,
}

impl ControlObjective {
    /// Stable lower-snake label.
    pub fn label(self) -> &'static str {
        match self {
            ControlObjective::AuditLogging => "audit_logging",
            ControlObjective::LogIntegrity => "log_integrity",
            ControlObjective::AccessControl => "access_control",
            ControlObjective::Retention => "retention",
            ControlObjective::DataSubjectRights => "data_subject_rights",
            ControlObjective::Encryption => "encryption",
            ControlObjective::Monitoring => "monitoring",
            ControlObjective::IncidentResponse => "incident_response",
            ControlObjective::SegregationOfDuties => "segregation_of_duties",
            ControlObjective::ChangeManagement => "change_management",
        }
    }
}

/// A single control within a [`Standard`], pinned to the abstract
/// [`ControlObjective`]s it satisfies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    /// The framework this control belongs to.
    pub standard: Standard,
    /// The control's native identifier (e.g. "A.12.4.1", "CC7.2", "AU-2").
    pub control_id: String,
    /// Short title.
    pub title: String,
    /// The normalized objectives this control addresses.
    pub objectives: BTreeSet<ControlObjective>,
}

impl Control {
    /// Builds a control.
    pub fn new(
        standard: Standard,
        control_id: impl Into<String>,
        title: impl Into<String>,
        objectives: impl IntoIterator<Item = ControlObjective>,
    ) -> Self {
        Self {
            standard,
            control_id: control_id.into(),
            title: title.into(),
            objectives: objectives.into_iter().collect(),
        }
    }

    /// A globally-unique key `"<code>:<control_id>"`.
    pub fn key(&self) -> String {
        format!("{}:{}", self.standard.code(), self.control_id)
    }

    /// `true` when this control addresses `objective`.
    pub fn addresses(&self, objective: ControlObjective) -> bool {
        self.objectives.contains(&objective)
    }

    /// Count of objectives shared with `other`.
    pub fn shared_objectives(&self, other: &Control) -> usize {
        self.objectives.intersection(&other.objectives).count()
    }
}

/// A discovered equivalence between two controls in different standards, scored
/// by their objective overlap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossMapping {
    /// The source control key.
    pub from: String,
    /// The target control key.
    pub to: String,
    /// The target standard.
    pub target_standard: Standard,
    /// The objectives both controls share.
    pub shared_objectives: Vec<ControlObjective>,
    /// Jaccard similarity over objectives in `[0, 1]`.
    pub similarity: f64,
}

/// The registry of standards, controls, and cross-mapping queries.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StandardMapping {
    /// Controls keyed by [`Control::key`].
    controls: BTreeMap<String, Control>,
}

impl StandardMapping {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self {
            controls: BTreeMap::new(),
        }
    }

    /// Adds (or replaces) a control. Builder style.
    pub fn add_control(mut self, control: Control) -> Self {
        self.controls.insert(control.key(), control);
        self
    }

    /// Inserts a control in place.
    pub fn insert_control(&mut self, control: Control) {
        self.controls.insert(control.key(), control);
    }

    /// Number of controls registered.
    pub fn len(&self) -> usize {
        self.controls.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.controls.is_empty()
    }

    /// All controls.
    pub fn controls(&self) -> impl Iterator<Item = &Control> {
        self.controls.values()
    }

    /// Looks up a control by its `"<code>:<id>"` key.
    pub fn get(&self, key: &str) -> Option<&Control> {
        self.controls.get(key)
    }

    /// All controls belonging to `standard`.
    pub fn controls_for(&self, standard: Standard) -> Vec<&Control> {
        self.controls
            .values()
            .filter(|c| c.standard == standard)
            .collect()
    }

    /// All controls (across every standard) addressing `objective`.
    pub fn controls_addressing(&self, objective: ControlObjective) -> Vec<&Control> {
        self.controls
            .values()
            .filter(|c| c.addresses(objective))
            .collect()
    }

    /// Cross-maps `control_key` to the controls of `target` that share at least
    /// one objective, scored by Jaccard similarity and sorted descending.
    pub fn cross_map(&self, control_key: &str, target: Standard) -> Vec<CrossMapping> {
        let Some(source) = self.controls.get(control_key) else {
            return Vec::new();
        };
        let mut out: Vec<CrossMapping> = self
            .controls
            .values()
            .filter(|c| c.standard == target && c.key() != source.key())
            .filter_map(|c| {
                let shared: Vec<ControlObjective> = source
                    .objectives
                    .intersection(&c.objectives)
                    .copied()
                    .collect();
                if shared.is_empty() {
                    return None;
                }
                let union = source.objectives.union(&c.objectives).count();
                let similarity = if union == 0 {
                    0.0
                } else {
                    shared.len() as f64 / union as f64
                };
                Some(CrossMapping {
                    from: source.key(),
                    to: c.key(),
                    target_standard: target,
                    shared_objectives: shared,
                    similarity,
                })
            })
            .collect();
        out.sort_by(|a, b| b.similarity.total_cmp(&a.similarity).then(a.to.cmp(&b.to)));
        out
    }

    /// Given the set of [`ControlObjective`]s a system actually provides,
    /// returns the controls satisfied (all their objectives are covered) and
    /// the controls only partially covered, per standard.
    pub fn coverage(&self, provided: &BTreeSet<ControlObjective>) -> CoverageReport {
        let mut satisfied = Vec::new();
        let mut partial = Vec::new();
        let mut unmet = Vec::new();
        for control in self.controls.values() {
            let covered = control.objectives.intersection(provided).count();
            let total = control.objectives.len();
            if total == 0 {
                continue;
            }
            if covered == total {
                satisfied.push(control.key());
            } else if covered > 0 {
                partial.push(control.key());
            } else {
                unmet.push(control.key());
            }
        }
        satisfied.sort();
        partial.sort();
        unmet.sort();
        CoverageReport {
            provided: provided.iter().copied().collect(),
            satisfied,
            partial,
            unmet,
            total_controls: self.controls.len(),
        }
    }

    /// Returns, for each [`ControlObjective`], the number of distinct standards
    /// that have at least one control addressing it — a measure of how
    /// "load-bearing" each capability is across frameworks.
    pub fn objective_leverage(&self) -> BTreeMap<ControlObjective, usize> {
        let mut map: BTreeMap<ControlObjective, BTreeSet<Standard>> = BTreeMap::new();
        for control in self.controls.values() {
            for obj in &control.objectives {
                map.entry(*obj).or_default().insert(control.standard);
            }
        }
        map.into_iter().map(|(k, v)| (k, v.len())).collect()
    }

    /// Builds a registry pre-populated with the audit-relevant controls of the
    /// major frameworks. This is a curated, deliberately conservative mapping
    /// focused on the capabilities this audit crate provides.
    pub fn with_builtin_controls() -> Self {
        use ControlObjective::*;
        let mut m = Self::new();

        // ISO/IEC 27001 (Annex A).
        m.insert_control(Control::new(
            Standard::Iso27001,
            "A.12.4.1",
            "Event logging",
            [AuditLogging, Monitoring],
        ));
        m.insert_control(Control::new(
            Standard::Iso27001,
            "A.12.4.2",
            "Protection of log information",
            [LogIntegrity, AccessControl],
        ));
        m.insert_control(Control::new(
            Standard::Iso27001,
            "A.12.4.3",
            "Administrator and operator logs",
            [AuditLogging, SegregationOfDuties],
        ));
        m.insert_control(Control::new(
            Standard::Iso27001,
            "A.18.1.3",
            "Protection of records",
            [Retention, LogIntegrity],
        ));
        m.insert_control(Control::new(
            Standard::Iso27001,
            "A.10.1.1",
            "Policy on the use of cryptographic controls",
            [Encryption],
        ));
        m.insert_control(Control::new(
            Standard::Iso27001,
            "A.16.1.7",
            "Collection of evidence",
            [IncidentResponse, LogIntegrity],
        ));

        // SOC 2 (Common Criteria).
        m.insert_control(Control::new(
            Standard::Soc2,
            "CC7.2",
            "Detection and monitoring of anomalies",
            [Monitoring, AuditLogging],
        ));
        m.insert_control(Control::new(
            Standard::Soc2,
            "CC7.3",
            "Security incident evaluation",
            [IncidentResponse, Monitoring],
        ));
        m.insert_control(Control::new(
            Standard::Soc2,
            "CC6.1",
            "Logical access security",
            [AccessControl, Encryption],
        ));
        m.insert_control(Control::new(
            Standard::Soc2,
            "CC8.1",
            "Change management",
            [ChangeManagement, AuditLogging],
        ));

        // NIST SP 800-53 (AU / AC families).
        m.insert_control(Control::new(
            Standard::Nist80053,
            "AU-2",
            "Event logging",
            [AuditLogging],
        ));
        m.insert_control(Control::new(
            Standard::Nist80053,
            "AU-9",
            "Protection of audit information",
            [LogIntegrity, AccessControl],
        ));
        m.insert_control(Control::new(
            Standard::Nist80053,
            "AU-11",
            "Audit record retention",
            [Retention],
        ));
        m.insert_control(Control::new(
            Standard::Nist80053,
            "AU-6",
            "Audit record review, analysis, and reporting",
            [Monitoring, AuditLogging],
        ));
        m.insert_control(Control::new(
            Standard::Nist80053,
            "AC-5",
            "Separation of duties",
            [SegregationOfDuties, AccessControl],
        ));
        m.insert_control(Control::new(
            Standard::Nist80053,
            "SC-13",
            "Cryptographic protection",
            [Encryption],
        ));
        m.insert_control(Control::new(
            Standard::Nist80053,
            "IR-4",
            "Incident handling",
            [IncidentResponse],
        ));

        // GDPR.
        m.insert_control(Control::new(
            Standard::Gdpr,
            "Art.15",
            "Right of access by the data subject",
            [DataSubjectRights, AuditLogging],
        ));
        m.insert_control(Control::new(
            Standard::Gdpr,
            "Art.17",
            "Right to erasure",
            [DataSubjectRights, Retention],
        ));
        m.insert_control(Control::new(
            Standard::Gdpr,
            "Art.30",
            "Records of processing activities",
            [AuditLogging, Retention],
        ));
        m.insert_control(Control::new(
            Standard::Gdpr,
            "Art.32",
            "Security of processing",
            [Encryption, LogIntegrity, AccessControl],
        ));
        m.insert_control(Control::new(
            Standard::Gdpr,
            "Art.33",
            "Notification of a personal data breach",
            [IncidentResponse, Monitoring],
        ));

        // HIPAA (Security Rule).
        m.insert_control(Control::new(
            Standard::Hipaa,
            "164.312(b)",
            "Audit controls",
            [AuditLogging, Monitoring],
        ));
        m.insert_control(Control::new(
            Standard::Hipaa,
            "164.312(c)(1)",
            "Integrity controls",
            [LogIntegrity],
        ));
        m.insert_control(Control::new(
            Standard::Hipaa,
            "164.312(a)(1)",
            "Access control",
            [AccessControl, Encryption],
        ));
        m.insert_control(Control::new(
            Standard::Hipaa,
            "164.316(b)(2)",
            "Retention of documentation",
            [Retention],
        ));

        // SOX.
        m.insert_control(Control::new(
            Standard::Sox,
            "Sec.404",
            "Management assessment of internal controls",
            [ChangeManagement, SegregationOfDuties, AuditLogging],
        ));
        m.insert_control(Control::new(
            Standard::Sox,
            "Sec.802",
            "Criminal penalties for altering documents",
            [LogIntegrity, Retention],
        ));

        // CCPA.
        m.insert_control(Control::new(
            Standard::Ccpa,
            "1798.100",
            "Consumer right to know",
            [DataSubjectRights, AuditLogging],
        ));
        m.insert_control(Control::new(
            Standard::Ccpa,
            "1798.105",
            "Consumer right to delete",
            [DataSubjectRights, Retention],
        ));

        // PCI-DSS.
        m.insert_control(Control::new(
            Standard::PciDss,
            "Req.10",
            "Log and monitor all access to system components and cardholder data",
            [AuditLogging, Monitoring, AccessControl],
        ));
        m.insert_control(Control::new(
            Standard::PciDss,
            "Req.10.5",
            "Secure audit trails so they cannot be altered",
            [LogIntegrity],
        ));
        m.insert_control(Control::new(
            Standard::PciDss,
            "Req.3",
            "Protect stored account data",
            [Encryption, Retention],
        ));

        m
    }
}

/// The result of a coverage query over provided capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    /// The objectives the system provides.
    pub provided: Vec<ControlObjective>,
    /// Keys of controls fully satisfied.
    pub satisfied: Vec<String>,
    /// Keys of controls only partially covered.
    pub partial: Vec<String>,
    /// Keys of controls not addressed at all.
    pub unmet: Vec<String>,
    /// Total controls in the registry.
    pub total_controls: usize,
}

impl CoverageReport {
    /// Fraction of controls fully satisfied in `[0, 1]`.
    pub fn satisfied_fraction(&self) -> f64 {
        if self.total_controls == 0 {
            return 0.0;
        }
        self.satisfied.len() as f64 / self.total_controls as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ControlObjective::*;

    #[test]
    fn test_builtin_registry_populated() {
        let m = StandardMapping::with_builtin_controls();
        assert!(m.len() > 20);
        // Every supported standard has at least one control.
        for s in Standard::all() {
            assert!(
                !m.controls_for(s).is_empty(),
                "no controls for {}",
                s.code()
            );
        }
    }

    #[test]
    fn test_control_key_and_lookup() {
        let m = StandardMapping::with_builtin_controls();
        let c = m.get("NIST-800-53:AU-9").expect("AU-9");
        assert_eq!(c.standard, Standard::Nist80053);
        assert!(c.addresses(LogIntegrity));
    }

    #[test]
    fn test_controls_addressing_objective() {
        let m = StandardMapping::with_builtin_controls();
        let integrity = m.controls_addressing(LogIntegrity);
        // Multiple frameworks cover log integrity.
        let standards: BTreeSet<Standard> = integrity.iter().map(|c| c.standard).collect();
        assert!(standards.len() >= 4);
    }

    #[test]
    fn test_cross_map_finds_equivalents() {
        let m = StandardMapping::with_builtin_controls();
        // ISO event logging should cross-map to NIST AU-2 (both AuditLogging).
        let mappings = m.cross_map("ISO27001:A.12.4.1", Standard::Nist80053);
        assert!(!mappings.is_empty());
        assert!(mappings.iter().any(|cm| cm.to == "NIST-800-53:AU-2"));
        // Sorted by descending similarity.
        for pair in mappings.windows(2) {
            assert!(pair[0].similarity >= pair[1].similarity);
        }
    }

    #[test]
    fn test_cross_map_unknown_control() {
        let m = StandardMapping::with_builtin_controls();
        assert!(m.cross_map("BOGUS:X", Standard::Gdpr).is_empty());
    }

    #[test]
    fn test_coverage_full_and_partial() {
        let mut m = StandardMapping::new();
        m.insert_control(Control::new(Standard::Soc2, "X1", "x1", [AuditLogging]));
        m.insert_control(Control::new(
            Standard::Soc2,
            "X2",
            "x2",
            [AuditLogging, Encryption],
        ));
        m.insert_control(Control::new(Standard::Soc2, "X3", "x3", [Encryption]));

        let provided: BTreeSet<ControlObjective> = [AuditLogging].into_iter().collect();
        let report = m.coverage(&provided);
        assert!(report.satisfied.contains(&"SOC2:X1".to_string()));
        assert!(report.partial.contains(&"SOC2:X2".to_string()));
        assert!(report.unmet.contains(&"SOC2:X3".to_string()));
        assert!((report.satisfied_fraction() - 1.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_objective_leverage() {
        let m = StandardMapping::with_builtin_controls();
        let leverage = m.objective_leverage();
        // Audit logging is referenced by many standards.
        assert!(*leverage.get(&AuditLogging).unwrap_or(&0) >= 5);
        // Every leverage count is <= number of standards.
        for v in leverage.values() {
            assert!(*v <= Standard::all().len());
        }
    }

    #[test]
    fn test_shared_objectives_count() {
        let a = Control::new(Standard::Gdpr, "A", "a", [AuditLogging, Retention]);
        let b = Control::new(Standard::Hipaa, "B", "b", [Retention, Encryption]);
        assert_eq!(a.shared_objectives(&b), 1);
    }

    #[test]
    fn test_registry_serializes() {
        let m = StandardMapping::with_builtin_controls();
        let json = serde_json::to_string(&m).expect("serialize");
        let back: StandardMapping = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.len(), m.len());
    }

    #[test]
    fn test_standard_codes_unique() {
        let codes: BTreeSet<&str> = Standard::all().iter().map(|s| s.code()).collect();
        assert_eq!(codes.len(), Standard::all().len());
    }
}
