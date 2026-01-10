//! GDPR Article 25 - Data Protection by Design and by Default
//!
//! This module implements Article 25 requirements for data protection by design (DPbD)
//! and data protection by default (DPbDefault).
//!
//! ## Article 25(1) - Data Protection by Design
//!
//! Taking into account the state of the art, the cost of implementation and the nature,
//! scope, context and purposes of processing as well as the risks of varying likelihood
//! and severity for rights and freedoms of natural persons posed by the processing,
//! the controller shall implement appropriate technical and organisational measures designed
//! to implement data-protection principles (such as data minimisation) and to integrate
//! the necessary safeguards into the processing.
//!
//! ## Article 25(2) - Data Protection by Default
//!
//! The controller shall implement appropriate technical and organisational measures for
//! ensuring that, by default, only personal data which are necessary for each specific
//! purpose of the processing are processed. That obligation applies to the amount of
//! personal data collected, the extent of their processing, the period of their storage
//! and their accessibility.
//!
//! EUR-Lex: CELEX:32016R0679 Art. 25
//!
//! ## Example
//!
//! ```rust
//! use legalis_eu::gdpr::*;
//!
//! let dpbd = DataProtectionByDesign::new()
//!     .with_system_name("User Registration System")
//!     .with_processing_purpose("User account management")
//!     .add_design_principle(DesignPrinciple::DataMinimisation {
//!         only_necessary_data: true,
//!         justification: "Only collect email, name, password".to_string(),
//!     })
//!     .add_design_principle(DesignPrinciple::PurposeLimitation {
//!         limited_to_purpose: true,
//!         documented: true,
//!     })
//!     .add_default_setting(DefaultSetting::MinimalDataCollection {
//!         optional_fields_opt_in: true,
//!         no_pre_ticked_boxes: true,
//!     })
//!     .add_default_setting(DefaultSetting::LimitedAccessibility {
//!         need_to_know_basis: true,
//!         role_based_access: true,
//!     })
//!     .with_state_of_art_considered(true)
//!     .with_costs_considered(true)
//!     .with_risks_assessed(true);
//!
//! let validation = dpbd.validate();
//! assert!(validation.is_ok());
//! ```

use crate::gdpr::error::GdprError;
use chrono::{DateTime, Utc};

/// Data protection design principles from Article 25(1)
///
/// These implement the principles from Article 5 GDPR (lawfulness, fairness, transparency,
/// purpose limitation, data minimisation, accuracy, storage limitation, integrity, confidentiality).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DesignPrinciple {
    /// Data minimisation (Article 5(1)(c))
    ///
    /// Personal data shall be adequate, relevant and limited to what is necessary.
    DataMinimisation {
        only_necessary_data: bool,
        justification: String,
    },

    /// Purpose limitation (Article 5(1)(b))
    ///
    /// Data collected for specified, explicit and legitimate purposes, not further processed
    /// in a manner incompatible with those purposes.
    PurposeLimitation {
        limited_to_purpose: bool,
        documented: bool,
    },

    /// Storage limitation (Article 5(1)(e))
    ///
    /// Personal data kept in a form which permits identification for no longer than necessary.
    StorageLimitation {
        retention_period_defined: bool,
        automated_deletion: bool,
    },

    /// Accuracy (Article 5(1)(d))
    ///
    /// Personal data shall be accurate and, where necessary, kept up to date.
    Accuracy {
        validation_implemented: bool,
        correction_mechanism: bool,
    },

    /// Integrity and confidentiality (Article 5(1)(f))
    ///
    /// Processed in a manner that ensures appropriate security, including protection
    /// against unauthorised or unlawful processing and accidental loss.
    IntegrityConfidentiality {
        security_measures_integrated: bool,
        access_controls: bool,
    },

    /// Transparency (Article 5(1)(a))
    ///
    /// Processing must be transparent to the data subject.
    Transparency {
        privacy_notice: bool,
        clear_language: bool,
    },

    /// Lawfulness, fairness (Article 5(1)(a))
    ///
    /// Processing must be lawful and fair.
    LawfulnessFairness {
        lawful_basis_identified: bool,
        no_deceptive_practices: bool,
    },

    /// Accountability (Article 5(2))
    ///
    /// Controller responsible for demonstrating compliance.
    Accountability {
        documentation: bool,
        compliance_monitoring: bool,
    },
}

