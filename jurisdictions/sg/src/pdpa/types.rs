//! Personal Data Protection Act 2012 - Type Definitions
//!
//! This module provides type-safe representations of Singapore's PDPA framework,
//! including consent management, data breach notifications, and DNC registry.
//!
//! ## Key Differences from GDPR
//!
//! | Feature | GDPR | PDPA (Singapore) |
//! |---------|------|------------------|
//! | **Legal Basis** | 6 lawful bases | Consent-centric |
//! | **DPO** | Mandatory for certain | Recommended (not mandatory) |
//! | **Breach Notification** | 72 hours | 3 calendar days |
//! | **Fines** | Up to €20M/4% revenue | Up to SGD 1M |
//! | **Right to be Forgotten** | Explicit (Art. 17) | Limited (correction/access) |
//! | **DNC Registry** | No equivalent | Part IX - opt-out |
//!
//! ## PDPA Obligations
//!
//! 1. **Consent** (s. 13): Obtain valid consent
//! 2. **Purpose Limitation** (s. 18): Collect only for reasonable purposes
//! 3. **Data Breach Notification** (s. 26B/26C): Notify PDPC within 3 days
//! 4. **DNC Compliance** (Part IX): Check Do Not Call Registry
//! 5. **Access Requests** (s. 21): Respond within 30 days

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// PDPA-regulated organisation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdpaOrganisation {
    /// Organisation name
    pub name: String,

    /// Unique Entity Number (UEN)
    pub uen: Option<String>,

    /// Organisation type
    pub organisation_type: OrganisationType,

    /// Whether DPO is appointed (recommended but not mandatory)
    pub has_dpo: bool,

    /// DPO contact details (if appointed)
    pub dpo_contact: Option<DpoContact>,

    /// Data protection policy URL
    pub privacy_policy_url: Option<String>,

    /// Date of last DPIA (Data Protection Impact Assessment)
    pub last_dpia_date: Option<DateTime<Utc>>,
}

/// Organisation type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganisationType {
    /// Private sector company
    Private,
    /// Public agency (subject to different rules)
    PublicAgency,
    /// Non-profit organisation
    NonProfit,
}

/// Data Protection Officer contact information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DpoContact {
    /// DPO name
    pub name: String,

    /// DPO email
    pub email: String,

    /// DPO phone
    pub phone: String,

    /// Appointment date
    pub appointed_date: DateTime<Utc>,
}

impl DpoContact {
    /// Creates a new DPO contact
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        phone: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            phone: phone.into(),
            appointed_date: Utc::now(),
        }
    }
}

/// Consent record for PDPA s. 13 compliance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Unique consent ID
    pub consent_id: String,

    /// Data subject identifier (email, phone, NRIC partial, etc.)
    pub data_subject_id: String,

    /// Purpose of data collection
    pub purpose: PurposeOfCollection,

    /// Categories of personal data collected
    pub data_categories: Vec<PersonalDataCategory>,

    /// Consent method
    pub consent_method: ConsentMethod,

    /// Timestamp of consent
    pub consent_timestamp: DateTime<Utc>,

    /// Whether consent is still valid
    pub is_valid: bool,

    /// Withdrawal timestamp (if withdrawn)
    pub withdrawal_timestamp: Option<DateTime<Utc>>,

    /// Withdrawal reason
    pub withdrawal_reason: Option<String>,
}

impl ConsentRecord {
    /// Creates a new consent record
    pub fn new(
        consent_id: impl Into<String>,
        data_subject_id: impl Into<String>,
        purpose: PurposeOfCollection,
        consent_method: ConsentMethod,
    ) -> Self {
        Self {
            consent_id: consent_id.into(),
            data_subject_id: data_subject_id.into(),
            purpose,
            data_categories: Vec::new(),
            consent_method,
            consent_timestamp: Utc::now(),
            is_valid: true,
            withdrawal_timestamp: None,
            withdrawal_reason: None,
        }
    }

    /// Adds a data category to this consent
    pub fn add_data_category(&mut self, category: PersonalDataCategory) {
        if !self.data_categories.contains(&category) {
            self.data_categories.push(category);
        }
    }

    /// Withdraws consent
    pub fn withdraw(&mut self, reason: Option<String>) {
        self.is_valid = false;
        self.withdrawal_timestamp = Some(Utc::now());
        self.withdrawal_reason = reason;
    }
}

/// Consent method for data collection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentMethod {
    /// Written consent (signed document)
    Written,
    /// Electronic consent (checkbox, click-through)
    Electronic,
    /// Oral consent (phone call)
    Oral,
    /// Deemed consent (s. 15 - existing relationship)
    Deemed,
}

