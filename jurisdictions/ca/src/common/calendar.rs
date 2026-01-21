//! Canada Calendar Utilities
//!
//! Canadian statutory holidays and working day calculations.
//! Note: Holidays vary significantly by province.

#![allow(missing_docs)]

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

use super::types::Province;

// ============================================================================
// Holiday Types
// ============================================================================

/// Canadian statutory holiday
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Holiday {
    /// Name of the holiday
    pub name: String,
    /// Date of the holiday
    pub date: NaiveDate,
    /// Whether it's a federal holiday
    pub federal: bool,
    /// Provinces where this is a statutory holiday
    pub provinces: Vec<Province>,
}

impl Holiday {
    /// Check if this holiday applies to a province
    pub fn applies_to(&self, province: Province) -> bool {
        self.provinces.contains(&province)
    }

    /// Check if this is a national holiday (federal + all provinces)
    pub fn is_national(&self) -> bool {
        self.federal && self.provinces.len() >= 10
    }
}

// ============================================================================
// Holiday Calculator
// ============================================================================

/// Canadian holiday calculator
pub struct CanadianCalendar;

impl CanadianCalendar {
    /// Get all federal statutory holidays for a year
    pub fn federal_holidays(year: i32) -> Vec<Holiday> {
        let mut holidays = Vec::new();

        // New Year's Day - January 1
        holidays.push(Holiday {
            name: "New Year's Day".to_string(),
            date: NaiveDate::from_ymd_opt(year, 1, 1).expect("valid date"),
            federal: true,
            provinces: Province::all().to_vec(),
        });

        // Good Friday - varies (Friday before Easter)
        if let Some(easter) = Self::easter_date(year)
            && let Some(good_friday) = easter.pred_opt().and_then(|d| d.pred_opt())
        {
            holidays.push(Holiday {
                name: "Good Friday".to_string(),
                date: good_friday,
                federal: true,
                provinces: Province::all().to_vec(),
            });
        }

        // Victoria Day - Monday before May 25
        if let Some(date) = Self::monday_before(year, 5, 25) {
            holidays.push(Holiday {
                name: "Victoria Day".to_string(),
                date,
                federal: true,
                provinces: Province::all()
                    .iter()
                    .copied()
                    .filter(|p| *p != Province::Quebec && *p != Province::NovaScotia)
                    .collect(),
            });
        }

        // Canada Day - July 1 (or July 2 if July 1 is Sunday)
        let july_1 = NaiveDate::from_ymd_opt(year, 7, 1).expect("valid date");
        let canada_day = if july_1.weekday() == Weekday::Sun {
            july_1.succ_opt().expect("valid date")
        } else {
            july_1
        };
        holidays.push(Holiday {
            name: "Canada Day".to_string(),
            date: canada_day,
            federal: true,
            provinces: Province::all().to_vec(),
        });

        // Labour Day - First Monday of September
        if let Some(date) = Self::first_monday_of_month(year, 9) {
            holidays.push(Holiday {
                name: "Labour Day".to_string(),
                date,
                federal: true,
                provinces: Province::all().to_vec(),
            });
        }

        // National Day for Truth and Reconciliation - September 30 (federal + some provinces)
        holidays.push(Holiday {
            name: "National Day for Truth and Reconciliation".to_string(),
            date: NaiveDate::from_ymd_opt(year, 9, 30).expect("valid date"),
            federal: true,
            provinces: vec![
                Province::BritishColumbia,
                Province::Manitoba,
                Province::NorthwestTerritories,
                Province::PrinceEdwardIsland,
                Province::Yukon,
            ],
        });

        // Thanksgiving - Second Monday of October
        if let Some(date) = Self::nth_weekday_of_month(year, 10, Weekday::Mon, 2) {
            holidays.push(Holiday {
                name: "Thanksgiving".to_string(),
                date,
                federal: true,
                provinces: Province::all()
                    .iter()
                    .copied()
                    .filter(|p| {
                        *p != Province::NewBrunswick
                            && *p != Province::NovaScotia
                            && *p != Province::PrinceEdwardIsland
                            && *p != Province::NewfoundlandLabrador
                    })
                    .collect(),
            });
        }

        // Remembrance Day - November 11
        holidays.push(Holiday {
            name: "Remembrance Day".to_string(),
            date: NaiveDate::from_ymd_opt(year, 11, 11).expect("valid date"),
            federal: true,
            provinces: vec![
                Province::Alberta,
                Province::BritishColumbia,
                Province::NewBrunswick,
                Province::NewfoundlandLabrador,
                Province::NorthwestTerritories,
                Province::Nunavut,
                Province::PrinceEdwardIsland,
                Province::Saskatchewan,
                Province::Yukon,
            ],
        });

        // Christmas Day - December 25
        holidays.push(Holiday {
            name: "Christmas Day".to_string(),
            date: NaiveDate::from_ymd_opt(year, 12, 25).expect("valid date"),
            federal: true,
            provinces: Province::all().to_vec(),
        });

        // Boxing Day - December 26 (Ontario only as statutory)
        holidays.push(Holiday {
            name: "Boxing Day".to_string(),
            date: NaiveDate::from_ymd_opt(year, 12, 26).expect("valid date"),
            federal: false,
            provinces: vec![Province::Ontario],
        });

        holidays
    }

