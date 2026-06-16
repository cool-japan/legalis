//! Additional Compliance Frameworks and Cross-Framework Gap Analysis
//!
//! This module encodes the control families / requirements of several widely
//! used regulatory and security-assurance frameworks as *structured data* that
//! the existing compliance machinery can evaluate, and provides cross-framework
//! gap analysis to compare coverage across two or more frameworks.
//!
//! Frameworks modelled here:
//!
//! * **HIPAA** — the Health Insurance Portability and Accountability Act
//!   Security & Privacy Rules (Administrative, Physical, Technical safeguards and
//!   Privacy/Breach requirements).
//! * **PCI-DSS** — the Payment Card Industry Data Security Standard v4.0 (its 12
//!   core requirements).
//! * **FedRAMP** — the Federal Risk and Authorization Management Program control
//!   baselines, organised by the NIST SP 800-53 control families.
//! * **NIST** — the NIST Cybersecurity Framework (CSF 2.0) functions and the
//!   SP 800-53 control families they draw on.
//!
//! Each framework is described by a [`FrameworkDefinition`] containing
//! [`ControlFamily`] entries, each holding [`ControlRequirement`]s tagged with a
//! [`ControlCategory`]. The shared [`ControlCategory`] taxonomy is what makes
//! cross-framework comparison meaningful: two requirements from different
//! frameworks that share a category are treated as covering the same control
//! objective.
//!
//! The [`ComplianceFrameworkEvaluator`] evaluates a corpus of statutes against a
//! framework, reusing the same lightweight textual-evidence heuristics used by
//! the existing [`crate::certification_framework`] module. It implements the
//! [`crate::streaming_verification::ComplianceEvaluator`] trait so it can be
//! plugged directly into the continuous compliance monitor.
//!
//! # Examples
//!
//! ```
//! use legalis_verifier::compliance_frameworks::*;
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let def = framework_definition(ComplianceFrameworkKind::Hipaa);
//! assert!(!def.families.is_empty());
//!
//! let statutes = vec![Statute::new(
//!     "PHI-1",
//!     "Patient record access",
//!     Effect::new(EffectType::Grant, "Grant access to patient health records"),
//! )];
//! let evaluator = ComplianceFrameworkEvaluator::new(ComplianceFrameworkKind::Hipaa);
//! let report = evaluator.evaluate_detailed(&statutes);
//! assert_eq!(report.framework, ComplianceFrameworkKind::Hipaa);
//! ```

use crate::streaming_verification::{ComplianceEvaluator, ComplianceSnapshot};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// The compliance frameworks modelled by this module (plus references to the
/// frameworks already implemented in [`crate::certification_framework`] so that
/// gap analysis can compare against them).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceFrameworkKind {
    /// Health Insurance Portability and Accountability Act.
    Hipaa,
    /// Payment Card Industry Data Security Standard.
    PciDss,
    /// Federal Risk and Authorization Management Program.
    FedRamp,
    /// NIST Cybersecurity Framework / SP 800-53.
    Nist,
    /// General Data Protection Regulation (modelled for gap comparison).
    Gdpr,
    /// ISO/IEC 27001 (modelled for gap comparison).
    Iso27001,
    /// SOC 2 Trust Service Criteria (modelled for gap comparison).
    Soc2,
}

impl std::fmt::Display for ComplianceFrameworkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ComplianceFrameworkKind::Hipaa => "HIPAA",
            ComplianceFrameworkKind::PciDss => "PCI-DSS",
            ComplianceFrameworkKind::FedRamp => "FedRAMP",
            ComplianceFrameworkKind::Nist => "NIST",
            ComplianceFrameworkKind::Gdpr => "GDPR",
            ComplianceFrameworkKind::Iso27001 => "ISO 27001",
            ComplianceFrameworkKind::Soc2 => "SOC 2",
        };
        write!(f, "{}", s)
    }
}

/// A normalized control objective category shared across frameworks.
///
/// This is the cross-framework lingua franca: requirements from different
/// frameworks that map to the same [`ControlCategory`] are considered to address
/// the same underlying control objective. It is what lets gap analysis report
/// genuine overlaps and gaps rather than comparing incompatible vocabularies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlCategory {
    /// Governance, policy, roles and responsibilities.
    Governance,
    /// Risk assessment and management.
    RiskManagement,
    /// Identity, authentication and access control.
    AccessControl,
    /// Cryptography and protection of data at rest / in transit.
    Cryptography,
    /// Logging, monitoring, auditing and accountability.
    AuditLogging,
    /// Incident response and breach notification.
    IncidentResponse,
    /// Business continuity, disaster recovery and availability.
    Resilience,
    /// Network and communications security.
    NetworkSecurity,
    /// Physical and environmental protection.
    PhysicalSecurity,
    /// Personnel / human-resources security and awareness.
    PersonnelSecurity,
    /// Vendor, supplier and third-party management.
    VendorManagement,
    /// Data minimization, retention and lifecycle management.
    DataLifecycle,
    /// Privacy, consent and individual rights.
    Privacy,
    /// Vulnerability, patch and configuration management.
    VulnerabilityManagement,
    /// System and software development security.
    SecureDevelopment,
}

