//! Indonesian Tax Law
//!
//! ## Overview
//!
//! Indonesian taxation system is governed by multiple laws, primarily:
//! - UU No. 6/1983 as amended - General Tax Provisions (KUP)
//! - UU No. 7/1983 as amended by UU 7/2021 - Income Tax (PPh)
//! - UU No. 8/1983 as amended by UU 7/2021 - Value Added Tax (PPN)
//!
//! ## Tax Types
//!
//! ### Direct Taxes
//! - **Income Tax (PPh)**: Taxes on income of individuals and corporations
//! - **Land and Building Tax (PBB)**: Property tax (now regional tax)
//!
//! ### Indirect Taxes
//! - **Value Added Tax (PPN)**: Tax on consumption of goods and services
//! - **Luxury Goods Tax (PPnBM)**: Additional tax on luxury goods
//! - **Excise Duties**: On tobacco, alcohol, ethyl alcohol
//!
//! ## Tax Administration
//!
//! - **Tax authority**: Direktorat Jenderal Pajak (DGP)
//! - **Tax ID**: NPWP (Nomor Pokok Wajib Pajak) - 15 digits
//! - **Tax year**: Same as calendar year (January 1 - December 31)
//! - **Annual tax return**: Due by March 31 (individuals) or April 30 (corporations)
//!
//! ## Recent Reforms (UU HPP - UU No. 7/2021)
//!
//! - VAT rate increased from 10% to 11% (2022), then 12% (2025)
//! - New top individual income tax bracket: 35% for income > Rp 5 billion
//! - Corporate tax rate reduced to 22%
//! - Carbon tax framework introduced

pub mod income_tax;
pub mod vat;

pub use income_tax::{
    CorporateTaxRate, IncomeTaxBracket, Pph4ayat2Type, Pph21, Pph23Type, Pph26, Pph26PaymentType,
    PtkpStatus, TaxSubject,
};

pub use vat::{
    LuxuryGoodsTaxRate, TransactionType, VatExemptCategory, VatRate, VatRegistrationStatus,
    VatTransaction,
};
