//! Personal Data Protection Act (PDPA) 2010
//!
//! Malaysian data protection law governing processing of personal data.
//!
//! # Key Provisions
//!
//! - **Section 6**: General principle - Personal data shall not be processed unless consent is given
//! - **Section 40**: Notice and choice principle
//! - **Section 41**: Disclosure principle
//! - **Section 42**: Security principle
//! - **Section 43**: Retention principle
//! - **Section 44**: Data integrity principle
//! - **Section 45**: Access principle
//!
//! # Data Protection Principles
//!
//! 1. General (Consent)
//! 2. Notice and Choice
//! 3. Disclosure
//! 4. Security
//! 5. Retention
//! 6. Data Integrity
//! 7. Access

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// PDPA error types.
#[derive(Debug, Error)]
pub enum PdpaError {
    /// Missing consent.
    #[error("Missing consent for data processing: {purpose}")]
    MissingConsent { purpose: String },

    /// Invalid consent.
    #[error("Invalid consent: {reason}")]
    InvalidConsent { reason: String },

    /// Data breach.
    #[error("Data breach: {description}")]
    DataBreach { description: String },

    /// Non-compliance with principles.
    #[error("Non-compliance with PDPA principle: {principle}")]
    PrincipleViolation { principle: String },
}

/// Result type for PDPA operations.
pub type Result<T> = std::result::Result<T, PdpaError>;

/// Purpose of data collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PurposeOfCollection {
    /// Marketing and promotional purposes.
    Marketing,
    /// Service provision.
    ServiceProvision,
    /// Employment purposes.
    Employment,
    /// Legal compliance.
    LegalCompliance,
    /// Research and statistics.
    Research,
    /// Other purposes.
    Other,
}

/// Consent method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentMethod {
    /// Written consent (signed document).
    Written,
    /// Electronic consent (checkbox, click-through).
    Electronic,
    /// Verbal consent.
    Verbal,
    /// Implied consent (opt-out available).
    Implied,
}

/// Personal data category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonalDataCategory {
    /// Name.
    Name,
    /// IC number.
    IcNumber,
    /// Contact information (phone, email, address).
    ContactInfo,
    /// Financial information.
    FinancialInfo,
    /// Employment information.
    EmploymentInfo,
    /// Health information.
    HealthInfo,
    /// Sensitive personal data.
    SensitiveData,
}

/// Consent record for PDPA compliance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Record ID.
    pub id: Uuid,
    /// Data subject identifier.
    pub data_subject_id: String,
    /// Purpose of collection.
    pub purpose: PurposeOfCollection,
    /// Consent method.
    pub consent_method: ConsentMethod,
    /// Data categories being collected.
    pub data_categories: Vec<PersonalDataCategory>,
    /// Timestamp of consent.
    pub timestamp: DateTime<Utc>,
    /// Whether consent is still valid.
    pub valid: bool,
    /// Whether data subject was given notice.
    pub notice_given: bool,
}

impl ConsentRecord {
    /// Creates a consent record builder.
    #[must_use]
    pub fn builder() -> ConsentRecordBuilder {
        ConsentRecordBuilder::default()
    }

    /// Validates the consent record.
    pub fn validate(&self) -> Result<()> {
        validate_consent(self)
    }
}

/// Consent record builder.
#[derive(Debug, Clone, Default)]
pub struct ConsentRecordBuilder {
    data_subject_id: Option<String>,
    purpose: Option<PurposeOfCollection>,
    consent_method: Option<ConsentMethod>,
    data_categories: Vec<PersonalDataCategory>,
    notice_given: bool,
}

impl ConsentRecordBuilder {
    /// Sets the data subject ID.
    #[must_use]
    pub fn data_subject_id(mut self, id: impl Into<String>) -> Self {
        self.data_subject_id = Some(id.into());
        self
    }

    /// Sets the purpose of collection.
    #[must_use]
    pub fn purpose(mut self, purpose: PurposeOfCollection) -> Self {
        self.purpose = Some(purpose);
        self
    }

    /// Sets the consent method.
    #[must_use]
    pub fn consent_method(mut self, method: ConsentMethod) -> Self {
        self.consent_method = Some(method);
        self
    }

    /// Adds a data category.
    #[must_use]
    pub fn add_data_category(mut self, category: PersonalDataCategory) -> Self {
        self.data_categories.push(category);
        self
    }

    /// Sets whether notice was given.
    #[must_use]
    pub fn notice_given(mut self, given: bool) -> Self {
        self.notice_given = given;
        self
    }

