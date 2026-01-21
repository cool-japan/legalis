//! Core types for Australian Privacy Law
//!
//! This module defines the fundamental data structures for privacy law compliance
//! under the Privacy Act 1988.
//!
//! ## Key Definitions
//!
//! - **Personal Information**: Information about an identified or reasonably
//!   identifiable individual (s.6(1))
//! - **Sensitive Information**: Special category of personal information
//!   requiring additional protections (s.6(1))
//! - **APP Entity**: Organisation or agency to which APPs apply

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Personal information as defined in s.6(1) Privacy Act 1988
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalInformation {
    /// Unique identifier for this information record
    pub record_id: String,
    /// Type of personal information
    pub information_type: PersonalInformationType,
    /// Whether this is sensitive information
    pub is_sensitive: bool,
    /// Description of the information
    pub description: String,
    /// Data subject identifier (pseudonymised)
    pub data_subject_id: String,
    /// Collection date
    pub collection_date: Option<DateTime<Utc>>,
    /// Purpose of collection
    pub collection_purpose: Option<String>,
    /// Source of collection
    pub collection_source: CollectionSource,
    /// Current retention status
    pub retention_status: RetentionStatus,
}

impl PersonalInformation {
    /// Create new personal information record
    pub fn new(
        record_id: impl Into<String>,
        information_type: PersonalInformationType,
        data_subject_id: impl Into<String>,
    ) -> Self {
        let is_sensitive = information_type.is_sensitive();
        Self {
            record_id: record_id.into(),
            information_type,
            is_sensitive,
            description: String::new(),
            data_subject_id: data_subject_id.into(),
            collection_date: Some(Utc::now()),
            collection_purpose: None,
            collection_source: CollectionSource::DirectFromIndividual,
            retention_status: RetentionStatus::Active,
        }
    }

    /// Set collection purpose
    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.collection_purpose = Some(purpose.into());
        self
    }

    /// Set collection source
    pub fn with_source(mut self, source: CollectionSource) -> Self {
        self.collection_source = source;
        self
    }

    /// Check if information is sensitive
    pub fn requires_consent(&self) -> bool {
        self.is_sensitive
    }
}

/// Type of personal information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonalInformationType {
    // Standard personal information
    /// Name
    Name,
    /// Contact details (address, phone, email)
    ContactDetails,
    /// Date of birth
    DateOfBirth,
    /// Financial information
    Financial,
    /// Employment information
    Employment,
    /// Education history
    Education,
    /// Location data
    Location,
    /// IP address
    IpAddress,
    /// Device identifier
    DeviceId,
    /// Photograph
    Photograph,
    /// Video recording
    VideoRecording,
    /// Voice recording
    VoiceRecording,

    // Sensitive information types
    /// Racial or ethnic origin
    RacialOrigin,
    /// Political opinions
    PoliticalOpinions,
    /// Religious beliefs
    ReligiousBeliefs,
    /// Philosophical beliefs
    PhilosophicalBeliefs,
    /// Trade union membership
    TradeUnionMembership,
    /// Sexual orientation
    SexualOrientation,
    /// Criminal record
    CriminalRecord,
    /// Health information
    Health,
    /// Genetic information
    Genetic,
    /// Biometric data
    Biometric,
}

impl PersonalInformationType {
    /// Check if this is sensitive information under s.6(1)
    pub fn is_sensitive(&self) -> bool {
        matches!(
            self,
            PersonalInformationType::RacialOrigin
                | PersonalInformationType::PoliticalOpinions
                | PersonalInformationType::ReligiousBeliefs
                | PersonalInformationType::PhilosophicalBeliefs
                | PersonalInformationType::TradeUnionMembership
                | PersonalInformationType::SexualOrientation
                | PersonalInformationType::CriminalRecord
                | PersonalInformationType::Health
                | PersonalInformationType::Genetic
                | PersonalInformationType::Biometric
        )
    }

    /// Get category name
    pub fn category(&self) -> &'static str {
        if self.is_sensitive() {
            "Sensitive Information"
        } else {
            "Personal Information"
        }
    }
}

/// Sensitive information as defined in s.6(1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensitiveInformation {
    /// The personal information record
    pub information: PersonalInformation,
    /// Specific sensitive type
    pub sensitive_type: SensitiveType,
    /// Consent status
    pub consent: Option<Consent>,
    /// Additional protections applied
    pub additional_protections: Vec<String>,
}

