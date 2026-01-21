//! Canada Family Law Module
//!
//! This module provides comprehensive modeling of Canadian family law,
//! including the Divorce Act and provincial family law statutes.
//!
//! ## Key Areas
//!
//! - **Divorce**: Grounds for divorce (s.8 Divorce Act)
//! - **Parenting**: Decision-making responsibility and parenting time
//! - **Child Support**: Federal Child Support Guidelines
//! - **Spousal Support**: Spousal Support Advisory Guidelines (SSAG)
//! - **Property Division**: Provincial family property legislation
//!
//! ## 2019 Divorce Act Amendments
//!
//! Key changes in 2021:
//! - "Custody" and "access" replaced with "decision-making responsibility" and "parenting time"
//! - Family violence now mandatory consideration
//! - Relocation provisions (ss.16.9-16.96)
//!
//! ## Key Cases
//!
//! - **Gordon v Goertz** \[1996\] SCC 52: Relocation test
//! - **Moge v Moge** \[1992\] SCC: Compensatory spousal support
//! - **Bracklow v Bracklow** \[1999\] SCC 14: Three bases for support
//! - **DBS v SRG** \[2006\] SCC 37: Retroactive child support
//! - **Contino v Leonelli-Contino** \[2005\] SCC 63: Shared custody support

mod error;
mod parenting;
mod support;
mod types;

pub use error::{FamilyError, FamilyResult};
pub use parenting::{
    ArrangementFunctioning, BestInterestsAnalyzer, BestInterestsFacts, BestInterestsResult,
    ChildInfo, ChildViews, CurrentArrangement, DecisionMaker, DecisionMakingAllocation,
    FactorAnalysis, FactorWeight, FamilyViolenceAllegation, FlexibilityLevel, MaturityLevel,
    ParentInfo, ProposedArrangement, RelocationAnalyzer, RelocationFacts, RelocationResult,
    ViolenceFinding, ViolenceImpact, WillingnessLevel,
};
pub use support::{
    ChildSupportAnalyzer, ChildSupportCalculationType, ChildSupportFacts, ChildSupportResult,
    DurationRange, Section7ExpenseItem, SpousalSupportAnalyzer, SpousalSupportFacts,
    SpousalSupportRange, SpousalSupportResult, UndueHardshipClaim,
};
pub use types::{
    BestInterestsFactor, ChildSupportType, DivorceGround, DivorceStage, ExcludedPropertyType,
    FamilyArea, FamilyCase, FamilyViolence, MarriageStatus, ParentingArrangement,
    ParentingTimeSchedule, PropertyClassification, RelocationReason, RelocationRequest,
    Section7Expense, SpousalSupportBasis, SpousalSupportType, SsagFormula, SupportDuration,
    UndueHardshipFactor, ValuationDate,
};

// Re-export legalis-core types
pub use legalis_core::{Effect, EffectType, Statute};

use crate::common::Province;

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Divorce Act statute
pub fn create_divorce_act() -> Statute {
    Statute::new(
        "DA",
        "Divorce Act, RSC 1985, c 3 (2nd Supp)",
        Effect::new(
            EffectType::Grant,
            "Federal statute governing divorce, parenting arrangements (decision-making \
             responsibility and parenting time), and support orders. Best interests of \
             child is primary consideration for parenting matters.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create Federal Child Support Guidelines statute
pub fn create_child_support_guidelines() -> Statute {
    Statute::new(
        "CSG",
        "Federal Child Support Guidelines, SOR/97-175",
        Effect::new(
            EffectType::Obligation,
            "Establishes child support tables based on payor income and number of children. \
             Includes special/extraordinary expenses (s.7), shared custody (s.9), \
             split custody (s.8), and undue hardship (s.10).",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create provincial family law statute
pub fn create_family_law_act(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => ("ON-FLA", "Family Law Act, RSO 1990, c F.3"),
        Province::BritishColumbia => ("BC-FLA", "Family Law Act, SBC 2011, c 25"),
        Province::Alberta => ("AB-FLA", "Family Law Act, SA 2003, c F-4.5"),
        Province::Quebec => ("QC-CCQ-FAM", "Civil Code of Quebec - Book Two: The Family"),
        Province::Manitoba => ("MB-FPA", "Family Property Act, CCSM c F25"),
        Province::Saskatchewan => ("SK-FPA", "Family Property Act, 1997, SS 1997, c F-6.3"),
        Province::NovaScotia => ("NS-MPA", "Matrimonial Property Act, RSNS 1989, c 275"),
        Province::NewBrunswick => ("NB-MPA", "Marital Property Act, RSNB 2012, c 107"),
        Province::NewfoundlandLabrador => ("NL-FLA", "Family Law Act, RSNL 1990, c F-2"),
        Province::PrinceEdwardIsland => ("PE-FLA", "Family Law Act, RSPEI 1988, c F-2.1"),
        _ => ("FLA", "Family Law Act"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Grant,
            "Provincial family law statute governing property division, spousal support, \
             domestic contracts, and family law proceedings in provincial court.",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create Spousal Support Advisory Guidelines
pub fn create_ssag() -> Statute {
    Statute::new(
        "SSAG",
        "Spousal Support Advisory Guidelines (2008)",
        Effect::new(
            EffectType::Grant,
            "Non-binding guidelines for determining spousal support amount and duration. \
             Two formulas: without child support and with child support. \
             Considers length of relationship and income disparity.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create all family law statutes for a province
pub fn create_family_statutes(province: &Province) -> Vec<Statute> {
    vec![
        create_divorce_act(),
        create_child_support_guidelines(),
        create_family_law_act(province),
        create_ssag(),
    ]
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_divorce_act() {
        let statute = create_divorce_act();
        assert!(statute.title.contains("Divorce Act"));
    }

    #[test]
    fn test_create_child_support_guidelines() {
        let statute = create_child_support_guidelines();
        assert!(statute.title.contains("Child Support"));
    }

    #[test]
    fn test_create_family_law_act() {
        let statute = create_family_law_act(&Province::Ontario);
        assert!(statute.title.contains("Family Law Act"));
    }

    #[test]
    fn test_create_family_statutes() {
        let statutes = create_family_statutes(&Province::BritishColumbia);
        assert_eq!(statutes.len(), 4);
    }

    #[test]
    fn test_create_ssag() {
        let statute = create_ssag();
        assert!(statute.title.contains("Spousal Support"));
    }
}
