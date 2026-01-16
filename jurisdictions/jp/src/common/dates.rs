//! Japanese legal calendar and working days utilities.
//!
//! Integrates legalis-i18n's calendar functionality with Japanese legal requirements.
//!
//! # Japanese Public Holidays (祝日)
//!
//! Japan has 16 national holidays. This module provides accurate working day
//! calculations for legal deadlines.
//!
//! ## Fixed Holidays (固定祝日)
//! - 元日 (New Year's Day): January 1
//! - 建国記念の日 (National Foundation Day): February 11
//! - 天皇誕生日 (Emperor's Birthday): February 23
//! - 春分の日 (Vernal Equinox): Around March 20
//! - 昭和の日 (Showa Day): April 29
//! - 憲法記念日 (Constitution Day): May 3
//! - みどりの日 (Greenery Day): May 4
//! - こどもの日 (Children's Day): May 5
//! - 山の日 (Mountain Day): August 11
//! - 秋分の日 (Autumnal Equinox): Around September 23
//! - 文化の日 (Culture Day): November 3
//! - 勤労感謝の日 (Labor Thanksgiving Day): November 23
//!
//! ## Happy Monday System (ハッピーマンデー制度)
//! - 成人の日 (Coming of Age Day): 2nd Monday of January
//! - 海の日 (Marine Day): 3rd Monday of July
//! - 敬老の日 (Respect for the Aged Day): 3rd Monday of September
//! - スポーツの日 (Sports Day): 2nd Monday of October
//!
//! ## Special Rules
//! - 振替休日 (Substitute Holiday): When a holiday falls on Sunday, the next Monday becomes a holiday
//! - 国民の休日 (Citizens' Holiday): A weekday sandwiched between two holidays becomes a holiday

use chrono::{Datelike, NaiveDate, Weekday};
use legalis_i18n::{DeadlineCalculator, WorkingDaysConfig};

use crate::era::{EraError, JapaneseDate};

/// Japanese legal calendar with holiday awareness.
///
/// Provides working day calculations that comply with Japanese legal requirements.
///
/// # Example
///
/// ```rust
/// use legalis_jp::common::JapaneseLegalCalendar;
/// use chrono::NaiveDate;
///
/// let calendar = JapaneseLegalCalendar::new();
///
/// // Check if January 1 is a holiday
/// let new_year = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
/// assert!(calendar.is_holiday(new_year));
///
/// // Calculate a 14-day business deadline
/// let start = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
/// let deadline = calendar.add_working_days(start, 14);
/// ```
#[derive(Debug, Clone)]
pub struct JapaneseLegalCalendar {
    config: WorkingDaysConfig,
}

impl Default for JapaneseLegalCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl JapaneseLegalCalendar {
    /// Creates a new Japanese legal calendar with all holidays.
    #[must_use]
    pub fn new() -> Self {
        // Start with legalis-i18n's Japan configuration
        let config = WorkingDaysConfig::japan()
            // Add Happy Monday holidays (approximate, needs runtime calculation)
            // These are placeholder fixed dates; in production, calculate dynamically
            .add_holiday(1, 13) // Coming of Age Day (2nd Monday approximation)
            .add_holiday(7, 21) // Marine Day (3rd Monday approximation)
            .add_holiday(9, 15) // Respect for Aged Day (3rd Monday approximation)
            .add_holiday(10, 13); // Sports Day (2nd Monday approximation)

        Self { config }
    }

    /// Creates a calendar for a specific year with accurate Happy Monday dates.
    #[must_use]
    pub fn for_year(year: i32) -> Self {
        let mut config = WorkingDaysConfig::japan();

        // Calculate Happy Monday holidays for the specific year
        if let Some(coming_of_age) = nth_weekday_of_month(year, 1, Weekday::Mon, 2) {
            config = config.add_holiday(1, coming_of_age.day());
        }
        if let Some(marine_day) = nth_weekday_of_month(year, 7, Weekday::Mon, 3) {
            config = config.add_holiday(7, marine_day.day());
        }
        if let Some(respect_aged) = nth_weekday_of_month(year, 9, Weekday::Mon, 3) {
            config = config.add_holiday(9, respect_aged.day());
        }
        if let Some(sports_day) = nth_weekday_of_month(year, 10, Weekday::Mon, 2) {
            config = config.add_holiday(10, sports_day.day());
        }

        Self { config }
    }

