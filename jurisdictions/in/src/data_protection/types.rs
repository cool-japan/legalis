//! DPDPA Types
//!
//! # Digital Personal Data Protection Act, 2023

#![allow(missing_docs)]

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Data fiduciary category (Section 10)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataFiduciaryCategory {
    /// General data fiduciary
    General,
    /// Significant Data Fiduciary (SDF)
    Significant,
    /// Government agency
    Government,
    /// Start-up (registered with DPIIT)
    Startup,
}

impl DataFiduciaryCategory {
    pub fn name(&self) -> &str {
        match self {
            Self::General => "Data Fiduciary",
            Self::Significant => "Significant Data Fiduciary",
            Self::Government => "Government Data Fiduciary",
            Self::Startup => "Start-up Data Fiduciary",
        }
    }

    /// Whether additional obligations apply (Section 10)
    pub fn additional_obligations(&self) -> bool {
        matches!(self, Self::Significant)
    }
}

/// Criteria for Significant Data Fiduciary (SDF) designation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdfCriteria {
    /// Volume and sensitivity of personal data processed
    pub data_volume_significant: bool,
    /// Risk to rights of data principals
    pub risk_to_rights: bool,
    /// Potential impact on sovereignty and integrity
    pub sovereignty_impact: bool,
    /// Risk to electoral democracy
    pub electoral_risk: bool,
    /// Risk to security of State
    pub security_risk: bool,
    /// Risk to public order
    pub public_order_risk: bool,
}

impl SdfCriteria {
    /// Check if meets SDF threshold
    pub fn qualifies_as_sdf(&self) -> bool {
        self.data_volume_significant
            || self.risk_to_rights
            || self.sovereignty_impact
            || self.electoral_risk
            || self.security_risk
            || self.public_order_risk
    }
}

/// Lawful purposes for processing (Section 4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LawfulPurpose {
    /// Processing based on consent
    Consent,
    /// Processing for certain legitimate uses (Section 7)
    LegitimateUse(LegitimateUseType),
}

/// Legitimate use types (Section 7)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegitimateUseType {
    /// Voluntary provision for specified purpose
    VoluntaryProvision,
    /// State function - subsidy, benefit, service, license, permit
    StateFunction,
    /// Legal obligation of the State
    StateLegalObligation,
    /// Compliance with court judgment or order
    CourtOrder,
    /// Medical emergency (life or immediate health threat)
    MedicalEmergency,
    /// Disaster or public order breakdown
    EmergencySituation,
    /// Employment purpose
    Employment,
}

impl LegitimateUseType {
    pub fn description(&self) -> &str {
        match self {
            Self::VoluntaryProvision => "Voluntarily provided for specified purpose",
            Self::StateFunction => "State subsidy, benefit, service, license, or permit",
            Self::StateLegalObligation => "Compliance with legal obligation of State",
            Self::CourtOrder => "Compliance with court judgment or order",
            Self::MedicalEmergency => "Medical emergency involving threat to life/health",
            Self::EmergencySituation => "Disaster, breakdown of public order, or epidemic",
            Self::Employment => "Employment purposes",
        }
    }

    pub fn section(&self) -> &str {
        "Section 7"
    }
}

/// Consent record
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Data principal ID
    pub principal_id: String,
    /// Consent date
    pub consent_date: NaiveDate,
    /// Purpose of processing
    pub purpose: String,
    /// Personal data items consented
    pub data_items: Vec<String>,
    /// Consent obtained through consent manager
    pub via_consent_manager: bool,
    /// Consent manager ID (if applicable)
    pub consent_manager_id: Option<String>,
    /// Is consent for specific, clear, lawful purpose
    pub specific_purpose: bool,
    /// Is consent limited to necessary personal data
    pub limited_to_necessary: bool,
    /// Consent language
    pub language: String,
    /// Withdrawal date (if withdrawn)
    pub withdrawn_at: Option<NaiveDate>,
}

impl ConsentRecord {
    /// Check if consent is valid (Section 6)
    pub fn is_valid(&self) -> bool {
        self.specific_purpose && self.limited_to_necessary && self.withdrawn_at.is_none()
    }

    /// Check if consent was withdrawn
    pub fn is_withdrawn(&self) -> bool {
        self.withdrawn_at.is_some()
    }
}