    /// Get provincial holidays for a specific province
    pub fn provincial_holidays(year: i32, province: Province) -> Vec<Holiday> {
        let mut holidays = Vec::new();

        match province {
            Province::Quebec => {
                // Saint-Jean-Baptiste Day (Fête nationale) - June 24
                holidays.push(Holiday {
                    name: "Saint-Jean-Baptiste Day".to_string(),
                    date: NaiveDate::from_ymd_opt(year, 6, 24).expect("valid date"),
                    federal: false,
                    provinces: vec![Province::Quebec],
                });

                // National Patriots' Day (Monday before May 25, replaces Victoria Day)
                if let Some(date) = Self::monday_before(year, 5, 25) {
                    holidays.push(Holiday {
                        name: "National Patriots' Day".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::Quebec],
                    });
                }
            }
            Province::Alberta => {
                // Family Day - Third Monday of February
                if let Some(date) = Self::nth_weekday_of_month(year, 2, Weekday::Mon, 3) {
                    holidays.push(Holiday {
                        name: "Family Day".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::Alberta],
                    });
                }

                // Alberta Heritage Day - First Monday of August
                if let Some(date) = Self::first_monday_of_month(year, 8) {
                    holidays.push(Holiday {
                        name: "Heritage Day".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::Alberta],
                    });
                }
            }
            Province::BritishColumbia => {
                // Family Day - Third Monday of February
                if let Some(date) = Self::nth_weekday_of_month(year, 2, Weekday::Mon, 3) {
                    holidays.push(Holiday {
                        name: "Family Day".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::BritishColumbia],
                    });
                }

                // BC Day - First Monday of August
                if let Some(date) = Self::first_monday_of_month(year, 8) {
                    holidays.push(Holiday {
                        name: "British Columbia Day".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::BritishColumbia],
                    });
                }
            }
            Province::Ontario => {
                // Family Day - Third Monday of February
                if let Some(date) = Self::nth_weekday_of_month(year, 2, Weekday::Mon, 3) {
                    holidays.push(Holiday {
                        name: "Family Day".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::Ontario],
                    });
                }

                // Civic Holiday - First Monday of August (not statutory but widely observed)
                if let Some(date) = Self::first_monday_of_month(year, 8) {
                    holidays.push(Holiday {
                        name: "Civic Holiday".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::Ontario],
                    });
                }
            }
            Province::Manitoba => {
                // Louis Riel Day - Third Monday of February
                if let Some(date) = Self::nth_weekday_of_month(year, 2, Weekday::Mon, 3) {
                    holidays.push(Holiday {
                        name: "Louis Riel Day".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::Manitoba],
                    });
                }
            }
            Province::Saskatchewan => {
                // Family Day - Third Monday of February
                if let Some(date) = Self::nth_weekday_of_month(year, 2, Weekday::Mon, 3) {
                    holidays.push(Holiday {
                        name: "Family Day".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::Saskatchewan],
                    });
                }

                // Saskatchewan Day - First Monday of August
                if let Some(date) = Self::first_monday_of_month(year, 8) {
                    holidays.push(Holiday {
                        name: "Saskatchewan Day".to_string(),
                        date,
                        federal: false,
                        provinces: vec![Province::Saskatchewan],
                    });
                }
            }
            _ => {}
        }

        holidays
    }

    /// Get all holidays for a province in a year
    pub fn all_holidays(year: i32, province: Province) -> Vec<Holiday> {
        let mut holidays: Vec<_> = Self::federal_holidays(year)
            .into_iter()
            .filter(|h| h.applies_to(province))
            .collect();

        holidays.extend(Self::provincial_holidays(year, province));
        holidays.sort_by_key(|h| h.date);
        holidays
    }

    /// Check if a date is a statutory holiday
    pub fn is_holiday(date: NaiveDate, province: Province) -> bool {
        Self::all_holidays(date.year(), province)
            .iter()
            .any(|h| h.date == date)
    }

    /// Check if a date is a working day (not weekend, not holiday)
    pub fn is_working_day(date: NaiveDate, province: Province) -> bool {
        let weekday = date.weekday();
        if weekday == Weekday::Sat || weekday == Weekday::Sun {
            return false;
        }
        !Self::is_holiday(date, province)
    }

    /// Get the next working day on or after a date
    pub fn next_working_day(date: NaiveDate, province: Province) -> NaiveDate {
        let mut current = date;
        while !Self::is_working_day(current, province) {
            current = current.succ_opt().expect("valid date");
        }
        current
    }

    /// Add working days to a date
    pub fn add_working_days(date: NaiveDate, days: i32, province: Province) -> NaiveDate {
        let mut current = date;
        let mut remaining = days.abs();

        while remaining > 0 {
            current = if days > 0 {
                current.succ_opt().expect("valid date")
            } else {
                current.pred_opt().expect("valid date")
            };
            if Self::is_working_day(current, province) {
                remaining -= 1;
            }
        }
        current
    }

    // Helper functions

    /// Calculate Easter date using the Anonymous Gregorian algorithm
    fn easter_date(year: i32) -> Option<NaiveDate> {
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

    /// Get the first Monday of a month
    fn first_monday_of_month(year: i32, month: u32) -> Option<NaiveDate> {
        Self::nth_weekday_of_month(year, month, Weekday::Mon, 1)
    }

    /// Get the nth occurrence of a weekday in a month
    fn nth_weekday_of_month(year: i32, month: u32, weekday: Weekday, n: u32) -> Option<NaiveDate> {
        let first = NaiveDate::from_ymd_opt(year, month, 1)?;
        let first_weekday = first.weekday();
        let days_until = (weekday.num_days_from_monday() as i32
            - first_weekday.num_days_from_monday() as i32
            + 7)
            % 7;
        let day = 1 + days_until as u32 + (n - 1) * 7;
        NaiveDate::from_ymd_opt(year, month, day)
    }

    /// Get the Monday before a specific date
    fn monday_before(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        let target = NaiveDate::from_ymd_opt(year, month, day)?;
        let days_since_monday = target.weekday().num_days_from_monday();
        if days_since_monday == 0 {
            // If target is Monday, go back 7 days
            target
                .pred_opt()?
                .pred_opt()?
                .pred_opt()?
                .pred_opt()?
                .pred_opt()?
                .pred_opt()?
                .pred_opt()
        } else {
            // Go back to previous Monday
            let mut current = target;
            for _ in 0..days_since_monday {
                current = current.pred_opt()?;
            }
            Some(current)
        }
    }
}

