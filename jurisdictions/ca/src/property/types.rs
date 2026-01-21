//! Canada Property Law - Types
//!
//! Core types for Canadian property law including real property,
//! land registration, and Aboriginal title.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::common::{CaseCitation, Province};

// ============================================================================
// Land Classification
// ============================================================================

/// Property type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Freehold estate
    Freehold,
    /// Leasehold estate
    Leasehold,
    /// Condominium/strata title
    Condominium,
    /// Co-operative
    Cooperative,
    /// Crown land
    CrownLand,
    /// Aboriginal title land
    AboriginalTitle,
    /// Reserve land (Indian Act)
    ReserveLand,
}

/// Estate type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EstateType {
    /// Fee simple (full ownership)
    FeeSimple,
    /// Life estate
    LifeEstate,
    /// Leasehold (fixed term)
    Leasehold { term_years: u32 },
    /// Fee tail (historical)
    FeeTail,
    /// Periodic tenancy
    PeriodicTenancy { period: TenancyPeriod },
}

/// Tenancy period
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenancyPeriod {
    /// Month to month
    Monthly,
    /// Year to year
    Yearly,
    /// Week to week
    Weekly,
}

/// Co-ownership type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoOwnershipType {
    /// Joint tenancy (right of survivorship)
    JointTenancy,
    /// Tenancy in common
    TenancyInCommon,
    /// Tenancy by the entirety (married couples, some provinces)
    TenancyByEntirety,
}

// ============================================================================
// Land Registration
// ============================================================================

/// Land registration system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandRegistrationSystem {
    /// Torrens (title registration)
    Torrens,
    /// Registry (deeds registration)
    Registry,
    /// Land Titles (similar to Torrens)
    LandTitles,
}

/// Title assurance principle
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleAssurance {
    /// Mirror principle (register reflects title)
    Mirror,
    /// Curtain principle (no need to look behind register)
    Curtain,
    /// Insurance principle (compensation for errors)
    Insurance,
}

/// Title exception (overriding interest)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleException {
    /// Short-term lease
    ShortTermLease,
    /// Easement by prescription
    PrescriptiveEasement,
    /// Rights of person in actual occupation
    ActualOccupation,
    /// Aboriginal rights
    AboriginalRights,
    /// Tax liens
    TaxLiens,
    /// Statutory right of way
    StatutoryRightOfWay,
}

// ============================================================================
// Interests in Land
// ============================================================================

/// Interest in land type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterestInLand {
    /// Easement
    Easement(EasementType),
    /// Restrictive covenant
    RestrictiveCovenant,
    /// Mortgage/charge
    Mortgage,
    /// Lien
    Lien(LienType),
    /// Profit à prendre
    ProfitAPrendre,
    /// License (not an interest, but commonly confused)
    License,
}

/// Easement type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementType {
    /// Right of way
    RightOfWay,
    /// Right to light
    Light,
    /// Right of drainage
    Drainage,
    /// Right of support
    Support,
    /// Utility easement
    Utility,
    /// Parking
    Parking,
}

/// Lien type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LienType {
    /// Mechanics/construction lien
    ConstructionLien,
    /// Vendor's lien
    VendorsLien,
    /// Tax lien
    TaxLien,
    /// Judgment lien
    JudgmentLien,
}

/// Easement creation method
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementCreation {
    /// Express grant
    ExpressGrant,
    /// Express reservation
    ExpressReservation,
    /// Implied grant (necessity, common intention, Wheeldon v Burrows)
    Implied,
    /// Prescription (long use)
    Prescription,
    /// Statute
    Statutory,
}

// ============================================================================
// Aboriginal Title
// ============================================================================

/// Aboriginal title status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AboriginalTitleStatus {
    /// Proven Aboriginal title (Tsilhqot'in)
    Proven,
    /// Claimed but not yet determined
    Claimed,
    /// Subject to treaty
    TreatyLand,
    /// Traditional territory (not title land)
    TraditionalTerritory,
}

/// Elements of Aboriginal title (Tsilhqot'in)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AboriginalTitleElement {
    /// Sufficient occupation (intensity, frequency)
    SufficientOccupation,
    /// Continuity of occupation
    Continuity,
    /// Exclusivity of occupation
    Exclusivity,
}

