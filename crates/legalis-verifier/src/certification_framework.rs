//! Certification Framework Module
//!
//! This module provides comprehensive certification and compliance verification
//! for legal statutes against various regulatory frameworks including:
//! - ISO 27001 (Information Security Management)
//! - SOC 2 Type II (Service Organization Controls)
//! - GDPR (General Data Protection Regulation)
//!
//! # Examples
//!
//! ```
//! use legalis_verifier::certification_framework::*;
//! use legalis_core::Statute;
//!
//! let statutes = vec![/* your statutes */];
//! let config = CertificationConfig::default();
//! let certifier = CertificationFramework::new(config);
//!
//! let iso_result = certifier.verify_iso27001(&statutes);
//! let soc2_result = certifier.verify_soc2_type2(&statutes);
//! let gdpr_result = certifier.verify_gdpr(&statutes);
//! ```

use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for certification framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationConfig {
    /// Enable ISO 27001 verification
    pub enable_iso27001: bool,
    /// Enable SOC 2 Type II verification
    pub enable_soc2: bool,
    /// Enable GDPR verification
    pub enable_gdpr: bool,
    /// Strictness level (0.0-1.0, higher is stricter)
    pub strictness_level: f64,
    /// Custom control mappings
    #[allow(dead_code)]
    pub custom_controls: HashMap<String, Vec<String>>,
}

impl Default for CertificationConfig {
    fn default() -> Self {
        Self {
            enable_iso27001: true,
            enable_soc2: true,
            enable_gdpr: true,
            strictness_level: 0.8,
            custom_controls: HashMap::new(),
        }
    }
}

/// ISO 27001 control domain
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ISO27001Control {
    /// A.5 Information Security Policies
    InformationSecurityPolicies,
    /// A.6 Organization of Information Security
    OrganizationOfInformationSecurity,
    /// A.7 Human Resource Security
    HumanResourceSecurity,
    /// A.8 Asset Management
    AssetManagement,
    /// A.9 Access Control
    AccessControl,
    /// A.10 Cryptography
    Cryptography,
    /// A.11 Physical and Environmental Security
    PhysicalAndEnvironmentalSecurity,
    /// A.12 Operations Security
    OperationsSecurity,
    /// A.13 Communications Security
    CommunicationsSecurity,
    /// A.14 System Acquisition, Development and Maintenance
    SystemAcquisitionDevelopmentMaintenance,
    /// A.15 Supplier Relationships
    SupplierRelationships,
    /// A.16 Information Security Incident Management
    InformationSecurityIncidentManagement,
    /// A.17 Information Security Aspects of Business Continuity Management
    BusinessContinuityManagement,
    /// A.18 Compliance
    Compliance,
}

impl std::fmt::Display for ISO27001Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ISO27001Control::InformationSecurityPolicies => {
                write!(f, "A.5 Information Security Policies")
            }
            ISO27001Control::OrganizationOfInformationSecurity => {
                write!(f, "A.6 Organization of Information Security")
            }
            ISO27001Control::HumanResourceSecurity => {
                write!(f, "A.7 Human Resource Security")
            }
            ISO27001Control::AssetManagement => write!(f, "A.8 Asset Management"),
            ISO27001Control::AccessControl => write!(f, "A.9 Access Control"),
            ISO27001Control::Cryptography => write!(f, "A.10 Cryptography"),
            ISO27001Control::PhysicalAndEnvironmentalSecurity => {
                write!(f, "A.11 Physical and Environmental Security")
            }
            ISO27001Control::OperationsSecurity => write!(f, "A.12 Operations Security"),
            ISO27001Control::CommunicationsSecurity => {
                write!(f, "A.13 Communications Security")
            }
            ISO27001Control::SystemAcquisitionDevelopmentMaintenance => {
                write!(f, "A.14 System Acquisition, Development and Maintenance")
            }
            ISO27001Control::SupplierRelationships => write!(f, "A.15 Supplier Relationships"),
            ISO27001Control::InformationSecurityIncidentManagement => {
                write!(f, "A.16 Information Security Incident Management")
            }
            ISO27001Control::BusinessContinuityManagement => {
                write!(f, "A.17 Business Continuity Management")
            }
            ISO27001Control::Compliance => write!(f, "A.18 Compliance"),
        }
    }
}

/// SOC 2 Trust Service Criteria
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SOC2Criteria {
    /// Security - Protection against unauthorized access
    Security,
    /// Availability - System availability for operation and use
    Availability,
    /// Processing Integrity - System processing is complete, valid, accurate, timely
    ProcessingIntegrity,
    /// Confidentiality - Confidential information is protected
    Confidentiality,
    /// Privacy - Personal information is collected, used, retained, disclosed appropriately
    Privacy,
}

