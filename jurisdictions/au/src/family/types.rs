//! Family Law Types
//!
//! Types for Australian family law analysis under Family Law Act 1975 (Cth).

use serde::{Deserialize, Serialize};

// ============================================================================
// Marriage and Relationships
// ============================================================================

/// Type of relationship
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Marriage
    Marriage,
    /// De facto relationship
    DeFacto,
    /// Registered relationship
    Registered,
}

/// Grounds for divorce (s.48 - irretrievable breakdown)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivorceGrounds {
    /// Separated for 12 months
    pub separated_12_months: bool,
    /// Reasonable likelihood of cohabitation
    pub reasonable_likelihood_cohabitation: bool,
}

// ============================================================================
// Parenting
// ============================================================================

/// Parenting order type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentingOrderType {
    /// Who child lives with
    LivesWith,
    /// Time spent with other parent
    TimeSpentWith,
    /// Parental responsibility
    ParentalResponsibility,
    /// Communication with child
    Communication,
    /// Specific issues (education, medical, etc.)
    SpecificIssues,
}

/// Parental responsibility allocation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentalResponsibility {
    /// Equal shared parental responsibility
    EqualShared,
    /// Sole parental responsibility
    Sole,
    /// Shared with specific allocation
    SharedSpecific,
}

/// Best interests primary consideration (s.60CC(2))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimaryConsideration {
    /// Benefit of meaningful relationship with both parents
    MeaningfulRelationship,
    /// Need to protect from harm (family violence, abuse, neglect)
    ProtectionFromHarm,
}

/// Best interests additional considerations (s.60CC(3))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdditionalConsideration {
    /// Views of child (s.60CC(3)(a))
    ChildViews,
    /// Nature of relationship with parents (s.60CC(3)(b))
    RelationshipWithParents,
    /// Willingness to facilitate relationship (s.60CC(3)(c))
    WillingnessToFacilitate,
    /// Effect of change (s.60CC(3)(d))
    EffectOfChange,
    /// Practical difficulty of time arrangements (s.60CC(3)(e))
    PracticalDifficulty,
    /// Capacity of parents (s.60CC(3)(f))
    ParentalCapacity,
    /// Aboriginal/Torres Strait Islander heritage (s.60CC(3)(h))
    IndigenousHeritage,
    /// Family violence (s.60CC(3)(j))
    FamilyViolence,
}

/// Time arrangement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeArrangement {
    /// Equal time (50/50)
    EqualTime,
    /// Substantial and significant time
    SubstantialSignificant,
    /// Alternate weekends
    AlternateWeekends,
    /// Weekend and half holidays
    WeekendHalfHolidays,
    /// Supervised time
    Supervised,
    /// No time
    NoTime,
}

// ============================================================================
// Property
// ============================================================================

/// Property adjustment approach (s.79)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyApproach {
    /// Stanford v Stanford (2012) - No presumption of equal division
    Stanford,
    /// Four step approach
    FourStep,
}

/// Contribution type (s.79(4))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContributionType {
    /// Financial contribution to acquisition
    FinancialAcquisition,
    /// Financial contribution to conservation
    FinancialConservation,
    /// Non-financial contribution to acquisition
    NonFinancialAcquisition,
    /// Contribution to welfare of family (homemaker/parent)
    WelfareContribution,
}

/// Future needs factor (s.75(2))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FutureNeedsFactor {
    /// Age and health
    AgeHealth,
    /// Income and earning capacity
    IncomeCapacity,
    /// Care of children
    CareOfChildren,
    /// Commitments to support others
    SupportCommitments,
    /// Financial circumstances
    FinancialCircumstances,
    /// Duration of marriage
    DurationOfMarriage,
    /// Need for further education/training
    EducationTraining,
    /// Standard of living
    StandardOfLiving,
}

// ============================================================================
// Child Support
// ============================================================================

/// Child support formula element
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildSupportElement {
    /// Combined child support income
    CombinedIncome,
    /// Percentage of care
    CarePercentage,
    /// Cost of children
    CostOfChildren,
    /// Income percentage
    IncomePercentage,
}

/// Care percentage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CareLevel {
    /// Below regular (0-13%)
    BelowRegular,
    /// Regular (14-34%)
    Regular,
    /// Shared (35-65%)
    Shared,
    /// Primary (66-86%)
    Primary,
    /// Above primary (87-100%)
    AbovePrimary,
}

// ============================================================================
// Family Violence
// ============================================================================

/// Type of family violence (s.4AB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilyViolenceType {
    /// Physical violence
    Physical,
    /// Sexual abuse
    Sexual,
    /// Emotional or psychological abuse
    EmotionalPsychological,
    /// Economic abuse
    Economic,
    /// Threatening behaviour
    Threatening,
    /// Coercion or control
    CoercionControl,
    /// Damage to property
    PropertyDamage,
    /// Harm to animals
    HarmToAnimals,
}

/// Protection order type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtectionOrderType {
    /// Apprehended Violence Order (NSW)
    AVO,
    /// Family Violence Intervention Order (Vic)
    FVIO,
    /// Domestic Violence Order (Qld)
    DVO,
    /// Violence Restraining Order (WA)
    VRO,
    /// Intervention Order (SA)
    InterventionOrder,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Key Australian family law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyCase {
    /// Case name
    pub name: String,
    /// Citation
    pub citation: String,
    /// Key principle
    pub principle: String,
}

impl FamilyCase {
    /// Stanford v Stanford (2012) - Property approach
    pub fn stanford() -> Self {
        Self {
            name: "Stanford v Stanford".to_string(),
            citation: "[2012] HCA 52".to_string(),
            principle: "No right to property adjustment; just and equitable required".to_string(),
        }
    }

    /// Mallet v Mallet (1984) - Contributions
    pub fn mallet() -> Self {
        Self {
            name: "Mallet v Mallet".to_string(),
            citation: "(1984) 156 CLR 605".to_string(),
            principle: "Homemaker contributions may be equal to financial contributions"
                .to_string(),
        }
    }

    /// Rice v Asplund (1979) - Reopening property
    pub fn rice_asplund() -> Self {
        Self {
            name: "Rice v Asplund".to_string(),
            citation: "(1979) FLC 90-725".to_string(),
            principle: "Reopening property settlements - miscarriage of justice threshold"
                .to_string(),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stanford_case() {
        let case = FamilyCase::stanford();
        assert!(case.citation.contains("HCA"));
    }

    #[test]
    fn test_care_levels() {
        let levels = [
            CareLevel::BelowRegular,
            CareLevel::Regular,
            CareLevel::Shared,
            CareLevel::Primary,
            CareLevel::AbovePrimary,
        ];
        assert_eq!(levels.len(), 5);
    }
}
