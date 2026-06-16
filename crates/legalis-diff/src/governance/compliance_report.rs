//! Compliance reporting for SOC 2, GDPR and HIPAA.
//!
//! This module models the *system's* compliance posture (distinct from
//! [`crate::compliance`], which assesses regulatory impact of statute *changes*).
//! Each [`ComplianceFramework`] ships a built-in [`Control`] catalogue; a
//! [`ComplianceAssessment`] records a [`ControlStatus`] (plus evidence) per
//! control, and [`generate_report`] rolls the assessment up into a scored
//! [`ComplianceReport`] with findings and recommendations.
//!
//! A [`SecurityPosture`] of simple capability flags can auto-populate the
//! assessment ([`ComplianceAssessment::assess_from_posture`]) by mapping
//! capabilities (access control, audit logging, encryption, data-subject rights,
//! breach notification, …) onto the relevant controls of each framework — tying
//! together the RBAC, audit-log and security modules.
//!
//! # Example
//!
//! ```
//! use legalis_diff::governance::compliance_report::{
//!     ComplianceAssessment, ComplianceFramework, SecurityPosture, generate_report,
//! };
//!
//! let mut assessment = ComplianceAssessment::new(ComplianceFramework::Soc2);
//! let posture = SecurityPosture::default()
//!     .with_access_control(true)
//!     .with_audit_logging(true)
//!     .with_encryption_in_transit(true);
//! assessment.assess_from_posture(&posture);
//!
//! let report = generate_report(&assessment);
//! assert_eq!(report.framework, ComplianceFramework::Soc2);
//! assert!(report.score > 0.0);
//! ```

use crate::{DiffError, DiffResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A compliance framework with a built-in control catalogue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceFramework {
    /// AICPA SOC 2 Trust Services Criteria.
    Soc2,
    /// EU General Data Protection Regulation.
    Gdpr,
    /// US HIPAA Security Rule.
    Hipaa,
    /// A custom, user-supplied framework.
    Custom(String),
}

impl ComplianceFramework {
    /// A short human-readable name.
    pub fn name(&self) -> String {
        match self {
            Self::Soc2 => "SOC 2".to_string(),
            Self::Gdpr => "GDPR".to_string(),
            Self::Hipaa => "HIPAA".to_string(),
            Self::Custom(name) => name.clone(),
        }
    }

    /// The built-in control catalogue for this framework (empty for `Custom`).
    pub fn default_controls(&self) -> Vec<Control> {
        match self {
            Self::Soc2 => vec![
                Control::new(
                    "CC6.1",
                    "Logical access controls",
                    "Access security",
                    "Restrict logical access to information assets.",
                ),
                Control::new(
                    "CC6.6",
                    "Encryption in transit",
                    "Access security",
                    "Protect data transmitted over networks.",
                ),
                Control::new(
                    "CC6.7",
                    "Encryption at rest",
                    "Access security",
                    "Protect data at rest and restrict its movement.",
                ),
                Control::new(
                    "CC7.2",
                    "System monitoring",
                    "Operations",
                    "Monitor the system and log security-relevant events.",
                ),
                Control::new(
                    "CC7.3",
                    "Incident evaluation",
                    "Operations",
                    "Evaluate detected security events for incidents.",
                ),
                Control::new(
                    "CC8.1",
                    "Change management",
                    "Change management",
                    "Authorize, design and track system changes.",
                ),
                Control::new(
                    "A1.2",
                    "Availability & retention",
                    "Availability",
                    "Retain and back up data per defined policies.",
                ),
            ],
            Self::Gdpr => vec![
                Control::new(
                    "Art5",
                    "Principles of processing",
                    "Principles",
                    "Lawfulness, minimisation and accuracy of personal data.",
                ),
                Control::new(
                    "Art15",
                    "Right of access",
                    "Data-subject rights",
                    "Provide data subjects access to their data.",
                ),
                Control::new(
                    "Art17",
                    "Right to erasure",
                    "Data-subject rights",
                    "Erase personal data on request where applicable.",
                ),
                Control::new(
                    "Art25",
                    "Data protection by design",
                    "Governance",
                    "Privacy by design and by default.",
                ),
                Control::new(
                    "Art30",
                    "Records of processing",
                    "Governance",
                    "Maintain records of processing activities.",
                ),
                Control::new(
                    "Art32",
                    "Security of processing",
                    "Security",
                    "Implement appropriate technical and organisational measures.",
                ),
                Control::new(
                    "Art33",
                    "Breach notification",
                    "Incident response",
                    "Notify breaches within 72 hours.",
                ),
            ],
            Self::Hipaa => vec![
                Control::new(
                    "164.308",
                    "Administrative safeguards",
                    "Administrative",
                    "Security management, workforce and access processes.",
                ),
                Control::new(
                    "164.310",
                    "Physical safeguards",
                    "Physical",
                    "Facility access and device/media controls.",
                ),
                Control::new(
                    "164.312(a)",
                    "Access control",
                    "Technical",
                    "Unique user IDs and access enforcement.",
                ),
                Control::new(
                    "164.312(b)",
                    "Audit controls",
                    "Technical",
                    "Record and examine activity in systems with ePHI.",
                ),
                Control::new(
                    "164.312(c)",
                    "Integrity controls",
                    "Technical",
                    "Protect ePHI from improper alteration or destruction.",
                ),
                Control::new(
                    "164.312(e)",
                    "Transmission security",
                    "Technical",
                    "Guard against unauthorized access to transmitted ePHI.",
                ),
            ],
            Self::Custom(_) => Vec::new(),
        }
    }
}

impl std::fmt::Display for ComplianceFramework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

/// A single compliance control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Control {
    /// Control identifier (e.g. `CC6.1`, `Art17`, `164.312(b)`).
    pub id: String,
    /// Short title.
    pub title: String,
    /// Category / domain.
    pub category: String,
    /// Description of the control objective.
    pub description: String,
}

