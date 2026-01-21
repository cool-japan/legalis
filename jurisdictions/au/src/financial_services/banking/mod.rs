//! Banking Module (Banking Act 1959, APRA Prudential Standards)
//!
//! Regulation of Authorised Deposit-taking Institutions (ADIs) by APRA.
//!
//! # Key Legislation
//!
//! ## Banking Act 1959 (Cth)
//!
//! - Section 8: Restriction on using word "bank"
//! - Section 9: Authority required to conduct banking business
//! - Section 11: APRA authorization requirements
//!
//! ## APRA Prudential Standards
//!
//! - **APS 110**: Capital Adequacy
//! - **APS 210**: Liquidity
//! - **APS 220**: Credit Risk Management
//! - **APS 310**: Audit and Related Matters
//! - **APS 330**: Public Disclosure

pub mod error;
pub mod types;
pub mod validator;

pub use error::{BankingError, Result};
pub use types::{
    AdiStatus, ApraRequirement, AuthorizedDepositInstitution, CapitalRequirement,
    LiquidityRequirement, PrudentialStandard,
};
pub use validator::{validate_adi_compliance, validate_capital_adequacy, validate_liquidity};