/// Duty to consult trigger (Haida Nation)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsultationTrigger {
    /// Crown conduct
    CrownConduct,
    /// That might adversely affect
    AdverseEffect,
    /// Asserted or established Aboriginal rights
    AboriginalRights,
}

/// Consultation level (Haida spectrum)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsultationLevel {
    /// Low (notice, disclosure)
    Low,
    /// Moderate (opportunity to respond, consider concerns)
    Moderate,
    /// Deep (full consultation, accommodation)
    Deep,
}

/// Justification for infringement (Sparrow test)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfringementJustification {
    /// Valid legislative objective
    ValidObjective,
    /// Priority to Aboriginal use
    AboriginalPriority,
    /// Minimal impairment
    MinimalImpairment,
    /// Fair compensation
    FairCompensation,
    /// Consultation
    Consultation,
}

// ============================================================================
// Conveyancing
// ============================================================================

/// Conveyancing stage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConveyancingStage {
    /// Pre-contract (negotiations, conditions)
    PreContract,
    /// Agreement of purchase and sale
    AgreementOfPurchase,
    /// Due diligence period
    DueDiligence,
    /// Requisition period
    Requisitions,
    /// Pre-closing
    PreClosing,
    /// Closing
    Closing,
    /// Post-closing
    PostClosing,
}

/// Standard condition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardCondition {
    /// Financing
    Financing,
    /// Home inspection
    HomeInspection,
    /// Title search
    TitleSearch,
    /// Sale of buyer's property
    SaleOfProperty,
    /// Lawyer review
    LawyerReview,
    /// Environmental assessment
    EnvironmentalAssessment,
}

/// Title defect
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleDefect {
    /// Outstanding mortgage
    OutstandingMortgage,
    /// Encroachment
    Encroachment,
    /// Easement not disclosed
    UndisclosedEasement,
    /// Building code violation
    BuildingViolation,
    /// Zoning non-compliance
    ZoningIssue,
    /// Tax arrears
    TaxArrears,
    /// Survey issues
    SurveyIssue,
    /// Legal description error
    LegalDescriptionError,
}

// ============================================================================
// Property Cases
// ============================================================================

/// Property law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyCase {
    /// Citation
    pub citation: CaseCitation,
    /// Legal principle
    pub principle: String,
    /// Area of property law
    pub area: PropertyArea,
}

/// Area of property law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyArea {
    /// Land registration
    Registration,
    /// Aboriginal title
    AboriginalTitle,
    /// Easements
    Easements,
    /// Mortgages
    Mortgages,
    /// Co-ownership
    CoOwnership,
    /// Conveyancing
    Conveyancing,
}

