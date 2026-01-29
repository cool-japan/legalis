//! Tax Code of the Russian Federation (Налоговый кодекс РФ).
//!
//! Federal Law No. 146-FZ of July 31, 1998 (Part 1)
//! Federal Law No. 117-FZ of August 5, 2000 (Part 2)
//!
//! This module provides:
//! - VAT (НДС) - 20% standard rate
//! - Personal Income Tax (НДФЛ) - 13% standard rate
//! - Corporate Tax (Налог на прибыль) - 20% rate

pub mod corporate_tax;
pub mod income_tax;
pub mod vat;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use corporate_tax::{CorporateTaxCalculation, calculate_corporate_tax};
pub use income_tax::{IncomeTaxCalculation, calculate_income_tax};
pub use vat::{VatCalculation, VatRate, calculate_vat};

/// Errors related to Tax Code operations
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum TaxCodeError {
    /// Invalid tax calculation
    #[error("Invalid tax calculation: {0}")]
    InvalidCalculation(String),

    /// Invalid tax rate
    #[error("Invalid tax rate: {0}")]
    InvalidRate(String),

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Tax rate representation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TaxRate {
    /// Rate as percentage (e.g., 20.0 for 20%)
    pub percentage: f64,
    /// Description in Russian
    pub description_ru: &'static str,
    /// Description in English
    pub description_en: &'static str,
}

impl TaxRate {
    /// Standard VAT rate (20%)
    pub const VAT_STANDARD: Self = Self {
        percentage: 20.0,
        description_ru: "Основная ставка НДС",
        description_en: "Standard VAT rate",
    };

    /// Reduced VAT rate (10%)
    pub const VAT_REDUCED: Self = Self {
        percentage: 10.0,
        description_ru: "Пониженная ставка НДС",
        description_en: "Reduced VAT rate",
    };

    /// Zero VAT rate (0%)
    pub const VAT_ZERO: Self = Self {
        percentage: 0.0,
        description_ru: "Нулевая ставка НДС",
        description_en: "Zero VAT rate",
    };

    /// Standard personal income tax rate (13%)
    pub const INCOME_TAX_STANDARD: Self = Self {
        percentage: 13.0,
        description_ru: "Основная ставка НДФЛ",
        description_en: "Standard personal income tax rate",
    };

    /// Progressive income tax rate for high earners (15%)
    pub const INCOME_TAX_PROGRESSIVE: Self = Self {
        percentage: 15.0,
        description_ru: "Повышенная ставка НДФЛ",
        description_en: "Progressive personal income tax rate",
    };

    /// Corporate tax rate (20%)
    pub const CORPORATE_TAX: Self = Self {
        percentage: 20.0,
        description_ru: "Налог на прибыль организаций",
        description_en: "Corporate income tax rate",
    };

    /// Applies the tax rate to an amount
    pub fn apply(&self, amount: &crate::common::Currency) -> crate::common::Currency {
        amount.multiply_percentage(self.percentage)
    }
}

/// Quick validation for tax calculation
pub fn quick_validate_tax_calculation(
    base_amount: &crate::common::Currency,
    tax_rate: &TaxRate,
) -> Result<crate::common::Currency, TaxCodeError> {
    if !base_amount.is_positive() {
        return Err(TaxCodeError::InvalidCalculation(
            "Tax base must be positive".to_string(),
        ));
    }

    if tax_rate.percentage < 0.0 || tax_rate.percentage > 100.0 {
        return Err(TaxCodeError::InvalidRate(
            "Tax rate must be between 0 and 100 percent".to_string(),
        ));
    }

    Ok(tax_rate.apply(base_amount))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_rates() {
        assert_eq!(TaxRate::VAT_STANDARD.percentage, 20.0);
        assert_eq!(TaxRate::INCOME_TAX_STANDARD.percentage, 13.0);
        assert_eq!(TaxRate::CORPORATE_TAX.percentage, 20.0);
    }

    #[test]
    fn test_tax_rate_application() {
        let amount = crate::common::Currency::from_rubles(1000);
        let vat = TaxRate::VAT_STANDARD.apply(&amount);
        assert_eq!(vat.rubles(), 200); // 20% of 1000
    }

    #[test]
    fn test_tax_validation() {
        let amount = crate::common::Currency::from_rubles(1000);
        let result = quick_validate_tax_calculation(&amount, &TaxRate::VAT_STANDARD);
        assert!(result.is_ok());

        let zero_amount = crate::common::Currency::from_rubles(0);
        let result = quick_validate_tax_calculation(&zero_amount, &TaxRate::VAT_STANDARD);
        assert!(result.is_err());
    }
}