impl std::fmt::Display for ControlCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ControlCategory::Governance => "Governance",
            ControlCategory::RiskManagement => "Risk Management",
            ControlCategory::AccessControl => "Access Control",
            ControlCategory::Cryptography => "Cryptography",
            ControlCategory::AuditLogging => "Audit & Logging",
            ControlCategory::IncidentResponse => "Incident Response",
            ControlCategory::Resilience => "Resilience & Availability",
            ControlCategory::NetworkSecurity => "Network Security",
            ControlCategory::PhysicalSecurity => "Physical Security",
            ControlCategory::PersonnelSecurity => "Personnel Security",
            ControlCategory::VendorManagement => "Vendor Management",
            ControlCategory::DataLifecycle => "Data Lifecycle",
            ControlCategory::Privacy => "Privacy",
            ControlCategory::VulnerabilityManagement => "Vulnerability Management",
            ControlCategory::SecureDevelopment => "Secure Development",
        };
        write!(f, "{}", s)
    }
}

/// A single, atomic control requirement within a framework.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlRequirement {
    /// Framework-native identifier (e.g. `"164.312(a)(1)"`, `"PCI-3"`, `"AC-2"`).
    pub id: String,
    /// Short human-readable title.
    pub title: String,
    /// Normalized cross-framework category.
    pub category: ControlCategory,
    /// Keywords that, when present in a statute's text, are taken as positive
    /// evidence that the statute addresses this requirement.
    pub evidence_keywords: Vec<String>,
    /// Whether this requirement is mandatory (vs. addressable/advisory).
    pub mandatory: bool,
}

impl ControlRequirement {
    /// Builds a mandatory requirement.
    pub fn required(
        id: impl Into<String>,
        title: impl Into<String>,
        category: ControlCategory,
        keywords: &[&str],
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            category,
            evidence_keywords: keywords.iter().map(|k| k.to_string()).collect(),
            mandatory: true,
        }
    }

    /// Builds an addressable (non-mandatory) requirement.
    pub fn addressable(
        id: impl Into<String>,
        title: impl Into<String>,
        category: ControlCategory,
        keywords: &[&str],
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            category,
            evidence_keywords: keywords.iter().map(|k| k.to_string()).collect(),
            mandatory: false,
        }
    }
}

/// A named family / group of related control requirements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlFamily {
    /// Family identifier (e.g. `"Administrative Safeguards"`, `"AC"`).
    pub id: String,
    /// Family title / description.
    pub title: String,
    /// Requirements belonging to this family.
    pub requirements: Vec<ControlRequirement>,
}

impl ControlFamily {
    /// Returns the total number of requirements in this family.
    pub fn requirement_count(&self) -> usize {
        self.requirements.len()
    }

    /// Returns the number of mandatory requirements in this family.
    pub fn mandatory_count(&self) -> usize {
        self.requirements.iter().filter(|r| r.mandatory).count()
    }
}

/// A complete structured definition of a compliance framework.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkDefinition {
    /// Which framework this is.
    pub kind: ComplianceFrameworkKind,
    /// Display name.
    pub name: String,
    /// Version / revision (e.g. `"v4.0"`, `"SP 800-53 Rev. 5"`).
    pub version: String,
    /// Control families that make up the framework.
    pub families: Vec<ControlFamily>,
}

impl FrameworkDefinition {
    /// Iterates over every requirement across all families.
    pub fn all_requirements(&self) -> impl Iterator<Item = &ControlRequirement> {
        self.families.iter().flat_map(|fam| fam.requirements.iter())
    }

    /// Total requirement count across the whole framework.
    pub fn requirement_count(&self) -> usize {
        self.families.iter().map(|f| f.requirement_count()).sum()
    }

    /// The set of distinct control categories the framework covers.
    pub fn categories(&self) -> HashSet<ControlCategory> {
        self.all_requirements().map(|r| r.category).collect()
    }
}

/// Status of a single requirement after evaluation against a corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequirementStatus {
    /// At least one statute provides evidence for this requirement.
    Satisfied,
    /// Relevant statutes exist but lack the expected safeguards.
    PartiallySatisfied,
    /// No statute addresses this requirement at all.
    NotAddressed,
}

impl std::fmt::Display for RequirementStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequirementStatus::Satisfied => write!(f, "Satisfied"),
            RequirementStatus::PartiallySatisfied => write!(f, "Partially Satisfied"),
            RequirementStatus::NotAddressed => write!(f, "Not Addressed"),
        }
    }
}

/// Per-requirement evaluation outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementEvaluation {
    /// The requirement evaluated.
    pub requirement_id: String,
    /// Requirement title (copied for convenience in reports).
    pub title: String,
    /// Normalized category.
    pub category: ControlCategory,
    /// Whether the requirement is mandatory.
    pub mandatory: bool,
    /// Evaluation status.
    pub status: RequirementStatus,
    /// IDs of statutes that provided supporting evidence.
    pub supporting_statutes: Vec<String>,
    /// Remediation note when not satisfied.
    pub note: String,
}

/// Full structured result of evaluating a corpus against one framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkComplianceReport {
    /// Framework evaluated.
    pub framework: ComplianceFrameworkKind,
    /// Framework version.
    pub version: String,
    /// Overall compliance score in 0-100.
    pub score: f64,
    /// Whether the corpus is considered compliant (score >= threshold).
    pub compliant: bool,
    /// Compliance threshold used (0-100).
    pub threshold: f64,
    /// Per-requirement evaluations.
    pub requirements: Vec<RequirementEvaluation>,
}

impl FrameworkComplianceReport {
    /// Number of fully satisfied requirements.
    pub fn satisfied_count(&self) -> usize {
        self.requirements
            .iter()
            .filter(|r| r.status == RequirementStatus::Satisfied)
            .count()
    }

    /// Number of requirements with violations (partial or not addressed)
    /// counting only mandatory ones.
    pub fn violation_count(&self) -> usize {
        self.requirements
            .iter()
            .filter(|r| r.mandatory && r.status != RequirementStatus::Satisfied)
            .count()
    }

