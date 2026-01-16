//! Australian Corporate Law
//!
//! Implementation of Corporations Act 2001 (Cth) including:
//! - Directors duties (Part 2D.1)
//! - Insolvent trading (s.588G)
//! - External administration
//!
//! ## Key Legislation
//!
//! - Corporations Act 2001 (Cth)
//! - ASIC Act 2001 (Cth)
//!
//! ## Key Cases
//!
//! - ASIC v Healey [2011] - Directors duty of care
//! - Shafron v ASIC (2012) - Officers duty

pub mod corporations;
pub mod types;

// Re-export commonly used types
pub use corporations::{
    DirectorsDutiesAnalyzer, DutyFacts, DutyResult, InsolventTradingAnalyzer,
    InsolventTradingFacts, InsolventTradingResult,
};
pub use types::{
    AsicPower, BusinessJudgmentElement, CompanySize, CompanyType, CorporateCase, DirectorsDuty,
    ExternalAdministration, InsolventTradingDefence, LiquidationPriority, TakeoverMethod,
    TakeoverThreshold,
};

use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Corporations Act 2001 statute
pub fn create_corporations_act() -> Statute {
    Statute::new(
        "AU-CA-2001",
        "Corporations Act 2001 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "Corporate governance, directors duties, and company regulation",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create ASIC Act 2001 statute
pub fn create_asic_act() -> Statute {
    Statute::new(
        "AU-ASIC-2001",
        "ASIC Act 2001 (Cth)",
        Effect::new(
            EffectType::Grant,
            "ASIC powers and consumer protection in financial services",
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
    fn test_create_corporations_act() {
        let statute = create_corporations_act();
        assert!(statute.id.contains("CA"));
    }

    #[test]
    fn test_directors_duty_analysis() {
        let facts = DutyFacts {
            exercised_care_diligence: true,
            acted_good_faith: true,
            proper_purpose: true,
            ..Default::default()
        };

        let result = DirectorsDutiesAnalyzer::analyze(&facts);
        assert!(!result.breach_found);
    }

    #[test]
    fn test_insolvent_trading() {
        let facts = InsolventTradingFacts {
            company_insolvent: true,
            debt_incurred: true,
            reasonable_grounds_to_suspect: true,
            was_director_at_time: true,
            ..Default::default()
        };

        let result = InsolventTradingAnalyzer::analyze(&facts);
        assert!(result.liable);
    }
}
