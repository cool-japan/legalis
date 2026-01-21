//! German legal calendar and working days utilities (Fristberechnung).
//!
//! Integrates legalis-i18n's calendar functionality with German legal requirements.
//!
//! # German Public Holidays (Feiertage)
//!
//! Germany has a mix of nationwide (bundesweite) and state-specific (länderspezifische) holidays.
//!
//! ## Federal Holidays (Bundesweite Feiertage)
//! - Neujahrstag (New Year's Day): January 1
//! - Karfreitag (Good Friday): Movable (Friday before Easter)
//! - Ostermontag (Easter Monday): Movable (Monday after Easter)
//! - Tag der Arbeit (Labor Day): May 1
//! - Christi Himmelfahrt (Ascension Day): Movable (39 days after Easter)
//! - Pfingstmontag (Whit Monday): Movable (50 days after Easter)
//! - Tag der Deutschen Einheit (German Unity Day): October 3
//! - 1. Weihnachtsfeiertag (Christmas Day): December 25
//! - 2. Weihnachtsfeiertag (Boxing Day): December 26
//!
//! ## State-Specific Holidays (Länderspezifische Feiertage)
//! - Heilige Drei Könige (Epiphany): January 6 - BY, BW, ST
//! - Fronleichnam (Corpus Christi): Movable - BY, BW, HE, NW, RP, SL, some SN/TH
//! - Mariä Himmelfahrt (Assumption): August 15 - BY (Catholic areas), SL
//! - Reformationstag (Reformation Day): October 31 - BB, HB, HH, MV, NI, SN, ST, SH, TH
//! - Allerheiligen (All Saints' Day): November 1 - BY, BW, NW, RP, SL
//! - Buß- und Bettag (Repentance Day): Movable - SN only

use chrono::{Datelike, NaiveDate, Weekday};
use legalis_i18n::{DeadlineCalculator, WorkingDaysConfig};
use serde::{Deserialize, Serialize};

/// German federal states (Bundesländer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GermanState {
    /// Baden-Württemberg (BW)
    BadenWuerttemberg,
    /// Bavaria (BY) / Bayern
    Bayern,
    /// Berlin (BE)
    Berlin,
    /// Brandenburg (BB)
    Brandenburg,
    /// Bremen (HB)
    Bremen,
    /// Hamburg (HH)
    Hamburg,
    /// Hesse (HE) / Hessen
    Hessen,
    /// Lower Saxony (NI) / Niedersachsen
    Niedersachsen,
    /// Mecklenburg-Vorpommern (MV)
    MecklenburgVorpommern,
    /// North Rhine-Westphalia (NW) / Nordrhein-Westfalen
    NordrheinWestfalen,
    /// Rhineland-Palatinate (RP) / Rheinland-Pfalz
    RheinlandPfalz,
    /// Saarland (SL)
    Saarland,
    /// Saxony (SN) / Sachsen
    Sachsen,
    /// Saxony-Anhalt (ST) / Sachsen-Anhalt
    SachsenAnhalt,
    /// Schleswig-Holstein (SH)
    SchleswigHolstein,
    /// Thuringia (TH) / Thüringen
    Thueringen,
}