    /// Requirements that were not addressed at all.
    pub fn unaddressed(&self) -> Vec<&RequirementEvaluation> {
        self.requirements
            .iter()
            .filter(|r| r.status == RequirementStatus::NotAddressed)
            .collect()
    }

    /// Renders a Markdown compliance report.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {} Compliance Report\n\n", self.framework));
        out.push_str(&format!("- Version: {}\n", self.version));
        out.push_str(&format!("- Score: {:.1}%\n", self.score));
        out.push_str(&format!(
            "- Status: {}\n",
            if self.compliant {
                "COMPLIANT"
            } else {
                "NON-COMPLIANT"
            }
        ));
        out.push_str(&format!(
            "- Satisfied: {}/{}\n",
            self.satisfied_count(),
            self.requirements.len()
        ));
        out.push_str(&format!(
            "- Mandatory violations: {}\n\n",
            self.violation_count()
        ));

        let unaddressed = self.unaddressed();
        if !unaddressed.is_empty() {
            out.push_str("## Unaddressed Requirements\n\n");
            for r in unaddressed {
                out.push_str(&format!(
                    "- [{}] {} ({}){}\n",
                    r.requirement_id,
                    r.title,
                    r.category,
                    if r.mandatory { " — mandatory" } else { "" }
                ));
            }
            out.push('\n');
        }

        let partial: Vec<&RequirementEvaluation> = self
            .requirements
            .iter()
            .filter(|r| r.status == RequirementStatus::PartiallySatisfied)
            .collect();
        if !partial.is_empty() {
            out.push_str("## Partially Satisfied Requirements\n\n");
            for r in partial {
                out.push_str(&format!(
                    "- [{}] {} — {}\n",
                    r.requirement_id, r.title, r.note
                ));
            }
        }
        out
    }
}

/// Returns the structured definition for a framework.
pub fn framework_definition(kind: ComplianceFrameworkKind) -> FrameworkDefinition {
    match kind {
        ComplianceFrameworkKind::Hipaa => hipaa_definition(),
        ComplianceFrameworkKind::PciDss => pci_dss_definition(),
        ComplianceFrameworkKind::FedRamp => fedramp_definition(),
        ComplianceFrameworkKind::Nist => nist_definition(),
        ComplianceFrameworkKind::Gdpr => gdpr_definition(),
        ComplianceFrameworkKind::Iso27001 => iso27001_definition(),
        ComplianceFrameworkKind::Soc2 => soc2_definition(),
    }
}

// ===========================================================================
// HIPAA Security & Privacy Rules
// ===========================================================================

fn hipaa_definition() -> FrameworkDefinition {
    use ControlCategory as C;
    FrameworkDefinition {
        kind: ComplianceFrameworkKind::Hipaa,
        name: "Health Insurance Portability and Accountability Act".to_string(),
        version: "Security Rule 45 CFR 164".to_string(),
        families: vec![
            ControlFamily {
                id: "164.308".to_string(),
                title: "Administrative Safeguards".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "164.308(a)(1)(ii)(A)",
                        "Risk analysis",
                        C::RiskManagement,
                        &["risk", "assessment", "analysis"],
                    ),
                    ControlRequirement::required(
                        "164.308(a)(1)(ii)(B)",
                        "Risk management",
                        C::RiskManagement,
                        &["risk", "mitigat", "manage"],
                    ),
                    ControlRequirement::required(
                        "164.308(a)(3)",
                        "Workforce security",
                        C::PersonnelSecurity,
                        &["workforce", "personnel", "employee", "authoriz"],
                    ),
                    ControlRequirement::required(
                        "164.308(a)(4)",
                        "Information access management",
                        C::AccessControl,
                        &["access", "authoriz", "role"],
                    ),
                    ControlRequirement::addressable(
                        "164.308(a)(5)",
                        "Security awareness and training",
                        C::PersonnelSecurity,
                        &["training", "awareness", "education"],
                    ),
                    ControlRequirement::required(
                        "164.308(a)(6)",
                        "Security incident procedures",
                        C::IncidentResponse,
                        &["incident", "breach", "respond"],
                    ),
                    ControlRequirement::required(
                        "164.308(a)(7)",
                        "Contingency plan",
                        C::Resilience,
                        &["contingency", "backup", "recovery", "continuity"],
                    ),
                    ControlRequirement::required(
                        "164.308(b)(1)",
                        "Business associate contracts",
                        C::VendorManagement,
                        &["business associate", "contract", "vendor", "third party"],
                    ),
                ],
            },
            ControlFamily {
                id: "164.310".to_string(),
                title: "Physical Safeguards".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "164.310(a)(1)",
                        "Facility access controls",
                        C::PhysicalSecurity,
                        &["facility", "physical", "premises"],
                    ),
                    ControlRequirement::addressable(
                        "164.310(d)(1)",
                        "Device and media controls",
                        C::PhysicalSecurity,
                        &["device", "media", "disposal", "hardware"],
                    ),
                ],
            },
            ControlFamily {
                id: "164.312".to_string(),
                title: "Technical Safeguards".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "164.312(a)(1)",
                        "Access control",
                        C::AccessControl,
                        &["access", "authentication", "unique user", "role"],
                    ),
                    ControlRequirement::required(
                        "164.312(b)",
                        "Audit controls",
                        C::AuditLogging,
                        &["audit", "log", "monitor", "record"],
                    ),
                    ControlRequirement::required(
                        "164.312(c)(1)",
                        "Integrity",
                        C::Cryptography,
                        &["integrity", "tamper", "alteration", "hash"],
                    ),
                    ControlRequirement::addressable(
                        "164.312(e)(1)",
                        "Transmission security (encryption)",
                        C::Cryptography,
                        &["encrypt", "transmission", "secure channel", "tls"],
                    ),
                ],
            },
            ControlFamily {
                id: "164.5xx".to_string(),
                title: "Privacy & Breach Notification".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "164.502",
                        "Uses and disclosures of PHI",
                        C::Privacy,
                        &["consent", "disclosure", "authoriz", "minimum necessary"],
                    ),
                    ControlRequirement::required(
                        "164.524",
                        "Individual right of access",
                        C::Privacy,
                        &["right", "access", "individual", "patient"],
                    ),
                    ControlRequirement::required(
                        "164.530(j)",
                        "Retention of records",
                        C::DataLifecycle,
                        &["retention", "retain", "period", "delete"],
                    ),
                    ControlRequirement::required(
                        "164.404",
                        "Breach notification to individuals",
                        C::IncidentResponse,
                        &["breach", "notif", "notify"],
                    ),
                ],
            },
        ],
    }
}

