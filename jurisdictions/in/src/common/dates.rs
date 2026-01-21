//! Indian Legal Calendar and Date Utilities
//!
//! Provides date calculation utilities for Indian legal deadlines, including:
//! - National holidays (Republic Day, Independence Day, Gandhi Jayanti)
//! - Working day calculations
//! - Legal deadline calculations
//!
//! ## National Holidays
//!
//! India observes three national holidays under the Negotiable Instruments Act 1881:
//! - **Republic Day**: January 26
//! - **Independence Day**: August 15
//! - **Gandhi Jayanti**: October 2
//!
//! ## Usage
//!
//! ```rust
//! use legalis_in::common::dates::*;
//! use chrono::NaiveDate;
//!
//! let date = NaiveDate::from_ymd_opt(2024, 1, 26).unwrap();
//! assert!(is_national_holiday(date));
//!
//! let deadline = calculate_deadline(
//!     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     30,
//!     DeadlineType::WorkingDays
//! );
//! ```

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Type of deadline calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeadlineType {
    /// Calendar days (includes weekends and holidays)
    CalendarDays,
    /// Working days (excludes weekends and national holidays)
    WorkingDays,
    /// Business days (excludes weekends only)
    BusinessDays,
}

/// Indian national holiday
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NationalHoliday {
    /// Republic Day - January 26
    RepublicDay,
    /// Independence Day - August 15
    IndependenceDay,
    /// Gandhi Jayanti - October 2
    GandhiJayanti,
}

impl NationalHoliday {
    /// Get the date for this holiday in a given year
    pub fn date_in_year(&self, year: i32) -> NaiveDate {
        match self {
            NationalHoliday::RepublicDay => {
                NaiveDate::from_ymd_opt(year, 1, 26).expect("Invalid Republic Day date")
            }
            NationalHoliday::IndependenceDay => {
                NaiveDate::from_ymd_opt(year, 8, 15).expect("Invalid Independence Day date")
            }
            NationalHoliday::GandhiJayanti => {
                NaiveDate::from_ymd_opt(year, 10, 2).expect("Invalid Gandhi Jayanti date")
            }
        }
    }

    /// Get the name of this holiday
    pub fn name(&self) -> &'static str {
        match self {
            NationalHoliday::RepublicDay => "Republic Day",
            NationalHoliday::IndependenceDay => "Independence Day",
            NationalHoliday::GandhiJayanti => "Gandhi Jayanti (Mahatma Gandhi's Birthday)",
        }
    }

    /// Get all national holidays
    pub fn all() -> Vec<NationalHoliday> {
        vec![
            NationalHoliday::RepublicDay,
            NationalHoliday::IndependenceDay,
            NationalHoliday::GandhiJayanti,
        ]
    }
}

/// Check if a date is a national holiday in India
pub fn is_national_holiday(date: NaiveDate) -> bool {
    let year = date.year();
    NationalHoliday::all()
        .iter()
        .any(|holiday| holiday.date_in_year(year) == date)
}

/// Check if a date is a weekend (Saturday or Sunday)
pub fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

/// Check if a date is a working day (not weekend and not national holiday)
pub fn is_working_day(date: NaiveDate) -> bool {
    !is_weekend(date) && !is_national_holiday(date)
}

/// Calculate deadline from a start date
///
/// # Arguments
///
/// * `start_date` - The starting date
/// * `days` - Number of days to add
/// * `deadline_type` - Type of deadline calculation
///
/// # Examples
///
/// ```
/// use legalis_in::common::dates::*;
/// use chrono::NaiveDate;
///
/// let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
/// let deadline = calculate_deadline(start, 30, DeadlineType::WorkingDays);
/// ```
pub fn calculate_deadline(
    start_date: NaiveDate,
    days: u32,
    deadline_type: DeadlineType,
) -> NaiveDate {
    match deadline_type {
        DeadlineType::CalendarDays => start_date + Duration::days(days as i64),
        DeadlineType::BusinessDays => {
            let mut current = start_date;
            let mut remaining = days;

            while remaining > 0 {
                current += Duration::days(1);
                if !is_weekend(current) {
                    remaining -= 1;
                }
            }

            current
        }
        DeadlineType::WorkingDays => {
            let mut current = start_date;
            let mut remaining = days;

            while remaining > 0 {
                current += Duration::days(1);
                if is_working_day(current) {
                    remaining -= 1;
                }
            }

            current
        }
    }
}

/// Calculate working days between two dates (inclusive)
pub fn working_days_between(start: NaiveDate, end: NaiveDate) -> u32 {
    if start > end {
        return 0;
    }

    let mut count = 0;
    let mut current = start;

    while current <= end {
        if is_working_day(current) {
            count += 1;
        }
        current += Duration::days(1);
    }

    count
}

/// Calculate business days between two dates (inclusive)
pub fn business_days_between(start: NaiveDate, end: NaiveDate) -> u32 {
    if start > end {
        return 0;
    }

    let mut count = 0;
    let mut current = start;

    while current <= end {
        if !is_weekend(current) {
            count += 1;
        }
        current += Duration::days(1);
    }

    count
}

