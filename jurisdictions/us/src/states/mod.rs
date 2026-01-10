//! State-specific legal variations for the United States.
//!
//! This module provides functionality for modeling, comparing, and analyzing legal rules
//! across the 50 US states, which often have significant variations despite sharing a
//! Common Law heritage (except Louisiana, which follows Civil Law).
//!
//! ## Phase 1 Priority States
//!
//! The initial implementation focuses on 5 states:
//! - **California (CA)**: Largest economy, pure comparative negligence, interest analysis
//! - **New York (NY)**: Financial capital, Cardozo legacy, highest appellate influence
//! - **Texas (TX)**: Large economy, modified comparative 51%, tort reform
//! - **Louisiana (LA)**: ONLY Civil Law state, French/Spanish heritage
//! - **Florida (FL)**: Large population, Stand Your Ground, pure comparative
//!
//! ## Core Functionality
//!
//! - **Types**: Core data structures (`StateId`, `StateLawVariation`, `LegalTopic`, `StateRule`)
//! - **Registry**: State metadata and lookups (`StateRegistry`, `StateMetadata`)
//! - **State Modules**: State-specific implementations (california, new_york, texas, louisiana, florida)
//! - **Comparator**: Cross-state comparison engine (coming in Phase 1C)

// ===== Core Types and Registry =====
pub mod registry;
pub mod types;

// ===== State-Specific Modules (Phase 1B) =====
pub mod california;
pub mod florida;
pub mod louisiana;
pub mod new_york;
pub mod texas;

// ===== State-Specific Modules (Phase 2 - Tier 1) =====
pub mod georgia;
pub mod illinois;
pub mod massachusetts;
pub mod michigan;
pub mod new_jersey;
pub mod ohio;
pub mod pennsylvania;
pub mod washington;

// ===== State-Specific Modules (Phase 2 - Tier 2) =====
pub mod arizona;
pub mod colorado;
pub mod indiana;
pub mod maryland;
pub mod minnesota;
pub mod missouri;
pub mod north_carolina;
pub mod tennessee;
pub mod virginia;
pub mod wisconsin;

// ===== State-Specific Modules (Phase 2 - Tier 3) =====
pub mod alabama;
pub mod alaska;
pub mod arkansas;
pub mod connecticut;
pub mod delaware;
pub mod district_of_columbia;
pub mod hawaii;
pub mod idaho;
pub mod iowa;
pub mod kansas;
pub mod kentucky;
pub mod maine;
pub mod mississippi;
pub mod montana;
pub mod nebraska;
pub mod nevada;
pub mod new_hampshire;
pub mod new_mexico;
pub mod north_dakota;
pub mod oklahoma;
pub mod oregon;
pub mod rhode_island;
pub mod south_carolina;
pub mod south_dakota;
pub mod utah;
pub mod vermont;
pub mod west_virginia;
pub mod wyoming;

// ===== Comparison Engine (Phase 1C) =====
pub mod comparator;

// ===== Re-exports =====
pub use registry::{CourtStructure, GeographicRegion, StateMetadata, StateRegistry};
pub use types::{
    CaseReference, CauseOfAction, DamagesType, LegalTopic, LegalTradition, StateId,
    StateLawVariation, StateRule, StatuteReference,
};

// State modules - Phase 1
pub use california::CaliforniaLaw;
pub use florida::FloridaLaw;
pub use louisiana::{ComparisonReport, LouisianaLaw};
pub use new_york::NewYorkLaw;
pub use texas::TexasLaw;

// State modules - Phase 2 Tier 1
pub use georgia::GeorgiaLaw;
pub use illinois::IllinoisLaw;
pub use massachusetts::MassachusettsLaw;
pub use michigan::MichiganLaw;
pub use new_jersey::NewJerseyLaw;
pub use ohio::OhioLaw;
pub use pennsylvania::PennsylvaniaLaw;
pub use washington::WashingtonLaw;

