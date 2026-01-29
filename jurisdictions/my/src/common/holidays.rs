//! Malaysian legal calendar and public holidays.
//!
//! Provides utilities for working with Malaysian public holidays and legal deadlines.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Malaysian public holiday.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MalaysianPublicHoliday {
    /// Date of the holiday.
    pub date: NaiveDate,
    /// Name of the holiday (English).
    pub name_en: String,
    /// Name of the holiday (Malay).
    pub name_ms: String,
    /// Whether it's a federal holiday (true) or state-specific (false).
    pub federal: bool,
}

/// Malaysian legal calendar utilities.
#[derive(Debug, Clone)]
pub struct MalaysianLegalCalendar {
    _year: i32,
    holidays: Vec<MalaysianPublicHoliday>,
}

impl MalaysianLegalCalendar {
    /// Creates a new calendar for the specified year.
    #[must_use]
    pub fn new(year: i32) -> Self {
        let holidays = Self::generate_holidays(year);
        Self {
            _year: year,
            holidays,
        }
    }

    /// Generates federal public holidays for Malaysia.
    fn generate_holidays(year: i32) -> Vec<MalaysianPublicHoliday> {
        vec![
            // Fixed holidays
            MalaysianPublicHoliday {
                date: NaiveDate::from_ymd_opt(year, 1, 1).expect("Valid date"),
                name_en: "New Year's Day".to_string(),
                name_ms: "Tahun Baru".to_string(),
                federal: true,
            },
            MalaysianPublicHoliday {
                date: NaiveDate::from_ymd_opt(year, 2, 1).expect("Valid date"),
                name_en: "Federal Territory Day".to_string(),
                name_ms: "Hari Wilayah Persekutuan".to_string(),
                federal: false,
            },
            MalaysianPublicHoliday {
                date: NaiveDate::from_ymd_opt(year, 5, 1).expect("Valid date"),
                name_en: "Labour Day".to_string(),
                name_ms: "Hari Pekerja".to_string(),
                federal: true,
            },
            MalaysianPublicHoliday {
                date: NaiveDate::from_ymd_opt(year, 6, 7).expect("Valid date"),
                name_en: "Birthday of Yang di-Pertuan Agong".to_string(),
                name_ms: "Hari Keputeraan Yang di-Pertuan Agong".to_string(),
                federal: true,
            },
            MalaysianPublicHoliday {
                date: NaiveDate::from_ymd_opt(year, 8, 31).expect("Valid date"),
                name_en: "National Day".to_string(),
                name_ms: "Hari Kebangsaan".to_string(),
                federal: true,
            },
            MalaysianPublicHoliday {
                date: NaiveDate::from_ymd_opt(year, 9, 16).expect("Valid date"),
                name_en: "Malaysia Day".to_string(),
                name_ms: "Hari Malaysia".to_string(),
                federal: true,
            },
            MalaysianPublicHoliday {
                date: NaiveDate::from_ymd_opt(year, 12, 25).expect("Valid date"),
                name_en: "Christmas Day".to_string(),
                name_ms: "Hari Krismas".to_string(),
                federal: true,
            },
            // Note: Islamic and Chinese holidays are based on lunar calendars
            // and need to be calculated separately or obtained from official sources
        ]
    }

    /// Checks if a date is a public holiday.
    #[must_use]
    pub fn is_holiday(&self, date: &NaiveDate) -> bool {
        self.holidays.iter().any(|h| h.date == *date)
    }

    /// Checks if a date is a working day (not weekend or holiday).
    #[must_use]
    pub fn is_working_day(&self, date: &NaiveDate) -> bool {
        !self.is_weekend(date) && !self.is_holiday(date)
    }

    /// Checks if a date is a weekend (Saturday or Sunday).
    #[must_use]
    pub fn is_weekend(&self, date: &NaiveDate) -> bool {
        matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
    }

    /// Gets all holidays in the calendar.
    #[must_use]
    pub fn holidays(&self) -> &[MalaysianPublicHoliday] {
        &self.holidays
    }

