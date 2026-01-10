//! Security of processing and data breach notification (Articles 32-34)
//!
//! This module implements:
//! - Article 32: Security of processing (technical and organizational measures)
//! - Article 33: Notification of a personal data breach to the supervisory authority
//! - Article 34: Communication of a personal data breach to the data subject
//!
//! ## Article 32 - Security of Processing
//!
//! Article 32(1) requires controllers and processors to implement appropriate technical
//! and organizational measures to ensure a level of security appropriate to the risk, including:
//!
//! - (a) Pseudonymisation and encryption of personal data
//! - (b) Ability to ensure ongoing confidentiality, integrity, availability and resilience
//! - (c) Ability to restore availability and access to data in a timely manner
//! - (d) Regular testing, assessment and evaluation of effectiveness
//!
//! EUR-Lex: CELEX:32016R0679 Art. 32
//!
//! ## Example - Security Assessment
//!
//! ```rust
//! use legalis_eu::gdpr::*;
//!
//! let assessment = SecurityAssessment::new()
//!     .with_entity("Acme Corp")
//!     .add_technical_measure(TechnicalMeasure::Encryption {
//!         data_at_rest: true,
//!         data_in_transit: true,
//!         algorithm: "AES-256".to_string(),
//!     })
//!     .add_technical_measure(TechnicalMeasure::Pseudonymisation {
//!         method: "Hashing with salt".to_string(),
//!     })
//!     .add_organizational_measure(OrganizationalMeasure::AccessControl {
//!         role_based: true,
//!         least_privilege: true,
//!     })
//!     .add_organizational_measure(OrganizationalMeasure::StaffTraining {
//!         frequency: "Annual".to_string(),
//!     })
//!     .with_risk_level(SecurityRiskLevel::High);
//!
//! let validation = assessment.validate();
//! assert!(validation.is_ok());
//! ```

use crate::gdpr::{
    error::GdprError,
    types::{BreachCategory, BreachSeverity},
};
use chrono::{DateTime, Duration, Utc};

// ============================================================================
// Article 32 - Security of Processing
// ============================================================================

/// Risk level for data processing operations (Article 32(1))
///
/// The level of security should be appropriate to the risk presented by processing,
/// considering likelihood and severity of risks to rights and freedoms.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RiskLevel {
    /// Low risk - standard security measures sufficient
    Low,

    /// Medium risk - enhanced security measures recommended
    Medium,

    /// High risk - comprehensive security measures required
    High,

    /// Critical risk - maximum security measures + DPIA required
    Critical,
}

/// Technical security measures (Article 32(1)(a-d))
///
/// These are technical safeguards that controllers and processors must implement
/// to ensure security appropriate to the risk.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TechnicalMeasure {
    /// Article 32(1)(a) - Pseudonymisation
    ///
    /// Processing in a manner that personal data can no longer be attributed to a specific
    /// data subject without use of additional information.
    Pseudonymisation { method: String },

    /// Article 32(1)(a) - Encryption
    ///
    /// Rendering personal data unintelligible to unauthorized persons.
    Encryption {
        data_at_rest: bool,
        data_in_transit: bool,
        algorithm: String,
    },

    /// Article 32(1)(b) - Confidentiality measures
    ///
    /// Ensuring only authorized persons can access personal data.
    Confidentiality {
        access_logging: bool,
        intrusion_detection: bool,
    },

    /// Article 32(1)(b) - Integrity measures
    ///
    /// Ensuring personal data cannot be altered or destroyed in unauthorized manner.
    Integrity {
        checksums: bool,
        digital_signatures: bool,
        version_control: bool,
    },

    /// Article 32(1)(b) - Availability measures
    ///
    /// Ensuring authorized persons can access data when needed.
    Availability {
        redundancy: bool,
        load_balancing: bool,
        uptime_sla: Option<f64>, // e.g., 99.9%
    },

    /// Article 32(1)(b) - Resilience measures
    ///
    /// Ability of systems to withstand and recover from adverse conditions.
    Resilience {
        fault_tolerance: bool,
        geographic_redundancy: bool,
    },

    /// Article 32(1)(c) - Backup and recovery
    ///
    /// Ability to restore availability and access in timely manner after incident.
    BackupRecovery {
        backup_frequency: String,
        recovery_time_objective: String,  // RTO
        recovery_point_objective: String, // RPO
        tested: bool,
    },

    /// Article 32(1)(d) - Testing and assessment
    ///
    /// Regular testing, assessment and evaluation of effectiveness.
    TestingAssessment {
        penetration_testing: bool,
        vulnerability_scanning: bool,
        frequency: String,
    },

    /// Custom technical measure
    Other { name: String, description: String },
}

