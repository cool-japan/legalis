//! Australian Contract Law Module
//!
//! Comprehensive implementation of Australian contract law including:
//! - Formation (agreement, consideration, intention, capacity, certainty, legality)
//! - Terms (express, implied, incorporation, classification)
//! - Australian Consumer Law (ACL) consumer guarantees and protections
//! - Breach and remedies (damages, specific performance, termination)
//! - Vitiating factors (misrepresentation, mistake, duress, unconscionability)
//!
//! ## Key Features
//!
//! ### Consumer Protection (ACL)
//! The Australian Consumer Law provides:
//! - Consumer guarantees (ss.54-65)
//! - Protection against misleading conduct (s.18)
//! - Unfair contract terms provisions (Pt 2-3)
//! - Unconscionable conduct provisions (ss.50-51)
//!
//! ### Key Cases
//! - Waltons Stores v Maher (1988) - Proprietary estoppel
//! - Ermogenous v Greek Orthodox (2002) - Intention
//! - Hungry Jack's v Burger King (2001) - Good faith
//! - ACCC v CG Berbatis (2003) - Unconscionability

pub mod acl;
pub mod breach;
pub mod error;
pub mod formation;
pub mod types;

pub use acl::{
    ConsumerAnalyzer, ConsumerBasis, ConsumerRemedy, ConsumerStatusFacts, ConsumerStatusResult,
    GuaranteeAnalyzer, GuaranteeFacts, GuaranteeResult, MisleadingConductAnalyzer,
    MisleadingConductFacts, MisleadingConductResult, UnfairTermFacts, UnfairTermResult,
    UnfairTermsAnalyzer,
};
pub use breach::{
    BreachAnalyzer, BreachFacts, BreachResult, DamagesAnalyzer, DamagesFacts, DamagesResult,
};
pub use error::{ContractError, ContractResult};
pub use formation::{
    ClassificationFacts, FormationAnalyzer, FormationFacts, FormationResult, IncorporationFacts,
    IncorporationMethod, IncorporationResult, TermsAnalyzer,
};
pub use types::{
    AcceptanceMode, BreachType, ConsiderationType, ConsumerGuarantee, ContractRemedy, DamagesType,
    DuressType, ExclusionClauseStatus, FormationElement, MisrepresentationType, MistakeType,
    OfferType, ProhibitedConduct, TermClassification, TermType, UndueInfluenceType, UnfairTermType,
    VitiatingFactor,
};

use crate::common::StateTerritory;
use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Australian Consumer Law statute
pub fn create_acl_statute() -> Statute {
    Statute::new(
        "AU-ACL-2010",
        "Australian Consumer Law (Schedule 2 to Competition and Consumer Act 2010)",
        Effect::new(
            EffectType::Grant,
            "Provides consumer protection including guarantees, misleading conduct prohibition, and unfair terms",
        ),
    )
    .with_jurisdiction("AU-FED")
}

/// Create Competition and Consumer Act statute
pub fn create_cca_statute() -> Statute {
    Statute::new(
        "AU-CCA-2010",
        "Competition and Consumer Act 2010",
        Effect::new(
            EffectType::Grant,
            "Promotes competition and fair trading, protects consumers",
        ),
    )
    .with_jurisdiction("AU-FED")
}

/// Create state Sale of Goods Act equivalent
pub fn create_sale_of_goods_statute(state: &StateTerritory) -> Statute {
    Statute::new(
        format!("AU-{}-SOG", state.abbreviation()),
        format!("Sale of Goods Act ({})", state.full_name()),
        Effect::new(
            EffectType::Grant,
            "Governs contracts for sale of goods, implied terms and conditions",
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
    fn test_create_acl_statute() {
        let statute = create_acl_statute();
        assert_eq!(statute.id, "AU-ACL-2010");
        assert!(statute.effect.description.contains("consumer"));
    }

    #[test]
    fn test_formation_analysis() {
        let facts = FormationFacts {
            offer_made: true,
            acceptance_communicated: true,
            consideration_present: true,
            commercial_context: true,
            essential_terms_agreed: true,
            ..Default::default()
        };

        let result = FormationAnalyzer::analyze(&facts);
        assert!(result.contract_formed);
    }

    #[test]
    fn test_acl_guarantee_breach() {
        let facts = GuaranteeFacts {
            fit_for_normal_purpose: false,
            safe: true,
            durable: true,
            acceptable_appearance_finish: true,
            free_from_defects: true,
            ..Default::default()
        };

        let result = GuaranteeAnalyzer::analyze(ConsumerGuarantee::AcceptableQuality, &facts);
        assert!(result.breached);
    }
}
