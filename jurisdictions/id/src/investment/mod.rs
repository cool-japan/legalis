//! Indonesian Investment Law (UU Penanaman Modal) - UU No. 25 Tahun 2007
//!
//! As amended by Omnibus Law (UU Cipta Kerja) and PP 5/2021.
//!
//! ## Key Features
//!
//! - Negative Investment List (DNI) - restricted sectors
//! - OSS (Online Single Submission) risk-based licensing
//! - BKPM (Investment Coordinating Board) procedures
//! - PT PMA (Foreign Investment Company) requirements
//! - Tax incentives and investment facilities
//!
//! ## Omnibus Law Changes (2020/2023)
//!
//! - Risk-based licensing (OSS-RBA)
//! - Simplified DNI (Priority sectors, Restricted, Open)
//! - Digital economy provisions
//! - SEZ (Special Economic Zone) enhancements

mod error;
mod types;
mod validator;

pub use error::{InvestmentError, InvestmentResult};
pub use types::{
    BusinessLicense, BusinessRisk, ForeignInvestment, InvestmentSector, LicenseType,
    OwnershipLimit, PriorityInvestment, PrioritySector,
};
pub use validator::{
    InvestmentCompliance, check_ownership_limit, get_investment_checklist,
    validate_business_license, validate_foreign_investment, validate_investment_compliance,
    validate_sector_eligibility,
};
