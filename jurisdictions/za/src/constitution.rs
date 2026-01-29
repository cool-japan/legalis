//! Constitution of the Republic of South Africa, 1996
//!
//! The supreme law of South Africa, containing one of the world's most progressive
//! Bills of Rights. All law must be consistent with the Constitution.
//!
//! ## Key Features
//!
//! - Supremacy over all other law (s2)
//! - Bill of Rights (Chapter 2, s7-39)
//! - Constitutional Court as guardian
//! - Horizontal and vertical application of rights
//! - Limitation clause (s36)
//!
//! ## Structure
//!
//! 1. Founding Provisions
//! 2. Bill of Rights
//! 3. Cooperative Government
//! 4. Parliament
//! 5. The President and National Executive
//! 6. Provinces
//! 7. Local Government
//! 8. Courts and Administration of Justice
//! 9. State Institutions Supporting Constitutional Democracy
//! 10. Public Administration
//! 11. Security Services
//! 12. Traditional Leaders
//! 13. Finance
//! 14. General Provisions

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for constitutional operations
pub type ConstitutionalResult<T> = Result<T, ConstitutionalError>;

/// Bill of Rights guarantees (Chapter 2, s7-39)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BillOfRightsGuarantee {
    /// Equality (s9) - Non-discrimination on listed grounds
    Equality,
    /// Human dignity (s10)
    HumanDignity,
    /// Life (s11)
    Life,
    /// Freedom and security of person (s12)
    FreedomAndSecurity,
    /// Prohibition of slavery, servitude, forced labour (s13)
    SlaveryProhibition,
    /// Privacy (s14)
    Privacy,
    /// Freedom of religion, belief and opinion (s15)
    ReligionBeliefOpinion,
    /// Freedom of expression (s16)
    FreedomOfExpression,
    /// Assembly, demonstration, picket, petition (s17)
    AssemblyAndPetition,
    /// Freedom of association (s18)
    FreedomOfAssociation,
    /// Political rights (s19)
    PoliticalRights,
    /// Citizenship (s20)
    Citizenship,
    /// Freedom of movement and residence (s21)
    MovementAndResidence,
    /// Freedom of trade, occupation and profession (s22)
    TradeOccupationProfession,
    /// Labour relations (s23)
    LabourRelations,
    /// Environment (s24)
    Environment,
    /// Property (s25) - Including land reform
    Property,
    /// Housing (s26)
    Housing,
    /// Health care, food, water and social security (s27)
    HealthCareFoodWaterSocialSecurity,
    /// Children's rights (s28)
    ChildrensRights,
    /// Education (s29)
    Education,
    /// Language and culture (s30-31)
    LanguageAndCulture,
    /// Cultural, religious and linguistic communities (s31)
    CulturalCommunities,
    /// Access to information (s32)
    AccessToInformation,
    /// Just administrative action (s33)
    JustAdministrativeAction,
    /// Access to courts (s34)
    AccessToCourts,
    /// Arrested, detained and accused persons (s35)
    ArrestedDetainedAccused,
    /// Limitation of rights (s36)
    LimitationClause,
}

impl BillOfRightsGuarantee {
    /// Get section number in the Constitution
    pub fn section(&self) -> u8 {
        match self {
            Self::Equality => 9,
            Self::HumanDignity => 10,
            Self::Life => 11,
            Self::FreedomAndSecurity => 12,
            Self::SlaveryProhibition => 13,
            Self::Privacy => 14,
            Self::ReligionBeliefOpinion => 15,
            Self::FreedomOfExpression => 16,
            Self::AssemblyAndPetition => 17,
            Self::FreedomOfAssociation => 18,
            Self::PoliticalRights => 19,
            Self::Citizenship => 20,
            Self::MovementAndResidence => 21,
            Self::TradeOccupationProfession => 22,
            Self::LabourRelations => 23,
            Self::Environment => 24,
            Self::Property => 25,
            Self::Housing => 26,
            Self::HealthCareFoodWaterSocialSecurity => 27,
            Self::ChildrensRights => 28,
            Self::Education => 29,
            Self::LanguageAndCulture => 30,
            Self::CulturalCommunities => 31,
            Self::AccessToInformation => 32,
            Self::JustAdministrativeAction => 33,
            Self::AccessToCourts => 34,
            Self::ArrestedDetainedAccused => 35,
            Self::LimitationClause => 36,
        }
    }