// ===========================================================================
// PCI-DSS v4.0 (12 core requirements)
// ===========================================================================

fn pci_dss_definition() -> FrameworkDefinition {
    use ControlCategory as C;
    FrameworkDefinition {
        kind: ComplianceFrameworkKind::PciDss,
        name: "Payment Card Industry Data Security Standard".to_string(),
        version: "v4.0".to_string(),
        families: vec![
            ControlFamily {
                id: "build-maintain".to_string(),
                title: "Build and Maintain a Secure Network and Systems".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "PCI-1",
                        "Install and maintain network security controls",
                        C::NetworkSecurity,
                        &["firewall", "network", "segmentation"],
                    ),
                    ControlRequirement::required(
                        "PCI-2",
                        "Apply secure configurations to all system components",
                        C::VulnerabilityManagement,
                        &["configuration", "hardening", "default password", "baseline"],
                    ),
                ],
            },
            ControlFamily {
                id: "protect-data".to_string(),
                title: "Protect Account Data".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "PCI-3",
                        "Protect stored account data",
                        C::Cryptography,
                        &["encrypt", "stored", "account data", "cardholder"],
                    ),
                    ControlRequirement::required(
                        "PCI-4",
                        "Protect cardholder data with strong cryptography during transmission",
                        C::Cryptography,
                        &["encrypt", "transmission", "tls", "secure channel"],
                    ),
                ],
            },
            ControlFamily {
                id: "vuln-mgmt".to_string(),
                title: "Maintain a Vulnerability Management Program".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "PCI-5",
                        "Protect systems and networks from malicious software",
                        C::VulnerabilityManagement,
                        &["malware", "antivirus", "malicious"],
                    ),
                    ControlRequirement::required(
                        "PCI-6",
                        "Develop and maintain secure systems and software",
                        C::SecureDevelopment,
                        &["secure development", "patch", "vulnerability", "software"],
                    ),
                ],
            },
            ControlFamily {
                id: "access".to_string(),
                title: "Implement Strong Access Control Measures".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "PCI-7",
                        "Restrict access to system components by business need to know",
                        C::AccessControl,
                        &["access", "need to know", "least privilege", "role"],
                    ),
                    ControlRequirement::required(
                        "PCI-8",
                        "Identify users and authenticate access",
                        C::AccessControl,
                        &["authentication", "identity", "mfa", "unique"],
                    ),
                    ControlRequirement::required(
                        "PCI-9",
                        "Restrict physical access to cardholder data",
                        C::PhysicalSecurity,
                        &["physical", "facility", "premises"],
                    ),
                ],
            },
            ControlFamily {
                id: "monitor-test".to_string(),
                title: "Regularly Monitor and Test Networks".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "PCI-10",
                        "Log and monitor all access to system components and cardholder data",
                        C::AuditLogging,
                        &["log", "audit", "monitor", "track"],
                    ),
                    ControlRequirement::required(
                        "PCI-11",
                        "Test security of systems and networks regularly",
                        C::VulnerabilityManagement,
                        &["penetration", "scan", "test", "vulnerability"],
                    ),
                ],
            },
            ControlFamily {
                id: "policy".to_string(),
                title: "Maintain an Information Security Policy".to_string(),
                requirements: vec![ControlRequirement::required(
                    "PCI-12",
                    "Support information security with organizational policies and programs",
                    C::Governance,
                    &["policy", "program", "governance", "responsibility"],
                )],
            },
        ],
    }
}

// ===========================================================================
// FedRAMP (NIST SP 800-53 control families)
// ===========================================================================

