//! Canada Property Law Module
//!
//! This module provides comprehensive modeling of Canadian property law,
//! including real property, land registration, and Aboriginal title.
//!
//! ## Key Areas
//!
//! - **Land Classification**: Freehold, leasehold, condominium, Crown land
//! - **Land Registration**: Torrens system, Registry system, Land Titles
//! - **Interests in Land**: Easements, covenants, mortgages, liens
//! - **Aboriginal Title**: Tsilhqot'in test, duty to consult (Haida)
//! - **Conveyancing**: Agreement of purchase and sale, closing
//!
//! ## Provincial Variations
//!
//! Land registration systems vary by province:
//! - **Ontario**: Land Titles (electronic), former Registry
//! - **British Columbia**: Land Title Act (Torrens)
//! - **Alberta**: Land Titles Act (Torrens)
//! - **Quebec**: Civil law system (Registry of Real Rights)
//!
//! ## Key Cases
//!
//! - **Tsilhqot'in Nation v BC** [2014] SCC 44: Aboriginal title test
//! - **Haida Nation v BC** [2004] SCC 73: Duty to consult
//! - **Delgamuukw v BC** [1997] SCC: Aboriginal title content
//! - **R v Sparrow** [1990] SCC: Aboriginal rights framework

mod aboriginal;
mod error;
mod types;

pub use aboriginal::{
    AboriginalTitleAnalyzer, AboriginalTitleFacts, AboriginalTitleResult, ClaimStrength,
    ConsultationAnalyzer, ConsultationFacts, ConsultationResult, ConsultationStep,
    ContinuityFactor, ExclusivityFactor, ImpactSeverity, OccupationEvidence, OccupationFactor,
    TreatyStatus,
};
pub use error::{PropertyError, PropertyResult};
pub use types::{
    AboriginalTitleElement, AboriginalTitleStatus, CoOwnershipType, ConsultationLevel,
    ConsultationTrigger, ConveyancingStage, EasementCreation, EasementType, EstateType,
    InfringementJustification, InterestInLand, LandRegistrationSystem, LienType, PropertyArea,
    PropertyCase, PropertyType, StandardCondition, TenancyPeriod, TitleAssurance, TitleDefect,
    TitleException,
};

// Re-export legalis-core types
pub use legalis_core::{Effect, EffectType, Statute};

use crate::common::Province;

// ============================================================================
// Statute Builders
// ============================================================================