/// Purpose of personal data collection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PurposeOfCollection {
    /// Marketing and promotional purposes
    Marketing,
    /// Service delivery
    ServiceDelivery,
    /// Customer support
    CustomerSupport,
    /// Compliance and legal requirements
    Compliance,
    /// Analytics and research
    Analytics,
    /// Employment screening
    EmploymentScreening,
}

/// Category of personal data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonalDataCategory {
    /// Email address
    Email,
    /// Phone number
    Phone,
    /// NRIC/FIN (full or partial)
    NricFin,
    /// Name
    Name,
    /// Address
    Address,
    /// Date of birth
    DateOfBirth,
    /// Financial information
    Financial,
    /// Health information
    Health,
    /// Biometric data
    Biometric,
}

/// Data breach notification (s. 26B/26C/26D)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataBreachNotification {
    /// Breach ID
    pub breach_id: String,

    /// Type of breach
    pub breach_type: BreachType,

    /// Date/time breach was discovered
    pub discovery_date: DateTime<Utc>,

    /// Date/time breach occurred (if known)
    pub occurrence_date: Option<DateTime<Utc>>,

    /// Number of affected individuals
    pub affected_individuals: u32,

    /// Categories of personal data affected
    pub affected_data_categories: Vec<PersonalDataCategory>,

    /// Whether breach is notifiable to PDPC (s. 26B)
    pub is_notifiable: bool,

    /// Date notified to PDPC (must be within 3 calendar days)
    pub pdpc_notification_date: Option<DateTime<Utc>>,

    /// Date individuals notified
    pub individuals_notification_date: Option<DateTime<Utc>>,

    /// Description of breach
    pub description: String,

    /// Remedial actions taken
    pub remedial_actions: Vec<String>,
}

impl DataBreachNotification {
    /// Creates a new data breach notification
    pub fn new(
        breach_id: impl Into<String>,
        breach_type: BreachType,
        affected_individuals: u32,
        description: impl Into<String>,
    ) -> Self {
        Self {
            breach_id: breach_id.into(),
            breach_type,
            discovery_date: Utc::now(),
            occurrence_date: None,
            affected_individuals,
            affected_data_categories: Vec::new(),
            is_notifiable: Self::determine_notifiability(affected_individuals, &breach_type),
            pdpc_notification_date: None,
            individuals_notification_date: None,
            description: description.into(),
            remedial_actions: Vec::new(),
        }
    }

    /// Determines if breach is notifiable to PDPC (s. 26B)
    pub fn determine_notifiability(affected_individuals: u32, breach_type: &BreachType) -> bool {
        // Notifiable if significant harm or scale
        // Simplified: assume > 500 individuals or certain breach types are notifiable
        affected_individuals > 500
            || matches!(
                breach_type,
                BreachType::UnauthorizedAccess | BreachType::DataLoss
            )
    }

    /// Checks if notification is within 3-day deadline (s. 26C)
    pub fn is_timely_notification(&self) -> bool {
        if let Some(notification_date) = self.pdpc_notification_date {
            let elapsed = notification_date
                .signed_duration_since(self.discovery_date)
                .num_days();
            elapsed <= 3
        } else {
            false
        }
    }

    /// Records PDPC notification
    pub fn notify_pdpc(&mut self) {
        self.pdpc_notification_date = Some(Utc::now());
    }

    /// Records individual notification
    pub fn notify_individuals(&mut self) {
        self.individuals_notification_date = Some(Utc::now());
    }

    /// Adds a remedial action
    pub fn add_remedial_action(&mut self, action: impl Into<String>) {
        self.remedial_actions.push(action.into());
    }
}

/// Type of data breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Unauthorized access to data
    UnauthorizedAccess,
    /// Unauthorized disclosure
    UnauthorizedDisclosure,
    /// Data loss
    DataLoss,
    /// Ransomware attack
    Ransomware,
    /// Phishing attack
    Phishing,
    /// Insider threat
    InsiderThreat,
}

/// Do Not Call (DNC) Registry entry (Part IX)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DncRegistry {
    /// Singapore phone number
    pub phone_number: String,

    /// Types of messages opted out from
    pub opt_out_types: Vec<DncType>,

    /// Registration date
    pub registration_date: DateTime<Utc>,

    /// Whether currently opted out
    pub is_opted_out: bool,
}

impl DncRegistry {
    /// Creates a new DNC registry entry
    pub fn new(phone_number: impl Into<String>) -> Self {
        Self {
            phone_number: phone_number.into(),
            opt_out_types: Vec::new(),
            registration_date: Utc::now(),
            is_opted_out: false,
        }
    }

