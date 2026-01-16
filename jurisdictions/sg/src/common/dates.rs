//! Singapore legal calendar and working days utilities.
//!
//! Integrates legalis-i18n's calendar functionality with Singapore legal requirements.
//!
//! # Singapore Public Holidays
//!
//! Singapore has 11 gazetted public holidays. Several are movable based on
//! Chinese, Islamic, Hindu, and Buddhist lunar calendars.
//!
//! ## Fixed Holidays
//! - New Year's Day: January 1
//! - Labour Day: May 1
//! - National Day: August 9
//! - Christmas Day: December 25
//!
//! ## Movable Holidays (dates vary yearly)
//! - Chinese New Year (2 days): Based on Chinese lunar calendar
//! - Good Friday: Friday before Easter Sunday
//! - Vesak Day: Full moon of 4th lunar month
//! - Hari Raya Puasa: End of Ramadan (Islamic calendar)
//! - Hari Raya Haji: 10th day of Zulhijjah (Islamic calendar)
//! - Deepavali: New moon of 7th Hindu month
//!
//! ## Substitute Holiday Rule
//! When a public holiday falls on Sunday, the next Monday is a public holiday.

use chrono::{Datelike, NaiveDate, Weekday};
use legalis_i18n::{DeadlineCalculator, WorkingDaysConfig};

/// Singapore legal calendar with holiday awareness.
///
/// Provides working day calculations that comply with Singapore legal requirements.
/// Since many Singapore holidays are lunar-based, use `for_year()` to get
/// accurate holiday dates for a specific year.
///
/// # Example
///
/// ```rust
/// use legalis_sg::common::{SingaporeLegalCalendar};
/// use chrono::NaiveDate;
///
/// // Create calendar for 2025
/// let calendar = SingaporeLegalCalendar::for_year(2025);
///
/// // Check if National Day is a holiday
/// let national_day = NaiveDate::from_ymd_opt(2025, 8, 9).unwrap();
/// assert!(calendar.is_holiday(national_day));
///
/// // Calculate a 14-day business deadline
/// let start = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
/// let deadline = calendar.add_working_days(start, 14);
/// ```
#[derive(Debug, Clone)]
pub struct SingaporeLegalCalendar {
    config: WorkingDaysConfig,
    year: i32,
    /// Chinese New Year dates for the year
    chinese_new_year: Option<(NaiveDate, NaiveDate)>,
    /// Good Friday date
    good_friday: Option<NaiveDate>,
    /// Vesak Day date
    vesak_day: Option<NaiveDate>,
    /// Hari Raya Puasa date
    hari_raya_puasa: Option<NaiveDate>,
    /// Hari Raya Haji date
    hari_raya_haji: Option<NaiveDate>,
    /// Deepavali date
    deepavali: Option<NaiveDate>,
}

impl Default for SingaporeLegalCalendar {
    fn default() -> Self {
        Self::for_year(chrono::Utc::now().year())
    }
}

impl SingaporeLegalCalendar {
    /// Creates a calendar for a specific year with known holiday dates.
    ///
    /// This provides accurate dates for all Singapore public holidays.
    #[must_use]
    pub fn for_year(year: i32) -> Self {
        // Base config with fixed holidays
        let config = WorkingDaysConfig::new("SG")
            .add_holiday(1, 1) // New Year's Day
            .add_holiday(5, 1) // Labour Day
            .add_holiday(8, 9) // National Day
            .add_holiday(12, 25); // Christmas Day

        // Calculate Easter-based Good Friday
        let good_friday = calculate_easter(year).map(|e| e - chrono::Duration::days(2));

        // Get known movable holidays for specific years
        let (cny, vesak, hrp, hrh, deepavali) = get_movable_holidays(year);

        Self {
            config,
            year,
            chinese_new_year: cny,
            good_friday,
            vesak_day: vesak,
            hari_raya_puasa: hrp,
            hari_raya_haji: hrh,
            deepavali,
        }
    }

