//! Malaysian Ringgit (MYR) currency formatting utilities.
//!
//! Provides proper formatting for Malaysian Ringgit amounts in legal documents.
//!
//! # Currency Format
//!
//! Malaysian Ringgit uses the following conventions:
//! - Symbol: RM (prefix)
//! - Decimal separator: . (period)
//! - Thousands separator: , (comma)
//! - Minor unit: sen (100 sen = 1 MYR)
//!
//! # Examples
//!
//! - RM1,234.56 (standard format)
//! - RM50,000.00 (with sen)
//! - MYR 1,234.56 (ISO format)

use legalis_i18n::Locale;
use serde::{Deserialize, Serialize};

/// Malaysian Ringgit currency utilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalaysianCurrency {
    _locale: Locale,
}

impl Default for MalaysianCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl MalaysianCurrency {
    /// Creates a new Malaysian currency formatter.
    #[must_use]
    pub fn new() -> Self {
        let locale = Locale::new("ms").with_country("MY");
        Self { _locale: locale }
    }

    /// Formats an amount in MYR with the standard RM symbol.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount in MYR (e.g., 1234.56)
    ///
    /// # Returns
    ///
    /// Formatted string like "RM1,234.56"
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::common::MalaysianCurrency;
    ///
    /// let myr = MalaysianCurrency::new();
    /// assert_eq!(myr.format(1234.56), "RM1,234.56");
    /// assert_eq!(myr.format(50000.00), "RM50,000.00");
    /// ```
    #[must_use]
    pub fn format(&self, amount: f64) -> String {
        format_myr(amount)
    }

    /// Formats an amount from sen to MYR.
    ///
    /// # Arguments
    ///
    /// * `sen` - The amount in sen (e.g., 123456 for RM1,234.56)
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::common::MalaysianCurrency;
    ///
    /// let myr = MalaysianCurrency::new();
    /// assert_eq!(myr.format_sen(123456), "RM1,234.56");
    /// ```
    #[must_use]
    pub fn format_sen(&self, sen: i64) -> String {
        format_myr_cents(sen)
    }

    /// Formats an amount with ISO currency code (MYR).
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::common::MalaysianCurrency;
    ///
    /// let myr = MalaysianCurrency::new();
    /// assert_eq!(myr.format_iso(1234.56), "MYR 1,234.56");
    /// ```
    #[must_use]
    pub fn format_iso(&self, amount: f64) -> String {
        format!("MYR {}", format_number_with_commas(amount))
    }

    /// Formats an amount in words (for checks and legal documents) - English.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::common::MalaysianCurrency;
    ///
    /// let myr = MalaysianCurrency::new();
    /// assert_eq!(
    ///     myr.format_words(1234.56),
    ///     "One Thousand Two Hundred Thirty Four Ringgit and Fifty Six Sen"
    /// );
    /// ```
    #[must_use]
    pub fn format_words(&self, amount: f64) -> String {
        let ringgit = amount.trunc() as i64;
        let sen = ((amount.fract() * 100.0).round()) as i64;

        let ringgit_words = number_to_words(ringgit);
        let ringgit_unit = "Ringgit"; // Malay doesn't distinguish plural/singular

        if sen == 0 {
            format!("{} {} Only", ringgit_words, ringgit_unit)
        } else {
            let sen_words = number_to_words(sen);
            let sen_unit = "Sen"; // Malay doesn't distinguish plural/singular
            format!(
                "{} {} and {} {}",
                ringgit_words, ringgit_unit, sen_words, sen_unit
            )
        }
    }

    /// Formats an amount in Malay words.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::common::MalaysianCurrency;
    ///
    /// let myr = MalaysianCurrency::new();
    /// assert_eq!(
    ///     myr.format_words_malay(1234.56),
    ///     "Seribu Dua Ratus Tiga Puluh Empat Ringgit dan Lima Puluh Enam Sen"
    /// );
    /// ```
    #[must_use]
    pub fn format_words_malay(&self, amount: f64) -> String {
        let ringgit = amount.trunc() as i64;
        let sen = ((amount.fract() * 100.0).round()) as i64;

        let ringgit_words = number_to_malay(ringgit);

        if sen == 0 {
            format!("{} Ringgit Sahaja", ringgit_words)
        } else {
            let sen_words = number_to_malay(sen);
            format!("{} Ringgit dan {} Sen", ringgit_words, sen_words)
        }
    }

