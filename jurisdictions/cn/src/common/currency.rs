//! Chinese Currency Utilities
//!
//! Handles RMB/CNY formatting and calculations.
//!
//! # 货币工具 / Currency Utilities
//!
//! Chinese Yuan (CNY) formatting conventions:
//! - Symbol: ¥ or RMB
//! - Large amounts: 万 (wan, 10,000), 亿 (yi, 100,000,000)

use serde::{Deserialize, Serialize};
use std::fmt;

/// Currency amount in Chinese Yuan (CNY)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CnyAmount {
    /// Amount in yuan (base unit)
    pub yuan: f64,
}

impl CnyAmount {
    /// Create new amount
    pub fn new(yuan: f64) -> Self {
        Self { yuan }
    }

    /// Create from yuan (alias for new)
    pub fn from_yuan(yuan: f64) -> Self {
        Self { yuan }
    }

    /// Get yuan value
    pub fn yuan(&self) -> f64 {
        self.yuan
    }

    /// Create from fen (1/100 yuan)
    pub fn from_fen(fen: i64) -> Self {
        Self {
            yuan: fen as f64 / 100.0,
        }
    }

    /// Create from wan (万, 10,000)
    pub fn from_wan(wan: f64) -> Self {
        Self {
            yuan: wan * 10_000.0,
        }
    }

    /// Create from yi (亿, 100,000,000)
    pub fn from_yi(yi: f64) -> Self {
        Self {
            yuan: yi * 100_000_000.0,
        }
    }

    /// Convert to fen (分, 1/100 yuan)
    pub fn to_fen(&self) -> i64 {
        (self.yuan * 100.0).round() as i64
    }

    /// Convert to wan (万, 10,000)
    pub fn to_wan(&self) -> f64 {
        self.yuan / 10_000.0
    }

    /// Convert to yi (亿, 100,000,000)
    pub fn to_yi(&self) -> f64 {
        self.yuan / 100_000_000.0
    }

    /// Format as standard CNY (¥1,234.56)
    pub fn format_standard(&self) -> String {
        format!("¥{}", format_with_thousands(self.yuan))
    }

    /// Format as RMB (RMB 1,234.56)
    pub fn format_rmb(&self) -> String {
        format!("RMB {}", format_with_thousands(self.yuan))
    }

    /// Format with Chinese units (appropriate scale)
    ///
    /// - < 10,000: plain yuan
    /// - 10,000 - 100,000,000: 万元
    /// - >= 100,000,000: 亿元
    pub fn format_chinese(&self) -> String {
        if self.yuan.abs() >= 100_000_000.0 {
            format!("{:.2}亿元", self.to_yi())
        } else if self.yuan.abs() >= 10_000.0 {
            format!("{:.2}万元", self.to_wan())
        } else {
            format!("{:.2}元", self.yuan)
        }
    }

    /// Format bilingual (Chinese + English)
    pub fn format_bilingual(&self) -> String {
        format!("{} / {}", self.format_chinese(), self.format_standard())
    }

    /// Check if amount exceeds threshold
    pub fn exceeds(&self, threshold: f64) -> bool {
        self.yuan > threshold
    }

    /// Add two amounts
    pub fn add(&self, other: &CnyAmount) -> CnyAmount {
        CnyAmount::new(self.yuan + other.yuan)
    }

    /// Subtract amount
    pub fn subtract(&self, other: &CnyAmount) -> CnyAmount {
        CnyAmount::new(self.yuan - other.yuan)
    }

    /// Multiply by factor
    pub fn multiply(&self, factor: f64) -> CnyAmount {
        CnyAmount::new(self.yuan * factor)
    }
}

impl fmt::Display for CnyAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_standard())
    }
}

impl Default for CnyAmount {
    fn default() -> Self {
        Self { yuan: 0.0 }
    }
}

impl std::ops::Mul<f64> for CnyAmount {
    type Output = CnyAmount;

    fn mul(self, rhs: f64) -> Self::Output {
        CnyAmount::new(self.yuan * rhs)
    }
}

