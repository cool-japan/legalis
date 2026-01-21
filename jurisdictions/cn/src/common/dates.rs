//! Chinese Date Utilities
//!
//! Handles Chinese legal dates, holidays, and working day calculations.
//!
//! # 日期工具 / Date Utilities

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Chinese public holidays
///
/// # 法定节假日
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicHoliday {
    /// 元旦 / New Year's Day (January 1)
    NewYearsDay,
    /// 春节 / Spring Festival (Chinese New Year, 3 days)
    SpringFestival,
    /// 清明节 / Qingming Festival (Tomb Sweeping Day)
    QingmingFestival,
    /// 劳动节 / Labor Day (May 1)
    LaborDay,
    /// 端午节 / Dragon Boat Festival
    DragonBoatFestival,
    /// 中秋节 / Mid-Autumn Festival
    MidAutumnFestival,
    /// 国庆节 / National Day (October 1-7)
    NationalDay,
}

impl PublicHoliday {
    /// Get holiday name in Chinese
    pub fn name_zh(&self) -> &'static str {
        match self {
            Self::NewYearsDay => "元旦",
            Self::SpringFestival => "春节",
            Self::QingmingFestival => "清明节",
            Self::LaborDay => "劳动节",
            Self::DragonBoatFestival => "端午节",
            Self::MidAutumnFestival => "中秋节",
            Self::NationalDay => "国庆节",
        }
    }

    /// Get holiday name in English
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::NewYearsDay => "New Year's Day",
            Self::SpringFestival => "Spring Festival",
            Self::QingmingFestival => "Qingming Festival",
            Self::LaborDay => "Labor Day",
            Self::DragonBoatFestival => "Dragon Boat Festival",
            Self::MidAutumnFestival => "Mid-Autumn Festival",
            Self::NationalDay => "National Day",
        }
    }

    /// Get statutory leave days
    pub fn statutory_days(&self) -> u32 {
        match self {
            Self::NewYearsDay => 1,
            Self::SpringFestival => 3,
            Self::QingmingFestival => 1,
            Self::LaborDay => 1,
            Self::DragonBoatFestival => 1,
            Self::MidAutumnFestival => 1,
            Self::NationalDay => 3,
        }
    }

    /// Get typical extended holiday days (including weekends, makeup days)
    pub fn typical_extended_days(&self) -> u32 {
        match self {
            Self::NewYearsDay => 3,
            Self::SpringFestival => 7,
            Self::QingmingFestival => 3,
            Self::LaborDay => 5,
            Self::DragonBoatFestival => 3,
            Self::MidAutumnFestival => 3,
            Self::NationalDay => 7,
        }
    }
}

/// Check if a date is a weekend (Saturday or Sunday)
pub fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

/// Check if date is New Year's Day
pub fn is_new_years_day(date: NaiveDate) -> bool {
    date.month() == 1 && date.day() == 1
}

/// Check if date is Labor Day
pub fn is_labor_day(date: NaiveDate) -> bool {
    date.month() == 5 && date.day() == 1
}

/// Check if date is National Day (October 1-3 statutory)
pub fn is_national_day_statutory(date: NaiveDate) -> bool {
    date.month() == 10 && date.day() >= 1 && date.day() <= 3
}

/// Calculate working days between two dates (excluding weekends)
///
/// Note: This is a simplified calculation. Actual working days
/// depend on annual holiday schedules published by State Council.
pub fn working_days_between(start: NaiveDate, end: NaiveDate) -> i64 {
    if end < start {
        return 0;
    }

    let mut count = 0i64;
    let mut current = start;

    while current <= end {
        if !is_weekend(current) {
            count += 1;
        }
        current = current.succ_opt().unwrap_or(current);
    }

    count
}

/// Add working days to a date
pub fn add_working_days(start: NaiveDate, days: i64) -> NaiveDate {
    let mut current = start;
    let mut remaining = days;

    while remaining > 0 {
        current = current.succ_opt().unwrap_or(current);
        if !is_weekend(current) {
            remaining -= 1;
        }
    }

    current
}

/// Chinese legal deadline types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeadlineType {
    /// 日 / Days (calendar days)
    Days,
    /// 工作日 / Working Days
    WorkingDays,
    /// 月 / Months
    Months,
    /// 年 / Years
    Years,
}

/// Calculate deadline date
pub fn calculate_deadline(start: NaiveDate, count: i64, deadline_type: DeadlineType) -> NaiveDate {
    match deadline_type {
        DeadlineType::Days => start + chrono::Duration::days(count),
        DeadlineType::WorkingDays => add_working_days(start, count),
        DeadlineType::Months => {
            let new_month = (start.month() as i64 + count - 1).rem_euclid(12) + 1;
            let year_offset = (start.month() as i64 + count - 1).div_euclid(12);
            let new_year = start.year() + year_offset as i32;
            NaiveDate::from_ymd_opt(new_year, new_month as u32, start.day()).unwrap_or_else(|| {
                // Handle month-end edge cases (e.g., Jan 31 + 1 month)
                let last_day = last_day_of_month(new_year, new_month as u32);
                NaiveDate::from_ymd_opt(new_year, new_month as u32, last_day).expect("Valid date")
            })
        }
        DeadlineType::Years => {
            NaiveDate::from_ymd_opt(start.year() + count as i32, start.month(), start.day())
                .unwrap_or_else(|| {
                    // Handle Feb 29 in non-leap years
                    NaiveDate::from_ymd_opt(start.year() + count as i32, start.month(), 28)
                        .expect("Valid date")
                })
        }
    }
}

/// Get last day of month
fn last_day_of_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Check if year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_holiday_names() {
        assert_eq!(PublicHoliday::SpringFestival.name_zh(), "春节");
        assert_eq!(PublicHoliday::SpringFestival.name_en(), "Spring Festival");
    }

    #[test]
    fn test_statutory_days() {
        assert_eq!(PublicHoliday::SpringFestival.statutory_days(), 3);
        assert_eq!(PublicHoliday::NationalDay.statutory_days(), 3);
        assert_eq!(PublicHoliday::LaborDay.statutory_days(), 1);
    }

    #[test]
    fn test_is_weekend() {
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap();
        let sunday = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();

        assert!(is_weekend(saturday));
        assert!(is_weekend(sunday));
        assert!(!is_weekend(monday));
    }

    #[test]
    fn test_working_days_between() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2024, 1, 12).unwrap(); // Friday
        assert_eq!(working_days_between(start, end), 5);
    }

    #[test]
    fn test_add_working_days() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(); // Monday
        let result = add_working_days(start, 5);
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()); // Next Monday
    }

    #[test]
    fn test_calculate_deadline_days() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = calculate_deadline(start, 30, DeadlineType::Days);
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 1, 31).unwrap());
    }

    #[test]
    fn test_calculate_deadline_months() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let result = calculate_deadline(start, 3, DeadlineType::Months);
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 4, 15).unwrap());
    }
}
