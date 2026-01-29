//! Income Tax Act 1967
//!
//! Personal and corporate income tax in Malaysia.
//!
//! # Personal Income Tax (2024 rates)
//!
//! Progressive tax rates from 0% to 30%:
//! - RM 0 - 5,000: 0%
//! - RM 5,001 - 20,000: 1%
//! - RM 20,001 - 35,000: 3%
//! - RM 35,001 - 50,000: 6%
//! - RM 50,001 - 70,000: 11%
//! - RM 70,001 - 100,000: 19%
//! - RM 100,001 - 250,000: 25%
//! - RM 250,001 - 400,000: 26%
//! - RM 400,001 - 600,000: 28%
//! - RM 600,001 - 1,000,000: 30%
//! - Above RM 1,000,000: 30%
//!
//! # Corporate Income Tax
//!
//! - Standard rate: 24%
//! - Small and medium enterprises (SME): Graduated rates

use serde::{Deserialize, Serialize};

/// Income tax bracket.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IncomeTaxBracket {
    /// Minimum income in sen (inclusive).
    pub min_sen: i64,
    /// Maximum income in sen (exclusive, None for top bracket).
    pub max_sen: Option<i64>,
    /// Tax rate (percentage).
    pub rate: f64,
}

impl IncomeTaxBracket {
    /// Creates a new tax bracket.
    #[must_use]
    pub const fn new(min_sen: i64, max_sen: Option<i64>, rate: f64) -> Self {
        Self {
            min_sen,
            max_sen,
            rate,
        }
    }

    /// Checks if income falls within this bracket.
    #[must_use]
    pub fn contains(&self, income_sen: i64) -> bool {
        income_sen >= self.min_sen && self.max_sen.is_none_or(|max| income_sen < max)
    }

    /// Calculates tax for this bracket.
    #[must_use]
    pub fn calculate_tax(&self, income_sen: i64) -> i64 {
        if !self.contains(income_sen) {
            return 0;
        }

        let taxable = if let Some(max) = self.max_sen {
            (income_sen.min(max) - self.min_sen).max(0)
        } else {
            (income_sen - self.min_sen).max(0)
        };

        ((taxable as f64) * (self.rate / 100.0)).round() as i64
    }
}

/// Personal income tax calculator.
#[derive(Debug, Clone)]
pub struct IncomeTax {
    /// Tax brackets.
    brackets: Vec<IncomeTaxBracket>,
}

impl Default for IncomeTax {
    fn default() -> Self {
        Self::new()
    }
}

impl IncomeTax {
    /// Creates a new income tax calculator with 2024 rates.
    #[must_use]
    pub fn new() -> Self {
        let brackets = vec![
            IncomeTaxBracket::new(0, Some(500_000), 0.0), // RM 0 - 5,000: 0%
            IncomeTaxBracket::new(500_000, Some(2000000), 1.0), // RM 5,000 - 20,000: 1%
            IncomeTaxBracket::new(2000000, Some(3500000), 3.0), // RM 20,000 - 35,000: 3%
            IncomeTaxBracket::new(3500000, Some(5000000), 6.0), // RM 35,000 - 50,000: 6%
            IncomeTaxBracket::new(5000000, Some(7000000), 11.0), // RM 50,000 - 70,000: 11%
            IncomeTaxBracket::new(7000000, Some(10000000), 19.0), // RM 70,000 - 100,000: 19%
            IncomeTaxBracket::new(10000000, Some(25000000), 25.0), // RM 100,000 - 250,000: 25%
            IncomeTaxBracket::new(25000000, Some(40000000), 26.0), // RM 250,000 - 400,000: 26%
            IncomeTaxBracket::new(40000000, Some(60000000), 28.0), // RM 400,000 - 600,000: 28%
            IncomeTaxBracket::new(60000000, Some(100000000), 30.0), // RM 600,000 - 1,000,000: 30%
            IncomeTaxBracket::new(100000000, None, 30.0), // Above RM 1,000,000: 30%
        ];

        Self { brackets }
    }

    /// Calculates total income tax.
    #[must_use]
    pub fn calculate(&self, income_sen: i64) -> i64 {
        let mut total_tax = 0;
        let remaining_income = income_sen;

        for bracket in &self.brackets {
            if remaining_income <= bracket.min_sen {
                break;
            }

            let bracket_income = if let Some(max) = bracket.max_sen {
                remaining_income.min(max) - bracket.min_sen
            } else {
                remaining_income - bracket.min_sen
            };

            if bracket_income > 0 {
                let bracket_tax = ((bracket_income as f64) * (bracket.rate / 100.0)).round() as i64;
                total_tax += bracket_tax;
            }
        }

        total_tax
    }

    /// Calculates effective tax rate.
    #[must_use]
    pub fn effective_rate(&self, income_sen: i64) -> f64 {
        if income_sen == 0 {
            return 0.0;
        }

        let tax = self.calculate(income_sen);
        ((tax as f64) / (income_sen as f64)) * 100.0
    }
}

/// Calculates personal income tax.
#[must_use]
pub fn calculate_income_tax(income_sen: i64) -> i64 {
    let calculator = IncomeTax::new();
    calculator.calculate(income_sen)
}

/// Corporate income tax rate.
#[must_use]
pub fn corporate_tax_rate() -> f64 {
    24.0 // Standard corporate tax rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_income_tax_calculation() {
        let calculator = IncomeTax::new();

        // RM 10,000 income
        let tax1 = calculator.calculate(1000000);
        assert_eq!(tax1, 5_000); // RM 50 (1% of RM 5,000)

        // RM 50,000 income
        // Calculation: 0 + 150 + 450 + 900 = RM 1,500
        let tax2 = calculator.calculate(5000000);
        assert_eq!(tax2, 150_000); // RM 1,500

        // RM 100,000 income
        // Calculation: 0 + 150 + 450 + 900 + 2,200 + 5,700 = RM 9,400
        let tax3 = calculator.calculate(10000000);
        assert_eq!(tax3, 940_000); // RM 9,400
    }

    #[test]
    fn test_effective_rate() {
        let calculator = IncomeTax::new();

        // RM 100,000 income
        let rate = calculator.effective_rate(10000000);
        assert!((rate - 9.4).abs() < 0.1); // ~9.4% effective rate
    }

    #[test]
    fn test_no_tax_below_threshold() {
        let calculator = IncomeTax::new();

        // RM 3,000 income (below RM 5,000 threshold)
        let tax = calculator.calculate(300_000);
        assert_eq!(tax, 0);
    }
}