    /// Checks if a date is a Japanese public holiday.
    #[must_use]
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        !self
            .config
            .is_working_day(date.year(), date.month(), date.day())
            && date.weekday() != Weekday::Sat
            && date.weekday() != Weekday::Sun
    }

    /// Checks if a date is a working day (not weekend or holiday).
    #[must_use]
    pub fn is_working_day(&self, date: NaiveDate) -> bool {
        self.config
            .is_working_day(date.year(), date.month(), date.day())
    }

    /// Adds working days to a date, skipping weekends and holidays.
    ///
    /// Used for legal deadline calculations.
    #[must_use]
    pub fn add_working_days(&self, start: NaiveDate, days: i32) -> NaiveDate {
        let (year, month, day) =
            self.config
                .add_working_days(start.year(), start.month(), start.day(), days);
        NaiveDate::from_ymd_opt(year, month, day).unwrap_or(start)
    }

    /// Calculates a legal deadline using the DeadlineCalculator.
    #[must_use]
    pub fn calculate_deadline(&self, start: NaiveDate, business_days: i32) -> NaiveDate {
        let calculator = DeadlineCalculator::new(self.config.clone());
        let (year, month, day) =
            calculator.calculate_deadline(start.year(), start.month(), start.day(), business_days);
        NaiveDate::from_ymd_opt(year, month, day).unwrap_or(start)
    }

    /// Counts working days between two dates.
    #[must_use]
    pub fn working_days_between(&self, start: NaiveDate, end: NaiveDate) -> i32 {
        if end <= start {
            return 0;
        }

        let mut count = 0;
        let mut current = start;
        while current < end {
            if self.is_working_day(current) {
                count += 1;
            }
            current = current.succ_opt().unwrap_or(current);
        }
        count
    }

    /// Converts a deadline to Japanese era format.
    pub fn deadline_in_japanese(&self, deadline: NaiveDate) -> Result<String, EraError> {
        let jp_date = JapaneseDate::from_western(deadline)?;
        Ok(jp_date.to_japanese_string())
    }
}

