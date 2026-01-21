//! Australian Corporate Law
//!
//! Implementation of Corporations Act 2001 (Cth) including:
//! - Directors duties (Part 2D.1)
//! - Insolvent trading (s.588G)
//! - External administration
//! - Company formation and registration (Chapter 2A)
//! - Share capital (Chapter 2H)
//! - Members' rights (Chapters 2F, 2G)
//! - Oppression remedies (s.232-234)
//!
//! ## Key Legislation
//!
//! - Corporations Act 2001 (Cth)
//! - ASIC Act 2001 (Cth)
//!
//! ## Key Cases
//!
//! - ASIC v Healey \[2011\] - Directors duty of care
//! - Shafron v ASIC (2012) - Officers duty
//! - Re H R Harmer Ltd (1959) - Oppression remedy
//! - Gambotto v WCP Ltd (1995) - Expropriation of minority

pub mod company_law;
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

// Re-export company_law types
pub use company_law::{
    // Company formation
    AmendmentProvisions,
    // Share capital
    BuybackType,
    BuybackValidation,
    CapitalReduction,
    CapitalReductionType,
    ClassRightsVariation,
    Company,
    Constitution,
    DirectorAppointmentMethod,
    DirectorAppointmentRules,
    DividendRights,
    FinancialAssistance,
    FinancialAssistanceExemption,
    FinancialAssistanceType,
    FinancialAssistanceValidation,
    // Members' rights
    MeetingBusiness,
    MeetingProcedures,
    MeetingType,
    Member,
    MemberType,
    MembersMeeting,
    // Oppression
    OppressionApplicant,
    OppressionClaim,
    OppressionGround,
    OppressionRemedy,
    QuorumRequirement,
    RegistrationStatus,
    ReplaceableRuleCategory,
    ResolutionResult,
    ResolutionType,
    ShareBuyback,
    ShareClass,
    ShareConsideration,
    ShareIssue,
    ShareIssueType,
    ShareIssueValidation,
    Shareholding,
    // Validation functions
    validate_buyback,
    validate_financial_assistance,
    validate_share_issue,
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
