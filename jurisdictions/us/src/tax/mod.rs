//! State Tax Law Variations Across the United States
//!
//! This module provides comprehensive state-by-state tax law analysis, including:
//!
//! - **Income Tax**: State income tax rates, structures, and exemptions
//! - **Sales Tax**: State and local sales tax rates, nexus determination
//! - **Corporate Tax**: Corporate income tax, apportionment formulas
//!
//! # Overview
//!
//! The United States has significant tax variation across its 51 jurisdictions.
//! Unlike most countries with a unified national tax system, US states have
//! substantial autonomy in designing their tax structures.
//!
//! ## Key Variations
//!
//! - **9 states with NO state income tax**: AK, FL, NV, SD, TN, TX, WA, WY, NH
//! - **Sales tax ranges**: 0% (5 states) to 7.25%+ (California with locals)
//! - **Corporate tax havens**: Delaware (business-friendly), Nevada (no corporate tax)
//!
//! # Example: Income Tax Analysis
//!
//! ```rust
//! use legalis_us::tax::income_tax::{has_state_income_tax, income_tax_structure};
//!
//! // Check if a state has income tax
//! assert!(!has_state_income_tax("TX")); // Texas has no income tax
//! assert!(has_state_income_tax("CA"));  // California has progressive income tax
//!
//! // Get tax structure details
//! let ca_tax = income_tax_structure("CA");
//! // California: Progressive rates from 1% to 13.3%
//! ```

pub mod corporate_tax;
pub mod income_tax;
pub mod sales_tax;

// Re-exports
pub use corporate_tax::{
    ApportionmentFormula, CorporateTaxInfo, TaxHavenStatus, apportionment_formula,
    corporate_tax_rate, is_tax_haven,
};
pub use income_tax::{
    IncomeTaxStructure, IncomeTaxType, TaxBracket, has_state_income_tax, income_tax_structure,
    no_income_tax_states,
};
pub use sales_tax::{
    NexusType, SalesTaxInfo, SalesTaxNexus, has_sales_tax, post_wayfair_nexus, state_sales_tax_rate,
};
