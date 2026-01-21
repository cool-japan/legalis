//! Protection of Personal Information Act 4 of 2013 (POPIA)
//!
//! South Africa's comprehensive data protection law, came into full effect
//! 1 July 2021, with 1-year grace period ending 30 June 2022.
//!
//! ## Key Features
//!
//! - 8 conditions for lawful processing
//! - Information Regulator oversight
//! - Direct marketing opt-out requirement
//! - Mandatory data breach notification
//! - Cross-border transfer restrictions

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for POPIA operations
pub type PopiaResult<T> = Result<T, PopiaError>;

/// Personal information categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PersonalInformationCategory {
    /// General personal information
    General,
    /// Special personal information (s26-33)
    Special(SpecialPersonalInformation),
    /// Personal information of children (s34-35)
    Child,
}

/// Special personal information categories (s26)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialPersonalInformation {
    /// Religious or philosophical beliefs
    ReligiousBeliefs,
    /// Race or ethnic origin
    RaceOrEthnicOrigin,
    /// Trade union membership
    TradeUnionMembership,
    /// Political persuasion
    PoliticalPersuasion,
    /// Health or sex life
    HealthOrSexLife,
    /// Biometric information
    Biometric,
    /// Criminal behaviour
    CriminalBehaviour,
}

/// Conditions for lawful processing (Chapter 3)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProcessingCondition {
    /// Accountability (Condition 1)
    Accountability,
    /// Processing limitation (Condition 2)
    ProcessingLimitation,
    /// Purpose specification (Condition 3)
    PurposeSpecification,
    /// Further processing limitation (Condition 4)
    FurtherProcessingLimitation,
    /// Information quality (Condition 5)
    InformationQuality,
    /// Openness (Condition 6)
    Openness,
    /// Security safeguards (Condition 7)
    SecuritySafeguards,
    /// Data subject participation (Condition 8)
    DataSubjectParticipation,
}

impl ProcessingCondition {
    /// Get the statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::Accountability => "s8",
            Self::ProcessingLimitation => "s9-12",
            Self::PurposeSpecification => "s13-14",
            Self::FurtherProcessingLimitation => "s15",
            Self::InformationQuality => "s16",
            Self::Openness => "s17-18",
            Self::SecuritySafeguards => "s19-22",
            Self::DataSubjectParticipation => "s23-25",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Accountability => "Responsible party must ensure compliance",
            Self::ProcessingLimitation => "Lawful processing with consent or legitimate basis",
            Self::PurposeSpecification => "Collect for specific, explicit, legitimate purpose",
            Self::FurtherProcessingLimitation => "Compatible with original purpose",
            Self::InformationQuality => "Complete, accurate, not misleading",
            Self::Openness => "Document processing, notify data subject",
            Self::SecuritySafeguards => "Appropriate technical and organizational measures",
            Self::DataSubjectParticipation => "Access, correction, deletion rights",
        }
    }
}

/// Legal bases for processing (s11)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalBasis {
    /// Data subject consent
    Consent,
    /// Performance of contract
    Contract,
    /// Legal obligation
    LegalObligation,
    /// Protection of legitimate interests of data subject
    DataSubjectInterests,
    /// Public law duty
    PublicLawDuty,
    /// Legitimate interests of responsible party
    LegitimateInterests,
}

/// Data subject rights
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataSubjectRight {
    /// Right to be notified (s18)
    Notification,
    /// Right of access (s23)
    Access,
    /// Right to correction (s24)
    Correction,
    /// Right to deletion (s24)
    Deletion,
    /// Right to object to processing (s11(3))
    Objection,
    /// Right to object to direct marketing (s69)
    DirectMarketingOptOut,
    /// Right to lodge complaint (s74)
    Complaint,
}

impl DataSubjectRight {
    /// Get statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::Notification => "s18",
            Self::Access => "s23",
            Self::Correction => "s24(1)(a)",
            Self::Deletion => "s24(1)(b)",
            Self::Objection => "s11(3)",
            Self::DirectMarketingOptOut => "s69",
            Self::Complaint => "s74",
        }
    }
}

/// Cross-border transfer requirements (s72)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransferBasis {
    /// Adequate level of protection in recipient country
    AdequateProtection,
    /// Binding corporate rules
    BindingCorporateRules,
    /// Binding agreement with recipient
    BindingAgreement,
    /// Consent of data subject
    Consent,
    /// Performance of contract
    ContractPerformance,
    /// Legal proceedings
    LegalProceedings,
}

/// Information Officer registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationOfficer {
    /// Name
    pub name: String,
    /// Registered with Information Regulator
    pub registered: bool,
    /// Registration number
    pub registration_number: Option<String>,
    /// Contact email
    pub email: String,
}

/// POPIA compliance assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopiaCompliance {
    /// Information Officer appointed (s55)
    pub information_officer_appointed: bool,
    /// Information Officer registered (s56)
    pub information_officer_registered: bool,
    /// Privacy notice published (s18)
    pub privacy_notice_published: bool,
    /// Processing register maintained
    pub processing_register: bool,
    /// Security measures implemented (s19)
    pub security_measures: bool,
    /// Direct marketing consent obtained (s69)
    pub direct_marketing_consent: bool,
    /// Cross-border transfer compliance (s72)
    pub cross_border_compliant: bool,
    /// Data breach response plan
    pub breach_response_plan: bool,
}

