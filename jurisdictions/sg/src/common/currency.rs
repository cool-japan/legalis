//! Singapore Dollar (SGD) currency formatting utilities.
//!
//! Provides proper formatting for Singapore Dollar amounts in legal documents.
//!
//! # Currency Format
//!
//! Singapore Dollar uses the following conventions:
//! - Symbol: S$ (prefix)
//! - Decimal separator: . (period)
//! - Thousands separator: , (comma)
//! - Minor unit: cents (100 cents = 1 SGD)
//!
//! # Examples
//!
//! - S$1,234.56 (standard format)
//! - S$50,000.00 (with cents)
//! - SGD 1,234.56 (ISO format)

use legalis_i18n::Locale;

/// Singapore Dollar currency utilities.
#[derive(Debug, Clone)]
pub struct SingaporeCurrency {
    _locale: Locale,
}

impl Default for SingaporeCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl SingaporeCurrency {
    /// Creates a new Singapore currency formatter.
    #[must_use]
    pub fn new() -> Self {
        let locale = Locale::new("en").with_country("SG");
        Self { _locale: locale }
    }

    /// Formats an amount in SGD with the standard S$ symbol.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount in SGD (e.g., 1234.56)
    ///
    /// # Returns
    ///
    /// Formatted string like "S$1,234.56"
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporeCurrency;
    ///
    /// let sgd = SingaporeCurrency::new();
    /// assert_eq!(sgd.format(1234.56), "S$1,234.56");
    /// assert_eq!(sgd.format(50000.00), "S$50,000.00");
    /// ```
    #[must_use]
    pub fn format(&self, amount: f64) -> String {
        format_sgd(amount)
    }

    /// Formats an amount from cents to SGD.
    ///
    /// # Arguments
    ///
    /// * `cents` - The amount in cents (e.g., 123456 for S$1,234.56)
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporeCurrency;
    ///
    /// let sgd = SingaporeCurrency::new();
    /// assert_eq!(sgd.format_cents(123456), "S$1,234.56");
    /// ```
    #[must_use]
    pub fn format_cents(&self, cents: i64) -> String {
        format_sgd_cents(cents)
    }

    /// Formats an amount with ISO currency code (SGD).
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporeCurrency;
    ///
    /// let sgd = SingaporeCurrency::new();
    /// assert_eq!(sgd.format_iso(1234.56), "SGD 1,234.56");
    /// ```
    #[must_use]
    pub fn format_iso(&self, amount: f64) -> String {
        format!("SGD {}", format_number_with_commas(amount))
    }

    /// Formats an amount in words (for checks and legal documents).
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporeCurrency;
    ///
    /// let sgd = SingaporeCurrency::new();
    /// assert_eq!(
    ///     sgd.format_words(1234.56),
    ///     "One Thousand Two Hundred Thirty Four Dollars and Fifty Six Cents"
    /// );
    /// ```
    #[must_use]
    pub fn format_words(&self, amount: f64) -> String {
        let dollars = amount.trunc() as i64;
        let cents = ((amount.fract() * 100.0).round()) as i64;

        let dollars_words = number_to_words(dollars);
        let dollars_unit = if dollars == 1 { "Dollar" } else { "Dollars" };

        if cents == 0 {
            format!("{} {} Only", dollars_words, dollars_unit)
        } else {
            let cents_words = number_to_words(cents);
            let cents_unit = if cents == 1 { "Cent" } else { "Cents" };
            format!(
                "{} {} and {} {}",
                dollars_words, dollars_unit, cents_words, cents_unit
            )
        }
    }

