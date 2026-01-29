//! Christmas bonus (Aguinaldo)
//!
//! Federal Labor Law Article 87

use crate::common::MexicanCurrency;
use chrono::{Datelike, NaiveDate};

/// Minimum aguinaldo days (Article 87)
pub const MINIMUM_DAYS: u8 = 15;

/// Calculate aguinaldo amount
///
/// Article 87: Workers are entitled to a Christmas bonus
/// equivalent to at least 15 days of salary, payable before December 20
pub fn calculate_aguinaldo(daily_salary: MexicanCurrency, days_worked: u16) -> MexicanCurrency {
    if days_worked >= 365 {
        // Full year: 15 days minimum
        MexicanCurrency::from_centavos(daily_salary.centavos * MINIMUM_DAYS as i64)
    } else {
        // Proportional to days worked
        let proportional_days = (MINIMUM_DAYS as f64 * days_worked as f64) / 365.0;
        MexicanCurrency::from_centavos((daily_salary.centavos as f64 * proportional_days) as i64)
    }
}

/// Check if aguinaldo payment is overdue
pub fn is_payment_overdue(current_date: NaiveDate) -> bool {
    current_date.year() == current_date.year()
        && current_date.month() == 12
        && current_date.day() > 20
}

/// Calculate proportional aguinaldo for partial year
pub fn calculate_proportional(
    daily_salary: MexicanCurrency,
    days_worked: u16,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> MexicanCurrency {
    let total_days = (end_date - start_date).num_days().max(0) as u16;
    calculate_aguinaldo(daily_salary, total_days.min(days_worked))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_aguinaldo_full_year() {
        let daily_salary = MexicanCurrency::from_pesos(300);
        let aguinaldo = calculate_aguinaldo(daily_salary, 365);

        // 15 days * 300 pesos = 4,500 pesos
        assert_eq!(aguinaldo.pesos(), 4500);
    }

    #[test]
    fn test_calculate_aguinaldo_partial_year() {
        let daily_salary = MexicanCurrency::from_pesos(300);
        let aguinaldo = calculate_aguinaldo(daily_salary, 182); // ~6 months

        // Should be approximately half of full aguinaldo
        assert!(aguinaldo.pesos() >= 2200 && aguinaldo.pesos() <= 2300);
    }

    #[test]
    fn test_payment_deadline() {
        let before_deadline = NaiveDate::from_ymd_opt(2024, 12, 15).unwrap();
        let after_deadline = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();

        assert!(!is_payment_overdue(before_deadline));
        assert!(is_payment_overdue(after_deadline));
    }
}