    /// Check if right has horizontal application (applies to private parties)
    pub fn has_horizontal_application(&self) -> bool {
        // s8(2) - Bill of Rights binds natural and juristic persons
        // to extent applicable, taking into account nature of right
        matches!(
            self,
            Self::Equality
                | Self::HumanDignity
                | Self::Privacy
                | Self::FreedomOfExpression
                | Self::LabourRelations
                | Self::Environment
                | Self::Property
        )
    }

    /// Check if right is non-derogable (cannot be limited even in state of emergency)
    pub fn is_non_derogable(&self) -> bool {
        // s37 - Non-derogable rights in state of emergency
        matches!(
            self,
            Self::Equality
                | Self::HumanDignity
                | Self::Life
                | Self::SlaveryProhibition
                | Self::ArrestedDetainedAccused // Core fair trial rights
        )
    }
}

/// Prohibited grounds of discrimination (s9(3))
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiscriminationGround {
    /// Race
    Race,
    /// Gender
    Gender,
    /// Sex
    Sex,
    /// Pregnancy
    Pregnancy,
    /// Marital status
    MaritalStatus,
    /// Ethnic or social origin
    EthnicOrSocialOrigin,
    /// Colour
    Colour,
    /// Sexual orientation
    SexualOrientation,
    /// Age
    Age,
    /// Disability
    Disability,
    /// Religion
    Religion,
    /// Conscience
    Conscience,
    /// Belief
    Belief,
    /// Culture
    Culture,
    /// Language
    Language,
    /// Birth
    Birth,
}

/// Constitutional Court jurisdiction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstitutionalCourtJurisdiction {
    /// Constitutional matters
    ConstitutionalMatters,
    /// Disputes between organs of state
    InterGovernmentalDisputes,
    /// Certification of provincial constitutions
    ProvincialConstitutionCertification,
    /// Abstract constitutional review
    AbstractReview,
    /// Direct access on constitutional matters
    DirectAccess,
}

/// Founding values (s1)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoundingValue {
    /// Human dignity, equality and advancement of human rights and freedoms
    HumanDignityEqualityFreedom,
    /// Non-racialism and non-sexism
    NonRacialismNonSexism,
    /// Supremacy of constitution and rule of law
    ConstitutionalSupremacy,
    /// Universal adult suffrage
    UniversalSuffrage,
    /// Regular elections
    RegularElections,
    /// Multi-party system of democratic government
    MultiPartyDemocracy,
}

/// Limitation test (s36 - can rights be limited?)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitationTest {
    /// Law of general application
    pub law_of_general_application: bool,
    /// Reasonable limitation
    pub reasonable: bool,
    /// Justifiable in open and democratic society
    pub justifiable_in_democracy: bool,
    /// Based on human dignity, equality and freedom
    pub based_on_founding_values: bool,
    /// Nature of the right
    pub nature_of_right_considered: bool,
    /// Importance of purpose
    pub importance_of_purpose: bool,
    /// Nature and extent of limitation
    pub nature_and_extent_considered: bool,
    /// Relation between limitation and purpose
    pub relation_to_purpose: bool,
    /// Less restrictive means available
    pub less_restrictive_means_considered: bool,
}

impl LimitationTest {
    /// Apply the s36 limitation test
    pub fn is_valid_limitation(&self) -> bool {
        self.law_of_general_application
            && self.reasonable
            && self.justifiable_in_democracy
            && self.based_on_founding_values
    }

    /// Get all s36 factors
    pub fn all_factors_considered(&self) -> bool {
        self.nature_of_right_considered
            && self.importance_of_purpose
            && self.nature_and_extent_considered
            && self.relation_to_purpose
            && self.less_restrictive_means_considered
    }
}

/// State institutions supporting constitutional democracy (Chapter 9)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chapter9Institution {
    /// Public Protector (s181-182)
    PublicProtector,
    /// South African Human Rights Commission (s184)
    HumanRightsCommission,
    /// Commission for the Promotion and Protection of Rights of Cultural, Religious and Linguistic Communities (s185)
    CulturalCommission,
    /// Commission for Gender Equality (s187)
    GenderEqualityCommission,
    /// Auditor-General (s188)
    AuditorGeneral,
    /// Electoral Commission (s190)
    ElectoralCommission,
}

/// Constitutional errors
#[derive(Debug, Error)]
pub enum ConstitutionalError {
    /// Unconstitutional law or conduct
    #[error("Unconstitutional (violates s{section}): {description}")]
    Unconstitutional { section: u8, description: String },

    /// Rights violation
    #[error("Bill of Rights violation (s{section}): {right}")]
    RightsViolation { section: u8, right: String },

