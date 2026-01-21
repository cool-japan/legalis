//! Indonesian Public Holidays and Working Day Calculations
//!
//! Indonesia has a mix of national, religious, and government holidays.
//! Religious holidays follow lunar calendars (Islamic Hijri, Hindu Saka, etc.)

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Type of Indonesian holiday
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndonesianHolidayType {
    /// National holiday (Hari Libur Nasional)
    National,
    /// Islamic holiday (Hari Raya Islam)
    Islamic,
    /// Hindu holiday (Hari Raya Hindu)
    Hindu,
    /// Buddhist holiday (Hari Raya Buddha)
    Buddhist,
    /// Christian holiday (Hari Raya Kristen)
    Christian,
    /// Chinese/Confucian holiday
    Chinese,
    /// Government leave day (Cuti Bersama)
    CutiBersama,
}

/// Indonesian public holiday
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndonesianHoliday {
    /// Date of the holiday
    pub date: NaiveDate,
    /// Name in Indonesian
    pub name_id: String,
    /// Name in English
    pub name_en: String,
    /// Type of holiday
    pub holiday_type: IndonesianHolidayType,
    /// Whether it's a national day off
    pub is_national_day_off: bool,
}

/// Get national holidays for a given year
/// Note: Islamic and lunar holidays are approximate as they depend on moon sighting
pub fn get_national_holidays(year: i32) -> Vec<IndonesianHoliday> {
    let mut holidays = vec![
        // Fixed holidays
        IndonesianHoliday {
            date: NaiveDate::from_ymd_opt(year, 1, 1).unwrap_or_default(),
            name_id: "Tahun Baru Masehi".to_string(),
            name_en: "New Year's Day".to_string(),
            holiday_type: IndonesianHolidayType::National,
            is_national_day_off: true,
        },
        IndonesianHoliday {
            date: NaiveDate::from_ymd_opt(year, 5, 1).unwrap_or_default(),
            name_id: "Hari Buruh Internasional".to_string(),
            name_en: "International Labor Day".to_string(),
            holiday_type: IndonesianHolidayType::National,
            is_national_day_off: true,
        },
        IndonesianHoliday {
            date: NaiveDate::from_ymd_opt(year, 6, 1).unwrap_or_default(),
            name_id: "Hari Lahir Pancasila".to_string(),
            name_en: "Pancasila Day".to_string(),
            holiday_type: IndonesianHolidayType::National,
            is_national_day_off: true,
        },
        IndonesianHoliday {
            date: NaiveDate::from_ymd_opt(year, 8, 17).unwrap_or_default(),
            name_id: "Hari Kemerdekaan Republik Indonesia".to_string(),
            name_en: "Independence Day".to_string(),
            holiday_type: IndonesianHolidayType::National,
            is_national_day_off: true,
        },
        IndonesianHoliday {
            date: NaiveDate::from_ymd_opt(year, 12, 25).unwrap_or_default(),
            name_id: "Hari Raya Natal".to_string(),
            name_en: "Christmas Day".to_string(),
            holiday_type: IndonesianHolidayType::Christian,
            is_national_day_off: true,
        },
    ];

    // Add approximate religious holidays (2024 dates - adjust for other years)
    // In practice, exact dates are announced by the government annually
    if year == 2024 {
        holidays.extend(vec![
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 2, 8).unwrap_or_default(),
                name_id: "Isra Mi'raj Nabi Muhammad SAW".to_string(),
                name_en: "Isra Mi'raj".to_string(),
                holiday_type: IndonesianHolidayType::Islamic,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 2, 10).unwrap_or_default(),
                name_id: "Tahun Baru Imlek".to_string(),
                name_en: "Chinese New Year".to_string(),
                holiday_type: IndonesianHolidayType::Chinese,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 3, 11).unwrap_or_default(),
                name_id: "Hari Raya Nyepi".to_string(),
                name_en: "Nyepi (Balinese New Year)".to_string(),
                holiday_type: IndonesianHolidayType::Hindu,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 3, 29).unwrap_or_default(),
                name_id: "Wafat Isa Al-Masih".to_string(),
                name_en: "Good Friday".to_string(),
                holiday_type: IndonesianHolidayType::Christian,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 4, 10).unwrap_or_default(),
                name_id: "Hari Raya Idul Fitri 1445 H".to_string(),
                name_en: "Eid al-Fitr".to_string(),
                holiday_type: IndonesianHolidayType::Islamic,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 4, 11).unwrap_or_default(),
                name_id: "Hari Raya Idul Fitri 1445 H (Hari Kedua)".to_string(),
                name_en: "Eid al-Fitr (Second Day)".to_string(),
                holiday_type: IndonesianHolidayType::Islamic,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 5, 9).unwrap_or_default(),
                name_id: "Kenaikan Isa Al-Masih".to_string(),
                name_en: "Ascension Day".to_string(),
                holiday_type: IndonesianHolidayType::Christian,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 5, 23).unwrap_or_default(),
                name_id: "Hari Raya Waisak".to_string(),
                name_en: "Vesak Day".to_string(),
                holiday_type: IndonesianHolidayType::Buddhist,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 6, 17).unwrap_or_default(),
                name_id: "Hari Raya Idul Adha 1445 H".to_string(),
                name_en: "Eid al-Adha".to_string(),
                holiday_type: IndonesianHolidayType::Islamic,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 7, 7).unwrap_or_default(),
                name_id: "Tahun Baru Islam 1446 H".to_string(),
                name_en: "Islamic New Year".to_string(),
                holiday_type: IndonesianHolidayType::Islamic,
                is_national_day_off: true,
            },
            IndonesianHoliday {
                date: NaiveDate::from_ymd_opt(2024, 9, 16).unwrap_or_default(),
                name_id: "Maulid Nabi Muhammad SAW".to_string(),
                name_en: "Prophet Muhammad's Birthday".to_string(),
                holiday_type: IndonesianHolidayType::Islamic,
                is_national_day_off: true,
            },
        ]);
    }

    holidays.sort_by_key(|h| h.date);
    holidays
}

