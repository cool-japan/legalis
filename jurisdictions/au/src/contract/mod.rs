//! Australian Contract Law Module
//!
//! Comprehensive implementation of Australian contract law including:
//! - Formation (agreement, consideration, intention, capacity, certainty, legality)
//! - Terms (express, implied, incorporation, classification)
//! - Australian Consumer Law (ACL) consumer guarantees and protections
//! - Breach and remedies (damages, specific performance, termination)
//! - Vitiating factors (misrepresentation, mistake, duress, unconscionability)
//! - Unconscionable conduct (common law and statutory under ACL ss.20-22)
//! - Building contracts (Home Building Act, statutory warranties, defects liability)
//!
//! ## Key Features
//!
//! ### Consumer Protection (ACL)
//! The Australian Consumer Law provides:
//! - Consumer guarantees (ss.54-65)
//! - Protection against misleading conduct (s.18)
//! - Unfair contract terms provisions (Pt 2-3)
//! - Unconscionable conduct provisions (ss.20-22)
//!
//! ### Building and Construction
//! Building contract law coverage:
//! - Home Building Act statutory warranties
//! - Progress payments and Security of Payment
//! - Practical completion (Multiplex v Honeywell)
//! - Defects liability periods
//!
//! ### Key Cases
//! - Waltons Stores v Maher (1988) - Proprietary estoppel
//! - Ermogenous v Greek Orthodox (2002) - Intention
//! - Hungry Jack's v Burger King (2001) - Good faith
//! - ACCC v CG Berbatis (2003) - Statutory unconscionability
//! - Blomley v Ryan (1956) - Common law unconscionability
//! - Commercial Bank v Amadio (1983) - Special disadvantage
//! - Bellgrove v Eldridge (1954) - Cost of rectification
//! - Multiplex v Honeywell (2007) - Practical completion

pub mod acl;
pub mod breach;
pub mod building;
pub mod error;
pub mod formation;
pub mod types;
pub mod unconscionable;

pub use acl::{
    ConsumerAnalyzer, ConsumerBasis, ConsumerRemedy, ConsumerStatusFacts, ConsumerStatusResult,
    GuaranteeAnalyzer, GuaranteeFacts, GuaranteeResult, MisleadingConductAnalyzer,
    MisleadingConductFacts, MisleadingConductResult, UnfairTermFacts, UnfairTermResult,
    UnfairTermsAnalyzer,
};
pub use breach::{
    BreachAnalyzer, BreachFacts, BreachResult, DamagesAnalyzer, DamagesFacts, DamagesResult,
};
pub use building::{
    BuildingContractAnalyzer, BuildingContractFacts, BuildingContractResult, BuildingContractType,
    BuildingRemedy, ComplianceIssue, DefectClassification, DefectsLiabilityAnalyzer,
    DefectsLiabilityFacts, DefectsLiabilityResult, PaymentClaimStatus, PracticalCompletionAnalyzer,
    PracticalCompletionFacts, PracticalCompletionResult, PracticalCompletionStatus,
    ProgressPaymentAnalyzer, ProgressPaymentBasis, ProgressPaymentFacts, ProgressPaymentResult,
    StatutoryWarranty, StatutoryWarrantyAnalyzer, WarrantyBreachFacts, WarrantyBreachResult,
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
pub use unconscionable::{
    CommonLawUnconscionabilityAnalyzer, CommonLawUnconscionabilityFacts,
    CommonLawUnconscionabilityResult, PowerImbalance, ProceduralFactor, SmallBusinessConsideration,
    SpecialDisadvantage, StatutoryUnconscionabilityAnalyzer, StatutoryUnconscionabilityFacts,
    StatutoryUnconscionabilityResult, SubstantiveFactor, UnconscionabilityAnalysisResult,
    UnconscionabilityAnalyzer, UnconscionabilityType, UnconscionableElement, UnconscionableRemedy,
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
