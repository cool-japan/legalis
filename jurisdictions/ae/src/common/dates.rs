//! UAE Public Holidays and Working Day Calculations
//!
//! UAE observes both Islamic (Hijri) calendar-based holidays and fixed holidays.
//! The workweek in UAE is Monday-Friday since 2022 (previously Sunday-Thursday).
//! Friday is now the half-working day for government.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Type of UAE holiday
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UaeHolidayType {
    /// National/Secular holiday
    National,
    /// Islamic religious holiday (based on Hijri calendar)
    Islamic,
    /// Commemoration day
    Commemorative,
}

/// UAE public holiday
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UaeHoliday {
    /// Date of the holiday
    pub date: NaiveDate,
    /// Name in Arabic
    pub name_ar: String,
    /// Name in English
    pub name_en: String,
    /// Type of holiday
    pub holiday_type: UaeHolidayType,
    /// Number of days off
    pub days_off: u32,
}

/// Get public holidays for a given year
///
/// Note: Islamic holidays are based on lunar calendar and may vary.
/// These dates are approximate for planning purposes.
pub fn get_public_holidays(year: i32) -> Vec<UaeHoliday> {
    let mut holidays = vec![
        // Fixed holidays
        UaeHoliday {
            date: NaiveDate::from_ymd_opt(year, 1, 1).unwrap_or_default(),
            name_ar: "رأس السنة الميلادية".to_string(),
            name_en: "New Year's Day".to_string(),
            holiday_type: UaeHolidayType::National,
            days_off: 1,
        },
        UaeHoliday {
            date: NaiveDate::from_ymd_opt(year, 12, 2).unwrap_or_default(),
            name_ar: "اليوم الوطني".to_string(),
            name_en: "National Day".to_string(),
            holiday_type: UaeHolidayType::National,
            days_off: 2, // Dec 2-3
        },
        UaeHoliday {
            date: NaiveDate::from_ymd_opt(year, 11, 30).unwrap_or_default(),
            name_ar: "يوم الشهيد".to_string(),
            name_en: "Commemoration Day (Martyr's Day)".to_string(),
            holiday_type: UaeHolidayType::Commemorative,
            days_off: 1,
        },
    ];

    // Islamic holidays (approximate Gregorian dates - vary by year)
    // These dates are approximate and would need lunar calendar conversion
    if year == 2024 {
        holidays.extend(vec![
            // Eid Al Fitr 2024: April 10-12
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2024, 4, 10).unwrap_or_default(),
                name_ar: "عيد الفطر".to_string(),
                name_en: "Eid Al Fitr".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 4, // Typically 4 days
            },
            // Arafat Day 2024: June 15
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2024, 6, 15).unwrap_or_default(),
                name_ar: "يوم عرفة".to_string(),
                name_en: "Arafat Day".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 1,
            },
            // Eid Al Adha 2024: June 16-18
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2024, 6, 16).unwrap_or_default(),
                name_ar: "عيد الأضحى".to_string(),
                name_en: "Eid Al Adha".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 4,
            },
            // Islamic New Year 2024: July 7
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2024, 7, 7).unwrap_or_default(),
                name_ar: "رأس السنة الهجرية".to_string(),
                name_en: "Islamic New Year".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 1,
            },
            // Prophet's Birthday 2024: September 15
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2024, 9, 15).unwrap_or_default(),
                name_ar: "المولد النبوي".to_string(),
                name_en: "Prophet's Birthday (Mawlid)".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 1,
            },
        ]);
    } else if year == 2025 {
        holidays.extend(vec![
            // Eid Al Fitr 2025: March 30 - April 1
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2025, 3, 30).unwrap_or_default(),
                name_ar: "عيد الفطر".to_string(),
                name_en: "Eid Al Fitr".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 4,
            },
            // Arafat Day 2025: June 5
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2025, 6, 5).unwrap_or_default(),
                name_ar: "يوم عرفة".to_string(),
                name_en: "Arafat Day".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 1,
            },
            // Eid Al Adha 2025: June 6-8
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2025, 6, 6).unwrap_or_default(),
                name_ar: "عيد الأضحى".to_string(),
                name_en: "Eid Al Adha".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 4,
            },
            // Islamic New Year 2025: June 26
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2025, 6, 26).unwrap_or_default(),
                name_ar: "رأس السنة الهجرية".to_string(),
                name_en: "Islamic New Year".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 1,
            },
            // Prophet's Birthday 2025: September 4
            UaeHoliday {
                date: NaiveDate::from_ymd_opt(2025, 9, 4).unwrap_or_default(),
                name_ar: "المولد النبوي".to_string(),
                name_en: "Prophet's Birthday (Mawlid)".to_string(),
                holiday_type: UaeHolidayType::Islamic,
                days_off: 1,
            },
        ]);
    }

    holidays.sort_by_key(|h| h.date);
    holidays
}

/// Check if a date is a public holiday
pub fn is_public_holiday(date: NaiveDate) -> bool {
    let holidays = get_public_holidays(date.year());

    for holiday in &holidays {
        if holiday.days_off == 1 && holiday.date == date {
            return true;
        } else if holiday.days_off > 1 {
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

/// Check if a date is a working day
///
/// UAE workweek changed to Monday-Friday effective January 2022.
/// Saturday and Sunday are weekends (for federal government and most private sector).
pub fn is_working_day(date: NaiveDate) -> bool {
    let weekday = date.weekday();

    // Weekend is Saturday and Sunday (since 2022)
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

        // Check National Day exists
        let national_day = holidays.iter().find(|h| h.name_en == "National Day");
        assert!(national_day.is_some());
    }

    #[test]
    fn test_is_public_holiday() {
        // National Day 2024
        let national_day = NaiveDate::from_ymd_opt(2024, 12, 2).unwrap();
        assert!(is_public_holiday(national_day));

        // Regular day
        let regular = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert!(!is_public_holiday(regular));
    }

    #[test]
    fn test_is_working_day() {
        // Regular Monday
        let monday = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
        assert!(is_working_day(monday));

        // Saturday (weekend since 2022)
        let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
        assert!(!is_working_day(saturday));

        // Sunday (weekend)
        let sunday = NaiveDate::from_ymd_opt(2024, 3, 17).unwrap();
        assert!(!is_working_day(sunday));

        // Friday (working day since 2022)
        let friday = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert!(is_working_day(friday));
    }

    #[test]
    fn test_working_days_between() {
        // One week (Mon-Fri = 5 working days)
        let start = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2024, 3, 25).unwrap(); // Next Monday
        assert_eq!(working_days_between(start, end), 5);
    }

    #[test]
    fn test_eid_multi_day() {
        // Eid Al Fitr 2024 starts April 10, spans 4 days
        let eid_day1 = NaiveDate::from_ymd_opt(2024, 4, 10).unwrap();
        let eid_day3 = NaiveDate::from_ymd_opt(2024, 4, 12).unwrap();

        assert!(is_public_holiday(eid_day1));
        assert!(is_public_holiday(eid_day3));
    }
}
