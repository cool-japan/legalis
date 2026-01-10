//! Core types for GDPR implementation
//!
//! This module defines the fundamental types used across GDPR Articles.

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// Lawful bases for processing personal data under GDPR Article 6(1)
///
/// Article 6(1) requires at least one of these legal bases for processing to be lawful.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum LawfulBasis {
    /// Article 6(1)(a) - Consent
    ///
    /// The data subject has given consent for processing for one or more specific purposes.
    /// Consent must be freely given, specific, informed, and unambiguous (Article 7).
    Consent {
        freely_given: bool,
        specific: bool,
        informed: bool,
        unambiguous: bool,
    },

    /// Article 6(1)(b) - Contract performance
    ///
    /// Processing is necessary for performance of a contract with the data subject,
    /// or to take steps at the request of the data subject prior to entering into a contract.
    Contract { necessary_for_performance: bool },

    /// Article 6(1)(c) - Legal obligation
    ///
    /// Processing is necessary for compliance with a legal obligation to which
    /// the controller is subject under EU or Member State law.
    LegalObligation {
        eu_law: Option<String>,
        member_state_law: Option<String>,
    },

    /// Article 6(1)(d) - Vital interests
    ///
    /// Processing is necessary to protect the vital interests of the data subject
    /// or another natural person (e.g., life-threatening emergency).
    VitalInterests { life_threatening: bool },

    /// Article 6(1)(e) - Public task
    ///
    /// Processing is necessary for performance of a task carried out in the public interest
    /// or in the exercise of official authority vested in the controller.
    PublicTask { task_basis: String },

    /// Article 6(1)(f) - Legitimate interests
    ///
    /// Processing is necessary for the purposes of legitimate interests pursued by
    /// the controller or a third party, except where overridden by data subject rights.
    /// Requires balancing test (Recital 47).
    LegitimateInterests {
        controller_interest: String,
        balancing_test_passed: bool,
    },
}

/// Personal data categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum PersonalDataCategory {
    /// Regular personal data (name, email, address, etc.)
    Regular(String),

    /// Special categories under Article 9
    Special(SpecialCategory),
}

/// Special categories of personal data (Article 9)
///
/// Processing of these categories is prohibited except under specific conditions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum SpecialCategory {
    /// Racial or ethnic origin
    RacialEthnicOrigin,

    /// Political opinions
    PoliticalOpinions,

    /// Religious or philosophical beliefs
    ReligiousBeliefs,

    /// Trade union membership
    TradeUnionMembership,

    /// Genetic data
    GeneticData,

    /// Biometric data for unique identification
    BiometricData,

    /// Health data
    HealthData,

    /// Data concerning sex life or sexual orientation
    SexLifeOrOrientation,
}

/// Processing operations (Article 4(2))
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ProcessingOperation {
    Collection,
    Recording,
    Organization,
    Structuring,
    Storage,
    Adaptation,
    Retrieval,
    Consultation,
    Use,
    Disclosure,
    Dissemination,
    Restriction,
    Erasure,
    Destruction,
    CrossBorderTransfer(CrossBorderMechanism),
}

/// Cross-border transfer mechanisms (Chapter V)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum CrossBorderMechanism {
    /// Article 45 - Adequacy decision by European Commission
    AdequacyDecision { country: String },

    /// Article 46 - Standard contractual clauses (SCCs)
    StandardContractualClauses { scc_version: String },

    /// Article 46 - Binding corporate rules (BCRs)
    BindingCorporateRules { bcr_id: String },

    /// Article 49 - Derogations for specific situations
    Derogation(Article49Derogation),
}

/// Article 49 derogations for international transfers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum Article49Derogation {
    /// Explicit consent after being informed of risks
    ExplicitConsent,

    /// Necessary for contract performance
    ContractPerformance,

    /// Important reasons of public interest
    PublicInterestReasons,

    /// Establishment, exercise or defense of legal claims
    LegalClaims,

    /// Protection of vital interests
    VitalInterests,

    /// Transfer from public register
    PublicRegister,

    /// Compelling legitimate interests (exceptional, last resort)
    CompellingLegitimateInterests,
}

