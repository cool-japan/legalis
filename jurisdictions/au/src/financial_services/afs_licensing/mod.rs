//! AFS Licensing Module (Corporations Act 2001 Part 7.6)
//!
//! Australian Financial Services License (AFSL) requirements and administration.
//!
//! # Key Legislation
//!
//! ## Corporations Act 2001 Part 7.6 - Licensing of Financial Service Providers
//!
//! ### Division 1: Requirement to hold AFSL
//!
//! Section 911A requires a person carrying on a financial services business to hold an AFSL.
//!
//! ### Division 2: Granting an AFSL
//!
//! **Section 913B: When ASIC must grant AFSL**
//!
//! ASIC must grant a licence if satisfied the applicant:
//! - (a) Will comply with obligations as a licensee
//! - (b) Is of good fame and character
//! - (c) Has adequate competence to provide the financial services
//! - (d) Has adequate resources (financial, technological, human)
//! - (e) Has adequate risk management systems
//!
//! **Section 914A: Conditions on AFSL**
//!
//! ASIC may impose conditions including:
//! - Standard conditions (apply to all licensees)
//! - Specific conditions (tailored to licensee)
//! - Imposed conditions (added later by ASIC)
//!
//! ### Division 4: Authorized Representatives
//!
//! **Section 916A: Authorised Representatives**
//!
//! AFSL holder may authorize others to provide financial services on their behalf:
//! - Must notify ASIC within prescribed period
//! - Licensee remains responsible for representative's conduct
//! - Must take reasonable steps to ensure compliance
//!
//! **Section 916F: Corporate Authorised Representatives**
//!
//! A body corporate can be authorized as representative:
//! - CAR may sub-authorize individuals
//! - Licensee oversight obligations apply
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_au::financial_services::afs_licensing::*;
//! use chrono::NaiveDate;
//!
//! let license = AfslLicense {
//!     license_number: "123456".to_string(),
//!     licensee_name: "Example Financial Pty Ltd".to_string(),
//!     abn: "12345678901".to_string(),
//!     status: LicenseStatus::Current,
//!     issue_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
//!     authorized_services: vec![
//!         AuthorizedService::ProvideFinancialProductAdvice {
//!             product_type: ProductType::Securities,
//!             client_type: ClientType::Retail,
//!         },
//!     ],
//!     conditions: vec![],
//! };
//!
//! let service = AuthorizedService::ProvideFinancialProductAdvice {
//!     product_type: ProductType::Securities,
//!     client_type: ClientType::Retail,
//! };
//!
//! validate_afsl_authorization(&license, &service)?;
//! ```

pub mod error;
pub mod types;
pub mod validator;

pub use error::{AfsLicensingError, Result};
pub use types::{
    AfslCondition, AfslLicense, AuthorizedRepresentative, AuthorizedService, ClientType,
    ConditionType, LicenseStatus, ProductType, ResponsibleManager,
};
pub use validator::{
    validate_afsl_authorization, validate_authorized_representative, validate_license_conditions,
    validate_responsible_manager,
};
