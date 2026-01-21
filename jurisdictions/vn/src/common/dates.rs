//! Vietnamese Public Holidays and Working Day Calculations
//!
//! Vietnam has both fixed (Gregorian) and lunar calendar holidays.
//! Tết (Lunar New Year) is the most important holiday.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Type of Vietnamese holiday
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VietnameseHolidayType {
    /// National holiday (Ngày lễ quốc gia)
    National,
    /// Lunar New Year (Tết Nguyên Đán)
    Tet,
    /// Traditional/Cultural holiday
    Traditional,
    /// Commemorative day
    Commemorative,
}

/// Vietnamese public holiday
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VietnameseHoliday {
    /// Date of the holiday
    pub date: NaiveDate,
    /// Name in Vietnamese
    pub name_vi: String,
    /// Name in English
    pub name_en: String,
    /// Type of holiday
    pub holiday_type: VietnameseHolidayType,
    /// Number of days off
    pub days_off: u32,
}

/// Get public holidays for a given year
/// Note: Lunar calendar holidays (Tết) dates are approximate
pub fn get_public_holidays(year: i32) -> Vec<VietnameseHoliday> {
    let mut holidays = vec![
        // Fixed holidays (Gregorian calendar)
        VietnameseHoliday {
            date: NaiveDate::from_ymd_opt(year, 1, 1).unwrap_or_default(),
            name_vi: "Tết Dương lịch".to_string(),
            name_en: "New Year's Day".to_string(),
            holiday_type: VietnameseHolidayType::National,
            days_off: 1,
        },
        VietnameseHoliday {
            date: NaiveDate::from_ymd_opt(year, 4, 30).unwrap_or_default(),
            name_vi: "Ngày Giải phóng miền Nam".to_string(),
            name_en: "Reunification Day".to_string(),
            holiday_type: VietnameseHolidayType::National,
            days_off: 1,
        },
        VietnameseHoliday {
            date: NaiveDate::from_ymd_opt(year, 5, 1).unwrap_or_default(),
            name_vi: "Ngày Quốc tế Lao động".to_string(),
            name_en: "International Labor Day".to_string(),
            holiday_type: VietnameseHolidayType::National,
            days_off: 1,
        },
        VietnameseHoliday {
            date: NaiveDate::from_ymd_opt(year, 9, 2).unwrap_or_default(),
            name_vi: "Ngày Quốc khánh".to_string(),
            name_en: "National Day".to_string(),
            holiday_type: VietnameseHolidayType::National,
            days_off: 2, // 2 days: Sep 2 and Sep 3 (or Sep 1 makeup)
        },
    ];

    // Add Tết (Lunar New Year) - dates vary by year
    // These are approximate Gregorian dates for Tết
    if year == 2024 {
        // Tết 2024: Year of the Dragon - starts Feb 10, 2024
        holidays.push(VietnameseHoliday {
            date: NaiveDate::from_ymd_opt(2024, 2, 10).unwrap_or_default(),
            name_vi: "Tết Nguyên Đán (Tết Giáp Thìn)".to_string(),
            name_en: "Lunar New Year (Year of the Dragon)".to_string(),
            holiday_type: VietnameseHolidayType::Tet,
            days_off: 5, // 5 consecutive days off
        });

        // Hung Kings Commemoration Day 2024: April 18
        holidays.push(VietnameseHoliday {
            date: NaiveDate::from_ymd_opt(2024, 4, 18).unwrap_or_default(),
            name_vi: "Giỗ Tổ Hùng Vương".to_string(),
            name_en: "Hung Kings Commemoration Day".to_string(),
            holiday_type: VietnameseHolidayType::Traditional,
            days_off: 1,
        });
    } else if year == 2025 {
        // Tết 2025: Year of the Snake - starts Jan 29, 2025
        holidays.push(VietnameseHoliday {
            date: NaiveDate::from_ymd_opt(2025, 1, 29).unwrap_or_default(),
            name_vi: "Tết Nguyên Đán (Tết Ất Tỵ)".to_string(),
            name_en: "Lunar New Year (Year of the Snake)".to_string(),
            holiday_type: VietnameseHolidayType::Tet,
            days_off: 5,
        });

        // Hung Kings Commemoration Day 2025: April 7
        holidays.push(VietnameseHoliday {
            date: NaiveDate::from_ymd_opt(2025, 4, 7).unwrap_or_default(),
            name_vi: "Giỗ Tổ Hùng Vương".to_string(),
            name_en: "Hung Kings Commemoration Day".to_string(),
            holiday_type: VietnameseHolidayType::Traditional,
            days_off: 1,
        });
    }

    holidays.sort_by_key(|h| h.date);
    holidays
}

/// Check if a date is a public holiday
pub fn is_public_holiday(date: NaiveDate) -> bool {
    let holidays = get_public_holidays(date.year());

    for holiday in &holidays {
        // Check if date falls within the holiday period
        if holiday.days_off == 1 && holiday.date == date {
            return true;
        } else if holiday.days_off > 1 {
            // Multi-day holidays (like Tết)
            for i in 0..holiday.days_off {
                if let Some(holiday_date) =
                    holiday.date.checked_add_signed(Duration::days(i as i64))
                    && holiday_date == date
                {
                    return true;
                }
            }
        }
    }

    false
}

/// Check if a date is a working day (Monday-Saturday, not a holiday)
/// Note: Vietnam traditionally has Saturday as a half-working day
pub fn is_working_day(date: NaiveDate) -> bool {
    let weekday = date.weekday();
    // Sunday is the only weekly rest day by law
    if weekday == Weekday::Sun {
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

        // Check National Day
        let national_day = holidays.iter().find(|h| h.name_en == "National Day");
        assert!(national_day.is_some());
        let nd = national_day.expect("should exist");
        assert_eq!(nd.date, NaiveDate::from_ymd_opt(2024, 9, 2).unwrap());
    }

    #[test]
    fn test_is_public_holiday() {
        // Labor Day 2024
        let labor_day = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
        assert!(is_public_holiday(labor_day));

        // Regular day
        let regular = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert!(!is_public_holiday(regular));
    }

    #[test]
    fn test_tet_multi_day() {
        // Tết 2024 starts Feb 10, spans 5 days
        let tet_day1 = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
        let tet_day3 = NaiveDate::from_ymd_opt(2024, 2, 12).unwrap();

        assert!(is_public_holiday(tet_day1));
        assert!(is_public_holiday(tet_day3));
    }

    #[test]
    fn test_is_working_day() {
        // Regular Monday
        let monday = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
        assert!(is_working_day(monday));

        // Sunday (weekly rest)
        let sunday = NaiveDate::from_ymd_opt(2024, 3, 17).unwrap();
        assert!(!is_working_day(sunday));

        // Saturday (working day in Vietnam)
        let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
        assert!(is_working_day(saturday));
    }

    #[test]
    fn test_working_days_between() {
        // One week (Mon-Sat = 6 working days in Vietnam)
        let start = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2024, 3, 25).unwrap(); // Next Monday
        assert_eq!(working_days_between(start, end), 6);
    }
}
