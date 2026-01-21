//! Common Utilities for UAE Legal System
//!
//! Provides shared types and functions including:
//! - AED (UAE Dirham) currency formatting
//! - UAE public holidays (Islamic calendar-based)
//! - Emirates (Imarat) information

pub mod dates;

pub use dates::{
    UaeHoliday, UaeHolidayType, get_public_holidays, is_public_holiday, is_working_day,
    working_days_between,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// UAE Dirham (AED) currency representation
///
/// The UAE Dirham is pegged to the USD at 3.6725 AED per USD.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Aed(i64);

impl Aed {
    /// Create a new AED amount from fils (smallest unit)
    /// 1 AED = 100 fils
    pub fn from_fils(fils: i64) -> Self {
        Self(fils)
    }

    /// Create from full dirhams
    pub fn from_dirhams(dirhams: i64) -> Self {
        Self(dirhams * 100)
    }

    /// Create from thousands of dirhams (common for salaries)
    pub fn from_thousands(thousands: i64) -> Self {
        Self(thousands * 100_000)
    }

    /// Get amount in fils
    pub fn fils(&self) -> i64 {
        self.0
    }

    /// Get amount in dirhams (truncated)
    pub fn dirhams(&self) -> i64 {
        self.0 / 100
    }

    /// Format as Arabic style with locale
    /// Example: 10,000.00 د.إ
    pub fn format_ar(&self) -> String {
        let dirhams = self.0 / 100;
        let fils = (self.0 % 100).abs();
        format!("{},{:02} د.إ", format_with_commas(dirhams), fils)
    }

    /// Format as English style
    /// Example: AED 10,000.00
    pub fn format_en(&self) -> String {
        let dirhams = self.0 / 100;
        let fils = (self.0 % 100).abs();
        format!("AED {}.{:02}", format_with_commas(dirhams), fils)
    }

    /// Approximate USD value (at pegged rate of 3.6725)
    pub fn to_usd_approximate(&self) -> f64 {
        self.dirhams() as f64 / 3.6725
    }
}

impl fmt::Display for Aed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_en())
    }
}

impl std::ops::Add for Aed {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Aed {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/// Format number with thousand separators (commas)
fn format_with_commas(n: i64) -> String {
    let s = n.abs().to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*c);
    }

    if n < 0 {
        format!("-{}", result)
    } else {
        result
    }
}

/// UAE Labour Skill Levels
///
/// Used for determining minimum wage and visa categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillLevel {
    /// Unskilled/Elementary (Skill Level 4)
    Elementary,
    /// Semi-skilled/Services (Skill Level 3)
    Skilled,
    /// Skilled/Technical (Skill Level 2)
    Technical,
    /// Professional/Managerial (Skill Level 1)
    Professional,
}

impl SkillLevel {
    /// Get minimum monthly salary for skill level (approximate 2024 guidelines)
    /// Note: UAE does not have a statutory minimum wage except for certain categories
    pub fn typical_minimum_salary(&self) -> Aed {
        match self {
            Self::Elementary => Aed::from_dirhams(1_500), // Basic entry level
            Self::Skilled => Aed::from_dirhams(3_000),    // Service workers
            Self::Technical => Aed::from_dirhams(5_000),  // Technical/Trade
            Self::Professional => Aed::from_dirhams(10_000), // Professional/Managerial
        }
    }
}

/// UAE Free Zone types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FreeZone {
    /// Dubai International Financial Centre
    Difc,
    /// Abu Dhabi Global Market
    Adgm,
    /// Dubai Multi Commodities Centre
    Dmcc,
    /// Jebel Ali Free Zone
    Jafza,
    /// Dubai Silicon Oasis
    Dso,
    /// Dubai Media City
    DubaiMediaCity,
    /// Dubai Internet City
    DubaiInternetCity,
    /// Dubai Healthcare City
    DubaiHealthcareCity,
    /// Abu Dhabi Airport Free Zone
    Adafz,
    /// Sharjah Airport Free Zone
    Saif,
    /// Ras Al Khaimah Economic Zone
    Rakez,
    /// Other free zone
    Other(String),
}

impl FreeZone {
    /// Check if free zone uses Common Law (English law)
    pub fn uses_common_law(&self) -> bool {
        matches!(self, Self::Difc | Self::Adgm)
    }

    /// Get the jurisdiction name
    pub fn jurisdiction_name(&self) -> &str {
        match self {
            Self::Difc => "DIFC Courts",
            Self::Adgm => "ADGM Courts",
            _ => "UAE Federal Courts",
        }
    }

    /// Get typical visa processing time (days)
    pub fn typical_visa_days(&self) -> u32 {
        match self {
            Self::Difc | Self::Adgm => 5,  // Premium processing
            Self::Dmcc | Self::Jafza => 7, // Established zones
            _ => 10,                       // Other zones
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aed_creation() {
        let aed = Aed::from_dirhams(1000);
        assert_eq!(aed.dirhams(), 1000);
        assert_eq!(aed.fils(), 100000);
    }

    #[test]
    fn test_aed_formatting() {
        let aed = Aed::from_dirhams(10000);
        assert_eq!(aed.format_en(), "AED 10,000.00");
        assert!(aed.format_ar().contains("د.إ"));
    }

    #[test]
    fn test_aed_from_thousands() {
        let salary = Aed::from_thousands(15);
        assert_eq!(salary.dirhams(), 15000);
    }

    #[test]
    fn test_aed_usd_conversion() {
        let aed = Aed::from_dirhams(36725);
        let usd = aed.to_usd_approximate();
        // Should be approximately 10,000 USD
        assert!((usd - 10000.0).abs() < 100.0);
    }

    #[test]
    fn test_skill_levels() {
        let prof = SkillLevel::Professional;
        assert!(prof.typical_minimum_salary().dirhams() >= 10000);
    }

    #[test]
    fn test_free_zones() {
        assert!(FreeZone::Difc.uses_common_law());
        assert!(FreeZone::Adgm.uses_common_law());
        assert!(!FreeZone::Jafza.uses_common_law());
    }
}