fn fedramp_definition() -> FrameworkDefinition {
    use ControlCategory as C;
    FrameworkDefinition {
        kind: ComplianceFrameworkKind::FedRamp,
        name: "Federal Risk and Authorization Management Program".to_string(),
        version: "SP 800-53 Rev. 5 (Moderate baseline)".to_string(),
        families: vec![
            ControlFamily {
                id: "AC".to_string(),
                title: "Access Control".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "AC-2",
                        "Account management",
                        C::AccessControl,
                        &["account", "access", "authoriz", "role"],
                    ),
                    ControlRequirement::required(
                        "AC-3",
                        "Access enforcement",
                        C::AccessControl,
                        &["access", "enforce", "least privilege"],
                    ),
                ],
            },
            ControlFamily {
                id: "AU".to_string(),
                title: "Audit and Accountability".to_string(),
                requirements: vec![ControlRequirement::required(
                    "AU-2",
                    "Event logging",
                    C::AuditLogging,
                    &["log", "audit", "event", "monitor"],
                )],
            },
            ControlFamily {
                id: "CA".to_string(),
                title: "Assessment, Authorization, and Monitoring".to_string(),
                requirements: vec![ControlRequirement::required(
                    "CA-7",
                    "Continuous monitoring",
                    C::AuditLogging,
                    &["continuous", "monitor", "ongoing"],
                )],
            },
            ControlFamily {
                id: "CM".to_string(),
                title: "Configuration Management".to_string(),
                requirements: vec![ControlRequirement::required(
                    "CM-2",
                    "Baseline configuration",
                    C::VulnerabilityManagement,
                    &["configuration", "baseline", "hardening"],
                )],
            },
            ControlFamily {
                id: "CP".to_string(),
                title: "Contingency Planning".to_string(),
                requirements: vec![ControlRequirement::required(
                    "CP-9",
                    "System backup",
                    C::Resilience,
                    &["backup", "recovery", "contingency", "continuity"],
                )],
            },
            ControlFamily {
                id: "IA".to_string(),
                title: "Identification and Authentication".to_string(),
                requirements: vec![ControlRequirement::required(
                    "IA-2",
                    "Identification and authentication (organizational users)",
                    C::AccessControl,
                    &["authentication", "identity", "mfa", "credential"],
                )],
            },
            ControlFamily {
                id: "IR".to_string(),
                title: "Incident Response".to_string(),
                requirements: vec![ControlRequirement::required(
                    "IR-4",
                    "Incident handling",
                    C::IncidentResponse,
                    &["incident", "respond", "breach"],
                )],
            },
            ControlFamily {
                id: "RA".to_string(),
                title: "Risk Assessment".to_string(),
                requirements: vec![ControlRequirement::required(
                    "RA-3",
                    "Risk assessment",
                    C::RiskManagement,
                    &["risk", "assessment", "analysis"],
                )],
            },
            ControlFamily {
                id: "SC".to_string(),
                title: "System and Communications Protection".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "SC-13",
                        "Cryptographic protection",
                        C::Cryptography,
                        &["encrypt", "cryptograph", "fips"],
                    ),
                    ControlRequirement::required(
                        "SC-7",
                        "Boundary protection",
                        C::NetworkSecurity,
                        &["boundary", "firewall", "network", "segmentation"],
                    ),
                ],
            },
            ControlFamily {
                id: "PE".to_string(),
                title: "Physical and Environmental Protection".to_string(),
                requirements: vec![ControlRequirement::required(
                    "PE-3",
                    "Physical access control",
                    C::PhysicalSecurity,
                    &["physical", "facility", "premises"],
                )],
            },
        ],
    }
}

// ===========================================================================
// NIST Cybersecurity Framework (CSF 2.0 functions)
// ===========================================================================

fn nist_definition() -> FrameworkDefinition {
    use ControlCategory as C;
    FrameworkDefinition {
        kind: ComplianceFrameworkKind::Nist,
        name: "NIST Cybersecurity Framework".to_string(),
        version: "CSF 2.0".to_string(),
        families: vec![
            ControlFamily {
                id: "GV".to_string(),
                title: "Govern".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "GV.OC",
                        "Organizational context and policy",
                        C::Governance,
                        &["policy", "governance", "responsibility", "oversight"],
                    ),
                    ControlRequirement::required(
                        "GV.SC",
                        "Cybersecurity supply chain risk management",
                        C::VendorManagement,
                        &["supply chain", "vendor", "third party", "supplier"],
                    ),
                ],
            },
            ControlFamily {
                id: "ID".to_string(),
                title: "Identify".to_string(),
                requirements: vec![ControlRequirement::required(
                    "ID.RA",
                    "Risk assessment",
                    C::RiskManagement,
                    &["risk", "assessment", "analysis", "threat"],
                )],
            },
            ControlFamily {
                id: "PR".to_string(),
                title: "Protect".to_string(),
                requirements: vec![
                    ControlRequirement::required(
                        "PR.AA",
                        "Identity management, authentication and access control",
                        C::AccessControl,
                        &["access", "authentication", "identity", "authoriz"],
                    ),
                    ControlRequirement::required(
                        "PR.DS",
                        "Data security (encryption)",
                        C::Cryptography,
                        &["encrypt", "data security", "integrity", "cryptograph"],
                    ),
                    ControlRequirement::addressable(
                        "PR.AT",
                        "Awareness and training",
                        C::PersonnelSecurity,
                        &["training", "awareness"],
                    ),
                ],
            },
            ControlFamily {
                id: "DE".to_string(),
                title: "Detect".to_string(),
                requirements: vec![ControlRequirement::required(
                    "DE.CM",
                    "Continuous monitoring",
                    C::AuditLogging,
                    &["monitor", "log", "detect", "audit"],
                )],
            },
            ControlFamily {
                id: "RS".to_string(),
                title: "Respond".to_string(),
                requirements: vec![ControlRequirement::required(
                    "RS.MA",
                    "Incident management",
                    C::IncidentResponse,
                    &["incident", "respond", "breach"],
                )],
            },
            ControlFamily {
                id: "RC".to_string(),
                title: "Recover".to_string(),
                requirements: vec![ControlRequirement::required(
                    "RC.RP",
                    "Incident recovery plan execution",
                    C::Resilience,
                    &["recovery", "restore", "continuity", "backup"],
                )],
            },
        ],
    }
}

// ===========================================================================
// Reference definitions for frameworks already implemented elsewhere.
// These are intentionally compact - they exist so gap analysis can compare
// category coverage against HIPAA / PCI-DSS / FedRAMP / NIST.
// ===========================================================================

