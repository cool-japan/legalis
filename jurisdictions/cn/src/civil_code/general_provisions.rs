//! Book I: General Provisions (总则编)
//!
//! Articles 1-204 of the Civil Code
//!
//! Covers:
//! - Basic principles
//! - Natural persons
//! - Legal persons and unincorporated organizations
//! - Civil rights
//! - Civil juristic acts
//! - Agency
//! - Civil liability
//! - Limitation periods

use crate::i18n::BilingualText;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Legal capacity of natural persons (民事行为能力)
///
/// Article 17-24
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalCapacity {
    /// Full capacity (完全民事行为能力) - Age 18+
    Full,
    /// Limited capacity (限制民事行为能力) - Age 8-17
    Limited,
    /// No capacity (无民事行为能力) - Age 0-7
    None,
}

impl LegalCapacity {
    /// Determine legal capacity based on age
    ///
    /// Article 17-20
    pub fn from_age(age: u8, has_independent_income: bool) -> Self {
        match age {
            0..=7 => Self::None,
            8..=15 => Self::Limited,
            16..=17 => {
                if has_independent_income {
                    Self::Full // Article 18, para 2
                } else {
                    Self::Limited
                }
            }
            _ => Self::Full,
        }
    }

    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Full => BilingualText::new("完全民事行为能力", "Full legal capacity"),
            Self::Limited => BilingualText::new("限制民事行为能力", "Limited legal capacity"),
            Self::None => BilingualText::new("无民事行为能力", "No legal capacity"),
        }
    }
}

/// Natural person (自然人)
///
/// Articles 13-69
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalPerson {
    /// Name (姓名)
    pub name: String,
    /// Age
    pub age: u8,
    /// Has independent income source
    pub has_independent_income: bool,
    /// Is mentally competent
    pub is_mentally_competent: bool,
    /// Domicile (住所)
    pub domicile: Option<String>,
}

impl NaturalPerson {
    /// Create a new natural person
    pub fn new(name: impl Into<String>, age: u8) -> Self {
        Self {
            name: name.into(),
            age,
            has_independent_income: false,
            is_mentally_competent: true,
            domicile: None,
        }
    }

    /// Get legal capacity (Article 17-24)
    pub fn legal_capacity(&self) -> LegalCapacity {
        if !self.is_mentally_competent {
            // Article 21-24: Mental incompetence
            return LegalCapacity::None;
        }
        LegalCapacity::from_age(self.age, self.has_independent_income)
    }

    /// Check if person can perform juristic act independently
    pub fn can_perform_act_independently(&self, act: &JuristicAct) -> bool {
        match self.legal_capacity() {
            LegalCapacity::Full => true,
            LegalCapacity::Limited => {
                // Article 19: Limited capacity persons can perform acts appropriate to their age and intellect
                matches!(
                    act.nature,
                    ActNature::PurelyBeneficial | ActNature::AppropriateToAge
                )
            }
            LegalCapacity::None => false,
        }
    }
}

/// Legal person type (法人类型)
///
/// Articles 57-92
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalPersonType {
    /// For-profit legal person (营利法人) - Article 76
    ForProfit,
    /// Non-profit legal person (非营利法人) - Article 87
    NonProfit,
    /// Special legal person (特别法人) - Article 96
    Special,
}

/// Legal person (法人)
///
/// Article 57: Organizations with capacity for civil rights and civil conduct,
/// independently enjoying civil rights and bearing civil obligations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalPerson {
    /// Name (名称)
    pub name: String,
    /// Type
    pub legal_person_type: LegalPersonType,
    /// Domicile (住所)
    pub domicile: String,
    /// Legal representative (法定代表人)
    pub legal_representative: Option<String>,
    /// Registered capital
    pub registered_capital: Option<f64>,
}

/// Unincorporated organization (非法人组织)
///
/// Articles 102-108
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnincorporatedOrganization {
    /// Name (名称)
    pub name: String,
    /// Principal place of business (主要营业地)
    pub principal_place: String,
    /// Representative (负责人)
    pub representative: String,
}

/// Nature of juristic act (法律行为性质)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActNature {
    /// Purely beneficial (纯获利益) - e.g., accepting gifts
    PurelyBeneficial,
    /// Appropriate to age and intellect (与其年龄、智力相适应)
    AppropriateToAge,
    /// Major transaction (重大交易)
    MajorTransaction,
}

/// Juristic act (民事法律行为)
///
/// Articles 133-157
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JuristicAct {
    /// Description
    pub description: BilingualText,
    /// Nature of the act
    pub nature: ActNature,
    /// Performed by
    pub actor: String,
    /// Timestamp
    pub performed_at: DateTime<Utc>,
    /// Requires guardian consent (for limited capacity persons)
    pub requires_guardian_consent: bool,
}