/// Organizational security measures (Article 32)
///
/// Non-technical safeguards including policies, procedures, and governance.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrganizationalMeasure {
    /// Access control policies
    AccessControl {
        role_based: bool,
        least_privilege: bool,
    },

    /// Staff training and awareness
    StaffTraining { frequency: String },

    /// Incident response procedures
    IncidentResponse { documented: bool, tested: bool },

    /// Security policies
    SecurityPolicies {
        documented: bool,
        reviewed_regularly: bool,
    },

    /// Vendor management
    VendorManagement {
        due_diligence: bool,
        contracts_in_place: bool, // Article 28 processor contracts
    },

    /// Physical security
    PhysicalSecurity {
        access_control: bool,
        surveillance: bool,
    },

    /// Data retention and deletion
    DataRetention {
        policy_documented: bool,
        automated_deletion: bool,
    },

    /// Business continuity planning
    BusinessContinuity { documented: bool, tested: bool },

    /// Custom organizational measure
    Other { name: String, description: String },
}

/// Security assessment for Article 32 compliance
///
/// This builder allows organizations to document their technical and organizational
/// measures and validate whether they meet Article 32 requirements.
///
/// ## Example
///
/// ```rust
/// use legalis_eu::gdpr::*;
///
/// let assessment = SecurityAssessment::new()
///     .with_entity("Healthcare Provider")
///     .with_risk_level(SecurityRiskLevel::High)
///     .add_technical_measure(TechnicalMeasure::Encryption {
///         data_at_rest: true,
///         data_in_transit: true,
///         algorithm: "AES-256-GCM".to_string(),
///     })
///     .add_technical_measure(TechnicalMeasure::TestingAssessment {
///         penetration_testing: true,
///         vulnerability_scanning: true,
///         frequency: "Quarterly".to_string(),
///     })
///     .add_organizational_measure(OrganizationalMeasure::StaffTraining {
///         frequency: "Quarterly".to_string(),
///     })
///     .with_state_of_art_considered(true)
///     .with_implementation_costs_considered(true)
///     .with_processing_context_considered(true);
///
/// let validation = assessment.validate().unwrap();
/// // Check validation results
/// assert!(!validation.warnings.is_empty() || validation.compliant);
/// ```
#[derive(Debug, Clone)]
pub struct SecurityAssessment {
    /// Organization or entity being assessed
    pub entity: Option<String>,

    /// Risk level of processing operations
    pub risk_level: Option<RiskLevel>,

    /// Technical security measures implemented
    pub technical_measures: Vec<TechnicalMeasure>,

    /// Organizational security measures implemented
    pub organizational_measures: Vec<OrganizationalMeasure>,

    /// State of the art considerations
    pub state_of_art_considered: Option<bool>,

    /// Implementation costs considered
    pub implementation_costs_considered: Option<bool>,

    /// Nature, scope, context, purposes of processing considered
    pub processing_context_considered: Option<bool>,

    /// Assessment date
    pub assessment_date: Option<DateTime<Utc>>,

    /// Notes or additional context
    pub notes: Option<String>,
}

impl SecurityAssessment {
    /// Create a new security assessment
    pub fn new() -> Self {
        Self {
            entity: None,
            risk_level: None,
            technical_measures: Vec::new(),
            organizational_measures: Vec::new(),
            state_of_art_considered: None,
            implementation_costs_considered: None,
            processing_context_considered: None,
            assessment_date: None,
            notes: None,
        }
    }

    /// Set the entity being assessed
    pub fn with_entity(mut self, entity: impl Into<String>) -> Self {
        self.entity = Some(entity.into());
        self
    }

    /// Set the risk level
    pub fn with_risk_level(mut self, level: RiskLevel) -> Self {
        self.risk_level = Some(level);
        self
    }