impl Control {
    /// Creates a control.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        category: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            category: category.into(),
            description: description.into(),
        }
    }
}

/// The assessed status of a control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlStatus {
    /// Fully implemented and effective.
    Compliant,
    /// Partially implemented.
    PartiallyCompliant,
    /// Not implemented.
    NonCompliant,
    /// Does not apply to this system.
    NotApplicable,
    /// Not yet evaluated.
    NotAssessed,
}

impl ControlStatus {
    /// Score weight in `[0, 1]` for an *applicable* control.
    fn weight(self) -> f64 {
        match self {
            Self::Compliant => 1.0,
            Self::PartiallyCompliant => 0.5,
            Self::NonCompliant | Self::NotAssessed | Self::NotApplicable => 0.0,
        }
    }

    /// A short label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Compliant => "Compliant",
            Self::PartiallyCompliant => "Partially Compliant",
            Self::NonCompliant => "Non-Compliant",
            Self::NotApplicable => "Not Applicable",
            Self::NotAssessed => "Not Assessed",
        }
    }
}

/// The recorded assessment of one control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlAssessment {
    /// The control identifier.
    pub control_id: String,
    /// The assessed status.
    pub status: ControlStatus,
    /// Supporting evidence.
    pub evidence: Vec<String>,
    /// Optional notes.
    pub notes: Option<String>,
    /// When it was assessed.
    pub assessed_at: DateTime<Utc>,
    /// Who/what assessed it.
    pub assessor: Option<String>,
}

/// A working assessment of a framework's controls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAssessment {
    /// The framework being assessed.
    pub framework: ComplianceFramework,
    /// The controls in scope (from the catalogue or custom).
    pub controls: Vec<Control>,
    /// Per-control assessments, keyed by control id.
    pub assessments: BTreeMap<String, ControlAssessment>,
}

impl ComplianceAssessment {
    /// Creates an assessment seeded with the framework's default controls (all
    /// initially [`ControlStatus::NotAssessed`]).
    pub fn new(framework: ComplianceFramework) -> Self {
        let controls = framework.default_controls();
        Self {
            framework,
            controls,
            assessments: BTreeMap::new(),
        }
    }

