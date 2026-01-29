//! Korean Currency Utilities
//!
//! Handles KRW formatting and calculations.
//!
//! # 통화 유틸리티 / Currency Utilities
//!
//! Korean Won (KRW) formatting conventions:
//! - Symbol: ₩ or 원
//! - Large amounts: 만 (man, 10,000), 억 (eok, 100,000,000), 조 (jo, 1,000,000,000,000)

use serde::{Deserialize, Serialize};
use std::fmt;

/// Currency amount in Korean Won (KRW)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct KrwAmount {
    /// Amount in won (base unit)
    pub won: f64,
}

impl KrwAmount {
    /// Create new amount
    pub fn new(won: f64) -> Self {
        Self { won }
    }

    /// Create from won (alias for new)
    pub fn from_won(won: f64) -> Self {
        Self { won }
    }

    /// Get won value
    pub fn won(&self) -> f64 {
        self.won
    }

    /// Create from man (만, 10,000)
    pub fn from_man(man: f64) -> Self {
        Self {
            won: man * 10_000.0,
        }
    }

    /// Create from eok (억, 100,000,000)
    pub fn from_eok(eok: f64) -> Self {
        Self {
            won: eok * 100_000_000.0,
        }
    }

    /// Create from jo (조, 1,000,000,000,000)
    pub fn from_jo(jo: f64) -> Self {
        Self {
            won: jo * 1_000_000_000_000.0,
        }
    }

    /// Convert to man (만, 10,000)
    pub fn to_man(&self) -> f64 {
        self.won / 10_000.0
    }

    /// Convert to eok (억, 100,000,000)
    pub fn to_eok(&self) -> f64 {
        self.won / 100_000_000.0
    }

    /// Convert to jo (조, 1,000,000,000,000)
    pub fn to_jo(&self) -> f64 {
        self.won / 1_000_000_000_000.0
    }

    /// Format as standard KRW (₩1,234,567)
    pub fn format_standard(&self) -> String {
        format!("₩{}", format_with_thousands(self.won as i64))
    }

    /// Format as KRW with won symbol (1,234,567원)
    pub fn format_won(&self) -> String {
        format!("{}원", format_with_thousands(self.won as i64))
    }

    /// Format with Korean units (appropriate scale)
    ///
    /// - < 10,000: plain won
    /// - 10,000 - 100,000,000: 만원
    /// - 100,000,000 - 1,000,000,000,000: 억원
    /// - >= 1,000,000,000,000: 조원
    pub fn format_korean(&self) -> String {
        if self.won.abs() >= 1_000_000_000_000.0 {
            format!("{:.2}조원", self.to_jo())
        } else if self.won.abs() >= 100_000_000.0 {
            format!("{:.2}억원", self.to_eok())
        } else if self.won.abs() >= 10_000.0 {
            format!("{:.2}만원", self.to_man())
        } else {
            format!("{:.0}원", self.won)
        }
    }

    /// Format bilingual (Korean + English)
    pub fn format_bilingual(&self) -> String {
        format!("{} / {}", self.format_korean(), self.format_standard())
    }

    /// Check if amount exceeds threshold
    pub fn exceeds(&self, threshold: f64) -> bool {
        self.won > threshold
    }

    /// Add two amounts
    pub fn add(&self, other: &KrwAmount) -> KrwAmount {
        KrwAmount::new(self.won + other.won)
    }

    /// Subtract amount
    pub fn subtract(&self, other: &KrwAmount) -> KrwAmount {
        KrwAmount::new(self.won - other.won)
    }

    /// Multiply by factor
    pub fn multiply(&self, factor: f64) -> KrwAmount {
        KrwAmount::new(self.won * factor)
    }
}

impl fmt::Display for KrwAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_standard())
    }
}

impl Default for KrwAmount {
    fn default() -> Self {
        Self { won: 0.0 }
    }
}

impl std::ops::Mul<f64> for KrwAmount {
    type Output = KrwAmount;

    fn mul(self, rhs: f64) -> Self::Output {
        KrwAmount::new(self.won * rhs)
    }
}

/// Format a number with thousand separators
fn format_with_thousands(value: i64) -> String {
    let negative = value < 0;
    let abs_value = value.abs();
    let string = abs_value.to_string();
    let chars: Vec<char> = string.chars().collect();
    let len = chars.len();

    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*c);
    }

    if negative {
        format!("-{}", result)
    } else {
        result
    }
}