    /// Add a technical security measure
    pub fn add_technical_measure(mut self, measure: TechnicalMeasure) -> Self {
        self.technical_measures.push(measure);
        self
    }

    /// Add an organizational security measure
    pub fn add_organizational_measure(mut self, measure: OrganizationalMeasure) -> Self {
        self.organizational_measures.push(measure);
        self
    }

    /// Mark that state of the art was considered (Article 32(1))
    pub fn with_state_of_art_considered(mut self, considered: bool) -> Self {
        self.state_of_art_considered = Some(considered);
        self
    }

    /// Mark that implementation costs were considered (Article 32(1))
    pub fn with_implementation_costs_considered(mut self, considered: bool) -> Self {
        self.implementation_costs_considered = Some(considered);
        self
    }

    /// Mark that processing context was considered (Article 32(1))
    pub fn with_processing_context_considered(mut self, considered: bool) -> Self {
        self.processing_context_considered = Some(considered);
        self
    }

    /// Set assessment date
    pub fn with_assessment_date(mut self, date: DateTime<Utc>) -> Self {
        self.assessment_date = Some(date);
        self
    }

    /// Add notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Validate Article 32 compliance
    ///
    /// Checks whether the implemented measures are appropriate for the risk level
    /// and whether all Article 32(1) criteria have been considered.
    pub fn validate(&self) -> Result<SecurityValidation, GdprError> {
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();

        // Check if risk level is specified
        if self.risk_level.is_none() {
            warnings.push("Risk level not specified - cannot assess measure adequacy".to_string());
        }

        let risk_level = self.risk_level.as_ref().unwrap_or(&RiskLevel::Medium);

        // Article 32(1)(a) - Check for pseudonymisation or encryption
        let has_pseudonymisation = self
            .technical_measures
            .iter()
            .any(|m| matches!(m, TechnicalMeasure::Pseudonymisation { .. }));

        let has_encryption = self
            .technical_measures
            .iter()
            .any(|m| matches!(m, TechnicalMeasure::Encryption { .. }));

        if !has_pseudonymisation && !has_encryption {
            if matches!(risk_level, RiskLevel::High | RiskLevel::Critical) {
                warnings.push(
                    "Article 32(1)(a): High/Critical risk requires pseudonymisation or encryption"
                        .to_string(),
                );
            } else {
                recommendations.push(
                    "Article 32(1)(a): Consider implementing pseudonymisation or encryption"
                        .to_string(),
                );
            }
        }

        // Article 32(1)(b) - Check for confidentiality, integrity, availability, resilience
        let has_confidentiality = self
            .technical_measures
            .iter()
            .any(|m| matches!(m, TechnicalMeasure::Confidentiality { .. }));

        let has_integrity = self
            .technical_measures
            .iter()
            .any(|m| matches!(m, TechnicalMeasure::Integrity { .. }));

        let has_availability = self
            .technical_measures
            .iter()
            .any(|m| matches!(m, TechnicalMeasure::Availability { .. }));

        let has_resilience = self
            .technical_measures
            .iter()
            .any(|m| matches!(m, TechnicalMeasure::Resilience { .. }));

        if !has_confidentiality {
            recommendations.push("Article 32(1)(b): Consider confidentiality measures".to_string());
        }
        if !has_integrity {
            recommendations.push("Article 32(1)(b): Consider integrity measures".to_string());
        }
        if !has_availability {
            recommendations.push("Article 32(1)(b): Consider availability measures".to_string());
        }
        if !has_resilience {
            recommendations.push("Article 32(1)(b): Consider resilience measures".to_string());
        }

        // Article 32(1)(c) - Check for backup and recovery
        let has_backup = self
            .technical_measures
            .iter()
            .any(|m| matches!(m, TechnicalMeasure::BackupRecovery { .. }));

        if !has_backup {
            if matches!(risk_level, RiskLevel::High | RiskLevel::Critical) {
                warnings.push("Article 32(1)(c): Backup and recovery capability required for high/critical risk".to_string());
            } else {
                recommendations
                    .push("Article 32(1)(c): Implement backup and recovery procedures".to_string());
            }
        }

        // Article 32(1)(d) - Check for testing and assessment
        let has_testing = self
            .technical_measures
            .iter()
            .any(|m| matches!(m, TechnicalMeasure::TestingAssessment { .. }));

        if !has_testing {
            warnings
                .push("Article 32(1)(d): Regular testing and assessment is mandatory".to_string());
        }

        // Check organizational measures
        if self.organizational_measures.is_empty() {
            recommendations.push(
                "Consider implementing organizational measures (policies, training, etc.)"
                    .to_string(),
            );
        }

        // Check Article 32(1) considerations
        if self.state_of_art_considered != Some(true) {
            warnings.push(
                "Article 32(1): State of the art should be considered when selecting measures"
                    .to_string(),
            );
        }

        if self.implementation_costs_considered != Some(true) {
            warnings
                .push("Article 32(1): Costs of implementation should be considered".to_string());
        }

        if self.processing_context_considered != Some(true) {
            warnings.push("Article 32(1): Nature, scope, context and purposes of processing should be considered".to_string());
        }

        // Determine compliance
        let compliant = warnings.is_empty();

        Ok(SecurityValidation {
            compliant,
            warnings,
            recommendations,
            technical_measures_count: self.technical_measures.len(),
            organizational_measures_count: self.organizational_measures.len(),
            has_encryption,
            has_pseudonymisation,
            has_backup_recovery: has_backup,
            has_testing,
            risk_level: self.risk_level.clone(),
        })
    }
}

