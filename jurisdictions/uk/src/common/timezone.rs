//! UK timezone utilities for legal deadlines.
//!
//! The UK uses two time zones:
//! - **GMT** (Greenwich Mean Time): UTC+0, used in winter
//! - **BST** (British Summer Time): UTC+1, used in summer
//!
//! # Daylight Saving Time Rules
//!
//! BST begins at 01:00 GMT on the last Sunday of March.
//! BST ends at 02:00 BST on the last Sunday of October.
//!
//! # Legal Significance
//!
//! For legal deadlines and court filings in England & Wales:
//! - Deadlines typically expire at 23:59 local time
//! - During BST, this is 22:59 UTC
//! - During GMT, this is 23:59 UTC

use chrono::{Datelike, NaiveDate};
use legalis_i18n::TimeZone;

/// UK timezone representation with BST/GMT switching.
#[derive(Debug, Clone)]
pub struct UkTimeZone {
    gmt: TimeZone,
    bst: TimeZone,
}

impl Default for UkTimeZone {
    fn default() -> Self {
        Self::new()
    }
}

impl UkTimeZone {
    /// Creates a new UK timezone handler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            gmt: TimeZone::new("Europe/London", 0, "Greenwich Mean Time (GMT)", true),
            bst: TimeZone::new("Europe/London", 60, "British Summer Time (BST)", true),
        }
    }

    /// Gets the current timezone for a given date.
    ///
    /// Returns BST during summer (last Sunday of March to last Sunday of October),
    /// GMT otherwise.
    #[must_use]
    pub fn timezone_for_date(&self, date: NaiveDate) -> &TimeZone {
        if is_bst(date) { &self.bst } else { &self.gmt }
    }

    /// Gets the UTC offset in minutes for a given date.
    #[must_use]
    pub fn offset_minutes(&self, date: NaiveDate) -> i32 {
        if is_bst(date) {
            60 // BST: UTC+1
        } else {
            0 // GMT: UTC+0
        }
    }

    /// Gets the timezone name for a given date.
    #[must_use]
    pub fn timezone_name(&self, date: NaiveDate) -> &'static str {
        if is_bst(date) { "BST" } else { "GMT" }
    }

    /// Gets the full timezone display name.
    #[must_use]
    pub fn timezone_display_name(&self, date: NaiveDate) -> &'static str {
        if is_bst(date) {
            "British Summer Time"
        } else {
            "Greenwich Mean Time"
        }
    }

    /// Converts UTC time to UK local time.
    #[must_use]
    pub fn utc_to_local(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> (i32, u32, u32, u32, u32) {
        let date = NaiveDate::from_ymd_opt(year, month, day);
        let tz = date.map(|d| self.timezone_for_date(d)).unwrap_or(&self.gmt);
        tz.utc_to_local(year, month, day, hour, minute)
    }

    /// Converts UK local time to UTC.
    #[must_use]
    pub fn local_to_utc(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> (i32, u32, u32, u32, u32) {
        let date = NaiveDate::from_ymd_opt(year, month, day);
        let tz = date.map(|d| self.timezone_for_date(d)).unwrap_or(&self.gmt);
        tz.local_to_utc(year, month, day, hour, minute)
    }

    /// Formats the current offset as a string (e.g., "+00:00" or "+01:00").
    #[must_use]
    pub fn format_offset(&self, date: NaiveDate) -> String {
        if is_bst(date) {
            "+01:00".to_string()
        } else {
            "+00:00".to_string()
        }
    }
}

/// Checks if a date falls within British Summer Time (BST).
///
/// BST runs from the last Sunday of March at 01:00 GMT
/// to the last Sunday of October at 02:00 BST.
///
/// # Example
///
/// ```rust
/// use legalis_uk::common::is_bst;
/// use chrono::NaiveDate;
///
/// // Summer date - should be BST
/// let summer = NaiveDate::from_ymd_opt(2025, 7, 15).unwrap();
/// assert!(is_bst(summer));
///
/// // Winter date - should be GMT
/// let winter = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
/// assert!(!is_bst(winter));
/// ```
#[must_use]
pub fn is_bst(date: NaiveDate) -> bool {
    let year = date.year();

    // Find last Sunday of March (BST starts)
    let bst_start = last_sunday_of_month(year, 3);

    // Find last Sunday of October (BST ends)
    let bst_end = last_sunday_of_month(year, 10);

    match (bst_start, bst_end) {
        (Some(start), Some(end)) => date >= start && date < end,
        _ => false,
    }
}

/// Returns the current UK offset in minutes for a given date.
///
/// - GMT: 0 (UTC+0)
/// - BST: 60 (UTC+1)
#[must_use]
pub fn current_uk_offset(date: NaiveDate) -> i32 {
    if is_bst(date) { 60 } else { 0 }
}

/// Converts a UTC datetime to UK local time.
///
/// # Example
///
/// ```rust
/// use legalis_uk::common::convert_to_uk_local;
/// use chrono::NaiveDate;
///
/// // Summer: 12:00 UTC = 13:00 BST
/// let result = convert_to_uk_local(2025, 7, 15, 12, 0);
/// assert_eq!(result, (2025, 7, 15, 13, 0));
///
/// // Winter: 12:00 UTC = 12:00 GMT
/// let result = convert_to_uk_local(2025, 1, 15, 12, 0);
/// assert_eq!(result, (2025, 1, 15, 12, 0));
/// ```
#[must_use]
pub fn convert_to_uk_local(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
) -> (i32, u32, u32, u32, u32) {
    UkTimeZone::new().utc_to_local(year, month, day, hour, minute)
}

/// Finds the last Sunday of a given month.
fn last_sunday_of_month(year: i32, month: u32) -> Option<NaiveDate> {
    let last_of_month = if month == 12 {
        NaiveDate::from_ymd_opt(year, 12, 31)?
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)? - chrono::Duration::days(1)
    };

    let days_since_sunday = last_of_month.weekday().num_days_from_sunday();
    Some(last_of_month - chrono::Duration::days(days_since_sunday as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // BST detection tests
    // ========================================================================

    #[test]
    fn test_is_bst_summer() {
        let summer = NaiveDate::from_ymd_opt(2025, 7, 15).unwrap();
        assert!(is_bst(summer));
    }

    #[test]
    fn test_is_bst_winter() {
        let winter = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        assert!(!is_bst(winter));
    }

    #[test]
    fn test_is_bst_march_before_switch() {
        // In 2025, last Sunday of March is March 30
        let before_switch = NaiveDate::from_ymd_opt(2025, 3, 29).unwrap();
        assert!(!is_bst(before_switch));
    }

    #[test]
    fn test_is_bst_march_after_switch() {
        // In 2025, last Sunday of March is March 30
        let after_switch = NaiveDate::from_ymd_opt(2025, 3, 30).unwrap();
        assert!(is_bst(after_switch));
    }

    #[test]
    fn test_is_bst_october_before_switch() {
        // In 2025, last Sunday of October is October 26
        let before_switch = NaiveDate::from_ymd_opt(2025, 10, 25).unwrap();
        assert!(is_bst(before_switch));
    }

    #[test]
    fn test_is_bst_october_switch_day() {
        // Last Sunday of October - clocks go back, so this day is NOT BST
        let switch_day = NaiveDate::from_ymd_opt(2025, 10, 26).unwrap();
        assert!(!is_bst(switch_day));
    }

    // ========================================================================
    // Offset tests
    // ========================================================================

    #[test]
    fn test_current_uk_offset_summer() {
        let summer = NaiveDate::from_ymd_opt(2025, 7, 15).unwrap();
        assert_eq!(current_uk_offset(summer), 60);
    }

    #[test]
    fn test_current_uk_offset_winter() {
        let winter = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        assert_eq!(current_uk_offset(winter), 0);
    }

    // ========================================================================
    // Conversion tests
    // ========================================================================

    #[test]
    fn test_utc_to_uk_local_summer() {
        // 12:00 UTC in summer should be 13:00 BST
        let result = convert_to_uk_local(2025, 7, 15, 12, 0);
        assert_eq!(result, (2025, 7, 15, 13, 0));
    }

    #[test]
    fn test_utc_to_uk_local_winter() {
        // 12:00 UTC in winter should be 12:00 GMT
        let result = convert_to_uk_local(2025, 1, 15, 12, 0);
        assert_eq!(result, (2025, 1, 15, 12, 0));
    }

    #[test]
    fn test_utc_to_uk_local_day_boundary_summer() {
        // 23:30 UTC in summer should be 00:30 BST next day
        let result = convert_to_uk_local(2025, 7, 15, 23, 30);
        assert_eq!(result, (2025, 7, 16, 0, 30));
    }

    // ========================================================================
    // UkTimeZone tests
    // ========================================================================

    #[test]
    fn test_uk_timezone_name_summer() {
        let tz = UkTimeZone::new();
        let summer = NaiveDate::from_ymd_opt(2025, 7, 15).unwrap();
        assert_eq!(tz.timezone_name(summer), "BST");
    }

    #[test]
    fn test_uk_timezone_name_winter() {
        let tz = UkTimeZone::new();
        let winter = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        assert_eq!(tz.timezone_name(winter), "GMT");
    }

    #[test]
    fn test_uk_timezone_display_name() {
        let tz = UkTimeZone::new();
        let summer = NaiveDate::from_ymd_opt(2025, 7, 15).unwrap();
        assert_eq!(tz.timezone_display_name(summer), "British Summer Time");

        let winter = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        assert_eq!(tz.timezone_display_name(winter), "Greenwich Mean Time");
    }

    #[test]
    fn test_uk_timezone_format_offset() {
        let tz = UkTimeZone::new();
        let summer = NaiveDate::from_ymd_opt(2025, 7, 15).unwrap();
        assert_eq!(tz.format_offset(summer), "+01:00");

        let winter = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        assert_eq!(tz.format_offset(winter), "+00:00");
    }

    // ========================================================================
    // Last Sunday calculation tests
    // ========================================================================

    #[test]
    fn test_last_sunday_march_2025() {
        let result = last_sunday_of_month(2025, 3);
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 3, 30));
    }

    #[test]
    fn test_last_sunday_october_2025() {
        let result = last_sunday_of_month(2025, 10);
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 10, 26));
    }
}