impl SensitiveInformation {
    /// Create new sensitive information
    pub fn new(information: PersonalInformation, sensitive_type: SensitiveType) -> Self {
        Self {
            information,
            sensitive_type,
            consent: None,
            additional_protections: Vec::new(),
        }
    }

    /// Check if valid consent exists
    pub fn has_valid_consent(&self) -> bool {
        self.consent.as_ref().map(|c| c.is_valid).unwrap_or(false)
    }
}

/// Type of sensitive information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensitiveType {
    /// Racial or ethnic origin
    RacialOrigin,
    /// Political opinions
    PoliticalOpinions,
    /// Religious beliefs or affiliations
    ReligiousBeliefs,
    /// Philosophical beliefs
    PhilosophicalBeliefs,
    /// Trade union membership
    TradeUnionMembership,
    /// Sexual orientation or practices
    SexualOrientation,
    /// Criminal record
    CriminalRecord,
    /// Health information
    Health,
    /// Genetic information
    Genetic,
    /// Biometric data for identification
    Biometric,
}

/// Collection source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectionSource {
    /// Direct from the individual
    DirectFromIndividual,
    /// From a third party with consent
    ThirdPartyWithConsent,
    /// From a third party without consent (permitted circumstances)
    ThirdPartyWithoutConsent,
    /// Publicly available sources
    PubliclyAvailable,
    /// Government records
    GovernmentRecords,
    /// Unsolicited (received but not requested)
    Unsolicited,
}

/// Retention status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetentionStatus {
    /// Actively held and used
    Active,
    /// Archived but accessible
    Archived,
    /// Scheduled for deletion
    PendingDeletion,
    /// Deleted/destroyed
    Deleted,
    /// Anonymised (no longer personal information)
    Anonymised,
}

/// Consent record under Privacy Act
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consent {
    /// Consent ID
    pub consent_id: String,
    /// Data subject ID
    pub data_subject_id: String,
    /// Purpose of consent
    pub purpose: ConsentPurpose,
    /// Method of obtaining consent
    pub method: ConsentMethod,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Whether consent is currently valid
    pub is_valid: bool,
    /// Withdrawal timestamp (if withdrawn)
    pub withdrawal_timestamp: Option<DateTime<Utc>>,
    /// Expiry date (if applicable)
    pub expiry_date: Option<DateTime<Utc>>,
    /// Consent statement shown to individual
    pub consent_statement: String,
}

impl Consent {
    /// Create new consent record
    pub fn new(
        consent_id: impl Into<String>,
        data_subject_id: impl Into<String>,
        purpose: ConsentPurpose,
        method: ConsentMethod,
        statement: impl Into<String>,
    ) -> Self {
        Self {
            consent_id: consent_id.into(),
            data_subject_id: data_subject_id.into(),
            purpose,
            method,
            timestamp: Utc::now(),
            is_valid: true,
            withdrawal_timestamp: None,
            expiry_date: None,
            consent_statement: statement.into(),
        }
    }

    /// Withdraw consent
    pub fn withdraw(&mut self) {
        self.is_valid = false;
        self.withdrawal_timestamp = Some(Utc::now());
    }

    /// Check if consent has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry_date {
            Utc::now() > expiry
        } else {
            false
        }
    }

    /// Check if consent is valid (not withdrawn and not expired)
    pub fn is_currently_valid(&self) -> bool {
        self.is_valid && !self.is_expired()
    }
}

/// Purpose of consent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentPurpose {
    /// Collection of personal information
    Collection,
    /// Use of personal information
    Use,
    /// Disclosure to third parties
    Disclosure,
    /// Cross-border transfer
    CrossBorderTransfer,
    /// Direct marketing
    DirectMarketing,
    /// Collection of sensitive information
    SensitiveCollection,
}

/// Method of obtaining consent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentMethod {
    /// Express written consent
    ExpressWritten,
    /// Express oral consent
    ExpressOral,
    /// Express electronic consent (checkbox, click-through)
    ExpressElectronic,
    /// Implied consent (from conduct)
    Implied,
    /// Opt-out consent (for direct marketing)
    OptOut,
}

impl ConsentMethod {
    /// Check if consent method is express
    pub fn is_express(&self) -> bool {
        matches!(
            self,
            ConsentMethod::ExpressWritten
                | ConsentMethod::ExpressOral
                | ConsentMethod::ExpressElectronic
        )
    }

