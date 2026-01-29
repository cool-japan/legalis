//! Vacation rights (Vacaciones)
//!
//! Federal Labor Law Articles 76-81

use crate::common::MexicanCurrency;

/// Vacation days by years of service (Article 76)
pub fn get_vacation_days(years_of_service: u8) -> u8 {
    match years_of_service {
        1 => 12,
        2 => 14,
        3 => 16,
        4 => 18,
        5..=9 => 20,
        10..=14 => 22,
        15..=19 => 24,
        20..=24 => 26,
        25..=29 => 28,
        _ => 30, // 30+ years
    }
}

/// Vacation premium percentage (Article 80)
pub const VACATION_PREMIUM_PERCENT: u8 = 25;

/// Calculate vacation premium (prima vacacional)
///
/// Article 80: Workers receive a premium of at least 25%
/// of the salary corresponding to their vacation period
pub fn calculate_vacation_premium(
    daily_salary: MexicanCurrency,
    vacation_days: u8,
) -> MexicanCurrency {
    let vacation_salary =
        MexicanCurrency::from_centavos(daily_salary.centavos * vacation_days as i64);
    let premium_amount = (vacation_salary.centavos * VACATION_PREMIUM_PERCENT as i64) / 100;
    MexicanCurrency::from_centavos(premium_amount)
}

/// Calculate total vacation compensation (salary + premium)
pub fn calculate_total_vacation_compensation(
    daily_salary: MexicanCurrency,
    years_of_service: u8,
) -> MexicanCurrency {
    let vacation_days = get_vacation_days(years_of_service);
    let vacation_salary =
        MexicanCurrency::from_centavos(daily_salary.centavos * vacation_days as i64);
    let premium = calculate_vacation_premium(daily_salary, vacation_days);

    MexicanCurrency::from_centavos(vacation_salary.centavos + premium.centavos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacation_days() {
        assert_eq!(get_vacation_days(1), 12);
        assert_eq!(get_vacation_days(2), 14);
        assert_eq!(get_vacation_days(5), 20);
        assert_eq!(get_vacation_days(30), 30);
    }

    #[test]
    fn test_vacation_premium() {
        let daily_salary = MexicanCurrency::from_pesos(300);
        let vacation_days = 12;

        // 12 days * 300 = 3,600
        // Premium: 3,600 * 0.25 = 900
        let premium = calculate_vacation_premium(daily_salary, vacation_days);
        assert_eq!(premium.pesos(), 900);
    }

    #[test]
    fn test_total_compensation() {
        let daily_salary = MexicanCurrency::from_pesos(300);
        let total = calculate_total_vacation_compensation(daily_salary, 1);

        // 12 days * 300 = 3,600
        // Premium: 900
        // Total: 4,500
        assert_eq!(total.pesos(), 4500);
    }
}
