//! Canada Family Law - Types
//!
//! Core types for Canadian family law (Divorce Act, provincial family law).

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::common::CaseCitation;

// ============================================================================
// Marriage and Divorce
// ============================================================================

/// Marriage status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarriageStatus {
    /// Legally married
    Married,
    /// Common law relationship
    CommonLaw { years: u32 },
    /// Separated
    Separated { date: String },
    /// Divorced
    Divorced,
    /// Annulled
    Annulled,
}

/// Grounds for divorce under Divorce Act (s.8)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceGround {
    /// Separation for at least one year (s.8(2)(a))
    Separation { separation_date: String },
    /// Adultery (s.8(2)(b)(i))
    Adultery,
    /// Cruelty (s.8(2)(b)(ii))
    Cruelty,
}

/// Divorce process stage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceStage {
    /// Application filed
    ApplicationFiled,
    /// Served on spouse
    Served,
    /// Response/answer filed
    ResponseFiled,
    /// Discovery/disclosure
    Discovery,
    /// Mediation/negotiation
    Mediation,
    /// Trial
    Trial,
    /// Judgment
    Judgment,
    /// Appeal period
    AppealPeriod,
    /// Final (divorce effective)
    Final,
}

// ============================================================================
// Child Custody and Access
// ============================================================================

/// Parenting arrangement type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentingArrangement {
    /// Sole decision-making responsibility (one parent)
    SoleDecisionMaking,
    /// Joint decision-making responsibility
    JointDecisionMaking,
    /// Parallel parenting (divided decision-making)
    ParallelParenting,
    /// Supervised parenting time
    SupervisedParentingTime,
    /// No contact
    NoContact,
}

/// Parenting time schedule
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentingTimeSchedule {
    /// Equal time (50/50)
    EqualTime,
    /// Primarily with one parent
    PrimaryResidence { percentage: u32 },
    /// Every other weekend
    EveryOtherWeekend,
    /// Week on/week off
    WeekOnWeekOff,
    /// Specified days
    SpecifiedDays { schedule: String },
    /// Supervised only
    SupervisedOnly,
}

/// Best interests factors (s.16 Divorce Act)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BestInterestsFactor {
    /// Child's physical, emotional, psychological needs
    ChildNeeds,
    /// Child's cultural, linguistic, religious heritage
    CulturalHeritage,
    /// Child's views and preferences
    ChildViews { age: u32, maturity_level: String },
    /// Nature of relationship with each parent
    RelationshipWithParents,
    /// History of care
    HistoryOfCare,
    /// Each parent's willingness to support relationship
    WillingnessToSupport,
    /// Any family violence
    FamilyViolence,
    /// Civil/criminal proceedings history
    ProceedingsHistory,
    /// Plans for child's care
    CarePlans,
    /// Stability
    Stability,
    /// Ability to communicate/cooperate
    AbilityToCooperate,
}

/// Family violence type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilyViolence {
    /// Physical abuse
    PhysicalAbuse,
    /// Sexual abuse
    SexualAbuse,
    /// Psychological/emotional abuse
    PsychologicalAbuse,
    /// Coercive and controlling behaviour
    CoerciveControl,
    /// Financial abuse
    FinancialAbuse,
    /// Threats or harassment
    ThreatsHarassment,
    /// Exposure of child to violence
    ExposureToViolence,
}

/// Relocation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelocationRequest {
    /// Proposed new location
    pub proposed_location: String,
    /// Reason for relocation
    pub reason: RelocationReason,
    /// Impact on parenting time
    pub parenting_time_impact: String,
    /// Proposed new schedule
    pub proposed_schedule: Option<ParentingTimeSchedule>,
}

/// Relocation reason
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelocationReason {
    /// Employment opportunity
    Employment,
    /// Education
    Education,
    /// New relationship
    NewRelationship,
    /// Family support
    FamilySupport,
    /// Escaping violence
    Safety,
    /// Other
    Other { description: String },
}

// ============================================================================
// Child Support
// ============================================================================

