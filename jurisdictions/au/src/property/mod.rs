//! Australian Property Law
//!
//! Implementation of Australian property law including:
//! - Torrens title system
//! - Native title (Native Title Act 1993)
//! - Strata/community title
//!
//! ## Key Legislation
//!
//! - Real Property Acts (various states)
//! - Native Title Act 1993 (Cth)
//! - Strata Schemes Management Acts
//!
//! ## Key Cases
//!
//! - Mabo v Queensland (No 2) (1992) - Native title
//! - Wik v Queensland (1996) - Pastoral leases
//! - Breskvar v Wall (1971) - Indefeasibility

pub mod native_title;
pub mod strata;
pub mod torrens;
pub mod types;

// Re-export commonly used types
pub use native_title::{
    FutureActFacts, FutureActResult, FutureActsAnalyzer, NativeTitleAnalyzer, NativeTitleFacts,
    NativeTitleResult, ProceduralRight,
};
pub use torrens::{
    IndefeasibilityAnalyzer, IndefeasibilityFacts, IndefeasibilityResult, PriorityAnalyzer,
    PriorityFacts, PriorityResult, PriorityWinner,
};
pub use types::{
    CovenantType, DeterminationType, EasementType, Estate, FutureActType, IndefeasibilityException,
    LandInterest, LandTenure, NativeTitleRight, OverridingInterest, PropertyCase, StrataLotType,
    TorrensPrinciple,
};

// Re-export strata types
pub use strata::{
    // Meetings
    AgendaItem,
    BuildingClass,
    BuildingDefect,
    BuildingDetails,
    ByLaw,
    ByLawCategory,
    ByLawValidity,
    ClaimUrgency,
    // Owners corporation
    CommitteeMember,
    CommitteePosition,
    CommonFacility,
    CommonProperty,
    DefectClaimAssessment,
    DefectLocation,
    DefectRemedy,
    DefectSeverity,
    DefectType,
    ExclusiveUseArea,
    ExclusiveUseType,
    FinancialYearEnd,
    GeneralMeeting,
    InsuranceDetails,
    LevyCalculation,
    LevyStructure,
    LotOwner,
    LotType,
    MeetingType,
    OwnersCorporation,
    OwnershipType,
    PaymentFrequency,
    ResolutionCheck,
    ResolutionResult,
    ResolutionType,
    // Warranty
    ResponsibleParty,
    SpecialLevy,
    StrataJurisdiction,
    StrataLot,
    StrataManager,
    StrataScheme,
    StrataSchemeType,
    WarrantyStatus,
    WarrantyType,
    // Scheme structure
    assess_defect_claim,
    calculate_lot_levy,
    check_resolution_threshold,
    validate_bylaw,
};

use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Native Title Act 1993 statute
pub fn create_native_title_act() -> Statute {
    Statute::new(
        "AU-NTA-1993",
        "Native Title Act 1993 (Cth)",
        Effect::new(
            EffectType::Grant,
            "Recognition and protection of native title; future act regime",
        ),
    )
    .with_jurisdiction("AU")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_native_title_act() {
        let statute = create_native_title_act();
        assert!(statute.id.contains("NTA"));
    }

    #[test]
    fn test_indefeasibility_analysis() {
        let facts = IndefeasibilityFacts {
            duly_registered: true,
            ..Default::default()
        };

        let result = IndefeasibilityAnalyzer::analyze(&facts);
        assert!(result.indefeasible);
    }

    #[test]
    fn test_native_title_analysis() {
        let facts = NativeTitleFacts {
            traditional_laws_acknowledged: true,
            traditional_customs_observed: true,
            continuous_connection: true,
            identifiable_group: true,
            ..Default::default()
        };

        let result = NativeTitleAnalyzer::analyze(&facts);
        assert!(result.native_title_exists);
    }
}