/// Data subject rights under Chapter III
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum DataSubjectRight {
    /// Article 15 - Right of access
    Access,

    /// Article 16 - Right to rectification
    Rectification,

    /// Article 17 - Right to erasure ("right to be forgotten")
    Erasure,

    /// Article 18 - Right to restriction of processing
    RestrictionOfProcessing,

    /// Article 20 - Right to data portability
    DataPortability,

    /// Article 21 - Right to object to processing
    Object,

    /// Article 22 - Rights related to automated decision-making and profiling
    AutomatedDecisionMaking,
}

/// Consent quality indicators (Article 7 requirements)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ConsentQuality {
    /// Freely given (no coercion, imbalance of power considered)
    pub freely_given: bool,

    /// Specific (granular, not bundled)
    pub specific: bool,

    /// Informed (clear information provided)
    pub informed: bool,

    /// Unambiguous indication (affirmative action required)
    pub unambiguous: bool,

    /// Easily withdrawable (as easy as giving consent)
    pub easily_withdrawable: bool,
}

impl ConsentQuality {
    /// Check if consent meets all Article 7 requirements
    pub fn is_valid(&self) -> bool {
        self.freely_given
            && self.specific
            && self.informed
            && self.unambiguous
            && self.easily_withdrawable
    }
}

/// Data controller (Article 4(7))
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DataController {
    pub id: String,
    pub name: String,
    pub established_in_eu: bool,
    pub dpo_appointed: bool, // Data Protection Officer (Article 37)
}

/// Data processor (Article 4(8))
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DataProcessor {
    pub id: String,
    pub name: String,
    pub controller_id: String,
    pub processing_agreement: bool, // Article 28 requires written agreement
}

/// Data subject
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DataSubject {
    pub id: String,
    pub is_child: bool, // Under 16 years (or member state variation)
    pub consent_records: Vec<ConsentRecord>,
}

/// Consent record
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ConsentRecord {
    pub purpose: String,
    pub timestamp: DateTime<Utc>,
    pub freely_given: bool,
    pub specific: bool,
    pub informed: bool,
    pub unambiguous: bool,
}

impl ConsentRecord {
    /// Check if this consent record is valid according to Article 7
    pub fn is_valid(&self) -> bool {
        self.freely_given && self.specific && self.informed && self.unambiguous
    }
}

/// Data breach categories (for Articles 33/34)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum BreachCategory {
    /// Confidentiality breach (unauthorized access/disclosure)
    ConfidentialityBreach,

    /// Integrity breach (unauthorized alteration)
    IntegrityBreach,

    /// Availability breach (accidental/unlawful destruction or loss)
    AvailabilityBreach,
}

/// Severity assessment for data breaches
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum BreachSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance status
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant { violations: Vec<String> },
    RequiresAdditionalReview { reason: String },
}

impl ComplianceStatus {
    /// Check if the status is compliant
    pub fn is_compliant(&self) -> bool {
        matches!(self, Self::Compliant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_quality_validation() {
        let valid_consent = ConsentQuality {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
            easily_withdrawable: true,
        };
        assert!(valid_consent.is_valid());

        let invalid_consent = ConsentQuality {
            freely_given: false, // Coerced
            specific: true,
            informed: true,
            unambiguous: true,
            easily_withdrawable: true,
        };
        assert!(!invalid_consent.is_valid());
    }

    #[test]
    fn test_compliance_status() {
        let compliant = ComplianceStatus::Compliant;
        assert!(compliant.is_compliant());

        let non_compliant = ComplianceStatus::NonCompliant {
            violations: vec!["Missing lawful basis".to_string()],
        };
        assert!(!non_compliant.is_compliant());
    }

    #[test]
    fn test_consent_record_validation() {
        let record = ConsentRecord {
            purpose: "Marketing".to_string(),
            timestamp: Utc::now(),
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
        };
        assert!(record.is_valid());
    }
}
