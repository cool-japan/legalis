//! Saudi Riyal (SAR) Currency Support
//!
//! The Saudi Riyal is the official currency of Saudi Arabia.
//! 1 SAR = 100 halalas (هللة)

use serde::{Deserialize, Serialize};
use std::fmt;

/// Saudi Riyal (SAR) currency representation
///
/// The Saudi Riyal has been pegged to USD at approximately 3.75 SAR per USD
/// since 1986.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Sar(i64);

impl Sar {
    /// Create a new SAR amount from halalas (smallest unit)
    /// 1 SAR = 100 halalas
    pub fn from_halalas(halalas: i64) -> Self {
        Self(halalas)
    }

    /// Create from full riyals
    pub fn from_riyals(riyals: i64) -> Self {
        Self(riyals * 100)
    }

    /// Create from thousands of riyals (common for salaries)
    pub fn from_thousands(thousands: i64) -> Self {
        Self(thousands * 100_000)
    }

    /// Get amount in halalas
    pub fn halalas(&self) -> i64 {
        self.0
    }

    /// Get amount in riyals (truncated)
    pub fn riyals(&self) -> i64 {
        self.0 / 100
    }

    /// Get amount as float (riyals with decimal)
    pub fn as_decimal(&self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// Format as Arabic style with locale
    /// Example: 10,000.00 ر.س
    pub fn format_ar(&self) -> String {
        let riyals = self.0 / 100;
        let halalas = (self.0 % 100).abs();
        format!("{}.{:02} ر.س", format_with_commas(riyals), halalas)
    }

    /// Format as English style
    /// Example: SAR 10,000.00
    pub fn format_en(&self) -> String {
        let riyals = self.0 / 100;
        let halalas = (self.0 % 100).abs();
        format!("SAR {}.{:02}", format_with_commas(riyals), halalas)
    }

    /// Approximate USD value (at pegged rate of 3.75)
    pub fn to_usd_approximate(&self) -> f64 {
        self.as_decimal() / 3.75
    }

    /// Check if amount is zero
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Check if amount is positive
    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    /// Check if amount is negative
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    /// Get absolute value
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }
}

impl fmt::Display for Sar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_en())
    }
}

impl std::ops::Add for Sar {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Sar {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<i64> for Sar {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Div<i64> for Sar {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sar_creation() {
        let sar = Sar::from_riyals(1000);
        assert_eq!(sar.riyals(), 1000);
        assert_eq!(sar.halalas(), 100000);
    }

    #[test]
    fn test_sar_formatting() {
        let sar = Sar::from_riyals(10000);
        assert_eq!(sar.format_en(), "SAR 10,000.00");
        assert!(sar.format_ar().contains("ر.س"));
    }

    #[test]
    fn test_sar_from_thousands() {
        let salary = Sar::from_thousands(15);
        assert_eq!(salary.riyals(), 15000);
    }

    #[test]
    fn test_sar_usd_conversion() {
        let sar = Sar::from_riyals(3750);
        let usd = sar.to_usd_approximate();
        // Should be approximately 1,000 USD
        assert!((usd - 1000.0).abs() < 1.0);
    }

    #[test]
    fn test_sar_arithmetic() {
        let a = Sar::from_riyals(100);
        let b = Sar::from_riyals(50);

        assert_eq!((a + b).riyals(), 150);
        assert_eq!((a - b).riyals(), 50);
        assert_eq!((a * 2).riyals(), 200);
        assert_eq!((a / 2).riyals(), 50);
    }

    #[test]
    fn test_sar_comparisons() {
        let a = Sar::from_riyals(100);
        let b = Sar::from_riyals(50);

        assert!(a > b);
        assert!(b < a);
        assert_eq!(a, Sar::from_riyals(100));
    }

    #[test]
    fn test_sar_predicates() {
        assert!(Sar::from_riyals(100).is_positive());
        assert!(Sar::from_riyals(-100).is_negative());
        assert!(Sar::from_riyals(0).is_zero());
    }

    #[test]
    fn test_sar_abs() {
        let sar = Sar::from_riyals(-100);
        assert_eq!(sar.abs().riyals(), 100);
    }

    #[test]
    fn test_format_with_commas() {
        assert_eq!(format_with_commas(1000), "1,000");
        assert_eq!(format_with_commas(1000000), "1,000,000");
        assert_eq!(format_with_commas(-5000), "-5,000");
    }
}