impl DesignPrinciple {
    /// Get the Article 5 reference for this principle
    pub fn article_reference(&self) -> &'static str {
        match self {
            DesignPrinciple::DataMinimisation { .. } => "Article 5(1)(c)",
            DesignPrinciple::PurposeLimitation { .. } => "Article 5(1)(b)",
            DesignPrinciple::StorageLimitation { .. } => "Article 5(1)(e)",
            DesignPrinciple::Accuracy { .. } => "Article 5(1)(d)",
            DesignPrinciple::IntegrityConfidentiality { .. } => "Article 5(1)(f)",
            DesignPrinciple::Transparency { .. } => "Article 5(1)(a)",
            DesignPrinciple::LawfulnessFairness { .. } => "Article 5(1)(a)",
            DesignPrinciple::Accountability { .. } => "Article 5(2)",
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            DesignPrinciple::DataMinimisation { .. } => {
                "Data minimisation - collect only necessary data"
            }
            DesignPrinciple::PurposeLimitation { .. } => {
                "Purpose limitation - process only for specified purposes"
            }
            DesignPrinciple::StorageLimitation { .. } => {
                "Storage limitation - retain only as long as necessary"
            }
            DesignPrinciple::Accuracy { .. } => "Accuracy - ensure data is accurate and up to date",
            DesignPrinciple::IntegrityConfidentiality { .. } => {
                "Integrity and confidentiality - appropriate security"
            }
            DesignPrinciple::Transparency { .. } => {
                "Transparency - clear information to data subjects"
            }
            DesignPrinciple::LawfulnessFairness { .. } => {
                "Lawfulness and fairness - lawful basis and no deception"
            }
            DesignPrinciple::Accountability { .. } => "Accountability - demonstrate compliance",
        }
    }
}

/// Default settings for Article 25(2) - Data Protection by Default
///
/// Ensures that by default, only necessary personal data is processed.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DefaultSetting {
    /// Minimal data collection by default
    ///
    /// Optional fields are opt-in, not pre-filled or pre-selected.
    MinimalDataCollection {
        optional_fields_opt_in: bool,
        no_pre_ticked_boxes: bool,
    },

    /// Limited processing extent
    ///
    /// Data processed only to the extent necessary for each purpose.
    LimitedProcessing {
        purpose_specific: bool,
        no_excessive_processing: bool,
    },

    /// Limited storage period
    ///
    /// Data retained only for necessary duration by default.
    LimitedStorage {
        shortest_necessary_period: bool,
        automatic_deletion: bool,
    },

    /// Limited accessibility
    ///
    /// Data accessible only to those who need it by default.
    LimitedAccessibility {
        need_to_know_basis: bool,
        role_based_access: bool,
    },

    /// Privacy-preserving defaults
    ///
    /// Most privacy-protective settings enabled by default.
    PrivacyPreservingDefaults {
        strictest_privacy_settings: bool,
        user_must_opt_out: bool,
    },

    /// Minimal disclosure to third parties
    ///
    /// No third-party sharing by default unless necessary.
    MinimalThirdPartyDisclosure {
        no_default_sharing: bool,
        explicit_consent_required: bool,
    },
}

impl DefaultSetting {
    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            DefaultSetting::MinimalDataCollection { .. } => {
                "Minimal data collection - optional fields are opt-in"
            }
            DefaultSetting::LimitedProcessing { .. } => {
                "Limited processing - only to extent necessary"
            }
            DefaultSetting::LimitedStorage { .. } => "Limited storage - shortest necessary period",
            DefaultSetting::LimitedAccessibility { .. } => {
                "Limited accessibility - need-to-know basis"
            }
            DefaultSetting::PrivacyPreservingDefaults { .. } => {
                "Privacy-preserving defaults - strictest settings enabled"
            }
            DefaultSetting::MinimalThirdPartyDisclosure { .. } => {
                "Minimal third-party disclosure - no default sharing"
            }
        }
    }
}

/// Privacy-enhancing technology (PET) used in system design
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrivacyEnhancingTechnology {
    /// Pseudonymisation (Article 32(1)(a))
    Pseudonymisation { method: String },

    /// Encryption (Article 32(1)(a))
    Encryption { algorithm: String },

    /// Anonymisation (removes personal data status)
    Anonymisation { technique: String },

    /// Differential privacy
    DifferentialPrivacy { epsilon: f64 },

    /// Secure multi-party computation
    SecureComputation { protocol: String },

    /// Homomorphic encryption
    HomomorphicEncryption { scheme: String },

    /// Zero-knowledge proofs
    ZeroKnowledgeProof { protocol: String },

    /// Other PET
    Other { name: String, description: String },
}