    /// Adds N working days to a date.
    #[must_use]
    pub fn add_working_days(&self, date: NaiveDate, working_days: i64) -> NaiveDate {
        let mut current = date;
        let mut remaining = working_days;

        while remaining > 0 {
            current += Duration::days(1);
            if self.is_working_day(&current) {
                remaining -= 1;
            }
        }

        current
    }

    /// Calculates a legal deadline from a starting date.
    ///
    /// If the deadline falls on a non-working day, it's extended to the next working day.
    #[must_use]
    pub fn calculate_deadline(&self, start: NaiveDate, days: i64) -> NaiveDate {
        let mut deadline = start + Duration::days(days);

        // Extend if deadline falls on non-working day
        while !self.is_working_day(&deadline) {
            deadline += Duration::days(1);
        }

        deadline
    }
}

/// Checks if a date is a Malaysian public holiday (federal).
///
/// # Example
///
/// ```rust
/// use chrono::NaiveDate;
/// use legalis_my::common::is_malaysian_holiday;
///
/// let date = NaiveDate::from_ymd_opt(2024, 8, 31).unwrap(); // National Day
/// assert!(is_malaysian_holiday(&date));
/// ```
#[must_use]
pub fn is_malaysian_holiday(date: &NaiveDate) -> bool {
    let calendar = MalaysianLegalCalendar::new(date.year());
    calendar.is_holiday(date)
}

/// Checks if a date is a working day (not weekend or holiday).
///
/// # Example
///
/// ```rust
/// use chrono::NaiveDate;
/// use legalis_my::common::is_working_day;
///
/// let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(); // Monday
/// assert!(is_working_day(&date));
/// ```
#[must_use]
pub fn is_working_day(date: &NaiveDate) -> bool {
    let calendar = MalaysianLegalCalendar::new(date.year());
    calendar.is_working_day(date)
}

/// Calculates a legal deadline from a starting date.
///
/// # Example
///
/// ```rust
/// use chrono::NaiveDate;
/// use legalis_my::common::calculate_legal_deadline;
///
/// let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
/// let deadline = calculate_legal_deadline(start, 30);
/// // Returns date 30 days later, extended if it falls on weekend/holiday
/// ```
#[must_use]
pub fn calculate_legal_deadline(start: NaiveDate, days: i64) -> NaiveDate {
    let calendar = MalaysianLegalCalendar::new(start.year());
    calendar.calculate_deadline(start, days)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_holiday() {
        let calendar = MalaysianLegalCalendar::new(2024);

        // National Day
        let national_day = NaiveDate::from_ymd_opt(2024, 8, 31).expect("Valid date");
        assert!(calendar.is_holiday(&national_day));

        // Regular working day
        let working_day = NaiveDate::from_ymd_opt(2024, 1, 15).expect("Valid date");
        assert!(!calendar.is_holiday(&working_day));
    }

    #[test]
    fn test_is_weekend() {
        let calendar = MalaysianLegalCalendar::new(2024);

        // Saturday
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).expect("Valid date");
        assert!(calendar.is_weekend(&saturday));

        // Sunday
        let sunday = NaiveDate::from_ymd_opt(2024, 1, 7).expect("Valid date");
        assert!(calendar.is_weekend(&sunday));

        // Monday
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).expect("Valid date");
        assert!(!calendar.is_weekend(&monday));
    }

    #[test]
    fn test_is_working_day() {
        let calendar = MalaysianLegalCalendar::new(2024);

        // Monday (working day)
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).expect("Valid date");
        assert!(calendar.is_working_day(&monday));

        // Saturday (weekend)
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).expect("Valid date");
        assert!(!calendar.is_working_day(&saturday));

        // New Year's Day (holiday)
        let new_year = NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date");
        assert!(!calendar.is_working_day(&new_year));
    }

    #[test]
    fn test_add_working_days() {
        let calendar = MalaysianLegalCalendar::new(2024);
        let start = NaiveDate::from_ymd_opt(2024, 1, 8).expect("Valid date"); // Monday

        let result = calendar.add_working_days(start, 5);
        assert!(calendar.is_working_day(&result));
    }

    #[test]
    fn test_calculate_deadline() {
        let calendar = MalaysianLegalCalendar::new(2024);
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date");

        let deadline = calendar.calculate_deadline(start, 30);
        assert!(calendar.is_working_day(&deadline));
    }
}
