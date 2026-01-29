//! Income Tax Law (Ley del Impuesto Sobre la Renta)

use crate::common::MexicanCurrency;

/// ISR tax brackets for individuals (2024)
pub const TAX_BRACKETS: &[(i64, i64, f64)] = &[
    // (lower_limit, upper_limit, rate)
    (0, 746400, 0.0192),
    (746401, 634500, 0.064),
    (634501, 1111100, 0.1088),
    (1111101, 1292800, 0.16),
    (1292801, 1555200, 0.1792),
    (1555201, 1867500, 0.2136),
    (1867501, 2242800, 0.2352),
    (2242801, 2537700, 0.30),
    (2537701, 4446600, 0.32),
    (4446601, 7399400, 0.34),
    (7399401, i64::MAX, 0.35),
];

/// Corporate tax rate (Article 9)
pub const CORPORATE_TAX_RATE: f64 = 0.30; // 30%

/// Calculate ISR for individuals
pub fn calculate_individual_isr(annual_income: MexicanCurrency) -> MexicanCurrency {
    let income = annual_income.centavos;
    let mut tax = 0i64;
    let mut accumulated = 0i64;

    for (_lower, bracket_size, rate) in TAX_BRACKETS {
        if income > accumulated + bracket_size {
            // Full bracket applies
            tax += (*bracket_size as f64 * rate) as i64;
            accumulated += bracket_size;
        } else if income > accumulated {
            // Partial bracket applies
            let taxable_in_bracket = income - accumulated;
            tax += (taxable_in_bracket as f64 * rate) as i64;
            break;
        }
    }

    MexicanCurrency::from_centavos(tax)
}

/// Calculate corporate ISR
pub fn calculate_corporate_isr(annual_income: MexicanCurrency) -> MexicanCurrency {
    let tax = (annual_income.centavos as f64 * CORPORATE_TAX_RATE) as i64;
    MexicanCurrency::from_centavos(tax)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_individual_isr() {
        let income = MexicanCurrency::from_pesos(100000);
        let tax = calculate_individual_isr(income);
        assert!(tax.pesos() > 0);
    }

    #[test]
    fn test_calculate_corporate_isr() {
        let income = MexicanCurrency::from_pesos(1000000);
        let tax = calculate_corporate_isr(income);

        // 30% of 1,000,000 = 300,000
        assert_eq!(tax.pesos(), 300000);
    }
}
