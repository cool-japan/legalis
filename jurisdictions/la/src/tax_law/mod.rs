//! Tax Law (ກົດໝາຍພາສີ)
//!
//! This module implements Lao PDR tax law provisions, including:
//! - **Tax Law 2011** (Law No. 05/NA, effective October 20, 2011)
//! - **VAT Law** (Value Added Tax regulations)
//! - **Customs Law** (Import/Export duties)
//!
//! ## Historical Context
//!
//! The Lao PDR tax system has evolved significantly since the transition to a
//! market economy in 1986 (New Economic Mechanism - ນະໂຍບາຍເສດຖະກິດໃໝ່).
//! The Tax Law 2011 consolidated and modernized the tax framework, establishing
//! clear rates and procedures for various tax types.
//!
//! ## Tax Structure Overview
//!
//! ### 1. Personal Income Tax (ພາສີລາຍໄດ້ບຸກຄົນ)
//! Progressive tax rates from 0% to 25% based on monthly income:
//! - 0 - 1,300,000 LAK: 0%
//! - 1,300,001 - 8,500,000 LAK: 5%
//! - 8,500,001 - 15,000,000 LAK: 10%
//! - 15,000,001 - 24,000,000 LAK: 15%
//! - 24,000,001 - 65,000,000 LAK: 20%
//! - Over 65,000,000 LAK: 25%
//!
//! ### 2. Corporate Income Tax (ພາສີລາຍໄດ້ນິຕິບຸກຄົນ)
//! - Standard rate: 24% of taxable income
//! - Special rates apply to promoted sectors (Investment Promotion Law)
//!
//! ### 3. Value Added Tax (ພາສີມູນຄ່າເພີ່ມ)
//! - Standard rate: 10%
//! - Zero-rated: Exports
//! - Exempt: Financial services, education, healthcare, agriculture
//! - Registration threshold: 400,000,000 LAK annual turnover
//!
//! ### 4. Property Tax (ພາສີຊັບສິນ)
//! - Rate range: 0.1% - 0.5% of assessed value
//! - Applied to land and buildings
//!
//! ### 5. Excise Tax (ພາສີສິນຄ້າພິເສດ)
//! - Tobacco, alcohol, fuel, vehicles, luxury goods
//! - Rates vary by product category
//!
//! ### 6. Customs Duties (ພາສີສຸນລະກາກອນ)
//! - Rates: 0% - 40% based on HS code classification
//! - ASEAN Free Trade Area (AFTA) preferential rates apply
//!
//! ## Tax Residence Rules
//!
//! - **Resident**: Stays 183+ days per tax year in Lao PDR
//! - **Non-resident**: Taxed only on Lao-source income
//! - Tax treaties with several countries for double taxation relief
//!
//! ## Filing Requirements
//!
//! - **Personal Income Tax**: Annual filing by March 31
//! - **Corporate Income Tax**: Annual filing by March 31
//! - **VAT**: Monthly filing by 15th of following month
//! - **Property Tax**: Annual filing
//!
//! ## Example Usage
//!
//! ```
//! use legalis_la::tax_law::{
//!     PersonalIncomeTax, TaxResidenceStatus, IncomeType,
//!     calculate_personal_income_tax, validate_tax_residence,
//!     PERSONAL_INCOME_TAX_BRACKETS, INCOME_TAX_THRESHOLD,
//! };
//!
//! // Calculate personal income tax for a resident
//! let monthly_income: u64 = 20_000_000; // 20 million LAK
//! let tax_result = calculate_personal_income_tax(monthly_income);
//! assert!(tax_result.is_ok());
//!
//! // Validate tax residence (183 days rule)
//! let residence = TaxResidenceStatus::LaoResident {
//!     lao_id: Some("123456789".to_string()),
//!     days_in_lao: 200,
//! };
//! let validation = validate_tax_residence(&residence);
//! assert!(validation.is_ok());
//! ```
//!
//! ## VAT Calculation Example
//!
//! ```
//! use legalis_la::tax_law::{
//!     VAT_STANDARD_RATE, VAT_REGISTRATION_THRESHOLD,
//!     calculate_vat, validate_vat_registration,
//! };
//!
//! // Calculate VAT on a sale
//! let sale_amount: u64 = 10_000_000; // 10 million LAK
//! let vat_amount = calculate_vat(sale_amount, VAT_STANDARD_RATE);
//! assert!(vat_amount.is_ok());
//! let vat = vat_amount.expect("VAT calculation should succeed");
//! assert_eq!(vat, 1_000_000); // 10% VAT = 1 million LAK
//! ```
//!
//! ## Customs Duty Example
//!
//! ```
//! use legalis_la::tax_law::{
//!     CustomsDuty, CustomsDeclarationType,
//!     validate_hs_code, validate_customs_duty_rate,
//! };
//!
//! // Validate HS code format
//! let hs_code = "8471300000"; // Computers
//! let validation = validate_hs_code(hs_code);
//! assert!(validation.is_ok());
//!
//! // Validate customs duty rate
//! let rate = 0.10; // 10%
//! let validation = validate_customs_duty_rate(rate);
//! assert!(validation.is_ok());
//! ```
//!
//! ## Modules

pub mod error;
pub mod types;
pub mod validator;

// Re-export error types
pub use error::{Result, TaxLawError};

// Re-export constants
pub use types::{
    CORPORATE_INCOME_TAX_RATE, CUSTOMS_DUTY_RATE_MAX, CUSTOMS_DUTY_RATE_MIN, INCOME_TAX_THRESHOLD,
    PERSONAL_INCOME_TAX_BRACKETS, PROPERTY_TAX_RATE_MAX, PROPERTY_TAX_RATE_MIN,
    VAT_REGISTRATION_THRESHOLD, VAT_STANDARD_RATE, WITHHOLDING_TAX_DIVIDEND,
    WITHHOLDING_TAX_INTEREST, WITHHOLDING_TAX_ROYALTY, WITHHOLDING_TAX_SERVICE_NON_RESIDENT,
};

// Re-export tax residence types
pub use types::TaxResidenceStatus;

// Re-export personal income tax types
pub use types::{IncomeType, PersonalIncomeTax, PersonalIncomeTaxBracket};

// Re-export corporate income tax types
pub use types::{CorporateEntityType, CorporateIncomeTax};

// Re-export VAT types
pub use types::{
    VATExemptCategory, VATRateType, VATRegistrationStatus, VATReturn, VatExemption, VatRegistration,
};

// Re-export property tax types
pub use types::{PropertyTax, PropertyTaxType, PropertyType};

// Re-export excise tax types
pub use types::{ExciseTax, ExciseTaxCategory, FuelType};

// Re-export customs types
pub use types::{CustomsDeclarationType, CustomsDuty};

// Re-export filing and payment types
pub use types::{TaxFiling, TaxFilingPeriod, TaxFilingStatus, TaxPaymentMethod, TaxType};

// Re-export withholding tax types
pub use types::{WithholdingPaymentType, WithholdingTax};

// Re-export validators
pub use validator::{
    calculate_corporate_income_tax, calculate_excise_tax, calculate_personal_income_tax,
    calculate_property_tax, calculate_vat, validate_corporate_income_tax, validate_customs_duty,
    validate_customs_duty_rate, validate_excise_tax, validate_hs_code,
    validate_personal_income_tax, validate_property_tax, validate_property_tax_rate,
    validate_tax_filing, validate_tax_id_format, validate_tax_residence, validate_vat_calculation,
    validate_vat_exemption, validate_vat_rate, validate_vat_registration,
};
