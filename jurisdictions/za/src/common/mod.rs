//! Common Utilities for South African Legal System
//!
//! Provides shared types and functions including:
//! - South African Rand (ZAR) currency formatting
//! - Public holidays
//! - CCMA (Commission for Conciliation, Mediation and Arbitration) regions

pub mod dates;

pub use dates::{
    SouthAfricanHoliday, SouthAfricanHolidayType, get_public_holidays, is_public_holiday,
    is_working_day, working_days_between,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// South African Rand (ZAR) currency representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Zar(i64);

impl Zar {
    /// Create a new ZAR amount from cents
    pub fn from_cents(cents: i64) -> Self {
        Self(cents)
    }

    /// Create from full rands
    pub fn from_rands(rands: i64) -> Self {
        Self(rands * 100)
    }

    /// Create from thousands (common for salaries)
    pub fn from_thousands(thousands: i64) -> Self {
        Self(thousands * 100_000)
    }

    /// Get amount in cents
    pub fn cents(&self) -> i64 {
        self.0
    }

    /// Get amount in rands (truncated)
    pub fn rands(&self) -> i64 {
        self.0 / 100
    }

    /// Format as standard currency
    /// Example: R 15,000.00
    pub fn format(&self) -> String {
        let rands = self.0 / 100;
        let cents = (self.0 % 100).abs();
        format!("R {}.{:02}", format_with_spaces(rands), cents)
    }

    /// Format as short currency (no cents)
    /// Example: R15,000
    pub fn format_short(&self) -> String {
        format!("R{}", format_with_spaces(self.rands()))
    }
}

impl fmt::Display for Zar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl std::ops::Add for Zar {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Zar {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/// Format number with space thousand separators (South African style)
fn format_with_spaces(n: i64) -> String {
    let s = n.abs().to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(' ');
        }
        result.push(*c);
    }

    if n < 0 {
        format!("-{}", result)
    } else {
        result
    }
}

/// National Minimum Wage (2024 rates)
/// Updated annually, effective 1 March each year
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MinimumWageCategory {
    /// Standard workers (most sectors)
    Standard,
    /// Farm workers
    FarmWorker,
    /// Domestic workers
    DomesticWorker,
    /// Expanded Public Works Programme (EPWP)
    Epwp,
}

impl MinimumWageCategory {
    /// Get hourly minimum wage (2024 rates)
    pub fn hourly_rate_2024(&self) -> Zar {
        match self {
            Self::Standard => Zar::from_cents(2732),       // R27.32/hour
            Self::FarmWorker => Zar::from_cents(2732),     // R27.32/hour (now same as standard)
            Self::DomesticWorker => Zar::from_cents(2732), // R27.32/hour (now same as standard)
            Self::Epwp => Zar::from_cents(1550),           // R15.50/hour (special rate)
        }
    }

    /// Get monthly minimum wage (45 hours/week standard)
    pub fn monthly_rate_2024(&self) -> Zar {
        // 45 hours * 4.33 weeks = ~195 hours/month
        let hourly = self.hourly_rate_2024().cents();
        Zar::from_cents(hourly * 195)
    }
}

/// CCMA Regions for dispute resolution
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CcmaRegion {
    /// Central (Gauteng, Limpopo, North West, Mpumalanga)
    Central,
    /// Eastern (Eastern Cape)
    Eastern,
    /// KwaZulu-Natal
    KwaZuluNatal,
    /// Northern Cape/Free State
    NorthernCape,
    /// Western Cape
    WesternCape,
}

impl CcmaRegion {
    /// Get provinces covered by this CCMA region
    pub fn provinces(&self) -> Vec<&'static str> {
        match self {
            Self::Central => vec!["Gauteng", "Limpopo", "North West", "Mpumalanga"],
            Self::Eastern => vec!["Eastern Cape"],
            Self::KwaZuluNatal => vec!["KwaZulu-Natal"],
            Self::NorthernCape => vec!["Northern Cape", "Free State"],
            Self::WesternCape => vec!["Western Cape"],
        }
    }
}

/// UIF (Unemployment Insurance Fund) rates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UifContribution {
    /// Employee contribution (1%)
    pub employee_percent: f64,
    /// Employer contribution (1%)
    pub employer_percent: f64,
    /// Total contribution
    pub total_percent: f64,
}

impl UifContribution {
    /// Current UIF rates
    pub fn current() -> Self {
        Self {
            employee_percent: 1.0,
            employer_percent: 1.0,
            total_percent: 2.0,
        }
    }

    /// Calculate monthly contribution
    pub fn calculate_monthly(&self, monthly_salary: Zar) -> (Zar, Zar) {
        let employee = Zar::from_cents((monthly_salary.cents() as f64 * 0.01) as i64);
        let employer = Zar::from_cents((monthly_salary.cents() as f64 * 0.01) as i64);
        (employee, employer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zar_creation() {
        let zar = Zar::from_rands(1000);
        assert_eq!(zar.rands(), 1000);
        assert_eq!(zar.cents(), 100000);
    }

    #[test]
    fn test_zar_formatting() {
        let zar = Zar::from_rands(15000);
        assert_eq!(zar.format(), "R 15 000.00");
        assert_eq!(zar.format_short(), "R15 000");
    }

    #[test]
    fn test_zar_from_thousands() {
        let salary = Zar::from_thousands(25);
        assert_eq!(salary.rands(), 25000);
    }

    #[test]
    fn test_minimum_wage() {
        let standard = MinimumWageCategory::Standard;
        assert!(standard.hourly_rate_2024().cents() > 0);
        assert!(standard.monthly_rate_2024().rands() > 4000);
    }

    #[test]
    fn test_ccma_regions() {
        let central = CcmaRegion::Central;
        let provinces = central.provinces();
        assert!(provinces.contains(&"Gauteng"));
    }

    #[test]
    fn test_uif_contribution() {
        let uif = UifContribution::current();
        let salary = Zar::from_rands(20000);
        let (employee, employer) = uif.calculate_monthly(salary);
        assert_eq!(employee.rands(), 200);
        assert_eq!(employer.rands(), 200);
    }
}
