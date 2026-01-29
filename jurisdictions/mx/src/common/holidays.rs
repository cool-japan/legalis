//! Mexican federal holidays and observances

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Mexican federal holidays (Días Festivos Oficiales)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FederalHoliday {
    /// New Year's Day (Año Nuevo) - January 1
    NewYear,
    /// Constitution Day (Día de la Constitución) - First Monday of February
    ConstitutionDay,
    /// Benito Juárez's Birthday (Natalicio de Benito Juárez) - Third Monday of March
    BenitoJuarezBirthday,
    /// Labor Day (Día del Trabajo) - May 1
    LaborDay,
    /// Independence Day (Día de la Independencia) - September 16
    IndependenceDay,
    /// Revolution Day (Día de la Revolución) - Third Monday of November
    RevolutionDay,
    /// Christmas (Navidad) - December 25
    Christmas,
    /// Presidential Transition (Transmisión del Poder Ejecutivo) - Every 6 years, December 1
    PresidentialTransition,
}

impl FederalHoliday {
    /// Get the holiday name in Spanish
    pub fn nombre_es(&self) -> &'static str {
        match self {
            FederalHoliday::NewYear => "Año Nuevo",
            FederalHoliday::ConstitutionDay => "Día de la Constitución",
            FederalHoliday::BenitoJuarezBirthday => "Natalicio de Benito Juárez",
            FederalHoliday::LaborDay => "Día del Trabajo",
            FederalHoliday::IndependenceDay => "Día de la Independencia",
            FederalHoliday::RevolutionDay => "Día de la Revolución",
            FederalHoliday::Christmas => "Navidad",
            FederalHoliday::PresidentialTransition => "Transmisión del Poder Ejecutivo",
        }
    }

    /// Get the holiday name in English
    pub fn name_en(&self) -> &'static str {
        match self {
            FederalHoliday::NewYear => "New Year's Day",
            FederalHoliday::ConstitutionDay => "Constitution Day",
            FederalHoliday::BenitoJuarezBirthday => "Benito Juárez's Birthday",
            FederalHoliday::LaborDay => "Labor Day",
            FederalHoliday::IndependenceDay => "Independence Day",
            FederalHoliday::RevolutionDay => "Revolution Day",
            FederalHoliday::Christmas => "Christmas",
            FederalHoliday::PresidentialTransition => "Presidential Transition",
        }
    }

    /// Check if a date is this federal holiday
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        match self {
            FederalHoliday::NewYear => date.month() == 1 && date.day() == 1,
            FederalHoliday::ConstitutionDay => {
                is_nth_weekday_of_month(date, 2, chrono::Weekday::Mon, 1)
            }
            FederalHoliday::BenitoJuarezBirthday => {
                is_nth_weekday_of_month(date, 3, chrono::Weekday::Mon, 3)
            }
            FederalHoliday::LaborDay => date.month() == 5 && date.day() == 1,
            FederalHoliday::IndependenceDay => date.month() == 9 && date.day() == 16,
            FederalHoliday::RevolutionDay => {
                is_nth_weekday_of_month(date, 11, chrono::Weekday::Mon, 3)
            }
            FederalHoliday::Christmas => date.month() == 12 && date.day() == 25,
            FederalHoliday::PresidentialTransition => {
                // Every 6 years (2024, 2030, 2036, etc.)
                date.month() == 12 && date.day() == 1 && (date.year() - 2024) % 6 == 0
            }
        }
    }
}

/// Check if a date is a federal holiday
pub fn is_federal_holiday(date: NaiveDate) -> bool {
    [
        FederalHoliday::NewYear,
        FederalHoliday::ConstitutionDay,
        FederalHoliday::BenitoJuarezBirthday,
        FederalHoliday::LaborDay,
        FederalHoliday::IndependenceDay,
        FederalHoliday::RevolutionDay,
        FederalHoliday::Christmas,
    ]
    .iter()
    .any(|holiday| holiday.is_holiday(date))
}

/// Helper function to check if a date is the nth occurrence of a weekday in a month
fn is_nth_weekday_of_month(date: NaiveDate, month: u32, weekday: chrono::Weekday, n: u32) -> bool {
    if date.month() != month {
        return false;
    }

    if date.weekday() != weekday {
        return false;
    }

    // Calculate which occurrence of the weekday this is
    let day = date.day();
    let occurrence = (day - 1) / 7 + 1;

    occurrence == n
}

/// Get all federal holidays for a given year
pub fn get_federal_holidays(year: i32) -> Vec<(FederalHoliday, NaiveDate)> {
    let mut holidays = Vec::new();

    // New Year
    if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
        holidays.push((FederalHoliday::NewYear, date));
    }

    // Constitution Day (First Monday of February)
    if let Some(date) = find_nth_weekday(year, 2, chrono::Weekday::Mon, 1) {
        holidays.push((FederalHoliday::ConstitutionDay, date));
    }

    // Benito Juárez (Third Monday of March)
    if let Some(date) = find_nth_weekday(year, 3, chrono::Weekday::Mon, 3) {
        holidays.push((FederalHoliday::BenitoJuarezBirthday, date));
    }

    // Labor Day
    if let Some(date) = NaiveDate::from_ymd_opt(year, 5, 1) {
        holidays.push((FederalHoliday::LaborDay, date));
    }

    // Independence Day
    if let Some(date) = NaiveDate::from_ymd_opt(year, 9, 16) {
        holidays.push((FederalHoliday::IndependenceDay, date));
    }

    // Revolution Day (Third Monday of November)
    if let Some(date) = find_nth_weekday(year, 11, chrono::Weekday::Mon, 3) {
        holidays.push((FederalHoliday::RevolutionDay, date));
    }

    // Christmas
    if let Some(date) = NaiveDate::from_ymd_opt(year, 12, 25) {
        holidays.push((FederalHoliday::Christmas, date));
    }

    // Presidential Transition (every 6 years)
    if (year - 2024) % 6 == 0
        && let Some(date) = NaiveDate::from_ymd_opt(year, 12, 1)
    {
        holidays.push((FederalHoliday::PresidentialTransition, date));
    }

    holidays
}

/// Find the nth occurrence of a weekday in a month
fn find_nth_weekday(year: i32, month: u32, weekday: chrono::Weekday, n: u32) -> Option<NaiveDate> {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1)?;
    let first_weekday = first_day.weekday();

    // Calculate days until the first occurrence of the target weekday
    let days_until_first =
        (weekday.num_days_from_monday() + 7 - first_weekday.num_days_from_monday()) % 7;

    // Calculate the day of the month for the nth occurrence
    let target_day = 1 + days_until_first + (n - 1) * 7;

    NaiveDate::from_ymd_opt(year, month, target_day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_year() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert!(FederalHoliday::NewYear.is_holiday(date));
    }

    #[test]
    fn test_independence_day() {
        let date = NaiveDate::from_ymd_opt(2024, 9, 16).unwrap();
        assert!(FederalHoliday::IndependenceDay.is_holiday(date));
    }

    #[test]
    fn test_labor_day() {
        let date = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
        assert!(FederalHoliday::LaborDay.is_holiday(date));
    }

    #[test]
    fn test_get_federal_holidays() {
        let holidays = get_federal_holidays(2024);
        assert!(holidays.len() >= 7); // At least 7 holidays
    }
}