impl GermanState {
    /// Returns the official abbreviation (Kurzcode) for the state.
    #[must_use]
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::BadenWuerttemberg => "BW",
            Self::Bayern => "BY",
            Self::Berlin => "BE",
            Self::Brandenburg => "BB",
            Self::Bremen => "HB",
            Self::Hamburg => "HH",
            Self::Hessen => "HE",
            Self::Niedersachsen => "NI",
            Self::MecklenburgVorpommern => "MV",
            Self::NordrheinWestfalen => "NW",
            Self::RheinlandPfalz => "RP",
            Self::Saarland => "SL",
            Self::Sachsen => "SN",
            Self::SachsenAnhalt => "ST",
            Self::SchleswigHolstein => "SH",
            Self::Thueringen => "TH",
        }
    }

    /// Returns the German name of the state.
    #[must_use]
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::BadenWuerttemberg => "Baden-Württemberg",
            Self::Bayern => "Bayern",
            Self::Berlin => "Berlin",
            Self::Brandenburg => "Brandenburg",
            Self::Bremen => "Bremen",
            Self::Hamburg => "Hamburg",
            Self::Hessen => "Hessen",
            Self::Niedersachsen => "Niedersachsen",
            Self::MecklenburgVorpommern => "Mecklenburg-Vorpommern",
            Self::NordrheinWestfalen => "Nordrhein-Westfalen",
            Self::RheinlandPfalz => "Rheinland-Pfalz",
            Self::Saarland => "Saarland",
            Self::Sachsen => "Sachsen",
            Self::SachsenAnhalt => "Sachsen-Anhalt",
            Self::SchleswigHolstein => "Schleswig-Holstein",
            Self::Thueringen => "Thüringen",
        }
    }

    /// Checks if Epiphany (Heilige Drei Könige) is a holiday in this state.
    #[must_use]
    pub fn has_epiphany(&self) -> bool {
        matches!(
            self,
            Self::BadenWuerttemberg | Self::Bayern | Self::SachsenAnhalt
        )
    }

    /// Checks if Corpus Christi (Fronleichnam) is a holiday in this state.
    #[must_use]
    pub fn has_corpus_christi(&self) -> bool {
        matches!(
            self,
            Self::BadenWuerttemberg
                | Self::Bayern
                | Self::Hessen
                | Self::NordrheinWestfalen
                | Self::RheinlandPfalz
                | Self::Saarland
        )
    }

    /// Checks if Assumption Day (Mariä Himmelfahrt) is a holiday in this state.
    #[must_use]
    pub fn has_assumption(&self) -> bool {
        matches!(self, Self::Bayern | Self::Saarland)
    }

    /// Checks if Reformation Day (Reformationstag) is a holiday in this state.
    #[must_use]
    pub fn has_reformation_day(&self) -> bool {
        matches!(
            self,
            Self::Brandenburg
                | Self::Bremen
                | Self::Hamburg
                | Self::MecklenburgVorpommern
                | Self::Niedersachsen
                | Self::Sachsen
                | Self::SachsenAnhalt
                | Self::SchleswigHolstein
                | Self::Thueringen
        )
    }

    /// Checks if All Saints' Day (Allerheiligen) is a holiday in this state.
    #[must_use]
    pub fn has_all_saints(&self) -> bool {
        matches!(
            self,
            Self::BadenWuerttemberg
                | Self::Bayern
                | Self::NordrheinWestfalen
                | Self::RheinlandPfalz
                | Self::Saarland
        )
    }

    /// Checks if Repentance Day (Buß- und Bettag) is a holiday in this state.
    /// Only Saxony (Sachsen) observes this day.
    #[must_use]
    pub fn has_repentance_day(&self) -> bool {
        matches!(self, Self::Sachsen)
    }
}

/// German legal calendar with holiday awareness.
///
/// Provides working day calculations that comply with German legal requirements,
/// including support for state-specific holidays.
///
/// # Example
///
/// ```rust
/// use legalis_de::common::{GermanLegalCalendar, GermanState};
/// use chrono::NaiveDate;
///
/// // Federal holidays only
/// let calendar = GermanLegalCalendar::new();
///
/// // State-specific calendar for Bavaria
/// let bayern_calendar = GermanLegalCalendar::for_state(GermanState::Bayern);
///
/// // Check if German Unity Day is a holiday
/// let unity_day = NaiveDate::from_ymd_opt(2025, 10, 3).unwrap();
/// assert!(calendar.is_holiday(unity_day));
/// ```
#[derive(Debug, Clone)]
pub struct GermanLegalCalendar {
    config: WorkingDaysConfig,
    state: Option<GermanState>,
    year: Option<i32>,
}

