//! GDPR Article 6 - Lawfulness of processing
//!
//! This module implements validation logic for the 6 lawful bases under Article 6(1).

use crate::gdpr::{
    error::GdprError,
    types::{ComplianceStatus, LawfulBasis, PersonalDataCategory, ProcessingOperation},
};
use legalis_core::{Condition, Effect, EffectType, LegalResult, Statute};
use uuid::Uuid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// Builder for validating lawful basis for data processing (Article 6)
///
/// ## Example
///
/// ```rust
/// use legalis_eu::gdpr::*;
///
/// let processing = DataProcessing::new()
///     .with_controller("Acme Corp")
///     .with_purpose("Customer relationship management")
///     .add_data_category(PersonalDataCategory::Regular("name".to_string()))
///     .add_data_category(PersonalDataCategory::Regular("email".to_string()))
///     .with_operation(ProcessingOperation::Collection)
///     .with_operation(ProcessingOperation::Storage)
///     .with_lawful_basis(LawfulBasis::Consent {
///         freely_given: true,
///         specific: true,
///         informed: true,
///         unambiguous: true,
///     });
///
/// let result = processing.validate();
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DataProcessing {
    /// Data controller
    pub controller: Option<String>,

    /// Data processor (if applicable)
    pub processor: Option<String>,

    /// Purpose of processing (must be specified - purpose limitation principle)
    pub purpose: Option<String>,

    /// Personal data categories being processed
    pub data_categories: Vec<PersonalDataCategory>,

    /// Processing operations
    pub operations: Vec<ProcessingOperation>,

    /// Lawful basis under Article 6
    pub lawful_basis: Option<LawfulBasis>,

    /// Data subjects affected
    pub data_subjects: Vec<String>,

    /// Retention period
    pub retention_period: Option<String>,

    /// Technical and organizational measures (Article 32)
    pub security_measures: Vec<String>,
}

impl DataProcessing {
    /// Create a new data processing activity
    pub fn new() -> Self {
        Self {
            controller: None,
            processor: None,
            purpose: None,
            data_categories: Vec::new(),
            operations: Vec::new(),
            lawful_basis: None,
            data_subjects: Vec::new(),
            retention_period: None,
            security_measures: Vec::new(),
        }
    }

    /// Set the data controller
    pub fn with_controller(mut self, controller: impl Into<String>) -> Self {
        self.controller = Some(controller.into());
        self
    }

    /// Set the data processor
    pub fn with_processor(mut self, processor: impl Into<String>) -> Self {
        self.processor = Some(processor.into());
        self
    }