    /// Adds a custom control to the scope.
    pub fn add_control(&mut self, control: Control) {
        if !self.controls.iter().any(|c| c.id == control.id) {
            self.controls.push(control);
        }
    }

    /// Records the status of a control.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::UnsupportedOperation`] if `control_id` is not in scope.
    pub fn assess(
        &mut self,
        control_id: &str,
        status: ControlStatus,
        evidence: Vec<String>,
        assessor: Option<&str>,
    ) -> DiffResult<()> {
        if !self.controls.iter().any(|c| c.id == control_id) {
            return Err(DiffError::UnsupportedOperation(format!(
                "unknown control '{control_id}' for framework {}",
                self.framework
            )));
        }
        self.assessments.insert(
            control_id.to_string(),
            ControlAssessment {
                control_id: control_id.to_string(),
                status,
                evidence,
                notes: None,
                assessed_at: Utc::now(),
                assessor: assessor.map(str::to_string),
            },
        );
        Ok(())
    }

    /// Returns the recorded status of a control (or [`ControlStatus::NotAssessed`]).
    pub fn status_of(&self, control_id: &str) -> ControlStatus {
        self.assessments
            .get(control_id)
            .map(|a| a.status)
            .unwrap_or(ControlStatus::NotAssessed)
    }

    /// Auto-populates the assessment from a [`SecurityPosture`] by mapping
    /// capability flags onto this framework's controls.
    pub fn assess_from_posture(&mut self, posture: &SecurityPosture) {
        let mappings = posture.control_statuses(&self.framework);
        for (control_id, status, evidence) in mappings {
            // Ignore errors: only known controls are produced by the mapping.
            let _ = self.assess(&control_id, status, vec![evidence], Some("auto:posture"));
        }
    }
}

/// A finding for a non- or partially-compliant control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// The control identifier.
    pub control_id: String,
    /// The control title.
    pub title: String,
    /// The assessed status.
    pub status: ControlStatus,
    /// Remediation recommendation.
    pub recommendation: String,
}

/// A generated, scored compliance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// The framework.
    pub framework: ComplianceFramework,
    /// When generated.
    pub generated_at: DateTime<Utc>,
    /// Total controls in scope.
    pub total_controls: usize,
    /// Controls that apply (excludes [`ControlStatus::NotApplicable`]).
    pub applicable_controls: usize,
    /// Count by status label.
    pub status_counts: BTreeMap<String, usize>,
    /// Compliance score in `[0, 100]` over applicable controls.
    pub score: f64,
    /// Findings for non/partially-compliant controls.
    pub findings: Vec<Finding>,
    /// Top-level recommendations.
    pub recommendations: Vec<String>,
    /// One-line summary.
    pub summary: String,
}

impl ComplianceReport {
    /// Returns `true` if the report has no open findings.
    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }

    /// Serialises the report to pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if serialisation fails.
    pub fn to_json(&self) -> DiffResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| DiffError::SerializationError(e.to_string()))
    }

    /// Renders the report as Markdown.
    pub fn to_markdown(&self) -> String {
        let mut out = format!("# {} Compliance Report\n\n", self.framework.name());
        out.push_str(&format!(
            "- Generated: {}\n- Score: {:.1}/100\n- Controls: {} ({} applicable)\n\n",
            self.generated_at.to_rfc3339(),
            self.score,
            self.total_controls,
            self.applicable_controls
        ));
        out.push_str("## Status breakdown\n\n");
        for (status, count) in &self.status_counts {
            out.push_str(&format!("- {status}: {count}\n"));
        }
        if !self.findings.is_empty() {
            out.push_str("\n## Findings\n\n");
            for finding in &self.findings {
                out.push_str(&format!(
                    "- **{}** ({}) — {}: {}\n",
                    finding.control_id,
                    finding.status.label(),
                    finding.title,
                    finding.recommendation
                ));
            }
        }
        if !self.recommendations.is_empty() {
            out.push_str("\n## Recommendations\n\n");
            for rec in &self.recommendations {
                out.push_str(&format!("- {rec}\n"));
            }
        }
        out
    }
}

