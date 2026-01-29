//! Corporate Tax Act (법인세법)
//!
//! # 법인세법 / Corporate Tax Act
//!
//! Tax on corporate income

use crate::common::KrwAmount;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Corporate tax errors
#[derive(Debug, Error, Clone, PartialEq)]
pub enum CorporateTaxError {
    /// Calculation error
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Result type for corporate tax operations
pub type CorporateTaxResult<T> = Result<T, CorporateTaxError>;

/// Corporate tax rate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorporateTaxRate {
    /// Small business (under 200M KRW)
    SmallBusiness,
    /// Medium business (200M-20B KRW)
    MediumBusiness,
    /// Large business (20B-300B KRW)
    LargeBusiness,
    /// Very large business (over 300B KRW)
    VeryLargeBusiness,
}

/// Get corporate tax rate
pub fn get_tax_rate(taxable_income: &KrwAmount) -> f64 {
    if taxable_income.won <= 200_000_000.0 {
        0.10 // 10%
    } else if taxable_income.won <= 20_000_000_000.0 {
        0.20 // 20%
    } else if taxable_income.won <= 300_000_000_000.0 {
        0.22 // 22%
    } else {
        0.25 // 25%
    }
}

/// Calculate corporate tax
pub fn calculate_corporate_tax(taxable_income: &KrwAmount) -> CorporateTaxResult<KrwAmount> {
    let mut tax = 0.0;

    if taxable_income.won <= 0.0 {
        return Ok(KrwAmount::new(0.0));
    }

    // Progressive calculation
    if taxable_income.won > 300_000_000_000.0 {
        tax += (taxable_income.won - 300_000_000_000.0) * 0.25;
        tax += (300_000_000_000.0 - 20_000_000_000.0) * 0.22;
        tax += (20_000_000_000.0 - 200_000_000.0) * 0.20;
        tax += 200_000_000.0 * 0.10;
    } else if taxable_income.won > 20_000_000_000.0 {
        tax += (taxable_income.won - 20_000_000_000.0) * 0.22;
        tax += (20_000_000_000.0 - 200_000_000.0) * 0.20;
        tax += 200_000_000.0 * 0.10;
    } else if taxable_income.won > 200_000_000.0 {
        tax += (taxable_income.won - 200_000_000.0) * 0.20;
        tax += 200_000_000.0 * 0.10;
    } else {
        tax += taxable_income.won * 0.10;
    }

    Ok(KrwAmount::new(tax))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tax_rate() {
        let income1 = KrwAmount::from_eok(1.0);
        assert_eq!(get_tax_rate(&income1), 0.10);

        let income2 = KrwAmount::from_eok(100.0);
        assert_eq!(get_tax_rate(&income2), 0.20);
    }

    #[test]
    fn test_calculate_corporate_tax() {
        let income = KrwAmount::from_eok(10.0);
        let result = calculate_corporate_tax(&income);
        assert!(result.is_ok());
    }
}
