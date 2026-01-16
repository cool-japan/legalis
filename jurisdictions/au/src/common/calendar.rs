//! Australian Calendar
//!
//! Australian holidays, business days, and date calculations.

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

use super::types::StateTerritory;

// ============================================================================
// Holiday Types
// ============================================================================

/// Type of Australian holiday
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HolidayType {
    /// National public holiday (all states)
    National,
    /// State-specific public holiday
    State(StateTerritory),
    /// Banking/business day (not public holiday)
    Banking,
}

/// Australian public holiday
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AustralianHoliday {
    /// Name of the holiday
    pub name: String,
    /// Date
    pub date: NaiveDate,
    /// Type of holiday
    pub holiday_type: HolidayType,
    /// Whether substitute day applies if falls on weekend
    pub substitute_applies: bool,
}

impl AustralianHoliday {
    /// Create a new holiday
    pub fn new(
        name: impl Into<String>,
        date: NaiveDate,
        holiday_type: HolidayType,
        substitute_applies: bool,
    ) -> Self {
        Self {
            name: name.into(),
            date,
            holiday_type,
            substitute_applies,
        }
    }
}

// ============================================================================
// Calendar Calculator
// ============================================================================

/// Australian calendar calculator
pub struct AustralianCalendar;

impl AustralianCalendar {
    /// Get all national public holidays for a year
    pub fn national_holidays(year: i32) -> Vec<AustralianHoliday> {
        let mut holidays = Vec::new();

        // New Year's Day - January 1
        if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1) {
            holidays.push(AustralianHoliday::new(
                "New Year's Day",
                date,
                HolidayType::National,
                true,
            ));
        }

        // Australia Day - January 26
        if let Some(date) = NaiveDate::from_ymd_opt(year, 1, 26) {
            holidays.push(AustralianHoliday::new(
                "Australia Day",
                date,
                HolidayType::National,
                true,
            ));
        }

        // Good Friday (Friday before Easter Sunday)
        if let Some(easter) = Self::easter_sunday(year) {
            if let Some(good_friday) = easter
                .pred_opt()
                .and_then(|d| d.pred_opt().and_then(|d| d.pred_opt()))
            {
                holidays.push(AustralianHoliday::new(
                    "Good Friday",
                    good_friday,
                    HolidayType::National,
                    false,
                ));
            }

            // Easter Saturday
            if let Some(easter_sat) = easter.pred_opt() {
                holidays.push(AustralianHoliday::new(
                    "Easter Saturday",
                    easter_sat,
                    HolidayType::National,
                    false,
                ));
            }

            // Easter Monday
            if let Some(easter_mon) = easter.succ_opt() {
                holidays.push(AustralianHoliday::new(
                    "Easter Monday",
                    easter_mon,
                    HolidayType::National,
                    false,
                ));
            }
        }

        // ANZAC Day - April 25
        if let Some(date) = NaiveDate::from_ymd_opt(year, 4, 25) {
            holidays.push(AustralianHoliday::new(
                "ANZAC Day",
                date,
                HolidayType::National,
                true,
            ));
        }

        // Queen's Birthday - Second Monday of June (most states)
        if let Some(date) = Self::nth_weekday_of_month(year, 6, Weekday::Mon, 2) {
            holidays.push(AustralianHoliday::new(
                "Queen's Birthday",
                date,
                HolidayType::National, // Actually varies by state
                false,
            ));
        }

        // Christmas Day - December 25
        if let Some(date) = NaiveDate::from_ymd_opt(year, 12, 25) {
            holidays.push(AustralianHoliday::new(
                "Christmas Day",
                date,
                HolidayType::National,
                true,
            ));
        }

        // Boxing Day - December 26
        if let Some(date) = NaiveDate::from_ymd_opt(year, 12, 26) {
            holidays.push(AustralianHoliday::new(
                "Boxing Day",
                date,
                HolidayType::National,
                true,
            ));
        }

