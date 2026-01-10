//! GDPR Article 9 - Processing of Special Categories of Personal Data
//!
//! Article 9(1) prohibits processing of special categories unless an exception under Article 9(2) applies.

use crate::gdpr::error::GdprError;
use crate::gdpr::types::SpecialCategory;
use legalis_core::{Condition, Effect, EffectType, LegalResult, Statute};
use uuid::Uuid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Article 9(2) exceptions allowing processing of special categories
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Article9Exception {
    /// Article 9(2)(a) - Explicit consent
    ///
    /// Data subject gave explicit consent for processing for specific purposes.
    /// Higher bar than Article 6(1)(a) consent - must be explicit.
    ExplicitConsent {
        purposes: Vec<String>,
        consent_documented: bool,
    },

    /// Article 9(2)(b) - Employment and social security law
    ///
    /// Necessary for carrying out obligations/exercising rights in employment,
    /// social security, and social protection law.
    EmploymentSocialSecurityLaw {
        legal_basis: String,
        authorized_by_union_or_member_state_law: bool,
    },

    /// Article 9(2)(c) - Vital interests (data subject unable to consent)
    ///
    /// Necessary to protect vital interests when data subject is physically/legally
    /// incapable of giving consent.
    VitalInterestsUnableToConsent {
        life_threatening: bool,
        unable_to_consent: bool,
    },

    /// Article 9(2)(d) - Legitimate activities of foundations/associations
    ///
    /// Processing by foundation, association, or non-profit body with political,
    /// philosophical, religious, or trade union aim, limited to members/former members.
    LegitimateActivitiesFoundation {
        organization_type: String,
        limited_to_members: bool,
        no_disclosure_without_consent: bool,
    },

    /// Article 9(2)(e) - Data manifestly made public by data subject
    ///
    /// Data subject clearly made data public (social media, public campaign, etc.).
    DataManifestlyMadePublic { public_source: String },

    /// Article 9(2)(f) - Legal claims or judicial acts
    ///
    /// Necessary for establishment, exercise, or defense of legal claims,
    /// or courts acting in judicial capacity.
    LegalClaims { claim_description: String },

    /// Article 9(2)(g) - Substantial public interest
    ///
    /// Necessary for reasons of substantial public interest, based on Union/Member State law,
    /// proportionate to aim, respects essence of right to data protection.
    SubstantialPublicInterest {
        legal_basis: String,
        proportionate: bool,
        safeguards_in_place: bool,
    },

    /// Article 9(2)(h) - Healthcare, medical diagnosis, social care
    ///
    /// Necessary for preventive/occupational medicine, medical diagnosis,
    /// health/social care, management of health/social care systems.
    Healthcare {
        purpose: HealthcarePurpose,
        professional_secrecy: bool,
    },

    /// Article 9(2)(i) - Public health
    ///
    /// Necessary for reasons of public interest in public health (serious cross-border threats,
    /// quality/safety of healthcare, medicinal products).
    PublicHealth {
        public_health_purpose: String,
        authorized_by_union_or_member_state_law: bool,
        professional_secrecy: bool,
    },

    /// Article 9(2)(j) - Archiving, research, statistics
    ///
    /// Necessary for archiving in public interest, scientific/historical research,
    /// or statistical purposes, based on Union/Member State law.
    ArchivingResearchStatistics {
        purpose: ResearchPurpose,
        legal_basis: String,
        technical_organizational_measures: Vec<String>,
    },
}

/// Healthcare purposes under Article 9(2)(h)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HealthcarePurpose {
    PreventiveMedicine,
    OccupationalMedicine,
    MedicalDiagnosis,
    ProvisionOfHealthCare,
    ProvisionOfSocialCare,
    ManagementOfHealthSystems,
    ManagementOfSocialCareSystems,
}

/// Research purposes under Article 9(2)(j)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResearchPurpose {
    ArchivingInPublicInterest,
    ScientificResearch,
    HistoricalResearch,
    StatisticalPurposes,
}