fn gdpr_definition() -> FrameworkDefinition {
    use ControlCategory as C;
    FrameworkDefinition {
        kind: ComplianceFrameworkKind::Gdpr,
        name: "General Data Protection Regulation".to_string(),
        version: "Regulation (EU) 2016/679".to_string(),
        families: vec![ControlFamily {
            id: "principles".to_string(),
            title: "Data Protection Principles".to_string(),
            requirements: vec![
                ControlRequirement::required(
                    "Art.6",
                    "Lawful basis for processing",
                    C::Privacy,
                    &["consent", "legal basis", "lawful"],
                ),
                ControlRequirement::required(
                    "Art.5(1)(e)",
                    "Storage limitation",
                    C::DataLifecycle,
                    &["retention", "delete", "period"],
                ),
                ControlRequirement::required(
                    "Art.32",
                    "Security of processing",
                    C::Cryptography,
                    &["encrypt", "secure", "protect"],
                ),
                ControlRequirement::required(
                    "Art.33",
                    "Breach notification",
                    C::IncidentResponse,
                    &["breach", "notif", "notify"],
                ),
                ControlRequirement::required(
                    "Art.30",
                    "Records of processing / accountability",
                    C::AuditLogging,
                    &["log", "record", "audit"],
                ),
            ],
        }],
    }
}

fn iso27001_definition() -> FrameworkDefinition {
    use ControlCategory as C;
    FrameworkDefinition {
        kind: ComplianceFrameworkKind::Iso27001,
        name: "ISO/IEC 27001".to_string(),
        version: "2022".to_string(),
        families: vec![ControlFamily {
            id: "annexA".to_string(),
            title: "Annex A Controls".to_string(),
            requirements: vec![
                ControlRequirement::required(
                    "A.5",
                    "Organizational controls / policy",
                    C::Governance,
                    &["policy", "governance"],
                ),
                ControlRequirement::required(
                    "A.8.AC",
                    "Access control",
                    C::AccessControl,
                    &["access", "authoriz"],
                ),
                ControlRequirement::required(
                    "A.8.CRY",
                    "Cryptography",
                    C::Cryptography,
                    &["encrypt", "cryptograph"],
                ),
                ControlRequirement::required(
                    "A.8.LOG",
                    "Logging and monitoring",
                    C::AuditLogging,
                    &["log", "monitor", "audit"],
                ),
                ControlRequirement::required(
                    "A.5.IR",
                    "Incident management",
                    C::IncidentResponse,
                    &["incident", "breach"],
                ),
                ControlRequirement::required(
                    "A.5.SUP",
                    "Supplier relationships",
                    C::VendorManagement,
                    &["supplier", "vendor", "third party"],
                ),
                ControlRequirement::required(
                    "A.5.CONT",
                    "Business continuity",
                    C::Resilience,
                    &["continuity", "recovery", "backup"],
                ),
            ],
        }],
    }
}

fn soc2_definition() -> FrameworkDefinition {
    use ControlCategory as C;
    FrameworkDefinition {
        kind: ComplianceFrameworkKind::Soc2,
        name: "SOC 2 Trust Service Criteria".to_string(),
        version: "2017 TSC".to_string(),
        families: vec![ControlFamily {
            id: "TSC".to_string(),
            title: "Trust Service Criteria".to_string(),
            requirements: vec![
                ControlRequirement::required(
                    "CC6",
                    "Logical and physical access controls",
                    C::AccessControl,
                    &["access", "authoriz"],
                ),
                ControlRequirement::required(
                    "CC7",
                    "System operations / monitoring",
                    C::AuditLogging,
                    &["monitor", "log"],
                ),
                ControlRequirement::required(
                    "A1",
                    "Availability",
                    C::Resilience,
                    &["availab", "uptime", "redundan"],
                ),
                ControlRequirement::required(
                    "C1",
                    "Confidentiality (encryption)",
                    C::Cryptography,
                    &["encrypt", "confidential"],
                ),
                ControlRequirement::required(
                    "P1",
                    "Privacy",
                    C::Privacy,
                    &["privacy", "consent", "personal"],
                ),
            ],
        }],
    }
}

// ===========================================================================
// Evaluator
// ===========================================================================

/// Evaluates a corpus of statutes against a compliance framework.
///
/// Reuses the lightweight textual-evidence approach already used by
/// [`crate::certification_framework`]: a statute is treated as *relevant* to a
/// requirement's category if its text mentions the category's domain, and as
/// *supporting* the requirement if it additionally contains one of the
/// requirement's evidence keywords.
#[derive(Debug, Clone)]
pub struct ComplianceFrameworkEvaluator {
    definition: FrameworkDefinition,
    /// Compliance threshold (0-100). A corpus scoring at or above this is
    /// considered compliant.
    threshold: f64,
}

impl ComplianceFrameworkEvaluator {
    /// Creates an evaluator for the given framework with the default threshold
    /// (75%).
    pub fn new(kind: ComplianceFrameworkKind) -> Self {
        Self {
            definition: framework_definition(kind),
            threshold: 75.0,
        }
    }