    /// Set the purpose of processing
    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }

    /// Set data categories (replacing existing)
    pub fn with_data_categories(mut self, categories: Vec<PersonalDataCategory>) -> Self {
        self.data_categories = categories;
        self
    }

    /// Add a single data category
    pub fn add_data_category(mut self, category: PersonalDataCategory) -> Self {
        self.data_categories.push(category);
        self
    }

    /// Set operations (replacing existing)
    pub fn with_operations(mut self, operations: Vec<ProcessingOperation>) -> Self {
        self.operations = operations;
        self
    }

    /// Add a single operation
    pub fn with_operation(mut self, operation: ProcessingOperation) -> Self {
        self.operations.push(operation);
        self
    }

    /// Set the lawful basis
    pub fn with_lawful_basis(mut self, basis: LawfulBasis) -> Self {
        self.lawful_basis = Some(basis);
        self
    }

    /// Set security measures
    pub fn with_security_measures(mut self, measures: Vec<String>) -> Self {
        self.security_measures = measures;
        self
    }

    /// Validate the data processing activity
    pub fn validate(&self) -> Result<ProcessingValidation, GdprError> {
        // Check required fields
        if self.controller.is_none() {
            return Err(GdprError::missing_field("controller"));
        }

        if self.purpose.is_none() {
            return Err(GdprError::missing_field("purpose"));
        }

        if self.lawful_basis.is_none() {
            return Err(GdprError::MissingLawfulBasis);
        }

        if self.data_categories.is_empty() {
            return Err(GdprError::NoDataCategories);
        }

        // Validate lawful basis
        let lawful_basis_result = self.validate_lawful_basis()?;

        // Check for special categories
        let has_special_categories = self
            .data_categories
            .iter()
            .any(|cat| matches!(cat, PersonalDataCategory::Special(_)));

        if has_special_categories {
            // Special categories require Article 9 exception
            return Ok(ProcessingValidation {
                lawful_basis_valid: lawful_basis_result,
                requires_article9_exception: true,
                compliance_status: ComplianceStatus::RequiresAdditionalReview {
                    reason: "Processing special categories requires Article 9 exception"
                        .to_string(),
                },
            });
        }

        Ok(ProcessingValidation {
            lawful_basis_valid: lawful_basis_result,
            requires_article9_exception: false,
            compliance_status: ComplianceStatus::Compliant,
        })
    }

    /// Validate the specific lawful basis
    fn validate_lawful_basis(&self) -> Result<LegalResult<bool>, GdprError> {
        match &self.lawful_basis {
            Some(LawfulBasis::Consent {
                freely_given,
                specific,
                informed,
                unambiguous,
            }) => {
                // All criteria must be met for valid consent
                if !(*freely_given && *specific && *informed && *unambiguous) {
                    return Err(GdprError::invalid_consent(
                        "Consent must be freely given, specific, informed, and unambiguous",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Some(LawfulBasis::Contract {
                necessary_for_performance,
            }) => {
                // Contract basis requires necessity
                if !necessary_for_performance {
                    return Err(GdprError::invalid_lawful_basis(
                        "Processing must be necessary for contract performance",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Some(LawfulBasis::LegalObligation {
                eu_law,
                member_state_law,
            }) => {
                // Legal obligation requires specification of law
                if eu_law.is_none() && member_state_law.is_none() {
                    return Err(GdprError::invalid_lawful_basis(
                        "Must specify EU or Member State law creating the obligation",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Some(LawfulBasis::VitalInterests { life_threatening }) => {
                // Vital interests typically require life-threatening situation
                if !life_threatening {
                    return Ok(LegalResult::JudicialDiscretion {
                        issue: "Vital interests typically require life-threatening situation - \
                                verify necessity"
                            .to_string(),
                        context_id: Uuid::new_v4(),
                        narrative_hint: Some(
                            "Article 6(1)(d) requires necessity to protect vital interests. \
                             This is a narrow exception typically limited to emergency situations."
                                .to_string(),
                        ),
                    });
                }
                Ok(LegalResult::Deterministic(true))
            }

            Some(LawfulBasis::PublicTask { task_basis }) => {
                // Public task requires legal basis
                if task_basis.is_empty() {
                    return Err(GdprError::invalid_lawful_basis(
                        "Public task must be laid down by law",
                    ));
                }
                Ok(LegalResult::Deterministic(true))
            }

            Some(LawfulBasis::LegitimateInterests {
                controller_interest,
                balancing_test_passed,
            }) => {
                // Legitimate interests requires balancing test
                if !balancing_test_passed {
                    return Ok(LegalResult::JudicialDiscretion {
                        issue: format!(
                            "Balancing test required: Controller's interest '{}' vs \
                             data subject's rights and freedoms",
                            controller_interest
                        ),
                        context_id: Uuid::new_v4(),
                        narrative_hint: Some(
                            "Apply Recital 47 guidance: \
                             (1) Is processing necessary for the legitimate interest? \
                             (2) Are there less intrusive alternatives? \
                             (3) What are the reasonable expectations of the data subject? \
                             (4) What is the nature of the personal data? \
                             See WP29 Opinion 06/2014 on legitimate interests."
                                .to_string(),
                        ),
                    });
                }
                Ok(LegalResult::Deterministic(true))
            }

            None => Err(GdprError::MissingLawfulBasis),
        }
    }

    /// Convert to legalis-core Statute for integration
    pub fn as_statute(&self) -> Result<Statute, GdprError> {
        let purpose = self
            .purpose
            .as_ref()
            .ok_or_else(|| GdprError::missing_field("purpose"))?;

        let effect = Effect::new(
            EffectType::Obligation,
            format!("Process personal data for purpose: {}", purpose),
        );

        let mut statute = Statute::new(
            "gdpr-article-6",
            "GDPR Article 6 - Lawfulness of processing | CELEX:32016R0679",
            effect,
        )
        .with_jurisdiction("EU")
        .with_version(1);

        // Add conditions based on lawful basis
        if let Some(basis) = &self.lawful_basis {
            statute = match basis {
                LawfulBasis::Consent { .. } => {
                    statute.with_precondition(Condition::AttributeEquals {
                        key: "consent_valid".to_string(),
                        value: "true".to_string(),
                    })
                }
                LawfulBasis::Contract { .. } => {
                    statute.with_precondition(Condition::AttributeEquals {
                        key: "contract_exists".to_string(),
                        value: "true".to_string(),
                    })
                }
                _ => statute,
            };
        }

        Ok(statute)
    }
}

impl Default for DataProcessing {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of processing validation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProcessingValidation {
    #[cfg_attr(feature = "schema", schemars(skip))]
    pub lawful_basis_valid: LegalResult<bool>,
    pub requires_article9_exception: bool,
    pub compliance_status: ComplianceStatus,
}

impl ProcessingValidation {
    /// Check if processing is compliant
    pub fn is_compliant(&self) -> bool {
        self.compliance_status.is_compliant()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gdpr::types::SpecialCategory;

    #[test]
    fn test_consent_all_criteria_met() {
        let processing = DataProcessing::new()
            .with_controller("Acme Corp")
            .with_purpose("Marketing emails")
            .add_data_category(PersonalDataCategory::Regular("email".to_string()))
            .with_lawful_basis(LawfulBasis::Consent {
                freely_given: true,
                specific: true,
                informed: true,
                unambiguous: true,
            });

        let result = processing.validate();
        assert!(result.is_ok());
        assert!(result.unwrap().is_compliant());
    }

    #[test]
    fn test_consent_missing_criteria() {
        let processing = DataProcessing::new()
            .with_controller("Acme Corp")
            .with_purpose("Marketing")
            .add_data_category(PersonalDataCategory::Regular("email".to_string()))
            .with_lawful_basis(LawfulBasis::Consent {
                freely_given: false, // Coerced consent
                specific: true,
                informed: true,
                unambiguous: true,
            });

        let result = processing.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_controller() {
        let processing = DataProcessing::new()
            .with_purpose("Marketing")
            .with_lawful_basis(LawfulBasis::Consent {
                freely_given: true,
                specific: true,
                informed: true,
                unambiguous: true,
            });

        let result = processing.validate();
        assert!(matches!(result, Err(GdprError::MissingField(_))));
    }

    #[test]
    fn test_special_categories_requires_article9() {
        let processing = DataProcessing::new()
            .with_controller("Hospital")
            .with_purpose("Medical treatment")
            .add_data_category(PersonalDataCategory::Special(SpecialCategory::HealthData))
            .with_lawful_basis(LawfulBasis::Consent {
                freely_given: true,
                specific: true,
                informed: true,
                unambiguous: true,
            });

        let result = processing.validate().unwrap();
        assert!(result.requires_article9_exception);
    }

    #[test]
    fn test_legitimate_interests_requires_balancing() {
        let processing = DataProcessing::new()
            .with_controller("Security Co")
            .with_purpose("Fraud prevention")
            .add_data_category(PersonalDataCategory::Regular("IP address".to_string()))
            .with_lawful_basis(LawfulBasis::LegitimateInterests {
                controller_interest: "Fraud prevention".to_string(),
                balancing_test_passed: false, // Not yet assessed
            });

        let result = processing.validate().unwrap();

        // Should return JudicialDiscretion
        match result.lawful_basis_valid {
            LegalResult::JudicialDiscretion { .. } => {}
            _ => panic!("Expected judicial discretion"),
        }
    }
}