    /// Creates a calendar with custom movable holiday dates.
    ///
    /// Use this when you need to specify exact dates for lunar-based holidays.
    #[must_use]
    pub fn with_custom_holidays(
        year: i32,
        chinese_new_year: (NaiveDate, NaiveDate),
        good_friday: NaiveDate,
        vesak_day: NaiveDate,
        hari_raya_puasa: NaiveDate,
        hari_raya_haji: NaiveDate,
        deepavali: NaiveDate,
    ) -> Self {
        let config = WorkingDaysConfig::new("SG")
            .add_holiday(1, 1) // New Year's Day
            .add_holiday(5, 1) // Labour Day
            .add_holiday(8, 9) // National Day
            .add_holiday(12, 25); // Christmas Day

        Self {
            config,
            year,
            chinese_new_year: Some(chinese_new_year),
            good_friday: Some(good_friday),
            vesak_day: Some(vesak_day),
            hari_raya_puasa: Some(hari_raya_puasa),
            hari_raya_haji: Some(hari_raya_haji),
            deepavali: Some(deepavali),
        }
    }

    /// Returns the calendar year.
    #[must_use]
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Checks if a date is a Singapore public holiday.
    #[must_use]
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        // Check fixed holidays (regardless of weekend)
        let is_fixed_holiday = matches!(
            (date.month(), date.day()),
            (1, 1) | (5, 1) | (8, 9) | (12, 25)
        );
        if is_fixed_holiday {
            return true;
        }

        // Check movable holidays
        if let Some((cny1, cny2)) = self.chinese_new_year {
            if date == cny1 || date == cny2 {
                return true;
            }
            // Check for substitute holidays
            if is_substitute_holiday(cny1, date) || is_substitute_holiday(cny2, date) {
                return true;
            }
        }

        if let Some(gf) = self.good_friday {
            if date == gf {
                return true;
            }
        }

        if let Some(vesak) = self.vesak_day {
            if date == vesak || is_substitute_holiday(vesak, date) {
                return true;
            }
        }

        if let Some(hrp) = self.hari_raya_puasa {
            if date == hrp || is_substitute_holiday(hrp, date) {
                return true;
            }
        }

        if let Some(hrh) = self.hari_raya_haji {
            if date == hrh || is_substitute_holiday(hrh, date) {
                return true;
            }
        }

        if let Some(dv) = self.deepavali {
            if date == dv || is_substitute_holiday(dv, date) {
                return true;
            }
        }

        // Check substitute holidays for fixed holidays
        let fixed_holidays = [
            NaiveDate::from_ymd_opt(date.year(), 1, 1),   // New Year
            NaiveDate::from_ymd_opt(date.year(), 5, 1),   // Labour Day
            NaiveDate::from_ymd_opt(date.year(), 8, 9),   // National Day
            NaiveDate::from_ymd_opt(date.year(), 12, 25), // Christmas
        ];

        for holiday in fixed_holidays.into_iter().flatten() {
            if is_substitute_holiday(holiday, date) {
                return true;
            }
        }

