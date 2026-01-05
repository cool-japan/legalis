//! Japanese era (和暦) handling.
//!
//! Provides conversion between Western calendar (西暦) and Japanese era calendar (和暦).

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Japanese era definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Era {
    /// 明治 (Meiji) era: 1868-01-25 to 1912-07-29
    Meiji,
    /// 大正 (Taisho) era: 1912-07-30 to 1926-12-24
    Taisho,
    /// 昭和 (Showa) era: 1926-12-25 to 1989-01-07
    Showa,
    /// 平成 (Heisei) era: 1989-01-08 to 2019-04-30
    Heisei,
    /// 令和 (Reiwa) era: 2019-05-01 to present
    Reiwa,
}

impl Era {
    /// Returns the Japanese name of the era.
    #[must_use]
    pub fn japanese_name(&self) -> &'static str {
        match self {
            Era::Meiji => "明治",
            Era::Taisho => "大正",
            Era::Showa => "昭和",
            Era::Heisei => "平成",
            Era::Reiwa => "令和",
        }
    }

    /// Returns the romaji name of the era.
    #[must_use]
    pub fn romaji(&self) -> &'static str {
        match self {
            Era::Meiji => "Meiji",
            Era::Taisho => "Taisho",
            Era::Showa => "Showa",
            Era::Heisei => "Heisei",
            Era::Reiwa => "Reiwa",
        }
    }

    /// Returns the single-character abbreviation (used in official documents).
    #[must_use]
    pub fn abbreviation(&self) -> char {
        match self {
            Era::Meiji => 'M',
            Era::Taisho => 'T',
            Era::Showa => 'S',
            Era::Heisei => 'H',
            Era::Reiwa => 'R',
        }
    }

    /// Returns the start date of the era.
    #[must_use]
    pub fn start_date(&self) -> NaiveDate {
        match self {
            Era::Meiji => NaiveDate::from_ymd_opt(1868, 1, 25).unwrap(),
            Era::Taisho => NaiveDate::from_ymd_opt(1912, 7, 30).unwrap(),
            Era::Showa => NaiveDate::from_ymd_opt(1926, 12, 25).unwrap(),
            Era::Heisei => NaiveDate::from_ymd_opt(1989, 1, 8).unwrap(),
            Era::Reiwa => NaiveDate::from_ymd_opt(2019, 5, 1).unwrap(),
        }
    }

    /// Returns the end date of the era (None for current era).
    #[must_use]
    pub fn end_date(&self) -> Option<NaiveDate> {
        match self {
            Era::Meiji => Some(NaiveDate::from_ymd_opt(1912, 7, 29).unwrap()),
            Era::Taisho => Some(NaiveDate::from_ymd_opt(1926, 12, 24).unwrap()),
            Era::Showa => Some(NaiveDate::from_ymd_opt(1989, 1, 7).unwrap()),
            Era::Heisei => Some(NaiveDate::from_ymd_opt(2019, 4, 30).unwrap()),
            Era::Reiwa => None,
        }
    }

    /// Determines the era for a given date.
    pub fn from_date(date: NaiveDate) -> Result<Self, EraError> {
        if date >= NaiveDate::from_ymd_opt(2019, 5, 1).unwrap() {
            Ok(Era::Reiwa)
        } else if date >= NaiveDate::from_ymd_opt(1989, 1, 8).unwrap() {
            Ok(Era::Heisei)
        } else if date >= NaiveDate::from_ymd_opt(1926, 12, 25).unwrap() {
            Ok(Era::Showa)
        } else if date >= NaiveDate::from_ymd_opt(1912, 7, 30).unwrap() {
            Ok(Era::Taisho)
        } else if date >= NaiveDate::from_ymd_opt(1868, 1, 25).unwrap() {
            Ok(Era::Meiji)
        } else {
            Err(EraError::DateBeforeMeiji(date))
        }
    }

    /// Returns all eras in chronological order.
    #[must_use]
    pub fn all() -> &'static [Era] {
        &[Era::Meiji, Era::Taisho, Era::Showa, Era::Heisei, Era::Reiwa]
    }
}

impl std::fmt::Display for Era {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.japanese_name())
    }
}

/// A date in Japanese era format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct JapaneseDate {
    /// The era
    pub era: Era,
    /// Year within the era (1-indexed)
    pub year: u32,
    /// Month (1-12)
    pub month: u32,
    /// Day (1-31)
    pub day: u32,
}

impl JapaneseDate {
    /// Creates a new Japanese date.
    pub fn new(era: Era, year: u32, month: u32, day: u32) -> Result<Self, EraError> {
        let date = Self {
            era,
            year,
            month,
            day,
        };
        // Validate by converting to Western date
        date.to_western()?;
        Ok(date)
    }

    /// Parses a Japanese date from string (e.g., "令和6年4月1日" or "R6.4.1").
    pub fn parse(s: &str) -> Result<Self, EraError> {
        // Try abbreviated format first (R6.4.1)
        if let Some(result) = Self::parse_abbreviated(s) {
            return result;
        }

        // Try full Japanese format (令和6年4月1日)
        if let Some(result) = Self::parse_japanese(s) {
            return result;
        }

        Err(EraError::InvalidFormat(s.to_string()))
    }

    fn parse_abbreviated(s: &str) -> Option<Result<Self, EraError>> {
        let s = s.trim();
        if s.len() < 2 {
            return None;
        }

        let era_char = s.chars().next()?;
        let era = match era_char {
            'M' | 'm' => Era::Meiji,
            'T' | 't' => Era::Taisho,
            'S' | 's' => Era::Showa,
            'H' | 'h' => Era::Heisei,
            'R' | 'r' => Era::Reiwa,
            _ => return None,
        };

        let rest = &s[1..];
        let parts: Vec<&str> = rest.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        let year: u32 = parts[0].parse().ok()?;
        let month: u32 = parts[1].parse().ok()?;
        let day: u32 = parts[2].parse().ok()?;

        Some(Self::new(era, year, month, day))
    }