/// Generates a scored report from an assessment.
pub fn generate_report(assessment: &ComplianceAssessment) -> ComplianceReport {
    let mut status_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut applicable = 0usize;
    let mut earned = 0.0f64;
    let mut findings = Vec::new();

    for control in &assessment.controls {
        let status = assessment.status_of(&control.id);
        *status_counts.entry(status.label().to_string()).or_insert(0) += 1;
        if status != ControlStatus::NotApplicable {
            applicable += 1;
            earned += status.weight();
        }
        if matches!(
            status,
            ControlStatus::NonCompliant
                | ControlStatus::PartiallyCompliant
                | ControlStatus::NotAssessed
        ) {
            let recommendation = match status {
                ControlStatus::PartiallyCompliant => {
                    format!("Complete implementation of '{}'.", control.title)
                }
                ControlStatus::NotAssessed => format!("Assess control '{}'.", control.title),
                _ => format!("Implement control '{}'.", control.title),
            };
            findings.push(Finding {
                control_id: control.id.clone(),
                title: control.title.clone(),
                status,
                recommendation,
            });
        }
    }

    let score = if applicable == 0 {
        0.0
    } else {
        (earned / applicable as f64) * 100.0
    };

    let mut recommendations = Vec::new();
    if score >= 95.0 && findings.is_empty() {
        recommendations.push("Maintain controls and schedule periodic re-assessment.".to_string());
    } else {
        if findings
            .iter()
            .any(|f| f.status == ControlStatus::NotAssessed)
        {
            recommendations
                .push("Complete assessment of all controls to obtain a full score.".to_string());
        }
        if findings
            .iter()
            .any(|f| f.status == ControlStatus::NonCompliant)
        {
            recommendations
                .push("Prioritise remediation of non-compliant controls before audit.".to_string());
        }
    }

    let summary = format!(
        "{}: {:.1}/100 over {} applicable control(s); {} finding(s).",
        assessment.framework.name(),
        score,
        applicable,
        findings.len()
    );

    ComplianceReport {
        framework: assessment.framework.clone(),
        generated_at: Utc::now(),
        total_controls: assessment.controls.len(),
        applicable_controls: applicable,
        status_counts,
        score,
        findings,
        recommendations,
        summary,
    }
}

/// A set of capability flags describing the system's security posture, used to
/// auto-populate a [`ComplianceAssessment`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityPosture {
    /// RBAC / logical access control is enforced.
    pub access_control: bool,
    /// Security-relevant events are logged.
    pub audit_logging: bool,
    /// The audit log's integrity is cryptographically verified.
    pub audit_integrity_verified: bool,
    /// Data is encrypted in transit.
    pub encryption_in_transit: bool,
    /// Data is encrypted at rest.
    pub encryption_at_rest: bool,
    /// A data-retention policy is enforced.
    pub data_retention_policy: bool,
    /// Data subjects can export/access their data.
    pub data_access_supported: bool,
    /// Data subjects' data can be erased on request.
    pub data_erasure_supported: bool,
    /// A breach-notification process exists.
    pub breach_notification: bool,
    /// A change-management process exists.
    pub change_management: bool,
}

impl SecurityPosture {
    fn status(flag: bool) -> ControlStatus {
        if flag {
            ControlStatus::Compliant
        } else {
            ControlStatus::NonCompliant
        }
    }