    /// Check if consent method is valid for sensitive information
    pub fn valid_for_sensitive(&self) -> bool {
        // Sensitive information requires express consent
        self.is_express()
    }
}

/// Privacy policy requirements under APP 1
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivacyPolicy {
    /// Entity name
    pub entity_name: String,
    /// Policy version
    pub version: String,
    /// Last updated date
    pub last_updated: DateTime<Utc>,
    /// Policy URL (if online)
    pub policy_url: Option<String>,
    /// Types of personal information collected
    pub information_types_collected: Vec<PersonalInformationType>,
    /// Purposes of collection
    pub collection_purposes: Vec<String>,
    /// How information is collected
    pub collection_methods: Vec<String>,
    /// Third parties to whom information may be disclosed
    pub disclosure_recipients: Vec<String>,
    /// Whether information may be disclosed overseas
    pub overseas_disclosure: bool,
    /// Countries to which information may be disclosed
    pub overseas_countries: Vec<String>,
    /// How to access or correct information
    pub access_correction_process: String,
    /// How to make a complaint
    pub complaints_process: String,
    /// Whether likely to disclose to overseas recipients
    pub likely_overseas_disclosure: bool,
}

impl PrivacyPolicy {
    /// Create new privacy policy
    pub fn new(entity_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            entity_name: entity_name.into(),
            version: version.into(),
            last_updated: Utc::now(),
            policy_url: None,
            information_types_collected: Vec::new(),
            collection_purposes: Vec::new(),
            collection_methods: Vec::new(),
            disclosure_recipients: Vec::new(),
            overseas_disclosure: false,
            overseas_countries: Vec::new(),
            access_correction_process: String::new(),
            complaints_process: String::new(),
            likely_overseas_disclosure: false,
        }
    }

    /// Check if policy covers APP 1 requirements
    pub fn meets_app1_requirements(&self) -> bool {
        !self.information_types_collected.is_empty()
            && !self.collection_purposes.is_empty()
            && !self.access_correction_process.is_empty()
            && !self.complaints_process.is_empty()
    }
}

/// Access request under APP 12
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessRequest {
    /// Request ID
    pub request_id: String,
    /// Requestor ID
    pub requestor_id: String,
    /// Request date
    pub request_date: DateTime<Utc>,
    /// Information requested
    pub information_requested: String,
    /// Request status
    pub status: AccessRequestStatus,
    /// Response due date (30 days)
    pub due_date: DateTime<Utc>,
    /// Response date (if responded)
    pub response_date: Option<DateTime<Utc>>,
    /// Access granted
    pub access_granted: bool,
    /// Refusal reason (if refused)
    pub refusal_reason: Option<AccessRefusalReason>,
    /// Charge applied (if any)
    pub charge_aud: Option<f64>,
}

impl AccessRequest {
    /// Create new access request
    pub fn new(
        request_id: impl Into<String>,
        requestor_id: impl Into<String>,
        information_requested: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        let due = now + chrono::Duration::days(30);
        Self {
            request_id: request_id.into(),
            requestor_id: requestor_id.into(),
            request_date: now,
            information_requested: information_requested.into(),
            status: AccessRequestStatus::Received,
            due_date: due,
            response_date: None,
            access_granted: false,
            refusal_reason: None,
            charge_aud: None,
        }
    }

    /// Grant access
    pub fn grant(&mut self) {
        self.status = AccessRequestStatus::Granted;
        self.access_granted = true;
        self.response_date = Some(Utc::now());
    }

    /// Refuse access
    pub fn refuse(&mut self, reason: AccessRefusalReason) {
        self.status = AccessRequestStatus::Refused;
        self.access_granted = false;
        self.refusal_reason = Some(reason);
        self.response_date = Some(Utc::now());
    }

    /// Check if response is overdue
    pub fn is_overdue(&self) -> bool {
        self.response_date.is_none() && Utc::now() > self.due_date
    }
}

/// Access request status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessRequestStatus {
    /// Request received
    Received,
    /// Under assessment
    UnderAssessment,
    /// Access granted
    Granted,
    /// Access partially granted
    PartiallyGranted,
    /// Access refused
    Refused,
    /// Request withdrawn
    Withdrawn,
}