/// Check if a date is a national holiday
pub fn is_national_holiday(date: NaiveDate) -> bool {
    let holidays = get_national_holidays(date.year());
    holidays
        .iter()
        .any(|h| h.date == date && h.is_national_day_off)
}

/// Check if a date is a working day (Monday-Friday, not a holiday)
pub fn is_working_day(date: NaiveDate) -> bool {
    let weekday = date.weekday();
    if weekday == Weekday::Sat || weekday == Weekday::Sun {
        return false;
    }
    !is_national_holiday(date)
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
    fn test_national_holidays_2024() {
        let holidays = get_national_holidays(2024);
        assert!(!holidays.is_empty());

        // Check Independence Day
        let independence = holidays.iter().find(|h| h.name_en == "Independence Day");
        assert!(independence.is_some());
        let ind = independence.expect("should exist");
        assert_eq!(ind.date, NaiveDate::from_ymd_opt(2024, 8, 17).unwrap());
    }

    #[test]
    fn test_is_national_holiday() {
        let independence_day = NaiveDate::from_ymd_opt(2024, 8, 17).unwrap();
        assert!(is_national_holiday(independence_day));

        let regular_day = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert!(!is_national_holiday(regular_day));
    }

    #[test]
    fn test_is_working_day() {
        // Regular Monday
        let monday = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
        assert!(is_working_day(monday));

        // Saturday
        let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
        assert!(!is_working_day(saturday));

        // Independence Day (Saturday in 2024, but holiday check matters)
        let independence = NaiveDate::from_ymd_opt(2024, 8, 17).unwrap();
        assert!(!is_working_day(independence));
    }

    #[test]
    fn test_working_days_between() {
        // One week (Mon-Fri = 5 working days)
        let start = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2024, 3, 25).unwrap(); // Next Monday
        assert_eq!(working_days_between(start, end), 5);
    }
}
