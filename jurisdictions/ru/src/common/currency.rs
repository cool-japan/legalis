//! Russian Ruble (RUB) currency handling.

use serde::{Deserialize, Serialize};

/// Russian currency representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Currency {
    /// Amount in kopecks (1 ruble = 100 kopecks)
    pub kopecks: i64,
}

impl Currency {
    /// Creates a new Currency from rubles
    pub fn from_rubles(rubles: i64) -> Self {
        Self {
            kopecks: rubles * 100,
        }
    }

    /// Creates a new Currency from rubles and kopecks
    pub fn new(rubles: i64, kopecks: i64) -> Self {
        Self {
            kopecks: rubles * 100 + kopecks,
        }
    }

    /// Gets the ruble amount (integer part)
    pub fn rubles(&self) -> i64 {
        self.kopecks / 100
    }

    /// Gets the kopeck amount (fractional part)
    pub fn kopecks_part(&self) -> i64 {
        self.kopecks % 100
    }

    /// Converts to f64 (rubles with decimal)
    pub fn to_f64(&self) -> f64 {
        self.kopecks as f64 / 100.0
    }

    /// Adds two currency amounts
    pub fn add(&self, other: &Currency) -> Currency {
        Currency {
            kopecks: self.kopecks + other.kopecks,
        }
    }

    /// Subtracts two currency amounts
    pub fn subtract(&self, other: &Currency) -> Currency {
        Currency {
            kopecks: self.kopecks - other.kopecks,
        }
    }

    /// Multiplies by a percentage (for tax calculations)
    pub fn multiply_percentage(&self, percentage: f64) -> Currency {
        Currency {
            kopecks: ((self.kopecks as f64) * percentage / 100.0).round() as i64,
        }
    }

    /// Checks if the amount is positive
    pub fn is_positive(&self) -> bool {
        self.kopecks > 0
    }

    /// Checks if the amount is zero
    pub fn is_zero(&self) -> bool {
        self.kopecks == 0
    }

    /// Checks if the amount is negative
    pub fn is_negative(&self) -> bool {
        self.kopecks < 0
    }
}

/// Formats an amount in Russian Rubles
///
/// # Examples
///
/// ```
/// use legalis_ru::common::format_ruble;
///
/// assert_eq!(format_ruble(1000, 50), "1 000,50 ₽");
/// assert_eq!(format_ruble(42, 0), "42,00 ₽");
/// ```
pub fn format_ruble(rubles: i64, kopecks: i64) -> String {
    let total_kopecks = rubles * 100 + kopecks;
    let whole = total_kopecks / 100;
    let fraction = (total_kopecks % 100).abs();

    // Format with thousand separators (Russian style uses space)
    let whole_str = format_with_thousands_separator(whole);

    format!("{},{:02} ₽", whole_str, fraction)
}

fn format_with_thousands_separator(n: i64) -> String {
    let s = n.abs().to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

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

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_ruble(self.rubles(), self.kopecks_part()))
    }
}

impl std::ops::Add for Currency {
    type Output = Currency;

    fn add(self, other: Currency) -> Currency {
        Currency {
            kopecks: self.kopecks + other.kopecks,
        }
    }
}

impl std::ops::Sub for Currency {
    type Output = Currency;

    fn sub(self, other: Currency) -> Currency {
        Currency {
            kopecks: self.kopecks - other.kopecks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_creation() {
        let c = Currency::from_rubles(100);
        assert_eq!(c.rubles(), 100);
        assert_eq!(c.kopecks_part(), 0);

        let c2 = Currency::new(100, 50);
        assert_eq!(c2.rubles(), 100);
        assert_eq!(c2.kopecks_part(), 50);
    }

    #[test]
    fn test_currency_operations() {
        let c1 = Currency::new(100, 50);
        let c2 = Currency::new(50, 25);

        let sum = c1 + c2;
        assert_eq!(sum.rubles(), 150);
        assert_eq!(sum.kopecks_part(), 75);

        let diff = c1 - c2;
        assert_eq!(diff.rubles(), 50);
        assert_eq!(diff.kopecks_part(), 25);
    }

    #[test]
    fn test_currency_formatting() {
        assert_eq!(format_ruble(1000, 50), "1 000,50 ₽");
        assert_eq!(format_ruble(1000000, 0), "1 000 000,00 ₽");
        assert_eq!(format_ruble(42, 5), "42,05 ₽");
    }

    #[test]
    fn test_percentage_multiplication() {
        let c = Currency::from_rubles(1000);
        let vat = c.multiply_percentage(20.0); // 20% VAT
        assert_eq!(vat.rubles(), 200);
    }
}