    /// Builds the consent record.
    pub fn build(self) -> Result<ConsentRecord> {
        Ok(ConsentRecord {
            id: Uuid::new_v4(),
            data_subject_id: self
                .data_subject_id
                .ok_or_else(|| PdpaError::InvalidConsent {
                    reason: "Data subject ID not specified".to_string(),
                })?,
            purpose: self.purpose.ok_or_else(|| PdpaError::InvalidConsent {
                reason: "Purpose not specified".to_string(),
            })?,
            consent_method: self
                .consent_method
                .ok_or_else(|| PdpaError::InvalidConsent {
                    reason: "Consent method not specified".to_string(),
                })?,
            data_categories: self.data_categories,
            timestamp: Utc::now(),
            valid: true,
            notice_given: self.notice_given,
        })
    }
}

/// Data breach notification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataBreachNotification {
    /// Breach ID.
    pub id: Uuid,
    /// Date of breach discovery.
    pub discovery_date: DateTime<Utc>,
    /// Description of breach.
    pub description: String,
    /// Number of data subjects affected.
    pub affected_count: u64,
    /// Data categories affected.
    pub affected_categories: Vec<PersonalDataCategory>,
    /// Whether breach has been notified to PDPD.
    pub notified_to_authority: bool,
    /// Whether data subjects have been notified.
    pub data_subjects_notified: bool,
}

/// PDPA organisation (data user).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdpaOrganisation {
    /// Organisation ID.
    pub id: Uuid,
    /// Organisation name.
    pub name: String,
    /// Registration number.
    pub registration_number: String,
    /// Whether organisation has appointed a data protection officer.
    pub has_dpo: bool,
    /// Data protection policy in place.
    pub has_policy: bool,
}

impl PdpaOrganisation {
    /// Creates a new PDPA organisation.
    #[must_use]
    pub fn new(name: impl Into<String>, registration_number: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            registration_number: registration_number.into(),
            has_dpo: false,
            has_policy: false,
        }
    }

    /// Sets whether DPO is appointed.
    #[must_use]
    pub fn with_dpo(mut self, has_dpo: bool) -> Self {
        self.has_dpo = has_dpo;
        self
    }

    /// Sets whether policy is in place.
    #[must_use]
    pub fn with_policy(mut self, has_policy: bool) -> Self {
        self.has_policy = has_policy;
        self
    }
}

/// Validates consent under PDPA Section 6.
pub fn validate_consent(consent: &ConsentRecord) -> Result<()> {
    // Check if consent is still valid
    if !consent.valid {
        return Err(PdpaError::InvalidConsent {
            reason: "Consent has been withdrawn".to_string(),
        });
    }

    // Check if notice was given (Section 40)
    if !consent.notice_given {
        return Err(PdpaError::PrincipleViolation {
            principle: "Notice and Choice - Data subject must be given notice".to_string(),
        });
    }

    // Check if data categories are specified
    if consent.data_categories.is_empty() {
        return Err(PdpaError::InvalidConsent {
            reason: "No data categories specified in consent".to_string(),
        });
    }

    Ok(())
}

/// Validation report for PDPA compliance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Whether organisation is compliant.
    pub compliant: bool,
    /// Issues found.
    pub issues: Vec<String>,
    /// Recommendations.
    pub recommendations: Vec<String>,
}

/// Validates PDPA compliance for an organisation.
#[must_use]
pub fn validate_organisation_compliance(org: &PdpaOrganisation) -> ComplianceReport {
    let mut issues = Vec::new();
    let mut recommendations = Vec::new();

    if !org.has_dpo {
        recommendations.push("Consider appointing a Data Protection Officer (DPO)".to_string());
    }

    if !org.has_policy {
        issues.push("No data protection policy in place".to_string());
        recommendations.push("Implement a comprehensive data protection policy".to_string());
    }

    let compliant = issues.is_empty();

    ComplianceReport {
        compliant,
        issues,
        recommendations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_consent() {
        let consent = ConsentRecord::builder()
            .data_subject_id("customer@example.com")
            .purpose(PurposeOfCollection::Marketing)
            .consent_method(ConsentMethod::Written)
            .add_data_category(PersonalDataCategory::Name)
            .add_data_category(PersonalDataCategory::ContactInfo)
            .notice_given(true)
            .build()
            .expect("Valid consent");

        assert!(consent.validate().is_ok());
    }

    #[test]
    fn test_invalid_consent_no_notice() {
        let consent = ConsentRecord::builder()
            .data_subject_id("customer@example.com")
            .purpose(PurposeOfCollection::Marketing)
            .consent_method(ConsentMethod::Written)
            .add_data_category(PersonalDataCategory::Name)
            .notice_given(false) // No notice given
            .build()
            .expect("Consent built");

        assert!(consent.validate().is_err());
    }

    #[test]
    fn test_organisation_compliance() {
        let org = PdpaOrganisation::new("Tech Sdn Bhd", "201601012345")
            .with_dpo(true)
            .with_policy(true);

        let report = validate_organisation_compliance(&org);
        assert!(report.compliant);
    }
}