    /// Invalid limitation of rights
    #[error("Invalid limitation of rights (s36 test failed): {reason}")]
    InvalidLimitation { reason: String },

    /// Discrimination on prohibited ground
    #[error("Unfair discrimination on prohibited ground (s9): {ground}")]
    UnfairDiscrimination { ground: String },

    /// Separation of powers violation
    #[error("Separation of powers violation: {description}")]
    SeparationOfPowersViolation { description: String },

    /// Rule of law violation
    #[error("Rule of law violation (s1): {description}")]
    RuleOfLawViolation { description: String },
}

/// Validate if limitation of right is constitutional under s36
pub fn validate_limitation(test: &LimitationTest) -> ConstitutionalResult<()> {
    if !test.law_of_general_application {
        return Err(ConstitutionalError::InvalidLimitation {
            reason: "Not a law of general application".to_string(),
        });
    }

    if !test.reasonable {
        return Err(ConstitutionalError::InvalidLimitation {
            reason: "Limitation is not reasonable".to_string(),
        });
    }

    if !test.justifiable_in_democracy {
        return Err(ConstitutionalError::InvalidLimitation {
            reason: "Not justifiable in open and democratic society".to_string(),
        });
    }

    Ok(())
}

/// Get constitutional compliance checklist
pub fn get_constitutional_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Constitution is supreme law", "s2"),
        ("Law consistent with Constitution", "s2"),
        ("Bill of Rights applies to all law", "s8(1)"),
        ("Horizontal application where applicable", "s8(2)"),
        ("Rights limitation meets s36 test", "s36"),
        ("No unfair discrimination", "s9"),
        ("Human dignity respected", "s10"),
        ("Access to courts available", "s34"),
        ("Just administrative action", "s33"),
        ("Separation of powers maintained", "Doctrine"),
        ("Rule of law upheld", "s1"),
        ("Founding values respected", "s1"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bill_of_rights_sections() {
        assert_eq!(BillOfRightsGuarantee::Equality.section(), 9);
        assert_eq!(BillOfRightsGuarantee::Privacy.section(), 14);
        assert_eq!(BillOfRightsGuarantee::Property.section(), 25);
    }

    #[test]
    fn test_horizontal_application() {
        assert!(BillOfRightsGuarantee::Equality.has_horizontal_application());
        assert!(BillOfRightsGuarantee::Privacy.has_horizontal_application());
        assert!(!BillOfRightsGuarantee::PoliticalRights.has_horizontal_application());
    }

    #[test]
    fn test_non_derogable_rights() {
        assert!(BillOfRightsGuarantee::HumanDignity.is_non_derogable());
        assert!(BillOfRightsGuarantee::Life.is_non_derogable());
        assert!(!BillOfRightsGuarantee::FreedomOfExpression.is_non_derogable());
    }

    #[test]
    fn test_limitation_valid() {
        let test = LimitationTest {
            law_of_general_application: true,
            reasonable: true,
            justifiable_in_democracy: true,
            based_on_founding_values: true,
            nature_of_right_considered: true,
            importance_of_purpose: true,
            nature_and_extent_considered: true,
            relation_to_purpose: true,
            less_restrictive_means_considered: true,
        };

        assert!(test.is_valid_limitation());
        assert!(test.all_factors_considered());
        assert!(validate_limitation(&test).is_ok());
    }

    #[test]
    fn test_limitation_invalid_not_general() {
        let test = LimitationTest {
            law_of_general_application: false,
            reasonable: true,
            justifiable_in_democracy: true,
            based_on_founding_values: true,
            nature_of_right_considered: true,
            importance_of_purpose: true,
            nature_and_extent_considered: true,
            relation_to_purpose: true,
            less_restrictive_means_considered: true,
        };

        assert!(!test.is_valid_limitation());
        assert!(validate_limitation(&test).is_err());
    }

    #[test]
    fn test_limitation_invalid_not_reasonable() {
        let test = LimitationTest {
            law_of_general_application: true,
            reasonable: false,
            justifiable_in_democracy: true,
            based_on_founding_values: true,
            nature_of_right_considered: true,
            importance_of_purpose: true,
            nature_and_extent_considered: true,
            relation_to_purpose: true,
            less_restrictive_means_considered: true,
        };

        assert!(!test.is_valid_limitation());
        assert!(validate_limitation(&test).is_err());
    }

    #[test]
    fn test_constitutional_checklist() {
        let checklist = get_constitutional_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
