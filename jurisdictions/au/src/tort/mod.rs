//! Australian Tort Law Module
//!
//! Comprehensive implementation of Australian tort law including:
//! - Negligence (with Civil Liability Act reforms)
//! - Defamation (Uniform Defamation Laws)
//! - Nuisance (private and public)
//!
//! ## Civil Liability Act Reforms
//!
//! Following the Ipp Report (2002), all states adopted Civil Liability Acts:
//! - Modified breach standard (s.9 calculus)
//! - Obvious risk and inherent risk defences
//! - Recreational activity provisions
//! - Mental harm limitations (normal fortitude)
//! - Good Samaritan protections
//! - Damages caps
//!
//! ## Key Cases
//!
//! - Sullivan v Moody (2001) - Novel duty of care analysis
//! - Rogers v Whitaker (1992) - Medical disclosure
//! - Wyong v Shirt (1980) - Breach standard
//! - March v Stramare (1991) - Causation
//! - Tame v NSW (2002) - Mental harm

pub mod defamation;
pub mod error;
pub mod negligence;
pub mod types;

pub use defamation::{
    DamagesFacts as DefamationDamagesFacts, DamagesResult as DefamationDamagesResult,
    DefamationAnalyzer, DefamationDamages, DefamationFacts, DefamationResult, PreliminaryMatters,
};
pub use error::{TortError, TortResult};
pub use negligence::{
    BreachAnalyzer, BreachFacts, BreachResult, CausationAnalyzer, CausationFacts, CausationResult,
    DutyFacts, DutyOfCareAnalyzer, DutyResult, NegligenceAnalyzer, NegligenceResult,
    SalientFeaturesAnalysis,
};
pub use types::{
    CLADefence, CausationTest, DamagesCaps, DefamationDefence, DefamationElement, DutyCategory,
    ImputationType, MentalHarmCategory, NovusActus, NuisanceDefence, NuisanceInterference,
    NuisanceType, ObviousRisk, RecognizedDuty, SalientFeature, StandardOfCareFactor,
    TortDamagesType,
};

use crate::common::StateTerritory;
use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Civil Liability Act statute for a state
pub fn create_civil_liability_act(state: &StateTerritory) -> Statute {
    let (id, title) = match state {
        StateTerritory::NewSouthWales => ("AU-NSW-CLA-2002", "Civil Liability Act 2002 (NSW)"),
        StateTerritory::Victoria => ("AU-VIC-WRONGS-1958", "Wrongs Act 1958 (Vic)"),
        StateTerritory::Queensland => ("AU-QLD-CLA-2003", "Civil Liability Act 2003 (Qld)"),
        StateTerritory::SouthAustralia => ("AU-SA-CLA-1936", "Civil Liability Act 1936 (SA)"),
        StateTerritory::WesternAustralia => ("AU-WA-CLA-2002", "Civil Liability Act 2002 (WA)"),
        StateTerritory::Tasmania => ("AU-TAS-CLA-2002", "Civil Liability Act 2002 (Tas)"),
        StateTerritory::NorthernTerritory => (
            "AU-NT-PILD-2003",
            "Personal Injuries (Liabilities and Damages) Act 2003 (NT)",
        ),
        StateTerritory::AustralianCapitalTerritory => {
            ("AU-ACT-WRONGS-2002", "Civil Law (Wrongs) Act 2002 (ACT)")
        }
    };

    Statute::new(
        id,
        title,
        Effect::new(
            EffectType::Obligation,
            "Reforms personal injury law: breach standard, obvious risk, damages caps",
        ),
    )
    .with_jurisdiction(format!("AU-{}", state.abbreviation()))
}

/// Create Uniform Defamation Act statute for a state
pub fn create_defamation_act(state: &StateTerritory) -> Statute {
    Statute::new(
        format!("AU-{}-DEF-2005", state.abbreviation()),
        format!("Defamation Act 2005 ({})", state.full_name()),
        Effect::new(
            EffectType::Grant,
            "Uniform defamation law: serious harm threshold, defences, damages caps",
        ),
    )
    .with_jurisdiction(format!("AU-{}", state.abbreviation()))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_civil_liability_act() {
        let statute = create_civil_liability_act(&StateTerritory::NewSouthWales);
        assert_eq!(statute.id, "AU-NSW-CLA-2002");
    }

    #[test]
    fn test_create_defamation_act() {
        let statute = create_defamation_act(&StateTerritory::Victoria);
        assert!(statute.id.contains("VIC"));
        assert!(statute.id.contains("DEF"));
    }

    #[test]
    fn test_negligence_complete_claim() {
        let duty_facts = DutyFacts {
            professional_client_relationship: true,
            harm_foreseeable: true,
            ..Default::default()
        };

        let breach_facts = BreachFacts {
            risk_not_insignificant: true,
            high_probability_of_harm: true,
            ..Default::default()
        };

        let causation_facts = CausationFacts {
            but_for_negligence_no_harm: true,
            harm_within_scope: true,
            ..Default::default()
        };

        let result = NegligenceAnalyzer::analyze(
            &duty_facts,
            &breach_facts,
            &causation_facts,
            StateTerritory::NewSouthWales,
        );

        assert!(result.liable);
    }

    #[test]
    fn test_defamation_claim() {
        let facts = DefamationFacts {
            matter_published: true,
            publication_to_third_party: true,
            plaintiff_identified: true,
            matter_defamatory: true,
            serious_harm_caused: true,
            ..Default::default()
        };

        let result = DefamationAnalyzer::analyze(&facts);
        assert!(result.defamation_established);
    }
}