    /// Overrides the compliance threshold (clamped to 0-100).
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.clamp(0.0, 100.0);
        self
    }

    /// Returns the underlying framework definition.
    pub fn definition(&self) -> &FrameworkDefinition {
        &self.definition
    }

    /// The framework kind being evaluated.
    pub fn kind(&self) -> ComplianceFrameworkKind {
        self.definition.kind
    }

    /// Evaluates the corpus and returns a full structured report.
    pub fn evaluate_detailed(&self, statutes: &[Statute]) -> FrameworkComplianceReport {
        // Pre-compute the lowercased searchable text for each statute once.
        let texts: Vec<(String, String)> = statutes
            .iter()
            .map(|s| (s.id.clone(), statute_text(s)))
            .collect();

        let mut evaluations = Vec::new();
        let mut satisfied_weight = 0.0;
        let mut total_weight = 0.0;

        for req in self.definition.all_requirements() {
            // A requirement is mandatory -> weight 1.0, addressable -> 0.5.
            let weight = if req.mandatory { 1.0 } else { 0.5 };
            total_weight += weight;

            let category_terms = category_domain_terms(req.category);

            let mut supporting = Vec::new();
            let mut relevant = false;

            for (id, text) in &texts {
                let mentions_domain = category_terms.iter().any(|t| text.contains(t));
                let has_evidence = req
                    .evidence_keywords
                    .iter()
                    .any(|k| text.contains(&k.to_lowercase()));

                if has_evidence {
                    supporting.push(id.clone());
                } else if mentions_domain {
                    relevant = true;
                }
            }

            let (status, note) = if !supporting.is_empty() {
                satisfied_weight += weight;
                (RequirementStatus::Satisfied, String::new())
            } else if relevant {
                // Half credit for relevant-but-unsubstantiated coverage.
                satisfied_weight += weight * 0.5;
                (
                    RequirementStatus::PartiallySatisfied,
                    format!(
                        "Statutes touch {} but lack explicit evidence ({})",
                        req.category,
                        req.evidence_keywords.join(", ")
                    ),
                )
            } else {
                (
                    RequirementStatus::NotAddressed,
                    format!("No statute addresses {} ({})", req.title, req.category),
                )
            };

            evaluations.push(RequirementEvaluation {
                requirement_id: req.id.clone(),
                title: req.title.clone(),
                category: req.category,
                mandatory: req.mandatory,
                status,
                supporting_statutes: supporting,
                note,
            });
        }

        let score = if total_weight > 0.0 {
            (satisfied_weight / total_weight) * 100.0
        } else {
            100.0
        };

        FrameworkComplianceReport {
            framework: self.definition.kind,
            version: self.definition.version.clone(),
            score,
            compliant: score >= self.threshold,
            threshold: self.threshold,
            requirements: evaluations,
        }
    }
}

impl ComplianceEvaluator for ComplianceFrameworkEvaluator {
    fn name(&self) -> String {
        self.definition.kind.to_string()
    }

    fn evaluate(&self, statutes: &[Statute]) -> ComplianceSnapshot {
        let report = self.evaluate_detailed(statutes);
        ComplianceSnapshot {
            framework: report.framework.to_string(),
            score: report.score,
            compliant: report.compliant,
            violation_count: report.violation_count(),
        }
    }
}

/// Builds the lowercased searchable text for a statute (title + effect
/// description + discretion logic + condition debug strings).
fn statute_text(statute: &Statute) -> String {
    let mut text = String::new();
    text.push_str(&statute.title);
    text.push(' ');
    text.push_str(&statute.effect.description);
    if let Some(d) = &statute.discretion_logic {
        text.push(' ');
        text.push_str(d);
    }
    for c in &statute.preconditions {
        text.push(' ');
        text.push_str(&format!("{:?}", c));
    }
    for (k, v) in &statute.effect.parameters {
        text.push(' ');
        text.push_str(k);
        text.push(' ');
        text.push_str(v);
    }
    text.to_lowercase()
}

/// Domain terms that signal a statute is *relevant* to a control category, even
/// if it does not fully satisfy the requirement. Used to distinguish "partially
/// addressed" from "not addressed at all".
fn category_domain_terms(category: ControlCategory) -> &'static [&'static str] {
    match category {
        ControlCategory::Governance => &["policy", "govern", "responsib", "oversight"],
        ControlCategory::RiskManagement => &["risk", "threat", "assess"],
        ControlCategory::AccessControl => &["access", "authoriz", "permission", "role", "login"],
        ControlCategory::Cryptography => &[
            "encrypt",
            "cryptograph",
            "confidential",
            "integrity",
            "data",
        ],
        ControlCategory::AuditLogging => &["log", "audit", "monitor", "record", "track"],
        ControlCategory::IncidentResponse => &["incident", "breach", "respond", "notif"],
        ControlCategory::Resilience => {
            &["backup", "recovery", "continuity", "contingency", "availab"]
        }
        ControlCategory::NetworkSecurity => &["network", "firewall", "boundary", "segment"],
        ControlCategory::PhysicalSecurity => &["physical", "facility", "premises", "device"],
        ControlCategory::PersonnelSecurity => &[
            "personnel",
            "workforce",
            "employee",
            "training",
            "awareness",
        ],
        ControlCategory::VendorManagement => &[
            "vendor",
            "supplier",
            "third party",
            "business associate",
            "supply chain",
        ],
        ControlCategory::DataLifecycle => &["retention", "retain", "delete", "disposal", "period"],
        ControlCategory::Privacy => &["privacy", "personal", "consent", "patient", "disclosure"],
        ControlCategory::VulnerabilityManagement => {
            &["vulnerab", "patch", "malware", "configuration", "scan"]
        }
        ControlCategory::SecureDevelopment => &["development", "software", "code", "secure coding"],
    }
}

// ===========================================================================
// Cross-Framework Gap Analysis
// ===========================================================================

/// How a control category is covered across the compared frameworks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoverageKind {
    /// Every compared framework covers this category.
    Universal,
    /// Some, but not all, of the compared frameworks cover it.
    Partial,
    /// Only a single framework covers it.
    Unique,
}