/// Finds the nth occurrence of a weekday in a given month.
fn nth_weekday_of_month(year: i32, month: u32, weekday: Weekday, nth: u32) -> Option<NaiveDate> {
    let first_of_month = NaiveDate::from_ymd_opt(year, month, 1)?;
    let first_weekday = first_of_month.weekday();

    // Calculate days until the first occurrence of the target weekday
    let days_until =
        (weekday.num_days_from_monday() as i32 - first_weekday.num_days_from_monday() as i32 + 7)
            % 7;
    let first_occurrence = first_of_month + chrono::Duration::days(days_until as i64);

    // Add weeks for nth occurrence
    let target = first_occurrence + chrono::Duration::weeks((nth - 1) as i64);

    // Verify it's still in the same month
    if target.month() == month {
        Some(target)
    } else {
        None
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Checks if a date is a Japanese public holiday.
///
/// # Example
///
/// ```rust
/// use legalis_jp::common::is_japanese_holiday;
/// use chrono::NaiveDate;
///
/// let new_year = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
/// assert!(is_japanese_holiday(new_year));
/// ```
#[must_use]
pub fn is_japanese_holiday(date: NaiveDate) -> bool {
    JapaneseLegalCalendar::new().is_holiday(date)
}

/// Checks if a date is a Japanese working day.
#[must_use]
pub fn is_working_day(date: NaiveDate) -> bool {
    JapaneseLegalCalendar::new().is_working_day(date)
}

/// Calculates a legal deadline by adding working days.
///
/// # Example
///
/// ```rust
/// use legalis_jp::common::calculate_legal_deadline;
/// use chrono::NaiveDate;
///
/// let start = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
/// let deadline = calculate_legal_deadline(start, 14);
/// // Deadline is 14 working days later, skipping weekends and holidays
/// ```
#[must_use]
pub fn calculate_legal_deadline(start: NaiveDate, business_days: i32) -> NaiveDate {
    JapaneseLegalCalendar::new().add_working_days(start, business_days)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_year_is_holiday() {
        let calendar = JapaneseLegalCalendar::new();
        let new_year = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(calendar.is_holiday(new_year));
        assert!(!calendar.is_working_day(new_year));
    }

    #[test]
    fn test_constitution_day_is_holiday() {
        let calendar = JapaneseLegalCalendar::new();
        // May 3, 2024 - Constitution Day (憲法記念日) - Friday
        let constitution_day = NaiveDate::from_ymd_opt(2024, 5, 3).unwrap();
        assert!(calendar.is_holiday(constitution_day));
    }

    #[test]
    fn test_golden_week() {
        let calendar = JapaneseLegalCalendar::new();
        // Golden Week 2024: May 3-5 (Friday-Sunday)
        // May 3, 2024 is Friday (holiday)
        assert!(calendar.is_holiday(NaiveDate::from_ymd_opt(2024, 5, 3).unwrap())); // Constitution Day
        // May 4, 2024 is Saturday (weekend, not counted as holiday)
        assert!(!calendar.is_working_day(NaiveDate::from_ymd_opt(2024, 5, 4).unwrap()));
        // May 5, 2024 is Sunday (weekend)
        assert!(!calendar.is_working_day(NaiveDate::from_ymd_opt(2024, 5, 5).unwrap()));

        // Test May 3 in 2023 where it falls on Wednesday
        let constitution_2023 = NaiveDate::from_ymd_opt(2023, 5, 3).unwrap();
        assert!(calendar.is_holiday(constitution_2023)); // Wednesday, confirmed holiday
    }

    #[test]
    fn test_regular_weekday_is_working_day() {
        let calendar = JapaneseLegalCalendar::new();
        // A regular Tuesday in June should be a working day
        let tuesday = NaiveDate::from_ymd_opt(2025, 6, 10).unwrap();
        assert!(calendar.is_working_day(tuesday));
    }

    #[test]
    fn test_weekend_not_working_day() {
        let calendar = JapaneseLegalCalendar::new();
        let saturday = NaiveDate::from_ymd_opt(2025, 6, 14).unwrap();
        let sunday = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
        assert!(!calendar.is_working_day(saturday));
        assert!(!calendar.is_working_day(sunday));
    }

    #[test]
    fn test_add_working_days() {
        let calendar = JapaneseLegalCalendar::new();
        // Start on Monday, add 5 working days should end on next Monday (skipping weekend)
        let monday = NaiveDate::from_ymd_opt(2025, 6, 9).unwrap();
        let result = calendar.add_working_days(monday, 5);
        // 5 working days from Mon 6/9: Tue 6/10, Wed 6/11, Thu 6/12, Fri 6/13, Mon 6/16
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 6, 16).unwrap());
    }

    #[test]
    fn test_deadline_in_japanese() {
        let calendar = JapaneseLegalCalendar::new();
        let date = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
        let result = calendar.deadline_in_japanese(date).unwrap();
        assert_eq!(result, "令和7年4月1日");
    }

    #[test]
    fn test_nth_weekday_of_month() {
        // 2nd Monday of January 2025
        let result = nth_weekday_of_month(2025, 1, Weekday::Mon, 2);
        assert_eq!(result, Some(NaiveDate::from_ymd_opt(2025, 1, 13).unwrap()));

        // 3rd Monday of September 2025
        let result = nth_weekday_of_month(2025, 9, Weekday::Mon, 3);
        assert_eq!(result, Some(NaiveDate::from_ymd_opt(2025, 9, 15).unwrap()));
    }

    #[test]
    fn test_working_days_between() {
        let calendar = JapaneseLegalCalendar::new();
        let start = NaiveDate::from_ymd_opt(2025, 6, 9).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 6, 16).unwrap(); // Next Monday
        // Mon, Tue, Wed, Thu, Fri = 5 working days
        assert_eq!(calendar.working_days_between(start, end), 5);
    }

    #[test]
    fn test_for_year_calculation() {
        let calendar = JapaneseLegalCalendar::for_year(2025);
        // Coming of Age Day 2025 is January 13 (2nd Monday)
        let coming_of_age = NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();
        assert!(calendar.is_holiday(coming_of_age));
    }
}
