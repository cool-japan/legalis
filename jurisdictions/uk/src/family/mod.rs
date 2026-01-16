//! UK Family Law Module
//!
//! Comprehensive implementation of UK family law under:
//!
//! # Key Legislation
//!
//! ## Relationships
//! - **Marriage Act 1949**: Marriage formalities and validity
//! - **Civil Partnership Act 2004**: Civil partnerships
//! - **Divorce, Dissolution and Separation Act 2020**: No-fault divorce
//! - **Matrimonial Causes Act 1973**: Divorce, nullity, financial remedies
//!
//! ## Children
//! - **Children Act 1989**: Parental responsibility, s.8 orders, welfare principle
//! - **Adoption and Children Act 2002**: Adoption
//! - **Human Fertilisation and Embryology Act 2008**: Parenthood
//!
//! ## Financial
//! - **Matrimonial Causes Act 1973**: Financial remedies on divorce (ss.22-25A)
//! - **Child Support Act 1991**: Child maintenance
//!
//! ## Protection
//! - **Family Law Act 1996 Part IV**: Non-molestation and occupation orders
//! - **Domestic Abuse Act 2021**: Definition of domestic abuse
//! - **Serious Crime Act 2015 s.76**: Coercive control
//! - **Protection from Harassment Act 1997**: Harassment
//!
//! # Key Principles
//!
//! ## Welfare Principle (CA 1989 s.1)
//!
//! The child's welfare is the court's **paramount** consideration. Court must have
//! regard to the welfare checklist (s.1(3)) in contested cases.
//!
//! ## No Delay Principle (CA 1989 s.1(2))
//!
//! Delay is likely to prejudice the welfare of the child.
//!
//! ## No Order Principle (CA 1989 s.1(5))
//!
//! Court shall not make an order unless it considers that doing so would be
//! better for the child than making no order at all.
//!
//! ## Clean Break (MCA 1973 s.25A)
//!
//! Court must consider whether it is appropriate to terminate financial
//! obligations between parties as soon as just and reasonable.
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::family::*;
//!
//! // Analyze divorce application
//! let analysis = DivorceApplicationAnalysis::analyze(
//!     marriage_date,
//!     application_date,
//!     ApplicationType::Joint,
//!     true, // statement of breakdown
//!     &JurisdictionBasis::BothHabituallyResident,
//! );
//!
//! // Check if conditional order can be granted
//! let conditional = ConditionalOrderAnalysis::analyze(
//!     application_date,
//!     conditional_order_date,
//!     &ApplicationType::Joint,
//!     true, // service effected
//!     true, // acknowledgement filed
//! );
//!
//! // Analyze child arrangements
//! let standing = StandingAnalysis::analyze(
//!     "Parent",
//!     ApplicantCategory::Parent,
//!     Section8OrderType::ChildArrangements,
//! );
//!
//! // Analyze financial remedies
//! let sharing = SharingAnalysis::analyze(
//!     1000000.0, // matrimonial assets
//!     50000.0,   // party 1 non-matrimonial
//!     0.0,       // party 2 non-matrimonial
//!     15,        // marriage duration
//!     false,     // special contribution
//! );
//! ```

pub mod children;
pub mod divorce;
pub mod error;
pub mod financial;
pub mod protection;
pub mod types;

// Re-export error types
pub use error::{FamilyLawError, Result};

// Re-export core types
pub use types::{
    AbuseType, Asset, AssetOwnership, AssetType, AssociatedPersonRelationship,
    BalanceOfHarmOutcome, BalanceOfHarmTest, CeremonyType, ChildArrangementsOrder, ChildDetails,
    CivilPartnership, CivilPartnershipInvalidityGround, Cohabitation, ContactType, Contribution,
    ContributionType, FamilyCourtLevel, FamilyOrderType, FamilyProceedingsParty,
    FamilyProceedingsType, FinancialPosition, Gender, HarmFactor, HarmSeverity, HarmType,
    HousingNeeds, Marriage, MarriageInvalidityGround, NonMolestationOrder, OccupationOrder,
    OccupationOrderCategory, OccupationOrderProvision, OtherContactArrangement,
    PRAcquisitionMethod, PREndReason, ParentalResponsibility, ParentalResponsibilityHolder,
    PartyRole, PersonDetails, ProhibitedStep, ProhibitedStepsOrder, RelationshipType,
    Representation, SpecialMeasure, SpecificIssue, SpecificIssueOrder, SpendingTimeArrangement,
};

// Re-export divorce types and functions
pub use divorce::{
    ApplicationType, CalculatedDates, ConditionalOrderAnalysis, DivorceApplication,
    DivorceApplicationAnalysis, DivorceDates, DivorceStage, DivorceTimeline, FinalOrderAnalysis,
    FinancialMattersStatus, JurisdictionBasis, MarriageValidityAnalysis,
    validate_conditional_order_timing, validate_divorce_application, validate_final_order_timing,
};

// Re-export children types and functions
pub use children::{
    ApplicantCategory, CafcassRecommendation, CafcassReportType, ChildArrangementsAnalysis,
    FactorWeight, GuardianSuitability, IdentifiedRisk, ParentalResponsibilityAnalysis,
    ProposedArrangements, RiskAssessment, RiskImpact, RiskLikelihood, RiskType, Section8OrderType,
    SpecialGuardianshipAnalysis, StandingAnalysis, StatusQuoAssessment, WelfareChecklistAnalysis,
    WelfareChecklistFactor, WelfareFactorAssessment, validate_child_wishes_considered,
    validate_sg_notice, validate_welfare_checklist,
};

// Re-export financial types and functions
pub use financial::{
    AssetSchedule, CleanBreakAnalysis, CompensationAnalysis, EqualDivisionCalculation,
    FactorImpact, HousingNeedsAssessment, IncomeNeedsAssessment, NeedsAnalysis, PensionAnalysis,
    PensionApproach, PensionDetails, PensionSharingOrder, PensionType, PrenupAnalysis,
    PrenupWeight, ScheduledAsset, Section25Assessment, Section25Factor, SharingAnalysis,
    ThreeStrandsAnalysis, validate_clean_break_considered, validate_form_e_filed,
    validate_section25_factors,
};

// Re-export protection types and functions
pub use protection::{
    AbuseSeverity, AbuseTypeAssessment, AssociatedPersonAnalysis, DomesticAbuseAnalysis,
    EvidenceAssessment, EvidenceStrength, FGMProtectionAnalysis, ForcedMarriageAnalysis,
    NonMolestationOrderAnalysis, OccupationOrderAnalysis, RiskLevel, Section33_6Factors,
    UndertakingAnalysis, perform_balance_of_harm_test, validate_associated_persons,
    validate_undertaking_appropriate, validate_without_notice,
};
