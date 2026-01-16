//! Australian Constitutional Law Module
//!
//! Comprehensive implementation of Australian constitutional law including:
//! - Commonwealth legislative powers (s.51)
//! - Express constitutional rights (ss.80, 92, 116, 117, 51(xxxi))
//! - Implied constitutional rights (political communication, voting)
//! - Separation of powers and Kable doctrine
//! - Federal-state relations (s.109 inconsistency, Melbourne Corporation)
//!
//! ## Constitutional Framework
//!
//! Australia's Constitution establishes:
//! 1. A federal system with powers divided between Commonwealth and states
//! 2. Separation of powers (strict at Commonwealth level)
//! 3. Limited express rights protection
//! 4. Implied rights derived from constitutional structure
//!
//! ## Key Constitutional Principles
//!
//! - **Engineers Case (1920)**: Constitution interpreted according to its terms
//! - **Cole v Whitfield (1988)**: s.92 only prohibits protectionism
//! - **Lange v ABC (1997)**: Implied freedom of political communication
//! - **Work Choices (2006)**: Broad interpretation of corporations power

pub mod commonwealth;
pub mod error;
pub mod implied;
pub mod types;

pub use commonwealth::{
    CharacterizationAnalyzer, CharacterizationFacts, ExpressRightsAnalyzer, ExpressRightsFacts,
    ExpressRightsResult, InconsistencyAnalyzer, InconsistencyFacts, MelbourneCorporationAnalyzer,
    MelbourneCorporationFacts,
};
pub use error::{ConstitutionalError, ConstitutionalResult};
pub use implied::{
    ImpliedRightsAnalyzer, KableAnalyzer, KableFacts, KableResult, PoliticalCommunicationAnalyzer,
    PoliticalCommunicationFacts, PoliticalCommunicationResult, RightToVoteAnalyzer,
    VotingRightsFacts, VotingRightsResult,
};
pub use types::{
    CharacterizationResult, CommonwealthPower, ConstitutionalCase, ConstitutionalProvision,
    ExpressRight, GovernmentBranch, ImpliedRight, InconsistencyAnalysis, InconsistencyType,
    MelbourneCorporationAnalysis, SeparationDoctrine, StatePower,
};

use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Australian Constitution statute
pub fn create_constitution_statute() -> Statute {
    Statute::new(
        "AU-CONST-1901",
        "Commonwealth of Australia Constitution Act 1901",
        Effect::new(
            EffectType::Grant,
            "Establishes the Commonwealth of Australia and federal system of government",
        ),
    )
    .with_jurisdiction("AU-FED")
}

/// Create statute for a Commonwealth legislative power
pub fn create_power_statute(power: &CommonwealthPower) -> Statute {
    Statute::new(
        format!(
            "AU-CONST-{}",
            power.section().replace('.', "-").replace(['(', ')'], "")
        ),
        format!("Constitution {}", power.section()),
        Effect::new(
            EffectType::Grant,
            format!(
                "Grants Commonwealth legislative power over {}",
                match power {
                    CommonwealthPower::TradeAndCommerce =>
                        "trade and commerce with other countries and among states",
                    CommonwealthPower::Taxation => "taxation (but not discriminate between states)",
                    CommonwealthPower::Defence => "naval and military defence",
                    CommonwealthPower::Corporations =>
                        "foreign, trading, and financial corporations",
                    CommonwealthPower::ExternalAffairs =>
                        "external affairs and treaty implementation",
                    CommonwealthPower::Marriage => "marriage",
                    CommonwealthPower::DivorceMatrimonial => "divorce and matrimonial causes",
                    CommonwealthPower::Immigration => "naturalization and aliens",
                    _ => "the specified subject matter",
                }
            ),
        ),
    )
    .with_jurisdiction("AU-FED")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_constitution_statute() {
        let statute = create_constitution_statute();
        assert_eq!(statute.id, "AU-CONST-1901");
        assert!(
            statute
                .jurisdiction
                .as_ref()
                .is_some_and(|j| j.contains("AU-FED"))
        );
    }

    #[test]
    fn test_create_power_statute() {
        let statute = create_power_statute(&CommonwealthPower::Corporations);
        assert!(statute.id.contains("51"));
        assert!(statute.effect.description.contains("corporations"));
    }

    #[test]
    fn test_political_communication_analyzer() {
        let facts = PoliticalCommunicationFacts {
            restricts_political_discussion: true,
            serves_legitimate_end: true,
            compatible_with_representative_government: true,
            suitable_means: true,
            necessary_means: true,
            adequate_in_balance: true,
            reasonably_appropriate_and_adapted: true,
            ..Default::default()
        };

        let result = PoliticalCommunicationAnalyzer::analyze("Test Act", &facts);
        assert!(result.burdens_communication);
        assert!(result.justified);
        assert!(!result.invalid);
    }
}
