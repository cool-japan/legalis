//! Value-Added Tax Act (부가가치세법)
//!
//! # 부가가치세법 / Value-Added Tax Act
//!
//! Standard VAT rate: 10%

use crate::common::KrwAmount;
use thiserror::Error;

/// VAT errors
#[derive(Debug, Error, Clone, PartialEq)]
pub enum VatError {
    /// Calculation error
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Result type for VAT operations
pub type VatResult<T> = Result<T, VatError>;

/// Standard VAT rate (10%)
pub const STANDARD_VAT_RATE: f64 = 0.10;

/// VAT registration threshold (48M KRW per year)
pub fn vat_registration_threshold() -> KrwAmount {
    KrwAmount::from_man(4_800.0)
}

/// Calculate VAT
pub fn calculate_vat(amount: &KrwAmount) -> VatResult<KrwAmount> {
    Ok(amount.multiply(STANDARD_VAT_RATE))
}

/// Calculate amount including VAT
pub fn calculate_with_vat(amount: &KrwAmount) -> VatResult<KrwAmount> {
    Ok(amount.multiply(1.0 + STANDARD_VAT_RATE))
}

/// Calculate amount excluding VAT
pub fn calculate_without_vat(amount_with_vat: &KrwAmount) -> VatResult<KrwAmount> {
    Ok(amount_with_vat.multiply(1.0 / (1.0 + STANDARD_VAT_RATE)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_vat() {
        let amount = KrwAmount::from_man(100.0);
        let result = calculate_vat(&amount);
        assert!(result.is_ok());

        if let Ok(vat) = result {
            assert!((vat.won - 100_000.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_calculate_with_vat() {
        let amount = KrwAmount::from_man(100.0);
        let result = calculate_with_vat(&amount);
        assert!(result.is_ok());

        if let Ok(total) = result {
            assert!((total.won - 1_100_000.0).abs() < 0.01);
        }
    }
}