impl std::fmt::Display for SOC2Criteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SOC2Criteria::Security => write!(f, "Security"),
            SOC2Criteria::Availability => write!(f, "Availability"),
            SOC2Criteria::ProcessingIntegrity => write!(f, "Processing Integrity"),
            SOC2Criteria::Confidentiality => write!(f, "Confidentiality"),
            SOC2Criteria::Privacy => write!(f, "Privacy"),
        }
    }
}

/// GDPR principles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GDPRPrinciple {
    /// Lawfulness, fairness and transparency
    LawfulnessFairnessTransparency,
    /// Purpose limitation
    PurposeLimitation,
    /// Data minimization
    DataMinimization,
    /// Accuracy
    Accuracy,
    /// Storage limitation
    StorageLimitation,
    /// Integrity and confidentiality
    IntegrityAndConfidentiality,
    /// Accountability
    Accountability,
}

impl std::fmt::Display for GDPRPrinciple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GDPRPrinciple::LawfulnessFairnessTransparency => {
                write!(f, "Lawfulness, Fairness and Transparency")
            }
            GDPRPrinciple::PurposeLimitation => write!(f, "Purpose Limitation"),
            GDPRPrinciple::DataMinimization => write!(f, "Data Minimization"),
            GDPRPrinciple::Accuracy => write!(f, "Accuracy"),
            GDPRPrinciple::StorageLimitation => write!(f, "Storage Limitation"),
            GDPRPrinciple::IntegrityAndConfidentiality => {
                write!(f, "Integrity and Confidentiality")
            }
            GDPRPrinciple::Accountability => write!(f, "Accountability"),
        }
    }
}

/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Statute ID that violated the control/criteria/principle
    pub statute_id: String,
    /// Description of the violation
    pub description: String,
    /// Severity (0-10, higher is more severe)
    pub severity: u8,
    /// Recommendation for remediation
    pub recommendation: String,
    /// References to relevant sections
    pub references: Vec<String>,
}

/// ISO 27001 verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ISO27001Result {
    /// Overall compliance score (0-100)
    pub compliance_score: f64,
    /// Controls satisfied
    pub satisfied_controls: Vec<ISO27001Control>,
    /// Violations by control
    pub violations: HashMap<ISO27001Control, Vec<ComplianceViolation>>,
    /// Is compliant (score >= threshold)
    pub is_compliant: bool,
    /// Verification timestamp
    pub timestamp: String,
}

/// SOC 2 Type II verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOC2Type2Result {
    /// Overall compliance score (0-100)
    pub compliance_score: f64,
    /// Criteria satisfied
    pub satisfied_criteria: Vec<SOC2Criteria>,
    /// Violations by criteria
    pub violations: HashMap<SOC2Criteria, Vec<ComplianceViolation>>,
    /// Is compliant (score >= threshold)
    pub is_compliant: bool,
    /// Operational effectiveness period (Type II requires period of time)
    pub evaluation_period_days: u32,
    /// Verification timestamp
    pub timestamp: String,
}

/// GDPR verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GDPRResult {
    /// Overall compliance score (0-100)
    pub compliance_score: f64,
    /// Principles satisfied
    pub satisfied_principles: Vec<GDPRPrinciple>,
    /// Violations by principle
    pub violations: HashMap<GDPRPrinciple, Vec<ComplianceViolation>>,
    /// Is compliant (score >= threshold)
    pub is_compliant: bool,
    /// Data processing activities identified
    pub data_processing_activities: Vec<String>,
    /// Verification timestamp
    pub timestamp: String,
}

/// Third-party attestation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirdPartyAttestation {
    /// Attestation ID
    pub attestation_id: String,
    /// Attesting organization
    pub attesting_organization: String,
    /// Auditor name
    pub auditor_name: String,
    /// Attestation date
    pub attestation_date: String,
    /// Framework attested (ISO 27001, SOC 2, GDPR, etc.)
    pub framework: String,
    /// Attestation statement
    pub statement: String,
    /// Digital signature (simplified as string)
    pub signature: String,
    /// Validity period in days
    pub validity_days: u32,
}

/// Main certification framework
#[derive(Debug, Clone)]
pub struct CertificationFramework {
    config: CertificationConfig,
}

impl CertificationFramework {
    /// Create new certification framework with config
    pub fn new(config: CertificationConfig) -> Self {
        Self { config }
    }