/// Builder for Article 9 special category processing
#[derive(Debug, Clone)]
pub struct Article9Processing {
    pub controller: Option<String>,
    pub purpose: Option<String>,
    pub special_categories: Vec<SpecialCategory>,
    pub exception: Option<Article9Exception>,
}

impl Article9Processing {
    pub fn new() -> Self {
        Self {
            controller: None,
            purpose: None,
            special_categories: Vec::new(),
            exception: None,
        }
    }

    pub fn with_controller(mut self, controller: impl Into<String>) -> Self {
        self.controller = Some(controller.into());
        self
    }

    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }

    pub fn add_special_category(mut self, category: SpecialCategory) -> Self {
        self.special_categories.push(category);
        self
    }

    pub fn with_exception(mut self, exception: Article9Exception) -> Self {
        self.exception = Some(exception);
        self
    }

    /// Validate Article 9 compliance
    pub fn validate(&self) -> Result<Article9Validation, GdprError> {
        // Check required fields
        if self.controller.is_none() {
            return Err(GdprError::missing_field("controller"));
        }
        if self.purpose.is_none() {
            return Err(GdprError::missing_field("purpose"));
        }
        if self.special_categories.is_empty() {
            return Err(GdprError::MissingField(
                "special_categories (no special categories to process)".to_string(),
            ));
        }

        // Article 9(1) prohibition applies - exception required
        let exception = self
            .exception
            .as_ref()
            .ok_or(GdprError::SpecialCategoryWithoutException)?;

        // Validate exception
        let exception_valid = self.validate_exception(exception)?;

        let compliance_status = if matches!(exception_valid, LegalResult::Deterministic(true)) {
            crate::gdpr::types::ComplianceStatus::Compliant
        } else {
            crate::gdpr::types::ComplianceStatus::RequiresAdditionalReview {
                reason: "Article 9(2) exception requires human review".to_string(),
            }
        };

        Ok(Article9Validation {
            special_categories: self.special_categories.clone(),
            exception: exception.clone(),
            exception_valid,
            compliance_status,
        })
    }

    /// Validate specific Article 9(2) exception
    fn validate_exception(
        &self,
        exception: &Article9Exception,
    ) -> Result<LegalResult<bool>, GdprError> {
        match exception {
            Article9Exception::ExplicitConsent {
                purposes,
                consent_documented,
            } => {
                if !consent_documented {
                    return Err(GdprError::invalid_consent(
                        "Article 9(2)(a) requires documented explicit consent",
                    ));
                }
                if purposes.is_empty() {
                    return Err(GdprError::MissingField(
                        "purposes (explicit consent requires specific purposes)".to_string(),
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Article9Exception::EmploymentSocialSecurityLaw {
                authorized_by_union_or_member_state_law,
                ..
            } => {
                if !authorized_by_union_or_member_state_law {
                    return Ok(LegalResult::JudicialDiscretion {
                        issue: "Article 9(2)(b) requires authorization by Union or Member State law with safeguards".to_string(),
                        context_id: Uuid::new_v4(),
                        narrative_hint: Some("Check if processing is authorized by specific law and provides appropriate safeguards for fundamental rights".to_string()),
                    });
                }
                Ok(LegalResult::Deterministic(true))
            }

            Article9Exception::VitalInterestsUnableToConsent {
                life_threatening,
                unable_to_consent,
            } => {
                if !life_threatening {
                    return Err(GdprError::invalid_value(
                        "life_threatening",
                        "Article 9(2)(c) requires life-threatening situation for vital interests",
                    ));
                }
                if !unable_to_consent {
                    return Err(GdprError::invalid_value(
                        "unable_to_consent",
                        "Article 9(2)(c) only applies when data subject is unable to consent",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Article9Exception::LegitimateActivitiesFoundation {
                limited_to_members,
                no_disclosure_without_consent,
                ..
            } => {
                if !limited_to_members {
                    return Err(GdprError::invalid_value(
                        "limited_to_members",
                        "Article 9(2)(d) processing must be limited to members/former members",
                    ));
                }
                if !no_disclosure_without_consent {
                    return Err(GdprError::invalid_value(
                        "no_disclosure_without_consent",
                        "Article 9(2)(d) prohibits disclosure outside organization without consent",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Article9Exception::DataManifestlyMadePublic { .. } => {
                // Requires judgment on whether data is truly "manifestly" public
                Ok(LegalResult::JudicialDiscretion {
                    issue: "Determine if data subject manifestly made data public".to_string(),
                    context_id: Uuid::new_v4(),
                    narrative_hint: Some(
                        "Data must be clearly and deliberately made public by data subject (e.g., public campaign, social media). Not sufficient if merely accessible.".to_string(),
                    ),
                })
            }

            Article9Exception::LegalClaims { .. } => Ok(LegalResult::Deterministic(true)),

            Article9Exception::SubstantialPublicInterest {
                proportionate,
                safeguards_in_place,
                ..
            } => {
                if !proportionate {
                    return Ok(LegalResult::JudicialDiscretion {
                        issue: "Assess proportionality of processing to substantial public interest aim".to_string(),
                        context_id: Uuid::new_v4(),
                        narrative_hint: Some("Processing must be proportionate to the aim pursued and respect essence of right to data protection".to_string()),
                    });
                }
                if !safeguards_in_place {
                    return Err(GdprError::invalid_value(
                        "safeguards_in_place",
                        "Article 9(2)(g) requires appropriate safeguards for data protection",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Article9Exception::Healthcare {
                professional_secrecy,
                ..
            } => {
                if !professional_secrecy {
                    return Err(GdprError::invalid_value(
                        "professional_secrecy",
                        "Article 9(2)(h) requires processing by/under professional secrecy obligation",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Article9Exception::PublicHealth {
                authorized_by_union_or_member_state_law,
                professional_secrecy,
                ..
            } => {
                if !authorized_by_union_or_member_state_law {
                    return Err(GdprError::invalid_value(
                        "authorized_by_union_or_member_state_law",
                        "Article 9(2)(i) requires authorization by Union or Member State law",
                    ));
                }
                if !professional_secrecy {
                    return Err(GdprError::invalid_value(
                        "professional_secrecy",
                        "Article 9(2)(i) requires processing by/under professional secrecy obligation",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Article9Exception::ArchivingResearchStatistics {
                technical_organizational_measures,
                ..
            } => {
                if technical_organizational_measures.is_empty() {
                    return Err(GdprError::invalid_value(
                        "technical_organizational_measures",
                        "Article 9(2)(j) requires appropriate technical and organizational measures (e.g., pseudonymization, minimization)",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }
        }
    }

    /// Convert to legalis-core Statute
    pub fn as_statute(&self) -> Result<Statute, GdprError> {
        let exception_type = self
            .exception
            .as_ref()
            .ok_or_else(|| GdprError::MissingField("exception".to_string()))?;

        let (exception_letter, description) = match exception_type {
            Article9Exception::ExplicitConsent { .. } => ("a", "Explicit consent"),
            Article9Exception::EmploymentSocialSecurityLaw { .. } => {
                ("b", "Employment/social security law")
            }
            Article9Exception::VitalInterestsUnableToConsent { .. } => {
                ("c", "Vital interests (unable to consent)")
            }
            Article9Exception::LegitimateActivitiesFoundation { .. } => {
                ("d", "Legitimate activities of foundations")
            }
            Article9Exception::DataManifestlyMadePublic { .. } => {
                ("e", "Data manifestly made public")
            }
            Article9Exception::LegalClaims { .. } => ("f", "Legal claims"),
            Article9Exception::SubstantialPublicInterest { .. } => {
                ("g", "Substantial public interest")
            }
            Article9Exception::Healthcare { .. } => ("h", "Healthcare/medical diagnosis"),
            Article9Exception::PublicHealth { .. } => ("i", "Public health"),
            Article9Exception::ArchivingResearchStatistics { .. } => {
                ("j", "Archiving/research/statistics")
            }
        };

        Ok(Statute::new(
            format!("gdpr-art9-2-{}", exception_letter),
            format!(
                "GDPR Article 9(2)({}) - {} | CELEX:32016R0679",
                exception_letter, description
            ),
            Effect::new(
                EffectType::Grant,
                format!(
                    "Lawful processing of special categories under Article 9(2)({})",
                    exception_letter
                ),
            ),
        )
        .with_jurisdiction("EU")
        .with_precondition(Condition::Custom {
            description: format!(
                "Article 9(2)({}) exception requirements met",
                exception_letter
            ),
        }))
    }
}

impl Default for Article9Processing {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result for Article 9 processing
#[derive(Debug, Clone)]
pub struct Article9Validation {
    pub special_categories: Vec<SpecialCategory>,
    pub exception: Article9Exception,
    pub exception_valid: LegalResult<bool>,
    pub compliance_status: crate::gdpr::types::ComplianceStatus,
}

impl Article9Validation {
    pub fn is_compliant(&self) -> bool {
        self.compliance_status.is_compliant()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explicit_consent_exception() {
        let processing = Article9Processing::new()
            .with_controller("Hospital")
            .with_purpose("Medical research")
            .add_special_category(SpecialCategory::HealthData)
            .with_exception(Article9Exception::ExplicitConsent {
                purposes: vec!["Cancer research study".to_string()],
                consent_documented: true,
            });

        let result = processing.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(matches!(
            validation.exception_valid,
            LegalResult::Deterministic(true)
        ));
    }

    #[test]
    fn test_healthcare_exception() {
        let processing = Article9Processing::new()
            .with_controller("Medical Practice")
            .with_purpose("Patient diagnosis")
            .add_special_category(SpecialCategory::HealthData)
            .with_exception(Article9Exception::Healthcare {
                purpose: HealthcarePurpose::MedicalDiagnosis,
                professional_secrecy: true,
            });

        let result = processing.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_vital_interests_exception() {
        let processing = Article9Processing::new()
            .with_controller("Emergency Services")
            .with_purpose("Life-saving treatment")
            .add_special_category(SpecialCategory::HealthData)
            .with_exception(Article9Exception::VitalInterestsUnableToConsent {
                life_threatening: true,
                unable_to_consent: true,
            });

        let result = processing.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_missing_exception() {
        let processing = Article9Processing::new()
            .with_controller("Company")
            .with_purpose("Background checks")
            .add_special_category(SpecialCategory::BiometricData);

        let result = processing.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GdprError::SpecialCategoryWithoutException
        ));
    }

    #[test]
    fn test_invalid_explicit_consent_no_purposes() {
        let processing = Article9Processing::new()
            .with_controller("Research Org")
            .with_purpose("Study")
            .add_special_category(SpecialCategory::GeneticData)
            .with_exception(Article9Exception::ExplicitConsent {
                purposes: vec![], // No purposes specified
                consent_documented: true,
            });

        let result = processing.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_research_exception_with_measures() {
        let processing = Article9Processing::new()
            .with_controller("University")
            .with_purpose("Historical research")
            .add_special_category(SpecialCategory::RacialEthnicOrigin)
            .with_exception(Article9Exception::ArchivingResearchStatistics {
                purpose: ResearchPurpose::HistoricalResearch,
                legal_basis: "EU Research Regulation".to_string(),
                technical_organizational_measures: vec![
                    "Pseudonymization".to_string(),
                    "Data minimization".to_string(),
                ],
            });

        let result = processing.validate();
        assert!(result.is_ok());
    }
}
