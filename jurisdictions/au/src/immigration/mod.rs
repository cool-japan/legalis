//! Australian Immigration and Citizenship Law
//!
//! Comprehensive implementation of Australian immigration legislation.
//!
//! ## Key Legislation
//!
//! - Migration Act 1958 (Cth)
//! - Migration Regulations 1994
//! - Australian Citizenship Act 2007 (Cth)
//! - Australian Citizenship Regulations 2007
//!
//! ## Visa Categories
//!
//! | Category | Description | Key Subclasses |
//! |----------|-------------|----------------|
//! | Skilled | Points-tested skilled migration | 189, 190, 491 |
//! | Employer Sponsored | Sponsored by approved employer | 482, 494, 186 |
//! | Family | Family stream migration | 820, 309, 143 |
//! | Business | Business and investment | 188, 888, 858 |
//! | Student | Study in Australia | 500 |
//! | Visitor | Tourism, business visits | 600, 601, 651 |
//! | Working Holiday | Work and travel | 417, 462 |
//! | Humanitarian | Protection visas | 200, 866 |
//!
//! ## Skilled Migration
//!
//! ### Points Test (65 point pass mark)
//!
//! | Factor | Maximum Points |
//! |--------|----------------|
//! | Age (25-32) | 30 |
//! | English (Superior) | 20 |
//! | Australian employment (8+ years) | 20 |
//! | Education (Doctorate) | 20 |
//! | Regional nomination | 15 |
//!
//! ## Character Test (s.501)
//!
//! Visa may be refused/cancelled if applicant does not pass character test:
//! - Substantial criminal record (12+ months imprisonment)
//! - Association with criminal group
//! - Adverse ASIO security assessment
//! - Past visa refusal/cancellation
//!
//! ## Citizenship Requirements
//!
//! ### By Conferral (s.21)
//!
//! - Lawful resident for 4 years
//! - Permanent resident for at least 12 months
//! - Present in Australia for at least 1460 days (4 years)
//! - No more than 90 days absence in 12 months before application
//! - Good character
//! - Pass citizenship test (75%)
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_au::immigration::*;
//!
//! // Assess points test
//! let points_result = assess_points_test(
//!     28,                    // Age
//!     &english_result,       // English test
//!     5.0,                   // Overseas employment
//!     2.0,                   // Australian employment
//!     "Bachelor degree",     // Qualification
//!     true,                  // Australian study
//!     false, false, false,   // Specialist, language, professional year
//!     false, true,           // Partner skills, single
//!     Some("state"),         // Nomination
//! );
//!
//! if points_result.passed {
//!     println!("Points test passed with {} points", points_result.total_points);
//! }
//!
//! // Assess character test
//! let character_result = assess_character_test(
//!     has_criminal_record,
//!     sentence_months,
//!     is_on_parole,
//!     has_criminal_association,
//!     has_security_assessment,
//!     past_refusals,
//!     past_cancellations,
//! );
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{ImmigrationError, Result};
pub use types::{
    AssessmentOutcome, CharacterTestGround, CitizenshipApplication, CitizenshipApplicationStatus,
    CitizenshipStream, EnglishLanguageLevel, EnglishLanguageTest, EnglishTestResult,
    ImmigrationStatus, MigrationZone, OccupationAssessment, PointsCategory, PointsTestResult,
    ResidenceRequirement, SkilledOccupationList, Sponsor, SponsorComplianceHistory, SponsorType,
    VisaApplication, VisaCategory, VisaCondition, VisaHolder, VisaStatus, VisaSubclass,
};
pub use validator::{
    CITIZENSHIP_LAST_12_MONTHS_DAYS, CITIZENSHIP_MAX_ABSENCES_12_MONTHS,
    CITIZENSHIP_RESIDENCE_DAYS, CharacterConcern, CharacterConcernSeverity, CharacterTestResult,
    CitizenshipEligibilityResult, POINTS_PASS_MARK, SIGNIFICANT_COST_THRESHOLD,
    SponsorValidationResult, VisaEligibilityResult, assess_character_test,
    assess_citizenship_by_conferral, assess_citizenship_test,
    assess_employer_sponsored_eligibility, assess_points_test, assess_skilled_visa_eligibility,
    calculate_age_points, calculate_australian_employment_points, calculate_education_points,
    calculate_english_points, calculate_overseas_employment_points, validate_sponsor,
    validate_visa_condition_compliance,
};

use legalis_core::{Effect, EffectType, Statute};

/// Create Migration Act 1958 statute
pub fn create_migration_act() -> Statute {
    Statute::new(
        "AU-MIGRATION-1958",
        "Migration Act 1958 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "Establishes framework for entry to and stay in Australia, \
             including visa system, character test, and enforcement powers",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create Australian Citizenship Act 2007 statute
pub fn create_citizenship_act() -> Statute {
    Statute::new(
        "AU-CITIZENSHIP-2007",
        "Australian Citizenship Act 2007 (Cth)",
        Effect::new(
            EffectType::Grant,
            "Establishes framework for acquisition of Australian citizenship \
             by birth, descent, conferral and resumption",
        ),
    )
    .with_jurisdiction("AU")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_migration_act() {
        let statute = create_migration_act();
        assert!(statute.id.contains("MIGRATION"));
        assert!(statute.title.contains("Migration Act 1958"));
    }

    #[test]
    fn test_create_citizenship_act() {
        let statute = create_citizenship_act();
        assert!(statute.id.contains("CITIZENSHIP"));
        assert!(statute.title.contains("Australian Citizenship Act 2007"));
    }
}