/// Data principal rights (Chapter III)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataPrincipalRight {
    /// Right to access information (Section 11)
    Access,
    /// Right to correction (Section 12)
    Correction,
    /// Right to erasure (Section 12)
    Erasure,
    /// Right to grievance redressal (Section 13)
    GrievanceRedressal,
    /// Right to nominate (Section 14)
    Nomination,
}

impl DataPrincipalRight {
    pub fn description(&self) -> &str {
        match self {
            Self::Access => "Right to access summary of personal data and processing activities",
            Self::Correction => "Right to correction of inaccurate or misleading personal data",
            Self::Erasure => "Right to erasure of personal data no longer necessary",
            Self::GrievanceRedressal => "Right to have grievances redressed",
            Self::Nomination => "Right to nominate person to exercise rights",
        }
    }

    pub fn section(&self) -> u8 {
        match self {
            Self::Access => 11,
            Self::Correction | Self::Erasure => 12,
            Self::GrievanceRedressal => 13,
            Self::Nomination => 14,
        }
    }
}

/// Data principal duties (Section 15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataPrincipalDuty {
    /// Comply with provisions when exercising rights
    ComplyWithLaw,
    /// Not provide false particulars
    NoFalseParticulars,
    /// Not file false or frivolous complaint
    NoFrivolousComplaint,
    /// Not impersonate another person
    NoImpersonation,
    /// Not suppress material information
    NoSuppression,
    /// Provide authentic information for documents/services
    AuthenticInformation,
}

impl DataPrincipalDuty {
    pub fn description(&self) -> &str {
        match self {
            Self::ComplyWithLaw => "Comply with applicable law when exercising rights",
            Self::NoFalseParticulars => "Not register false or frivolous grievance",
            Self::NoFrivolousComplaint => "Not file false or frivolous complaint",
            Self::NoImpersonation => "Not impersonate another person while providing data",
            Self::NoSuppression => "Not suppress any material information",
            Self::AuthenticInformation => "Provide verifiably authentic information",
        }
    }
}

/// Data Fiduciary entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataFiduciary {
    /// Registration number
    pub registration_number: Option<String>,
    /// Entity name
    pub name: String,
    /// Category
    pub category: DataFiduciaryCategory,
    /// Principal place of business
    pub principal_place: String,
    /// Contact details
    pub contact_email: String,
    /// Data Protection Officer (for SDF)
    pub dpo: Option<DataProtectionOfficer>,
    /// Consent Manager registered
    pub consent_manager: Option<String>,
    /// Date of registration
    pub registration_date: Option<NaiveDate>,
}

/// Data Protection Officer (Section 10)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataProtectionOfficer {
    /// Name
    pub name: String,
    /// Designation
    pub designation: String,
    /// Contact details
    pub contact_email: String,
    /// Phone number
    pub phone: String,
    /// Based in India
    pub based_in_india: bool,
    /// Appointment date
    pub appointment_date: NaiveDate,
}

/// Consent Manager (Section 6)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentManager {
    /// Registration number with Board
    pub registration_number: String,
    /// Name
    pub name: String,
    /// Contact details
    pub contact_email: String,
    /// Registered with Data Protection Board
    pub registered_with_board: bool,
    /// Interoperability capability
    pub interoperable: bool,
}

/// Child's personal data processing (Section 9)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildDataProcessing {
    /// Child's identifier (anonymous)
    pub child_id: String,
    /// Parent/guardian consent obtained
    pub parental_consent: bool,
    /// Age verified
    pub age_verified: bool,
    /// Verification method
    pub verification_method: String,
    /// Processing purpose
    pub purpose: String,
    /// Detrimental processing check done
    pub detrimental_check: bool,
    /// Tracking/behavioral monitoring
    pub tracking_enabled: bool,
    /// Targeted advertising
    pub targeted_advertising: bool,
}

impl ChildDataProcessing {
    /// Check compliance with Section 9
    pub fn is_compliant(&self) -> bool {
        self.parental_consent
            && self.age_verified
            && self.detrimental_check
            && !self.tracking_enabled
            && !self.targeted_advertising
    }
}

