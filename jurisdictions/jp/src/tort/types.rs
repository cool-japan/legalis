//! Core types for Japanese tort law (不法行為法)
//!
//! This module defines the fundamental types used in Article 709 tort claims.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Intent or negligence (故意・過失)
///
/// Article 709 requires either intentional conduct or negligence.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Intent {
    /// Intentional conduct (故意)
    Intentional { age: u8 },

    /// Negligence (過失) - breach of duty of care
    Negligence,

    /// Negligence with specific duty of care (注意義務違反)
    NegligenceWithDuty { duty_of_care: String },
}

/// Protected interest under Article 709 (保護法益)
///
/// Rights or legally protected interests that can be infringed.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProtectedInterest<'a> {
    /// Property rights (財産権)
    Property(&'a str),

    /// Body and health (身体・健康) - triggers Article 710
    BodyAndHealth,

    /// Liberty (自由)
    Liberty,

    /// Privacy (プライバシー)
    Privacy,

    /// Reputation (名誉)
    Reputation,

    /// Other legally protected interest
    Other(&'a str),
}

/// Damage (損害)
///
/// Actual damages that must be proven and compensated.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Damage {
    /// Monetary amount (金額)
    pub amount: u64,

    /// Description of damage (損害の内容)
    pub description: String,

    /// Type of damage
    pub damage_type: DamageType,
}

impl Damage {
    /// Create a new damage claim
    pub fn new(amount: u64, description: impl Into<String>) -> Self {
        Self {
            amount,
            description: description.into(),
            damage_type: DamageType::Property,
        }
    }

    /// Set damage type
    pub fn with_type(mut self, damage_type: DamageType) -> Self {
        self.damage_type = damage_type;
        self
    }
}

/// Type of damage (損害の種類)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DamageType {
    /// Property damage (財産的損害)
    Property,

    /// Non-pecuniary damage (非財産的損害) - Article 710
    NonPecuniary,

    /// Medical expenses (治療費)
    Medical,

    /// Lost wages (休業損害)
    LostWages,

    /// Pain and suffering (慰謝料)
    PainAndSuffering,
}

/// Causal link between act and damage (因果関係)
///
/// The causal connection required under Article 709.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CausalLink<'a> {
    /// Direct causation (直接因果関係)
    Direct,

    /// Adequate causation (相当因果関係)
    Adequate(&'a str),

    /// Conditional causation with explanation
    Conditional {
        condition: &'a str,
        explanation: &'a str,
    },
}

/// Negligence standard (過失の基準)
///
/// Standard of care for negligence assessment.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Negligence {
    /// Ordinary negligence (通常の過失)
    Ordinary,

    /// Gross negligence (重過失)
    Gross,

    /// Professional negligence (専門職の過失)
    Professional { profession: String },
}

/// Non-pecuniary damage type for Article 710 (非財産的損害の種類)
///
/// Types of emotional/non-pecuniary harm recognized under Article 710.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NonPecuniaryDamageType {
    /// Pain and suffering from bodily injury (身体的損害の慰謝料)
    BodyAndHealth,

    /// Damage to reputation/honor (名誉毀損の慰謝料)
    ReputationDamage,

    /// Infringement of liberty/freedom (自由侵害の慰謝料)
    LibertyInfringement,

    /// Emotional distress from property loss (財産侵害に伴う精神的損害)
    PropertyRelatedDistress,
}

/// Severity of harm for compensation calculation (被害の程度)
///
/// Used in Article 710 to categorize the degree of non-pecuniary harm.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HarmSeverity {
    /// Minor harm (軽度)
    Minor,

    /// Moderate harm (中度)
    Moderate,

    /// Severe harm (重度)
    Severe,

    /// Catastrophic harm (最重度)
    Catastrophic,
}

/// Employment type for Article 715 (雇用形態)
///
/// Categories of employment relationships relevant to vicarious liability.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EmploymentType {
    /// Regular full-time employee (正社員)
    FullTime,

    /// Part-time employee (パート・アルバイト)
    PartTime,

    /// Contract employee (契約社員)
    Contract,

    /// Temporary dispatch worker (派遣労働者)
    Dispatch,

    /// Independent contractor (請負人) - typically excluded from Article 715
    Independent,

    /// Agent/Representative (代理人)
    Agent,
}

/// Employment relationship for Article 715 (使用関係)
///
/// Describes the employer-employee relationship necessary for vicarious liability.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmploymentRelationship<'a> {
    /// Employer name (使用者名)
    pub employer_name: &'a str,

    /// Employee name (被用者名)
    pub employee_name: &'a str,

    /// Type of employment (雇用形態)
    pub employment_type: EmploymentType,

    /// Duration of relationship (関係期間) - optional
    pub relationship_duration: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_creation() {
        let damage = Damage::new(500_000, "修理費");
        assert_eq!(damage.amount, 500_000);
        assert_eq!(damage.description, "修理費");
    }

    #[test]
    fn test_intent_variants() {
        let intentional = Intent::Intentional { age: 20 };
        let negligence = Intent::Negligence;

        assert!(matches!(intentional, Intent::Intentional { .. }));
        assert!(matches!(negligence, Intent::Negligence));
    }
}