        holidays
    }

    /// Get state-specific holidays
    pub fn state_holidays(year: i32, state: StateTerritory) -> Vec<AustralianHoliday> {
        let mut holidays = Vec::new();

        match state {
            StateTerritory::Victoria => {
                // Melbourne Cup Day - First Tuesday of November
                if let Some(date) = Self::nth_weekday_of_month(year, 11, Weekday::Tue, 1) {
                    holidays.push(AustralianHoliday::new(
                        "Melbourne Cup Day",
                        date,
                        HolidayType::State(StateTerritory::Victoria),
                        false,
                    ));
                }
                // AFL Grand Final Friday
                // Usually last Friday of September - simplified
                if let Some(date) = Self::last_weekday_of_month(year, 9, Weekday::Fri) {
                    holidays.push(AustralianHoliday::new(
                        "AFL Grand Final Friday",
                        date,
                        HolidayType::State(StateTerritory::Victoria),
                        false,
                    ));
                }
            }
            StateTerritory::Queensland => {
                // Royal Queensland Show (Ekka) - Brisbane region, second Wednesday August
                if let Some(date) = Self::nth_weekday_of_month(year, 8, Weekday::Wed, 2) {
                    holidays.push(AustralianHoliday::new(
                        "Royal Queensland Show",
                        date,
                        HolidayType::State(StateTerritory::Queensland),
                        false,
                    ));
                }
            }
            StateTerritory::SouthAustralia => {
                // Adelaide Cup Day - Second Monday of March
                if let Some(date) = Self::nth_weekday_of_month(year, 3, Weekday::Mon, 2) {
                    holidays.push(AustralianHoliday::new(
                        "Adelaide Cup Day",
                        date,
                        HolidayType::State(StateTerritory::SouthAustralia),
                        false,
                    ));
                }
                // Proclamation Day - December 24 (or substitute)
                if let Some(date) = NaiveDate::from_ymd_opt(year, 12, 24) {
                    holidays.push(AustralianHoliday::new(
                        "Proclamation Day",
                        date,
                        HolidayType::State(StateTerritory::SouthAustralia),
                        true,
                    ));
                }
            }
            StateTerritory::WesternAustralia => {
                // Western Australia Day - First Monday of June
                if let Some(date) = Self::nth_weekday_of_month(year, 6, Weekday::Mon, 1) {
                    holidays.push(AustralianHoliday::new(
                        "Western Australia Day",
                        date,
                        HolidayType::State(StateTerritory::WesternAustralia),
                        false,
                    ));
                }
            }
            StateTerritory::Tasmania => {
                // Royal Hobart Regatta - Second Monday of February (Southern Tas)
                if let Some(date) = Self::nth_weekday_of_month(year, 2, Weekday::Mon, 2) {
                    holidays.push(AustralianHoliday::new(
                        "Royal Hobart Regatta",
                        date,
                        HolidayType::State(StateTerritory::Tasmania),
                        false,
                    ));
                }
            }
            StateTerritory::NorthernTerritory => {
                // May Day - First Monday of May
                if let Some(date) = Self::nth_weekday_of_month(year, 5, Weekday::Mon, 1) {
                    holidays.push(AustralianHoliday::new(
                        "May Day",
                        date,
                        HolidayType::State(StateTerritory::NorthernTerritory),
                        false,
                    ));
                }
                // Picnic Day - First Monday of August
                if let Some(date) = Self::nth_weekday_of_month(year, 8, Weekday::Mon, 1) {
                    holidays.push(AustralianHoliday::new(
                        "Picnic Day",
                        date,
                        HolidayType::State(StateTerritory::NorthernTerritory),
                        false,
                    ));
                }
            }
            StateTerritory::AustralianCapitalTerritory => {
                // Canberra Day - Second Monday of March
                if let Some(date) = Self::nth_weekday_of_month(year, 3, Weekday::Mon, 2) {
                    holidays.push(AustralianHoliday::new(
                        "Canberra Day",
                        date,
                        HolidayType::State(StateTerritory::AustralianCapitalTerritory),
                        false,
                    ));
                }
                // Reconciliation Day - 27 May (or Monday after if weekend)
                if let Some(date) = NaiveDate::from_ymd_opt(year, 5, 27) {
                    holidays.push(AustralianHoliday::new(
                        "Reconciliation Day",
                        date,
                        HolidayType::State(StateTerritory::AustralianCapitalTerritory),
                        true,
                    ));
                }
            }
            StateTerritory::NewSouthWales => {
                // Bank Holiday - First Monday of August (banking only)
                if let Some(date) = Self::nth_weekday_of_month(year, 8, Weekday::Mon, 1) {
                    holidays.push(AustralianHoliday::new(
                        "Bank Holiday",
                        date,
                        HolidayType::State(StateTerritory::NewSouthWales),
                        false,
                    ));
                }
            }
        }

        holidays
    }

    /// Get all holidays for a state/territory in a year
    pub fn all_holidays(year: i32, state: StateTerritory) -> Vec<AustralianHoliday> {
        let mut holidays = Self::national_holidays(year);
        holidays.extend(Self::state_holidays(year, state));
        holidays.sort_by(|a, b| a.date.cmp(&b.date));
        holidays
    }

    /// Check if a date is a public holiday
    pub fn is_public_holiday(date: NaiveDate, state: StateTerritory) -> bool {
        let holidays = Self::all_holidays(date.year(), state);
        holidays.iter().any(|h| h.date == date)
    }

    /// Check if a date is a business day
    pub fn is_business_day(date: NaiveDate, state: StateTerritory) -> bool {
        let weekday = date.weekday();
        if weekday == Weekday::Sat || weekday == Weekday::Sun {
            return false;
        }
        !Self::is_public_holiday(date, state)
    }

    /// Add business days to a date
    pub fn add_business_days(
        start: NaiveDate,
        days: i32,
        state: StateTerritory,
    ) -> Option<NaiveDate> {
        if days == 0 {
            return Some(start);
        }

        let mut current = start;
        let mut remaining = days.abs();
        let direction = if days > 0 { 1 } else { -1 };

        while remaining > 0 {
            current = if direction > 0 {
                current.succ_opt()?
            } else {
                current.pred_opt()?
            };

            if Self::is_business_day(current, state) {
                remaining -= 1;
            }
        }

        Some(current)
    }

    /// Calculate Easter Sunday using Anonymous Gregorian algorithm
    pub fn easter_sunday(year: i32) -> Option<NaiveDate> {
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

    /// Get the nth weekday of a month (e.g., 2nd Monday)
    fn nth_weekday_of_month(year: i32, month: u32, weekday: Weekday, n: u32) -> Option<NaiveDate> {
        let first_of_month = NaiveDate::from_ymd_opt(year, month, 1)?;
        let first_weekday = first_of_month.weekday();

        // Days until first occurrence of target weekday
        let days_until = (weekday.num_days_from_monday() as i32
            - first_weekday.num_days_from_monday() as i32
            + 7)
            % 7;

        let day = 1 + days_until as u32 + (n - 1) * 7;
        NaiveDate::from_ymd_opt(year, month, day)
    }

    /// Get the last weekday of a month
    fn last_weekday_of_month(year: i32, month: u32, weekday: Weekday) -> Option<NaiveDate> {
        // Get last day of month
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        let first_of_next = NaiveDate::from_ymd_opt(next_year, next_month, 1)?;
        let last_of_month = first_of_next.pred_opt()?;

        let last_weekday = last_of_month.weekday();
        let days_back = (last_weekday.num_days_from_monday() as i32
            - weekday.num_days_from_monday() as i32
            + 7)
            % 7;

        let day = last_of_month.day() as i32 - days_back;
        NaiveDate::from_ymd_opt(year, month, day as u32)
    }
}