/// Validity of juristic act (法律行为效力)
///
/// Articles 143-157
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActValidity {
    /// Valid (有效)
    Valid,
    /// Void (无效) - Article 144-146
    Void,
    /// Voidable (可撤销) - Article 147-151
    Voidable,
    /// Effective upon ratification (效力待定) - Article 145
    EffectivePendingRatification,
}

impl ActValidity {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Valid => BilingualText::new("有效", "Valid"),
            Self::Void => BilingualText::new("无效", "Void"),
            Self::Voidable => BilingualText::new("可撤销", "Voidable"),
            Self::EffectivePendingRatification => {
                BilingualText::new("效力待定", "Effective pending ratification")
            }
        }
    }
}

/// Agency type (代理类型)
///
/// Articles 161-174
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgencyType {
    /// Commissioned agency (委托代理)
    Commissioned,
    /// Statutory agency (法定代理)
    Statutory,
}

/// Agency relationship (代理关系)
///
/// Article 161: An agent performs juristic acts in the name of the principal,
/// and the legal effects belong to the principal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agency {
    /// Principal (被代理人)
    pub principal: String,
    /// Agent (代理人)
    pub agent: String,
    /// Type of agency
    pub agency_type: AgencyType,
    /// Scope of authority (代理权限)
    pub scope: BilingualText,
    /// Valid from
    pub valid_from: DateTime<Utc>,
    /// Valid until
    pub valid_until: Option<DateTime<Utc>>,
}

/// Limitation period (诉讼时效)
///
/// Articles 188-199
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitationPeriod {
    /// Claim description
    pub claim: BilingualText,
    /// Date when claimant knew or should have known of harm and tortfeasor
    pub knowledge_date: DateTime<Utc>,
    /// Date of occurrence
    pub occurrence_date: DateTime<Utc>,
    /// Special period length (if not 3 years)
    pub special_period_years: Option<u32>,
}

impl LimitationPeriod {
    /// Get the limitation period length
    ///
    /// Article 188: General period is 3 years
    pub fn period_years(&self) -> u32 {
        self.special_period_years.unwrap_or(3)
    }

    /// Get expiration date of limitation period
    ///
    /// Article 188
    pub fn expiration_date(&self) -> DateTime<Utc> {
        self.knowledge_date + Duration::days(365 * i64::from(self.period_years()))
    }

    /// Get maximum expiration date (20 years from occurrence)
    ///
    /// Article 188, para 2
    pub fn maximum_expiration_date(&self) -> DateTime<Utc> {
        self.occurrence_date + Duration::days(365 * 20)
    }

    /// Check if claim is time-barred
    pub fn is_time_barred(&self, current_date: DateTime<Utc>) -> bool {
        current_date > self.expiration_date() || current_date > self.maximum_expiration_date()
    }
}

// ============================================================================
// Validators
// ============================================================================

/// Validate a juristic act
///
/// Articles 143-157
pub fn validate_juristic_act(
    act: &JuristicAct,
    actor: &NaturalPerson,
) -> Result<ActValidity, GeneralProvisionsError> {
    // Article 143: Requirements for valid juristic act
    // 1. Actor has corresponding legal capacity
    // 2. Expression of true intent
    // 3. Does not violate mandatory provisions of law or public policy

    let capacity = actor.legal_capacity();

    match capacity {
        LegalCapacity::Full => Ok(ActValidity::Valid),
        LegalCapacity::Limited => {
            // Article 19, 145
            if actor.can_perform_act_independently(act) {
                Ok(ActValidity::Valid)
            } else if act.requires_guardian_consent {
                // Requires guardian consent, pending ratification
                Ok(ActValidity::EffectivePendingRatification)
            } else {
                Err(GeneralProvisionsError::InsufficientCapacity {
                    required: BilingualText::new("完全民事行为能力", "Full legal capacity"),
                    actual: capacity.description(),
                })
            }
        }
        LegalCapacity::None => {
            // Article 144: Acts by persons with no capacity are void
            // Exception: Article 20 - purely beneficial acts or appropriate to age
            if matches!(
                act.nature,
                ActNature::PurelyBeneficial | ActNature::AppropriateToAge
            ) {
                Ok(ActValidity::Valid)
            } else {
                Ok(ActValidity::Void)
            }
        }
    }
}

/// Validate agency authority
///
/// Articles 161-174
pub fn validate_agency(
    agency: &Agency,
    current_date: DateTime<Utc>,
) -> Result<(), GeneralProvisionsError> {
    // Check if agency is still valid
    if current_date < agency.valid_from {
        return Err(GeneralProvisionsError::AgencyNotYetEffective {
            effective_date: agency.valid_from,
        });
    }

    if let Some(valid_until) = agency.valid_until
        && current_date > valid_until
    {
        return Err(GeneralProvisionsError::AgencyExpired {
            expired_date: valid_until,
        });
    }

    Ok(())
}

