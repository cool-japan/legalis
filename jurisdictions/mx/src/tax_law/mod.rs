//! Tax Law (Código Fiscal de la Federación and related laws)
//!
//! Mexican tax regulations including:
//! - ISR (Impuesto Sobre la Renta) - Income Tax
//! - IVA (Impuesto al Valor Agregado) - Value Added Tax (16%)
//! - IEPS (Impuesto Especial sobre Producción y Servicios) - Special Production Tax

pub mod ieps;
pub mod isr;
pub mod iva;
pub mod types;
pub mod validator;

// Re-export main types
pub use ieps::{IEPSCategory, calculate_ieps, calculate_sugary_drinks_ieps};
pub use isr::{
    CORPORATE_TAX_RATE, TAX_BRACKETS, calculate_corporate_isr, calculate_individual_isr,
};
pub use iva::{
    BORDER_RATE, IVARate, STANDARD_RATE, ZERO_RATE, calculate_iva, calculate_with_iva,
    extract_iva_from_total,
};
pub use types::{TaxObligation, TaxPeriod, TaxType, Taxpayer, TaxpayerType};
pub use validator::{TaxValidationError, validate_taxpayer};