    fn parse_japanese(s: &str) -> Option<Result<Self, EraError>> {
        let s = s.trim();

        let era = if s.starts_with("明治") {
            Era::Meiji
        } else if s.starts_with("大正") {
            Era::Taisho
        } else if s.starts_with("昭和") {
            Era::Showa
        } else if s.starts_with("平成") {
            Era::Heisei
        } else if s.starts_with("令和") {
            Era::Reiwa
        } else {
            return None;
        };

        // Remove era name
        let rest = &s[era.japanese_name().len()..];

        // Parse year (元年 = year 1, or numeric)
        let (year, rest) = if let Some(stripped) = rest.strip_prefix("元年") {
            (1, stripped)
        } else {
            let year_end = rest.find('年')?;
            let year_str = &rest[..year_end];
            let year: u32 = year_str.parse().ok()?;
            (year, &rest[year_end + "年".len()..])
        };

        // Parse month
        let month_end = rest.find('月')?;
        let month_str = &rest[..month_end];
        let month: u32 = month_str.parse().ok()?;
        let rest = &rest[month_end + "月".len()..];

        // Parse day
        let day_end = rest.find('日')?;
        let day_str = &rest[..day_end];
        let day: u32 = day_str.parse().ok()?;

        Some(Self::new(era, year, month, day))
    }

    /// Converts from a Western date.
    pub fn from_western(date: NaiveDate) -> Result<Self, EraError> {
        let era = Era::from_date(date)?;
        let era_start = era.start_date();
        let year = (date.year() - era_start.year()) as u32 + 1;

        Ok(Self {
            era,
            year,
            month: date.month(),
            day: date.day(),
        })
    }

    /// Converts to a Western date.
    pub fn to_western(&self) -> Result<NaiveDate, EraError> {
        let era_start = self.era.start_date();
        let western_year = era_start.year() + (self.year as i32) - 1;

        NaiveDate::from_ymd_opt(western_year, self.month, self.day).ok_or(EraError::InvalidDate {
            era: self.era,
            year: self.year,
            month: self.month,
            day: self.day,
        })
    }

    /// Returns the date in Japanese format (e.g., "令和6年4月1日").
    #[must_use]
    pub fn to_japanese_string(&self) -> String {
        if self.year == 1 {
            format!(
                "{}元年{}月{}日",
                self.era.japanese_name(),
                self.month,
                self.day
            )
        } else {
            format!(
                "{}{}年{}月{}日",
                self.era.japanese_name(),
                self.year,
                self.month,
                self.day
            )
        }
    }

    /// Returns the date in abbreviated format (e.g., "R6.4.1").
    #[must_use]
    pub fn to_abbreviated_string(&self) -> String {
        format!(
            "{}{}.{}.{}",
            self.era.abbreviation(),
            self.year,
            self.month,
            self.day
        )
    }
}

impl std::fmt::Display for JapaneseDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_japanese_string())
    }
}

/// Errors related to Japanese era handling.
#[derive(Debug, Error)]
pub enum EraError {
    /// Date is before the Meiji era.
    #[error("Date {0} is before the Meiji era (1868-01-25)")]
    DateBeforeMeiji(NaiveDate),

    /// Invalid date components.
    #[error("Invalid date: {era} {year}年{month}月{day}日")]
    InvalidDate {
        era: Era,
        year: u32,
        month: u32,
        day: u32,
    },

    /// Invalid date format.
    #[error("Invalid date format: {0}")]
    InvalidFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_era_from_date() {
        let date = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        assert_eq!(Era::from_date(date).unwrap(), Era::Reiwa);

        let date = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();
        assert_eq!(Era::from_date(date).unwrap(), Era::Heisei);

        let date = NaiveDate::from_ymd_opt(1950, 1, 1).unwrap();
        assert_eq!(Era::from_date(date).unwrap(), Era::Showa);
    }

    #[test]
    fn test_japanese_date_conversion() {
        let date = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        let jp_date = JapaneseDate::from_western(date).unwrap();

        assert_eq!(jp_date.era, Era::Reiwa);
        assert_eq!(jp_date.year, 6);
        assert_eq!(jp_date.month, 4);
        assert_eq!(jp_date.day, 1);

        assert_eq!(jp_date.to_western().unwrap(), date);
    }

    #[test]
    fn test_japanese_date_string() {
        let date = JapaneseDate::new(Era::Reiwa, 6, 4, 1).unwrap();
        assert_eq!(date.to_japanese_string(), "令和6年4月1日");
        assert_eq!(date.to_abbreviated_string(), "R6.4.1");

        let date = JapaneseDate::new(Era::Reiwa, 1, 5, 1).unwrap();
        assert_eq!(date.to_japanese_string(), "令和元年5月1日");
    }

    #[test]
    fn test_parse_japanese_date() {
        let date = JapaneseDate::parse("令和6年4月1日").unwrap();
        assert_eq!(date.era, Era::Reiwa);
        assert_eq!(date.year, 6);

        let date = JapaneseDate::parse("R6.4.1").unwrap();
        assert_eq!(date.era, Era::Reiwa);
        assert_eq!(date.year, 6);
    }

    #[test]
    fn test_showa_constitution() {
        // Japanese Constitution promulgation: November 3, Showa 21 (1946)
        let date = JapaneseDate::new(Era::Showa, 21, 11, 3).unwrap();
        let western = date.to_western().unwrap();
        assert_eq!(western, NaiveDate::from_ymd_opt(1946, 11, 3).unwrap());
    }
}