impl Default for GermanLegalCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl GermanLegalCalendar {
    /// Creates a new German legal calendar with federal holidays only.
    ///
    /// This includes the 9 nationwide holidays but excludes state-specific holidays.
    /// Movable holidays (Easter-based) are calculated dynamically.
    #[must_use]
    pub fn new() -> Self {
        // Start with a base config for Germany
        let config = WorkingDaysConfig::new("DE")
            // Federal fixed holidays
            .add_holiday(1, 1) // Neujahrstag
            .add_holiday(5, 1) // Tag der Arbeit
            .add_holiday(10, 3) // Tag der Deutschen Einheit
            .add_holiday(12, 25) // 1. Weihnachtsfeiertag
            .add_holiday(12, 26); // 2. Weihnachtsfeiertag

        Self {
            config,
            state: None,
            year: None,
        }
    }

    /// Creates a calendar for a specific German state with state-specific holidays.
    #[must_use]
    pub fn for_state(state: GermanState) -> Self {
        let mut config = WorkingDaysConfig::new("DE")
            // Federal fixed holidays
            .add_holiday(1, 1) // Neujahrstag
            .add_holiday(5, 1) // Tag der Arbeit
            .add_holiday(10, 3) // Tag der Deutschen Einheit
            .add_holiday(12, 25) // 1. Weihnachtsfeiertag
            .add_holiday(12, 26); // 2. Weihnachtsfeiertag

        // Add state-specific fixed holidays
        if state.has_epiphany() {
            config = config.add_holiday(1, 6); // Heilige Drei Könige
        }
        if state.has_assumption() {
            config = config.add_holiday(8, 15); // Mariä Himmelfahrt
        }
        if state.has_reformation_day() {
            config = config.add_holiday(10, 31); // Reformationstag
        }
        if state.has_all_saints() {
            config = config.add_holiday(11, 1); // Allerheiligen
        }

        Self {
            config,
            state: Some(state),
            year: None,
        }
    }

    /// Creates a calendar for a specific year with accurate movable holidays.
    ///
    /// This stores the year for reference. Movable holidays (Easter-based)
    /// are calculated dynamically in `is_holiday()`.
    #[must_use]
    pub fn for_year(year: i32) -> Self {
        let mut calendar = Self::new();
        calendar.year = Some(year);
        calendar
    }

    /// Creates a calendar for a specific year and state with all applicable holidays.
    ///
    /// Movable holidays (Easter-based and state-specific) are calculated
    /// dynamically in `is_holiday()`.
    #[must_use]
    pub fn for_year_and_state(year: i32, state: GermanState) -> Self {
        let mut calendar = Self::for_state(state);
        calendar.year = Some(year);
        calendar
    }

    /// Returns the state for this calendar, if any.
    #[must_use]
    pub fn state(&self) -> Option<GermanState> {
        self.state
    }

    /// Checks if a date is a German public holiday.
    #[must_use]
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        // Check fixed holidays in config
        if !self
            .config
            .is_working_day(date.year(), date.month(), date.day())
            && date.weekday() != Weekday::Sat
            && date.weekday() != Weekday::Sun
        {
            return true;
        }

