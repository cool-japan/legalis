//! Australian Employment Law
//!
//! Implementation of Australian employment law including:
//! - Fair Work Act 2009
//! - National Employment Standards
//! - Modern awards system
//! - Enterprise agreements
//! - Unfair dismissal
//! - General protections
//!
//! ## Key Legislation
//!
//! - Fair Work Act 2009 (Cth)
//! - Fair Work Regulations 2009
//! - Modern Awards
//!
//! ## Key Cases
//!
//! - Byrne v Australian Airlines (1995) - Employment contract limits
//! - Sayer v Melsteel (2011) - Constructive dismissal
//! - Rankin v Marine Power (2001) - Genuine redundancy

pub mod fair_work;
pub mod types;
pub mod unfair_dismissal;

// Re-export commonly used types
pub use fair_work::{
    AgreementApprovalResult, AgreementFacts, EnterpriseAgreementAnalyzer,
    GeneralProtectionsAnalyzer, GeneralProtectionsFacts, GeneralProtectionsRemedy,
    GeneralProtectionsResult, NesAnalyzer, NesFacts, NesResult,
};
pub use types::{
    AdverseAction, AwardCoverage, AwardType, BootResult, ConductType, DiscriminationGround,
    DismissalReason, DismissalType, EmployeeCategory, EmploymentCase, EmploymentInstrument,
    EmploymentType, EnterpriseAgreementType, NationalEmploymentStandard, ProtectedAttribute,
    Section387Factor, UnfairDismissalElement, UnfairDismissalRemedy,
};
pub use unfair_dismissal::{
    CompensationCalculator, CompensationFacts, CompensationResult, EligibilityAnalyzer,
    EligibilityFacts, EligibilityResult, Section387Result, UnfairDismissalAnalyzer,
    UnfairDismissalFacts, UnfairDismissalResult,
};

use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create Fair Work Act 2009 statute
pub fn create_fair_work_act() -> Statute {
    Statute::new(
        "AU-FWA-2009",
        "Fair Work Act 2009",
        Effect::new(
            EffectType::Grant,
            "National employment standards and workplace relations framework",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create a Modern Award statute
pub fn create_modern_award(award_id: &str, award_name: &str) -> Statute {
    Statute::new(
        format!("AU-MA-{}", award_id),
        award_name,
        Effect::new(
            EffectType::Grant,
            format!("Industry/occupation specific conditions - {}", award_name),
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
    fn test_create_fair_work_act() {
        let statute = create_fair_work_act();
        assert!(statute.id.contains("FWA"));
    }

    #[test]
    fn test_create_modern_award() {
        let statute = create_modern_award("MA000001", "General Retail Industry Award 2020");
        assert!(statute.id.contains("MA-MA000001"));
    }

    #[test]
    fn test_nes_compliance() {
        let facts = NesFacts {
            employment_type: EmploymentType::FullTime,
            applicable_standards: vec![
                NationalEmploymentStandard::AnnualLeave,
                NationalEmploymentStandard::MaximumWeeklyHours,
            ],
            weekly_hours: 38.0,
            annual_leave_weeks: 4.0,
            ..Default::default()
        };

        let result = NesAnalyzer::check_compliance(&facts);
        assert!(result.compliant);
    }

    #[test]
    fn test_unfair_dismissal_claim() {
        // Eligible employee unfairly dismissed
        let eligibility = EligibilityFacts {
            months_employed: 12.0,
            national_system_employer: true,
            annual_earnings: 80_000.0,
            covered_by_award_or_agreement: true,
            ..Default::default()
        };

        let eligible_result = EligibilityAnalyzer::check_eligibility(&eligibility);
        assert!(eligible_result.eligible);

        let dismissal = UnfairDismissalFacts {
            dismissal_occurred: true,
            valid_reason: false,
            notified_of_reason: false,
            opportunity_to_respond: false,
            ..Default::default()
        };

        let dismissal_result = UnfairDismissalAnalyzer::analyze(&dismissal);
        assert!(dismissal_result.dismissal_unfair);
    }
}