/// Get the next working day from a given date
pub fn next_working_day(date: NaiveDate) -> NaiveDate {
    let mut next = date + Duration::days(1);
    while !is_working_day(next) {
        next += Duration::days(1);
    }
    next
}

/// Get the previous working day from a given date
pub fn previous_working_day(date: NaiveDate) -> NaiveDate {
    let mut prev = date - Duration::days(1);
    while !is_working_day(prev) {
        prev -= Duration::days(1);
    }
    prev
}

/// Get all national holidays in a given year
pub fn national_holidays_in_year(year: i32) -> Vec<(NationalHoliday, NaiveDate)> {
    NationalHoliday::all()
        .into_iter()
        .map(|holiday| (holiday, holiday.date_in_year(year)))
        .collect()
}

/// Financial year in India (April 1 to March 31)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FinancialYear {
    /// Starting year (e.g., 2023 for FY 2023-24)
    pub year: i32,
}

impl FinancialYear {
    /// Create a new financial year
    pub fn new(year: i32) -> Self {
        Self { year }
    }

    /// Get the financial year for a given date
    pub fn from_date(date: NaiveDate) -> Self {
        let year = date.year();
        let month = date.month();

        if month >= 4 {
            Self { year }
        } else {
            Self { year: year - 1 }
        }
    }

    /// Get the start date of the financial year
    pub fn start_date(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year, 4, 1).expect("Invalid FY start date")
    }

    /// Get the end date of the financial year
    pub fn end_date(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year + 1, 3, 31).expect("Invalid FY end date")
    }

    /// Get the display name (e.g., "FY 2023-24")
    pub fn display_name(&self) -> String {
        format!("FY {}-{:02}", self.year, (self.year + 1) % 100)
    }

    /// Check if a date falls within this financial year
    pub fn contains(&self, date: NaiveDate) -> bool {
        date >= self.start_date() && date <= self.end_date()
    }
}

impl std::fmt::Display for FinancialYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_national_holidays() {
        let republic_day = NaiveDate::from_ymd_opt(2024, 1, 26).unwrap();
        let independence_day = NaiveDate::from_ymd_opt(2024, 8, 15).unwrap();
        let gandhi_jayanti = NaiveDate::from_ymd_opt(2024, 10, 2).unwrap();

        assert!(is_national_holiday(republic_day));
        assert!(is_national_holiday(independence_day));
        assert!(is_national_holiday(gandhi_jayanti));

        let regular_day = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
        assert!(!is_national_holiday(regular_day));
    }

    #[test]
    fn test_weekend() {
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap();
        let sunday = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();

        assert!(is_weekend(saturday));
        assert!(is_weekend(sunday));
        assert!(!is_weekend(monday));
    }

    #[test]
    fn test_working_day() {
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap();
        let republic_day = NaiveDate::from_ymd_opt(2024, 1, 26).unwrap();

        assert!(is_working_day(monday));
        assert!(!is_working_day(saturday));
        assert!(!is_working_day(republic_day));
    }

    #[test]
    fn test_calculate_deadline() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        // Calendar days
        let calendar_deadline = calculate_deadline(start, 30, DeadlineType::CalendarDays);
        assert_eq!(
            calendar_deadline,
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap()
        );

        // Business days (excludes weekends)
        let business_deadline = calculate_deadline(start, 5, DeadlineType::BusinessDays);
        assert!(business_deadline > start);
    }

    #[test]
    fn test_working_days_between() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(); // Friday

        let days = working_days_between(start, end);
        // Jan 1-5, 2024: Mon-Fri, no holidays
        assert_eq!(days, 5);
    }

    #[test]
    fn test_next_working_day() {
        let friday = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap();
        let next = next_working_day(friday);
        // Next working day after Friday should be Monday
        assert_eq!(next, NaiveDate::from_ymd_opt(2024, 1, 8).unwrap());
    }

    #[test]
    fn test_financial_year() {
        let fy = FinancialYear::new(2023);
        assert_eq!(
            fy.start_date(),
            NaiveDate::from_ymd_opt(2023, 4, 1).unwrap()
        );
        assert_eq!(fy.end_date(), NaiveDate::from_ymd_opt(2024, 3, 31).unwrap());
        assert_eq!(fy.display_name(), "FY 2023-24");

        let date_in_fy = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        assert!(fy.contains(date_in_fy));

        let fy_from_date = FinancialYear::from_date(date_in_fy);
        assert_eq!(fy_from_date.year, 2023);
    }

    #[test]
    fn test_national_holidays_in_year() {
        let holidays = national_holidays_in_year(2024);
        assert_eq!(holidays.len(), 3);

        let names: Vec<&str> = holidays.iter().map(|(h, _)| h.name()).collect();
        assert!(names.contains(&"Republic Day"));
        assert!(names.contains(&"Independence Day"));
        assert!(names.contains(&"Gandhi Jayanti (Mahatma Gandhi's Birthday)"));
    }
}