    /// Formats an amount in Chinese characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporeCurrency;
    ///
    /// let sgd = SingaporeCurrency::new();
    /// assert_eq!(sgd.format_chinese(1234.00), "新币壹仟贰佰叁拾肆元整");
    /// ```
    #[must_use]
    pub fn format_chinese(&self, amount: f64) -> String {
        let dollars = amount.trunc() as i64;
        let cents = ((amount.fract() * 100.0).round()) as i64;

        let dollars_chinese = number_to_chinese(dollars);

        if cents == 0 {
            format!("新币{}元整", dollars_chinese)
        } else {
            let jiao = cents / 10;
            let fen = cents % 10;

            if fen == 0 {
                format!("新币{}元{}角整", dollars_chinese, digit_to_chinese(jiao))
            } else if jiao == 0 {
                format!("新币{}元零{}分", dollars_chinese, digit_to_chinese(fen))
            } else {
                format!(
                    "新币{}元{}角{}分",
                    dollars_chinese,
                    digit_to_chinese(jiao),
                    digit_to_chinese(fen)
                )
            }
        }
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Formats an amount in SGD with the standard S$ symbol.
///
/// # Example
///
/// ```rust
/// use legalis_sg::common::format_sgd;
///
/// assert_eq!(format_sgd(1234.56), "S$1,234.56");
/// assert_eq!(format_sgd(50000.00), "S$50,000.00");
/// assert_eq!(format_sgd(0.99), "S$0.99");
/// ```
#[must_use]
pub fn format_sgd(amount: f64) -> String {
    format!("S${}", format_number_with_commas(amount))
}

/// Formats an amount from cents to SGD.
///
/// # Example
///
/// ```rust
/// use legalis_sg::common::format_sgd_cents;
///
/// assert_eq!(format_sgd_cents(123456), "S$1,234.56");
/// assert_eq!(format_sgd_cents(5000000), "S$50,000.00");
/// ```
#[must_use]
pub fn format_sgd_cents(cents: i64) -> String {
    let amount = cents as f64 / 100.0;
    format_sgd(amount)
}

/// Formats a number with comma thousands separators and 2 decimal places.
fn format_number_with_commas(amount: f64) -> String {
    let negative = amount < 0.0;
    let abs_amount = amount.abs();
    let whole = abs_amount.trunc() as i64;
    let frac = ((abs_amount.fract() * 100.0).round()) as i64;

    // Format whole part with commas
    let whole_str = whole.to_string();
    let mut result = String::new();
    let chars: Vec<char> = whole_str.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }

    // Add negative sign if needed
    if negative {
        result = format!("-{}", result);
    }

    // Add decimal part
    format!("{}.{:02}", result, frac)
}

/// Converts a number to English words.
fn number_to_words(n: i64) -> String {
    if n == 0 {
        return "Zero".to_string();
    }

    let ones = [
        "",
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fourteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eighteen",
        "Nineteen",
    ];

    let tens = [
        "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
    ];

    let mut result = Vec::new();
    let mut num = n;

    // Billions
    if num >= 1_000_000_000 {
        let billions = num / 1_000_000_000;
        result.push(format!("{} Billion", number_to_words(billions)));
        num %= 1_000_000_000;
    }

    // Millions
    if num >= 1_000_000 {
        let millions = num / 1_000_000;
        result.push(format!("{} Million", number_to_words(millions)));
        num %= 1_000_000;
    }

    // Thousands
    if num >= 1000 {
        let thousands = num / 1000;
        result.push(format!("{} Thousand", number_to_words(thousands)));
        num %= 1000;
    }

    // Hundreds
    if num >= 100 {
        let hundreds = num / 100;
        result.push(format!("{} Hundred", ones[hundreds as usize]));
        num %= 100;
    }

    // Tens and ones
    if num >= 20 {
        let t = num / 10;
        let o = num % 10;
        if o > 0 {
            result.push(format!("{} {}", tens[t as usize], ones[o as usize]));
        } else {
            result.push(tens[t as usize].to_string());
        }
    } else if num > 0 {
        result.push(ones[num as usize].to_string());
    }

    result.join(" ")
}

/// Converts a single digit to Chinese numeral.
fn digit_to_chinese(n: i64) -> &'static str {
    match n {
        0 => "零",
        1 => "壹",
        2 => "贰",
        3 => "叁",
        4 => "肆",
        5 => "伍",
        6 => "陆",
        7 => "柒",
        8 => "捌",
        9 => "玖",
        _ => "",
    }
}

/// Converts a number to Chinese numerals (simplified financial format).
fn number_to_chinese(n: i64) -> String {
    if n == 0 {
        return "零".to_string();
    }

    let mut result = String::new();
    let mut num = n;
    let mut need_zero = false;

    // Ten thousands (万)
    if num >= 10000 {
        let wan = num / 10000;
        result.push_str(&number_to_chinese_under_10000(wan));
        result.push('万');
        num %= 10000;
        need_zero = num > 0 && num < 1000;
    }

    // Under 10000
    if num > 0 {
        if need_zero {
            result.push('零');
        }
        result.push_str(&number_to_chinese_under_10000(num));
    }

    result
}

