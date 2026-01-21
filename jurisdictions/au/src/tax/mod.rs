//! Australian Tax Law
//!
//! Comprehensive implementation of Australian tax law administered by the ATO.
//!
//! ## Key Legislation
//!
//! ### Income Tax Assessment Act 1997 (ITAA 1997)
//!
//! Primary income tax legislation covering:
//! - Assessable income (Division 6)
//! - Deductions (Division 8)
//! - Tax offsets (Division 13)
//! - Capital Gains Tax (Parts 3-1 to 3-3)
//! - Trading stock (Division 70)
//! - Small business concessions (Division 328)
//!
//! ### A New Tax System (Goods and Services Tax) Act 1999 (GST Act)
//!
//! GST legislation covering:
//! - Taxable supplies (Division 9)
//! - GST-free supplies (Division 38)
//! - Input-taxed supplies (Division 40)
//! - Input tax credits (Division 11)
//!
//! ### Fringe Benefits Tax Assessment Act 1986 (FBT Act)
//!
//! FBT on non-cash employee benefits.
//!
//! ### Superannuation Laws
//!
//! - Superannuation Guarantee (Administration) Act 1992
//! - Superannuation Industry (Supervision) Act 1993
//!
//! ## Tax Rates (2024-25)
//!
//! ### Individual Income Tax
//!
//! | Taxable Income | Tax Rate |
//! |----------------|----------|
//! | $0 - $18,200 | Nil |
//! | $18,201 - $45,000 | 19c for each $1 over $18,200 |
//! | $45,001 - $120,000 | $5,092 plus 32.5c for each $1 over $45,000 |
//! | $120,001 - $180,000 | $29,467 plus 37c for each $1 over $120,000 |
//! | $180,001+ | $51,667 plus 45c for each $1 over $180,000 |
//!
//! ### GST
//!
//! Standard rate: **10%**
//!
//! ### Company Tax
//!
//! - Base rate entities: **25%**
//! - Other companies: **30%**
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_au::tax::*;
//!
//! // Calculate individual income tax
//! let tax = calculate_individual_tax(TaxableIncome {
//!     assessable_income: 120_000.0,
//!     deductions: 5_000.0,
//!     tax_offsets: vec![],
//!     financial_year: FinancialYear::FY2024_25,
//! })?;
//!
//! // Calculate GST on a supply
//! let gst = calculate_gst(&TaxableSupply {
//!     supply_type: SupplyType::Goods,
//!     gst_inclusive_price: 110.0,
//!     is_taxable: true,
//!     // ...
//! })?;
//! ```

pub mod cgt;
pub mod error;
pub mod gst;
pub mod income;
pub mod types;

// Re-exports
pub use cgt::{
    CgtAsset, CgtCalculation, CgtDiscount, CgtEvent, CgtExemption, calculate_capital_gain,
    validate_cgt_event,
};
pub use error::{Result, TaxError};
pub use gst::{
    GstCalculation, GstStatus, InputTaxCredit, SupplyType, TaxableSupply, calculate_gst,
    calculate_input_tax_credit, validate_bas_lodgement,
};
pub use income::{
    Deduction, IncomeCategory, TaxCalculation, TaxOffset, TaxableIncome, calculate_company_tax,
    calculate_individual_tax, validate_deduction,
};
pub use types::{
    Abn, EntityType, FinancialYear, GstRegistration, TaxAgent, TaxFileNumber, TaxPayer,
};

use legalis_core::{Effect, EffectType, Statute};

/// Create Income Tax Assessment Act 1997 statute
pub fn create_itaa_1997() -> Statute {
    Statute::new(
        "AU-ITAA-1997",
        "Income Tax Assessment Act 1997 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "Imposes income tax on assessable income of Australian residents and \
             certain income of non-residents, with deductions and offsets",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create GST Act 1999 statute
pub fn create_gst_act() -> Statute {
    Statute::new(
        "AU-GST-1999",
        "A New Tax System (Goods and Services Tax) Act 1999 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "Imposes 10% GST on taxable supplies of goods and services in Australia",
        ),
    )
    .with_jurisdiction("AU")
}

/// Create FBT Act 1986 statute
pub fn create_fbt_act() -> Statute {
    Statute::new(
        "AU-FBT-1986",
        "Fringe Benefits Tax Assessment Act 1986 (Cth)",
        Effect::new(
            EffectType::Obligation,
            "Imposes tax on employers providing non-cash fringe benefits to employees",
        ),
    )
    .with_jurisdiction("AU")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_itaa_1997() {
        let statute = create_itaa_1997();
        assert_eq!(statute.id, "AU-ITAA-1997");
        assert!(statute.title.contains("Income Tax"));
    }

    #[test]
    fn test_create_gst_act() {
        let statute = create_gst_act();
        assert_eq!(statute.id, "AU-GST-1999");
        assert!(statute.title.contains("Goods and Services Tax"));
    }

    #[test]
    fn test_create_fbt_act() {
        let statute = create_fbt_act();
        assert_eq!(statute.id, "AU-FBT-1986");
        assert!(statute.title.contains("Fringe Benefits"));
    }
}