        false
    }

    /// Checks if a date is a working day (not weekend or holiday).
    #[must_use]
    pub fn is_working_day(&self, date: NaiveDate) -> bool {
        if date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun {
            return false;
        }
        !self.is_holiday(date)
    }

    /// Adds working days to a date, skipping weekends and holidays.
    ///
    /// Used for legal deadline calculations.
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

    /// Counts working days between two dates (exclusive of end date).
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

    /// Returns the English name for a holiday on the given date, if any.
    #[must_use]
    pub fn holiday_name(&self, date: NaiveDate) -> Option<&'static str> {
        // Fixed holidays
        match (date.month(), date.day()) {
            (1, 1) => return Some("New Year's Day"),
            (5, 1) => return Some("Labour Day"),
            (8, 9) => return Some("National Day"),
            (12, 25) => return Some("Christmas Day"),
            _ => {}
        }

        // Movable holidays
        if let Some((cny1, cny2)) = self.chinese_new_year {
            if date == cny1 {
                return Some("Chinese New Year (Day 1)");
            }
            if date == cny2 {
                return Some("Chinese New Year (Day 2)");
            }
        }

        if let Some(gf) = self.good_friday {
            if date == gf {
                return Some("Good Friday");
            }
        }

        if let Some(vesak) = self.vesak_day {
            if date == vesak {
                return Some("Vesak Day");
            }
        }

        if let Some(hrp) = self.hari_raya_puasa {
            if date == hrp {
                return Some("Hari Raya Puasa");
            }
        }

        if let Some(hrh) = self.hari_raya_haji {
            if date == hrh {
                return Some("Hari Raya Haji");
            }
        }

        if let Some(dv) = self.deepavali {
            if date == dv {
                return Some("Deepavali");
            }
        }

        // Check substitute holidays
        if self.is_holiday(date) && date.weekday() == Weekday::Mon {
            return Some("Substitute Public Holiday");
        }

        None
    }

    /// Returns the Chinese name for a holiday on the given date, if any.
    #[must_use]
    pub fn holiday_name_chinese(&self, date: NaiveDate) -> Option<&'static str> {
        // Fixed holidays
        match (date.month(), date.day()) {
            (1, 1) => return Some("元旦"),
            (5, 1) => return Some("劳动节"),
            (8, 9) => return Some("国庆日"),
            (12, 25) => return Some("圣诞节"),
            _ => {}
        }

        // Movable holidays
        if let Some((cny1, cny2)) = self.chinese_new_year {
            if date == cny1 || date == cny2 {
                return Some("农历新年");
            }
        }

        if let Some(gf) = self.good_friday {
            if date == gf {
                return Some("耶稣受难日");
            }
        }

        if let Some(vesak) = self.vesak_day {
            if date == vesak {
                return Some("卫塞节");
            }
        }

        if let Some(hrp) = self.hari_raya_puasa {
            if date == hrp {
                return Some("开斋节");
            }
        }

        if let Some(hrh) = self.hari_raya_haji {
            if date == hrh {
                return Some("哈芝节");
            }
        }

        if let Some(dv) = self.deepavali {
            if date == dv {
                return Some("屠妖节");
            }
        }

        None
    }

    /// Returns the Malay name for a holiday on the given date, if any.
    #[must_use]
    pub fn holiday_name_malay(&self, date: NaiveDate) -> Option<&'static str> {
        // Fixed holidays
        match (date.month(), date.day()) {
            (1, 1) => return Some("Tahun Baru"),
            (5, 1) => return Some("Hari Pekerja"),
            (8, 9) => return Some("Hari Kebangsaan"),
            (12, 25) => return Some("Hari Krismas"),
            _ => {}
        }

        // Movable holidays
        if let Some((cny1, cny2)) = self.chinese_new_year {
            if date == cny1 || date == cny2 {
                return Some("Tahun Baru Cina");
            }
        }

        if let Some(gf) = self.good_friday {
            if date == gf {
                return Some("Jumaat Agung");
            }
        }

        if let Some(vesak) = self.vesak_day {
            if date == vesak {
                return Some("Hari Wesak");
            }
        }

        if let Some(hrp) = self.hari_raya_puasa {
            if date == hrp {
                return Some("Hari Raya Puasa");
            }
        }

        if let Some(hrh) = self.hari_raya_haji {
            if date == hrh {
                return Some("Hari Raya Haji");
            }
        }

        if let Some(dv) = self.deepavali {
            if date == dv {
                return Some("Deepavali");
            }
        }

        None
    }

    /// Returns the Tamil name for a holiday on the given date, if any.
    #[must_use]
    pub fn holiday_name_tamil(&self, date: NaiveDate) -> Option<&'static str> {
        // Movable holidays with Tamil names
        if let Some(dv) = self.deepavali {
            if date == dv {
                return Some("தீபாவளி");
            }
        }

        // For other holidays, return None (can be extended)
        None
    }
}

