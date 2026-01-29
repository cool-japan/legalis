//! Book V: Marriage and Family (婚姻家庭编)
//!
//! Articles 1040-1118 of the Civil Code
//!
//! Covers:
//! - Marriage
//! - Parent-child relationships
//! - Adoption
//! - Support obligations

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Marriage requirements (结婚条件)
///
/// Articles 1046-1051
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarriageRequirements {
    /// Minimum age for men (22)
    pub min_age_men: u8,
    /// Minimum age for women (20)
    pub min_age_women: u8,
    /// Voluntary (自愿)
    pub voluntary: bool,
    /// Monogamy (一夫一妻)
    pub monogamy: bool,
}

impl Default for MarriageRequirements {
    fn default() -> Self {
        Self {
            min_age_men: 22,   // Article 1047
            min_age_women: 20, // Article 1047
            voluntary: true,   // Article 1046
            monogamy: true,    // Article 1041
        }
    }
}

/// Marriage (婚姻)
///
/// Articles 1046-1065
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marriage {
    /// Husband
    pub husband: String,
    /// Wife
    pub wife: String,
    /// Registration date
    pub registration_date: DateTime<Utc>,
    /// Is registered
    pub is_registered: bool,
    /// Marital property regime
    pub property_regime: MaritalPropertyRegime,
}

impl Marriage {
    /// Check if marriage is valid
    ///
    /// Article 1049: Marriage takes effect upon registration
    pub fn is_valid(&self) -> bool {
        self.is_registered
    }
}

/// Marital property regime (夫妻财产制)
///
/// Articles 1062-1066
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaritalPropertyRegime {
    /// Joint property (共同财产) - Default
    CommunityProperty,
    /// Separate property (分别财产)
    SeparateProperty,
    /// Partial community (部分共同)
    PartialCommunity,
}

impl MaritalPropertyRegime {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::CommunityProperty => BilingualText::new("夫妻共同财产", "Community property"),
            Self::SeparateProperty => BilingualText::new("分别财产", "Separate property"),
            Self::PartialCommunity => BilingualText::new("部分共同财产", "Partial community"),
        }
    }
}

/// Divorce type (离婚类型)
///
/// Articles 1076-1092
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceType {
    /// Divorce by mutual consent (协议离婚) - Article 1076
    MutualConsent,
    /// Divorce by litigation (诉讼离婚) - Article 1079
    Litigation,
}

impl DivorceType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::MutualConsent => BilingualText::new("协议离婚", "Divorce by mutual consent"),
            Self::Litigation => BilingualText::new("诉讼离婚", "Divorce by litigation"),
        }
    }
}

/// Grounds for divorce by litigation (诉讼离婚事由)
///
/// Article 1079
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceGrounds {
    /// Bigamy or cohabitation with another (重婚或与他人同居)
    BigamyOrCohabitation,
    /// Domestic violence (家庭暴力)
    DomesticViolence,
    /// Abuse or abandonment (虐待、遗弃)
    AbuseOrAbandonment,
    /// Gambling, drug addiction (赌博、吸毒等恶习屡教不改)
    GamblingOrDrugAddiction,
    /// Separation for 2+ years (因感情不和分居满二年)
    SeparationTwoYears,
    /// Other major relationship breakdown (其他导致夫妻感情破裂的情形)
    OtherBreakdown,
}

impl DivorceGrounds {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::BigamyOrCohabitation => {
                BilingualText::new("重婚或与他人同居", "Bigamy or cohabitation")
            }
            Self::DomesticViolence => BilingualText::new("家庭暴力", "Domestic violence"),
            Self::AbuseOrAbandonment => BilingualText::new("虐待、遗弃", "Abuse or abandonment"),
            Self::GamblingOrDrugAddiction => {
                BilingualText::new("赌博、吸毒等恶习", "Gambling, drug addiction")
            }
            Self::SeparationTwoYears => {
                BilingualText::new("因感情不和分居满二年", "Separation for 2+ years")
            }
            Self::OtherBreakdown => {
                BilingualText::new("其他导致夫妻感情破裂", "Other relationship breakdown")
            }
        }
    }
}

/// Parent-child relationship (亲子关系)
///
/// Articles 1067-1075
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentChildRelationship {
    /// Parent
    pub parent: String,
    /// Child
    pub child: String,
    /// Is biological relationship
    pub is_biological: bool,
    /// Is adoptive relationship
    pub is_adoptive: bool,
    /// Support obligation
    pub support_obligation: SupportObligation,
}

/// Support obligation (扶养义务)
///
/// Articles 1067, 1074
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportObligation {
    /// Parents support minor children (父母抚养未成年子女)
    ParentsToMinorChildren,
    /// Adult children support parents (成年子女赡养父母)
    AdultChildrenToParents,
    /// Mutual support between spouses (夫妻互相扶养)
    MutualBetweenSpouses,
}

impl SupportObligation {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::ParentsToMinorChildren => {
                BilingualText::new("父母抚养未成年子女", "Parents support minor children")
            }
            Self::AdultChildrenToParents => {
                BilingualText::new("成年子女赡养父母", "Adult children support parents")
            }
            Self::MutualBetweenSpouses => {
                BilingualText::new("夫妻互相扶养", "Mutual support between spouses")
            }
        }
    }
}

/// Adoption (收养)
///
/// Articles 1093-1118
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adoption {
    /// Adopter (收养人)
    pub adopter: String,
    /// Adopted child (被收养人)
    pub adopted_child: String,
    /// Adoption date
    pub adoption_date: DateTime<Utc>,
    /// Is registered
    pub is_registered: bool,
    /// Consent from biological parents obtained
    pub biological_parent_consent: bool,
}