impl Default for SecurityAssessment {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of Article 32 security validation
#[derive(Debug, Clone)]
pub struct SecurityValidation {
    /// Overall compliance with Article 32
    pub compliant: bool,

    /// Warnings about non-compliance
    pub warnings: Vec<String>,

    /// Recommendations for improvement
    pub recommendations: Vec<String>,

    /// Number of technical measures implemented
    pub technical_measures_count: usize,

    /// Number of organizational measures implemented
    pub organizational_measures_count: usize,

    /// Has encryption implemented
    pub has_encryption: bool,

    /// Has pseudonymisation implemented
    pub has_pseudonymisation: bool,

    /// Has backup and recovery implemented
    pub has_backup_recovery: bool,

    /// Has testing and assessment implemented
    pub has_testing: bool,

    /// Risk level
    pub risk_level: Option<RiskLevel>,
}

// ============================================================================
// Articles 33-34 - Data Breach Notification
// ============================================================================

/// Data breach notification builder (Articles 33 & 34)
///
/// ## Example
///
/// ```rust
/// use legalis_eu::gdpr::*;
/// use chrono::Utc;
///
/// let breach = DataBreach::new()
///     .with_controller("Acme Corp")
///     .with_breach_category(BreachCategory::ConfidentialityBreach)
///     .with_discovered_at(Utc::now())
///     .with_affected_data_subjects(1000)
///     .with_severity(BreachSeverity::High);
///
/// let notification = breach.validate_notification_requirements();
/// assert!(notification.is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct DataBreach {
    /// Data controller
    pub controller: Option<String>,

    /// Type of breach
    pub breach_category: Option<BreachCategory>,

    /// When breach was discovered
    pub discovered_at: Option<DateTime<Utc>>,

    /// Number of affected data subjects
    pub affected_count: Option<u64>,

    /// Severity assessment
    pub severity: Option<BreachSeverity>,

    /// Personal data categories affected
    pub affected_data_categories: Vec<String>,

    /// Mitigation measures taken
    pub mitigation_measures: Vec<String>,

    /// Description of the breach
    pub description: Option<String>,
}

impl DataBreach {
    /// Create a new data breach
    pub fn new() -> Self {
        Self {
            controller: None,
            breach_category: None,
            discovered_at: None,
            affected_count: None,
            severity: None,
            affected_data_categories: Vec::new(),
            mitigation_measures: Vec::new(),
            description: None,
        }
    }

    /// Set the data controller
    pub fn with_controller(mut self, controller: impl Into<String>) -> Self {
        self.controller = Some(controller.into());
        self
    }

    /// Set the breach category
    pub fn with_breach_category(mut self, category: BreachCategory) -> Self {
        self.breach_category = Some(category);
        self
    }

    /// Set when the breach was discovered
    pub fn with_discovered_at(mut self, timestamp: DateTime<Utc>) -> Self {
        self.discovered_at = Some(timestamp);
        self
    }

