//! Canada Reasoning Module
//!
//! Integration with legalis-core for Canadian legal reasoning,
//! constitutional verification, and cross-jurisdiction interoperability.
//!
//! ## Key Components
//!
//! - **Reasoning Engine**: Load and query Canadian statutes
//! - **Constitutional Verifier**: Check Charter and division of powers
//! - **Interoperability**: Handle inter-provincial conflicts of law
//!
//! ## Constitutional Framework
//!
//! Canadian constitutional review involves:
//! 1. **Charter Analysis**: Does law limit Charter rights? Is it saved by s.1?
//! 2. **Division of Powers**: Is law within federal or provincial jurisdiction?
//! 3. **Aboriginal Rights**: Does law affect s.35 rights? Was consultation adequate?
//!
//! ## Bijural Considerations
//!
//! The interop module handles:
//! - Common law vs. civil law mapping
//! - Inter-provincial choice of law
//! - Federal-provincial concurrent jurisdiction

mod engine;
mod interop;
mod verifier;

pub use engine::{
    CanadianReasoningEngine, ConflictType, ReasoningJurisdiction, ReasoningQuery, ReasoningResult,
    StatuteConflict,
};
pub use interop::{
    ApplicableLawResult, CanadianInterop, CivilLawConcept, CommonLawConcept, GoverningLaw,
    InterProvincialFacts, LegalAreaType,
};
pub use verifier::{
    ConstitutionalIssue, ConstitutionalVerifier, IssueType, VerificationContext, VerificationResult,
};

// Re-export legalis-core types
pub use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Convenience Functions
// ============================================================================

use crate::common::Province;

/// Create a reasoning engine for a province with all applicable statutes
pub fn create_provincial_engine(province: Province) -> CanadianReasoningEngine {
    let mut engine = CanadianReasoningEngine::new(ReasoningJurisdiction::Combined(province));
    engine.load_all();
    engine
}

/// Create a federal-only reasoning engine
pub fn create_federal_engine() -> CanadianReasoningEngine {
    let mut engine = CanadianReasoningEngine::new(ReasoningJurisdiction::Federal);
    engine.load_federal_statutes();
    engine
}

/// Verify a statute for constitutional compliance
pub fn verify_statute(statute: &Statute, context: &VerificationContext) -> VerificationResult {
    ConstitutionalVerifier::verify(statute, context)
}

/// Determine applicable law for inter-provincial matter
pub fn determine_applicable_law(facts: &InterProvincialFacts) -> ApplicableLawResult {
    CanadianInterop::determine_applicable_law(facts)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provincial_engine() {
        let engine = create_provincial_engine(Province::Ontario);
        assert!(!engine.statutes().is_empty());
    }

    #[test]
    fn test_create_federal_engine() {
        let engine = create_federal_engine();
        assert!(!engine.statutes().is_empty());
    }

    #[test]
    fn test_verify_statute() {
        let statute = legalis_core::Statute::new(
            "TEST",
            "Test Statute",
            legalis_core::Effect::new(legalis_core::EffectType::Grant, "Test grant"),
        );
        let context = VerificationContext::default();
        let result = verify_statute(&statute, &context);
        assert!(result.valid);
    }

    #[test]
    fn test_determine_applicable_law() {
        let facts = InterProvincialFacts {
            legal_area: LegalAreaType::Contract,
            connected_provinces: vec![Province::Ontario],
            ..Default::default()
        };
        let result = determine_applicable_law(&facts);
        assert!(matches!(result.governing_law, GoverningLaw::Provincial(_)));
    }
}