    /// Opts out from specific message types
    pub fn opt_out(&mut self, dnc_types: Vec<DncType>) {
        self.opt_out_types = dnc_types;
        self.is_opted_out = !self.opt_out_types.is_empty();
    }

    /// Checks if opted out from specific type
    pub fn is_opted_out_from(&self, dnc_type: DncType) -> bool {
        self.opt_out_types.contains(&dnc_type)
    }
}

/// DNC message type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DncType {
    /// Voice calls
    VoiceCall,
    /// SMS/text messages
    SmsText,
    /// Fax messages
    Fax,
}

/// Cross-border data transfer record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTransfer {
    /// Transfer ID
    pub transfer_id: String,

    /// Destination country
    pub destination_country: String,

    /// Purpose of transfer
    pub purpose: String,

    /// Legal mechanism for transfer
    pub legal_basis: TransferLegalBasis,

    /// Date of transfer
    pub transfer_date: DateTime<Utc>,

    /// Categories of data transferred
    pub data_categories: Vec<PersonalDataCategory>,

    /// Number of individuals affected
    pub affected_individuals: u32,
}

impl DataTransfer {
    /// Creates a new cross-border transfer record
    pub fn new(
        transfer_id: impl Into<String>,
        destination_country: impl Into<String>,
        purpose: impl Into<String>,
        legal_basis: TransferLegalBasis,
    ) -> Self {
        Self {
            transfer_id: transfer_id.into(),
            destination_country: destination_country.into(),
            purpose: purpose.into(),
            legal_basis,
            transfer_date: Utc::now(),
            data_categories: Vec::new(),
            affected_individuals: 0,
        }
    }
}

/// Legal basis for cross-border transfer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferLegalBasis {
    /// Consent of individual
    Consent,
    /// Performance of contract
    Contract,
    /// Legally required
    LegalObligation,
    /// Protection of vital interests
    VitalInterests,
    /// Comparable standard of protection in destination
    ComparableProtection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_record_creation() {
        let consent = ConsentRecord::new(
            "consent-001",
            "user@example.com",
            PurposeOfCollection::Marketing,
            ConsentMethod::Electronic,
        );

        assert_eq!(consent.consent_id, "consent-001");
        assert!(consent.is_valid);
        assert!(consent.withdrawal_timestamp.is_none());
    }

    #[test]
    fn test_consent_withdrawal() {
        let mut consent = ConsentRecord::new(
            "consent-002",
            "user@example.com",
            PurposeOfCollection::ServiceDelivery,
            ConsentMethod::Written,
        );

        consent.withdraw(Some("No longer interested".to_string()));

        assert!(!consent.is_valid);
        assert!(consent.withdrawal_timestamp.is_some());
        assert_eq!(
            consent.withdrawal_reason,
            Some("No longer interested".to_string())
        );
    }

    #[test]
    fn test_data_breach_notifiability() {
        let breach = DataBreachNotification::new(
            "breach-001",
            BreachType::UnauthorizedAccess,
            1000,
            "Database exposed",
        );

        assert!(breach.is_notifiable);
        assert_eq!(breach.affected_individuals, 1000);
    }

    #[test]
    fn test_breach_notification_timing() {
        let mut breach =
            DataBreachNotification::new("breach-002", BreachType::DataLoss, 100, "USB drive lost");

        breach.notify_pdpc();

        assert!(breach.is_timely_notification());
        assert!(breach.pdpc_notification_date.is_some());
    }

    #[test]
    fn test_dnc_registry() {
        let mut dnc = DncRegistry::new("+6598765432");

        assert!(!dnc.is_opted_out);

        dnc.opt_out(vec![DncType::VoiceCall, DncType::SmsText]);

        assert!(dnc.is_opted_out);
        assert!(dnc.is_opted_out_from(DncType::VoiceCall));
        assert!(dnc.is_opted_out_from(DncType::SmsText));
        assert!(!dnc.is_opted_out_from(DncType::Fax));
    }

    #[test]
    fn test_dpo_contact() {
        let dpo = DpoContact::new("Jane Tan", "dpo@company.sg", "+6512345678");

        assert_eq!(dpo.name, "Jane Tan");
        assert_eq!(dpo.email, "dpo@company.sg");
        assert_eq!(dpo.phone, "+6512345678");
    }

    #[test]
    fn test_data_transfer() {
        let transfer = DataTransfer::new(
            "transfer-001",
            "United States",
            "Cloud backup",
            TransferLegalBasis::Consent,
        );

        assert_eq!(transfer.destination_country, "United States");
        assert_eq!(transfer.legal_basis, TransferLegalBasis::Consent);
    }
}