impl std::fmt::Display for CoverageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoverageKind::Universal => write!(f, "Universal"),
            CoverageKind::Partial => write!(f, "Partial"),
            CoverageKind::Unique => write!(f, "Unique"),
        }
    }
}

/// Coverage of a single control category across the compared frameworks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCoverage {
    /// The control category.
    pub category: ControlCategory,
    /// Frameworks that cover this category.
    pub covered_by: Vec<ComplianceFrameworkKind>,
    /// Frameworks that do NOT cover this category (the gaps).
    pub missing_from: Vec<ComplianceFrameworkKind>,
    /// Classification of the coverage.
    pub coverage_kind: CoverageKind,
}

/// Result of comparing two or more frameworks' coverage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossFrameworkGapAnalysis {
    /// Frameworks that were compared.
    pub frameworks: Vec<ComplianceFrameworkKind>,
    /// Per-category coverage breakdown (sorted by category name).
    pub coverage: Vec<CategoryCoverage>,
}

impl CrossFrameworkGapAnalysis {
    /// Categories covered by every compared framework (full overlap).
    pub fn universal_categories(&self) -> Vec<ControlCategory> {
        self.coverage
            .iter()
            .filter(|c| c.coverage_kind == CoverageKind::Universal)
            .map(|c| c.category)
            .collect()
    }

    /// Categories covered by exactly one framework (gaps in the others).
    pub fn unique_categories(&self) -> Vec<&CategoryCoverage> {
        self.coverage
            .iter()
            .filter(|c| c.coverage_kind == CoverageKind::Unique)
            .collect()
    }

    /// Categories that some framework is missing entirely.
    pub fn gap_categories(&self) -> Vec<&CategoryCoverage> {
        self.coverage
            .iter()
            .filter(|c| !c.missing_from.is_empty())
            .collect()
    }

    /// Returns, for the given framework, the categories it does not cover that
    /// at least one other compared framework does (its specific blind spots).
    pub fn gaps_for(&self, framework: ComplianceFrameworkKind) -> Vec<ControlCategory> {
        self.coverage
            .iter()
            .filter(|c| c.missing_from.contains(&framework))
            .map(|c| c.category)
            .collect()
    }

    /// Renders a Markdown gap-analysis report.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Cross-Framework Gap Analysis\n\n");
        let names: Vec<String> = self.frameworks.iter().map(|f| f.to_string()).collect();
        out.push_str(&format!("Frameworks compared: {}\n\n", names.join(", ")));

        out.push_str("## Universal Coverage\n\n");
        let universal = self.universal_categories();
        if universal.is_empty() {
            out.push_str("_None_\n\n");
        } else {
            for cat in &universal {
                out.push_str(&format!("- {}\n", cat));
            }
            out.push('\n');
        }

        out.push_str("## Gaps\n\n");
        let gaps = self.gap_categories();
        if gaps.is_empty() {
            out.push_str("_No gaps: all categories covered by all frameworks._\n\n");
        } else {
            for c in gaps {
                let missing: Vec<String> = c.missing_from.iter().map(|f| f.to_string()).collect();
                let covered: Vec<String> = c.covered_by.iter().map(|f| f.to_string()).collect();
                out.push_str(&format!(
                    "- **{}** ({}): covered by [{}], missing from [{}]\n",
                    c.category,
                    c.coverage_kind,
                    covered.join(", "),
                    missing.join(", ")
                ));
            }
        }
        out
    }
}

/// Compares coverage across two or more frameworks and reports gaps and overlaps
/// at the normalized [`ControlCategory`] level.
///
/// Returns `None` if fewer than two frameworks are supplied (a comparison needs
/// at least two participants).
pub fn cross_framework_gap_analysis(
    frameworks: &[ComplianceFrameworkKind],
) -> Option<CrossFrameworkGapAnalysis> {
    if frameworks.len() < 2 {
        return None;
    }

    // De-duplicate while preserving order.
    let mut unique_fw: Vec<ComplianceFrameworkKind> = Vec::new();
    for fw in frameworks {
        if !unique_fw.contains(fw) {
            unique_fw.push(*fw);
        }
    }
    if unique_fw.len() < 2 {
        return None;
    }

    // Map: framework -> set of categories it covers.
    let coverage_by_fw: HashMap<ComplianceFrameworkKind, HashSet<ControlCategory>> = unique_fw
        .iter()
        .map(|fw| (*fw, framework_definition(*fw).categories()))
        .collect();

    // Union of all categories across the compared frameworks.
    let mut all_categories: Vec<ControlCategory> = coverage_by_fw
        .values()
        .flat_map(|set| set.iter().copied())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    // Deterministic ordering by display name.
    all_categories.sort_by_key(|c| c.to_string());

    let total = unique_fw.len();
    let mut coverage = Vec::new();
    for category in all_categories {
        let mut covered_by = Vec::new();
        let mut missing_from = Vec::new();
        // Preserve framework input order.
        for fw in &unique_fw {
            if coverage_by_fw
                .get(fw)
                .map(|set| set.contains(&category))
                .unwrap_or(false)
            {
                covered_by.push(*fw);
            } else {
                missing_from.push(*fw);
            }
        }

        let coverage_kind = if covered_by.len() == total {
            CoverageKind::Universal
        } else if covered_by.len() == 1 {
            CoverageKind::Unique
        } else {
            CoverageKind::Partial
        };

        coverage.push(CategoryCoverage {
            category,
            covered_by,
            missing_from,
            coverage_kind,
        });
    }

    Some(CrossFrameworkGapAnalysis {
        frameworks: unique_fw,
        coverage,
    })
}

#[cfg(test)]
mod tests;
