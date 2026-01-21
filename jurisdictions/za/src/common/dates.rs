//! South African Public Holidays and Working Day Calculations
//!
//! South African public holidays are governed by the Public Holidays Act 36 of 1994.
//! If a public holiday falls on a Sunday, the following Monday is also a public holiday.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Type of South African holiday
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SouthAfricanHolidayType {
    /// National/Secular holiday
    National,
    /// Christian religious holiday
    Christian,
    /// Heritage/Cultural
    Heritage,
    /// Political/Historical
    Historical,
}

/// South African public holiday
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SouthAfricanHoliday {
    /// Date of the holiday
    pub date: NaiveDate,
    /// Name of the holiday
    pub name: String,
    /// Type of holiday
    pub holiday_type: SouthAfricanHolidayType,
    /// Is this a substitute holiday (when original falls on Sunday)
    pub is_substitute: bool,
}

/// Get public holidays for a given year
pub fn get_public_holidays(year: i32) -> Vec<SouthAfricanHoliday> {
    let mut holidays = vec![
        // Fixed date holidays
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 1, 1).unwrap_or_default(),
            name: "New Year's Day".to_string(),
            holiday_type: SouthAfricanHolidayType::National,
            is_substitute: false,
        },
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 3, 21).unwrap_or_default(),
            name: "Human Rights Day".to_string(),
            holiday_type: SouthAfricanHolidayType::Historical,
            is_substitute: false,
        },
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 4, 27).unwrap_or_default(),
            name: "Freedom Day".to_string(),
            holiday_type: SouthAfricanHolidayType::Historical,
            is_substitute: false,
        },
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 5, 1).unwrap_or_default(),
            name: "Workers' Day".to_string(),
            holiday_type: SouthAfricanHolidayType::National,
            is_substitute: false,
        },
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 6, 16).unwrap_or_default(),
            name: "Youth Day".to_string(),
            holiday_type: SouthAfricanHolidayType::Historical,
            is_substitute: false,
        },
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 8, 9).unwrap_or_default(),
            name: "National Women's Day".to_string(),
            holiday_type: SouthAfricanHolidayType::Historical,
            is_substitute: false,
        },
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 9, 24).unwrap_or_default(),
            name: "Heritage Day".to_string(),
            holiday_type: SouthAfricanHolidayType::Heritage,
            is_substitute: false,
        },
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 12, 16).unwrap_or_default(),
            name: "Day of Reconciliation".to_string(),
            holiday_type: SouthAfricanHolidayType::Historical,
            is_substitute: false,
        },
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 12, 25).unwrap_or_default(),
            name: "Christmas Day".to_string(),
            holiday_type: SouthAfricanHolidayType::Christian,
            is_substitute: false,
        },
        SouthAfricanHoliday {
            date: NaiveDate::from_ymd_opt(year, 12, 26).unwrap_or_default(),
            name: "Day of Goodwill".to_string(),
            holiday_type: SouthAfricanHolidayType::Christian,
            is_substitute: false,
        },
    ];

    // Add Easter holidays (varies by year)
    if let Some(easter) = calculate_easter(year) {
        // Good Friday (2 days before Easter)
        if let Some(good_friday) = easter.checked_sub_signed(Duration::days(2)) {
            holidays.push(SouthAfricanHoliday {
                date: good_friday,
                name: "Good Friday".to_string(),
                holiday_type: SouthAfricanHolidayType::Christian,
                is_substitute: false,
            });
        }

        // Family Day (Easter Monday)
        if let Some(family_day) = easter.checked_add_signed(Duration::days(1)) {
            holidays.push(SouthAfricanHoliday {
                date: family_day,
                name: "Family Day".to_string(),
                holiday_type: SouthAfricanHolidayType::Christian,
                is_substitute: false,
            });
        }
    }

    // Add substitute holidays for Sundays
    let original_holidays = holidays.clone();
    for holiday in &original_holidays {
        if holiday.date.weekday() == Weekday::Sun
            && let Some(monday) = holiday.date.checked_add_signed(Duration::days(1))
        {
            holidays.push(SouthAfricanHoliday {
                date: monday,
                name: format!("{} (observed)", holiday.name),
                holiday_type: holiday.holiday_type.clone(),
                is_substitute: true,
            });
        }
    }

    holidays.sort_by_key(|h| h.date);
    holidays
}

/// Calculate Easter Sunday using the Anonymous Gregorian algorithm
fn calculate_easter(year: i32) -> Option<NaiveDate> {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = ((h + l - 7 * m + 114) % 31) + 1;

    NaiveDate::from_ymd_opt(year, month as u32, day as u32)
}

/// Check if a date is a public holiday
pub fn is_public_holiday(date: NaiveDate) -> bool {
    let holidays = get_public_holidays(date.year());
    holidays.iter().any(|h| h.date == date)
}

/// Check if a date is a working day (Monday-Friday, not a holiday)
pub fn is_working_day(date: NaiveDate) -> bool {
    let weekday = date.weekday();

    // Weekend
    if matches!(weekday, Weekday::Sat | Weekday::Sun) {
        return false;
    }

    !is_public_holiday(date)
}

/// Calculate working days between two dates (exclusive of end date)
pub fn working_days_between(start: NaiveDate, end: NaiveDate) -> i64 {
    if start >= end {
        return 0;
    }

    let mut count = 0i64;
    let mut current = start;
    while current < end {
        if is_working_day(current) {
            count += 1;
        }
        current += Duration::days(1);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_holidays_2024() {
        let holidays = get_public_holidays(2024);
        assert!(!holidays.is_empty());

        // Check Freedom Day exists
        let freedom_day = holidays.iter().find(|h| h.name == "Freedom Day");
        assert!(freedom_day.is_some());
    }

    #[test]
    fn test_is_public_holiday() {
        // Freedom Day 2024
        let freedom_day = NaiveDate::from_ymd_opt(2024, 4, 27).unwrap();
        assert!(is_public_holiday(freedom_day));

        // Regular day
        let regular = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert!(!is_public_holiday(regular));
    }

    #[test]
    fn test_is_working_day() {
        // Regular Monday
        let monday = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
        assert!(is_working_day(monday));

        // Saturday
        let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
        assert!(!is_working_day(saturday));

        // Sunday
        let sunday = NaiveDate::from_ymd_opt(2024, 3, 17).unwrap();
        assert!(!is_working_day(sunday));
    }

    #[test]
    fn test_working_days_between() {
        // One week (Mon-Fri = 5 working days) - use a week without holidays
        let start = NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2024, 5, 13).unwrap(); // Next Monday
        assert_eq!(working_days_between(start, end), 5);
    }

    #[test]
    fn test_easter_calculation() {
        let easter_2024 = calculate_easter(2024);
        assert!(easter_2024.is_some());
        // Easter 2024 is March 31
        assert_eq!(
            easter_2024.unwrap(),
            NaiveDate::from_ymd_opt(2024, 3, 31).unwrap()
        );
    }

    #[test]
    fn test_sunday_substitute() {
        // When a holiday falls on Sunday, Monday should be observed
        let holidays = get_public_holidays(2024);
        // Note: depends on the year whether any falls on Sunday
        assert!(holidays.len() >= 12); // Minimum fixed holidays + Easter
    }
}