        // For movable holidays, we need year-specific calculation
        if let Some(easter) = calculate_easter(date.year()) {
            // Federal movable holidays
            let good_friday = easter - chrono::Duration::days(2);
            let easter_monday = easter + chrono::Duration::days(1);
            let ascension = easter + chrono::Duration::days(39);
            let whit_monday = easter + chrono::Duration::days(50);

            if date == good_friday
                || date == easter_monday
                || date == ascension
                || date == whit_monday
            {
                return true;
            }

            // State-specific movable holidays
            if let Some(state) = self.state {
                if state.has_corpus_christi() {
                    let corpus_christi = easter + chrono::Duration::days(60);
                    if date == corpus_christi {
                        return true;
                    }
                }
                if state.has_repentance_day()
                    && let Some(repentance) = calculate_repentance_day(date.year())
                    && date == repentance
                {
                    return true;
                }
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
    /// Used for legal deadline calculations (Fristberechnung).
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

    /// Returns the German name for a holiday on the given date, if any.
    #[must_use]
    pub fn holiday_name(&self, date: NaiveDate) -> Option<&'static str> {
        // Fixed federal holidays
        match (date.month(), date.day()) {
            (1, 1) => return Some("Neujahrstag"),
            (5, 1) => return Some("Tag der Arbeit"),
            (10, 3) => return Some("Tag der Deutschen Einheit"),
            (12, 25) => return Some("1. Weihnachtsfeiertag"),
            (12, 26) => return Some("2. Weihnachtsfeiertag"),
            _ => {}
        }

        // State-specific fixed holidays
        if let Some(state) = self.state {
            match (date.month(), date.day()) {
                (1, 6) if state.has_epiphany() => return Some("Heilige Drei Könige"),
                (8, 15) if state.has_assumption() => return Some("Mariä Himmelfahrt"),
                (10, 31) if state.has_reformation_day() => return Some("Reformationstag"),
                (11, 1) if state.has_all_saints() => return Some("Allerheiligen"),
                _ => {}
            }
        }

        // Movable holidays
        if let Some(easter) = calculate_easter(date.year()) {
            if date == easter - chrono::Duration::days(2) {
                return Some("Karfreitag");
            }
            if date == easter {
                return Some("Ostersonntag"); // Easter Sunday (not official, but informative)
            }
            if date == easter + chrono::Duration::days(1) {
                return Some("Ostermontag");
            }
            if date == easter + chrono::Duration::days(39) {
                return Some("Christi Himmelfahrt");
            }
            if date == easter + chrono::Duration::days(49) {
                return Some("Pfingstsonntag"); // Not official
            }
            if date == easter + chrono::Duration::days(50) {
                return Some("Pfingstmontag");
            }

            if let Some(state) = self.state
                && state.has_corpus_christi()
                && date == easter + chrono::Duration::days(60)
            {
                return Some("Fronleichnam");
            }
        }

        // Buß- und Bettag (Saxony)
        if let Some(state) = self.state
            && state.has_repentance_day()
            && let Some(repentance) = calculate_repentance_day(date.year())
            && date == repentance
        {
            return Some("Buß- und Bettag");
        }

        None
    }
}

/// Calculates Easter Sunday using the Anonymous Gregorian algorithm.
///
/// Returns the date of Easter Sunday for the given year.
fn calculate_easter(year: i32) -> Option<NaiveDate> {
    // Anonymous Gregorian algorithm (Meeus/Jones/Butcher)
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

/// Calculates Buß- und Bettag (Repentance Day) for Saxony.
///
/// It falls on the Wednesday before November 23.
fn calculate_repentance_day(year: i32) -> Option<NaiveDate> {
    let nov_23 = NaiveDate::from_ymd_opt(year, 11, 23)?;
    // Wednesday is num_days_from_monday() = 2
    let days_since_wednesday = (nov_23.weekday().num_days_from_monday() as i64 - 2 + 7) % 7;
    if days_since_wednesday == 0 {
        // If Nov 23 is Wednesday, go back 7 days
        Some(nov_23 - chrono::Duration::days(7))
    } else {
        Some(nov_23 - chrono::Duration::days(days_since_wednesday))
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Checks if a date is a German public holiday (federal only).
///
/// # Example
///
/// ```rust
/// use legalis_de::common::is_german_holiday;
/// use chrono::NaiveDate;
///
/// let unity_day = NaiveDate::from_ymd_opt(2025, 10, 3).unwrap();
/// assert!(is_german_holiday(unity_day));
/// ```
#[must_use]
pub fn is_german_holiday(date: NaiveDate) -> bool {
    GermanLegalCalendar::new().is_holiday(date)
}

/// Checks if a date is a German working day.
#[must_use]
pub fn is_working_day(date: NaiveDate) -> bool {
    GermanLegalCalendar::new().is_working_day(date)
}

/// Calculates a legal deadline by adding working days (Fristberechnung).
///
/// # Example
///
/// ```rust
/// use legalis_de::common::calculate_legal_deadline;
/// use chrono::NaiveDate;
///
/// let start = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
/// let deadline = calculate_legal_deadline(start, 14);
/// // Deadline is 14 working days later, skipping weekends and holidays
/// ```
#[must_use]
pub fn calculate_legal_deadline(start: NaiveDate, business_days: i32) -> NaiveDate {
    GermanLegalCalendar::for_year(start.year()).add_working_days(start, business_days)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Easter calculation tests
    // ========================================================================

    #[test]
    fn test_easter_calculation() {
        // Known Easter dates
        assert_eq!(calculate_easter(2024), NaiveDate::from_ymd_opt(2024, 3, 31));
        assert_eq!(calculate_easter(2025), NaiveDate::from_ymd_opt(2025, 4, 20));
        assert_eq!(calculate_easter(2026), NaiveDate::from_ymd_opt(2026, 4, 5));
    }

    #[test]
    fn test_repentance_day_calculation() {
        // Buß- und Bettag falls on Wednesday before Nov 23
        // 2025: Nov 19 (Wednesday)
        assert_eq!(
            calculate_repentance_day(2025),
            NaiveDate::from_ymd_opt(2025, 11, 19)
        );
        // 2024: Nov 20 (Wednesday)
        assert_eq!(
            calculate_repentance_day(2024),
            NaiveDate::from_ymd_opt(2024, 11, 20)
        );
    }

    // ========================================================================
    // Federal holiday tests
    // ========================================================================

    #[test]
    fn test_new_year_is_holiday() {
        let calendar = GermanLegalCalendar::new();
        let new_year = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(calendar.is_holiday(new_year));
        assert!(!calendar.is_working_day(new_year));
    }

    #[test]
    fn test_labor_day_is_holiday() {
        let calendar = GermanLegalCalendar::new();
        let labor_day = NaiveDate::from_ymd_opt(2025, 5, 1).unwrap();
        assert!(calendar.is_holiday(labor_day));
    }

    #[test]
    fn test_german_unity_day_is_holiday() {
        let calendar = GermanLegalCalendar::new();
        let unity_day = NaiveDate::from_ymd_opt(2025, 10, 3).unwrap();
        assert!(calendar.is_holiday(unity_day));
        assert_eq!(
            calendar.holiday_name(unity_day),
            Some("Tag der Deutschen Einheit")
        );
    }

    #[test]
    fn test_christmas_holidays() {
        let calendar = GermanLegalCalendar::new();
        let christmas1 = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        let christmas2 = NaiveDate::from_ymd_opt(2025, 12, 26).unwrap();
        assert!(calendar.is_holiday(christmas1));
        assert!(calendar.is_holiday(christmas2));
        assert_eq!(
            calendar.holiday_name(christmas1),
            Some("1. Weihnachtsfeiertag")
        );
        assert_eq!(
            calendar.holiday_name(christmas2),
            Some("2. Weihnachtsfeiertag")
        );
    }

    // ========================================================================
    // Movable holiday tests
    // ========================================================================

    #[test]
    fn test_good_friday_2025() {
        let calendar = GermanLegalCalendar::for_year(2025);
        // Easter 2025 is April 20, so Good Friday is April 18
        let good_friday = NaiveDate::from_ymd_opt(2025, 4, 18).unwrap();
        assert!(calendar.is_holiday(good_friday));
        assert_eq!(calendar.holiday_name(good_friday), Some("Karfreitag"));
    }

    #[test]
    fn test_easter_monday_2025() {
        let calendar = GermanLegalCalendar::for_year(2025);
        // Easter 2025 is April 20, so Easter Monday is April 21
        let easter_monday = NaiveDate::from_ymd_opt(2025, 4, 21).unwrap();
        assert!(calendar.is_holiday(easter_monday));
        assert_eq!(calendar.holiday_name(easter_monday), Some("Ostermontag"));
    }

    #[test]
    fn test_ascension_day_2025() {
        let calendar = GermanLegalCalendar::for_year(2025);
        // Easter 2025 is April 20, so Ascension is May 29 (Easter + 39)
        let ascension = NaiveDate::from_ymd_opt(2025, 5, 29).unwrap();
        assert!(calendar.is_holiday(ascension));
        assert_eq!(
            calendar.holiday_name(ascension),
            Some("Christi Himmelfahrt")
        );
    }

    #[test]
    fn test_whit_monday_2025() {
        let calendar = GermanLegalCalendar::for_year(2025);
        // Easter 2025 is April 20, so Whit Monday is June 9 (Easter + 50)
        let whit_monday = NaiveDate::from_ymd_opt(2025, 6, 9).unwrap();
        assert!(calendar.is_holiday(whit_monday));
        assert_eq!(calendar.holiday_name(whit_monday), Some("Pfingstmontag"));
    }

    // ========================================================================
    // State-specific holiday tests
    // ========================================================================

    #[test]
    fn test_epiphany_in_bavaria() {
        let calendar = GermanLegalCalendar::for_state(GermanState::Bayern);
        let epiphany = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        assert!(calendar.is_holiday(epiphany));
        assert_eq!(calendar.holiday_name(epiphany), Some("Heilige Drei Könige"));
    }

    #[test]
    fn test_epiphany_not_in_berlin() {
        let calendar = GermanLegalCalendar::for_state(GermanState::Berlin);
        let epiphany = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        // January 6, 2025 is a Monday - should be working day in Berlin
        assert!(calendar.is_working_day(epiphany));
    }

    #[test]
    fn test_reformation_day_in_brandenburg() {
        let calendar = GermanLegalCalendar::for_state(GermanState::Brandenburg);
        let reformation = NaiveDate::from_ymd_opt(2025, 10, 31).unwrap();
        assert!(calendar.is_holiday(reformation));
        assert_eq!(calendar.holiday_name(reformation), Some("Reformationstag"));
    }

    #[test]
    fn test_reformation_day_not_in_bayern() {
        let calendar = GermanLegalCalendar::for_state(GermanState::Bayern);
        let reformation = NaiveDate::from_ymd_opt(2025, 10, 31).unwrap();
        // October 31, 2025 is Friday - should be working day in Bavaria
        assert!(calendar.is_working_day(reformation));
    }

    #[test]
    fn test_all_saints_in_nrw() {
        let calendar = GermanLegalCalendar::for_state(GermanState::NordrheinWestfalen);
        // Nov 1, 2025 is Saturday - need a year where it's weekday
        let all_saints = NaiveDate::from_ymd_opt(2024, 11, 1).unwrap(); // Friday
        assert!(calendar.is_holiday(all_saints));
        assert_eq!(calendar.holiday_name(all_saints), Some("Allerheiligen"));
    }

    #[test]
    fn test_corpus_christi_in_hessen() {
        let calendar = GermanLegalCalendar::for_year_and_state(2025, GermanState::Hessen);
        // Easter 2025 is April 20, so Corpus Christi is June 19 (Easter + 60)
        let corpus_christi = NaiveDate::from_ymd_opt(2025, 6, 19).unwrap();
        assert!(calendar.is_holiday(corpus_christi));
        assert_eq!(calendar.holiday_name(corpus_christi), Some("Fronleichnam"));
    }

    #[test]
    fn test_repentance_day_in_sachsen() {
        let calendar = GermanLegalCalendar::for_year_and_state(2025, GermanState::Sachsen);
        let repentance = NaiveDate::from_ymd_opt(2025, 11, 19).unwrap();
        assert!(calendar.is_holiday(repentance));
        assert_eq!(calendar.holiday_name(repentance), Some("Buß- und Bettag"));
    }

    #[test]
    fn test_repentance_day_not_in_berlin() {
        let calendar = GermanLegalCalendar::for_year_and_state(2025, GermanState::Berlin);
        let repentance = NaiveDate::from_ymd_opt(2025, 11, 19).unwrap();
        // November 19, 2025 is Wednesday - should be working day in Berlin
        assert!(calendar.is_working_day(repentance));
    }

    // ========================================================================
    // Working day calculation tests
    // ========================================================================

    #[test]
    fn test_regular_weekday_is_working_day() {
        let calendar = GermanLegalCalendar::new();
        // A regular Wednesday in June should be a working day
        let wednesday = NaiveDate::from_ymd_opt(2025, 6, 11).unwrap();
        assert!(calendar.is_working_day(wednesday));
    }

    #[test]
    fn test_weekend_not_working_day() {
        let calendar = GermanLegalCalendar::new();
        let saturday = NaiveDate::from_ymd_opt(2025, 6, 14).unwrap();
        let sunday = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
        assert!(!calendar.is_working_day(saturday));
        assert!(!calendar.is_working_day(sunday));
    }

    #[test]
    fn test_add_working_days() {
        let calendar = GermanLegalCalendar::for_year(2025);
        // Start on Monday June 2, add 5 working days
        let monday = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
        let result = calendar.add_working_days(monday, 5);
        // Mon 6/2, Tue 6/3, Wed 6/4, Thu 6/5, Fri 6/6 = 5 working days
        // But June 9 is Pfingstmontag (Whit Monday)!
        // So: Mon 6/2, Tue 6/3, Wed 6/4, Thu 6/5, Fri 6/6 -> result = 6/9
        // Actually: 5 working days from Mon includes Mon, so result is end of 5th day
        // Let me recalculate:
        // Day 1: Tue 6/3, Day 2: Wed 6/4, Day 3: Thu 6/5, Day 4: Fri 6/6,
        // Sat 6/7 skip, Sun 6/8 skip, Mon 6/9 is Pfingstmontag skip,
        // Day 5: Tue 6/10
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 6, 10).unwrap());
    }

    #[test]
    fn test_working_days_between() {
        let calendar = GermanLegalCalendar::for_year(2025);
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 6, 13).unwrap(); // Friday
        // Working days: Mon 6/2, Tue 6/3, Wed 6/4, Thu 6/5, Fri 6/6 = 5
        // + Tue 6/10, Wed 6/11, Thu 6/12 = 3 (Mon 6/9 is Pfingstmontag)
        // Total = 8
        assert_eq!(calendar.working_days_between(start, end), 8);
    }

    // ========================================================================
    // GermanState tests
    // ========================================================================

    #[test]
    fn test_state_abbreviations() {
        assert_eq!(GermanState::Bayern.abbreviation(), "BY");
        assert_eq!(GermanState::BadenWuerttemberg.abbreviation(), "BW");
        assert_eq!(GermanState::NordrheinWestfalen.abbreviation(), "NW");
    }

    #[test]
    fn test_state_german_names() {
        assert_eq!(GermanState::Bayern.german_name(), "Bayern");
        assert_eq!(GermanState::Thueringen.german_name(), "Thüringen");
        assert_eq!(
            GermanState::NordrheinWestfalen.german_name(),
            "Nordrhein-Westfalen"
        );
    }

    #[test]
    fn test_state_holiday_flags() {
        // Bavaria has Epiphany, Assumption, All Saints, Corpus Christi
        assert!(GermanState::Bayern.has_epiphany());
        assert!(GermanState::Bayern.has_assumption());
        assert!(GermanState::Bayern.has_all_saints());
        assert!(GermanState::Bayern.has_corpus_christi());
        assert!(!GermanState::Bayern.has_reformation_day());
        assert!(!GermanState::Bayern.has_repentance_day());

        // Saxony has Reformation Day, Repentance Day
        assert!(GermanState::Sachsen.has_reformation_day());
        assert!(GermanState::Sachsen.has_repentance_day());
        assert!(!GermanState::Sachsen.has_epiphany());
    }

    // ========================================================================
    // Convenience function tests
    // ========================================================================

    #[test]
    fn test_is_german_holiday_function() {
        let unity_day = NaiveDate::from_ymd_opt(2025, 10, 3).unwrap();
        assert!(is_german_holiday(unity_day));

        let regular_day = NaiveDate::from_ymd_opt(2025, 6, 11).unwrap();
        assert!(!is_german_holiday(regular_day));
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
        // Should skip weekends and Good Friday (April 18)
        // April 1 is Tuesday
        // Day 1: Wed 4/2, Day 2: Thu 4/3, Day 3: Fri 4/4, Day 4: Mon 4/7, Day 5: Tue 4/8
        // Day 6: Wed 4/9, Day 7: Thu 4/10, Day 8: Fri 4/11, Day 9: Mon 4/14, Day 10: Tue 4/15
        assert_eq!(deadline, NaiveDate::from_ymd_opt(2025, 4, 15).unwrap());
    }
}
