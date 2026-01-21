//! UK legal calendar and working days utilities.
//!
//! Integrates legalis-i18n's calendar functionality with UK legal requirements.
//!
//! # UK Bank Holidays
//!
//! Bank holidays differ between UK regions:
//!
//! ## England & Wales
//! - New Year's Day (1 January, or substitute)
//! - Good Friday (movable)
//! - Easter Monday (movable)
//! - Early May Bank Holiday (first Monday in May)
//! - Spring Bank Holiday (last Monday in May)
//! - Summer Bank Holiday (last Monday in August)
//! - Christmas Day (25 December, or substitute)
//! - Boxing Day (26 December, or substitute)
//!
//! ## Scotland
//! - New Year's Day (1 January, or substitute)
//! - 2nd January (or substitute)
//! - Good Friday (movable)
//! - Early May Bank Holiday
//! - Spring Bank Holiday
//! - Summer Bank Holiday (first Monday in August)
//! - St Andrew's Day (30 November, or substitute)
//! - Christmas Day
//! - Boxing Day
//!
//! ## Northern Ireland
//! - Same as England & Wales, plus:
//! - St Patrick's Day (17 March, or substitute)
//! - Battle of the Boyne (12 July, or substitute)
//!
//! ## Substitute Day Rule
//! When a bank holiday falls on a weekend, a substitute weekday is given:
//! - Saturday: Following Monday
//! - Sunday: Following Monday (or Tuesday if Monday is already a holiday)

use chrono::{Datelike, NaiveDate, Weekday};
use legalis_i18n::{DeadlineCalculator, WorkingDaysConfig};
use serde::{Deserialize, Serialize};

/// UK regions with different bank holiday schedules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum UkRegion {
    /// England and Wales
    #[default]
    EnglandWales,
    /// Scotland
    Scotland,
    /// Northern Ireland
    NorthernIreland,
}

impl UkRegion {
    /// Returns the region name.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::EnglandWales => "England & Wales",
            Self::Scotland => "Scotland",
            Self::NorthernIreland => "Northern Ireland",
        }
    }

    /// Returns the ISO 3166-2 code.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::EnglandWales => "GB-EAW",
            Self::Scotland => "GB-SCT",
            Self::NorthernIreland => "GB-NIR",
        }
    }

    /// Checks if this region has St Patrick's Day as a bank holiday.
    #[must_use]
    pub fn has_st_patricks_day(&self) -> bool {
        matches!(self, Self::NorthernIreland)
    }

    /// Checks if this region has Battle of the Boyne as a bank holiday.
    #[must_use]
    pub fn has_battle_of_boyne(&self) -> bool {
        matches!(self, Self::NorthernIreland)
    }

    /// Checks if this region has St Andrew's Day as a bank holiday.
    #[must_use]
    pub fn has_st_andrews_day(&self) -> bool {
        matches!(self, Self::Scotland)
    }

    /// Checks if this region has 2nd January as a bank holiday.
    #[must_use]
    pub fn has_second_january(&self) -> bool {
        matches!(self, Self::Scotland)
    }

    /// Returns the month for summer bank holiday (August).
    /// Scotland: First Monday, Others: Last Monday
    #[must_use]
    pub fn summer_bank_holiday_week(&self) -> u32 {
        match self {
            Self::Scotland => 1, // First Monday
            _ => 0,              // Last Monday (0 indicates "last")
        }
    }
}

/// UK legal calendar with bank holiday awareness.
///
/// Provides working day calculations that comply with UK legal requirements.
///
/// # Example
///
/// ```rust
/// use legalis_uk::common::{UkLegalCalendar, UkRegion};
/// use chrono::NaiveDate;
///
/// // Create calendar for England & Wales
/// let calendar = UkLegalCalendar::for_year(2025);
///
/// // Check if Christmas Day is a bank holiday
/// let christmas = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
/// assert!(calendar.is_bank_holiday(christmas));
///
/// // Calculate a 14-day business deadline
/// let start = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
/// let deadline = calendar.add_working_days(start, 14);
/// ```
#[derive(Debug, Clone)]
pub struct UkLegalCalendar {
    config: WorkingDaysConfig,
    region: UkRegion,
    year: i32,
    /// Good Friday
    good_friday: Option<NaiveDate>,
    /// Easter Monday
    easter_monday: Option<NaiveDate>,
}