/// Create provincial land titles statute
pub fn create_land_titles_act(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => ("ON-LTA", "Land Titles Act, RSO 1990, c L.5"),
        Province::BritishColumbia => ("BC-LTA", "Land Title Act, RSBC 1996, c 250"),
        Province::Alberta => ("AB-LTA", "Land Titles Act, RSA 2000, c L-4"),
        Province::Saskatchewan => ("SK-LTA", "Land Titles Act, 2000, SS 2000, c L-5.1"),
        Province::Manitoba => ("MB-RPA", "Real Property Act, CCSM c R30"),
        Province::NovaScotia => ("NS-LRA", "Land Registration Act, SNS 2001, c 6"),
        Province::NewBrunswick => ("NB-LTA", "Land Titles Act, SNB 1981, c L-1.1"),
        Province::NewfoundlandLabrador => {
            ("NL-RPA", "Registration of Deeds Act, RSNL 1990, c R-10")
        }
        Province::PrinceEdwardIsland => ("PE-RPA", "Real Property Act, RSPEI 1988, c R-3"),
        Province::Quebec => (
            "QC-CCQ-REG",
            "Civil Code of Quebec - Book Nine: Publication of Rights",
        ),
        _ => ("LTA", "Land Titles Act"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Grant,
            "Provincial land titles/registration statute establishing the system for \
             registering interests in land. Provides title insurance (Torrens) or \
             priority based on registration (Registry).",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create Condominium/Strata Act
pub fn create_condominium_act(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => ("ON-CA", "Condominium Act, 1998, SO 1998, c 19"),
        Province::BritishColumbia => ("BC-SPA", "Strata Property Act, SBC 1998, c 43"),
        Province::Alberta => ("AB-CA", "Condominium Property Act, RSA 2000, c C-22"),
        Province::Quebec => ("QC-CCQ-COP", "Civil Code of Quebec - Divided Co-ownership"),
        _ => ("CA", "Condominium Act"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Grant,
            "Provincial condominium/strata legislation governing creation, registration, \
             governance, and management of condominium corporations. Establishes unit \
             boundaries, common elements, and owner rights/obligations.",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create Construction Lien/Builders Lien Act
pub fn create_construction_lien_act(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => ("ON-CLA", "Construction Act, RSO 1990, c C.30"),
        Province::BritishColumbia => ("BC-BLA", "Builders Lien Act, SBC 1997, c 45"),
        Province::Alberta => ("AB-BLA", "Builders' Lien Act, RSA 2000, c B-7"),
        Province::Quebec => ("QC-CCQ-HYP", "Civil Code of Quebec - Legal Hypothecs"),
        _ => ("CLA", "Construction Lien Act"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Grant,
            "Provincial construction/builders lien legislation providing security \
             for suppliers, contractors, and workers who improve land. Establishes \
             lien rights, holdback requirements, and enforcement procedures.",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create Planning Act / Land Use statute
pub fn create_planning_act(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => ("ON-PA", "Planning Act, RSO 1990, c P.13"),
        Province::BritishColumbia => ("BC-LGCA", "Local Government Act, RSBC 2015, c 1"),
        Province::Alberta => ("AB-MGA", "Municipal Government Act, RSA 2000, c M-26"),
        _ => ("PA", "Planning Act"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Grant,
            "Provincial planning legislation governing land use, zoning, subdivision, \
             and development. Establishes official plans, zoning by-laws, and \
             development approval processes.",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create Expropriation Act
pub fn create_expropriation_act(province: &Province) -> Statute {
    let (id, title) = match province {
        Province::Ontario => ("ON-EA", "Expropriations Act, RSO 1990, c E.26"),
        Province::BritishColumbia => ("BC-EA", "Expropriation Act, RSBC 1996, c 125"),
        Province::Alberta => ("AB-EA", "Expropriation Act, RSA 2000, c E-13"),
        _ => ("EA", "Expropriation Act"),
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Grant,
            "Provincial expropriation legislation governing compulsory acquisition \
             of land for public purposes. Establishes procedures, compensation \
             requirements (market value plus disturbance), and appeal rights.",
        ),
    )
    .with_jurisdiction(province.abbreviation())
}

/// Create federal Indian Act (reserve lands)
pub fn create_indian_act() -> Statute {
    Statute::new(
        "IA",
        "Indian Act, RSC 1985, c I-5",
        Effect::new(
            EffectType::Grant,
            "Federal statute governing reserve lands, band governance, and status \
             Indians. Section 89: personal property on reserve exempt from seizure. \
             Section 28: no interest in reserve land without ministerial consent.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create federal First Nations Land Management Act
pub fn create_fnlma() -> Statute {
    Statute::new(
        "FNLMA",
        "First Nations Land Management Act, SC 1999, c 24",
        Effect::new(
            EffectType::Grant,
            "Federal statute allowing First Nations to opt out of Indian Act land \
             management provisions. Enables development of land codes for \
             self-governance of reserve lands.",
        ),
    )
    .with_jurisdiction("CA-FED")
}

/// Create all property law statutes for a province
pub fn create_property_statutes(province: &Province) -> Vec<Statute> {
    vec![
        create_land_titles_act(province),
        create_condominium_act(province),
        create_construction_lien_act(province),
        create_planning_act(province),
        create_expropriation_act(province),
        create_indian_act(),
        create_fnlma(),
    ]
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_land_titles_act() {
        let statute = create_land_titles_act(&Province::Ontario);
        assert!(statute.title.contains("Land Titles Act"));
    }

    #[test]
    fn test_create_bc_land_title_act() {
        let statute = create_land_titles_act(&Province::BritishColumbia);
        assert!(statute.title.contains("Land Title Act"));
    }

    #[test]
    fn test_create_condominium_act() {
        let statute = create_condominium_act(&Province::Ontario);
        assert!(statute.title.contains("Condominium Act"));
    }

    #[test]
    fn test_create_strata_act() {
        let statute = create_condominium_act(&Province::BritishColumbia);
        assert!(statute.title.contains("Strata Property Act"));
    }

    #[test]
    fn test_create_construction_lien_act() {
        let statute = create_construction_lien_act(&Province::Ontario);
        assert!(statute.title.contains("Construction Act"));
    }

    #[test]
    fn test_create_planning_act() {
        let statute = create_planning_act(&Province::Ontario);
        assert!(statute.title.contains("Planning Act"));
    }

    #[test]
    fn test_create_expropriation_act() {
        let statute = create_expropriation_act(&Province::Alberta);
        assert!(statute.title.contains("Expropriation Act"));
    }

    #[test]
    fn test_create_indian_act() {
        let statute = create_indian_act();
        assert!(statute.title.contains("Indian Act"));
    }

    #[test]
    fn test_create_fnlma() {
        let statute = create_fnlma();
        assert!(statute.title.contains("First Nations Land Management"));
    }

    #[test]
    fn test_create_property_statutes() {
        let statutes = create_property_statutes(&Province::Ontario);
        assert!(statutes.len() >= 7);
    }

    #[test]
    fn test_quebec_registration() {
        let statute = create_land_titles_act(&Province::Quebec);
        assert!(statute.title.contains("Civil Code of Quebec"));
    }
}
