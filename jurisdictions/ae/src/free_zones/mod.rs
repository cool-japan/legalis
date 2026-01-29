//! UAE Free Zone Legal Frameworks
//!
//! The UAE has 40+ free zones, each with special legal and tax regimes.
//! This module covers the three major financial/commercial free zones:
//!
//! ## Common Law Free Zones
//!
//! - **DIFC** (Dubai International Financial Centre) - [`difc`]
//! - **ADGM** (Abu Dhabi Global Market) - [`adgm`]
//!
//! Both operate under Common Law (English law) with independent courts.
//!
//! ## Federal Law Free Zones
//!
//! - **JAFZA** (Jebel Ali Free Zone) - [`jafza`]
//! - Others (DMCC, Dubai Silicon Oasis, etc.)
//!
//! Operate under UAE Federal Law with special exemptions.
//!
//! ## Key Differences
//!
//! | Aspect | DIFC/ADGM | JAFZA/Others |
//! |--------|-----------|--------------|
//! | Legal System | Common Law | UAE Federal Law |
//! | Courts | Independent | Dubai/Abu Dhabi Courts |
//! | Corporate Tax | 0% (50 years) | 0% (50 years) |
//! | Ownership | 100% foreign | 100% foreign |
//! | Activities | Financial services | Trading, manufacturing, services |

pub mod adgm;
pub mod difc;
pub mod jafza;

pub use adgm::{
    AdgmCompanyType, AdgmCourtJurisdiction, AdgmDataProtection, AdgmEmployment, AdgmError,
    AdgmFinancialActivity, AdgmResult, get_adgm_benefits,
    validate_company_registration as validate_adgm_registration,
};

pub use difc::{
    DifcCompanyType, DifcCourtJurisdiction, DifcEmployment, DifcError, DifcFinancialActivity,
    DifcResult, get_difc_benefits, validate_company_registration as validate_difc_registration,
};

pub use jafza::{
    JafzaActivity, JafzaCompanyType, JafzaError, JafzaLicenseType, JafzaOfficeType,
    JafzaRegistration, JafzaResult, estimate_setup_cost, get_jafza_benefits,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_law_zones() {
        // DIFC and ADGM are Common Law jurisdictions
        let difc_company = DifcCompanyType::PrivateCompanyLimitedByShares;
        assert_eq!(difc_company.abbreviation(), "Ltd.");

        let adgm_company = AdgmCompanyType::LimitedLiabilityCompany;
        assert_eq!(adgm_company.abbreviation(), "LLC");
    }

    #[test]
    fn test_federal_law_zones() {
        // JAFZA operates under UAE Federal Law
        let jafza_company = JafzaCompanyType::Fze;
        assert_eq!(jafza_company.abbreviation(), "FZE");
    }
}