/// Converts a number under 10000 to Chinese numerals.
fn number_to_chinese_under_10000(n: i64) -> String {
    if n == 0 {
        return String::new();
    }

    let mut result = String::new();
    let mut num = n;
    let mut need_zero = false;

    // Thousands (仟)
    if num >= 1000 {
        let qian = num / 1000;
        result.push_str(digit_to_chinese(qian));
        result.push('仟');
        num %= 1000;
        need_zero = num > 0 && num < 100;
    }

    // Hundreds (佰)
    if num >= 100 {
        if need_zero {
            result.push('零');
        }
        let bai = num / 100;
        result.push_str(digit_to_chinese(bai));
        result.push('佰');
        num %= 100;
        need_zero = num > 0 && num < 10;
    }

    // Tens (拾)
    if num >= 10 {
        if need_zero {
            result.push('零');
            need_zero = false;
        }
        let shi = num / 10;
        result.push_str(digit_to_chinese(shi));
        result.push('拾');
        num %= 10;
    }

    // Ones
    if num > 0 {
        if need_zero {
            result.push('零');
        }
        result.push_str(digit_to_chinese(num));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_sgd() {
        assert_eq!(format_sgd(1234.56), "S$1,234.56");
        assert_eq!(format_sgd(50000.00), "S$50,000.00");
        assert_eq!(format_sgd(0.99), "S$0.99");
        assert_eq!(format_sgd(1000000.00), "S$1,000,000.00");
    }

    #[test]
    fn test_format_sgd_cents() {
        assert_eq!(format_sgd_cents(123456), "S$1,234.56");
        assert_eq!(format_sgd_cents(5000000), "S$50,000.00");
        assert_eq!(format_sgd_cents(99), "S$0.99");
    }

    #[test]
    fn test_singapore_currency_format() {
        let sgd = SingaporeCurrency::new();
        assert_eq!(sgd.format(1234.56), "S$1,234.56");
        assert_eq!(sgd.format_cents(123456), "S$1,234.56");
    }

    #[test]
    fn test_format_iso() {
        let sgd = SingaporeCurrency::new();
        assert_eq!(sgd.format_iso(1234.56), "SGD 1,234.56");
    }

    #[test]
    fn test_format_words() {
        let sgd = SingaporeCurrency::new();
        assert_eq!(
            sgd.format_words(1234.56),
            "One Thousand Two Hundred Thirty Four Dollars and Fifty Six Cents"
        );
        assert_eq!(sgd.format_words(1.00), "One Dollar Only");
        assert_eq!(sgd.format_words(0.01), "Zero Dollars and One Cent");
    }

    #[test]
    fn test_format_chinese() {
        let sgd = SingaporeCurrency::new();
        assert_eq!(sgd.format_chinese(1234.00), "新币壹仟贰佰叁拾肆元整");
        assert_eq!(sgd.format_chinese(50000.00), "新币伍万元整");
        assert_eq!(sgd.format_chinese(1234.56), "新币壹仟贰佰叁拾肆元伍角陆分");
        assert_eq!(sgd.format_chinese(1234.50), "新币壹仟贰佰叁拾肆元伍角整");
        assert_eq!(sgd.format_chinese(1234.05), "新币壹仟贰佰叁拾肆元零伍分");
    }

    #[test]
    fn test_number_to_words() {
        assert_eq!(number_to_words(0), "Zero");
        assert_eq!(number_to_words(1), "One");
        assert_eq!(number_to_words(15), "Fifteen");
        assert_eq!(number_to_words(42), "Forty Two");
        assert_eq!(number_to_words(100), "One Hundred");
        assert_eq!(number_to_words(1000), "One Thousand");
        assert_eq!(
            number_to_words(1234),
            "One Thousand Two Hundred Thirty Four"
        );
    }

    #[test]
    fn test_number_to_chinese() {
        assert_eq!(number_to_chinese(0), "零");
        assert_eq!(number_to_chinese(1), "壹");
        assert_eq!(number_to_chinese(12), "壹拾贰");
        assert_eq!(number_to_chinese(123), "壹佰贰拾叁");
        assert_eq!(number_to_chinese(1234), "壹仟贰佰叁拾肆");
        assert_eq!(number_to_chinese(50000), "伍万");
    }
}