impl PopiaCompliance {
    /// Check if fully compliant
    pub fn is_compliant(&self) -> bool {
        self.information_officer_appointed
            && self.information_officer_registered
            && self.privacy_notice_published
            && self.processing_register
            && self.security_measures
            && self.breach_response_plan
    }
}

/// POPIA errors
#[derive(Debug, Error)]
pub enum PopiaError {
    /// No legal basis for processing
    #[error("No lawful basis for processing personal information (s9)")]
    NoLegalBasis,

    /// Consent not valid
    #[error("Consent is not valid (s11): {reason}")]
    InvalidConsent { reason: String },

    /// Special information processed without authorization
    #[error("Special personal information processed without authorization (s26-33)")]
    UnauthorizedSpecialProcessing,

    /// Child information processed without consent
    #[error("Child's personal information processed without competent person's consent (s34-35)")]
    ChildDataWithoutConsent,

    /// Cross-border transfer violation
    #[error("Cross-border transfer without adequate safeguards (s72): {destination}")]
    UnlawfulTransfer { destination: String },

    /// Security compromise
    #[error("Security compromise notification required (s22): {description}")]
    SecurityCompromise { description: String },

    /// Information Officer not registered
    #[error("Information Officer not registered with Information Regulator (s55-56)")]
    InformationOfficerNotRegistered,

    /// Direct marketing violation
    #[error("Direct marketing without consent or opt-out (s69)")]
    DirectMarketingViolation,
}

/// Validate processing
pub fn validate_processing(
    category: &PersonalInformationCategory,
    legal_basis: &LegalBasis,
    has_consent: bool,
) -> PopiaResult<()> {
    // Special personal information requires consent or specific authorization
    if let PersonalInformationCategory::Special(_) = category
        && !has_consent
    {
        return Err(PopiaError::UnauthorizedSpecialProcessing);
    }

    // Child data requires competent person's consent
    if matches!(category, PersonalInformationCategory::Child) && !has_consent {
        return Err(PopiaError::ChildDataWithoutConsent);
    }

    // Check legal basis
    if *legal_basis == LegalBasis::Consent && !has_consent {
        return Err(PopiaError::InvalidConsent {
            reason: "Consent claimed but not obtained".to_string(),
        });
    }

    Ok(())
}

/// Get POPIA compliance checklist
pub fn get_popia_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Appoint Information Officer", "s55"),
        ("Register Information Officer", "s56"),
        ("Publish privacy notice", "s18"),
        ("Implement security safeguards", "s19"),
        ("Create breach response plan", "s22"),
        ("Document processing activities", "Reg 4"),
        ("Obtain consent for direct marketing", "s69"),
        ("Validate cross-border transfers", "s72"),
        ("Enable access requests", "s23"),
        ("Enable correction/deletion", "s24"),
        ("PAIA manual (if applicable)", "s51"),
        ("Training and awareness", "Best practice"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing_conditions() {
        let condition = ProcessingCondition::SecuritySafeguards;
        assert_eq!(condition.statutory_reference(), "s19-22");
    }

    #[test]
    fn test_data_subject_rights() {
        let right = DataSubjectRight::DirectMarketingOptOut;
        assert_eq!(right.statutory_reference(), "s69");
    }

    #[test]
    fn test_validate_general_processing() {
        let result = validate_processing(
            &PersonalInformationCategory::General,
            &LegalBasis::Contract,
            false,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_special_without_consent() {
        let result = validate_processing(
            &PersonalInformationCategory::Special(SpecialPersonalInformation::HealthOrSexLife),
            &LegalBasis::Consent,
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_special_with_consent() {
        let result = validate_processing(
            &PersonalInformationCategory::Special(SpecialPersonalInformation::HealthOrSexLife),
            &LegalBasis::Consent,
            true,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_child_without_consent() {
        let result = validate_processing(
            &PersonalInformationCategory::Child,
            &LegalBasis::Consent,
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_compliance_check() {
        let compliance = PopiaCompliance {
            information_officer_appointed: true,
            information_officer_registered: true,
            privacy_notice_published: true,
            processing_register: true,
            security_measures: true,
            direct_marketing_consent: true,
            cross_border_compliant: true,
            breach_response_plan: true,
        };

        assert!(compliance.is_compliant());
    }

    #[test]
    fn test_compliance_incomplete() {
        let compliance = PopiaCompliance {
            information_officer_appointed: true,
            information_officer_registered: false, // Not registered
            privacy_notice_published: true,
            processing_register: true,
            security_measures: true,
            direct_marketing_consent: true,
            cross_border_compliant: true,
            breach_response_plan: true,
        };

        assert!(!compliance.is_compliant());
    }

    #[test]
    fn test_popia_checklist() {
        let checklist = get_popia_checklist();
        assert!(!checklist.is_empty());
    }
}