impl Default for UkLegalCalendar {
    fn default() -> Self {
        Self::for_year(chrono::Utc::now().year())
    }
}

impl UkLegalCalendar {
    /// Creates a calendar for a specific year (England & Wales).
    #[must_use]
    pub fn for_year(year: i32) -> Self {
        Self::for_year_and_region(year, UkRegion::EnglandWales)
    }

    /// Creates a calendar for a specific year and UK region.
    #[must_use]
    pub fn for_year_and_region(year: i32, region: UkRegion) -> Self {
        let mut config = WorkingDaysConfig::new("GB")
            // Fixed holidays (will handle substitutes separately)
            .add_holiday(1, 1) // New Year's Day
            .add_holiday(12, 25) // Christmas Day
            .add_holiday(12, 26); // Boxing Day

        // Region-specific fixed holidays
        if region.has_second_january() {
            config = config.add_holiday(1, 2);
        }
        if region.has_st_patricks_day() {
            config = config.add_holiday(3, 17);
        }
        if region.has_battle_of_boyne() {
            config = config.add_holiday(7, 12);
        }
        if region.has_st_andrews_day() {
            config = config.add_holiday(11, 30);
        }

        // Calculate Easter-based holidays
        let easter = calculate_easter(year);
        let good_friday = easter.map(|e| e - chrono::Duration::days(2));
        let easter_monday = easter.map(|e| e + chrono::Duration::days(1));

        Self {
            config,
            region,
            year,
            good_friday,
            easter_monday,
        }
    }

    /// Returns the calendar year.
    #[must_use]
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Returns the UK region for this calendar.
    #[must_use]
    pub fn region(&self) -> UkRegion {
        self.region
    }

    /// Checks if a date is a UK bank holiday.
    #[must_use]
    pub fn is_bank_holiday(&self, date: NaiveDate) -> bool {
        // Check fixed holidays (including substitutes)
        if self.is_fixed_bank_holiday(date) {
            return true;
        }

        // Check movable holidays
        if self.is_movable_bank_holiday(date) {
            return true;
        }

        // Check substitute days
        if self.is_substitute_day(date) {
            return true;
        }

        false
    }

    /// Checks if date is a fixed bank holiday.
    fn is_fixed_bank_holiday(&self, date: NaiveDate) -> bool {
        match (date.month(), date.day()) {
            (1, 1) => true,   // New Year's Day
            (12, 25) => true, // Christmas Day
            (12, 26) => true, // Boxing Day
            (1, 2) if self.region.has_second_january() => true,
            (3, 17) if self.region.has_st_patricks_day() => true,
            (7, 12) if self.region.has_battle_of_boyne() => true,
            (11, 30) if self.region.has_st_andrews_day() => true,
            _ => false,
        }
    }

    /// Checks if date is a movable bank holiday (Easter-based and Monday bank holidays).
    fn is_movable_bank_holiday(&self, date: NaiveDate) -> bool {
        // Good Friday
        if let Some(gf) = self.good_friday
            && date == gf
        {
            return true;
        }

        // Easter Monday (not Scotland)
        if !matches!(self.region, UkRegion::Scotland)
            && let Some(em) = self.easter_monday
            && date == em
        {
            return true;
        }

        // Early May Bank Holiday (first Monday in May)
        if let Some(early_may) = nth_weekday_of_month(date.year(), 5, Weekday::Mon, 1)
            && date == early_may
        {
            return true;
        }

        // Spring Bank Holiday (last Monday in May)
        if let Some(spring) = last_weekday_of_month(date.year(), 5, Weekday::Mon)
            && date == spring
        {
            return true;
        }

        // Summer Bank Holiday
        let summer = if matches!(self.region, UkRegion::Scotland) {
            // Scotland: first Monday in August
            nth_weekday_of_month(date.year(), 8, Weekday::Mon, 1)
        } else {
            // England/Wales/NI: last Monday in August
            last_weekday_of_month(date.year(), 8, Weekday::Mon)
        };
        if let Some(summer_date) = summer
            && date == summer_date
        {
            return true;
        }

        false
    }

