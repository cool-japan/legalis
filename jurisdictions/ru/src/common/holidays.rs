//! Russian federal holidays and working day calculations.
//!
//! According to the Labor Code of the Russian Federation, the following are
//! non-working holidays:
//! - New Year holidays (January 1-8)
//! - Orthodox Christmas (January 7, included in New Year holidays)
//! - Defender of the Fatherland Day (February 23)
//! - International Women's Day (March 8)
//! - Spring and Labour Day (May 1)
//! - Victory Day (May 9)
//! - Russia Day (June 12)
//! - Unity Day (November 4)

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Russian federal holiday
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Holiday {
    /// Month (1-12)
    pub month: u32,
    /// Day (1-31)
    pub day: u32,
    /// Name in Russian
    pub name_ru: String,
    /// Name in English
    pub name_en: String,
}

impl Holiday {
    /// Creates a new holiday
    pub fn new(
        month: u32,
        day: u32,
        name_ru: impl Into<String>,
        name_en: impl Into<String>,
    ) -> Self {
        Self {
            month,
            day,
            name_ru: name_ru.into(),
            name_en: name_en.into(),
        }
    }

    /// Checks if the given date matches this holiday
    pub fn matches(&self, date: &NaiveDate) -> bool {
        date.month() == self.month && date.day() == self.day
    }
}

/// Russian legal calendar system
pub struct RussianLegalCalendar;

impl RussianLegalCalendar {
    /// Returns all federal holidays
    pub fn federal_holidays() -> Vec<Holiday> {
        vec![
            // New Year holidays (January 1-8)
            Holiday::new(1, 1, "Новый год", "New Year's Day"),
            Holiday::new(1, 2, "Новогодние каникулы", "New Year Holidays"),
            Holiday::new(1, 3, "Новогодние каникулы", "New Year Holidays"),
            Holiday::new(1, 4, "Новогодние каникулы", "New Year Holidays"),
            Holiday::new(1, 5, "Новогодние каникулы", "New Year Holidays"),
            Holiday::new(1, 6, "Новогодние каникулы", "New Year Holidays"),
            Holiday::new(1, 7, "Рождество Христово", "Orthodox Christmas"),
            Holiday::new(1, 8, "Новогодние каникулы", "New Year Holidays"),
            // February
            Holiday::new(
                2,
                23,
                "День защитника Отечества",
                "Defender of the Fatherland Day",
            ),
            // March
            Holiday::new(
                3,
                8,
                "Международный женский день",
                "International Women's Day",
            ),
            // May
            Holiday::new(5, 1, "Праздник Весны и Труда", "Spring and Labour Day"),
            Holiday::new(5, 9, "День Победы", "Victory Day"),
            // June
            Holiday::new(6, 12, "День России", "Russia Day"),
            // November
            Holiday::new(11, 4, "День народного единства", "Unity Day"),
        ]
    }

    /// Gets the holiday for a given date, if any
    pub fn get_holiday(date: &NaiveDate) -> Option<Holiday> {
        Self::federal_holidays()
            .into_iter()
            .find(|h| h.matches(date))
    }
}

/// Checks if a given date is a Russian federal holiday
pub fn is_russian_holiday(date: &NaiveDate) -> bool {
    RussianLegalCalendar::get_holiday(date).is_some()
}

/// Checks if a given date is a working day (not weekend or holiday)
pub fn is_working_day(date: &NaiveDate) -> bool {
    // Check if weekend
    if matches!(date.weekday(), Weekday::Sat | Weekday::Sun) {
        return false;
    }

    // Check if federal holiday
    !is_russian_holiday(date)
}

/// Calculates the number of business days between two dates (inclusive)
pub fn calculate_business_days(start: &NaiveDate, end: &NaiveDate) -> i64 {
    let mut count = 0;
    let mut current = *start;

    while current <= *end {
        if is_working_day(&current) {
            count += 1;
        }
        current = current
            .succ_opt()
            .unwrap_or_else(|| panic!("Date overflow"));
    }

    count
}

/// Adds business days to a date
pub fn add_business_days(start: &NaiveDate, days: i64) -> NaiveDate {
    let mut count = 0;
    let mut current = *start;

    while count < days {
        current = current
            .succ_opt()
            .unwrap_or_else(|| panic!("Date overflow"));
        if is_working_day(&current) {
            count += 1;
        }
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_year_holiday() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date");
        assert!(is_russian_holiday(&date));

        let holiday = RussianLegalCalendar::get_holiday(&date).expect("Should be a holiday");
        assert_eq!(holiday.name_ru, "Новый год");
    }

    #[test]
    fn test_victory_day() {
        let date = NaiveDate::from_ymd_opt(2024, 5, 9).expect("Valid date");
        assert!(is_russian_holiday(&date));

        let holiday = RussianLegalCalendar::get_holiday(&date).expect("Should be a holiday");
        assert_eq!(holiday.name_en, "Victory Day");
    }

    #[test]
    fn test_working_day() {
        // Regular Tuesday
        let date = NaiveDate::from_ymd_opt(2024, 2, 6).expect("Valid date");
        assert!(is_working_day(&date));

        // Saturday
        let sat = NaiveDate::from_ymd_opt(2024, 2, 10).expect("Valid date");
        assert!(!is_working_day(&sat));

        // Sunday
        let sun = NaiveDate::from_ymd_opt(2024, 2, 11).expect("Valid date");
        assert!(!is_working_day(&sun));
    }

    #[test]
    fn test_business_days_calculation() {
        // Week with no holidays: Feb 5-9, 2024 (Mon-Fri)
        let start = NaiveDate::from_ymd_opt(2024, 2, 5).expect("Valid date");
        let end = NaiveDate::from_ymd_opt(2024, 2, 9).expect("Valid date");
        assert_eq!(calculate_business_days(&start, &end), 5);

        // Week including weekend: Feb 5-11, 2024 (Mon-Sun)
        let end2 = NaiveDate::from_ymd_opt(2024, 2, 11).expect("Valid date");
        assert_eq!(calculate_business_days(&start, &end2), 5);
    }

    #[test]
    fn test_add_business_days() {
        // Starting on Monday, add 5 business days should give next Monday
        let start = NaiveDate::from_ymd_opt(2024, 2, 5).expect("Valid date");
        let result = add_business_days(&start, 5);
        assert_eq!(
            result,
            NaiveDate::from_ymd_opt(2024, 2, 12).expect("Valid date")
        );
    }

    #[test]
    fn test_all_holidays_count() {
        let holidays = RussianLegalCalendar::federal_holidays();
        assert_eq!(holidays.len(), 14); // 8 New Year + 6 other holidays
    }
}