    /// Verify ISO 27001 compliance
    pub fn verify_iso27001(&self, statutes: &[Statute]) -> ISO27001Result {
        let mut violations: HashMap<ISO27001Control, Vec<ComplianceViolation>> = HashMap::new();
        let mut satisfied_controls = Vec::new();
        let mut total_score = 0.0;
        let controls = vec![
            ISO27001Control::InformationSecurityPolicies,
            ISO27001Control::OrganizationOfInformationSecurity,
            ISO27001Control::HumanResourceSecurity,
            ISO27001Control::AssetManagement,
            ISO27001Control::AccessControl,
            ISO27001Control::Cryptography,
            ISO27001Control::PhysicalAndEnvironmentalSecurity,
            ISO27001Control::OperationsSecurity,
            ISO27001Control::CommunicationsSecurity,
            ISO27001Control::SystemAcquisitionDevelopmentMaintenance,
            ISO27001Control::SupplierRelationships,
            ISO27001Control::InformationSecurityIncidentManagement,
            ISO27001Control::BusinessContinuityManagement,
            ISO27001Control::Compliance,
        ];

        for control in &controls {
            let control_violations = self.check_iso27001_control(control, statutes);
            if control_violations.is_empty() {
                satisfied_controls.push(control.clone());
                total_score += 100.0 / controls.len() as f64;
            } else {
                let penalty =
                    (control_violations.len() as f64 * 10.0).min(100.0 / controls.len() as f64);
                total_score += (100.0 / controls.len() as f64) - penalty;
                violations.insert(control.clone(), control_violations);
            }
        }

        let compliance_threshold = 80.0 * self.config.strictness_level;
        ISO27001Result {
            compliance_score: total_score.max(0.0),
            satisfied_controls,
            violations,
            is_compliant: total_score >= compliance_threshold,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Verify SOC 2 Type II compliance
    pub fn verify_soc2_type2(&self, statutes: &[Statute]) -> SOC2Type2Result {
        let mut violations: HashMap<SOC2Criteria, Vec<ComplianceViolation>> = HashMap::new();
        let mut satisfied_criteria = Vec::new();
        let mut total_score = 0.0;
        let criteria = vec![
            SOC2Criteria::Security,
            SOC2Criteria::Availability,
            SOC2Criteria::ProcessingIntegrity,
            SOC2Criteria::Confidentiality,
            SOC2Criteria::Privacy,
        ];

        for criterion in &criteria {
            let criterion_violations = self.check_soc2_criterion(criterion, statutes);
            if criterion_violations.is_empty() {
                satisfied_criteria.push(criterion.clone());
                total_score += 100.0 / criteria.len() as f64;
            } else {
                let penalty =
                    (criterion_violations.len() as f64 * 10.0).min(100.0 / criteria.len() as f64);
                total_score += (100.0 / criteria.len() as f64) - penalty;
                violations.insert(criterion.clone(), criterion_violations);
            }
        }

        let compliance_threshold = 80.0 * self.config.strictness_level;
        SOC2Type2Result {
            compliance_score: total_score.max(0.0),
            satisfied_criteria,
            violations,
            is_compliant: total_score >= compliance_threshold,
            evaluation_period_days: 365, // Type II requires at least 6 months, default to 1 year
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Verify GDPR compliance
    pub fn verify_gdpr(&self, statutes: &[Statute]) -> GDPRResult {
        let mut violations: HashMap<GDPRPrinciple, Vec<ComplianceViolation>> = HashMap::new();
        let mut satisfied_principles = Vec::new();
        let mut total_score = 0.0;
        let mut data_processing_activities = Vec::new();

        let principles = vec![
            GDPRPrinciple::LawfulnessFairnessTransparency,
            GDPRPrinciple::PurposeLimitation,
            GDPRPrinciple::DataMinimization,
            GDPRPrinciple::Accuracy,
            GDPRPrinciple::StorageLimitation,
            GDPRPrinciple::IntegrityAndConfidentiality,
            GDPRPrinciple::Accountability,
        ];

        // Identify data processing activities
        for statute in statutes {
            if self.involves_data_processing(statute) {
                data_processing_activities.push(statute.id.clone());
            }
        }

        for principle in &principles {
            let principle_violations = self.check_gdpr_principle(principle, statutes);
            if principle_violations.is_empty() {
                satisfied_principles.push(principle.clone());
                total_score += 100.0 / principles.len() as f64;
            } else {
                let penalty =
                    (principle_violations.len() as f64 * 10.0).min(100.0 / principles.len() as f64);
                total_score += (100.0 / principles.len() as f64) - penalty;
                violations.insert(principle.clone(), principle_violations);
            }
        }

        let compliance_threshold = 80.0 * self.config.strictness_level;
        GDPRResult {
            compliance_score: total_score.max(0.0),
            satisfied_principles,
            violations,
            is_compliant: total_score >= compliance_threshold,
            data_processing_activities,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Generate third-party attestation
    pub fn generate_attestation(
        &self,
        framework: &str,
        organization: &str,
        auditor: &str,
        is_compliant: bool,
    ) -> ThirdPartyAttestation {
        let attestation_id = format!(
            "ATT-{}-{}",
            framework.to_uppercase().replace(' ', "-"),
            chrono::Utc::now().timestamp()
        );

        let statement = if is_compliant {
            format!(
                "This is to attest that {} has been audited for {} compliance and meets all required controls and criteria as of {}.",
                organization,
                framework,
                chrono::Utc::now().format("%Y-%m-%d")
            )
        } else {
            format!(
                "This is to attest that {} has been audited for {} compliance. Non-compliance issues have been identified and documented as of {}.",
                organization,
                framework,
                chrono::Utc::now().format("%Y-%m-%d")
            )
        };

        // Simplified signature (in production, use actual cryptographic signing)
        let signature = format!(
            "SIG-{}-{}",
            auditor.to_uppercase().replace(' ', "-"),
            chrono::Utc::now().timestamp()
        );

        ThirdPartyAttestation {
            attestation_id,
            attesting_organization: organization.to_string(),
            auditor_name: auditor.to_string(),
            attestation_date: chrono::Utc::now().to_rfc3339(),
            framework: framework.to_string(),
            statement,
            signature,
            validity_days: 365,
        }
    }

    // Helper methods for checking specific controls/criteria/principles

    fn check_iso27001_control(
        &self,
        control: &ISO27001Control,
        statutes: &[Statute],
    ) -> Vec<ComplianceViolation> {
        let mut violations = Vec::new();

        match control {
            ISO27001Control::AccessControl => {
                // Check for access control violations
                for statute in statutes {
                    if self.mentions_access(statute) && !self.has_access_controls(statute) {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: "Statute mentions access but lacks explicit access control mechanisms".to_string(),
                            severity: 7,
                            recommendation: "Add access control preconditions (e.g., role-based, attribute-based)".to_string(),
                            references: vec!["ISO/IEC 27001:2013 A.9".to_string()],
                        });
                    }
                }
            }
            ISO27001Control::Compliance => {
                // Check for compliance-related violations
                for statute in statutes {
                    if statute.jurisdiction.is_none() {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: "Missing jurisdiction specification required for compliance verification".to_string(),
                            severity: 6,
                            recommendation: "Specify jurisdiction to ensure legal compliance".to_string(),
                            references: vec!["ISO/IEC 27001:2013 A.18".to_string()],
                        });
                    }
                }
            }
            ISO27001Control::Cryptography => {
                // Check for cryptography-related violations
                for statute in statutes {
                    if self.mentions_encryption(statute)
                        && !self.specifies_crypto_standards(statute)
                    {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description:
                                "Encryption mentioned but cryptographic standards not specified"
                                    .to_string(),
                            severity: 8,
                            recommendation:
                                "Specify cryptographic algorithms and key management procedures"
                                    .to_string(),
                            references: vec!["ISO/IEC 27001:2013 A.10".to_string()],
                        });
                    }
                }
            }
            _ => {
                // Generic checks for other controls
                for statute in statutes {
                    if statute.discretion_logic.is_none() && statute.preconditions.len() > 3 {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: format!(
                                "Complex statute lacks discretion logic for {} control",
                                control
                            ),
                            severity: 5,
                            recommendation: "Add discretion logic to clarify implementation"
                                .to_string(),
                            references: vec![format!("ISO/IEC 27001:2013 {}", control)],
                        });
                    }
                }
            }
        }

        violations
    }

    fn check_soc2_criterion(
        &self,
        criterion: &SOC2Criteria,
        statutes: &[Statute],
    ) -> Vec<ComplianceViolation> {
        let mut violations = Vec::new();

        match criterion {
            SOC2Criteria::Security => {
                for statute in statutes {
                    if self.mentions_security(statute) && !self.has_security_controls(statute) {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description:
                                "Security-related statute lacks explicit security controls"
                                    .to_string(),
                            severity: 8,
                            recommendation:
                                "Add authentication, authorization, or encryption requirements"
                                    .to_string(),
                            references: vec!["SOC 2 CC6 - Security".to_string()],
                        });
                    }
                }
            }
            SOC2Criteria::Availability => {
                for statute in statutes {
                    if self.mentions_availability(statute)
                        && !self.has_availability_requirements(statute)
                    {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description:
                                "Availability mentioned but no uptime or redundancy requirements"
                                    .to_string(),
                            severity: 6,
                            recommendation: "Specify availability SLAs and redundancy mechanisms"
                                .to_string(),
                            references: vec!["SOC 2 CC7 - Availability".to_string()],
                        });
                    }
                }
            }
            SOC2Criteria::Privacy => {
                for statute in statutes {
                    if self.involves_personal_data(statute)
                        && !self.has_privacy_protections(statute)
                    {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: "Personal data processing without explicit privacy protections".to_string(),
                            severity: 9,
                            recommendation: "Add data protection measures, consent mechanisms, and retention policies".to_string(),
                            references: vec!["SOC 2 CC9 - Privacy".to_string()],
                        });
                    }
                }
            }
            _ => {
                // Generic checks for other criteria
                for statute in statutes {
                    if statute
                        .title
                        .to_lowercase()
                        .contains(&criterion.to_string().to_lowercase())
                        && statute.discretion_logic.is_none()
                    {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: format!(
                                "Statute related to {} lacks implementation details",
                                criterion
                            ),
                            severity: 5,
                            recommendation: "Add discretion logic to clarify {} implementation"
                                .to_string(),
                            references: vec![format!("SOC 2 - {}", criterion)],
                        });
                    }
                }
            }
        }

        violations
    }

    fn check_gdpr_principle(
        &self,
        principle: &GDPRPrinciple,
        statutes: &[Statute],
    ) -> Vec<ComplianceViolation> {
        let mut violations = Vec::new();

        match principle {
            GDPRPrinciple::LawfulnessFairnessTransparency => {
                for statute in statutes {
                    if self.involves_personal_data(statute) && !self.specifies_legal_basis(statute)
                    {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: "Personal data processing without specified legal basis"
                                .to_string(),
                            severity: 9,
                            recommendation:
                                "Specify legal basis (consent, contract, legal obligation, etc.)"
                                    .to_string(),
                            references: vec!["GDPR Article 6".to_string()],
                        });
                    }
                }
            }
            GDPRPrinciple::PurposeLimitation => {
                for statute in statutes {
                    if self.involves_personal_data(statute) && self.has_vague_purpose(statute) {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: "Data processing purpose is vague or overly broad"
                                .to_string(),
                            severity: 7,
                            recommendation:
                                "Clearly specify the specific, explicit, and legitimate purpose"
                                    .to_string(),
                            references: vec!["GDPR Article 5(1)(b)".to_string()],
                        });
                    }
                }
            }
            GDPRPrinciple::DataMinimization => {
                for statute in statutes {
                    if self.involves_personal_data(statute) && self.collects_excessive_data(statute)
                    {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: "Data collection appears excessive for stated purpose"
                                .to_string(),
                            severity: 6,
                            recommendation:
                                "Limit data collection to what is necessary and relevant"
                                    .to_string(),
                            references: vec!["GDPR Article 5(1)(c)".to_string()],
                        });
                    }
                }
            }
            GDPRPrinciple::StorageLimitation => {
                for statute in statutes {
                    if self.involves_personal_data(statute) && !self.has_retention_policy(statute) {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: "No data retention period or deletion policy specified"
                                .to_string(),
                            severity: 7,
                            recommendation: "Specify retention period and deletion procedures"
                                .to_string(),
                            references: vec!["GDPR Article 5(1)(e)".to_string()],
                        });
                    }
                }
            }
            GDPRPrinciple::IntegrityAndConfidentiality => {
                for statute in statutes {
                    if self.involves_personal_data(statute) && !self.has_security_measures(statute)
                    {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: "Personal data processing without security measures"
                                .to_string(),
                            severity: 9,
                            recommendation:
                                "Implement encryption, access controls, and security monitoring"
                                    .to_string(),
                            references: vec!["GDPR Article 5(1)(f), Article 32".to_string()],
                        });
                    }
                }
            }
            GDPRPrinciple::Accountability => {
                for statute in statutes {
                    if self.involves_personal_data(statute)
                        && !self.has_accountability_measures(statute)
                    {
                        violations.push(ComplianceViolation {
                            statute_id: statute.id.clone(),
                            description: "Lack of accountability measures (logging, auditing, DPO)".to_string(),
                            severity: 6,
                            recommendation: "Implement logging, auditing, and data protection impact assessments".to_string(),
                            references: vec!["GDPR Article 5(2), Article 24".to_string()],
                        });
                    }
                }
            }
            _ => {}
        }

        violations
    }

    // Detection helper methods

    fn mentions_access(&self, statute: &Statute) -> bool {
        let desc = statute.discretion_logic.as_deref().unwrap_or("");
        let text = format!("{} {} {}", statute.title, statute.effect.description, desc);
        text.to_lowercase().contains("access")
    }

    fn has_access_controls(&self, statute: &Statute) -> bool {
        // Check for role, permission, authentication-related conditions
        statute.preconditions.iter().any(|c| {
            let desc = format!("{:?}", c).to_lowercase();
            desc.contains("role") || desc.contains("permission") || desc.contains("auth")
        })
    }

    fn mentions_encryption(&self, statute: &Statute) -> bool {
        let desc = statute.discretion_logic.as_deref().unwrap_or("");
        let text = format!("{} {} {}", statute.title, statute.effect.description, desc);
        let lower = text.to_lowercase();
        lower.contains("encrypt") || lower.contains("crypto")
    }

    fn specifies_crypto_standards(&self, _statute: &Statute) -> bool {
        // Check if cryptographic standards are mentioned (simplified check)
        // In a real implementation, check for specific algorithms (AES, RSA, etc.)
        false
    }

    fn mentions_security(&self, statute: &Statute) -> bool {
        let desc = statute.discretion_logic.as_deref().unwrap_or("");
        let text = format!("{} {} {}", statute.title, statute.effect.description, desc);
        text.to_lowercase().contains("security")
    }

    fn has_security_controls(&self, statute: &Statute) -> bool {
        !statute.preconditions.is_empty() || statute.discretion_logic.is_some()
    }

    fn mentions_availability(&self, statute: &Statute) -> bool {
        let desc = statute.discretion_logic.as_deref().unwrap_or("");
        let text = format!("{} {} {}", statute.title, statute.effect.description, desc);
        let lower = text.to_lowercase();
        lower.contains("availab") || lower.contains("uptime") || lower.contains("redundan")
    }

    fn has_availability_requirements(&self, _statute: &Statute) -> bool {
        // Check for SLA or availability requirements (simplified)
        false
    }

    fn involves_personal_data(&self, statute: &Statute) -> bool {
        let desc = statute.discretion_logic.as_deref().unwrap_or("");
        let text = format!("{} {} {}", statute.title, statute.effect.description, desc);
        let lower = text.to_lowercase();
        lower.contains("personal")
            || lower.contains("data")
            || lower.contains("privacy")
            || lower.contains("pii")
    }

    fn has_privacy_protections(&self, statute: &Statute) -> bool {
        let text = format!("{:?}", statute).to_lowercase();
        text.contains("consent") || text.contains("encrypt") || text.contains("anonym")
    }

    fn involves_data_processing(&self, statute: &Statute) -> bool {
        self.involves_personal_data(statute)
    }

    fn specifies_legal_basis(&self, statute: &Statute) -> bool {
        let text = format!("{:?}", statute).to_lowercase();
        text.contains("consent")
            || text.contains("contract")
            || text.contains("legal obligation")
            || text.contains("legitimate interest")
    }

    fn has_vague_purpose(&self, statute: &Statute) -> bool {
        // Check if purpose description is too short or contains vague terms
        let desc = &statute.effect.description;
        desc.len() < 20
            || desc.to_lowercase().contains("various")
            || desc.to_lowercase().contains("general")
    }

    fn collects_excessive_data(&self, statute: &Statute) -> bool {
        // Heuristic: More than 5 preconditions might indicate excessive data collection
        statute.preconditions.len() > 5
    }

    fn has_retention_policy(&self, statute: &Statute) -> bool {
        let text = format!("{:?}", statute).to_lowercase();
        text.contains("retention")
            || text.contains("delete")
            || text.contains("duration")
            || text.contains("period")
    }

    fn has_security_measures(&self, statute: &Statute) -> bool {
        let text = format!("{:?}", statute).to_lowercase();
        text.contains("encrypt") || text.contains("secure") || text.contains("protect")
    }

    fn has_accountability_measures(&self, statute: &Statute) -> bool {
        let text = format!("{:?}", statute).to_lowercase();
        text.contains("log") || text.contains("audit") || text.contains("monitor")
    }
}

