//! GDPR Article 24 - Responsibility of the Controller
//!
//! This module implements Article 24 of the GDPR, which establishes the controller's
//! general accountability obligation to implement appropriate technical and organizational
//! measures to ensure and demonstrate GDPR compliance.
//!
//! ## Article 24 Structure
//!
//! - **Article 24(1)**: Controller must implement appropriate technical and organizational measures
//!   taking into account:
//!   - Nature, scope, context, and purposes of processing
//!   - Risks of varying likelihood and severity for rights and freedoms
//!   - Ability to demonstrate compliance with the regulation
//!
//! - **Article 24(2)**: Approved codes of conduct and certification mechanisms as elements
//!   to demonstrate compliance
//!
//! - **Article 24(3)**: Accountability principle - controller is responsible for and must
//!   be able to demonstrate compliance with Article 5 principles
//!
//! ## Key Concepts
//!
//! Article 24 is the **foundational accountability principle** that underpins:
//! - Article 5 - Data protection principles (lawfulness, fairness, transparency, etc.)
//! - Article 25 - Data protection by design and by default
//! - Article 28 - Processor contracts
//! - Article 30 - Records of processing activities
//! - Article 32 - Security of processing
//! - Article 35 - Data protection impact assessment
//! - Articles 37-39 - Data protection officer
//!
//! ## Implementation Approach
//!
//! This module provides an **accountability framework** that integrates with all other
//! GDPR modules to assess overall controller compliance.
//!
//! ## References
//!
//! - EUR-Lex: CELEX:32016R0679 - Article 24
//! - Recital 74: Accountability principle
//! - Recital 78: Appropriate technical and organizational measures
//! - Article 5(2): Accountability for Article 5 principles
//!
//! ## Example
//!
//! ```rust
//! use legalis_eu::gdpr::*;
//! use chrono::Utc;
//!
//! let accountability = ControllerAccountability::new()
//!     .with_controller_name("Acme Corp")
//!     .with_processing_description("Customer data for e-commerce")
//!     .with_data_volume(DataVolume::Medium)
//!     .with_data_sensitivity(DataSensitivity::High)
//!     .with_risk_level_assessed(SecurityRiskLevel::High)
//!     .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
//!         implemented: true,
//!         documented: true,
//!         notes: Some("Privacy-preserving defaults implemented".to_string()),
//!     })
//!     .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
//!         article32_compliant: true,
//!         documented: true,
//!         notes: Some("Encryption, access controls, testing in place".to_string()),
//!     })
//!     .add_organizational_measure(AccountabilityMeasure::RecordsOfProcessing {
//!         ropa_maintained: true,
//!         up_to_date: true,
//!         notes: Some("Article 30 ROPA maintained and reviewed quarterly".to_string()),
//!     })
//!     .with_compliance_documentation(true)
//!     .with_assessment_date(Utc::now());
//!
//! match accountability.validate() {
//!     Ok(validation) => {
//!         if validation.compliant {
//!             println!("✅ Controller demonstrates Article 24 accountability");
//!             println!("Compliance score: {}/100", validation.compliance_score);
//!         } else {
//!             println!("❌ Accountability gaps identified:");
//!             for warning in &validation.warnings {
//!                 println!("  - {}", warning);
//!             }
//!         }
//!     }
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```

use crate::gdpr::error::GdprError;
use crate::gdpr::security::RiskLevel as SecurityRiskLevel;
use chrono::{DateTime, Utc};

/// Data volume categories for proportionality assessment (Article 24(1))
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DataVolume {
    /// Small scale (<1,000 data subjects)
    Small,
    /// Medium scale (1,000-100,000 data subjects)
    Medium,
    /// Large scale (100,000-1 million data subjects)
    Large,
    /// Very large scale (>1 million data subjects)
    VeryLarge,
}

/// Data sensitivity categories for risk assessment (Article 24(1))
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DataSensitivity {
    /// Regular personal data (name, email, etc.)
    Low,
    /// Sensitive personal data (financial, location, etc.)
    Medium,
    /// Special categories (Article 9) or criminal convictions (Article 10)
    High,
    /// Children's data + special categories
    Critical,
}

/// Accountability measures that controllers must implement (Article 24(1))
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccountabilityMeasure {
    /// Article 25 - Data protection by design and by default
    DataProtectionByDesign {
        implemented: bool,
        documented: bool,
        notes: Option<String>,
    },

    /// Article 32 - Security of processing
    SecurityMeasures {
        article32_compliant: bool,
        documented: bool,
        notes: Option<String>,
    },

    /// Article 30 - Records of processing activities (ROPA)
    RecordsOfProcessing {
        ropa_maintained: bool,
        up_to_date: bool,
        notes: Option<String>,
    },

    /// Article 35 - Data protection impact assessment (DPIA)
    DataProtectionImpactAssessment {
        dpia_required: bool,
        dpia_conducted: bool,
        notes: Option<String>,
    },

    /// Articles 37-39 - Data protection officer (DPO)
    DataProtectionOfficer {
        dpo_required: bool,
        dpo_designated: bool,
        contact_published: bool,
        notes: Option<String>,
    },

    /// Article 28 - Processor contracts
    ProcessorContracts {
        processors_identified: bool,
        article28_contracts_in_place: bool,
        notes: Option<String>,
    },

    /// Article 26 - Joint controller arrangements
    JointControllerArrangements {
        joint_controllers: bool,
        article26_arrangement_documented: bool,
        notes: Option<String>,
    },

    /// Article 44-49 - International data transfers
    InternationalTransfers {
        transfers_outside_eea: bool,
        chapter5_compliant: bool,
        notes: Option<String>,
    },

    /// Data subject rights procedures (Articles 15-22)
    DataSubjectRightsProcedures {
        procedures_documented: bool,
        response_process_established: bool,
        notes: Option<String>,
    },

    /// Breach notification procedures (Articles 33-34)
    BreachNotificationProcedures {
        procedures_documented: bool,
        tested: bool,
        notes: Option<String>,
    },

    /// Staff training and awareness
    StaffTraining {
        training_program_established: bool,
        frequency: Option<String>,
        notes: Option<String>,
    },

    /// Privacy notices and transparency (Articles 13-14)
    PrivacyNotices {
        provided: bool,
        compliant_with_article13_14: bool,
        notes: Option<String>,
    },

    /// Other custom accountability measure
    Other {
        name: String,
        implemented: bool,
        documented: bool,
        description: String,
    },
}

impl AccountabilityMeasure {
    /// Returns true if the measure is implemented
    pub fn is_implemented(&self) -> bool {
        match self {
            AccountabilityMeasure::DataProtectionByDesign { implemented, .. } => *implemented,
            AccountabilityMeasure::SecurityMeasures {
                article32_compliant,
                ..
            } => *article32_compliant,
            AccountabilityMeasure::RecordsOfProcessing {
                ropa_maintained, ..
            } => *ropa_maintained,
            AccountabilityMeasure::DataProtectionImpactAssessment {
                dpia_required,
                dpia_conducted,
                ..
            } => !dpia_required || *dpia_conducted,
            AccountabilityMeasure::DataProtectionOfficer {
                dpo_required,
                dpo_designated,
                ..
            } => !dpo_required || *dpo_designated,
            AccountabilityMeasure::ProcessorContracts {
                processors_identified,
                article28_contracts_in_place,
                ..
            } => *processors_identified && *article28_contracts_in_place,
            AccountabilityMeasure::JointControllerArrangements {
                joint_controllers,
                article26_arrangement_documented,
                ..
            } => !joint_controllers || *article26_arrangement_documented,
            AccountabilityMeasure::InternationalTransfers {
                transfers_outside_eea,
                chapter5_compliant,
                ..
            } => !transfers_outside_eea || *chapter5_compliant,
            AccountabilityMeasure::DataSubjectRightsProcedures {
                procedures_documented,
                response_process_established,
                ..
            } => *procedures_documented && *response_process_established,
            AccountabilityMeasure::BreachNotificationProcedures {
                procedures_documented,
                tested,
                ..
            } => *procedures_documented && *tested,
            AccountabilityMeasure::StaffTraining {
                training_program_established,
                ..
            } => *training_program_established,
            AccountabilityMeasure::PrivacyNotices {
                provided,
                compliant_with_article13_14,
                ..
            } => *provided && *compliant_with_article13_14,
            AccountabilityMeasure::Other { implemented, .. } => *implemented,
        }
    }

    /// Returns true if the measure is documented
    pub fn is_documented(&self) -> bool {
        match self {
            AccountabilityMeasure::DataProtectionByDesign { documented, .. } => *documented,
            AccountabilityMeasure::SecurityMeasures { documented, .. } => *documented,
            AccountabilityMeasure::RecordsOfProcessing { .. } => true, // ROPA is documentation
            AccountabilityMeasure::DataProtectionImpactAssessment { .. } => true, // DPIA is documentation
            AccountabilityMeasure::DataProtectionOfficer {
                contact_published, ..
            } => *contact_published,
            AccountabilityMeasure::ProcessorContracts {
                article28_contracts_in_place,
                ..
            } => *article28_contracts_in_place,
            AccountabilityMeasure::JointControllerArrangements {
                article26_arrangement_documented,
                ..
            } => *article26_arrangement_documented,
            AccountabilityMeasure::InternationalTransfers { .. } => true, // Chapter 5 mechanisms are documented
            AccountabilityMeasure::DataSubjectRightsProcedures {
                procedures_documented,
                ..
            } => *procedures_documented,
            AccountabilityMeasure::BreachNotificationProcedures {
                procedures_documented,
                ..
            } => *procedures_documented,
            AccountabilityMeasure::StaffTraining { .. } => true, // Training programs are documented
            AccountabilityMeasure::PrivacyNotices { provided, .. } => *provided,
            AccountabilityMeasure::Other { documented, .. } => *documented,
        }
    }

    /// Returns the GDPR article(s) this measure addresses
    pub fn article_reference(&self) -> &'static str {
        match self {
            AccountabilityMeasure::DataProtectionByDesign { .. } => "Article 25",
            AccountabilityMeasure::SecurityMeasures { .. } => "Article 32",
            AccountabilityMeasure::RecordsOfProcessing { .. } => "Article 30",
            AccountabilityMeasure::DataProtectionImpactAssessment { .. } => "Article 35",
            AccountabilityMeasure::DataProtectionOfficer { .. } => "Articles 37-39",
            AccountabilityMeasure::ProcessorContracts { .. } => "Article 28",
            AccountabilityMeasure::JointControllerArrangements { .. } => "Article 26",
            AccountabilityMeasure::InternationalTransfers { .. } => "Articles 44-49",
            AccountabilityMeasure::DataSubjectRightsProcedures { .. } => "Articles 15-22",
            AccountabilityMeasure::BreachNotificationProcedures { .. } => "Articles 33-34",
            AccountabilityMeasure::StaffTraining { .. } => "Article 32(4)",
            AccountabilityMeasure::PrivacyNotices { .. } => "Articles 13-14",
            AccountabilityMeasure::Other { .. } => "Article 24",
        }
    }
}

/// Compliance certification or code of conduct (Article 24(2))
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ComplianceCertification {
    /// Article 40 - Approved code of conduct
    CodeOfConduct {
        code_name: String,
        approval_authority: String,
        valid_until: Option<DateTime<Utc>>,
    },

    /// Article 42 - Certification mechanism
    Certification {
        certification_name: String,
        certifying_body: String,
        valid_until: Option<DateTime<Utc>>,
    },

    /// ISO/IEC 27001 or similar information security standard
    InformationSecurity {
        standard: String,
        certified: bool,
        valid_until: Option<DateTime<Utc>>,
    },

    /// Other industry-specific certification
    Other {
        name: String,
        description: String,
        valid_until: Option<DateTime<Utc>>,
    },
}

/// Controller's accountability assessment (Article 24)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ControllerAccountability {
    /// Controller name/identifier
    pub controller_name: Option<String>,

    /// Description of processing operations
    pub processing_description: Option<String>,

    /// Data volume for proportionality assessment
    pub data_volume: Option<DataVolume>,

    /// Data sensitivity for risk assessment
    pub data_sensitivity: Option<DataSensitivity>,

    /// Overall risk level assessed (Article 24(1) consideration)
    pub risk_level_assessed: Option<SecurityRiskLevel>,

    /// Technical accountability measures implemented
    pub technical_measures: Vec<AccountabilityMeasure>,

    /// Organizational accountability measures implemented
    pub organizational_measures: Vec<AccountabilityMeasure>,

    /// Compliance certifications or codes of conduct (Article 24(2))
    pub certifications: Vec<ComplianceCertification>,

    /// Whether compliance documentation is maintained
    pub compliance_documentation_maintained: Option<bool>,

    /// Nature of processing considered (Article 24(1))
    pub nature_of_processing_considered: Option<bool>,

    /// Scope of processing considered (Article 24(1))
    pub scope_of_processing_considered: Option<bool>,

    /// Context of processing considered (Article 24(1))
    pub context_of_processing_considered: Option<bool>,

    /// Purposes of processing considered (Article 24(1))
    pub purposes_of_processing_considered: Option<bool>,

    /// Date of assessment
    pub assessment_date: Option<DateTime<Utc>>,

    /// Person responsible for accountability
    pub responsible_person: Option<String>,

    /// Additional notes
    pub notes: Option<String>,
}

impl ControllerAccountability {
    /// Create a new accountability assessment
    pub fn new() -> Self {
        Self {
            controller_name: None,
            processing_description: None,
            data_volume: None,
            data_sensitivity: None,
            risk_level_assessed: None,
            technical_measures: Vec::new(),
            organizational_measures: Vec::new(),
            certifications: Vec::new(),
            compliance_documentation_maintained: None,
            nature_of_processing_considered: None,
            scope_of_processing_considered: None,
            context_of_processing_considered: None,
            purposes_of_processing_considered: None,
            assessment_date: None,
            responsible_person: None,
            notes: None,
        }
    }

    /// Set controller name
    pub fn with_controller_name(mut self, name: impl Into<String>) -> Self {
        self.controller_name = Some(name.into());
        self
    }

    /// Set processing description
    pub fn with_processing_description(mut self, description: impl Into<String>) -> Self {
        self.processing_description = Some(description.into());
        self
    }

    /// Set data volume
    pub fn with_data_volume(mut self, volume: DataVolume) -> Self {
        self.data_volume = Some(volume);
        self
    }

    /// Set data sensitivity
    pub fn with_data_sensitivity(mut self, sensitivity: DataSensitivity) -> Self {
        self.data_sensitivity = Some(sensitivity);
        self
    }

    /// Set risk level
    pub fn with_risk_level_assessed(mut self, risk: SecurityRiskLevel) -> Self {
        self.risk_level_assessed = Some(risk);
        self
    }

    /// Add a technical accountability measure
    pub fn add_technical_measure(mut self, measure: AccountabilityMeasure) -> Self {
        self.technical_measures.push(measure);
        self
    }

    /// Add multiple technical measures
    pub fn with_technical_measures(mut self, measures: Vec<AccountabilityMeasure>) -> Self {
        self.technical_measures = measures;
        self
    }

    /// Add an organizational accountability measure
    pub fn add_organizational_measure(mut self, measure: AccountabilityMeasure) -> Self {
        self.organizational_measures.push(measure);
        self
    }

    /// Add multiple organizational measures
    pub fn with_organizational_measures(mut self, measures: Vec<AccountabilityMeasure>) -> Self {
        self.organizational_measures = measures;
        self
    }

    /// Add a compliance certification
    pub fn add_certification(mut self, certification: ComplianceCertification) -> Self {
        self.certifications.push(certification);
        self
    }

    /// Set compliance documentation status
    pub fn with_compliance_documentation(mut self, maintained: bool) -> Self {
        self.compliance_documentation_maintained = Some(maintained);
        self
    }

    /// Set nature of processing considered
    pub fn with_nature_considered(mut self, considered: bool) -> Self {
        self.nature_of_processing_considered = Some(considered);
        self
    }

    /// Set scope of processing considered
    pub fn with_scope_considered(mut self, considered: bool) -> Self {
        self.scope_of_processing_considered = Some(considered);
        self
    }

    /// Set context of processing considered
    pub fn with_context_considered(mut self, considered: bool) -> Self {
        self.context_of_processing_considered = Some(considered);
        self
    }

    /// Set purposes of processing considered
    pub fn with_purposes_considered(mut self, considered: bool) -> Self {
        self.purposes_of_processing_considered = Some(considered);
        self
    }

    /// Set all Article 24(1) considerations at once
    pub fn with_all_considerations(mut self, considered: bool) -> Self {
        self.nature_of_processing_considered = Some(considered);
        self.scope_of_processing_considered = Some(considered);
        self.context_of_processing_considered = Some(considered);
        self.purposes_of_processing_considered = Some(considered);
        self
    }

    /// Set assessment date
    pub fn with_assessment_date(mut self, date: DateTime<Utc>) -> Self {
        self.assessment_date = Some(date);
        self
    }

    /// Set responsible person
    pub fn with_responsible_person(mut self, person: impl Into<String>) -> Self {
        self.responsible_person = Some(person.into());
        self
    }

    /// Set notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Validate accountability and demonstrate compliance (Article 24)
    pub fn validate(&self) -> Result<Article24Validation, GdprError> {
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_score = 0u32;
        let mut max_score = 0u32;

        // 1. Check controller name is provided
        if self.controller_name.is_none() {
            return Err(GdprError::missing_field(
                "controller_name (required for accountability assessment)",
            ));
        }

        // 2. Check Article 24(1) considerations (MANDATORY)
        max_score += 40;
        let mut considerations_met = 0;

        if !self.nature_of_processing_considered.unwrap_or(false) {
            warnings.push("Article 24(1): Nature of processing not considered".to_string());
        } else {
            considerations_met += 10;
        }

        if !self.scope_of_processing_considered.unwrap_or(false) {
            warnings.push("Article 24(1): Scope of processing not considered".to_string());
        } else {
            considerations_met += 10;
        }

        if !self.context_of_processing_considered.unwrap_or(false) {
            warnings.push("Article 24(1): Context of processing not considered".to_string());
        } else {
            considerations_met += 10;
        }

        if !self.purposes_of_processing_considered.unwrap_or(false) {
            warnings.push("Article 24(1): Purposes of processing not considered".to_string());
        } else {
            considerations_met += 10;
        }

        compliance_score += considerations_met;

        // 3. Check compliance documentation (Article 24(1) - demonstrate compliance)
        max_score += 10;
        if !self.compliance_documentation_maintained.unwrap_or(false) {
            warnings.push(
                "Article 24(1): Compliance documentation must be maintained to demonstrate accountability".to_string(),
            );
        } else {
            compliance_score += 10;
        }

        // 4. Check technical and organizational measures
        let all_measures: Vec<&AccountabilityMeasure> = self
            .technical_measures
            .iter()
            .chain(self.organizational_measures.iter())
            .collect();

        if all_measures.is_empty() {
            warnings.push(
                "Article 24(1): No technical or organizational measures documented".to_string(),
            );
        }

        // 5. Evaluate essential accountability measures
        let has_article25 = all_measures.iter().any(|m| {
            matches!(
                m,
                AccountabilityMeasure::DataProtectionByDesign {
                    implemented: true,
                    ..
                }
            )
        });
        let has_article32 = all_measures.iter().any(|m| {
            matches!(
                m,
                AccountabilityMeasure::SecurityMeasures {
                    article32_compliant: true,
                    ..
                }
            )
        });
        let has_article30 = all_measures.iter().any(|m| {
            matches!(
                m,
                AccountabilityMeasure::RecordsOfProcessing {
                    ropa_maintained: true,
                    ..
                }
            )
        });

        // Article 25 - Required
        max_score += 15;
        if !has_article25 {
            warnings.push(
                "Article 25: Data protection by design and by default not implemented".to_string(),
            );
        } else {
            compliance_score += 15;
        }

        // Article 32 - Required
        max_score += 15;
        if !has_article32 {
            warnings
                .push("Article 32: Security measures not implemented or not compliant".to_string());
        } else {
            compliance_score += 15;
        }

        // Article 30 - Required (unless exempted)
        max_score += 10;
        if !has_article30 {
            warnings.push(
                "Article 30: Records of processing activities (ROPA) not maintained".to_string(),
            );
            recommendations.push("Maintain ROPA unless exempted under Article 30(5)".to_string());
        } else {
            compliance_score += 10;
        }

        // 6. Check for data protection impact assessment if high risk
        if matches!(
            self.risk_level_assessed,
            Some(SecurityRiskLevel::High) | Some(SecurityRiskLevel::Critical)
        ) {
            max_score += 10;
            let has_dpia = all_measures.iter().any(|m| {
                matches!(
                    m,
                    AccountabilityMeasure::DataProtectionImpactAssessment {
                        dpia_required: true,
                        dpia_conducted: true,
                        ..
                    }
                )
            });

            if !has_dpia {
                warnings.push(
                    "Article 35: High-risk processing requires Data Protection Impact Assessment (DPIA)".to_string(),
                );
            } else {
                compliance_score += 10;
            }
        }

        // 7. Calculate measure implementation rate
        let total_measures = all_measures.len();
        let implemented_measures = all_measures.iter().filter(|m| m.is_implemented()).count();
        let documented_measures = all_measures.iter().filter(|m| m.is_documented()).count();

        if total_measures > 0 {
            let implementation_rate = (implemented_measures as f64 / total_measures as f64) * 100.0;
            let documentation_rate = (documented_measures as f64 / total_measures as f64) * 100.0;

            if implementation_rate < 100.0 {
                recommendations.push(format!(
                    "{} of {} measures not fully implemented ({:.1}% implementation rate)",
                    total_measures - implemented_measures,
                    total_measures,
                    implementation_rate
                ));
            }

            if documentation_rate < 100.0 {
                recommendations.push(format!(
                    "{} of {} measures not documented ({:.1}% documentation rate)",
                    total_measures - documented_measures,
                    total_measures,
                    documentation_rate
                ));
            }
        }

        // 8. Check for Article 24(2) certifications or codes of conduct
        if !self.certifications.is_empty() {
            recommendations.push(format!(
                "Article 24(2): {} compliance certification(s) or code(s) of conduct in place (demonstrates compliance)",
                self.certifications.len()
            ));
        } else {
            recommendations.push(
                "Article 24(2): Consider adopting approved codes of conduct or certification mechanisms".to_string(),
            );
        }

        // 9. Final compliance determination
        let compliant = warnings.is_empty();
        let compliance_percentage = if max_score > 0 {
            ((compliance_score as f64 / max_score as f64) * 100.0).round() as u32
        } else {
            0
        };

        Ok(Article24Validation {
            compliant,
            warnings,
            recommendations,
            compliance_score: compliance_percentage,
            considerations_complete: considerations_met == 40,
            documentation_maintained: self.compliance_documentation_maintained.unwrap_or(false),
            technical_measures_count: self.technical_measures.len(),
            organizational_measures_count: self.organizational_measures.len(),
            certifications_count: self.certifications.len(),
            has_article25_compliance: has_article25,
            has_article32_compliance: has_article32,
            has_article30_compliance: has_article30,
        })
    }
}

impl Default for ControllerAccountability {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result for Article 24 accountability assessment
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Article24Validation {
    /// Whether the controller demonstrates Article 24 accountability
    pub compliant: bool,

    /// List of non-compliance warnings
    pub warnings: Vec<String>,

    /// List of recommendations for improvement
    pub recommendations: Vec<String>,

    /// Overall compliance score (0-100)
    pub compliance_score: u32,

    /// Whether all Article 24(1) considerations are complete
    pub considerations_complete: bool,

    /// Whether compliance documentation is maintained
    pub documentation_maintained: bool,

    /// Number of technical measures implemented
    pub technical_measures_count: usize,

    /// Number of organizational measures implemented
    pub organizational_measures_count: usize,

    /// Number of certifications or codes of conduct
    pub certifications_count: usize,

    /// Whether Article 25 (data protection by design) is implemented
    pub has_article25_compliance: bool,

    /// Whether Article 32 (security measures) is compliant
    pub has_article32_compliance: bool,

    /// Whether Article 30 (ROPA) is maintained
    pub has_article30_compliance: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_accountability_assessment() {
        let accountability = ControllerAccountability::new()
            .with_controller_name("Acme Corp")
            .with_processing_description("E-commerce customer data")
            .with_data_volume(DataVolume::Medium)
            .with_data_sensitivity(DataSensitivity::Medium)
            .with_risk_level_assessed(SecurityRiskLevel::Medium)
            .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
                implemented: true,
                documented: true,
                notes: Some("Privacy by design implemented".to_string()),
            })
            .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
                article32_compliant: true,
                documented: true,
                notes: Some("Article 32 compliant".to_string()),
            })
            .add_organizational_measure(AccountabilityMeasure::RecordsOfProcessing {
                ropa_maintained: true,
                up_to_date: true,
                notes: Some("ROPA maintained".to_string()),
            })
            .with_compliance_documentation(true)
            .with_all_considerations(true)
            .with_assessment_date(Utc::now());

        let validation = accountability.validate().unwrap();

        assert!(validation.compliant);
        assert_eq!(validation.warnings.len(), 0);
        assert_eq!(validation.compliance_score, 100);
        assert!(validation.considerations_complete);
        assert!(validation.documentation_maintained);
        assert!(validation.has_article25_compliance);
        assert!(validation.has_article32_compliance);
        assert!(validation.has_article30_compliance);
    }

    #[test]
    fn test_missing_article24_considerations() {
        let accountability = ControllerAccountability::new()
            .with_controller_name("Acme Corp")
            .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
                implemented: true,
                documented: true,
                notes: None,
            })
            .with_compliance_documentation(true);

        let validation = accountability.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Article 24(1): Nature of processing not considered"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Article 24(1): Scope of processing not considered"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Article 24(1): Context of processing not considered"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Article 24(1): Purposes of processing not considered"))
        );
        assert!(!validation.considerations_complete);
    }

    #[test]
    fn test_missing_compliance_documentation() {
        let accountability = ControllerAccountability::new()
            .with_controller_name("Acme Corp")
            .with_all_considerations(true)
            .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
                implemented: true,
                documented: true,
                notes: None,
            })
            .with_compliance_documentation(false);

        let validation = accountability.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Compliance documentation must be maintained"))
        );
        assert!(!validation.documentation_maintained);
    }

    #[test]
    fn test_missing_essential_measures() {
        let accountability = ControllerAccountability::new()
            .with_controller_name("Acme Corp")
            .with_all_considerations(true)
            .with_compliance_documentation(true);

        let validation = accountability.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Article 25: Data protection by design"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Article 32: Security measures"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Article 30: Records of processing"))
        );
        assert!(!validation.has_article25_compliance);
        assert!(!validation.has_article32_compliance);
        assert!(!validation.has_article30_compliance);
    }

    #[test]
    fn test_high_risk_requires_dpia() {
        let accountability = ControllerAccountability::new()
            .with_controller_name("Acme Corp")
            .with_risk_level_assessed(SecurityRiskLevel::High)
            .with_all_considerations(true)
            .with_compliance_documentation(true)
            .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
                implemented: true,
                documented: true,
                notes: None,
            })
            .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
                article32_compliant: true,
                documented: true,
                notes: None,
            })
            .add_organizational_measure(AccountabilityMeasure::RecordsOfProcessing {
                ropa_maintained: true,
                up_to_date: true,
                notes: None,
            });

        let validation = accountability.validate().unwrap();

        assert!(!validation.compliant);
        assert!(validation.warnings.iter().any(|w| w.contains(
            "Article 35: High-risk processing requires Data Protection Impact Assessment"
        )));
    }

    #[test]
    fn test_accountability_measure_article_references() {
        let measure_article25 = AccountabilityMeasure::DataProtectionByDesign {
            implemented: true,
            documented: true,
            notes: None,
        };
        assert_eq!(measure_article25.article_reference(), "Article 25");

        let measure_article32 = AccountabilityMeasure::SecurityMeasures {
            article32_compliant: true,
            documented: true,
            notes: None,
        };
        assert_eq!(measure_article32.article_reference(), "Article 32");

        let measure_article30 = AccountabilityMeasure::RecordsOfProcessing {
            ropa_maintained: true,
            up_to_date: true,
            notes: None,
        };
        assert_eq!(measure_article30.article_reference(), "Article 30");
    }

    #[test]
    fn test_certifications_provide_compliance_evidence() {
        let accountability = ControllerAccountability::new()
            .with_controller_name("Acme Corp")
            .with_all_considerations(true)
            .with_compliance_documentation(true)
            .add_technical_measure(AccountabilityMeasure::DataProtectionByDesign {
                implemented: true,
                documented: true,
                notes: None,
            })
            .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
                article32_compliant: true,
                documented: true,
                notes: None,
            })
            .add_organizational_measure(AccountabilityMeasure::RecordsOfProcessing {
                ropa_maintained: true,
                up_to_date: true,
                notes: None,
            })
            .add_certification(ComplianceCertification::InformationSecurity {
                standard: "ISO/IEC 27001:2022".to_string(),
                certified: true,
                valid_until: None,
            });

        let validation = accountability.validate().unwrap();

        assert!(validation.compliant);
        assert_eq!(validation.certifications_count, 1);
        assert!(
            validation
                .recommendations
                .iter()
                .any(|r| r.contains("1 compliance certification"))
        );
    }
}
