//! Vietnamese Tax Law Module
//!
//! Comprehensive tax law implementations for Vietnam.
//!
//! ## Modules
//!
//! - [`vat`] - Value Added Tax (Thuế GTGT)
//! - [`cit`] - Corporate Income Tax (Thuế TNDN)
//! - [`pit`] - Personal Income Tax (Thuế TNCN)

pub mod cit;
pub mod pit;
pub mod vat;

// Re-export commonly used items
pub use cit::{
    CitDeclarationPeriod, CitIncentive, CitRate, CitResult, CitTaxableIncome, IncentiveSector,
    determine_cit_rate, get_cit_checklist, validate_taxable_income,
};

pub use pit::{
    PersonalDeduction, PitBracket, PitCalculation, PitIncomeType, PitResult,
    calculate_flat_rate_pit, calculate_progressive_pit, get_pit_checklist,
    validate_pit_calculation,
};

pub use vat::{
    VatDeclaration, VatExemptCategory, VatMethod, VatRate, VatResult,
    calculate_base_price_from_inclusive, calculate_price_including_vat, calculate_vat,
    get_vat_checklist, validate_vat_declaration,
};