    /// Maps the posture onto `(control_id, status, evidence)` triples for the
    /// given framework. Only control ids present in that framework's catalogue
    /// are emitted.
    fn control_statuses(
        &self,
        framework: &ComplianceFramework,
    ) -> Vec<(String, ControlStatus, String)> {
        let mut out: Vec<(String, ControlStatus, String)> = Vec::new();
        let mut push = |id: &str, flag: bool, label: &str| {
            out.push((
                id.to_string(),
                Self::status(flag),
                format!("posture: {label}"),
            ));
        };
        match framework {
            ComplianceFramework::Soc2 => {
                push("CC6.1", self.access_control, "access control");
                push("CC6.6", self.encryption_in_transit, "encryption in transit");
                push("CC6.7", self.encryption_at_rest, "encryption at rest");
                push("CC7.2", self.audit_logging, "audit logging");
                push("CC8.1", self.change_management, "change management");
                push("A1.2", self.data_retention_policy, "retention policy");
            }
            ComplianceFramework::Gdpr => {
                push("Art15", self.data_access_supported, "data access");
                push("Art17", self.data_erasure_supported, "data erasure");
                push("Art25", self.access_control, "privacy by design");
                push("Art30", self.audit_logging, "records of processing");
                push(
                    "Art32",
                    self.encryption_in_transit && self.encryption_at_rest,
                    "security of processing",
                );
                push("Art33", self.breach_notification, "breach notification");
            }
            ComplianceFramework::Hipaa => {
                push("164.308", self.access_control, "administrative safeguards");
                push("164.312(a)", self.access_control, "access control");
                push("164.312(b)", self.audit_logging, "audit controls");
                push(
                    "164.312(c)",
                    self.audit_integrity_verified,
                    "integrity controls",
                );
                push(
                    "164.312(e)",
                    self.encryption_in_transit,
                    "transmission security",
                );
            }
            ComplianceFramework::Custom(_) => {}
        }
        out
    }

    /// Sets the access-control flag.
    #[must_use]
    pub fn with_access_control(mut self, value: bool) -> Self {
        self.access_control = value;
        self
    }

    /// Sets the audit-logging flag.
    #[must_use]
    pub fn with_audit_logging(mut self, value: bool) -> Self {
        self.audit_logging = value;
        self
    }

    /// Sets the audit-integrity-verified flag.
    #[must_use]
    pub fn with_audit_integrity_verified(mut self, value: bool) -> Self {
        self.audit_integrity_verified = value;
        self
    }

    /// Sets the encryption-in-transit flag.
    #[must_use]
    pub fn with_encryption_in_transit(mut self, value: bool) -> Self {
        self.encryption_in_transit = value;
        self
    }

    /// Sets the encryption-at-rest flag.
    #[must_use]
    pub fn with_encryption_at_rest(mut self, value: bool) -> Self {
        self.encryption_at_rest = value;
        self
    }

    /// Sets the data-retention-policy flag.
    #[must_use]
    pub fn with_data_retention_policy(mut self, value: bool) -> Self {
        self.data_retention_policy = value;
        self
    }

    /// Sets the data-access-supported flag.
    #[must_use]
    pub fn with_data_access_supported(mut self, value: bool) -> Self {
        self.data_access_supported = value;
        self
    }

    /// Sets the data-erasure-supported flag.
    #[must_use]
    pub fn with_data_erasure_supported(mut self, value: bool) -> Self {
        self.data_erasure_supported = value;
        self
    }

    /// Sets the breach-notification flag.
    #[must_use]
    pub fn with_breach_notification(mut self, value: bool) -> Self {
        self.breach_notification = value;
        self
    }