impl Adoption {
    /// Check if adoption is valid
    ///
    /// Article 1105: Adoption takes effect upon registration
    pub fn is_valid(&self) -> bool {
        self.is_registered
    }
}

// ============================================================================
// Validators
// ============================================================================

/// Validate marriage eligibility
///
/// Articles 1046-1051
pub fn validate_marriage_eligibility(
    husband_age: u8,
    wife_age: u8,
    voluntary: bool,
    either_already_married: bool,
) -> Result<(), MarriageFamilyError> {
    let req = MarriageRequirements::default();

    // Article 1047: Minimum age
    if husband_age < req.min_age_men {
        return Err(MarriageFamilyError::BelowMinimumAge {
            person: BilingualText::new("男方", "Husband"),
            age: husband_age,
            minimum: req.min_age_men,
        });
    }

    if wife_age < req.min_age_women {
        return Err(MarriageFamilyError::BelowMinimumAge {
            person: BilingualText::new("女方", "Wife"),
            age: wife_age,
            minimum: req.min_age_women,
        });
    }

    // Article 1046: Voluntary
    if !voluntary {
        return Err(MarriageFamilyError::NotVoluntary);
    }

    // Article 1041, 1051: Monogamy - cannot marry if already married
    if either_already_married {
        return Err(MarriageFamilyError::AlreadyMarried);
    }

    Ok(())
}

/// Validate divorce by mutual consent
///
/// Article 1076-1078
pub fn validate_divorce_by_mutual_consent(
    has_divorce_agreement: bool,
    cooling_off_period_completed: bool,
) -> Result<(), MarriageFamilyError> {
    // Article 1076: Must have divorce agreement
    if !has_divorce_agreement {
        return Err(MarriageFamilyError::NoDivorceAgreement);
    }

    // Article 1077: 30-day cooling-off period
    if !cooling_off_period_completed {
        return Err(MarriageFamilyError::CoolingOffPeriodNotCompleted);
    }

    Ok(())
}

/// Validate adoption
///
/// Articles 1093-1105
pub fn validate_adoption(adoption: &Adoption) -> Result<(), MarriageFamilyError> {
    // Article 1105: Adoption takes effect upon registration
    if !adoption.is_registered {
        return Err(MarriageFamilyError::AdoptionNotRegistered {
            adopter: adoption.adopter.clone(),
            child: adoption.adopted_child.clone(),
        });
    }

    // Article 1097: Biological parent consent generally required
    // (with exceptions for orphans, abandoned children, etc.)
    if !adoption.biological_parent_consent {
        // This might be acceptable in certain cases (orphans, etc.)
        // but flag as potential issue
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for Marriage and Family
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum MarriageFamilyError {
    /// Below minimum marriage age
    #[error("{person} is below minimum marriage age: {age} (minimum: {minimum})")]
    BelowMinimumAge {
        /// Person
        person: BilingualText,
        /// Age
        age: u8,
        /// Minimum age
        minimum: u8,
    },

    /// Marriage not voluntary
    #[error("Marriage must be voluntary")]
    NotVoluntary,

    /// Already married
    #[error("Cannot marry - already married (monogamy required)")]
    AlreadyMarried,

    /// No divorce agreement
    #[error("Divorce by mutual consent requires divorce agreement")]
    NoDivorceAgreement,

    /// Cooling-off period not completed
    #[error("30-day cooling-off period not completed")]
    CoolingOffPeriodNotCompleted,

    /// Adoption not registered
    #[error("Adoption not registered: adopter {adopter}, child {child}")]
    AdoptionNotRegistered {
        /// Adopter
        adopter: String,
        /// Child
        child: String,
    },

    /// Support obligation violation
    #[error("Support obligation violated: {obligation}")]
    SupportObligationViolation {
        /// Obligation type
        obligation: BilingualText,
    },
}

/// Result type for Marriage and Family operations
pub type MarriageFamilyResult<T> = Result<T, MarriageFamilyError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marriage_eligibility_valid() {
        let result = validate_marriage_eligibility(25, 23, true, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_marriage_eligibility_husband_too_young() {
        let result = validate_marriage_eligibility(20, 23, true, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_marriage_eligibility_wife_too_young() {
        let result = validate_marriage_eligibility(25, 18, true, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_marriage_eligibility_not_voluntary() {
        let result = validate_marriage_eligibility(25, 23, false, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_marriage_eligibility_already_married() {
        let result = validate_marriage_eligibility(25, 23, true, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_divorce_by_mutual_consent_valid() {
        let result = validate_divorce_by_mutual_consent(true, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_divorce_cooling_off_period() {
        let result = validate_divorce_by_mutual_consent(true, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_marriage_validity() {
        let marriage = Marriage {
            husband: "张三".to_string(),
            wife: "李四".to_string(),
            registration_date: Utc::now(),
            is_registered: true,
            property_regime: MaritalPropertyRegime::CommunityProperty,
        };

        assert!(marriage.is_valid());
    }

    #[test]
    fn test_adoption_validity() {
        let adoption = Adoption {
            adopter: "收养人".to_string(),
            adopted_child: "被收养人".to_string(),
            adoption_date: Utc::now(),
            is_registered: true,
            biological_parent_consent: true,
        };

        assert!(adoption.is_valid());
        assert!(validate_adoption(&adoption).is_ok());
    }
}
