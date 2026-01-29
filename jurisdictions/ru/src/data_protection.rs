//! Personal Data Protection (Federal Law 152-FZ).
//!
//! Federal Law No. 152-FZ of July 27, 2006
//! "On Personal Data" (О персональных данных)
//!
//! This module provides:
//! - Personal data operator requirements
//! - Data subject rights
//! - Processing rules
//! - Security measures

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors related to data protection operations
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum DataProtectionError {
    /// Invalid data processing
    #[error("Invalid data processing: {0}")]
    InvalidProcessing(String),

    /// Consent violation
    #[error("Consent violation: {0}")]
    ConsentViolation(String),

    /// Security violation
    #[error("Security violation: {0}")]
    SecurityViolation(String),

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Personal data operator representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDataOperator {
    /// Operator name
    pub name: String,
    /// INN (tax ID)
    pub inn: String,
    /// Legal address
    pub legal_address: String,
    /// Processing purposes
    pub processing_purposes: Vec<ProcessingPurpose>,
    /// Categories of personal data
    pub data_categories: Vec<DataCategory>,
    /// Security measures implemented
    pub security_measures: Vec<SecurityMeasure>,
    /// Is registered with Roskomnadzor
    pub registered_with_roskomnadzor: bool,
}

impl PersonalDataOperator {
    /// Creates a new personal data operator
    pub fn new(
        name: impl Into<String>,
        inn: impl Into<String>,
        legal_address: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            inn: inn.into(),
            legal_address: legal_address.into(),
            processing_purposes: Vec::new(),
            data_categories: Vec::new(),
            security_measures: Vec::new(),
            registered_with_roskomnadzor: false,
        }
    }

    /// Adds a processing purpose
    pub fn add_purpose(mut self, purpose: ProcessingPurpose) -> Self {
        self.processing_purposes.push(purpose);
        self
    }

    /// Adds a data category
    pub fn add_data_category(mut self, category: DataCategory) -> Self {
        self.data_categories.push(category);
        self
    }

    /// Adds a security measure
    pub fn add_security_measure(mut self, measure: SecurityMeasure) -> Self {
        self.security_measures.push(measure);
        self
    }

    /// Sets registration status
    pub fn registered(mut self) -> Self {
        self.registered_with_roskomnadzor = true;
        self
    }

    /// Validates the operator
    pub fn validate(&self) -> Result<(), DataProtectionError> {
        // Must have at least one processing purpose
        if self.processing_purposes.is_empty() {
            return Err(DataProtectionError::ValidationFailed(
                "Operator must specify processing purposes".to_string(),
            ));
        }

        // Must have at least one data category
        if self.data_categories.is_empty() {
            return Err(DataProtectionError::ValidationFailed(
                "Operator must specify data categories".to_string(),
            ));
        }

        // Must have security measures
        if self.security_measures.is_empty() {
            return Err(DataProtectionError::SecurityViolation(
                "Operator must implement security measures".to_string(),
            ));
        }

        // Should be registered with Roskomnadzor
        if !self.registered_with_roskomnadzor {
            return Err(DataProtectionError::ValidationFailed(
                "Operator should be registered with Roskomnadzor".to_string(),
            ));
        }

        Ok(())
    }
}

/// Processing purposes (Article 3)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingPurpose {
    /// Employment relationship
    Employment,
    /// Contract execution
    ContractExecution,
    /// Legal obligation
    LegalObligation,
    /// Legitimate interest
    LegitimateInterest,
    /// Consent of data subject
    ConsentBased,
    /// Public interest
    PublicInterest,
}

/// Categories of personal data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataCategory {
    /// General data (name, address, phone)
    GeneralData,
    /// Biometric data
    BiometricData,
    /// Special categories (health, religion, etc.)
    SpecialCategories,
    /// Data of minors
    MinorData,
}

/// Security measures (Article 19)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMeasure {
    /// Type of measure
    pub measure_type: SecurityMeasureType,
    /// Description
    pub description: String,
    /// Is implemented
    pub implemented: bool,
}

/// Types of security measures
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityMeasureType {
    /// Organizational measures
    Organizational,
    /// Technical measures (encryption, access control)
    Technical,
    /// Physical measures
    Physical,
}

impl SecurityMeasure {
    /// Creates a new security measure
    pub fn new(measure_type: SecurityMeasureType, description: impl Into<String>) -> Self {
        Self {
            measure_type,
            description: description.into(),
            implemented: false,
        }
    }

    /// Marks as implemented
    pub fn implemented(mut self) -> Self {
        self.implemented = true;
        self
    }
}