// ============================================================================
// Limitation Periods
// ============================================================================

/// Common limitation periods in Australian law
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitationPeriod {
    /// Personal injury (typically 3 years)
    PersonalInjury,
    /// Contract claims (typically 6 years)
    Contract,
    /// Tort claims (typically 6 years)
    Tort,
    /// Property claims (typically 12-15 years)
    Property,
    /// Summary criminal matters (typically 6-12 months)
    SummaryCriminal,
    /// Indictable criminal matters (no limit generally)
    IndictableCriminal,
    /// Tax debts (typically 4-5 years)
    Tax,
}

impl LimitationPeriod {
    /// Get the limitation period in years for a state
    pub fn years(&self, state: StateTerritory) -> Option<u32> {
        match self {
            Self::PersonalInjury => Some(3),
            Self::Contract => Some(6),
            Self::Tort => Some(6),
            Self::Property => match state {
                StateTerritory::Victoria => Some(15),
                StateTerritory::Queensland => Some(12),
                _ => Some(12),
            },
            Self::SummaryCriminal => None,    // Varies significantly
            Self::IndictableCriminal => None, // No limit
            Self::Tax => Some(4),
        }
    }

    /// Calculate limitation expiry date
    pub fn expiry_date(
        &self,
        cause_of_action: NaiveDate,
        state: StateTerritory,
    ) -> Option<NaiveDate> {
        let years = self.years(state)?;
        cause_of_action.with_year(cause_of_action.year() + years as i32)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_national_holidays_2024() {
        let holidays = AustralianCalendar::national_holidays(2024);
        let names: Vec<_> = holidays.iter().map(|h| h.name.as_str()).collect();

        assert!(names.contains(&"New Year's Day"));
        assert!(names.contains(&"Australia Day"));
        assert!(names.contains(&"Good Friday"));
        assert!(names.contains(&"ANZAC Day"));
        assert!(names.contains(&"Christmas Day"));
    }

    #[test]
    fn test_easter_2024() {
        let easter = AustralianCalendar::easter_sunday(2024);
        assert_eq!(easter, NaiveDate::from_ymd_opt(2024, 3, 31));
    }

    #[test]
    fn test_easter_2025() {
        let easter = AustralianCalendar::easter_sunday(2025);
        assert_eq!(easter, NaiveDate::from_ymd_opt(2025, 4, 20));
    }

    #[test]
    fn test_melbourne_cup() {
        let holidays = AustralianCalendar::state_holidays(2024, StateTerritory::Victoria);
        let cup = holidays.iter().find(|h| h.name == "Melbourne Cup Day");
        assert!(cup.is_some());
    }

    #[test]
    fn test_is_business_day() {
        // A regular weekday
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).expect("valid date");
        assert!(AustralianCalendar::is_business_day(
            monday,
            StateTerritory::NewSouthWales
        ));

        // A weekend
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).expect("valid date");
        assert!(!AustralianCalendar::is_business_day(
            saturday,
            StateTerritory::NewSouthWales
        ));

        // Australia Day
        let aus_day = NaiveDate::from_ymd_opt(2024, 1, 26).expect("valid date");
        assert!(!AustralianCalendar::is_business_day(
            aus_day,
            StateTerritory::NewSouthWales
        ));
    }

    #[test]
    fn test_add_business_days() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 8).expect("valid date"); // Monday
        let result = AustralianCalendar::add_business_days(start, 5, StateTerritory::NewSouthWales);

        // Should skip the weekend (13th-14th) and land on 15th
        assert!(result.is_some());
        let end = result.expect("valid date");
        assert_eq!(end.weekday(), Weekday::Mon);
    }

    #[test]
    fn test_limitation_periods() {
        let cause = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");

        let personal_injury =
            LimitationPeriod::PersonalInjury.expiry_date(cause, StateTerritory::NewSouthWales);
        assert_eq!(personal_injury, NaiveDate::from_ymd_opt(2027, 1, 1));

        let contract = LimitationPeriod::Contract.expiry_date(cause, StateTerritory::Victoria);
        assert_eq!(contract, NaiveDate::from_ymd_opt(2030, 1, 1));
    }
}