    /// Checks if date is a substitute bank holiday.
    fn is_substitute_day(&self, date: NaiveDate) -> bool {
        // When a bank holiday falls on a weekend, the following Monday is a substitute
        // (or Tuesday if Monday is already a holiday)
        let fixed_holidays = [
            (1, 1),   // New Year's Day
            (12, 25), // Christmas
            (12, 26), // Boxing Day
        ];

        for (month, day) in fixed_holidays {
            if let Some(holiday) = NaiveDate::from_ymd_opt(date.year(), month, day) {
                match holiday.weekday() {
                    Weekday::Sat => {
                        // Saturday -> Monday
                        let monday = holiday + chrono::Duration::days(2);
                        if date == monday {
                            return true;
                        }
                    }
                    Weekday::Sun => {
                        // Sunday -> Monday (or Tuesday if Christmas/Boxing Day overlap)
                        let monday = holiday + chrono::Duration::days(1);
                        if date == monday {
                            return true;
                        }
                        // Special case: Christmas/Boxing Day
                        if month == 12 && day == 25 {
                            // If Christmas is Sunday, Boxing Day (Mon) is also holiday,
                            // so Christmas substitute is Tuesday
                            if date == holiday + chrono::Duration::days(2) {
                                return true;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Region-specific substitute days
        if self.region.has_st_patricks_day()
            && let Some(st_pat) = NaiveDate::from_ymd_opt(date.year(), 3, 17)
            && is_weekend_substitute(st_pat, date)
        {
            return true;
        }

        if self.region.has_battle_of_boyne()
            && let Some(boyne) = NaiveDate::from_ymd_opt(date.year(), 7, 12)
            && is_weekend_substitute(boyne, date)
        {
            return true;
        }

        if self.region.has_st_andrews_day()
            && let Some(st_and) = NaiveDate::from_ymd_opt(date.year(), 11, 30)
            && is_weekend_substitute(st_and, date)
        {
            return true;
        }

        false
    }

    /// Checks if a date is a working day (not weekend or bank holiday).
    #[must_use]
    pub fn is_working_day(&self, date: NaiveDate) -> bool {
        if date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun {
            return false;
        }
        !self.is_bank_holiday(date)
    }

    /// Adds working days to a date, skipping weekends and bank holidays.
    #[must_use]
    pub fn add_working_days(&self, start: NaiveDate, days: i32) -> NaiveDate {
        if days == 0 {
            return start;
        }

        let mut current = start;
        let mut remaining = days;
        let increment = if days > 0 { 1 } else { -1 };

        while remaining != 0 {
            current = if increment > 0 {
                current.succ_opt().unwrap_or(current)
            } else {
                current.pred_opt().unwrap_or(current)
            };

            if self.is_working_day(current) {
                remaining -= increment;
            }
        }

        current
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

    /// Returns the name of a bank holiday on the given date, if any.
    #[must_use]
    pub fn bank_holiday_name(&self, date: NaiveDate) -> Option<&'static str> {
        // Fixed holidays
        match (date.month(), date.day()) {
            (1, 1) => return Some("New Year's Day"),
            (1, 2) if self.region.has_second_january() => return Some("2nd January"),
            (3, 17) if self.region.has_st_patricks_day() => return Some("St Patrick's Day"),
            (7, 12) if self.region.has_battle_of_boyne() => return Some("Battle of the Boyne"),
            (11, 30) if self.region.has_st_andrews_day() => return Some("St Andrew's Day"),
            (12, 25) => return Some("Christmas Day"),
            (12, 26) => return Some("Boxing Day"),
            _ => {}
        }

        // Movable holidays
        if let Some(gf) = self.good_friday
            && date == gf
        {
            return Some("Good Friday");
        }

        if let Some(em) = self.easter_monday
            && date == em
            && !matches!(self.region, UkRegion::Scotland)
        {
            return Some("Easter Monday");
        }

        if let Some(early_may) = nth_weekday_of_month(date.year(), 5, Weekday::Mon, 1)
            && date == early_may
        {
            return Some("Early May Bank Holiday");
        }

        if let Some(spring) = last_weekday_of_month(date.year(), 5, Weekday::Mon)
            && date == spring
        {
            return Some("Spring Bank Holiday");
        }

        let summer = if matches!(self.region, UkRegion::Scotland) {
            nth_weekday_of_month(date.year(), 8, Weekday::Mon, 1)
        } else {
            last_weekday_of_month(date.year(), 8, Weekday::Mon)
        };
        if let Some(summer_date) = summer
            && date == summer_date
        {
            return Some("Summer Bank Holiday");
        }

        // Substitute days
        if self.is_substitute_day(date) {
            return Some("Bank Holiday (substitute day)");
        }

        None
    }
}

/// Check if a date is a substitute day for a weekend holiday.
fn is_weekend_substitute(holiday: NaiveDate, check_date: NaiveDate) -> bool {
    match holiday.weekday() {
        Weekday::Sat => check_date == holiday + chrono::Duration::days(2), // Monday
        Weekday::Sun => check_date == holiday + chrono::Duration::days(1), // Monday
        _ => false,
    }
}

/// Calculates Easter Sunday using the Anonymous Gregorian algorithm.
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

/// Finds the nth occurrence of a weekday in a given month.
fn nth_weekday_of_month(year: i32, month: u32, weekday: Weekday, nth: u32) -> Option<NaiveDate> {
    let first_of_month = NaiveDate::from_ymd_opt(year, month, 1)?;
    let first_weekday = first_of_month.weekday();

    let days_until =
        (weekday.num_days_from_monday() as i32 - first_weekday.num_days_from_monday() as i32 + 7)
            % 7;
    let first_occurrence = first_of_month + chrono::Duration::days(days_until as i64);
    let target = first_occurrence + chrono::Duration::weeks((nth - 1) as i64);

    if target.month() == month {
        Some(target)
    } else {
        None
    }
}

/// Finds the last occurrence of a weekday in a given month.
fn last_weekday_of_month(year: i32, month: u32, weekday: Weekday) -> Option<NaiveDate> {
    let last_of_month = if month == 12 {
        NaiveDate::from_ymd_opt(year, 12, 31)?
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)? - chrono::Duration::days(1)
    };

    let last_weekday_num = last_of_month.weekday().num_days_from_monday();
    let target_weekday_num = weekday.num_days_from_monday();

    let days_back = (last_weekday_num as i32 - target_weekday_num as i32 + 7) % 7;
    Some(last_of_month - chrono::Duration::days(days_back as i64))
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Checks if a date is a UK bank holiday (England & Wales).
///
/// # Example
///
/// ```rust
/// use legalis_uk::common::is_bank_holiday;
/// use chrono::NaiveDate;
///
/// let christmas = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
/// assert!(is_bank_holiday(christmas));
/// ```
#[must_use]
pub fn is_bank_holiday(date: NaiveDate) -> bool {
    UkLegalCalendar::for_year(date.year()).is_bank_holiday(date)
}

/// Checks if a date is a UK working day (England & Wales).
#[must_use]
pub fn is_working_day(date: NaiveDate) -> bool {
    UkLegalCalendar::for_year(date.year()).is_working_day(date)
}

/// Calculates a legal deadline by adding working days (England & Wales).
///
/// # Example
///
/// ```rust
/// use legalis_uk::common::calculate_legal_deadline;
/// use chrono::NaiveDate;
///
/// let start = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
/// let deadline = calculate_legal_deadline(start, 14);
/// // Deadline is 14 working days later, skipping weekends and bank holidays
/// ```
#[must_use]
pub fn calculate_legal_deadline(start: NaiveDate, business_days: i32) -> NaiveDate {
    UkLegalCalendar::for_year(start.year()).add_working_days(start, business_days)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Fixed holiday tests
    // ========================================================================

    #[test]
    fn test_new_year_is_bank_holiday() {
        let calendar = UkLegalCalendar::for_year(2025);
        let new_year = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(calendar.is_bank_holiday(new_year));
        assert_eq!(calendar.bank_holiday_name(new_year), Some("New Year's Day"));
    }

    #[test]
    fn test_christmas_is_bank_holiday() {
        let calendar = UkLegalCalendar::for_year(2025);
        let christmas = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        assert!(calendar.is_bank_holiday(christmas));
        assert_eq!(calendar.bank_holiday_name(christmas), Some("Christmas Day"));
    }

    #[test]
    fn test_boxing_day_is_bank_holiday() {
        let calendar = UkLegalCalendar::for_year(2025);
        let boxing_day = NaiveDate::from_ymd_opt(2025, 12, 26).unwrap();
        assert!(calendar.is_bank_holiday(boxing_day));
        assert_eq!(calendar.bank_holiday_name(boxing_day), Some("Boxing Day"));
    }

    // ========================================================================
    // Easter holiday tests
    // ========================================================================

    #[test]
    fn test_good_friday_2025() {
        let calendar = UkLegalCalendar::for_year(2025);
        // Easter 2025 is April 20, so Good Friday is April 18
        let good_friday = NaiveDate::from_ymd_opt(2025, 4, 18).unwrap();
        assert!(calendar.is_bank_holiday(good_friday));
        assert_eq!(calendar.bank_holiday_name(good_friday), Some("Good Friday"));
    }

    #[test]
    fn test_easter_monday_2025() {
        let calendar = UkLegalCalendar::for_year(2025);
        // Easter 2025 is April 20, so Easter Monday is April 21
        let easter_monday = NaiveDate::from_ymd_opt(2025, 4, 21).unwrap();
        assert!(calendar.is_bank_holiday(easter_monday));
        assert_eq!(
            calendar.bank_holiday_name(easter_monday),
            Some("Easter Monday")
        );
    }

    #[test]
    fn test_easter_monday_not_in_scotland() {
        let calendar = UkLegalCalendar::for_year_and_region(2025, UkRegion::Scotland);
        let easter_monday = NaiveDate::from_ymd_opt(2025, 4, 21).unwrap();
        // Scotland doesn't have Easter Monday as a bank holiday
        assert!(!calendar.is_bank_holiday(easter_monday));
    }

    // ========================================================================
    // Monday bank holiday tests
    // ========================================================================

    #[test]
    fn test_early_may_bank_holiday_2025() {
        let calendar = UkLegalCalendar::for_year(2025);
        // First Monday in May 2025 is May 5
        let early_may = NaiveDate::from_ymd_opt(2025, 5, 5).unwrap();
        assert!(calendar.is_bank_holiday(early_may));
        assert_eq!(
            calendar.bank_holiday_name(early_may),
            Some("Early May Bank Holiday")
        );
    }

    #[test]
    fn test_spring_bank_holiday_2025() {
        let calendar = UkLegalCalendar::for_year(2025);
        // Last Monday in May 2025 is May 26
        let spring = NaiveDate::from_ymd_opt(2025, 5, 26).unwrap();
        assert!(calendar.is_bank_holiday(spring));
        assert_eq!(
            calendar.bank_holiday_name(spring),
            Some("Spring Bank Holiday")
        );
    }

    #[test]
    fn test_summer_bank_holiday_england_2025() {
        let calendar = UkLegalCalendar::for_year(2025);
        // Last Monday in August 2025 is August 25
        let summer = NaiveDate::from_ymd_opt(2025, 8, 25).unwrap();
        assert!(calendar.is_bank_holiday(summer));
        assert_eq!(
            calendar.bank_holiday_name(summer),
            Some("Summer Bank Holiday")
        );
    }

    #[test]
    fn test_summer_bank_holiday_scotland_2025() {
        let calendar = UkLegalCalendar::for_year_and_region(2025, UkRegion::Scotland);
        // First Monday in August 2025 is August 4
        let summer = NaiveDate::from_ymd_opt(2025, 8, 4).unwrap();
        assert!(calendar.is_bank_holiday(summer));
        assert_eq!(
            calendar.bank_holiday_name(summer),
            Some("Summer Bank Holiday")
        );
    }

    // ========================================================================
    // Region-specific holiday tests
    // ========================================================================

    #[test]
    fn test_st_patricks_day_northern_ireland() {
        let calendar = UkLegalCalendar::for_year_and_region(2025, UkRegion::NorthernIreland);
        let st_pat = NaiveDate::from_ymd_opt(2025, 3, 17).unwrap();
        assert!(calendar.is_bank_holiday(st_pat));
        assert_eq!(calendar.bank_holiday_name(st_pat), Some("St Patrick's Day"));
    }

    #[test]
    fn test_st_patricks_day_not_in_england() {
        let calendar = UkLegalCalendar::for_year(2025);
        let st_pat = NaiveDate::from_ymd_opt(2025, 3, 17).unwrap();
        // March 17, 2025 is Monday - should be working day in England
        assert!(calendar.is_working_day(st_pat));
    }

    #[test]
    fn test_st_andrews_day_scotland() {
        let calendar = UkLegalCalendar::for_year_and_region(2025, UkRegion::Scotland);
        // Nov 30, 2025 is Sunday, so substitute is Monday Dec 1
        let st_and = NaiveDate::from_ymd_opt(2025, 11, 30).unwrap();
        assert!(calendar.is_bank_holiday(st_and));
    }

    #[test]
    fn test_second_january_scotland() {
        let calendar = UkLegalCalendar::for_year_and_region(2025, UkRegion::Scotland);
        let jan_2 = NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();
        assert!(calendar.is_bank_holiday(jan_2));
        assert_eq!(calendar.bank_holiday_name(jan_2), Some("2nd January"));
    }

    // ========================================================================
    // Substitute day tests
    // ========================================================================

    #[test]
    fn test_substitute_day_new_year_on_saturday() {
        // In 2028, New Year's Day is Saturday, substitute is Monday Jan 3
        let calendar = UkLegalCalendar::for_year(2028);
        let substitute = NaiveDate::from_ymd_opt(2028, 1, 3).unwrap();
        assert!(calendar.is_bank_holiday(substitute));
    }

    // ========================================================================
    // Working day tests
    // ========================================================================

    #[test]
    fn test_regular_weekday_is_working_day() {
        let calendar = UkLegalCalendar::for_year(2025);
        let tuesday = NaiveDate::from_ymd_opt(2025, 6, 10).unwrap();
        assert!(calendar.is_working_day(tuesday));
    }

    #[test]
    fn test_weekend_not_working_day() {
        let calendar = UkLegalCalendar::for_year(2025);
        let saturday = NaiveDate::from_ymd_opt(2025, 6, 14).unwrap();
        let sunday = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
        assert!(!calendar.is_working_day(saturday));
        assert!(!calendar.is_working_day(sunday));
    }

    #[test]
    fn test_add_working_days() {
        let calendar = UkLegalCalendar::for_year(2025);
        let monday = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
        let result = calendar.add_working_days(monday, 5);
        // Mon 6/2 -> Tue 6/3 -> Wed 6/4 -> Thu 6/5 -> Fri 6/6 -> Mon 6/9
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 6, 9).unwrap());
    }

    #[test]
    fn test_working_days_between() {
        let calendar = UkLegalCalendar::for_year(2025);
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 6, 13).unwrap(); // Friday
        // Working days: Mon-Fri week 1 (5) + Mon-Thu week 2 (4) = 9
        assert_eq!(calendar.working_days_between(start, end), 9);
    }

    // ========================================================================
    // Convenience function tests
    // ========================================================================

    #[test]
    fn test_is_bank_holiday_function() {
        let christmas = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        assert!(is_bank_holiday(christmas));

        let regular_day = NaiveDate::from_ymd_opt(2025, 6, 10).unwrap();
        assert!(!is_bank_holiday(regular_day));
    }

    #[test]
    fn test_is_working_day_function() {
        let monday = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
        assert!(is_working_day(monday));

        let saturday = NaiveDate::from_ymd_opt(2025, 6, 7).unwrap();
        assert!(!is_working_day(saturday));
    }

    // ========================================================================
    // Region tests
    // ========================================================================

    #[test]
    fn test_region_names() {
        assert_eq!(UkRegion::EnglandWales.name(), "England & Wales");
        assert_eq!(UkRegion::Scotland.name(), "Scotland");
        assert_eq!(UkRegion::NorthernIreland.name(), "Northern Ireland");
    }

    #[test]
    fn test_region_codes() {
        assert_eq!(UkRegion::EnglandWales.code(), "GB-EAW");
        assert_eq!(UkRegion::Scotland.code(), "GB-SCT");
        assert_eq!(UkRegion::NorthernIreland.code(), "GB-NIR");
    }

    // ========================================================================
    // Easter calculation tests
    // ========================================================================

    #[test]
    fn test_easter_calculation() {
        assert_eq!(calculate_easter(2024), NaiveDate::from_ymd_opt(2024, 3, 31));
        assert_eq!(calculate_easter(2025), NaiveDate::from_ymd_opt(2025, 4, 20));
        assert_eq!(calculate_easter(2026), NaiveDate::from_ymd_opt(2026, 4, 5));
    }

    // ========================================================================
    // Monday calculation tests
    // ========================================================================

    #[test]
    fn test_nth_weekday_of_month() {
        // First Monday in May 2025
        let result = nth_weekday_of_month(2025, 5, Weekday::Mon, 1);
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 5, 5));
    }

    #[test]
    fn test_last_weekday_of_month() {
        // Last Monday in May 2025
        let result = last_weekday_of_month(2025, 5, Weekday::Mon);
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 5, 26));

        // Last Monday in August 2025
        let result = last_weekday_of_month(2025, 8, Weekday::Mon);
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 8, 25));
    }
}