impl PropertyCase {
    /// Tsilhqot'in Nation v BC \[2014\] - Aboriginal title test
    pub fn tsilhqotin() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Tsilhqot'in Nation v British Columbia",
                2014,
                44,
                "Test for Aboriginal title",
            ),
            principle: "Aboriginal title established by showing: (1) Sufficient occupation \
                (regular, exclusive use), (2) Continuity between present occupation and \
                pre-sovereignty occupation, (3) Exclusive occupation at sovereignty. \
                Title is collective, inalienable except to Crown, and comes with inherent limit \
                that it cannot be used in ways inconsistent with group's attachment."
                .to_string(),
            area: PropertyArea::AboriginalTitle,
        }
    }

    /// Haida Nation v BC \[2004\] - Duty to consult
    pub fn haida() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Haida Nation v British Columbia (Minister of Forests)",
                2004,
                73,
                "Duty to consult and accommodate",
            ),
            principle: "Crown has duty to consult (and possibly accommodate) when it has \
                knowledge of potential Aboriginal right or title AND contemplates conduct \
                that might adversely affect it. Duty is on spectrum based on strength of claim \
                and severity of impact. Good faith required."
                .to_string(),
            area: PropertyArea::AboriginalTitle,
        }
    }

    /// Delgamuukw v BC \[1997\] - Aboriginal title content
    pub fn delgamuukw() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Delgamuukw v British Columbia",
                1997,
                1010,
                "Nature and content of Aboriginal title",
            ),
            principle: "Aboriginal title is sui generis - unique. It is: (1) Inalienable except \
                to Crown, (2) Source is prior occupation, (3) Held communally, \
                (4) Encompasses right to exclusive use and occupation. \
                Can be infringed but requires justification (Sparrow test)."
                .to_string(),
            area: PropertyArea::AboriginalTitle,
        }
    }

    /// R v Sparrow \[1990\] - Aboriginal rights framework
    pub fn sparrow() -> Self {
        Self {
            citation: CaseCitation::scc(
                "R v Sparrow",
                1990,
                1075,
                "Framework for Aboriginal rights under s.35",
            ),
            principle: "s.35 provides constitutional protection for existing Aboriginal \
                rights. Rights can be infringed if: (1) Valid legislative objective, \
                (2) Honour of Crown maintained (consultation, priority to Aboriginal use, \
                minimal impairment, fair compensation)."
                .to_string(),
            area: PropertyArea::AboriginalTitle,
        }
    }

    /// CIBC Mortgages v Rowatt \[2002\] - Priority of mortgages
    pub fn cibc_mortgages() -> Self {
        Self {
            citation: CaseCitation {
                name: "CIBC Mortgages Inc v Rowatt".to_string(),
                year: 2002,
                neutral_citation: Some("[2002] ONCA".to_string()),
                report_citation: Some("161 OAC 359".to_string()),
                court: crate::common::Court::ProvincialCourtOfAppeal {
                    province: Province::Ontario,
                },
                principle: "Mortgage priority rules".to_string(),
            },
            principle: "Priority between mortgages determined by registration, \
                subject to fraud, postponement agreements, and statutory exceptions."
                .to_string(),
            area: PropertyArea::Mortgages,
        }
    }

    /// Stack v Dowden \[2007\] - Beneficial interests (UK persuasive)
    pub fn stack_v_dowden() -> Self {
        Self {
            citation: CaseCitation {
                name: "Stack v Dowden".to_string(),
                year: 2007,
                neutral_citation: Some("[2007] UKHL 17".to_string()),
                report_citation: Some("[2007] 2 AC 432".to_string()),
                court: crate::common::Court::Tribunal {
                    name: "House of Lords (UK - persuasive)".to_string(),
                },
                principle: "Presumption of equal beneficial shares".to_string(),
            },
            principle: "Where property held in joint names, presumption of equal beneficial \
                shares. Can be rebutted by evidence of common intention otherwise. \
                Persuasive in Canadian courts."
                .to_string(),
            area: PropertyArea::CoOwnership,
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
    fn test_property_type() {
        let freehold = PropertyType::Freehold;
        let leasehold = PropertyType::Leasehold;
        assert_ne!(freehold, leasehold);
    }

    #[test]
    fn test_estate_type() {
        let fee_simple = EstateType::FeeSimple;
        let leasehold = EstateType::Leasehold { term_years: 99 };
        assert_ne!(fee_simple, leasehold);
    }

    #[test]
    fn test_land_registration_system() {
        let torrens = LandRegistrationSystem::Torrens;
        assert_eq!(torrens, LandRegistrationSystem::Torrens);
    }

    #[test]
    fn test_tsilhqotin_case() {
        let case = PropertyCase::tsilhqotin();
        assert_eq!(case.citation.year, 2014);
        assert!(case.principle.contains("Sufficient occupation"));
    }

    #[test]
    fn test_haida_case() {
        let case = PropertyCase::haida();
        assert_eq!(case.area, PropertyArea::AboriginalTitle);
        assert!(case.principle.contains("duty to consult"));
    }

    #[test]
    fn test_delgamuukw_case() {
        let case = PropertyCase::delgamuukw();
        assert!(case.principle.contains("sui generis"));
    }

    #[test]
    fn test_sparrow_case() {
        let case = PropertyCase::sparrow();
        assert!(case.principle.contains("s.35"));
    }

    #[test]
    fn test_co_ownership() {
        let joint = CoOwnershipType::JointTenancy;
        let common = CoOwnershipType::TenancyInCommon;
        assert_ne!(joint, common);
    }
}