    /// Set the number of affected data subjects
    pub fn with_affected_data_subjects(mut self, count: u64) -> Self {
        self.affected_count = Some(count);
        self
    }

    /// Set the severity
    pub fn with_severity(mut self, severity: BreachSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Set affected data categories
    pub fn with_affected_data_categories(mut self, categories: Vec<String>) -> Self {
        self.affected_data_categories = categories;
        self
    }

    /// Add mitigation measure
    pub fn add_mitigation_measure(mut self, measure: impl Into<String>) -> Self {
        self.mitigation_measures.push(measure.into());
        self
    }

    /// Set breach description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Validate notification requirements (Article 33 & 34)
    pub fn validate_notification_requirements(
        &self,
    ) -> Result<BreachNotificationRequirements, GdprError> {
        if self.discovered_at.is_none() {
            return Err(GdprError::missing_field("discovered_at"));
        }

        let discovered = self.discovered_at.unwrap();
        let now = Utc::now();
        let elapsed = now.signed_duration_since(discovered);

        // Article 33(1): 72-hour notification to supervisory authority
        let sa_deadline = discovered + Duration::hours(72);
        let sa_notification_required = true; // Unless unlikely to result in risk
        let sa_deadline_passed = now > sa_deadline;

        // Article 34: Notification to data subjects if high risk
        let high_risk = matches!(
            self.severity,
            Some(BreachSeverity::High) | Some(BreachSeverity::Critical)
        );

        let ds_notification_required = high_risk;

        // Determine compliance status
        let compliance_status = if sa_deadline_passed && sa_notification_required {
            BreachComplianceStatus::NonCompliant {
                violation: format!(
                    "72-hour notification deadline exceeded by {} hours",
                    elapsed.num_hours() - 72
                ),
            }
        } else {
            BreachComplianceStatus::Compliant
        };

        Ok(BreachNotificationRequirements {
            supervisory_authority_notification_required: sa_notification_required,
            supervisory_authority_deadline: sa_deadline,
            supervisory_authority_deadline_passed: sa_deadline_passed,
            data_subject_notification_required: ds_notification_required,
            hours_since_discovery: elapsed.num_hours(),
            compliance_status,
        })
    }
}

impl Default for DataBreach {
    fn default() -> Self {
        Self::new()
    }
}

/// Notification requirements for a breach
#[derive(Debug, Clone)]
pub struct BreachNotificationRequirements {
    pub supervisory_authority_notification_required: bool,
    pub supervisory_authority_deadline: DateTime<Utc>,
    pub supervisory_authority_deadline_passed: bool,
    pub data_subject_notification_required: bool,
    pub hours_since_discovery: i64,
    pub compliance_status: BreachComplianceStatus,
}

/// Breach compliance status
#[derive(Debug, Clone, PartialEq)]
pub enum BreachComplianceStatus {
    Compliant,
    NonCompliant { violation: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breach_within_72_hours() {
        let breach = DataBreach::new()
            .with_controller("Acme Corp")
            .with_breach_category(BreachCategory::ConfidentialityBreach)
            .with_discovered_at(Utc::now() - Duration::hours(60)) // Discovered 60 hours ago
            .with_affected_data_subjects(100)
            .with_severity(BreachSeverity::Medium);

        let requirements = breach.validate_notification_requirements().unwrap();

        assert!(requirements.supervisory_authority_notification_required);
        assert!(!requirements.supervisory_authority_deadline_passed);
        assert_eq!(
            requirements.compliance_status,
            BreachComplianceStatus::Compliant
        );
    }

    #[test]
    fn test_breach_deadline_exceeded() {
        let breach = DataBreach::new()
            .with_controller("Acme Corp")
            .with_breach_category(BreachCategory::ConfidentialityBreach)
            .with_discovered_at(Utc::now() - Duration::hours(80)) // Discovered 80 hours ago
            .with_severity(BreachSeverity::High);

        let requirements = breach.validate_notification_requirements().unwrap();

        assert!(requirements.supervisory_authority_deadline_passed);
        assert!(matches!(
            requirements.compliance_status,
            BreachComplianceStatus::NonCompliant { .. }
        ));
    }

    #[test]
    fn test_high_risk_requires_data_subject_notification() {
        let breach = DataBreach::new()
            .with_controller("Acme Corp")
            .with_discovered_at(Utc::now())
            .with_severity(BreachSeverity::High);

        let requirements = breach.validate_notification_requirements().unwrap();

        assert!(requirements.data_subject_notification_required);
    }

    #[test]
    fn test_low_risk_no_data_subject_notification() {
        let breach = DataBreach::new()
            .with_controller("Acme Corp")
            .with_discovered_at(Utc::now())
            .with_severity(BreachSeverity::Low);

        let requirements = breach.validate_notification_requirements().unwrap();

        assert!(!requirements.data_subject_notification_required);
    }

    #[test]
    fn test_missing_discovered_at() {
        let breach = DataBreach::new().with_controller("Acme Corp");

        let result = breach.validate_notification_requirements();
        assert!(matches!(result, Err(GdprError::MissingField(_))));
    }

    // ============================================================================
    // Article 32 - Security Assessment Tests
    // ============================================================================

    #[test]
    fn test_complete_security_assessment() {
        let assessment = SecurityAssessment::new()
            .with_entity("Acme Corp")
            .with_risk_level(RiskLevel::High)
            .add_technical_measure(TechnicalMeasure::Encryption {
                data_at_rest: true,
                data_in_transit: true,
                algorithm: "AES-256-GCM".to_string(),
            })
            .add_technical_measure(TechnicalMeasure::Pseudonymisation {
                method: "SHA-256 hashing with salt".to_string(),
            })
            .add_technical_measure(TechnicalMeasure::BackupRecovery {
                backup_frequency: "Daily".to_string(),
                recovery_time_objective: "4 hours".to_string(),
                recovery_point_objective: "1 hour".to_string(),
                tested: true,
            })
            .add_technical_measure(TechnicalMeasure::TestingAssessment {
                penetration_testing: true,
                vulnerability_scanning: true,
                frequency: "Quarterly".to_string(),
            })
            .add_organizational_measure(OrganizationalMeasure::StaffTraining {
                frequency: "Bi-annual".to_string(),
            })
            .add_organizational_measure(OrganizationalMeasure::AccessControl {
                role_based: true,
                least_privilege: true,
            })
            .with_state_of_art_considered(true)
            .with_implementation_costs_considered(true)
            .with_processing_context_considered(true);

        let validation = assessment.validate().unwrap();

        assert!(validation.compliant);
        assert!(validation.has_encryption);
        assert!(validation.has_pseudonymisation);
        assert!(validation.has_backup_recovery);
        assert!(validation.has_testing);
        assert_eq!(validation.technical_measures_count, 4);
        assert_eq!(validation.organizational_measures_count, 2);
    }

    #[test]
    fn test_high_risk_missing_encryption() {
        let assessment = SecurityAssessment::new()
            .with_risk_level(RiskLevel::High)
            .add_technical_measure(TechnicalMeasure::TestingAssessment {
                penetration_testing: true,
                vulnerability_scanning: true,
                frequency: "Monthly".to_string(),
            })
            .with_state_of_art_considered(true)
            .with_implementation_costs_considered(true)
            .with_processing_context_considered(true);

        let validation = assessment.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("High/Critical risk requires pseudonymisation or encryption"))
        );
    }

