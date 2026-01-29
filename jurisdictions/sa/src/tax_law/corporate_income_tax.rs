//! Corporate Income Tax (ضريبة الدخل)
//!
//! Royal Decree No. M/1 dated 15/1/1425H (2004)
//!
//! Applies to:
//! - Foreign companies (non-GCC)
//! - Companies with foreign shareholders
//! - Natural gas investment companies
//!
//! Rate: 20% (85% for oil and hydrocarbon production)

use crate::common::Sar;
use serde::{Deserialize, Serialize};

/// Corporate income tax rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorporateTaxRate {
    /// Standard rate (20%)
    Standard,
    /// Oil and gas production (85%)
    OilAndGas,
}

impl CorporateTaxRate {
    /// Get rate as percentage
    pub fn rate(&self) -> f64 {
        match self {
            Self::Standard => 20.0,
            Self::OilAndGas => 85.0,
        }
    }

    /// Get description
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Standard => "20% for foreign companies and mixed ownership",
            Self::OilAndGas => "85% for oil and hydrocarbon production companies",
        }
    }
}

/// Corporate income tax structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateIncomeTax {
    /// Taxable income
    pub taxable_income: Sar,
    /// Tax rate applied
    pub tax_rate: CorporateTaxRate,
    /// Calculated tax amount
    pub tax_amount: Sar,
    /// Foreign ownership percentage
    pub foreign_ownership_pct: f64,
}

/// Calculate corporate income tax
pub fn calculate_corporate_tax(
    net_income: Sar,
    foreign_ownership_pct: f64,
    is_oil_and_gas: bool,
) -> CorporateIncomeTax {
    let rate = if is_oil_and_gas {
        CorporateTaxRate::OilAndGas
    } else {
        CorporateTaxRate::Standard
    };

    // For mixed ownership, tax applies only to foreign portion
    let taxable_income = if foreign_ownership_pct > 0.0 && foreign_ownership_pct < 100.0 {
        let foreign_portion = foreign_ownership_pct / 100.0;
        Sar::from_halalas((net_income.halalas() as f64 * foreign_portion).round() as i64)
    } else if foreign_ownership_pct == 100.0 {
        net_income
    } else {
        Sar::from_halalas(0) // No tax for fully Saudi/GCC owned
    };

    let tax_rate_decimal = rate.rate() / 100.0;
    let tax_halalas = (taxable_income.halalas() as f64 * tax_rate_decimal).round() as i64;
    let tax_amount = Sar::from_halalas(tax_halalas);

    CorporateIncomeTax {
        taxable_income,
        tax_rate: rate,
        tax_amount,
        foreign_ownership_pct,
    }
}

/// Calculate effective tax rate
pub fn calculate_effective_rate(tax_paid: Sar, net_income: Sar) -> f64 {
    if net_income.is_zero() {
        return 0.0;
    }
    (tax_paid.as_decimal() / net_income.as_decimal()) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_rates() {
        assert_eq!(CorporateTaxRate::Standard.rate(), 20.0);
        assert_eq!(CorporateTaxRate::OilAndGas.rate(), 85.0);
    }

    #[test]
    fn test_fully_foreign_company() {
        let tax = calculate_corporate_tax(Sar::from_riyals(1_000_000), 100.0, false);
        assert_eq!(tax.taxable_income.riyals(), 1_000_000);
        assert_eq!(tax.tax_amount.riyals(), 200_000); // 20% of 1M
    }

    #[test]
    fn test_mixed_ownership() {
        let tax = calculate_corporate_tax(
            Sar::from_riyals(1_000_000),
            40.0, // 40% foreign
            false,
        );
        assert_eq!(tax.taxable_income.riyals(), 400_000); // 40% of 1M
        assert_eq!(tax.tax_amount.riyals(), 80_000); // 20% of 400K
    }

    #[test]
    fn test_fully_saudi_company() {
        let tax = calculate_corporate_tax(
            Sar::from_riyals(1_000_000),
            0.0, // 0% foreign
            false,
        );
        assert_eq!(tax.taxable_income.riyals(), 0);
        assert_eq!(tax.tax_amount.riyals(), 0);
    }

    #[test]
    fn test_oil_and_gas_company() {
        let tax = calculate_corporate_tax(
            Sar::from_riyals(1_000_000),
            100.0,
            true, // Oil and gas
        );
        assert_eq!(tax.taxable_income.riyals(), 1_000_000);
        assert_eq!(tax.tax_amount.riyals(), 850_000); // 85% of 1M
    }

    #[test]
    fn test_effective_rate() {
        let rate = calculate_effective_rate(Sar::from_riyals(200_000), Sar::from_riyals(1_000_000));
        assert!((rate - 20.0).abs() < 0.01);
    }
}