/// Data protection by design and by default assessment (Article 25)
///
/// Documents how a system implements data protection principles from the design stage
/// and ensures privacy-protective defaults.
#[derive(Debug, Clone)]
pub struct DataProtectionByDesign {
    /// System or processing operation name
    pub system_name: Option<String>,

    /// Processing purpose
    pub processing_purpose: Option<String>,

    /// Design principles implemented (Article 25(1))
    pub design_principles: Vec<DesignPrinciple>,

    /// Default settings (Article 25(2))
    pub default_settings: Vec<DefaultSetting>,

    /// Privacy-enhancing technologies used
    pub privacy_technologies: Vec<PrivacyEnhancingTechnology>,

    /// State of the art considered (Article 25(1))
    pub state_of_art_considered: Option<bool>,

    /// Implementation costs considered (Article 25(1))
    pub costs_considered: Option<bool>,

    /// Nature, scope, context, purposes of processing considered (Article 25(1))
    pub processing_context_considered: Option<bool>,

    /// Risks to rights and freedoms assessed (Article 25(1))
    pub risks_assessed: Option<bool>,

    /// Assessment or design review date
    pub assessment_date: Option<DateTime<Utc>>,

    /// Developer/designer responsible
    pub responsible_party: Option<String>,

    /// Notes or additional documentation
    pub notes: Option<String>,
}

impl DataProtectionByDesign {
    /// Create a new data protection by design assessment
    pub fn new() -> Self {
        Self {
            system_name: None,
            processing_purpose: None,
            design_principles: Vec::new(),
            default_settings: Vec::new(),
            privacy_technologies: Vec::new(),
            state_of_art_considered: None,
            costs_considered: None,
            processing_context_considered: None,
            risks_assessed: None,
            assessment_date: None,
            responsible_party: None,
            notes: None,
        }
    }

    /// Set the system name
    pub fn with_system_name(mut self, name: impl Into<String>) -> Self {
        self.system_name = Some(name.into());
        self
    }

    /// Set the processing purpose
    pub fn with_processing_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.processing_purpose = Some(purpose.into());
        self
    }

    /// Add a design principle
    pub fn add_design_principle(mut self, principle: DesignPrinciple) -> Self {
        self.design_principles.push(principle);
        self
    }

    /// Add a default setting
    pub fn add_default_setting(mut self, setting: DefaultSetting) -> Self {
        self.default_settings.push(setting);
        self
    }

    /// Add a privacy-enhancing technology
    pub fn add_privacy_technology(mut self, tech: PrivacyEnhancingTechnology) -> Self {
        self.privacy_technologies.push(tech);
        self
    }

    /// Mark that state of the art was considered
    pub fn with_state_of_art_considered(mut self, considered: bool) -> Self {
        self.state_of_art_considered = Some(considered);
        self
    }

    /// Mark that implementation costs were considered
    pub fn with_costs_considered(mut self, considered: bool) -> Self {
        self.costs_considered = Some(considered);
        self
    }

    /// Mark that processing context was considered
    pub fn with_processing_context_considered(mut self, considered: bool) -> Self {
        self.processing_context_considered = Some(considered);
        self
    }

    /// Mark that risks were assessed
    pub fn with_risks_assessed(mut self, assessed: bool) -> Self {
        self.risks_assessed = Some(assessed);
        self
    }

    /// Set assessment date
    pub fn with_assessment_date(mut self, date: DateTime<Utc>) -> Self {
        self.assessment_date = Some(date);
        self
    }

    /// Set responsible party
    pub fn with_responsible_party(mut self, party: impl Into<String>) -> Self {
        self.responsible_party = Some(party.into());
        self
    }

    /// Add notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Validate Article 25 compliance
    ///
    /// Checks whether the system implements data protection by design and by default.
    pub fn validate(&self) -> Result<Article25Validation, GdprError> {
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();

        // Check Article 25(1) considerations
        if self.state_of_art_considered != Some(true) {
            warnings
                .push("Article 25(1): State of the art should be considered in design".to_string());
        }

        if self.costs_considered != Some(true) {
            warnings.push("Article 25(1): Implementation costs should be considered".to_string());
        }

        if self.processing_context_considered != Some(true) {
            warnings.push("Article 25(1): Nature, scope, context and purposes of processing should be considered".to_string());
        }

        if self.risks_assessed != Some(true) {
            warnings
                .push("Article 25(1): Risks to rights and freedoms should be assessed".to_string());
        }

        // Check for key design principles
        let has_data_minimisation = self
            .design_principles
            .iter()
            .any(|p| matches!(p, DesignPrinciple::DataMinimisation { .. }));

        let has_purpose_limitation = self
            .design_principles
            .iter()
            .any(|p| matches!(p, DesignPrinciple::PurposeLimitation { .. }));

        let has_storage_limitation = self
            .design_principles
            .iter()
            .any(|p| matches!(p, DesignPrinciple::StorageLimitation { .. }));

        let has_security = self
            .design_principles
            .iter()
            .any(|p| matches!(p, DesignPrinciple::IntegrityConfidentiality { .. }));

        if !has_data_minimisation {
            warnings.push("Article 25(1) + Article 5(1)(c): Data minimisation principle should be implemented in design".to_string());
        }

        if !has_purpose_limitation {
            recommendations.push("Consider implementing purpose limitation in design".to_string());
        }

        if !has_storage_limitation {
            recommendations.push("Consider implementing storage limitation in design".to_string());
        }

        if !has_security {
            warnings.push("Article 25(1) + Article 5(1)(f): Security measures should be integrated into design".to_string());
        }

        // Check Article 25(2) default settings
        if self.default_settings.is_empty() {
            warnings.push(
                "Article 25(2): Data protection by default settings should be configured"
                    .to_string(),
            );
        } else {
            let has_minimal_collection = self
                .default_settings
                .iter()
                .any(|s| matches!(s, DefaultSetting::MinimalDataCollection { .. }));

            let has_limited_accessibility = self
                .default_settings
                .iter()
                .any(|s| matches!(s, DefaultSetting::LimitedAccessibility { .. }));

            if !has_minimal_collection {
                recommendations
                    .push("Article 25(2): Consider minimal data collection by default".to_string());
            }

            if !has_limited_accessibility {
                recommendations
                    .push("Article 25(2): Consider limited accessibility by default".to_string());
            }
        }

        // Determine compliance
        let compliant = warnings.is_empty();

        Ok(Article25Validation {
            compliant,
            warnings,
            recommendations,
            design_principles_count: self.design_principles.len(),
            default_settings_count: self.default_settings.len(),
            privacy_technologies_count: self.privacy_technologies.len(),
            has_data_minimisation,
            has_purpose_limitation,
            has_storage_limitation,
            has_security,
            has_default_settings: !self.default_settings.is_empty(),
        })
    }
}