/// Check if a claim is within limitation period
///
/// Articles 188-199
pub fn check_limitation_period(
    period: &LimitationPeriod,
    current_date: DateTime<Utc>,
) -> Result<(), GeneralProvisionsError> {
    if period.is_time_barred(current_date) {
        Err(GeneralProvisionsError::LimitationExpired {
            claim: period.claim.clone(),
            expiration: period.expiration_date(),
            maximum_expiration: period.maximum_expiration_date(),
        })
    } else {
        Ok(())
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for General Provisions
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum GeneralProvisionsError {
    /// Insufficient legal capacity
    #[error("Insufficient legal capacity: required {required}, actual {actual}")]
    InsufficientCapacity {
        /// Required capacity
        required: BilingualText,
        /// Actual capacity
        actual: BilingualText,
    },

    /// Agency not yet effective
    #[error("Agency not yet effective until {effective_date}")]
    AgencyNotYetEffective {
        /// Effective date
        effective_date: DateTime<Utc>,
    },

    /// Agency expired
    #[error("Agency expired on {expired_date}")]
    AgencyExpired {
        /// Expiration date
        expired_date: DateTime<Utc>,
    },

    /// Limitation period expired
    #[error("Limitation period expired for claim: {claim}")]
    LimitationExpired {
        /// Claim description
        claim: BilingualText,
        /// Expiration date
        expiration: DateTime<Utc>,
        /// Maximum expiration (20 years)
        maximum_expiration: DateTime<Utc>,
    },
}

/// Result type for General Provisions operations
pub type GeneralProvisionsResult<T> = Result<T, GeneralProvisionsError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_capacity_from_age() {
        assert_eq!(LegalCapacity::from_age(5, false), LegalCapacity::None);
        assert_eq!(LegalCapacity::from_age(10, false), LegalCapacity::Limited);
        assert_eq!(LegalCapacity::from_age(16, false), LegalCapacity::Limited);
        assert_eq!(LegalCapacity::from_age(16, true), LegalCapacity::Full); // With independent income
        assert_eq!(LegalCapacity::from_age(20, false), LegalCapacity::Full);
    }

    #[test]
    fn test_natural_person_capacity() {
        let person = NaturalPerson::new("张三", 25);
        assert_eq!(person.legal_capacity(), LegalCapacity::Full);

        let child = NaturalPerson::new("李四", 6);
        assert_eq!(child.legal_capacity(), LegalCapacity::None);

        let teen = NaturalPerson::new("王五", 15);
        assert_eq!(teen.legal_capacity(), LegalCapacity::Limited);
    }

    #[test]
    fn test_limitation_period() {
        let knowledge_date = Utc::now();
        let occurrence_date = knowledge_date - Duration::days(100);

        let period = LimitationPeriod {
            claim: BilingualText::new("侵权赔偿", "Tort compensation"),
            knowledge_date,
            occurrence_date,
            special_period_years: None,
        };

        assert_eq!(period.period_years(), 3);
        assert!(!period.is_time_barred(Utc::now()));

        // Test with date 4 years in future
        let future_date = knowledge_date + Duration::days(365 * 4);
        assert!(period.is_time_barred(future_date));
    }

    #[test]
    fn test_act_validity_full_capacity() {
        let person = NaturalPerson::new("成年人", 30);
        let act = JuristicAct {
            description: BilingualText::new("购买合同", "Purchase contract"),
            nature: ActNature::MajorTransaction,
            actor: "成年人".to_string(),
            performed_at: Utc::now(),
            requires_guardian_consent: false,
        };

        let result = validate_juristic_act(&act, &person);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ActValidity::Valid);
    }

    #[test]
    fn test_act_validity_no_capacity() {
        let child = NaturalPerson::new("儿童", 5);
        let act = JuristicAct {
            description: BilingualText::new("购买合同", "Purchase contract"),
            nature: ActNature::MajorTransaction,
            actor: "儿童".to_string(),
            performed_at: Utc::now(),
            requires_guardian_consent: false,
        };

        let result = validate_juristic_act(&act, &child);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ActValidity::Void);
    }

    #[test]
    fn test_act_validity_limited_capacity() {
        let teen = NaturalPerson::new("少年", 14);
        let act = JuristicAct {
            description: BilingualText::new("接受礼物", "Accept gift"),
            nature: ActNature::PurelyBeneficial,
            actor: "少年".to_string(),
            performed_at: Utc::now(),
            requires_guardian_consent: false,
        };

        let result = validate_juristic_act(&act, &teen);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ActValidity::Valid);
    }

    #[test]
    fn test_agency_validation() {
        let now = Utc::now();
        let agency = Agency {
            principal: "委托人".to_string(),
            agent: "代理人".to_string(),
            agency_type: AgencyType::Commissioned,
            scope: BilingualText::new("代理签订合同", "Authority to sign contracts"),
            valid_from: now - Duration::days(10),
            valid_until: Some(now + Duration::days(30)),
        };

        assert!(validate_agency(&agency, now).is_ok());

        // Test expired agency
        let future = now + Duration::days(60);
        assert!(validate_agency(&agency, future).is_err());
    }
}