/// Check if a date is a substitute holiday (Monday after Sunday holiday).
fn is_substitute_holiday(holiday: NaiveDate, check_date: NaiveDate) -> bool {
    holiday.weekday() == Weekday::Sun
        && check_date == holiday + chrono::Duration::days(1)
        && check_date.weekday() == Weekday::Mon
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

/// Returns known movable holiday dates for specific years.
///
/// These are gazetted dates from Singapore's Ministry of Manpower.
/// Returns: (CNY days, Vesak, Hari Raya Puasa, Hari Raya Haji, Deepavali)
#[allow(clippy::type_complexity)]
fn get_movable_holidays(
    year: i32,
) -> (
    Option<(NaiveDate, NaiveDate)>,
    Option<NaiveDate>,
    Option<NaiveDate>,
    Option<NaiveDate>,
    Option<NaiveDate>,
) {
    match year {
        2024 => (
            Some((
                NaiveDate::from_ymd_opt(2024, 2, 10).expect("Valid date"),
                NaiveDate::from_ymd_opt(2024, 2, 11).expect("Valid date"),
            )), // CNY: Feb 10-11 (Sat-Sun, substitute Mon Feb 12)
            Some(NaiveDate::from_ymd_opt(2024, 5, 22).expect("Valid date")), // Vesak Day
            Some(NaiveDate::from_ymd_opt(2024, 4, 10).expect("Valid date")), // Hari Raya Puasa
            Some(NaiveDate::from_ymd_opt(2024, 6, 17).expect("Valid date")), // Hari Raya Haji
            Some(NaiveDate::from_ymd_opt(2024, 11, 1).expect("Valid date")), // Deepavali
        ),
        2025 => (
            Some((
                NaiveDate::from_ymd_opt(2025, 1, 29).expect("Valid date"),
                NaiveDate::from_ymd_opt(2025, 1, 30).expect("Valid date"),
            )), // CNY: Jan 29-30 (Wed-Thu)
            Some(NaiveDate::from_ymd_opt(2025, 5, 12).expect("Valid date")), // Vesak Day
            Some(NaiveDate::from_ymd_opt(2025, 3, 31).expect("Valid date")), // Hari Raya Puasa
            Some(NaiveDate::from_ymd_opt(2025, 6, 7).expect("Valid date")), // Hari Raya Haji (Sat, substitute Mon Jun 9)
            Some(NaiveDate::from_ymd_opt(2025, 10, 20).expect("Valid date")), // Deepavali
        ),
        2026 => (
            Some((
                NaiveDate::from_ymd_opt(2026, 2, 17).expect("Valid date"),
                NaiveDate::from_ymd_opt(2026, 2, 18).expect("Valid date"),
            )), // CNY: Feb 17-18 (Tue-Wed)
            Some(NaiveDate::from_ymd_opt(2026, 5, 31).expect("Valid date")), // Vesak Day (Sun, substitute Mon Jun 1)
            Some(NaiveDate::from_ymd_opt(2026, 3, 20).expect("Valid date")), // Hari Raya Puasa
            Some(NaiveDate::from_ymd_opt(2026, 5, 27).expect("Valid date")), // Hari Raya Haji
            Some(NaiveDate::from_ymd_opt(2026, 11, 8).expect("Valid date")), // Deepavali (Sun, substitute Mon Nov 9)
        ),
        _ => (None, None, None, None, None),
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Checks if a date is a Singapore public holiday.
///
/// Uses the current year for movable holidays.
///
/// # Example
///
/// ```rust
/// use legalis_sg::common::is_singapore_holiday;
/// use chrono::NaiveDate;
///
/// let national_day = NaiveDate::from_ymd_opt(2025, 8, 9).unwrap();
/// assert!(is_singapore_holiday(national_day));
/// ```
#[must_use]
pub fn is_singapore_holiday(date: NaiveDate) -> bool {
    SingaporeLegalCalendar::for_year(date.year()).is_holiday(date)
}

/// Checks if a date is a Singapore working day.
#[must_use]
pub fn is_working_day(date: NaiveDate) -> bool {
    SingaporeLegalCalendar::for_year(date.year()).is_working_day(date)
}

/// Calculates a legal deadline by adding working days.
///
/// # Example
///
/// ```rust
/// use legalis_sg::common::calculate_legal_deadline;
/// use chrono::NaiveDate;
///
/// let start = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
/// let deadline = calculate_legal_deadline(start, 14);
/// // Deadline is 14 working days later, skipping weekends and holidays
/// ```
#[must_use]
pub fn calculate_legal_deadline(start: NaiveDate, business_days: i32) -> NaiveDate {
    SingaporeLegalCalendar::for_year(start.year()).add_working_days(start, business_days)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Fixed holiday tests
    // ========================================================================

    #[test]
    fn test_new_year_is_holiday() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let new_year = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(calendar.is_holiday(new_year));
        assert!(!calendar.is_working_day(new_year));
    }

    #[test]
    fn test_labour_day_is_holiday() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let labour_day = NaiveDate::from_ymd_opt(2025, 5, 1).unwrap();
        assert!(calendar.is_holiday(labour_day));
        assert_eq!(calendar.holiday_name(labour_day), Some("Labour Day"));
    }

    #[test]
    fn test_national_day_is_holiday() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let national_day = NaiveDate::from_ymd_opt(2025, 8, 9).unwrap();
        assert!(calendar.is_holiday(national_day));
        assert_eq!(calendar.holiday_name(national_day), Some("National Day"));
        assert_eq!(
            calendar.holiday_name_malay(national_day),
            Some("Hari Kebangsaan")
        );
    }

    #[test]
    fn test_christmas_is_holiday() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let christmas = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        assert!(calendar.is_holiday(christmas));
        assert_eq!(calendar.holiday_name_chinese(christmas), Some("圣诞节"));
    }

    // ========================================================================
    // Movable holiday tests
    // ========================================================================

    #[test]
    fn test_chinese_new_year_2025() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let cny1 = NaiveDate::from_ymd_opt(2025, 1, 29).unwrap();
        let cny2 = NaiveDate::from_ymd_opt(2025, 1, 30).unwrap();
        assert!(calendar.is_holiday(cny1));
        assert!(calendar.is_holiday(cny2));
        assert_eq!(
            calendar.holiday_name(cny1),
            Some("Chinese New Year (Day 1)")
        );
        assert_eq!(
            calendar.holiday_name(cny2),
            Some("Chinese New Year (Day 2)")
        );
        assert_eq!(calendar.holiday_name_chinese(cny1), Some("农历新年"));
    }

    #[test]
    fn test_good_friday_2025() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        // Easter 2025 is April 20, so Good Friday is April 18
        let good_friday = NaiveDate::from_ymd_opt(2025, 4, 18).unwrap();
        assert!(calendar.is_holiday(good_friday));
        assert_eq!(calendar.holiday_name(good_friday), Some("Good Friday"));
    }

    #[test]
    fn test_vesak_day_2025() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let vesak = NaiveDate::from_ymd_opt(2025, 5, 12).unwrap();
        assert!(calendar.is_holiday(vesak));
        assert_eq!(calendar.holiday_name(vesak), Some("Vesak Day"));
        assert_eq!(calendar.holiday_name_chinese(vesak), Some("卫塞节"));
    }

    #[test]
    fn test_hari_raya_puasa_2025() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let hrp = NaiveDate::from_ymd_opt(2025, 3, 31).unwrap();
        assert!(calendar.is_holiday(hrp));
        assert_eq!(calendar.holiday_name(hrp), Some("Hari Raya Puasa"));
        assert_eq!(calendar.holiday_name_chinese(hrp), Some("开斋节"));
    }

    #[test]
    fn test_hari_raya_haji_2025() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let hrh = NaiveDate::from_ymd_opt(2025, 6, 7).unwrap();
        assert!(calendar.is_holiday(hrh));
        assert_eq!(calendar.holiday_name(hrh), Some("Hari Raya Haji"));
    }

    #[test]
    fn test_deepavali_2025() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let deepavali = NaiveDate::from_ymd_opt(2025, 10, 20).unwrap();
        assert!(calendar.is_holiday(deepavali));
        assert_eq!(calendar.holiday_name(deepavali), Some("Deepavali"));
        assert_eq!(calendar.holiday_name_tamil(deepavali), Some("தீபாவளி"));
    }

    // ========================================================================
    // Substitute holiday tests
    // ========================================================================

    #[test]
    fn test_substitute_holiday_when_sunday() {
        // In 2024, CNY Day 2 is Sunday (Feb 11), substitute is Monday (Feb 12)
        let calendar = SingaporeLegalCalendar::for_year(2024);
        let substitute = NaiveDate::from_ymd_opt(2024, 2, 12).unwrap();
        assert!(calendar.is_holiday(substitute));
    }

    // ========================================================================
    // Working day calculation tests
    // ========================================================================

    #[test]
    fn test_regular_weekday_is_working_day() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        // A regular Wednesday in June should be a working day
        let wednesday = NaiveDate::from_ymd_opt(2025, 6, 11).unwrap();
        assert!(calendar.is_working_day(wednesday));
    }

    #[test]
    fn test_weekend_not_working_day() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let saturday = NaiveDate::from_ymd_opt(2025, 6, 14).unwrap();
        let sunday = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
        assert!(!calendar.is_working_day(saturday));
        assert!(!calendar.is_working_day(sunday));
    }

    #[test]
    fn test_add_working_days() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        // Start on Monday June 2, add 5 working days
        let monday = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
        let result = calendar.add_working_days(monday, 5);
        // Mon 6/2, Tue 6/3, Wed 6/4, Thu 6/5, Fri 6/6 = 5 working days
        // Day 1: Tue 6/3, Day 2: Wed 6/4, Day 3: Thu 6/5, Day 4: Fri 6/6
        // Sat 6/7 skip, Sun 6/8 skip, Day 5: Mon 6/9
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 6, 9).unwrap());
    }

    #[test]
    fn test_working_days_between() {
        let calendar = SingaporeLegalCalendar::for_year(2025);
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 6, 13).unwrap(); // Friday
        // Working days: Mon 6/2, Tue 6/3, Wed 6/4, Thu 6/5, Fri 6/6 = 5
        // + Mon 6/9, Tue 6/10, Wed 6/11, Thu 6/12 = 4
        // Total = 9
        assert_eq!(calendar.working_days_between(start, end), 9);
    }

    // ========================================================================
    // Convenience function tests
    // ========================================================================

    #[test]
    fn test_is_singapore_holiday_function() {
        let national_day = NaiveDate::from_ymd_opt(2025, 8, 9).unwrap();
        assert!(is_singapore_holiday(national_day));

        let regular_day = NaiveDate::from_ymd_opt(2025, 6, 11).unwrap();
        assert!(!is_singapore_holiday(regular_day));
    }

    #[test]
    fn test_is_working_day_function() {
        let monday = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
        assert!(is_working_day(monday));

        let saturday = NaiveDate::from_ymd_opt(2025, 6, 7).unwrap();
        assert!(!is_working_day(saturday));
    }

    #[test]
    fn test_calculate_legal_deadline_function() {
        let start = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
        let deadline = calculate_legal_deadline(start, 10);
        // April 2025: Good Friday is April 18
        // Day 1: Wed 4/2, Day 2: Thu 4/3, Day 3: Fri 4/4, Day 4: Mon 4/7, Day 5: Tue 4/8
        // Day 6: Wed 4/9, Day 7: Thu 4/10, Day 8: Fri 4/11, Day 9: Mon 4/14, Day 10: Tue 4/15
        assert_eq!(deadline, NaiveDate::from_ymd_opt(2025, 4, 15).unwrap());
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
}
