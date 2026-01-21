//! Indian Rupee (INR) Currency Formatting
//!
//! India uses a unique numbering system:
//! - **Lakh**: 1,00,000 (one hundred thousand)
//! - **Crore**: 1,00,00,000 (ten million)
//!
//! ## Indian Number Formatting
//!
//! Indian number formatting uses the following pattern:
//! - First separator after 3 digits from right
//! - Then separators after every 2 digits
//!
//! Examples:
//! - 1,234 (one thousand)
//! - 12,345 (twelve thousand)
//! - 1,23,456 (one lakh twenty-three thousand)
//! - 12,34,567 (twelve lakh thirty-four thousand)
//! - 1,23,45,678 (one crore twenty-three lakh)
//!
//! ## Usage
//!
//! ```rust
//! use legalis_in::common::currency::*;
//!
//! // Format amounts in Indian style
//! let amount = 5_000_000.0;
//! assert_eq!(format_inr(amount), "₹50,00,000.00");
//! assert_eq!(to_lakhs(amount), 50.0);
//! assert_eq!(to_crores(amount), 0.5);
//!
//! // Format with custom precision
//! assert_eq!(format_inr_precision(1_234_567.89, 2), "₹12,34,567.89");
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Indian Rupee amount (stored in paise internally, 1 INR = 100 paise)
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct InrAmount {
    /// Amount in paise (1 INR = 100 paise)
    paise: i64,
}

impl InrAmount {
    /// Create a new INR amount from rupees
    pub fn from_rupees(rupees: f64) -> Self {
        Self {
            paise: (rupees * 100.0).round() as i64,
        }
    }

    /// Create from paise
    pub fn from_paise(paise: i64) -> Self {
        Self { paise }
    }

    /// Get amount in rupees
    pub fn to_rupees(&self) -> f64 {
        self.paise as f64 / 100.0
    }

    /// Get amount in paise
    pub fn to_paise(&self) -> i64 {
        self.paise
    }

    /// Convert to lakhs (1 lakh = 1,00,000)
    pub fn to_lakhs(&self) -> f64 {
        self.to_rupees() / 100_000.0
    }

    /// Convert to crores (1 crore = 1,00,00,000)
    pub fn to_crores(&self) -> f64 {
        self.to_rupees() / 10_000_000.0
    }

    /// Format as INR with Indian number formatting
    pub fn format(&self) -> String {
        format_inr(self.to_rupees())
    }

    /// Format with custom precision
    pub fn format_precision(&self, precision: usize) -> String {
        format_inr_precision(self.to_rupees(), precision)
    }

    /// Add two amounts
    pub fn add(&self, other: &InrAmount) -> Self {
        Self {
            paise: self.paise + other.paise,
        }
    }

    /// Subtract two amounts
    pub fn subtract(&self, other: &InrAmount) -> Result<Self, String> {
        if self.paise < other.paise {
            return Err("Cannot subtract larger amount from smaller amount".to_string());
        }
        Ok(Self {
            paise: self.paise - other.paise,
        })
    }

    /// Multiply by a factor
    pub fn multiply(&self, factor: f64) -> Self {
        Self {
            paise: (self.paise as f64 * factor).round() as i64,
        }
    }

    /// Calculate percentage
    pub fn percentage(&self, percent: f64) -> Self {
        self.multiply(percent / 100.0)
    }

    /// Check if amount is zero
    pub fn is_zero(&self) -> bool {
        self.paise == 0
    }

    /// Check if amount is negative
    pub fn is_negative(&self) -> bool {
        self.paise < 0
    }
}

impl fmt::Display for InrAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Format amount in Indian Rupees with Indian number formatting
///
/// # Examples
///
/// ```
/// use legalis_in::common::currency::format_inr;
///
/// assert_eq!(format_inr(1234.56), "₹1,234.56");
/// assert_eq!(format_inr(123456.78), "₹1,23,456.78");
/// assert_eq!(format_inr(12345678.90), "₹1,23,45,678.90");
/// ```
pub fn format_inr(amount: f64) -> String {
    format_inr_precision(amount, 2)
}

/// Format amount in Indian Rupees with custom precision
pub fn format_inr_precision(amount: f64, precision: usize) -> String {
    let sign = if amount < 0.0 { "-" } else { "" };
    let abs_amount = amount.abs();

    // Separate integer and decimal parts
    let integer_part = abs_amount.floor() as i64;
    let decimal_part =
        ((abs_amount - abs_amount.floor()) * 10_f64.powi(precision as i32)).round() as i64;

    // Format integer part with Indian number system
    let formatted_integer = format_indian_number(integer_part);

    // Format with decimal part
    format!(
        "{}₹{}.{:0width$}",
        sign,
        formatted_integer,
        decimal_part,
        width = precision
    )
}

/// Format a number in Indian numbering system
/// First separator after 3 digits, then every 2 digits
fn format_indian_number(num: i64) -> String {
    let num_str = num.to_string();
    let len = num_str.len();

    if len <= 3 {
        return num_str;
    }

    let mut result = String::new();
    let chars: Vec<char> = num_str.chars().collect();

    // First group (rightmost 3 digits)
    let first_group_start = len.saturating_sub(3);

    // Add groups of 2 from right to left (excluding the first 3)
    let mut pos = first_group_start;
    let mut group_count = 0;

    while pos > 0 {
        let group_size = if pos >= 2 { 2 } else { pos };
        let start = pos - group_size;

        if group_count > 0 {
            result.insert(0, ',');
        }

        // Need to insert in reverse order when building from front
        for &ch in chars[start..pos].iter().rev() {
            result.insert(0, ch);
        }

        pos = start;
        group_count += 1;
    }

    // Add the first group (rightmost 3 digits)
    if first_group_start > 0 {
        result.push(',');
    }
    for &ch in &chars[first_group_start..len] {
        result.push(ch);
    }

    result
}