// ============================================================================
// Timezone
// ============================================================================

/// Canadian time zones
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanadianTimeZone {
    /// Pacific Time (BC, Yukon)
    Pacific,
    /// Mountain Time (Alberta, parts of BC, NWT, Nunavut)
    Mountain,
    /// Central Time (Saskatchewan, Manitoba, parts of Ontario, Nunavut)
    Central,
    /// Eastern Time (Ontario, Quebec)
    Eastern,
    /// Atlantic Time (New Brunswick, Nova Scotia, PEI)
    Atlantic,
    /// Newfoundland Time (Newfoundland and Labrador)
    Newfoundland,
}

impl CanadianTimeZone {
    /// Get the primary time zone for a province
    pub fn for_province(province: Province) -> Self {
        match province {
            Province::BritishColumbia | Province::Yukon => Self::Pacific,
            Province::Alberta | Province::NorthwestTerritories => Self::Mountain,
            Province::Saskatchewan | Province::Manitoba => Self::Central,
            Province::Ontario | Province::Quebec | Province::Nunavut => Self::Eastern,
            Province::NewBrunswick | Province::NovaScotia | Province::PrinceEdwardIsland => {
                Self::Atlantic
            }
            Province::NewfoundlandLabrador => Self::Newfoundland,
        }
    }

    /// UTC offset in hours (standard time)
    pub fn utc_offset_hours(&self) -> i32 {
        match self {
            Self::Pacific => -8,
            Self::Mountain => -7,
            Self::Central => -6,
            Self::Eastern => -5,
            Self::Atlantic => -4,
            Self::Newfoundland => -3, // Actually -3:30 but simplified
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canada_day_2025() {
        let holidays = CanadianCalendar::federal_holidays(2025);
        let canada_day = holidays
            .iter()
            .find(|h| h.name == "Canada Day")
            .expect("Canada Day should exist");
        // July 1, 2025 is Tuesday, so it stays July 1
        assert_eq!(
            canada_day.date,
            NaiveDate::from_ymd_opt(2025, 7, 1).expect("valid date")
        );
    }

    #[test]
    fn test_quebec_holidays() {
        let holidays = CanadianCalendar::all_holidays(2025, Province::Quebec);
        let names: Vec<_> = holidays.iter().map(|h| h.name.as_str()).collect();
        assert!(names.contains(&"Saint-Jean-Baptiste Day"));
        assert!(names.contains(&"National Patriots' Day"));
    }

    #[test]
    fn test_working_day() {
        // January 1, 2025 is a holiday
        let new_year = NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date");
        assert!(!CanadianCalendar::is_working_day(
            new_year,
            Province::Ontario
        ));

        // January 2, 2025 is Thursday (working day)
        let jan_2 = NaiveDate::from_ymd_opt(2025, 1, 2).expect("valid date");
        assert!(CanadianCalendar::is_working_day(jan_2, Province::Ontario));
    }

    #[test]
    fn test_timezone() {
        assert_eq!(
            CanadianTimeZone::for_province(Province::Ontario),
            CanadianTimeZone::Eastern
        );
        assert_eq!(
            CanadianTimeZone::for_province(Province::BritishColumbia),
            CanadianTimeZone::Pacific
        );
    }

    #[test]
    fn test_add_working_days() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 2).expect("valid date"); // Thursday
        let result = CanadianCalendar::add_working_days(start, 5, Province::Ontario);
        // Skip weekend, should be Thursday Jan 9
        assert_eq!(result.weekday(), Weekday::Thu);
    }
}