/// Format a number with thousand separators
fn format_with_thousands(value: f64) -> String {
    let formatted = format!("{:.2}", value);
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = parts.get(1).unwrap_or(&"00");

    let negative = integer_part.starts_with('-');
    let digits: String = integer_part
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    // Build the string with thousand separators
    let chars: Vec<char> = digits.chars().collect();
    let mut result = String::new();
    let len = chars.len();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*c);
    }

    if negative {
        format!("-{}.{}", result, decimal_part)
    } else {
        format!("{}.{}", result, decimal_part)
    }
}

/// Common monetary thresholds in Chinese law
pub mod thresholds {
    use super::CnyAmount;

    /// PIPL sensitive PI threshold for security assessment (Article 40)
    /// Processing sensitive PI of 100,000+ individuals requires assessment
    pub const PIPL_SENSITIVE_PI_COUNT: u64 = 100_000;

    /// PIPL personal information threshold for security assessment
    /// Processing PI of 1,000,000+ individuals requires assessment
    pub const PIPL_PI_COUNT_THRESHOLD: u64 = 1_000_000;

    /// Small company registered capital threshold
    pub fn small_company_capital() -> CnyAmount {
        CnyAmount::from_wan(30.0) // 30万元
    }

    /// Large company annual revenue threshold
    pub fn large_company_revenue() -> CnyAmount {
        CnyAmount::from_yi(1.0) // 1亿元
    }

    /// Anti-monopoly turnover threshold for merger filing (domestic)
    pub fn merger_domestic_threshold() -> CnyAmount {
        CnyAmount::from_yi(4.0) // 4亿元 combined turnover
    }

    /// Anti-monopoly turnover threshold for merger filing (single operator)
    pub fn merger_single_operator_threshold() -> CnyAmount {
        CnyAmount::from_yi(0.4) // 4000万元 = 0.4亿元
    }

    /// Individual income tax exemption threshold (monthly)
    pub fn individual_tax_exemption_monthly() -> CnyAmount {
        CnyAmount::new(5_000.0) // 5000元/月
    }

    /// Individual income tax exemption threshold (annual)
    pub fn individual_tax_exemption_annual() -> CnyAmount {
        CnyAmount::new(60_000.0) // 6万元/年
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cny_creation() {
        let amount = CnyAmount::new(1234.56);
        assert!((amount.yuan - 1234.56).abs() < 0.001);
    }

    #[test]
    fn test_from_wan() {
        let amount = CnyAmount::from_wan(5.0);
        assert!((amount.yuan - 50_000.0).abs() < 0.001);
    }

    #[test]
    fn test_from_yi() {
        let amount = CnyAmount::from_yi(2.5);
        assert!((amount.yuan - 250_000_000.0).abs() < 0.001);
    }

    #[test]
    fn test_to_wan() {
        let amount = CnyAmount::new(150_000.0);
        assert!((amount.to_wan() - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_to_yi() {
        let amount = CnyAmount::new(500_000_000.0);
        assert!((amount.to_yi() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_format_standard() {
        let amount = CnyAmount::new(1234567.89);
        assert_eq!(amount.format_standard(), "¥1,234,567.89");
    }

    #[test]
    fn test_format_chinese_yuan() {
        let amount = CnyAmount::new(500.0);
        assert_eq!(amount.format_chinese(), "500.00元");
    }

    #[test]
    fn test_format_chinese_wan() {
        let amount = CnyAmount::new(50_000.0);
        assert_eq!(amount.format_chinese(), "5.00万元");
    }

    #[test]
    fn test_format_chinese_yi() {
        let amount = CnyAmount::new(200_000_000.0);
        assert_eq!(amount.format_chinese(), "2.00亿元");
    }

    #[test]
    fn test_arithmetic() {
        let a = CnyAmount::new(1000.0);
        let b = CnyAmount::new(500.0);

        assert!((a.add(&b).yuan - 1500.0).abs() < 0.001);
        assert!((a.subtract(&b).yuan - 500.0).abs() < 0.001);
        assert!((a.multiply(2.0).yuan - 2000.0).abs() < 0.001);
    }

    #[test]
    fn test_thresholds() {
        let small_cap = thresholds::small_company_capital();
        assert!((small_cap.yuan - 300_000.0).abs() < 0.001);
    }
}
