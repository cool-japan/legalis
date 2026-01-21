//! UK Land Law Module
//!
//! This module provides comprehensive analysis of English land law,
//! covering estates, interests, registration, and conveyancing.
//!
//! # Overview
//!
//! Land law in England and Wales is primarily governed by:
//! - **Law of Property Act 1925** (LPA 1925) - defines legal estates and interests
//! - **Land Registration Act 2002** (LRA 2002) - registered land system
//! - **Landlord and Tenant Act 1954** (LTA 1954) - business tenancy protection
//! - **Trusts of Land and Appointment of Trustees Act 1996** (TOLATA)
//!
//! # Module Structure
//!
//! - `types` - Core types for estates, interests, and registration
//! - `error` - Error types for land law operations
//! - `estates` - Freehold, leasehold, and lease/licence analysis
//! - `interests` - Easements, covenants, and mortgages
//! - `registration` - LRA 2002 and unregistered land
//!
//! # Key Cases
//!
//! - **Street v Mountford** \[1985\] AC 809 - lease vs licence distinction
//! - **Re Ellenborough Park** \[1956\] Ch 131 - easement requirements
//! - **Tulk v Moxhay** (1848) 2 Ph 774 - restrictive covenants in equity
//! - **Williams & Glyn's Bank v Boland** \[1981\] AC 487 - overriding interests
//! - **City of London BS v Flegg** \[1988\] AC 54 - overreaching
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_uk::land_law::{
//!     LeaseOrLicenceAnalyzer, LeaseOrLicenceFacts,
//!     EasementAnalyzer, EasementFacts,
//! };
//!
//! // Analyze lease vs licence
//! let facts = LeaseOrLicenceFacts {
//!     description: "Room in shared house".into(),
//!     exclusive_possession: true,
//!     certain_term: true,
//!     rent_payable: true,
//!     shared_occupation: false,
//!     grantor_access: false,
//!     service_element: false,
//!     label_used: "Licence Agreement".into(),
//!     sham_indicators: vec![],
//! };
//!
//! let result = LeaseOrLicenceAnalyzer::analyze(&facts);
//! assert!(result.is_lease);
//! ```

#![allow(missing_docs)]

pub mod error;
pub mod estates;
pub mod interests;
pub mod registration;
pub mod types;

// ============================================================================
// Re-exports - Types
// ============================================================================

pub use types::{
    // Estates
    CoOwnershipType,
    // Conveyancing
    ConveyancingSearch,
    ConveyancingStage,
    // Interests
    Covenant,
    CovenantNature,
    Easement,
    EasementCreation,
    EasementType,
    EstateType,
    // Registration
    FirstRegistrationTrigger,
    FreeholdEstate,
    InterestType,
    LandChargeClass,
    LandContract,
    LandLawCase,
    LeaseDuration,
    LeaseholdEstate,
    Mortgage,
    MortgageRemedy,
    OverridingInterest,
    Owner,
    OwnerType,
    PeriodicTenancy,
    PropertyAddress,
    RegisterEntry,
    RegisterProtection,
    RegistrationStatus,
    RestrictionType,
    // Trusts of land
    Section15Factor,
    TitleClass,
    TitleGuarantee,
    TolataClaim,
    TolataOrder,
    TransferDeed,
    TrustOfLandType,
};

// ============================================================================
// Re-exports - Errors
// ============================================================================

pub use error::{
    ConveyancingError, EstateError, InterestError, LandLawError, LandLawResult, MortgageError,
    RegistrationError,
};

// ============================================================================
// Re-exports - Estates Analysis
// ============================================================================

pub use estates::{
    // Freehold
    AcquisitionType,
    // Leasehold
    BreachType,
    ForfeitureAnalysisResult,
    // Forfeiture
    ForfeitureAnalyzer,
    ForfeitureFacts,
    ForfeitureRisk,
    FreeholdAnalysisResult,
    FreeholdAnalyzer,
    FreeholdFacts,
    // Lease vs Licence
    LeaseOrLicenceAnalyzer,
    LeaseOrLicenceFacts,
    LeaseOrLicenceResult,
    LeaseUseType,
    LeaseholdAnalysisResult,
    LeaseholdAnalyzer,
    LeaseholdFacts,
    LicenceException,
    // LTA 1954
    Lta1954Analyzer,
    Lta1954Facts,
    Lta1954Ground,
    Lta1954Result,
    ReliefLikelihood,
    TitleQuality,
};

// ============================================================================
// Re-exports - Interests Analysis
// ============================================================================

pub use interests::{
    // Covenants
    CovenantAnalysisResult,
    CovenantAnalyzer,
    CovenantFacts,
    // Easements
    CreationFacts,
    // Mortgages
    DefaultType,
    EasementAnalysisResult,
    EasementAnalyzer,
    EasementBenefit,
    EasementFacts,
    EnforcementMethod,
    LegalOrEquitable,
    MortgageAnalysisResult,
    MortgageAnalyzer,
    MortgageFacts,
    UndueInfluenceRisk,
};

// ============================================================================
// Re-exports - Registration Analysis
// ============================================================================

pub use registration::{
    // Alteration
    AlterationAnalyzer,
    AlterationFacts,
    AlterationResult,
    AlterationType,
    // First Registration
    FirstRegistrationAnalyzer,
    FirstRegistrationFacts,
    FirstRegistrationResult,
    // Priority
    InterestCategory,
    // Overriding Interests
    LegalEasementFacts,
    OccupationFacts,
    OverridingInterestAnalyzer,
    OverridingInterestFacts,
    OverridingInterestResult,
    PriorityAnalyzer,
    PriorityBasis,
    PriorityFacts,
    PriorityResult,
    ShortLeaseFacts,
    // Unregistered Land
    UnregisteredLandAnalyzer,
    UnregisteredLandFacts,
    UnregisteredLandResult,
};

// ============================================================================
// Integration with legalis-core
// ============================================================================

use legalis_core::{Effect, Statute, StatuteBuilder};

/// Creates an LPA 1925 statute for core framework integration
pub fn create_lpa_1925_statute() -> Statute {
    StatuteBuilder::new()
        .id("uk-lpa-1925")
        .title("Law of Property Act 1925")
        .effect(Effect::grant("Defines legal estates and interests in land"))
        .jurisdiction("UK")
        .build()
        .expect("LPA 1925 statute should build successfully")
}

/// Creates an LRA 2002 statute for core framework integration
pub fn create_lra_2002_statute() -> Statute {
    StatuteBuilder::new()
        .id("uk-lra-2002")
        .title("Land Registration Act 2002")
        .effect(Effect::grant("Establishes modern land registration system"))
        .jurisdiction("UK")
        .build()
        .expect("LRA 2002 statute should build successfully")
}

/// Creates an LTA 1954 statute for core framework integration
pub fn create_lta_1954_statute() -> Statute {
    StatuteBuilder::new()
        .id("uk-lta-1954")
        .title("Landlord and Tenant Act 1954")
        .effect(Effect::grant(
            "Provides security of tenure for business tenancies",
        ))
        .jurisdiction("UK")
        .build()
        .expect("LTA 1954 statute should build successfully")
}

/// Creates a TOLATA 1996 statute for core framework integration
pub fn create_tolata_statute() -> Statute {
    StatuteBuilder::new()
        .id("uk-tolata-1996")
        .title("Trusts of Land and Appointment of Trustees Act 1996")
        .effect(Effect::grant(
            "Reforms law on trusts of land and co-ownership",
        ))
        .jurisdiction("UK")
        .build()
        .expect("TOLATA statute should build successfully")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_lpa_statute() {
        let statute = create_lpa_1925_statute();
        assert_eq!(statute.id, "uk-lpa-1925");
    }

    #[test]
    fn test_create_lra_statute() {
        let statute = create_lra_2002_statute();
        assert_eq!(statute.id, "uk-lra-2002");
    }

    #[test]
    fn test_create_lta_statute() {
        let statute = create_lta_1954_statute();
        assert_eq!(statute.id, "uk-lta-1954");
    }

    #[test]
    fn test_create_tolata_statute() {
        let statute = create_tolata_statute();
        assert_eq!(statute.id, "uk-tolata-1996");
    }

    #[test]
    fn test_lease_vs_licence_basic() {
        let facts = LeaseOrLicenceFacts {
            description: "Flat rental".into(),
            exclusive_possession: true,
            certain_term: true,
            rent_payable: true,
            shared_occupation: false,
            grantor_access: false,
            service_element: false,
            label_used: "Tenancy Agreement".into(),
            sham_indicators: vec![],
        };

        let result = LeaseOrLicenceAnalyzer::analyze(&facts);
        assert!(result.is_lease);
        assert!(result.exclusive_possession_found);
    }

    #[test]
    fn test_lodger_is_licence() {
        let facts = LeaseOrLicenceFacts {
            description: "Room with meals".into(),
            exclusive_possession: false,
            certain_term: true,
            rent_payable: true,
            shared_occupation: false,
            grantor_access: true,
            service_element: true,
            label_used: "Lodger Agreement".into(),
            sham_indicators: vec![],
        };

        let result = LeaseOrLicenceAnalyzer::analyze(&facts);
        assert!(!result.is_lease);
    }

    #[test]
    fn test_first_registration_trigger() {
        let facts = FirstRegistrationFacts {
            status: RegistrationStatus::Unregistered,
            trigger: Some(FirstRegistrationTrigger::TransferOfFreehold),
            trigger_date: Some("2025-01-01".into()),
            days_since_trigger: 30,
            voluntary: false,
            property: "123 High Street".into(),
        };

        let result = FirstRegistrationAnalyzer::analyze(&facts);
        assert!(result.registration_required);
        assert!(result.registration_compulsory);
        assert_eq!(result.deadline_days, 60);
    }
}