/// Data subject representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubject {
    /// Name
    pub name: String,
    /// Identification (passport, etc.)
    pub identification: String,
    /// Has given consent
    pub consent_given: bool,
    /// Consent type
    pub consent_type: Option<ConsentType>,
}

/// Types of consent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentType {
    /// Written consent
    Written,
    /// Electronic consent
    Electronic,
    /// Other form of consent
    Other,
}

/// Data subject rights (Article 14)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubjectRights {
    /// Right to access
    pub access: bool,
    /// Right to rectification
    pub rectification: bool,
    /// Right to erasure
    pub erasure: bool,
    /// Right to object
    pub object: bool,
    /// Right to withdraw consent
    pub withdraw_consent: bool,
}

impl DataSubjectRights {
    /// Creates full rights
    pub fn full_rights() -> Self {
        Self {
            access: true,
            rectification: true,
            erasure: true,
            object: true,
            withdraw_consent: true,
        }
    }
}

/// Third party data transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirdPartyTransfer {
    /// Third party name
    pub third_party_name: String,
    /// Purpose of transfer
    pub purpose: String,
    /// Is cross-border transfer
    pub cross_border: bool,
    /// Target country (if cross-border)
    pub target_country: Option<String>,
    /// Has data subject consent
    pub has_consent: bool,
}

impl ThirdPartyTransfer {
    /// Validates the transfer
    pub fn validate(&self) -> Result<(), DataProtectionError> {
        // Cross-border transfers require special approval (Article 12)
        if self.cross_border {
            if self.target_country.is_none() {
                return Err(DataProtectionError::InvalidProcessing(
                    "Cross-border transfer requires target country specification".to_string(),
                ));
            }

            // Check if country has adequate protection level
            // (Simplified - in reality, check against approved countries list)
            if !self.has_consent {
                return Err(DataProtectionError::ConsentViolation(
                    "Cross-border transfer requires data subject consent".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// Quick validation for data processing
pub fn quick_validate_data_processing(
    operator: &PersonalDataOperator,
) -> Result<(), DataProtectionError> {
    operator.validate()
}

/// Article 9: Consent requirements
pub fn validate_consent(
    consent_given: bool,
    consent_type: Option<ConsentType>,
    processing_special_data: bool,
) -> Result<(), DataProtectionError> {
    if processing_special_data && !consent_given {
        return Err(DataProtectionError::ConsentViolation(
            "Processing special categories requires explicit consent".to_string(),
        ));
    }

    if consent_given && consent_type.is_none() {
        return Err(DataProtectionError::ConsentViolation(
            "Consent must specify type (written, electronic, etc.)".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_creation() {
        let operator = PersonalDataOperator::new("ООО Компания", "1234567890", "Москва")
            .add_purpose(ProcessingPurpose::Employment)
            .add_data_category(DataCategory::GeneralData)
            .add_security_measure(
                SecurityMeasure::new(SecurityMeasureType::Technical, "Encryption").implemented(),
            )
            .registered();

        assert!(operator.validate().is_ok());
    }

    #[test]
    fn test_operator_validation_fails() {
        let operator = PersonalDataOperator::new("ООО Компания", "1234567890", "Москва");

        assert!(operator.validate().is_err());
    }

    #[test]
    fn test_third_party_transfer() {
        let transfer = ThirdPartyTransfer {
            third_party_name: "Partner Company".to_string(),
            purpose: "Data processing".to_string(),
            cross_border: true,
            target_country: Some("Germany".to_string()),
            has_consent: true,
        };

        assert!(transfer.validate().is_ok());

        let invalid_transfer = ThirdPartyTransfer {
            third_party_name: "Partner Company".to_string(),
            purpose: "Data processing".to_string(),
            cross_border: true,
            target_country: None,
            has_consent: false,
        };

        assert!(invalid_transfer.validate().is_err());
    }

    #[test]
    fn test_consent_validation() {
        // Valid: Processing special data with consent
        assert!(validate_consent(true, Some(ConsentType::Written), true).is_ok());

        // Invalid: Processing special data without consent
        assert!(validate_consent(false, None, true).is_err());

        // Invalid: Consent given but no type specified
        assert!(validate_consent(true, None, false).is_err());
    }

    #[test]
    fn test_data_subject_rights() {
        let rights = DataSubjectRights::full_rights();
        assert!(rights.access);
        assert!(rights.rectification);
        assert!(rights.erasure);
        assert!(rights.object);
        assert!(rights.withdraw_consent);
    }
}