/// Child support type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildSupportType {
    /// Basic table amount (Federal Child Support Guidelines)
    TableAmount,
    /// Section 7 special expenses
    Section7Expenses,
    /// Undue hardship claim
    UndueHardship,
    /// Split custody
    SplitCustody,
    /// Shared custody (over 40%)
    SharedCustody,
}

/// Section 7 special expense categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Section7Expense {
    /// Child care expenses
    ChildCare,
    /// Medical/dental insurance premiums
    HealthInsurance,
    /// Health-related expenses not covered by insurance
    UncoveredHealthExpenses,
    /// Extraordinary extracurricular activities
    ExtracurricularActivities,
    /// Educational expenses (private school, tutoring)
    EducationalExpenses,
    /// Post-secondary education
    PostSecondaryEducation,
}

/// Undue hardship factors (s.10 Guidelines)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UndueHardshipFactor {
    /// High debts from relationship
    RelationshipDebts,
    /// High costs of access
    AccessCosts,
    /// Legal duty to support others
    OtherSupportObligations,
    /// Legal duty to support child from other relationship
    OtherChildSupport,
}

// ============================================================================
// Spousal Support
// ============================================================================

/// Spousal support type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpousalSupportType {
    /// Compensatory (economic disadvantage)
    Compensatory,
    /// Non-compensatory (need-based)
    NonCompensatory,
    /// Contractual (agreement-based)
    Contractual,
}

/// Spousal support basis (s.15.2 Divorce Act)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpousalSupportBasis {
    /// Economic disadvantages from marriage/breakdown
    EconomicDisadvantage,
    /// Financial consequences of child care
    ChildCareConsequences,
    /// Economic hardship from breakdown
    EconomicHardship,
    /// Self-sufficiency promotion
    SelfSufficiency,
}

/// Spousal Support Advisory Guidelines (SSAG) formula
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SsagFormula {
    /// Without child support formula
    WithoutChildSupport,
    /// With child support formula
    WithChildSupport,
    /// Custodial payor formula
    CustodialPayor,
}

/// Duration type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportDuration {
    /// Indefinite (no fixed end)
    Indefinite,
    /// Time-limited
    TimeLimited { months: u32 },
    /// Reviewable
    Reviewable { review_date: String },
}

// ============================================================================
// Property Division
// ============================================================================

/// Property classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyClassification {
    /// Family/matrimonial property (to be divided)
    FamilyProperty,
    /// Excluded property
    ExcludedProperty,
    /// Matrimonial home (special rules)
    MatrimonialHome,
}

/// Excluded property type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExcludedPropertyType {
    /// Property owned before marriage
    PreMaritalProperty,
    /// Gift from third party
    Gift,
    /// Inheritance
    Inheritance,
    /// Personal injury settlement
    PersonalInjuryAward,
    /// Traceable proceeds of above
    TraceableProceeds,
}

/// Property valuation date
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValuationDate {
    /// Date of separation
    SeparationDate,
    /// Date of trial
    TrialDate,
    /// Date of agreement
    AgreementDate,
    /// Other specified date
    Other { date: String },
}

// ============================================================================
// Family Law Cases
// ============================================================================

/// Family law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyCase {
    /// Citation
    pub citation: CaseCitation,
    /// Legal principle
    pub principle: String,
    /// Area of family law
    pub area: FamilyArea,
}

/// Area of family law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilyArea {
    /// Divorce
    Divorce,
    /// Child custody/parenting
    Parenting,
    /// Child support
    ChildSupport,
    /// Spousal support
    SpousalSupport,
    /// Property division
    PropertyDivision,
    /// Family violence
    FamilyViolence,
}

