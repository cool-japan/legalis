//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types_5::TranslationManager;
use super::types_8::LegalDictionary;
use super::types_10::{CurrencyFormatter, Locale};
use super::types_11::I18nError;
use super::types_12::DateTimeFormatter;

/// Result type for i18n operations.
pub type I18nResult<T> = Result<T, I18nError>;
/// External translation service interface.
/// Implement this trait to integrate with services like Google Translate, DeepL, etc.
pub trait TranslationService: Send + Sync + std::fmt::Debug {
    /// Translates text from source locale to target locale.
    fn translate(&self, text: &str, source: &Locale, target: &Locale) -> I18nResult<String>;
    /// Translates multiple texts in batch.
    fn translate_batch(
        &self,
        texts: &[&str],
        source: &Locale,
        target: &Locale,
    ) -> I18nResult<Vec<String>>;
    /// Gets the name of this translation service.
    fn service_name(&self) -> &str;
    /// Checks if the service is available.
    fn is_available(&self) -> bool;
}
/// Detects the most likely locale from a text sample.
/// Uses simple heuristics based on character sets.
pub fn detect_locale_from_text(text: &str) -> Option<Locale> {
    let has_cjk = text.chars().any(|c| {
        matches!(
            c, '\u{4E00}'..='\u{9FFF}' | '\u{3040}'..='\u{309F}' | '\u{30A0}'
            ..='\u{30FF}'
        )
    });
    let has_hiragana = text.chars().any(|c| matches!(c, '\u{3040}'..='\u{309F}'));
    let has_katakana = text.chars().any(|c| matches!(c, '\u{30A0}'..='\u{30FF}'));
    let has_cyrillic = text.chars().any(|c| matches!(c, '\u{0400}'..='\u{04FF}'));
    let has_arabic = text.chars().any(|c| matches!(c, '\u{0600}'..='\u{06FF}'));
    if has_hiragana || has_katakana {
        Some(Locale::new("ja").with_country("JP"))
    } else if has_cjk {
        Some(Locale::new("zh").with_country("CN"))
    } else if has_cyrillic {
        Some(Locale::new("ru").with_country("RU"))
    } else if has_arabic {
        Some(Locale::new("ar"))
    } else {
        Some(Locale::new("en").with_country("US"))
    }
}
/// Formats a legal date with appropriate context and locale.
pub fn format_legal_date(
    year: i32,
    month: u32,
    day: u32,
    locale: &Locale,
    context: &str,
) -> String {
    let formatter = DateTimeFormatter::new(locale.clone());
    let date = formatter.format_date(year, month, day);
    match context {
        "effective" => format!("Effective Date: {}", date),
        "expiration" => format!("Expiration Date: {}", date),
        "execution" => format!("Date of Execution: {}", date),
        "filing" => format!("Filing Date: {}", date),
        _ => date,
    }
}
/// Batch translates multiple keys using a translation manager.
pub fn batch_translate(
    manager: &TranslationManager,
    keys: &[&str],
    locale: &Locale,
) -> Vec<Result<String, I18nError>> {
    keys.iter()
        .map(|key| manager.translate(key, locale))
        .collect()
}
/// Creates a locale-aware error message.
pub fn format_error_message(error_key: &str, locale: &Locale, params: &[(&str, &str)]) -> String {
    let mut message = match (error_key, locale.language.as_str()) {
        ("missing_field", "ja") => "必須フィールドが不足しています".to_string(),
        ("missing_field", _) => "Required field is missing".to_string(),
        ("invalid_format", "ja") => "形式が無効です".to_string(),
        ("invalid_format", _) => "Invalid format".to_string(),
        ("unauthorized", "ja") => "権限がありません".to_string(),
        ("unauthorized", _) => "Unauthorized".to_string(),
        ("not_found", "ja") => "見つかりません".to_string(),
        ("not_found", _) => "Not found".to_string(),
        _ => error_key.to_string(),
    };
    for (key, value) in params {
        message.push_str(&format!(" {}={}", key, value));
    }
    message
}
/// Formats a monetary amount in a legal context.
pub fn format_legal_amount(amount: f64, currency: &str, locale: &Locale, context: &str) -> String {
    let formatter = CurrencyFormatter::new(locale.clone());
    let formatted = formatter.format(amount, currency);
    match context {
        "compensation" => format!("Compensation Amount: {}", formatted),
        "damages" => format!("Damages: {}", formatted),
        "fine" => format!("Fine Amount: {}", formatted),
        "payment" => format!("Payment: {}", formatted),
        _ => formatted,
    }
}
/// Creates a multi-locale translation manager with all standard dictionaries.
pub fn create_standard_translation_manager() -> TranslationManager {
    let mut manager = TranslationManager::new();
    manager.add_dictionary(LegalDictionary::english_us());
    manager.add_dictionary(LegalDictionary::japanese());
    manager.add_dictionary(LegalDictionary::german());
    manager.add_dictionary(LegalDictionary::french());
    manager.add_dictionary(LegalDictionary::spanish());
    manager.add_dictionary(LegalDictionary::chinese_simplified());
    manager
}
/// Normalizes a locale string to a standard format.
/// Examples: "en_US" -> "en-US", "ja" -> "ja", "ZH-HANS-CN" -> "zh-Hans-CN"
pub fn normalize_locale_string(input: &str) -> String {
    let parts: Vec<&str> = input.split(['-', '_']).collect();
    if parts.is_empty() {
        return input.to_lowercase();
    }
    let mut normalized = parts[0].to_lowercase();
    for part in parts.iter().skip(1) {
        normalized.push('-');
        if part.len() == 2 && part.chars().all(|c| c.is_alphabetic()) {
            normalized.push_str(&part.to_uppercase());
        } else if part.len() == 4 && part.chars().all(|c| c.is_alphabetic()) {
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                normalized.push(
                    first
                        .to_uppercase()
                        .next()
                        .expect("invariant: ASCII char always has uppercase mapping"),
                );
                normalized.extend(chars.map(|c| {
                    c.to_lowercase()
                        .next()
                        .expect("invariant: ASCII char always has lowercase mapping")
                }));
            }
        } else {
            normalized.push_str(&part.to_lowercase());
        }
    }
    normalized
}
/// Validates a language code (ISO 639-1).
/// Returns true if the code is a valid 2-letter language code.
pub fn is_valid_language_code(code: &str) -> bool {
    code.len() == 2 && code.chars().all(|c| c.is_ascii_lowercase())
}
/// Validates a country code (ISO 3166-1 alpha-2).
/// Returns true if the code is a valid 2-letter uppercase country code.
pub fn is_valid_country_code(code: &str) -> bool {
    code.len() == 2 && code.chars().all(|c| c.is_ascii_uppercase())
}
/// Validates a script code (ISO 15924).
/// Returns true if the code is a valid 4-letter script code with title case.
pub fn is_valid_script_code(code: &str) -> bool {
    if code.len() != 4 {
        return false;
    }
    let chars: Vec<char> = code.chars().collect();
    chars[0].is_uppercase() && chars[1..].iter().all(|c| c.is_lowercase())
}
/// Validates a locale tag.
/// Returns true if the locale tag has valid structure.
pub fn is_valid_locale_tag(tag: &str) -> bool {
    if let Ok(locale) = Locale::parse(tag) {
        if !is_valid_language_code(&locale.language) {
            return false;
        }
        if let Some(ref country) = locale.country
            && !is_valid_country_code(country)
        {
            return false;
        }
        if let Some(ref script) = locale.script
            && !is_valid_script_code(script)
        {
            return false;
        }
        true
    } else {
        false
    }
}
/// Gets a list of common legal jurisdiction locales.
pub fn common_legal_locales() -> Vec<Locale> {
    vec![
        Locale::new("en").with_country("US"),
        Locale::new("en").with_country("GB"),
        Locale::new("ja").with_country("JP"),
        Locale::new("de").with_country("DE"),
        Locale::new("fr").with_country("FR"),
        Locale::new("es").with_country("ES"),
        Locale::new("zh").with_script("Hans").with_country("CN"),
        Locale::new("zh").with_script("Hant").with_country("TW"),
        Locale::new("ko").with_country("KR"),
        Locale::new("it").with_country("IT"),
    ]
}
/// Suggests the best matching locale from a list of available locales.
/// Uses fallback chain logic to find the best match.
pub fn suggest_best_locale<'a>(requested: &Locale, available: &'a [Locale]) -> Option<&'a Locale> {
    for locale in available {
        if locale == requested {
            return Some(locale);
        }
    }
    if requested.country.is_some() {
        for locale in available {
            if locale.language == requested.language && locale.country == requested.country {
                return Some(locale);
            }
        }
    }
    for locale in available {
        if locale.language == requested.language && locale.country.is_none() {
            return Some(locale);
        }
    }
    available
        .iter()
        .find(|locale| locale.language == requested.language)
        .map(|v| v as _)
}