// State modules - Phase 2 Tier 2
pub use arizona::ArizonaLaw;
pub use colorado::ColoradoLaw;
pub use indiana::IndianaLaw;
pub use maryland::MarylandLaw;
pub use minnesota::MinnesotaLaw;
pub use missouri::MissouriLaw;
pub use north_carolina::NorthCarolinaLaw;
pub use tennessee::TennesseeLaw;
pub use virginia::VirginiaLaw;
pub use wisconsin::WisconsinLaw;

// State modules - Phase 2 Tier 3
pub use alabama::AlabamaLaw;
pub use alaska::AlaskaLaw;
pub use arkansas::ArkansasLaw;
pub use connecticut::ConnecticutLaw;
pub use delaware::DelawareLaw;
pub use district_of_columbia::DistrictOfColumbiaLaw;
pub use hawaii::HawaiiLaw;
pub use idaho::IdahoLaw;
pub use iowa::IowaLaw;
pub use kansas::KansasLaw;
pub use kentucky::KentuckyLaw;
pub use maine::MaineLaw;
pub use mississippi::MississippiLaw;
pub use montana::MontanaLaw;
pub use nebraska::NebraskaLaw;
pub use nevada::NevadaLaw;
pub use new_hampshire::NewHampshireLaw;
pub use new_mexico::NewMexicoLaw;
pub use north_dakota::NorthDakotaLaw;
pub use oklahoma::OklahomaLaw;
pub use oregon::OregonLaw;
pub use rhode_island::RhodeIslandLaw;
pub use south_carolina::SouthCarolinaLaw;
pub use south_dakota::SouthDakotaLaw;
pub use utah::UtahLaw;
pub use vermont::VermontLaw;
pub use west_virginia::WestVirginiaLaw;
pub use wyoming::WyomingLaw;

// Comparison engine
pub use comparator::{StateComparison, StateLawComparator};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_1_states_accessible() {
        // Verify we can create Phase 1 state IDs
        let ca = StateId::california();
        let ny = StateId::new_york();
        let tx = StateId::texas();
        let la = StateId::louisiana();
        let fl = StateId::florida();

        assert_eq!(ca.code, "CA");
        assert_eq!(ny.code, "NY");
        assert_eq!(tx.code, "TX");
        assert_eq!(la.code, "LA");
        assert_eq!(fl.code, "FL");
    }

    #[test]
    fn test_louisiana_unique() {
        let la = StateId::louisiana();
        assert_eq!(la.legal_tradition, LegalTradition::CivilLaw);

        // All others should be Common Law
        let ca = StateId::california();
        let ny = StateId::new_york();
        let tx = StateId::texas();
        let fl = StateId::florida();

        assert_eq!(ca.legal_tradition, LegalTradition::CommonLaw);
        assert_eq!(ny.legal_tradition, LegalTradition::CommonLaw);
        assert_eq!(tx.legal_tradition, LegalTradition::CommonLaw);
        assert_eq!(fl.legal_tradition, LegalTradition::CommonLaw);
    }

    #[test]
    fn test_registry_integration() {
        let registry = StateRegistry::new();

        // All Phase 1 states should be registered
        assert!(registry.get("CA").is_some());
        assert!(registry.get("NY").is_some());
        assert!(registry.get("TX").is_some());
        assert!(registry.get("LA").is_some());
        assert!(registry.get("FL").is_some());

        // Non-Phase-1 states should not be registered yet
        assert!(registry.get("MA").is_none());
        assert!(registry.get("IL").is_none());
    }

    #[test]
    fn test_jurisdiction_strings() {
        assert_eq!(StateId::california().jurisdiction_string(), "US-CA");
        assert_eq!(StateId::new_york().jurisdiction_string(), "US-NY");
        assert_eq!(StateId::texas().jurisdiction_string(), "US-TX");
        assert_eq!(StateId::louisiana().jurisdiction_string(), "US-LA");
        assert_eq!(StateId::florida().jurisdiction_string(), "US-FL");
    }
}