    /// Sets the change-management flag.
    #[must_use]
    pub fn with_change_management(mut self, value: bool) -> Self {
        self.change_management = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_catalogues() {
        assert_eq!(ComplianceFramework::Soc2.name(), "SOC 2");
        assert_eq!(ComplianceFramework::Soc2.default_controls().len(), 7);
        assert_eq!(ComplianceFramework::Gdpr.default_controls().len(), 7);
        assert_eq!(ComplianceFramework::Hipaa.default_controls().len(), 6);
        assert!(
            ComplianceFramework::Custom("X".into())
                .default_controls()
                .is_empty()
        );
        assert_eq!(ComplianceFramework::Custom("X".into()).name(), "X");
    }

    #[test]
    fn test_assess_and_score() {
        let mut a = ComplianceAssessment::new(ComplianceFramework::Hipaa);
        a.assess(
            "164.308",
            ControlStatus::Compliant,
            vec!["policy".into()],
            Some("auditor"),
        )
        .unwrap();
        a.assess("164.312(a)", ControlStatus::Compliant, vec![], None)
            .unwrap();
        a.assess(
            "164.312(b)",
            ControlStatus::PartiallyCompliant,
            vec![],
            None,
        )
        .unwrap();
        a.assess("164.310", ControlStatus::NotApplicable, vec![], None)
            .unwrap();
        assert_eq!(a.status_of("164.308"), ControlStatus::Compliant);
        assert!(
            a.assess("bogus", ControlStatus::Compliant, vec![], None)
                .is_err()
        );

        let report = generate_report(&a);
        // 6 controls, 1 NotApplicable -> 5 applicable. Earned = 1+1+0.5+0+0 = 2.5.
        assert_eq!(report.total_controls, 6);
        assert_eq!(report.applicable_controls, 5);
        assert!((report.score - 50.0).abs() < 1e-9);
        assert!(!report.is_clean());
    }

    #[test]
    fn test_findings_for_unassessed_and_noncompliant() {
        let mut a = ComplianceAssessment::new(ComplianceFramework::Soc2);
        a.assess("CC6.1", ControlStatus::NonCompliant, vec![], None)
            .unwrap();
        let report = generate_report(&a);
        // Every other control remains NotAssessed -> all become findings.
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.control_id == "CC6.1" && f.status == ControlStatus::NonCompliant)
        );
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.status == ControlStatus::NotAssessed)
        );
        assert!(
            report
                .recommendations
                .iter()
                .any(|r| r.contains("non-compliant"))
        );
    }

    #[test]
    fn test_assess_from_posture_soc2() {
        let mut a = ComplianceAssessment::new(ComplianceFramework::Soc2);
        let posture = SecurityPosture::default()
            .with_access_control(true)
            .with_audit_logging(true)
            .with_encryption_in_transit(true)
            .with_encryption_at_rest(true)
            .with_change_management(true)
            .with_data_retention_policy(true);
        a.assess_from_posture(&posture);
        let report = generate_report(&a);
        assert_eq!(a.status_of("CC6.1"), ControlStatus::Compliant);
        // All six mapped SOC2 controls compliant; CC7.3 remains NotAssessed.
        assert!(report.score > 80.0);
        assert!(report.score < 100.0);
    }

    #[test]
    fn test_posture_gdpr_partial_security() {
        let mut a = ComplianceAssessment::new(ComplianceFramework::Gdpr);
        let posture = SecurityPosture::default()
            .with_encryption_in_transit(true)
            .with_encryption_at_rest(false) // Art32 needs both -> non-compliant
            .with_data_erasure_supported(true);
        a.assess_from_posture(&posture);
        assert_eq!(a.status_of("Art32"), ControlStatus::NonCompliant);
        assert_eq!(a.status_of("Art17"), ControlStatus::Compliant);
    }

    #[test]
    fn test_report_json_and_markdown() {
        let mut a = ComplianceAssessment::new(ComplianceFramework::Soc2);
        a.assess("CC6.1", ControlStatus::Compliant, vec![], None)
            .unwrap();
        let report = generate_report(&a);
        let json = report.to_json().unwrap();
        assert!(json.contains("\"score\""));
        let md = report.to_markdown();
        assert!(md.contains("# SOC 2 Compliance Report"));
        assert!(md.contains("Findings"));
    }

    #[test]
    fn test_clean_report_recommendation() {
        let mut a = ComplianceAssessment::new(ComplianceFramework::Custom("Mini".into()));
        a.add_control(Control::new("X1", "Only control", "Cat", "desc"));
        a.assess("X1", ControlStatus::Compliant, vec![], None)
            .unwrap();
        let report = generate_report(&a);
        assert!(report.is_clean());
        assert!((report.score - 100.0).abs() < 1e-9);
        assert!(
            report
                .recommendations
                .iter()
                .any(|r| r.contains("Maintain"))
        );
    }
}