    /// Formats an amount in Chinese characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::common::MalaysianCurrency;
    ///
    /// let myr = MalaysianCurrency::new();
    /// assert_eq!(myr.format_chinese(1234.00), "令吉壹仟贰佰叁拾肆元整");
    /// ```
    #[must_use]
    pub fn format_chinese(&self, amount: f64) -> String {
        let ringgit = amount.trunc() as i64;
        let sen = ((amount.fract() * 100.0).round()) as i64;

        let ringgit_chinese = number_to_chinese(ringgit);

        if sen == 0 {
            format!("令吉{}元整", ringgit_chinese)
        } else {
            let jiao = sen / 10;
            let fen = sen % 10;

            if fen == 0 {
                format!("令吉{}元{}角整", ringgit_chinese, digit_to_chinese(jiao))
            } else if jiao == 0 {
                format!("令吉{}元零{}分", ringgit_chinese, digit_to_chinese(fen))
            } else {
                format!(
                    "令吉{}元{}角{}分",
                    ringgit_chinese,
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

/// Formats an amount in MYR with the standard RM symbol.
///
/// # Example
///
/// ```rust
/// use legalis_my::common::format_myr;
///
/// assert_eq!(format_myr(1234.56), "RM1,234.56");
/// assert_eq!(format_myr(50000.00), "RM50,000.00");
/// assert_eq!(format_myr(0.99), "RM0.99");
/// ```
#[must_use]
pub fn format_myr(amount: f64) -> String {
    format!("RM{}", format_number_with_commas(amount))
}

/// Formats an amount from sen to MYR.
///
/// # Example
///
/// ```rust
/// use legalis_my::common::format_myr_cents;
///
/// assert_eq!(format_myr_cents(123456), "RM1,234.56");
/// assert_eq!(format_myr_cents(5000000), "RM50,000.00");
/// ```
#[must_use]
pub fn format_myr_cents(sen: i64) -> String {
    let amount = sen as f64 / 100.0;
    format_myr(amount)
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
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
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

    // Millions
    if num >= 1000000 {
        let millions = num / 1000000;
        result.push(format!("{} Million", number_to_words(millions)));
        num %= 1000000;
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

/// Converts a number to Malay words.
fn number_to_malay(n: i64) -> String {
    if n == 0 {
        return "Kosong".to_string();
    }

    let ones = [
        "", "Satu", "Dua", "Tiga", "Empat", "Lima", "Enam", "Tujuh", "Lapan", "Sembilan",
    ];

    let mut result = Vec::new();
    let mut num = n;

    // Millions (Juta)
    if num >= 1000000 {
        let millions = num / 1000000;
        result.push(format!("{} Juta", number_to_malay(millions)));
        num %= 1000000;
    }

    // Thousands (Ribu)
    if num >= 1000 {
        let thousands = num / 1000;
        if thousands == 1 {
            result.push("Seribu".to_string());
        } else {
            result.push(format!("{} Ribu", number_to_malay(thousands)));
        }
        num %= 1000;
    }

    // Hundreds (Ratus)
    if num >= 100 {
        let hundreds = num / 100;
        result.push(format!("{} Ratus", ones[hundreds as usize]));
        num %= 100;
    }

    // Tens and ones
    if num >= 20 {
        let t = num / 10;
        let o = num % 10;
        if o > 0 {
            result.push(format!("{} Puluh {}", ones[t as usize], ones[o as usize]));
        } else {
            result.push(format!("{} Puluh", ones[t as usize]));
        }
    } else if num >= 10 {
        let o = num % 10;
        if o == 0 {
            result.push("Sepuluh".to_string());
        } else if o == 1 {
            result.push("Sebelas".to_string());
        } else {
            result.push(format!("{} Belas", ones[o as usize]));
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
    fn test_format_myr() {
        assert_eq!(format_myr(1234.56), "RM1,234.56");
        assert_eq!(format_myr(50000.00), "RM50,000.00");
        assert_eq!(format_myr(0.99), "RM0.99");
        assert_eq!(format_myr(1000000.00), "RM1,000,000.00");
    }

    #[test]
    fn test_format_myr_cents() {
        assert_eq!(format_myr_cents(123456), "RM1,234.56");
        assert_eq!(format_myr_cents(5000000), "RM50,000.00");
        assert_eq!(format_myr_cents(99), "RM0.99");
    }

    #[test]
    fn test_malaysian_currency_format() {
        let myr = MalaysianCurrency::new();
        assert_eq!(myr.format(1234.56), "RM1,234.56");
        assert_eq!(myr.format_sen(123456), "RM1,234.56");
    }

    #[test]
    fn test_format_iso() {
        let myr = MalaysianCurrency::new();
        assert_eq!(myr.format_iso(1234.56), "MYR 1,234.56");
    }

    #[test]
    fn test_format_words() {
        let myr = MalaysianCurrency::new();
        assert_eq!(
            myr.format_words(1234.56),
            "One Thousand Two Hundred Thirty Four Ringgit and Fifty Six Sen"
        );
        assert_eq!(myr.format_words(1.00), "One Ringgit Only");
    }

    #[test]
    fn test_format_words_malay() {
        let myr = MalaysianCurrency::new();
        assert_eq!(
            myr.format_words_malay(1234.56),
            "Seribu Dua Ratus Tiga Puluh Empat Ringgit dan Lima Puluh Enam Sen"
        );
        assert_eq!(myr.format_words_malay(1.00), "Satu Ringgit Sahaja");
    }

    #[test]
    fn test_format_chinese() {
        let myr = MalaysianCurrency::new();
        assert_eq!(myr.format_chinese(1234.00), "令吉壹仟贰佰叁拾肆元整");
        assert_eq!(myr.format_chinese(1234.56), "令吉壹仟贰佰叁拾肆元伍角陆分");
    }
}