impl Default for DataProtectionByDesign {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of Article 25 validation
#[derive(Debug, Clone)]
pub struct Article25Validation {
    /// Overall compliance with Article 25
    pub compliant: bool,

    /// Warnings about non-compliance
    pub warnings: Vec<String>,

    /// Recommendations for improvement
    pub recommendations: Vec<String>,

    /// Number of design principles implemented
    pub design_principles_count: usize,

    /// Number of default settings configured
    pub default_settings_count: usize,

    /// Number of privacy-enhancing technologies used
    pub privacy_technologies_count: usize,

    /// Has data minimisation principle
    pub has_data_minimisation: bool,

    /// Has purpose limitation principle
    pub has_purpose_limitation: bool,

    /// Has storage limitation principle
    pub has_storage_limitation: bool,

    /// Has security measures integrated
    pub has_security: bool,

    /// Has default settings configured
    pub has_default_settings: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_dpbd_assessment() {
        let dpbd = DataProtectionByDesign::new()
            .with_system_name("User Management System")
            .with_processing_purpose("User account and profile management")
            .add_design_principle(DesignPrinciple::DataMinimisation {
                only_necessary_data: true,
                justification: "Only collect essential user data".to_string(),
            })
            .add_design_principle(DesignPrinciple::PurposeLimitation {
                limited_to_purpose: true,
                documented: true,
            })
            .add_design_principle(DesignPrinciple::StorageLimitation {
                retention_period_defined: true,
                automated_deletion: true,
            })
            .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
                security_measures_integrated: true,
                access_controls: true,
            })
            .add_default_setting(DefaultSetting::MinimalDataCollection {
                optional_fields_opt_in: true,
                no_pre_ticked_boxes: true,
            })
            .add_default_setting(DefaultSetting::LimitedAccessibility {
                need_to_know_basis: true,
                role_based_access: true,
            })
            .add_privacy_technology(PrivacyEnhancingTechnology::Encryption {
                algorithm: "AES-256".to_string(),
            })
            .with_state_of_art_considered(true)
            .with_costs_considered(true)
            .with_processing_context_considered(true)
            .with_risks_assessed(true);

        let validation = dpbd.validate().unwrap();

