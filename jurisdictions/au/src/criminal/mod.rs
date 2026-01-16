//! Australian Criminal Law
//!
//! Implementation of Australian criminal law including:
//! - Criminal Code Act 1995 (Cth)
//! - State criminal codes
//! - Sentencing principles
//!
//! ## Key Legislation
//!
//! - Criminal Code Act 1995 (Cth)
//! - Crimes Act 1914 (Cth)
//! - State criminal codes (Qld, WA, Tas, NT)
//! - Crimes Acts (NSW, Vic, SA, ACT)
//!
//! ## Key Cases
//!
//! - He Kaw Teh v R (1985) - Presumption of mens rea
//! - Zecevic v DPP (Vic) (1987) - Self-defence test
//! - Veen v R (No 2) (1988) - Sentencing proportionality

pub mod commonwealth;
pub mod sentencing;
pub mod types;

// Re-export commonly used types
pub use commonwealth::{
    DefenceResult, LiabilityStatus, OffenceAnalyzer, OffenceFacts, OffenceResult,
};
pub use sentencing::{
    SentenceRecommendation, SentencingAnalyzer, SentencingFacts, SentencingRange, SentencingResult,
};
pub use types::{
    AggravatingFactor, AssaultGrade, CriminalCase, CriminalJurisdiction, Defence, DrugOffenceType,
    ElementType, FaultElement, FraudType, MitigatingFactor, OffenceCategory, OffenceType,
    PhysicalElement, SentenceType, SentencingPurpose, SexualOffenceType,
};

use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Criminal Code Act 1995 (Cth) statute
pub fn create_criminal_code_act() -> Statute {
    Statute::new(
        "AU-CCA-1995",
        "Criminal Code Act 1995 (Cth)",
        Effect::new(
            EffectType::Prohibition,
            "Federal criminal offences and general principles of criminal responsibility",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create Crimes Act 1914 (Cth) statute
pub fn create_crimes_act() -> Statute {
    Statute::new(
        "AU-CA-1914",
        "Crimes Act 1914 (Cth)",
        Effect::new(
            EffectType::Prohibition,
            "Federal criminal offences and investigation powers",
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
    fn test_create_criminal_code_act() {
        let statute = create_criminal_code_act();
        assert!(statute.id.contains("CCA"));
    }

    #[test]
    fn test_create_crimes_act() {
        let statute = create_crimes_act();
        assert!(statute.id.contains("CA"));
    }

    #[test]
    fn test_offence_analysis() {
        let facts = OffenceFacts {
            offence_description: "Fraud".to_string(),
            conduct_occurred: true,
            required_fault_element: FaultElement::Intention,
            fault_element_proved: true,
            ..Default::default()
        };

        let result = OffenceAnalyzer::analyze_offence(&facts);
        assert_eq!(result.liability_status, LiabilityStatus::Guilty);
    }

    #[test]
    fn test_sentencing_analysis() {
        let facts = SentencingFacts {
            offence_category: OffenceCategory::Indictable,
            maximum_penalty_months: 60,
            early_guilty_plea: true,
            rehabilitation_prospects_good: true,
            ..Default::default()
        };

        let result = SentencingAnalyzer::analyze(&facts);
        assert!(!result.available_sentences.is_empty());
    }
}
