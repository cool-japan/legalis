//! UK Intellectual Property Law
//!
//! Comprehensive implementation of UK IP law across four main areas:
//! - Patents (Patents Act 1977)
//! - Trade Marks (Trade Marks Act 1994)
//! - Copyright (Copyright, Designs and Patents Act 1988)
//! - Designs (Registered Designs Act 1949, Design Right)
//!
//! ## Key Legislation
//!
//! ### Patents Act 1977
//! - Implements European Patent Convention (EPC) in UK law
//! - Sections 1-4: Patentability requirements
//! - Section 60: Infringement
//! - Section 72: Revocation
//!
//! ### Trade Marks Act 1994
//! - Implements EU Trade Mark Directive
//! - Section 1: Registrable trade marks
//! - Section 3: Absolute grounds for refusal
//! - Section 5: Relative grounds for refusal
//! - Section 10: Infringement
//!
//! ### Copyright, Designs and Patents Act 1988
//! - Chapter I: Copyright
//! - Chapter II: Rights in performances
//! - Chapter III: Design right
//! - Section 16: Acts restricted by copyright
//! - Sections 29-76: Permitted acts (fair dealing)
//!
//! ### Registered Designs Act 1949
//! - Registration system for designs
//! - Novelty and individual character requirements
//!
//! ## UK IPO (Intellectual Property Office)
//!
//! The UK IPO is the official government body responsible for IP rights:
//! - Patent examination and grant
//! - Trade mark registration
//! - Design registration
//! - IP tribunals and hearings
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::intellectual_property::*;
//!
//! // Check patent validity
//! let invention = PatentApplication {
//!     title: "New pharmaceutical compound".to_string(),
//!     claims: vec!["A compound of formula X".to_string()],
//!     prior_art: vec!["US Patent 123456".to_string()],
//!     filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//! };
//!
//! let validity = check_patentability(&invention)?;
//! assert!(validity.is_novel);
//! assert!(validity.has_inventive_step);
//!
//! // Check trademark infringement
//! let mark1 = TradeMark {
//!     sign: "APPLE".to_string(),
//!     goods_services: vec!["Computers".to_string()],
//!     registration_date: Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
//! };
//!
//! let mark2 = TradeMark {
//!     sign: "APLE".to_string(),
//!     goods_services: vec!["Computers".to_string()],
//!     registration_date: None,
//! };
//!
//! let likelihood = assess_likelihood_of_confusion(&mark1, &mark2)?;
//! // Likely to be confusing
//! ```
//!
//! ## Legal References
//!
//! - [UK IPO](https://www.gov.uk/government/organisations/intellectual-property-office)
//! - [Patents Act 1977](https://www.legislation.gov.uk/ukpga/1977/37)
//! - [Trade Marks Act 1994](https://www.legislation.gov.uk/ukpga/1994/26)
//! - [CDPA 1988](https://www.legislation.gov.uk/ukpga/1988/48)

pub mod copyright;
pub mod designs;
pub mod enforcement;
pub mod error;
pub mod patents;
pub mod trademarks;
pub mod types;

// Re-exports
pub use copyright::{
    CopyrightDuration, CopyrightWork, CopyrightWorkType, FairDealingPurpose, PerformanceRight,
    calculate_copyright_duration, check_fair_dealing, validate_copyright_work,
};
pub use designs::{
    Design, DesignRegistration, DesignRightType, DesignType, validate_design_registration,
    validate_design_right,
};
pub use enforcement::{IpEnforcementAction, IpRemedy, IpRemedyType, IpTribunal, UkIpoProceeding};
pub use error::{IpError, IpResult};
pub use patents::{
    IndustrialApplicability, InventiveStep, Novelty, Patent, PatentApplication, PatentClaim,
    PatentInfringement, Patentability, assess_inventive_step, check_industrial_application,
    check_novelty, check_patentability, validate_patent_claim,
};
pub use trademarks::{
    AbsoluteGroundsRefusal, LikelihoodOfConfusion, RelativeGroundsRefusal, TradeMark,
    TradeMarkApplication, TradeMarkInfringement, TradeMarkType, assess_likelihood_of_confusion,
    check_absolute_grounds, check_relative_grounds, validate_trademark,
};
pub use types::{IpOwner, IpRight, IpRightType, LicenseType, PriorArt, RegistrationStatus};