        assert!(validation.compliant);
        assert!(validation.has_data_minimisation);
        assert!(validation.has_purpose_limitation);
        assert!(validation.has_storage_limitation);
        assert!(validation.has_security);
        assert!(validation.has_default_settings);
        assert_eq!(validation.design_principles_count, 4);
        assert_eq!(validation.default_settings_count, 2);
        assert_eq!(validation.privacy_technologies_count, 1);
    }

    #[test]
    fn test_missing_data_minimisation() {
        let dpbd = DataProtectionByDesign::new()
            .with_system_name("System")
            .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
                security_measures_integrated: true,
                access_controls: true,
            })
            .add_default_setting(DefaultSetting::MinimalDataCollection {
                optional_fields_opt_in: true,
                no_pre_ticked_boxes: true,
            })
            .with_state_of_art_considered(true)
            .with_costs_considered(true)
            .with_processing_context_considered(true)
            .with_risks_assessed(true);

        let validation = dpbd.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Data minimisation"))
        );
    }

    #[test]
    fn test_missing_default_settings() {
        let dpbd = DataProtectionByDesign::new()
            .with_system_name("System")
            .add_design_principle(DesignPrinciple::DataMinimisation {
                only_necessary_data: true,
                justification: "Essential only".to_string(),
            })
            .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
                security_measures_integrated: true,
                access_controls: true,
            })
            .with_state_of_art_considered(true)
            .with_costs_considered(true)
            .with_processing_context_considered(true)
            .with_risks_assessed(true);

        let validation = dpbd.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("default settings"))
        );
    }

    #[test]
    fn test_missing_article_25_considerations() {
        let dpbd = DataProtectionByDesign::new()
            .with_system_name("System")
            .add_design_principle(DesignPrinciple::DataMinimisation {
                only_necessary_data: true,
                justification: "Essential only".to_string(),
            })
            .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
                security_measures_integrated: true,
                access_controls: true,
            })
            .add_default_setting(DefaultSetting::MinimalDataCollection {
                optional_fields_opt_in: true,
                no_pre_ticked_boxes: true,
            });

        let validation = dpbd.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("State of the art"))
        );
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Implementation costs"))
        );
        assert!(validation.warnings.iter().any(|w| w.contains("context")));
        assert!(validation.warnings.iter().any(|w| w.contains("Risks")));
    }

    #[test]
    fn test_missing_security_integration() {
        let dpbd = DataProtectionByDesign::new()
            .with_system_name("System")
            .add_design_principle(DesignPrinciple::DataMinimisation {
                only_necessary_data: true,
                justification: "Essential only".to_string(),
            })
            .add_default_setting(DefaultSetting::MinimalDataCollection {
                optional_fields_opt_in: true,
                no_pre_ticked_boxes: true,
            })
            .with_state_of_art_considered(true)
            .with_costs_considered(true)
            .with_processing_context_considered(true)
            .with_risks_assessed(true);

        let validation = dpbd.validate().unwrap();

        assert!(!validation.compliant);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("Security measures"))
        );
    }

    #[test]
    fn test_design_principle_references() {
        let minimisation = DesignPrinciple::DataMinimisation {
            only_necessary_data: true,
            justification: "Test".to_string(),
        };

        assert_eq!(minimisation.article_reference(), "Article 5(1)(c)");
        assert!(minimisation.description().contains("minimisation"));

        let purpose = DesignPrinciple::PurposeLimitation {
            limited_to_purpose: true,
            documented: true,
        };

        assert_eq!(purpose.article_reference(), "Article 5(1)(b)");
    }

    #[test]
    fn test_privacy_technologies() {
        let dpbd = DataProtectionByDesign::new()
            .with_system_name("Advanced System")
            .add_design_principle(DesignPrinciple::DataMinimisation {
                only_necessary_data: true,
                justification: "Essential only".to_string(),
            })
            .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
                security_measures_integrated: true,
                access_controls: true,
            })
            .add_default_setting(DefaultSetting::MinimalDataCollection {
                optional_fields_opt_in: true,
                no_pre_ticked_boxes: true,
            })
            .add_privacy_technology(PrivacyEnhancingTechnology::Encryption {
                algorithm: "AES-256".to_string(),
            })
            .add_privacy_technology(PrivacyEnhancingTechnology::DifferentialPrivacy {
                epsilon: 0.1,
            })
            .add_privacy_technology(PrivacyEnhancingTechnology::HomomorphicEncryption {
                scheme: "BFV".to_string(),
            })
            .with_state_of_art_considered(true)
            .with_costs_considered(true)
            .with_processing_context_considered(true)
            .with_risks_assessed(true);

        let validation = dpbd.validate().unwrap();

        assert!(validation.compliant);
        assert_eq!(validation.privacy_technologies_count, 3);
    }
}