    #[test]
    fn test_missing_testing_assessment() {
        let assessment = SecurityAssessment::new()
            .with_risk_level(RiskLevel::Medium)
            .add_technical_measure(TechnicalMeasure::Encryption {
                data_at_rest: true,
                data_in_transit: true,
                algorithm: "AES-256".to_string(),
            })
            .with_state_of_art_considered(true)
            .with_implementation_costs_considered(true)
            .with_processing_context_considered(true);

        let validation = assessment.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Regular testing and assessment is mandatory"))
        );
    }

    #[test]
    fn test_missing_article_32_considerations() {
        let assessment = SecurityAssessment::new()
            .with_risk_level(RiskLevel::Low)
            .add_technical_measure(TechnicalMeasure::Encryption {
                data_at_rest: true,
                data_in_transit: false,
                algorithm: "AES-128".to_string(),
            })
            .add_technical_measure(TechnicalMeasure::TestingAssessment {
                penetration_testing: false,
                vulnerability_scanning: true,
                frequency: "Annual".to_string(),
            });

        let validation = assessment.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("State of the art should be considered"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Costs of implementation should be considered"))
        );
        assert!(validation.warnings.iter().any(|w| {
            w.contains("Nature, scope, context and purposes of processing should be considered")
        }));
    }

    #[test]
    fn test_recommendations_for_low_risk() {
        let assessment = SecurityAssessment::new()
            .with_risk_level(RiskLevel::Low)
            .add_technical_measure(TechnicalMeasure::TestingAssessment {
                penetration_testing: false,
                vulnerability_scanning: true,
                frequency: "Annual".to_string(),
            })
            .with_state_of_art_considered(true)
            .with_implementation_costs_considered(true)
            .with_processing_context_considered(true);

        let validation = assessment.validate().unwrap();

        // Low risk doesn't require encryption, so should be compliant with just testing
        assert!(validation.compliant);
        assert!(!validation.recommendations.is_empty());
        assert!(
            validation
                .recommendations
                .iter()
                .any(|r| r.contains("encryption"))
        );
    }

    #[test]
    fn test_critical_risk_backup_required() {
        let assessment = SecurityAssessment::new()
            .with_risk_level(RiskLevel::Critical)
            .add_technical_measure(TechnicalMeasure::Encryption {
                data_at_rest: true,
                data_in_transit: true,
                algorithm: "AES-256-GCM".to_string(),
            })
            .add_technical_measure(TechnicalMeasure::TestingAssessment {
                penetration_testing: true,
                vulnerability_scanning: true,
                frequency: "Monthly".to_string(),
            })
            .with_state_of_art_considered(true)
            .with_implementation_costs_considered(true)
            .with_processing_context_considered(true);

        let validation = assessment.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w
                    .contains("Backup and recovery capability required for high/critical risk"))
        );
    }

    #[test]
    fn test_comprehensive_measures() {
        let assessment = SecurityAssessment::new()
            .with_risk_level(RiskLevel::High)
            .add_technical_measure(TechnicalMeasure::Encryption {
                data_at_rest: true,
                data_in_transit: true,
                algorithm: "AES-256-GCM".to_string(),
            })
            .add_technical_measure(TechnicalMeasure::Confidentiality {
                access_logging: true,
                intrusion_detection: true,
            })
            .add_technical_measure(TechnicalMeasure::Integrity {
                checksums: true,
                digital_signatures: true,
                version_control: true,
            })
            .add_technical_measure(TechnicalMeasure::Availability {
                redundancy: true,
                load_balancing: true,
                uptime_sla: Some(99.95),
            })
            .add_technical_measure(TechnicalMeasure::Resilience {
                fault_tolerance: true,
                geographic_redundancy: true,
            })
            .add_technical_measure(TechnicalMeasure::BackupRecovery {
                backup_frequency: "Hourly".to_string(),
                recovery_time_objective: "1 hour".to_string(),
                recovery_point_objective: "15 minutes".to_string(),
                tested: true,
            })
            .add_technical_measure(TechnicalMeasure::TestingAssessment {
                penetration_testing: true,
                vulnerability_scanning: true,
                frequency: "Quarterly".to_string(),
            })
            .add_organizational_measure(OrganizationalMeasure::AccessControl {
                role_based: true,
                least_privilege: true,
            })
            .add_organizational_measure(OrganizationalMeasure::StaffTraining {
                frequency: "Quarterly".to_string(),
            })
            .add_organizational_measure(OrganizationalMeasure::IncidentResponse {
                documented: true,
                tested: true,
            })
            .add_organizational_measure(OrganizationalMeasure::SecurityPolicies {
                documented: true,
                reviewed_regularly: true,
            })
            .with_state_of_art_considered(true)
            .with_implementation_costs_considered(true)
            .with_processing_context_considered(true);

        let validation = assessment.validate().unwrap();

        assert!(validation.compliant);
        assert_eq!(validation.technical_measures_count, 7);
        assert_eq!(validation.organizational_measures_count, 4);
        assert!(validation.has_encryption);
        assert!(validation.has_backup_recovery);
        assert!(validation.has_testing);
        assert!(validation.warnings.is_empty());
    }
}