impl FamilyCase {
    /// Gordon v Goertz [1996] - relocation test
    pub fn gordon_v_goertz() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Gordon v Goertz",
                1996,
                52,
                "Relocation - best interests analysis",
            ),
            principle: "Relocation requires fresh best interests analysis. \
                No presumption for or against relocation. Consider all circumstances."
                .to_string(),
            area: FamilyArea::Parenting,
        }
    }

    /// Young v Young [1993] - religious upbringing
    pub fn young_v_young() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Young v Young",
                1993,
                3,
                "Best interests paramount - religious practices",
            ),
            principle: "Best interests of child are paramount. Access can be restricted \
                if religious practices cause demonstrable harm to child."
                .to_string(),
            area: FamilyArea::Parenting,
        }
    }

    /// Bracklow v Bracklow [1999] - spousal support bases
    pub fn bracklow() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Bracklow v Bracklow",
                1999,
                14,
                "Three bases for spousal support",
            ),
            principle: "Three conceptual bases for spousal support: \
                (1) Compensatory (Moge), (2) Non-compensatory (needs-based), \
                (3) Contractual. Not mutually exclusive."
                .to_string(),
            area: FamilyArea::SpousalSupport,
        }
    }

    /// Moge v Moge [1992] - compensatory support
    pub fn moge() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Moge v Moge",
                1992,
                3,
                "Compensatory spousal support model",
            ),
            principle: "Spousal support should compensate for economic disadvantage \
                arising from marriage or its breakdown. Self-sufficiency important \
                but not paramount - support obligations continue post-divorce."
                .to_string(),
            area: FamilyArea::SpousalSupport,
        }
    }

    /// DBS v SRG [2006] - child support guidelines
    pub fn dbs_v_srg() -> Self {
        Self {
            citation: CaseCitation::scc(
                "DBS v SRG",
                2006,
                37,
                "Child support - retroactive claims",
            ),
            principle: "Retroactive child support claims assessed based on: \
                (1) Reason for recipient's delay, (2) Conduct of payor, \
                (3) Circumstances of child, (4) Hardship to payor."
                .to_string(),
            area: FamilyArea::ChildSupport,
        }
    }

    /// Contino v Leonelli-Contino [2005] - shared custody support
    pub fn contino() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Contino v Leonelli-Contino",
                2005,
                63,
                "Shared custody child support (s.9)",
            ),
            principle: "Section 9 (shared custody) requires consideration of: \
                (1) Set-off between table amounts, (2) Increased costs of shared custody, \
                (3) Conditions, means, needs of each spouse/child."
                .to_string(),
            area: FamilyArea::ChildSupport,
        }
    }

    /// Michel v Graydon [2020] - 2019 Divorce Act amendments
    pub fn michel_v_graydon() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Michel v Graydon",
                2020,
                24,
                "Divorce Act parenting terminology",
            ),
            principle: "2019 Divorce Act amendments replaced 'custody' and 'access' \
                with 'decision-making responsibility' and 'parenting time'. \
                Best interests remain paramount."
                .to_string(),
            area: FamilyArea::Parenting,
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
    fn test_marriage_status() {
        let married = MarriageStatus::Married;
        let common_law = MarriageStatus::CommonLaw { years: 3 };
        assert_ne!(married, common_law);
    }

    #[test]
    fn test_divorce_ground() {
        let separation = DivorceGround::Separation {
            separation_date: "2024-01-01".to_string(),
        };
        match separation {
            DivorceGround::Separation { separation_date } => {
                assert!(separation_date.contains("2024"));
            }
            _ => panic!("Expected separation"),
        }
    }

    #[test]
    fn test_best_interests_factors() {
        let factor = BestInterestsFactor::ChildViews {
            age: 12,
            maturity_level: "mature for age".to_string(),
        };
        match factor {
            BestInterestsFactor::ChildViews { age, .. } => assert_eq!(age, 12),
            _ => panic!("Expected child views"),
        }
    }

    #[test]
    fn test_gordon_v_goertz() {
        let case = FamilyCase::gordon_v_goertz();
        assert_eq!(case.citation.year, 1996);
        assert!(case.principle.contains("relocation"));
    }

    #[test]
    fn test_bracklow_case() {
        let case = FamilyCase::bracklow();
        assert!(case.principle.contains("Three"));
    }

    #[test]
    fn test_moge_case() {
        let case = FamilyCase::moge();
        assert_eq!(case.area, FamilyArea::SpousalSupport);
    }

    #[test]
    fn test_contino_case() {
        let case = FamilyCase::contino();
        assert!(case.principle.contains("shared custody"));
    }
}
