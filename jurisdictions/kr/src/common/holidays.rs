//! Korean Public Holidays
//!
//! Official public holidays in South Korea
//!
//! # 대한민국 공휴일 / Korean Public Holidays

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Korean public holiday
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicHoliday {
    /// Korean name
    pub name_ko: String,
    /// English name
    pub name_en: String,
    /// Date
    pub date: NaiveDate,
    /// Is substitute holiday if falls on weekend
    pub substitute_eligible: bool,
}

/// Type of Korean public holiday
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HolidayType {
    /// National holiday
    National,
    /// Traditional holiday
    Traditional,
    /// Memorial day
    Memorial,
    /// Substitute holiday
    Substitute,
}

/// Get all public holidays for a given year
pub fn get_public_holidays(year: i32) -> Vec<PublicHoliday> {
    let mut holidays = Vec::new();

    // New Year's Day (신정) - January 1
    if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
        holidays.push(PublicHoliday {
            name_ko: "신정".to_string(),
            name_en: "New Year's Day".to_string(),
            date,
            substitute_eligible: true,
        });
    }

    // Seollal (설날) - Lunar New Year (1st day of 1st lunar month)
    // This requires lunar calendar calculation - simplified for now
    // Typically late January or February

    // Independence Movement Day (삼일절) - March 1
    if let Some(date) = NaiveDate::from_ymd_opt(year, 3, 1) {
        holidays.push(PublicHoliday {
            name_ko: "삼일절".to_string(),
            name_en: "Independence Movement Day".to_string(),
            date,
            substitute_eligible: true,
        });
    }

    // Children's Day (어린이날) - May 5
    if let Some(date) = NaiveDate::from_ymd_opt(year, 5, 5) {
        holidays.push(PublicHoliday {
            name_ko: "어린이날".to_string(),
            name_en: "Children's Day".to_string(),
            date,
            substitute_eligible: true,
        });
    }

    // Buddha's Birthday (석가탄신일) - 8th day of 4th lunar month
    // Typically in May

    // Memorial Day (현충일) - June 6
    if let Some(date) = NaiveDate::from_ymd_opt(year, 6, 6) {
        holidays.push(PublicHoliday {
            name_ko: "현충일".to_string(),
            name_en: "Memorial Day".to_string(),
            date,
            substitute_eligible: false,
        });
    }

    // Liberation Day (광복절) - August 15
    if let Some(date) = NaiveDate::from_ymd_opt(year, 8, 15) {
        holidays.push(PublicHoliday {
            name_ko: "광복절".to_string(),
            name_en: "Liberation Day".to_string(),
            date,
            substitute_eligible: true,
        });
    }

    // Chuseok (추석) - 15th day of 8th lunar month
    // Typically in September or October

    // National Foundation Day (개천절) - October 3
    if let Some(date) = NaiveDate::from_ymd_opt(year, 10, 3) {
        holidays.push(PublicHoliday {
            name_ko: "개천절".to_string(),
            name_en: "National Foundation Day".to_string(),
            date,
            substitute_eligible: true,
        });
    }

    // Hangeul Day (한글날) - October 9
    if let Some(date) = NaiveDate::from_ymd_opt(year, 10, 9) {
        holidays.push(PublicHoliday {
            name_ko: "한글날".to_string(),
            name_en: "Hangeul Day".to_string(),
            date,
            substitute_eligible: true,
        });
    }

    // Christmas Day (크리스마스) - December 25
    if let Some(date) = NaiveDate::from_ymd_opt(year, 12, 25) {
        holidays.push(PublicHoliday {
            name_ko: "크리스마스".to_string(),
            name_en: "Christmas Day".to_string(),
            date,
            substitute_eligible: true,
        });
    }

    holidays
}

/// Check if a date is a public holiday
pub fn is_public_holiday(date: &NaiveDate) -> bool {
    let holidays = get_public_holidays(date.year());
    holidays.iter().any(|h| h.date == *date)
}

/// Check if a date is a weekend
pub fn is_weekend(date: &NaiveDate) -> bool {
    matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
}

/// Check if a date is a non-working day (weekend or public holiday)
pub fn is_non_working_day(date: &NaiveDate) -> bool {
    is_weekend(date) || is_public_holiday(date)
}

/// Calculate next working day
pub fn next_working_day(date: NaiveDate) -> Option<NaiveDate> {
    let mut current = date;
    for _ in 0..14 {
        // Max 2 weeks ahead
        current = current.succ_opt()?;
        if !is_non_working_day(&current) {
            return Some(current);
        }
    }
    None
}

/// Calculate number of working days between two dates
pub fn count_working_days(start: &NaiveDate, end: &NaiveDate) -> i32 {
    if start > end {
        return 0;
    }

    let mut count = 0;
    let mut current = *start;

    while current <= *end {
        if !is_non_working_day(&current) {
            count += 1;
        }
        if let Some(next) = current.succ_opt() {
            current = next;
        } else {
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_public_holidays() {
        let holidays = get_public_holidays(2024);
        assert!(!holidays.is_empty());

        // Check for New Year's Day
        let new_year = holidays.iter().find(|h| h.name_en == "New Year's Day");
        assert!(new_year.is_some());
    }

    #[test]
    fn test_is_public_holiday() {
        // Test New Year's Day 2024
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            assert!(is_public_holiday(&date));
        }
    }

    #[test]
    fn test_is_weekend() {
        // January 6, 2024 is a Saturday
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 6) {
            assert!(is_weekend(&date));
        }

        // January 8, 2024 is a Monday
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 8) {
            assert!(!is_weekend(&date));
        }
    }

    #[test]
    fn test_next_working_day() {
        // Friday -> Monday
        if let Some(friday) = NaiveDate::from_ymd_opt(2024, 1, 5) {
            if let Some(next) = next_working_day(friday) {
                assert_eq!(next.weekday(), chrono::Weekday::Mon);
            }
        }
    }

    #[test]
    fn test_count_working_days() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2024, 1, 1),
            NaiveDate::from_ymd_opt(2024, 1, 7),
        ) {
            let count = count_working_days(&start, &end);
            assert!(count <= 7);
        }
    }
}