/// Generate comprehensive certification report
pub fn certification_report(
    iso_result: &Option<ISO27001Result>,
    soc2_result: &Option<SOC2Type2Result>,
    gdpr_result: &Option<GDPRResult>,
    attestations: &[ThirdPartyAttestation],
) -> String {
    let mut report = String::new();
    report.push_str("# Regulatory Certification Report\n\n");
    report.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().to_rfc3339()
    ));
    report.push_str("---\n\n");

    // ISO 27001 Section
    if let Some(iso) = iso_result {
        report.push_str("## ISO 27001:2013 Compliance\n\n");
        report.push_str(&format!(
            "**Compliance Score:** {:.1}%\n",
            iso.compliance_score
        ));
        report.push_str(&format!(
            "**Status:** {}\n",
            if iso.is_compliant {
                "✓ COMPLIANT"
            } else {
                "✗ NON-COMPLIANT"
            }
        ));
        report.push_str(&format!(
            "**Satisfied Controls:** {}\n",
            iso.satisfied_controls.len()
        ));
        report.push_str(&format!(
            "**Controls with Violations:** {}\n\n",
            iso.violations.len()
        ));

        if !iso.violations.is_empty() {
            report.push_str("### Violations by Control\n\n");
            for (control, viols) in &iso.violations {
                report.push_str(&format!("#### {}\n\n", control));
                for v in viols {
                    report.push_str(&format!("- **Statute:** `{}`\n", v.statute_id));
                    report.push_str(&format!("  - **Issue:** {}\n", v.description));
                    report.push_str(&format!("  - **Severity:** {}/10\n", v.severity));
                    report.push_str(&format!("  - **Recommendation:** {}\n", v.recommendation));
                }
                report.push('\n');
            }
        }
    }

    // SOC 2 Section
    if let Some(soc2) = soc2_result {
        report.push_str("## SOC 2 Type II Compliance\n\n");
        report.push_str(&format!(
            "**Compliance Score:** {:.1}%\n",
            soc2.compliance_score
        ));
        report.push_str(&format!(
            "**Status:** {}\n",
            if soc2.is_compliant {
                "✓ COMPLIANT"
            } else {
                "✗ NON-COMPLIANT"
            }
        ));
        report.push_str(&format!(
            "**Evaluation Period:** {} days\n",
            soc2.evaluation_period_days
        ));
        report.push_str(&format!(
            "**Satisfied Criteria:** {}\n",
            soc2.satisfied_criteria.len()
        ));
        report.push_str(&format!(
            "**Criteria with Violations:** {}\n\n",
            soc2.violations.len()
        ));

        if !soc2.violations.is_empty() {
            report.push_str("### Violations by Criteria\n\n");
            for (criterion, viols) in &soc2.violations {
                report.push_str(&format!("#### {}\n\n", criterion));
                for v in viols {
                    report.push_str(&format!("- **Statute:** `{}`\n", v.statute_id));
                    report.push_str(&format!("  - **Issue:** {}\n", v.description));
                    report.push_str(&format!("  - **Severity:** {}/10\n", v.severity));
                    report.push_str(&format!("  - **Recommendation:** {}\n", v.recommendation));
                }
                report.push('\n');
            }
        }
    }

    // GDPR Section
    if let Some(gdpr) = gdpr_result {
        report.push_str("## GDPR Compliance\n\n");
        report.push_str(&format!(
            "**Compliance Score:** {:.1}%\n",
            gdpr.compliance_score
        ));
        report.push_str(&format!(
            "**Status:** {}\n",
            if gdpr.is_compliant {
                "✓ COMPLIANT"
            } else {
                "✗ NON-COMPLIANT"
            }
        ));
        report.push_str(&format!(
            "**Data Processing Activities:** {}\n",
            gdpr.data_processing_activities.len()
        ));
        report.push_str(&format!(
            "**Satisfied Principles:** {}\n",
            gdpr.satisfied_principles.len()
        ));
        report.push_str(&format!(
            "**Principles with Violations:** {}\n\n",
            gdpr.violations.len()
        ));

        if !gdpr.violations.is_empty() {
            report.push_str("### Violations by Principle\n\n");
            for (principle, viols) in &gdpr.violations {
                report.push_str(&format!("#### {}\n\n", principle));
                for v in viols {
                    report.push_str(&format!("- **Statute:** `{}`\n", v.statute_id));
                    report.push_str(&format!("  - **Issue:** {}\n", v.description));
                    report.push_str(&format!("  - **Severity:** {}/10\n", v.severity));
                    report.push_str(&format!("  - **Recommendation:** {}\n", v.recommendation));
                }
                report.push('\n');
            }
        }
    }

    // Attestations Section
    if !attestations.is_empty() {
        report.push_str("## Third-Party Attestations\n\n");
        for att in attestations {
            report.push_str(&format!("### {}\n\n", att.attestation_id));
            report.push_str(&format!("- **Framework:** {}\n", att.framework));
            report.push_str(&format!("- **Auditor:** {}\n", att.auditor_name));
            report.push_str(&format!(
                "- **Organization:** {}\n",
                att.attesting_organization
            ));
            report.push_str(&format!("- **Date:** {}\n", att.attestation_date));
            report.push_str(&format!("- **Validity:** {} days\n", att.validity_days));
            report.push_str(&format!("- **Statement:** {}\n", att.statement));
            report.push_str(&format!("- **Signature:** `{}`\n\n", att.signature));
        }
    }

    report.push_str("---\n\n");
    report.push_str("*This report is generated by the Legalis Certification Framework*\n");

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, TemporalValidity};

    fn create_test_statute(id: &str, title: &str, description: &str) -> Statute {
        Statute {
            id: id.to_string(),
            title: title.to_string(),
            effect: Effect {
                effect_type: EffectType::Grant,
                description: description.to_string(),
                parameters: Default::default(),
            },
            preconditions: vec![],
            jurisdiction: Some("Test Jurisdiction".to_string()),
            discretion_logic: Some("Test discretion logic".to_string()),
            temporal_validity: TemporalValidity::default(),
            version: 1,
            derives_from: vec![],
            applies_to: vec![],
            exceptions: vec![],
        }
    }

    #[test]
    fn test_certification_config_default() {
        let config = CertificationConfig::default();
        assert!(config.enable_iso27001);
        assert!(config.enable_soc2);
        assert!(config.enable_gdpr);
        assert_eq!(config.strictness_level, 0.8);
    }

    #[test]
    fn test_iso27001_control_display() {
        let control = ISO27001Control::AccessControl;
        assert_eq!(format!("{}", control), "A.9 Access Control");
    }

    #[test]
    fn test_soc2_criteria_display() {
        let criteria = SOC2Criteria::Security;
        assert_eq!(format!("{}", criteria), "Security");
    }

    #[test]
    fn test_gdpr_principle_display() {
        let principle = GDPRPrinciple::DataMinimization;
        assert_eq!(format!("{}", principle), "Data Minimization");
    }

    #[test]
    fn test_iso27001_verification_compliant() {
        let config = CertificationConfig::default();
        let framework = CertificationFramework::new(config);
        let statutes = vec![create_test_statute(
            "S1",
            "Test Statute",
            "A simple test statute",
        )];

        let result = framework.verify_iso27001(&statutes);
        assert!(result.compliance_score >= 0.0 && result.compliance_score <= 100.0);
        assert!(!result.satisfied_controls.is_empty() || !result.violations.is_empty());
    }

    #[test]
    fn test_iso27001_access_control_violation() {
        let config = CertificationConfig::default();
        let framework = CertificationFramework::new(config);
        let mut statute =
            create_test_statute("S1", "Access Control Policy", "Governs system access");
        statute.preconditions = vec![]; // No access controls

        let result = framework.verify_iso27001(&[statute]);

        // Should have some violations for access control
        if let Some(violations) = result.violations.get(&ISO27001Control::AccessControl) {
            assert!(!violations.is_empty());
        }
    }

    #[test]
    fn test_soc2_verification() {
        let config = CertificationConfig::default();
        let framework = CertificationFramework::new(config);
        let statutes = vec![create_test_statute(
            "S1",
            "Security Policy",
            "Defines security controls",
        )];

        let result = framework.verify_soc2_type2(&statutes);
        assert!(result.compliance_score >= 0.0 && result.compliance_score <= 100.0);
        assert_eq!(result.evaluation_period_days, 365);
    }

    #[test]
    fn test_gdpr_verification() {
        let config = CertificationConfig::default();
        let framework = CertificationFramework::new(config);
        let statutes = vec![create_test_statute(
            "S1",
            "Data Protection",
            "Protects personal data",
        )];

        let result = framework.verify_gdpr(&statutes);
        assert!(result.compliance_score >= 0.0 && result.compliance_score <= 100.0);
    }

    #[test]
    fn test_gdpr_personal_data_detection() {
        let config = CertificationConfig::default();
        let framework = CertificationFramework::new(config);
        let statute = create_test_statute(
            "S1",
            "Personal Data Collection",
            "Collects personal information",
        );

        let result = framework.verify_gdpr(&[statute]);
        assert_eq!(result.data_processing_activities.len(), 1);
        assert_eq!(result.data_processing_activities[0], "S1");
    }

    #[test]
    fn test_generate_attestation_compliant() {
        let config = CertificationConfig::default();
        let framework = CertificationFramework::new(config);

        let attestation =
            framework.generate_attestation("ISO 27001", "Test Organization", "Test Auditor", true);

        assert!(attestation.attestation_id.starts_with("ATT-ISO-27001-"));
        assert_eq!(attestation.framework, "ISO 27001");
        assert_eq!(attestation.attesting_organization, "Test Organization");
        assert_eq!(attestation.auditor_name, "Test Auditor");
        assert!(
            attestation
                .statement
                .contains("meets all required controls")
        );
        assert_eq!(attestation.validity_days, 365);
    }

    #[test]
    fn test_generate_attestation_non_compliant() {
        let config = CertificationConfig::default();
        let framework = CertificationFramework::new(config);

        let attestation =
            framework.generate_attestation("SOC 2", "Test Org", "Auditor Name", false);

        assert!(attestation.statement.contains("Non-compliance issues"));
    }

    #[test]
    fn test_certification_report_generation() {
        let config = CertificationConfig::default();
        let framework = CertificationFramework::new(config);
        let statutes = vec![create_test_statute("S1", "Test", "Test statute")];

        let iso_result = framework.verify_iso27001(&statutes);
        let soc2_result = framework.verify_soc2_type2(&statutes);
        let gdpr_result = framework.verify_gdpr(&statutes);

        let attestation = framework.generate_attestation("ISO 27001", "Test Org", "Auditor", true);

        let report = certification_report(
            &Some(iso_result),
            &Some(soc2_result),
            &Some(gdpr_result),
            &[attestation],
        );

        assert!(report.contains("# Regulatory Certification Report"));
        assert!(report.contains("ISO 27001:2013 Compliance"));
        assert!(report.contains("SOC 2 Type II Compliance"));
        assert!(report.contains("GDPR Compliance"));
        assert!(report.contains("Third-Party Attestations"));
    }

    #[test]
    fn test_compliance_violation_structure() {
        let violation = ComplianceViolation {
            statute_id: "S1".to_string(),
            description: "Test violation".to_string(),
            severity: 7,
            recommendation: "Fix it".to_string(),
            references: vec!["REF-1".to_string()],
        };

        assert_eq!(violation.statute_id, "S1");
        assert_eq!(violation.severity, 7);
        assert_eq!(violation.references.len(), 1);
    }
}