/// Cross-border transfer (Section 16)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossBorderTransfer {
    /// Destination country
    pub destination_country: String,
    /// Receiving entity
    pub receiving_entity: String,
    /// Purpose of transfer
    pub purpose: String,
    /// Country on restricted list
    pub country_restricted: bool,
    /// Government notification allows transfer
    pub transfer_allowed: bool,
    /// Transfer date
    pub transfer_date: NaiveDate,
}

impl CrossBorderTransfer {
    /// Check if transfer is permitted (Section 16)
    pub fn is_permitted(&self) -> bool {
        !self.country_restricted || self.transfer_allowed
    }
}

/// Penalty tier (Section 33)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyTier {
    /// Up to Rs. 10,000 for data principal breach of duty
    DataPrincipalBreach,
    /// Up to Rs. 50 crore
    Tier1,
    /// Up to Rs. 200 crore
    Tier2,
    /// Up to Rs. 250 crore
    Tier3,
}

impl PenaltyTier {
    /// Maximum penalty amount in rupees
    pub fn max_amount_rupees(&self) -> u64 {
        match self {
            Self::DataPrincipalBreach => 10_000,
            Self::Tier1 => 500_000_000,   // 50 crore
            Self::Tier2 => 2_000_000_000, // 200 crore
            Self::Tier3 => 2_500_000_000, // 250 crore
        }
    }

    /// Description of tier
    pub fn description(&self) -> &str {
        match self {
            Self::DataPrincipalBreach => "Breach of data principal duties",
            Self::Tier1 => "Failure to take security safeguards",
            Self::Tier2 => {
                "Failure to notify data breach, non-compliance with children's data provisions"
            }
            Self::Tier3 => {
                "Non-compliance with processing without lawful grounds, other provisions"
            }
        }
    }
}

/// Processing activity record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingRecord {
    /// Record ID
    pub id: String,
    /// Data fiduciary
    pub fiduciary_name: String,
    /// Data processor (if any)
    pub processor_name: Option<String>,
    /// Personal data categories
    pub data_categories: Vec<String>,
    /// Purpose of processing
    pub purpose: String,
    /// Lawful basis
    pub lawful_basis: LawfulPurpose,
    /// Retention period
    pub retention_period_days: Option<u32>,
    /// Cross-border transfer
    pub cross_border: bool,
    /// Processing started
    pub start_date: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdf_criteria() {
        let criteria = SdfCriteria {
            data_volume_significant: true,
            risk_to_rights: false,
            sovereignty_impact: false,
            electoral_risk: false,
            security_risk: false,
            public_order_risk: false,
        };
        assert!(criteria.qualifies_as_sdf());
    }

    #[test]
    fn test_consent_validity() {
        let consent = ConsentRecord {
            principal_id: "DP001".to_string(),
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            purpose: "Marketing".to_string(),
            data_items: vec!["email".to_string()],
            via_consent_manager: false,
            consent_manager_id: None,
            specific_purpose: true,
            limited_to_necessary: true,
            language: "English".to_string(),
            withdrawn_at: None,
        };
        assert!(consent.is_valid());
    }

    #[test]
    fn test_child_data_compliance() {
        let processing = ChildDataProcessing {
            child_id: "CH001".to_string(),
            parental_consent: true,
            age_verified: true,
            verification_method: "DigiLocker".to_string(),
            purpose: "Education".to_string(),
            detrimental_check: true,
            tracking_enabled: false,
            targeted_advertising: false,
        };
        assert!(processing.is_compliant());
    }

    #[test]
    #[allow(clippy::inconsistent_digit_grouping)] // Indian number system uses lakh/crore grouping
    fn test_penalty_amounts() {
        assert_eq!(PenaltyTier::Tier1.max_amount_rupees(), 50_00_00_000); // 50 crore
        assert_eq!(PenaltyTier::Tier2.max_amount_rupees(), 200_00_00_000); // 200 crore
        assert_eq!(PenaltyTier::Tier3.max_amount_rupees(), 250_00_00_000); // 250 crore
    }

    #[test]
    fn test_data_principal_rights() {
        assert_eq!(DataPrincipalRight::Access.section(), 11);
        assert_eq!(DataPrincipalRight::Correction.section(), 12);
    }
}
