//! Australian Superannuation Law
//!
//! Comprehensive implementation of Australian superannuation legislation.
//!
//! ## Key Legislation
//!
//! ### Superannuation Guarantee (Administration) Act 1992
//!
//! Establishes the compulsory superannuation system:
//! - Employer obligations (Division 2)
//! - SG charge (Division 5)
//! - Contribution rules (Division 6)
//!
//! ### Superannuation Industry (Supervision) Act 1993 (SIS Act)
//!
//! Regulates superannuation funds:
//! - Fund types and structure (Part 2)
//! - Operating standards (Part 3)
//! - Trustee duties (Part 5)
//! - Preservation (Part 6)
//! - Benefit payments (Part 6A)
//! - In-house asset rules (Part 8)
//!
//! ### Superannuation (Unclaimed Money and Lost Members) Act 1999
//!
//! Transfer of unclaimed super and lost member accounts.
//!
//! ## SG Rate Schedule
//!
//! | Financial Year | Rate |
//! |----------------|------|
//! | 2024-25 | 11.5% |
//! | 2025-26+ | 12.0% |
//!
//! ## Contribution Caps (2024-25)
//!
//! | Cap Type | Amount |
//! |----------|--------|
//! | Concessional | $30,000 |
//! | Non-concessional | $120,000 |
//! | Transfer balance cap | $1,900,000 |
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_au::superannuation::*;
//!
//! // Calculate SG contribution
//! let sg = calculate_sg_contribution(10_000.0, "2024-25", true);
//! println!("SG required: ${:.2}", sg.sg_amount);
//!
//! // Assess benefit release eligibility
//! let release = assess_benefit_release(
//!     &member,
//!     ConditionOfRelease::Retirement,
//!     assessment_date,
//!     None,
//! )?;
//! ```

pub mod benefits;
pub mod contributions;
pub mod error;
pub mod smsf;
pub mod types;

// Re-exports
pub use benefits::{
    BeneficiaryAllocation, BenefitReleaseAssessment, BenefitTaxTreatment, DeathBenefitDistribution,
    LOW_RATE_CAP_2024_25, PensionDrawdown, assess_benefit_release, calculate_pension_drawdown,
    minimum_drawdown_factor, validate_benefit_payment, validate_death_benefit,
};
pub use contributions::{
    ContributionCapAssessment, EmployeeSgEligibility, EmploymentType, SgCalculation,
    SgShortfallCalculation, assess_contribution_caps, calculate_sg_contribution,
    calculate_sg_shortfall, check_sg_eligibility, is_sg_payment_on_time, sg_due_date,
    validate_contribution,
};
pub use error::{Result, SuperannuationError};
pub use smsf::{
    AuditContravention, AuditContraventionReport, ComplianceCategory, ComplianceIssue,
    DisqualificationReason, IN_HOUSE_ASSET_LIMIT, InHouseAsset, InHouseAssetCalculation,
    InHouseAssetType, InvestmentStrategy, IssueSeverity, SmsfComplianceAssessment,
    assess_smsf_compliance, calculate_in_house_assets, validate_member_count,
    validate_trustee_eligibility,
};
pub use types::{
    BeneficiaryNomination, BeneficiaryRelationship, BenefitPaymentType, ConditionOfRelease,
    Contribution, ContributionCapType, ContributionCaps, ContributionType, FundMember, FundType,
    IncomeProtection, MAX_SUPER_CONTRIBUTION_BASE_QUARTERLY_2024_25, MemberCategory,
    MemberInsurance, NominationType, PreservationStatus, SG_RATES, SgQuarter, SgRate, SmsfDetails,
    SmsfTrusteeType, SuperannuationFund, sg_rate_for_year,
};

use legalis_core::{Effect, EffectType, Statute};

/// Create Superannuation Guarantee (Administration) Act 1992 statute
pub fn create_sg_act() -> Statute {
    Statute::new(
        "AU-SG-1992",
        "Superannuation Guarantee (Administration) Act 1992 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "Requires employers to make superannuation contributions for eligible employees \
             at the SG rate, with SG charge applying for shortfalls",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create Superannuation Industry (Supervision) Act 1993 statute
pub fn create_sis_act() -> Statute {
    Statute::new(
        "AU-SIS-1993",
        "Superannuation Industry (Supervision) Act 1993 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "Regulates superannuation fund operation including trustee duties, \
             preservation requirements, and operating standards",
        ),
    )
    .with_jurisdiction("AU")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sg_act() {
        let statute = create_sg_act();
        assert_eq!(statute.id, "AU-SG-1992");
        assert!(statute.title.contains("Superannuation Guarantee"));
    }

    #[test]
    fn test_create_sis_act() {
        let statute = create_sis_act();
        assert_eq!(statute.id, "AU-SIS-1993");
        assert!(statute.title.contains("Supervision"));
    }
}