/// Common monetary thresholds in Korean law
pub mod thresholds {
    use super::KrwAmount;

    /// Minimum wage threshold (2024: approximately 9,860 KRW/hour)
    pub fn minimum_wage_hourly() -> KrwAmount {
        KrwAmount::new(9_860.0)
    }

    /// Minimum monthly salary (based on 209 hours/month)
    pub fn minimum_wage_monthly() -> KrwAmount {
        KrwAmount::new(2_060_740.0) // 9,860 × 209
    }

    /// Small company capital threshold
    pub fn small_company_capital() -> KrwAmount {
        KrwAmount::from_man(1_000.0) // 1000만원
    }

    /// Large company annual revenue threshold
    pub fn large_company_revenue() -> KrwAmount {
        KrwAmount::from_eok(100.0) // 100억원
    }

    /// Fair Trade Act merger filing threshold
    pub fn merger_filing_threshold() -> KrwAmount {
        KrwAmount::from_eok(300.0) // 300억원
    }

    /// Severance pay calculation base
    pub fn severance_pay_base() -> KrwAmount {
        KrwAmount::new(30.0) // 30 days of average wage
    }

    /// Personal information protection administrative fine max
    pub fn pipa_max_admin_fine() -> KrwAmount {
        KrwAmount::from_man(5_000.0) // 5000만원
    }

    /// Personal information protection penalty max
    pub fn pipa_max_penalty() -> KrwAmount {
        KrwAmount::from_eok(5.0) // 5억원 or 3% of revenue
    }

    /// Corporate tax minimum threshold
    pub fn corporate_tax_threshold() -> KrwAmount {
        KrwAmount::from_eok(2.0) // 2억원
    }

    /// VAT registration threshold
    pub fn vat_registration_threshold() -> KrwAmount {
        KrwAmount::from_man(4_800.0) // 4800만원
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_krw_creation() {
        let amount = KrwAmount::new(123_456.0);
        assert!((amount.won - 123_456.0).abs() < 0.001);
    }

    #[test]
    fn test_from_man() {
        let amount = KrwAmount::from_man(5.0);
        assert!((amount.won - 50_000.0).abs() < 0.001);
    }

    #[test]
    fn test_from_eok() {
        let amount = KrwAmount::from_eok(2.5);
        assert!((amount.won - 250_000_000.0).abs() < 0.001);
    }

    #[test]
    fn test_from_jo() {
        let amount = KrwAmount::from_jo(1.5);
        assert!((amount.won - 1_500_000_000_000.0).abs() < 0.001);
    }

    #[test]
    fn test_to_man() {
        let amount = KrwAmount::new(150_000.0);
        assert!((amount.to_man() - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_to_eok() {
        let amount = KrwAmount::new(500_000_000.0);
        assert!((amount.to_eok() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_format_standard() {
        let amount = KrwAmount::new(1_234_567.0);
        assert_eq!(amount.format_standard(), "₩1,234,567");
    }

    #[test]
    fn test_format_won() {
        let amount = KrwAmount::new(1_234_567.0);
        assert_eq!(amount.format_won(), "1,234,567원");
    }

    #[test]
    fn test_format_korean_won() {
        let amount = KrwAmount::new(500.0);
        assert_eq!(amount.format_korean(), "500원");
    }

    #[test]
    fn test_format_korean_man() {
        let amount = KrwAmount::new(50_000.0);
        assert_eq!(amount.format_korean(), "5.00만원");
    }

    #[test]
    fn test_format_korean_eok() {
        let amount = KrwAmount::new(200_000_000.0);
        assert_eq!(amount.format_korean(), "2.00억원");
    }

    #[test]
    fn test_format_korean_jo() {
        let amount = KrwAmount::new(1_500_000_000_000.0);
        assert_eq!(amount.format_korean(), "1.50조원");
    }

    #[test]
    fn test_arithmetic() {
        let a = KrwAmount::new(1000.0);
        let b = KrwAmount::new(500.0);

        assert!((a.add(&b).won - 1500.0).abs() < 0.001);
        assert!((a.subtract(&b).won - 500.0).abs() < 0.001);
        assert!((a.multiply(2.0).won - 2000.0).abs() < 0.001);
    }

    #[test]
    fn test_thresholds() {
        let min_wage = thresholds::minimum_wage_hourly();
        assert!((min_wage.won - 9_860.0).abs() < 0.001);
    }
}