/// Reason for refusing access under APP 12.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessRefusalReason {
    /// Serious threat to life/health/safety
    SeriousThreat,
    /// Unreasonable impact on privacy of others
    PrivacyImpact,
    /// Frivolous or vexatious request
    FrivolousVexatious,
    /// Legal proceedings
    LegalProceedings,
    /// Prejudice negotiations
    PrejudiceNegotiations,
    /// Unlawful activity
    UnlawfulActivity,
    /// Enforcement body activity
    EnforcementActivity,
    /// Security of Australia
    NationalSecurity,
}

/// Correction request under APP 13
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrectionRequest {
    /// Request ID
    pub request_id: String,
    /// Requestor ID
    pub requestor_id: String,
    /// Request date
    pub request_date: DateTime<Utc>,
    /// Information to be corrected
    pub information_to_correct: String,
    /// Proposed correction
    pub proposed_correction: String,
    /// Request status
    pub status: CorrectionRequestStatus,
    /// Response due date (30 days)
    pub due_date: DateTime<Utc>,
    /// Response date
    pub response_date: Option<DateTime<Utc>>,
    /// Correction made
    pub correction_made: bool,
    /// Statement attached (if refused)
    pub statement_attached: bool,
}

impl CorrectionRequest {
    /// Create new correction request
    pub fn new(
        request_id: impl Into<String>,
        requestor_id: impl Into<String>,
        information: impl Into<String>,
        proposed: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            request_id: request_id.into(),
            requestor_id: requestor_id.into(),
            request_date: now,
            information_to_correct: information.into(),
            proposed_correction: proposed.into(),
            status: CorrectionRequestStatus::Received,
            due_date: now + chrono::Duration::days(30),
            response_date: None,
            correction_made: false,
            statement_attached: false,
        }
    }
}

/// Correction request status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorrectionRequestStatus {
    /// Request received
    Received,
    /// Under assessment
    UnderAssessment,
    /// Correction made
    CorrectionMade,
    /// Correction refused
    Refused,
    /// Statement attached
    StatementAttached,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personal_information_creation() {
        let info = PersonalInformation::new("rec-001", PersonalInformationType::Name, "user-123")
            .with_purpose("Customer service");

        assert_eq!(info.record_id, "rec-001");
        assert!(!info.is_sensitive);
        assert!(info.collection_purpose.is_some());
    }

    #[test]
    fn test_sensitive_information() {
        let info = PersonalInformation::new("rec-002", PersonalInformationType::Health, "user-123");
        assert!(info.is_sensitive);
        assert!(info.requires_consent());
    }

    #[test]
    fn test_information_type_sensitivity() {
        assert!(!PersonalInformationType::Name.is_sensitive());
        assert!(PersonalInformationType::Health.is_sensitive());
        assert!(PersonalInformationType::Biometric.is_sensitive());
        assert!(PersonalInformationType::CriminalRecord.is_sensitive());
    }

    #[test]
    fn test_consent_creation() {
        let consent = Consent::new(
            "consent-001",
            "user-123",
            ConsentPurpose::Collection,
            ConsentMethod::ExpressElectronic,
            "I consent to collection of my personal information",
        );

        assert!(consent.is_valid);
        assert!(consent.is_currently_valid());
    }

    #[test]
    fn test_consent_withdrawal() {
        let mut consent = Consent::new(
            "consent-002",
            "user-123",
            ConsentPurpose::DirectMarketing,
            ConsentMethod::OptOut,
            "Marketing consent",
        );

        consent.withdraw();

        assert!(!consent.is_valid);
        assert!(consent.withdrawal_timestamp.is_some());
    }

    #[test]
    fn test_consent_method_validity() {
        assert!(ConsentMethod::ExpressWritten.is_express());
        assert!(ConsentMethod::ExpressWritten.valid_for_sensitive());
        assert!(!ConsentMethod::Implied.valid_for_sensitive());
    }

    #[test]
    fn test_access_request() {
        let request = AccessRequest::new("req-001", "user-123", "All personal information");

        assert_eq!(request.status, AccessRequestStatus::Received);
        assert!(!request.is_overdue());
    }

    #[test]
    fn test_privacy_policy_requirements() {
        let mut policy = PrivacyPolicy::new("Acme Corp", "1.0");
        assert!(!policy.meets_app1_requirements());

        policy
            .information_types_collected
            .push(PersonalInformationType::Name);
        policy.collection_purposes.push("Service delivery".into());
        policy.access_correction_process = "Contact privacy@acme.com".into();
        policy.complaints_process = "Lodge complaint with OAIC".into();

        assert!(policy.meets_app1_requirements());
    }
}
