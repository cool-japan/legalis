//! Omnibus Law - Job Creation Law (UU Cipta Kerja)
//!
//! ## Overview
//!
//! The Job Creation Law represents Indonesia's most significant legal reform:
//! - **UU No. 11/2020**: Original Omnibus Law (problematic enactment)
//! - **Perppu No. 2/2022**: Government Regulation in Lieu of Law (revision)
//! - **UU No. 6/2023**: Formal enactment replacing UU 11/2020
//!
//! ## Scope
//!
//! The Omnibus Law amends **79 existing laws** across 11 clusters:
//! 1. Simplification of permits (perizinan)
//! 2. Investment requirements
//! 3. Employment (ketenagakerjaan)
//! 4. Ease of doing business
//! 5. Research and innovation
//! 6. Land acquisition
//! 7. Economic zones
//! 8. Central government investment
//! 9. Implementation of government projects
//! 10. Administrative sanctions
//! 11. Licensing institutions
//!
//! ## Key Reforms
//!
//! - **Risk-Based Licensing (OSS)**: Online Single Submission system
//! - **Labor law changes**: Contract types, severance, outsourcing
//! - **Investment liberalization**: Opening more sectors to foreign investment
//! - **Tax incentives**: Super deductions for R&D and vocational training
//! - **Land rights streamlining**: HGB for apartments up to 80 years
//!
//! ## Modules
//!
//! - [`investment`]: Investment and licensing reforms
//! - [`labor`]: Labor law amendments
//! - [`taxation`]: Tax incentives and changes

pub mod investment;
pub mod labor;
pub mod taxation;

pub use investment::{
    InvestmentIncentive, LicenseCategory, OssRiskLevel, RiskBasedLicense,
    SimplifiedSectorEligibility,
};
pub use labor::{OmnibusContractType, OmnibusOutsourcing, OmnibusSeverance, OmnibusWorkingHours};
pub use taxation::{SuperDeduction, TaxHoliday, TaxIncentiveType};