/// Convert amount to lakhs
///
/// # Examples
///
/// ```
/// use legalis_in::common::currency::to_lakhs;
///
/// assert_eq!(to_lakhs(100_000.0), 1.0);
/// assert_eq!(to_lakhs(5_000_000.0), 50.0);
/// ```
pub fn to_lakhs(amount: f64) -> f64 {
    amount / 100_000.0
}

/// Convert amount to crores
///
/// # Examples
///
/// ```
/// use legalis_in::common::currency::to_crores;
///
/// assert_eq!(to_crores(10_000_000.0), 1.0);
/// assert_eq!(to_crores(50_000_000.0), 5.0);
/// ```
pub fn to_crores(amount: f64) -> f64 {
    amount / 10_000_000.0
}

/// Convert lakhs to rupees
pub fn lakhs_to_rupees(lakhs: f64) -> f64 {
    lakhs * 100_000.0
}

/// Convert crores to rupees
pub fn crores_to_rupees(crores: f64) -> f64 {
    crores * 10_000_000.0
}

/// Format amount in lakhs
///
/// # Examples
///
/// ```
/// use legalis_in::common::currency::format_lakhs;
///
/// assert_eq!(format_lakhs(50.5), "50.50 lakhs");
/// assert_eq!(format_lakhs(1.0), "1.00 lakh");
/// ```
pub fn format_lakhs(lakhs: f64) -> String {
    if (lakhs - 1.0).abs() < f64::EPSILON {
        format!("{:.2} lakh", lakhs)
    } else {
        format!("{:.2} lakhs", lakhs)
    }
}

/// Format amount in crores
///
/// # Examples
///
/// ```
/// use legalis_in::common::currency::format_crores;
///
/// assert_eq!(format_crores(5.5), "5.50 crores");
/// assert_eq!(format_crores(1.0), "1.00 crore");
/// ```
pub fn format_crores(crores: f64) -> String {
    if (crores - 1.0).abs() < f64::EPSILON {
        format!("{:.2} crore", crores)
    } else {
        format!("{:.2} crores", crores)
    }
}

/// Parse INR amount from string
pub fn parse_inr(s: &str) -> Result<f64, String> {
    let cleaned = s
        .trim()
        .trim_start_matches('₹')
        .trim_start_matches("Rs.")
        .trim_start_matches("Rs")
        .replace(',', "")
        .trim()
        .to_string();

    cleaned
        .parse::<f64>()
        .map_err(|e| format!("Failed to parse INR amount: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_indian_number() {
        assert_eq!(format_indian_number(1234), "1,234");
        assert_eq!(format_indian_number(12345), "12,345");
        assert_eq!(format_indian_number(123456), "1,23,456");
        assert_eq!(format_indian_number(1234567), "12,34,567");
        assert_eq!(format_indian_number(12345678), "1,23,45,678");
        assert_eq!(format_indian_number(123456789), "12,34,56,789");
    }

    #[test]
    fn test_format_inr() {
        assert_eq!(format_inr(1234.56), "₹1,234.56");
        assert_eq!(format_inr(123456.78), "₹1,23,456.78");
        assert_eq!(format_inr(12345678.90), "₹1,23,45,678.90");
        assert_eq!(format_inr(5_000_000.0), "₹50,00,000.00");
    }

    #[test]
    fn test_to_lakhs_crores() {
        assert_eq!(to_lakhs(100_000.0), 1.0);
        assert_eq!(to_lakhs(5_000_000.0), 50.0);

        assert_eq!(to_crores(10_000_000.0), 1.0);
        assert_eq!(to_crores(50_000_000.0), 5.0);
    }

    #[test]
    fn test_inr_amount() {
        let amount = InrAmount::from_rupees(5_000_000.0);
        assert_eq!(amount.to_lakhs(), 50.0);
        assert_eq!(amount.to_crores(), 0.5);
        assert_eq!(amount.format(), "₹50,00,000.00");
    }

    #[test]
    fn test_inr_amount_arithmetic() {
        let a = InrAmount::from_rupees(1000.0);
        let b = InrAmount::from_rupees(500.0);

        assert_eq!(a.add(&b).to_rupees(), 1500.0);
        assert_eq!(a.subtract(&b).unwrap().to_rupees(), 500.0);
        assert_eq!(a.multiply(2.0).to_rupees(), 2000.0);
        assert_eq!(a.percentage(10.0).to_rupees(), 100.0);
    }

    #[test]
    fn test_format_lakhs_crores() {
        assert_eq!(format_lakhs(1.0), "1.00 lakh");
        assert_eq!(format_lakhs(50.5), "50.50 lakhs");

        assert_eq!(format_crores(1.0), "1.00 crore");
        assert_eq!(format_crores(5.5), "5.50 crores");
    }

    #[test]
    fn test_parse_inr() {
        assert_eq!(parse_inr("₹1,234.56").unwrap(), 1234.56);
        assert_eq!(parse_inr("Rs. 1,23,456").unwrap(), 123456.0);
        assert_eq!(parse_inr("50000").unwrap(), 50000.0);
    }

    #[test]
    fn test_negative_amounts() {
        let amount = format_inr(-1234.56);
        assert_eq!(amount, "-₹1,234.56");

        let inr = InrAmount::from_rupees(-500.0);
        assert!(inr.is_negative());
    }
}
