//! Income Tax Act (소득세법)
//!
//! # 소득세법 / Income Tax Act
//!
//! Progressive tax rates on individual income

use crate::common::KrwAmount;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Income tax errors
#[derive(Debug, Error, Clone, PartialEq)]
pub enum IncomeTaxError {
    /// Calculation error
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Result type for income tax operations
pub type IncomeTaxResult<T> = Result<T, IncomeTaxError>;

/// Income tax bracket
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxBracket {
    /// Lower bound
    pub lower_bound: KrwAmount,
    /// Upper bound (None for highest bracket)
    pub upper_bound: Option<KrwAmount>,
    /// Tax rate
    pub rate: f64,
}

/// Get 2024 income tax brackets
pub fn get_tax_brackets() -> Vec<TaxBracket> {
    vec![
        TaxBracket {
            lower_bound: KrwAmount::new(0.0),
            upper_bound: Some(KrwAmount::from_man(1_400.0)),
            rate: 0.06,
        },
        TaxBracket {
            lower_bound: KrwAmount::from_man(1_400.0),
            upper_bound: Some(KrwAmount::from_man(5_000.0)),
            rate: 0.15,
        },
        TaxBracket {
            lower_bound: KrwAmount::from_man(5_000.0),
            upper_bound: Some(KrwAmount::from_man(8_800.0)),
            rate: 0.24,
        },
        TaxBracket {
            lower_bound: KrwAmount::from_man(8_800.0),
            upper_bound: Some(KrwAmount::from_eok(1.5)),
            rate: 0.35,
        },
        TaxBracket {
            lower_bound: KrwAmount::from_eok(1.5),
            upper_bound: Some(KrwAmount::from_eok(3.0)),
            rate: 0.38,
        },
        TaxBracket {
            lower_bound: KrwAmount::from_eok(3.0),
            upper_bound: Some(KrwAmount::from_eok(5.0)),
            rate: 0.40,
        },
        TaxBracket {
            lower_bound: KrwAmount::from_eok(5.0),
            upper_bound: Some(KrwAmount::from_eok(10.0)),
            rate: 0.42,
        },
        TaxBracket {
            lower_bound: KrwAmount::from_eok(10.0),
            upper_bound: None,
            rate: 0.45,
        },
    ]
}

/// Calculate income tax
pub fn calculate_income_tax(taxable_income: &KrwAmount) -> IncomeTaxResult<KrwAmount> {
    let brackets = get_tax_brackets();
    let mut tax = 0.0;
    let income = taxable_income.won;

    for bracket in brackets {
        let lower = bracket.lower_bound.won;
        let upper = bracket.upper_bound.as_ref().map(|u| u.won);

        if income <= lower {
            break;
        }

        let taxable_in_bracket = if let Some(upper_bound) = upper {
            if income > upper_bound {
                upper_bound - lower
            } else {
                income - lower
            }
        } else {
            income - lower
        };

        tax += taxable_in_bracket * bracket.rate;
    }

    Ok(KrwAmount::new(tax))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_income_tax() {
        let income = KrwAmount::from_man(5_000.0);
        let result = calculate_income_tax(&income);
        assert!(result.is_ok());
    }
}
