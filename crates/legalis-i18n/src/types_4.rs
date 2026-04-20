//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::functions::I18nResult;
use super::functions_3::QualityScore;
use super::types::{LegalEntity, LegalEntityType, SimplificationStrategy};
use super::types_3::{ColonialLegacy, CompletenessReport, GlossaryViolation};
use super::types_5::{CitationError, CitationValidator, TranslationManager};
use super::types_6::{CitationComponents, CitationStyle, ColonialPower};
use super::types_8::LegalDictionary;
use super::types_9::{CitationCompletenessChecker, TranscriptionSegment};
use super::types_10::{CitationFormatter, LegalSpeechTranscriber, Locale, ViolationType};
use super::types_11::{HistoricalContext, InterpretationMode, TranslationEngine};
use super::types_12::{ContextCategory, EquivalenceLevel, LegalSpeechDomain, ReadingLevelAssessor};

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl DayOfWeek {
    /// Returns the day number (0 = Monday, 6 = Sunday).
    pub fn to_number(&self) -> u32 {
        match self {
            DayOfWeek::Monday => 0,
            DayOfWeek::Tuesday => 1,
            DayOfWeek::Wednesday => 2,
            DayOfWeek::Thursday => 3,
            DayOfWeek::Friday => 4,
            DayOfWeek::Saturday => 5,
            DayOfWeek::Sunday => 6,
        }
    }
}
/// Legal entity recognizer for identifying entities in legal text.
pub struct LegalEntityRecognizer {
    /// Court patterns
    pub(crate) court_patterns: Vec<String>,
    /// Company suffixes
    pub(crate) company_suffixes: Vec<String>,
    /// Statute keywords
    statute_keywords: Vec<String>,
    /// Government agency patterns
    agency_patterns: Vec<String>,
    /// Law firm suffixes
    law_firm_suffixes: Vec<String>,
}
impl LegalEntityRecognizer {
    /// Creates a new legal entity recognizer with default patterns.
    pub fn new() -> Self {
        Self {
            court_patterns: vec![
                "Court".to_string(),
                "Tribunal".to_string(),
                "裁判所".to_string(),
                "Gericht".to_string(),
                "Cour".to_string(),
                "Corte".to_string(),
            ],
            company_suffixes: vec![
                "Inc.".to_string(),
                "LLC".to_string(),
                "Ltd.".to_string(),
                "Corp.".to_string(),
                "GmbH".to_string(),
                "株式会社".to_string(),
                "S.A.".to_string(),
                "AG".to_string(),
            ],
            statute_keywords: vec![
                "Act".to_string(),
                "Code".to_string(),
                "Law".to_string(),
                "Statute".to_string(),
                "法".to_string(),
                "Gesetz".to_string(),
                "Loi".to_string(),
            ],
            agency_patterns: vec![
                "SEC".to_string(),
                "FTC".to_string(),
                "FDA".to_string(),
                "EPA".to_string(),
                "Commission".to_string(),
                "Agency".to_string(),
                "Bureau".to_string(),
            ],
            law_firm_suffixes: vec![
                "LLP".to_string(),
                "P.C.".to_string(),
                "P.A.".to_string(),
                "& Associates".to_string(),
            ],
        }
    }
    /// Recognizes legal entities in text.
    pub fn recognize(&self, text: &str) -> Vec<LegalEntity> {
        let mut entities = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        for window in words.windows(1) {
            let combined = window.join(" ");
            let pos = text.find(window[0]).unwrap_or(0);
            if self.court_patterns.iter().any(|p| combined.contains(p)) {
                entities.push(
                    LegalEntity::new(combined.clone(), LegalEntityType::Court, pos)
                        .with_confidence(0.8),
                );
            }
            if self.company_suffixes.iter().any(|s| combined.ends_with(s)) {
                entities.push(
                    LegalEntity::new(combined.clone(), LegalEntityType::Company, pos)
                        .with_confidence(0.85),
                );
            }
            if self.statute_keywords.iter().any(|k| combined.contains(k)) {
                entities.push(
                    LegalEntity::new(combined.clone(), LegalEntityType::Statute, pos)
                        .with_confidence(0.75),
                );
            }
            if self.agency_patterns.iter().any(|p| combined.contains(p)) {
                entities.push(
                    LegalEntity::new(combined.clone(), LegalEntityType::GovernmentAgency, pos)
                        .with_confidence(0.9),
                );
            }
            if self.law_firm_suffixes.iter().any(|s| combined.ends_with(s)) {
                entities.push(
                    LegalEntity::new(combined.clone(), LegalEntityType::LawFirm, pos)
                        .with_confidence(0.85),
                );
            }
        }
        entities
    }
    /// Adds a custom court pattern.
    pub fn add_court_pattern(&mut self, pattern: impl Into<String>) {
        self.court_patterns.push(pattern.into());
    }
    /// Adds a custom company suffix.
    pub fn add_company_suffix(&mut self, suffix: impl Into<String>) {
        self.company_suffixes.push(suffix.into());
    }
    /// Gets the count of recognized entities by type.
    pub fn count_by_type(&self, entities: &[LegalEntity], entity_type: &LegalEntityType) -> usize {
        entities
            .iter()
            .filter(|e| &e.entity_type == entity_type)
            .count()
    }
}
/// Historical period for legal language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HistoricalPeriod {
    /// Old English (450-1150 AD)
    OldEnglish,
    /// Middle English (1150-1500 AD)
    MiddleEnglish,
    /// Early Modern English (1500-1700 AD)
    EarlyModern,
    /// Classical Latin (Roman Empire)
    ClassicalLatin,
    /// Medieval Latin (500-1500 AD)
    MedievalLatin,
    /// Renaissance (1400-1600 AD)
    Renaissance,
    /// Enlightenment (1600-1800 AD)
    Enlightenment,
    /// Victorian (1837-1901 AD)
    Victorian,
}
/// Lazy-loading dictionary wrapper for efficient memory usage with large dictionaries.
/// Loads dictionary data on-demand using `Arc<Mutex>` for thread-safe initialization.
pub struct LazyDictionary {
    /// Locale for this dictionary
    pub locale: Locale,
    /// Lazy-loaded dictionary data
    pub(super) data: Arc<Mutex<Option<LegalDictionary>>>,
    /// Loading function
    pub(super) loader: Arc<dyn Fn() -> LegalDictionary + Send + Sync>,
}
impl LazyDictionary {
    /// Creates a new lazy dictionary with a custom loader function.
    pub fn new<F>(locale: Locale, loader: F) -> Self
    where
        F: Fn() -> LegalDictionary + Send + Sync + 'static,
    {
        Self {
            locale,
            data: Arc::new(Mutex::new(None)),
            loader: Arc::new(loader),
        }
    }
    /// Gets a reference to the loaded dictionary.
    /// Loads the dictionary on first access.
    pub fn get(&self) -> Arc<Mutex<LegalDictionary>> {
        let mut data = self.data.lock().expect("data mutex poisoned");
        if data.is_none() {
            *data = Some((self.loader)());
        }
        let dict = data.take().expect("invariant: data was set Some above");
        let result = Arc::new(Mutex::new(dict.clone()));
        *data = Some(dict);
        result
    }
    /// Checks if the dictionary has been loaded yet.
    pub fn is_loaded(&self) -> bool {
        self.data.lock().expect("data mutex poisoned").is_some()
    }
}
/// Equivalent term in another jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquivalentTerm {
    /// Equivalent term text
    pub term: String,
    /// Equivalence level (exact, approximate, loose)
    pub equivalence_level: EquivalenceLevel,
    /// Usage notes
    pub notes: Vec<String>,
}
/// Extracted deadline from a legal document.
#[derive(Debug, Clone)]
pub struct ExtractedDeadline {
    /// Date of the deadline (year, month, day)
    pub date: Option<(i32, u32, u32)>,
    /// Textual description of the deadline
    pub description: String,
    /// Position in document
    pub position: usize,
    /// Confidence score
    pub confidence: f64,
    /// Related obligation or clause
    pub context: String,
}
/// Cross-reference formatter for internal document references.
#[derive(Debug, Clone)]
pub struct CrossReferenceFormatter {
    locale: Locale,
}
impl CrossReferenceFormatter {
    /// Creates a new cross-reference formatter.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }
    /// Formats a cross-reference to a section.
    pub fn format_section_ref(&self, section: &str) -> String {
        match self.locale.language.as_str() {
            "en" => format!("See Section {}", section),
            "ja" => format!("第{}条参照", section),
            "de" => format!("Siehe Abschnitt {}", section),
            "fr" => format!("Voir l'article {}", section),
            "es" => format!("Véase la Sección {}", section),
            "it" => format!("Vedi Sezione {}", section),
            "pt" => format!("Veja a Seção {}", section),
            "nl" => format!("Zie Sectie {}", section),
            "pl" => format!("Zobacz Sekcja {}", section),
            "ko" => format!("제{} 조 참조", section),
            _ => format!("See Section {}", section),
        }
    }
    /// Formats a cross-reference to a page.
    pub fn format_page_ref(&self, page: usize) -> String {
        match self.locale.language.as_str() {
            "en" => format!("See page {}", page),
            "ja" => format!("{}ページ参照", page),
            "de" => format!("Siehe Seite {}", page),
            "fr" => format!("Voir page {}", page),
            "es" => format!("Véase la página {}", page),
            "it" => format!("Vedi pagina {}", page),
            "pt" => format!("Veja a página {}", page),
            "nl" => format!("Zie pagina {}", page),
            "pl" => format!("Zobacz strona {}", page),
            "ko" => format!("{} 페이지 참조", page),
            _ => format!("See page {}", page),
        }
    }
}
/// Readability assessment report.
#[derive(Debug, Clone)]
pub struct ReadabilityReport {
    /// Flesch Reading Ease score (0-100)
    pub flesch_reading_ease: f32,
    /// Flesch-Kincaid Grade Level
    pub flesch_kincaid_grade: f32,
    /// Difficulty description
    pub difficulty: String,
    /// Total word count
    pub word_count: usize,
    /// Total sentence count
    pub sentence_count: usize,
    /// Total syllable count
    pub syllable_count: usize,
}
/// Citation format suggester.
#[derive(Debug, Clone)]
pub struct CitationSuggester {
    style: CitationStyle,
}
impl CitationSuggester {
    /// Creates a new citation suggester.
    pub fn new(style: CitationStyle) -> Self {
        Self { style }
    }
    /// Suggests improvements for a case citation.
    pub fn suggest_case(&self, components: &CitationComponents) -> Vec<String> {
        let mut suggestions = Vec::new();
        let checker = CitationCompletenessChecker::new(self.style.clone());
        let report = checker.check_case(components);
        if !report.is_complete() {
            for field in &report.missing_required {
                suggestions.push(format!("Add required field: {}", field));
            }
        }
        for field in &report.missing_optional {
            suggestions.push(format!("Consider adding optional field: {}", field));
        }
        match &self.style {
            CitationStyle::Bluebook => {
                if components.volume.is_some()
                    && components.reporter.is_some()
                    && components.page.is_none()
                {
                    suggestions.push("Add page number for Bluebook format".to_string());
                }
                if components.year.is_none() {
                    suggestions.push("Add year in parentheses (Court Year)".to_string());
                }
            }
            CitationStyle::OSCOLA => {
                if components.year.is_none() {
                    suggestions.push("Add year in square brackets [Year]".to_string());
                }
                if let Some(title) = &components.title.chars().next()
                    && title.is_lowercase()
                {
                    suggestions.push("Case name should start with capital letter".to_string());
                }
            }
            CitationStyle::Japanese if components.reporter.is_none() => {
                suggestions
                    .push("Consider adding reporter name (e.g., 最高裁判所民事判例集)".to_string());
            }
            _ => {}
        }
        if components.title.is_empty() {
            suggestions.push("Title cannot be empty".to_string());
        }
        if components.title.len() > 200 {
            suggestions.push("Title seems unusually long - verify it's correct".to_string());
        }
        suggestions
    }
    /// Suggests improvements for a statute citation.
    pub fn suggest_statute(&self, components: &CitationComponents) -> Vec<String> {
        let mut suggestions = Vec::new();
        let checker = CitationCompletenessChecker::new(self.style.clone());
        let report = checker.check_statute(components);
        if !report.is_complete() {
            for field in &report.missing_required {
                suggestions.push(format!("Add required field: {}", field));
            }
        }
        match &self.style {
            CitationStyle::Bluebook if components.page.is_none() => {
                suggestions.push("Consider adding section number (§)".to_string());
            }
            CitationStyle::OSCOLA if components.year.is_none() => {
                suggestions.push("Add year for UK statutes".to_string());
            }
            _ => {}
        }
        suggestions
    }
    /// Suggests the best citation style for a jurisdiction.
    pub fn suggest_style_for_jurisdiction(jurisdiction: &str) -> CitationStyle {
        CitationFormatter::style_for_jurisdiction(jurisdiction)
    }
    /// Validates and suggests improvements in one call.
    pub fn validate_and_suggest_case(&self, components: &CitationComponents) -> ValidationReport {
        let validator = CitationValidator::new(self.style.clone());
        let errors = validator
            .validate_case(components)
            .err()
            .unwrap_or_default();
        let suggestions = self.suggest_case(components);
        let checker = CitationCompletenessChecker::new(self.style.clone());
        let completeness = checker.check_case(components);
        ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            suggestions,
            completeness,
        }
    }
    /// Validates and suggests improvements for statute.
    pub fn validate_and_suggest_statute(
        &self,
        components: &CitationComponents,
    ) -> ValidationReport {
        let validator = CitationValidator::new(self.style.clone());
        let errors = validator
            .validate_statute(components)
            .err()
            .unwrap_or_default();
        let suggestions = self.suggest_statute(components);
        let checker = CitationCompletenessChecker::new(self.style.clone());
        let completeness = checker.check_statute(components);
        ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            suggestions,
            completeness,
        }
    }
}
/// Type of template variable for validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariableType {
    /// Text string
    Text,
    /// Date value
    Date,
    /// Numeric value
    Number,
    /// Currency amount
    Currency,
    /// Boolean value
    Boolean,
    /// Email address
    Email,
    /// Address
    Address,
    /// Person name
    PersonName,
    /// List of values
    List,
}
/// Number formatter for locale-specific number formatting.
#[derive(Debug, Clone)]
pub struct NumberFormatter {
    locale: Locale,
}
impl NumberFormatter {
    /// Creates a new number formatter.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{NumberFormatter, Locale};
    ///
    /// let us_locale = Locale::new("en").with_country("US");
    /// let formatter = NumberFormatter::new(us_locale);
    /// assert!(formatter.format_integer(1234567).contains(","));
    ///
    /// let de_locale = Locale::new("de").with_country("DE");
    /// let de_formatter = NumberFormatter::new(de_locale);
    /// assert!(de_formatter.format_integer(1234567).contains("."));
    /// ```
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }
    /// Formats an integer with thousands separators.
    pub fn format_integer(&self, n: i64) -> String {
        let sign = if n < 0 { "-" } else { "" };
        let abs_n = n.abs();
        let s = abs_n.to_string();
        let separator = self.get_thousands_separator();
        let mut result = String::new();
        for (i, c) in s.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push_str(separator);
            }
            result.push(c);
        }
        format!("{}{}", sign, result.chars().rev().collect::<String>())
    }
    /// Formats a decimal number.
    pub fn format_decimal(&self, n: f64, decimal_places: usize) -> String {
        let decimal_sep = self.get_decimal_separator();
        let formatted = format!("{:.prec$}", n, prec = decimal_places);
        let parts: Vec<&str> = formatted.split('.').collect();
        if parts.len() == 2 {
            let integer_part = self.format_integer(parts[0].parse().unwrap_or(0));
            format!("{}{}{}", integer_part, decimal_sep, parts[1])
        } else {
            self.format_integer(n as i64)
        }
    }
    fn get_thousands_separator(&self) -> &str {
        match self.locale.language.as_str() {
            "de" | "es" | "it" | "pt" | "nl" => ".",
            "fr" => " ",
            "ja" | "zh" => "",
            _ => ",",
        }
    }
    fn get_decimal_separator(&self) -> &str {
        match self.locale.language.as_str() {
            "de" | "es" | "it" | "pt" | "nl" | "fr" => ",",
            _ => ".",
        }
    }
    /// Formats a percentage.
    pub fn format_percentage(&self, n: f64) -> String {
        let decimal_sep = self.get_decimal_separator();
        let formatted = format!("{:.1}", n);
        let with_sep = formatted.replace('.', decimal_sep);
        match self.locale.language.as_str() {
            "fr" | "de" => format!("{} %", with_sep),
            _ => format!("{}%", with_sep),
        }
    }
    /// Formats an ordinal number (1st, 2nd, 3rd, etc.) according to locale.
    /// Very useful for legal citations and document references.
    pub fn format_ordinal(&self, n: i64) -> String {
        match self.locale.language.as_str() {
            "en" => {
                let suffix = if n % 100 >= 11 && n % 100 <= 13 {
                    "th"
                } else {
                    match n % 10 {
                        1 => "st",
                        2 => "nd",
                        3 => "rd",
                        _ => "th",
                    }
                };
                format!("{}{}", n, suffix)
            }
            "es" => {
                if n == 1 {
                    "1º".to_string()
                } else {
                    format!("{}º", n)
                }
            }
            "fr" => {
                if n == 1 {
                    "1er".to_string()
                } else {
                    format!("{}e", n)
                }
            }
            "de" => format!("{}.", n),
            "ja" => format!("第{}", n),
            "zh" => format!("第{}", n),
            "ko" => format!("제{}", n),
            "pt" => {
                if n == 1 {
                    "1º".to_string()
                } else {
                    format!("{}º", n)
                }
            }
            "it" => {
                if n == 1 {
                    "1º".to_string()
                } else {
                    format!("{}º", n)
                }
            }
            "nl" => {
                if n == 1 {
                    "1e".to_string()
                } else {
                    format!("{}e", n)
                }
            }
            "pl" => format!("{}.", n),
            _ => format!("{}.", n),
        }
    }
    /// Converts a number to words in the specified locale.
    /// Useful for legal documents where numbers must be written out.
    /// Currently supports numbers 0-999,999.
    pub fn number_to_words(&self, n: i64) -> String {
        if n < 0 {
            match self.locale.language.as_str() {
                "en" => format!("minus {}", self.number_to_words(-n)),
                "ja" => format!("マイナス{}", self.number_to_words(-n)),
                "es" => format!("menos {}", self.number_to_words(-n)),
                "fr" => format!("moins {}", self.number_to_words(-n)),
                "de" => format!("minus {}", self.number_to_words(-n)),
                "ko" => format!("마이너스 {}", self.number_to_words(-n)),
                "pt" => format!("menos {}", self.number_to_words(-n)),
                "it" => format!("meno {}", self.number_to_words(-n)),
                "nl" => format!("min {}", self.number_to_words(-n)),
                "pl" => format!("minus {}", self.number_to_words(-n)),
                _ => format!("-{}", self.number_to_words(-n)),
            }
        } else {
            match self.locale.language.as_str() {
                "en" => self.number_to_words_en(n),
                "ja" => self.number_to_words_ja(n),
                "es" => self.number_to_words_es(n),
                "fr" => self.number_to_words_fr(n),
                "de" => self.number_to_words_de(n),
                "ko" => self.number_to_words_ko(n),
                "pt" => self.number_to_words_pt(n),
                "it" => self.number_to_words_it(n),
                "nl" => self.number_to_words_nl(n),
                "pl" => self.number_to_words_pl(n),
                _ => n.to_string(),
            }
        }
    }
    #[allow(clippy::only_used_in_recursion)]
    fn number_to_words_en(&self, n: i64) -> String {
        match n {
            0 => "zero".to_string(),
            1 => "one".to_string(),
            2 => "two".to_string(),
            3 => "three".to_string(),
            4 => "four".to_string(),
            5 => "five".to_string(),
            6 => "six".to_string(),
            7 => "seven".to_string(),
            8 => "eight".to_string(),
            9 => "nine".to_string(),
            10 => "ten".to_string(),
            11 => "eleven".to_string(),
            12 => "twelve".to_string(),
            13 => "thirteen".to_string(),
            14 => "fourteen".to_string(),
            15 => "fifteen".to_string(),
            16 => "sixteen".to_string(),
            17 => "seventeen".to_string(),
            18 => "eighteen".to_string(),
            19 => "nineteen".to_string(),
            20..=99 => {
                let tens = n / 10;
                let ones = n % 10;
                let tens_word = match tens {
                    2 => "twenty",
                    3 => "thirty",
                    4 => "forty",
                    5 => "fifty",
                    6 => "sixty",
                    7 => "seventy",
                    8 => "eighty",
                    9 => "ninety",
                    _ => "",
                };
                if ones == 0 {
                    tens_word.to_string()
                } else {
                    format!("{}-{}", tens_word, self.number_to_words_en(ones))
                }
            }
            100..=999 => {
                let hundreds = n / 100;
                let remainder = n % 100;
                if remainder == 0 {
                    format!("{} hundred", self.number_to_words_en(hundreds))
                } else {
                    format!(
                        "{} hundred and {}",
                        self.number_to_words_en(hundreds),
                        self.number_to_words_en(remainder)
                    )
                }
            }
            1000..=999_999 => {
                let thousands = n / 1000;
                let remainder = n % 1000;
                if remainder == 0 {
                    format!("{} thousand", self.number_to_words_en(thousands))
                } else {
                    format!(
                        "{} thousand {}",
                        self.number_to_words_en(thousands),
                        self.number_to_words_en(remainder)
                    )
                }
            }
            _ => n.to_string(),
        }
    }
    #[allow(dead_code)]
    #[allow(clippy::only_used_in_recursion)]
    fn number_to_words_ja(&self, n: i64) -> String {
        match n {
            0 => "零".to_string(),
            1..=9 => {
                ["一", "二", "三", "四", "五", "六", "七", "八", "九"][(n - 1) as usize].to_string()
            }
            10 => "十".to_string(),
            11..=99 => {
                let tens = n / 10;
                let ones = n % 10;
                let tens_str = if tens == 1 {
                    "十".to_string()
                } else {
                    format!("{}十", self.number_to_words_ja(tens))
                };
                if ones == 0 {
                    tens_str
                } else {
                    format!("{}{}", tens_str, self.number_to_words_ja(ones))
                }
            }
            100..=999 => {
                let hundreds = n / 100;
                let remainder = n % 100;
                let hundreds_str = if hundreds == 1 {
                    "百".to_string()
                } else {
                    format!("{}百", self.number_to_words_ja(hundreds))
                };
                if remainder == 0 {
                    hundreds_str
                } else {
                    format!("{}{}", hundreds_str, self.number_to_words_ja(remainder))
                }
            }
            1000..=9999 => {
                let thousands = n / 1000;
                let remainder = n % 1000;
                let thousands_str = if thousands == 1 {
                    "千".to_string()
                } else {
                    format!("{}千", self.number_to_words_ja(thousands))
                };
                if remainder == 0 {
                    thousands_str
                } else {
                    format!("{}{}", thousands_str, self.number_to_words_ja(remainder))
                }
            }
            10000..=99_999_999 => {
                let man = n / 10000;
                let remainder = n % 10000;
                if remainder == 0 {
                    format!("{}万", self.number_to_words_ja(man))
                } else {
                    format!(
                        "{}万{}",
                        self.number_to_words_ja(man),
                        self.number_to_words_ja(remainder)
                    )
                }
            }
            _ => n.to_string(),
        }
    }
    #[allow(dead_code)]
    fn number_to_words_es(&self, n: i64) -> String {
        match n {
            0 => "cero".to_string(),
            1 => "uno".to_string(),
            2 => "dos".to_string(),
            3 => "tres".to_string(),
            4 => "cuatro".to_string(),
            5 => "cinco".to_string(),
            6 => "seis".to_string(),
            7 => "siete".to_string(),
            8 => "ocho".to_string(),
            9 => "nueve".to_string(),
            10 => "diez".to_string(),
            _ => n.to_string(),
        }
    }
    #[allow(dead_code)]
    fn number_to_words_fr(&self, n: i64) -> String {
        match n {
            0 => "zéro".to_string(),
            1 => "un".to_string(),
            2 => "deux".to_string(),
            3 => "trois".to_string(),
            4 => "quatre".to_string(),
            5 => "cinq".to_string(),
            6 => "six".to_string(),
            7 => "sept".to_string(),
            8 => "huit".to_string(),
            9 => "neuf".to_string(),
            10 => "dix".to_string(),
            _ => n.to_string(),
        }
    }
    #[allow(dead_code)]
    fn number_to_words_de(&self, n: i64) -> String {
        match n {
            0 => "null".to_string(),
            1 => "eins".to_string(),
            2 => "zwei".to_string(),
            3 => "drei".to_string(),
            4 => "vier".to_string(),
            5 => "fünf".to_string(),
            6 => "sechs".to_string(),
            7 => "sieben".to_string(),
            8 => "acht".to_string(),
            9 => "neun".to_string(),
            10 => "zehn".to_string(),
            _ => n.to_string(),
        }
    }
    #[allow(dead_code)]
    fn number_to_words_ko(&self, n: i64) -> String {
        match n {
            0 => "영".to_string(),
            1 => "일".to_string(),
            2 => "이".to_string(),
            3 => "삼".to_string(),
            4 => "사".to_string(),
            5 => "오".to_string(),
            6 => "육".to_string(),
            7 => "칠".to_string(),
            8 => "팔".to_string(),
            9 => "구".to_string(),
            10 => "십".to_string(),
            20 => "이십".to_string(),
            30 => "삼십".to_string(),
            100 => "백".to_string(),
            1000 => "천".to_string(),
            10000 => "만".to_string(),
            _ => n.to_string(),
        }
    }
    #[allow(dead_code)]
    fn number_to_words_pt(&self, n: i64) -> String {
        match n {
            0 => "zero".to_string(),
            1 => "um".to_string(),
            2 => "dois".to_string(),
            3 => "três".to_string(),
            4 => "quatro".to_string(),
            5 => "cinco".to_string(),
            6 => "seis".to_string(),
            7 => "sete".to_string(),
            8 => "oito".to_string(),
            9 => "nove".to_string(),
            10 => "dez".to_string(),
            20 => "vinte".to_string(),
            30 => "trinta".to_string(),
            100 => "cem".to_string(),
            1000 => "mil".to_string(),
            _ => n.to_string(),
        }
    }
    #[allow(dead_code)]
    fn number_to_words_it(&self, n: i64) -> String {
        match n {
            0 => "zero".to_string(),
            1 => "uno".to_string(),
            2 => "due".to_string(),
            3 => "tre".to_string(),
            4 => "quattro".to_string(),
            5 => "cinque".to_string(),
            6 => "sei".to_string(),
            7 => "sette".to_string(),
            8 => "otto".to_string(),
            9 => "nove".to_string(),
            10 => "dieci".to_string(),
            20 => "venti".to_string(),
            30 => "trenta".to_string(),
            100 => "cento".to_string(),
            1000 => "mille".to_string(),
            _ => n.to_string(),
        }
    }
    #[allow(dead_code)]
    fn number_to_words_nl(&self, n: i64) -> String {
        match n {
            0 => "nul".to_string(),
            1 => "een".to_string(),
            2 => "twee".to_string(),
            3 => "drie".to_string(),
            4 => "vier".to_string(),
            5 => "vijf".to_string(),
            6 => "zes".to_string(),
            7 => "zeven".to_string(),
            8 => "acht".to_string(),
            9 => "negen".to_string(),
            10 => "tien".to_string(),
            20 => "twintig".to_string(),
            30 => "dertig".to_string(),
            100 => "honderd".to_string(),
            1000 => "duizend".to_string(),
            _ => n.to_string(),
        }
    }
    #[allow(dead_code)]
    fn number_to_words_pl(&self, n: i64) -> String {
        match n {
            0 => "zero".to_string(),
            1 => "jeden".to_string(),
            2 => "dwa".to_string(),
            3 => "trzy".to_string(),
            4 => "cztery".to_string(),
            5 => "pięć".to_string(),
            6 => "sześć".to_string(),
            7 => "siedem".to_string(),
            8 => "osiem".to_string(),
            9 => "dziewięć".to_string(),
            10 => "dziesięć".to_string(),
            20 => "dwadzieścia".to_string(),
            30 => "trzydzieści".to_string(),
            100 => "sto".to_string(),
            1000 => "tysiąc".to_string(),
            _ => n.to_string(),
        }
    }
}
/// Interpreted segment with source and target text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterpretedSegment {
    /// Original source segment.
    pub source_segment: TranscriptionSegment,
    /// Interpreted text in target language.
    pub target_text: String,
    /// Target language locale.
    pub target_locale: Locale,
    /// Interpretation confidence (0.0 to 1.0).
    pub interpretation_confidence: f64,
    /// Delay in milliseconds (for simultaneous interpretation).
    pub delay_ms: u64,
}
impl InterpretedSegment {
    /// Creates a new interpreted segment.
    pub fn new(
        source_segment: TranscriptionSegment,
        target_text: impl Into<String>,
        target_locale: Locale,
    ) -> Self {
        Self {
            source_segment,
            target_text: target_text.into(),
            target_locale,
            interpretation_confidence: 1.0,
            delay_ms: 0,
        }
    }
    /// Sets the interpretation confidence.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.interpretation_confidence = confidence.clamp(0.0, 1.0);
        self
    }
    /// Sets the interpretation delay.
    pub fn with_delay_ms(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }
    /// Formats the interpreted segment for display.
    pub fn format_bilingual(&self) -> String {
        format!(
            "[{}] {}\n[{}] {}",
            self.source_segment.locale.tag(),
            self.source_segment.text,
            self.target_locale.tag(),
            self.target_text
        )
    }
}
/// Cultural context annotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalContext {
    /// Locale this context applies to
    pub locale: Locale,
    /// Category of cultural context
    pub category: ContextCategory,
    /// Legal term or concept
    pub term: String,
    /// Cultural explanation
    pub explanation: String,
    /// Usage guidelines
    pub guidelines: Vec<String>,
    /// Related concepts in other cultures
    pub cross_cultural_equivalents: HashMap<String, String>,
}
impl CulturalContext {
    /// Creates a new cultural context annotation.
    pub fn new(
        locale: Locale,
        category: ContextCategory,
        term: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            locale,
            category,
            term: term.into(),
            explanation: explanation.into(),
            guidelines: Vec::new(),
            cross_cultural_equivalents: HashMap::new(),
        }
    }
    /// Adds a usage guideline.
    pub fn add_guideline(&mut self, guideline: impl Into<String>) {
        self.guidelines.push(guideline.into());
    }
    /// Adds a cross-cultural equivalent.
    pub fn add_equivalent(&mut self, culture: impl Into<String>, equivalent: impl Into<String>) {
        self.cross_cultural_equivalents
            .insert(culture.into(), equivalent.into());
    }
    /// Builder pattern for adding guideline.
    pub fn with_guideline(mut self, guideline: impl Into<String>) -> Self {
        self.add_guideline(guideline);
        self
    }
    /// Builder pattern for adding equivalent.
    pub fn with_equivalent(
        mut self,
        culture: impl Into<String>,
        equivalent: impl Into<String>,
    ) -> Self {
        self.add_equivalent(culture, equivalent);
        self
    }
}
/// Plain language generator with AI-assisted simplification.
#[derive(Debug, Clone)]
pub struct PlainLanguageGenerator {
    /// Target reading level (Flesch-Kincaid grade)
    target_grade: f64,
    /// Simplification strategies to apply
    strategies: Vec<SimplificationStrategy>,
    /// Custom jargon replacements
    pub(super) jargon_map: HashMap<String, String>,
    /// Locale for language-specific simplification
    locale: Locale,
}
impl PlainLanguageGenerator {
    /// Creates a new plain language generator.
    ///
    /// # Arguments
    ///
    /// * `target_grade` - Target reading level (Flesch-Kincaid grade, e.g., 8.0 for 8th grade)
    /// * `locale` - Locale for language-specific simplification
    pub fn new(target_grade: f64, locale: Locale) -> Self {
        Self {
            target_grade,
            strategies: vec![
                SimplificationStrategy::ReplaceJargon,
                SimplificationStrategy::ShortenSentences,
                SimplificationStrategy::SimplifyGrammar,
            ],
            jargon_map: HashMap::new(),
            locale,
        }
    }
    /// Adds a custom jargon replacement.
    pub fn add_jargon_replacement(
        mut self,
        legal_term: impl Into<String>,
        plain_term: impl Into<String>,
    ) -> Self {
        self.jargon_map.insert(legal_term.into(), plain_term.into());
        self
    }
    /// Sets the simplification strategies.
    pub fn with_strategies(mut self, strategies: Vec<SimplificationStrategy>) -> Self {
        self.strategies = strategies;
        self
    }
    /// Simplifies legal text to plain language.
    pub fn simplify(&self, text: &str) -> String {
        let mut result = text.to_string();
        for strategy in &self.strategies {
            result = match strategy {
                SimplificationStrategy::ReplaceJargon => self.replace_jargon(&result),
                SimplificationStrategy::ShortenSentences => self.shorten_sentences(&result),
                SimplificationStrategy::ActiveVoice => self.convert_to_active_voice(&result),
                SimplificationStrategy::AddContext => self.add_context(&result),
                SimplificationStrategy::SimplifyGrammar => self.simplify_grammar(&result),
            };
        }
        result
    }
    fn replace_jargon(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (legal_term, plain_term) in &self.jargon_map {
            result = result.replace(legal_term, plain_term);
        }
        let default_replacements = self.get_default_replacements();
        for (legal_term, plain_term) in default_replacements {
            result = result.replace(legal_term, plain_term);
        }
        result
    }
    fn get_default_replacements(&self) -> Vec<(&'static str, &'static str)> {
        match self.locale.language.as_str() {
            "en" => {
                vec![
                    ("hereinafter", "from now on"),
                    ("whereas", "because"),
                    ("pursuant to", "according to"),
                    ("notwithstanding", "despite"),
                    ("forthwith", "immediately"),
                    ("heretofore", "before now"),
                    ("hereby", "by this document"),
                    ("aforementioned", "mentioned above"),
                    ("commence", "start"),
                    ("terminate", "end"),
                ]
            }
            "ja" => {
                vec![
                    ("以下", "これから"),
                    ("前述", "上で述べた"),
                    ("規定", "ルール"),
                    ("条項", "項目"),
                ]
            }
            _ => vec![],
        }
    }
    fn shorten_sentences(&self, text: &str) -> String {
        text.replace(", and ", ". Also, ")
            .replace(", but ", ". However, ")
            .replace("; ", ". ")
    }
    fn convert_to_active_voice(&self, text: &str) -> String {
        text.replace("is required to", "must")
            .replace("shall be", "will be")
            .replace("is prohibited from", "cannot")
    }
    fn add_context(&self, text: &str) -> String {
        text.replace("liability", "liability (legal responsibility)")
            .replace("indemnify", "indemnify (compensate for loss or damage)")
    }
    fn simplify_grammar(&self, text: &str) -> String {
        text.replace("shall", "will")
            .replace("may not", "cannot")
            .replace("such", "this")
            .replace("said", "the")
    }
    /// Estimates reading level of text.
    pub fn estimate_reading_level(&self, text: &str) -> f64 {
        let assessor = ReadingLevelAssessor::new();
        assessor.flesch_kincaid_grade(text) as f64
    }
    /// Checks if text meets target reading level.
    pub fn meets_target(&self, text: &str) -> bool {
        self.estimate_reading_level(text) <= self.target_grade
    }
}
/// Term preservation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermPreservationMode {
    /// Preserve terms exactly as-is (no translation)
    Exact,
    /// Translate term but preserve formatting
    PreserveFormatting,
    /// Translate with glossary enforcement
    GlossaryEnforced,
}
/// Simultaneous interpretation engine with streaming support.
#[derive(Debug, Clone)]
pub struct SimultaneousInterpreter {
    /// Source language locale.
    pub source_locale: Locale,
    /// Target language locale.
    pub target_locale: Locale,
    /// Interpretation mode.
    pub mode: InterpretationMode,
    /// Legal domain for terminology.
    pub domain: LegalSpeechDomain,
    /// Source language transcriber.
    pub transcriber: LegalSpeechTranscriber,
    /// Translation manager for legal terms.
    pub translation_manager: Option<Arc<TranslationManager>>,
    /// Maximum acceptable delay in milliseconds (for simultaneous mode).
    pub max_delay_ms: u64,
}
impl SimultaneousInterpreter {
    /// Creates a new simultaneous interpreter.
    pub fn new(source_locale: Locale, target_locale: Locale, domain: LegalSpeechDomain) -> Self {
        Self {
            transcriber: LegalSpeechTranscriber::new(source_locale.clone(), domain),
            source_locale,
            target_locale,
            mode: InterpretationMode::Simultaneous,
            domain,
            translation_manager: None,
            max_delay_ms: 3000,
        }
    }
    /// Creates a simultaneous interpreter for court proceedings.
    pub fn for_court_proceedings(source_locale: Locale, target_locale: Locale) -> Self {
        let mut interpreter = Self::new(
            source_locale.clone(),
            target_locale,
            LegalSpeechDomain::CourtProceedings,
        );
        interpreter.transcriber = LegalSpeechTranscriber::for_court_proceedings(source_locale);
        interpreter
    }
    /// Sets the interpretation mode.
    pub fn with_mode(mut self, mode: InterpretationMode) -> Self {
        self.mode = mode;
        self
    }
    /// Sets the translation manager.
    pub fn with_translation_manager(mut self, manager: Arc<TranslationManager>) -> Self {
        self.translation_manager = Some(manager);
        self
    }
    /// Sets the maximum acceptable delay.
    pub fn with_max_delay(mut self, max_delay_ms: u64) -> Self {
        self.max_delay_ms = max_delay_ms;
        self
    }
    /// Interprets a transcription segment in real-time.
    pub fn interpret_segment(&self, segment: TranscriptionSegment) -> InterpretedSegment {
        let delay_ms = match self.mode {
            InterpretationMode::Simultaneous => 200,
            InterpretationMode::Consecutive => 0,
            InterpretationMode::Whispered => 300,
        };
        let target_text = if let Some(ref tm) = self.translation_manager {
            tm.translate(&segment.text, &self.target_locale)
                .unwrap_or_else(|_| segment.text.clone())
        } else {
            segment.text.clone()
        };
        let combined_confidence = segment.confidence * 0.9;
        InterpretedSegment::new(segment, target_text, self.target_locale.clone())
            .with_confidence(combined_confidence)
            .with_delay_ms(delay_ms)
    }
    /// Processes streaming audio and returns interpreted segments.
    pub fn interpret_stream(&self, audio_chunks: &[&[u8]]) -> Vec<InterpretedSegment> {
        let mut interpreted_segments = Vec::new();
        for chunk in audio_chunks {
            let segments = self.transcriber.transcribe(chunk);
            for segment in segments {
                let interpreted = self.interpret_segment(segment);
                interpreted_segments.push(interpreted);
            }
        }
        interpreted_segments
    }
}
/// Comprehensive validation report.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether citation is valid
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<CitationError>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
    /// Completeness report
    pub completeness: CompletenessReport,
}
impl ValidationReport {
    /// Gets a human-readable summary.
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        if self.is_valid {
            lines.push("✓ Citation is valid".to_string());
        } else {
            lines.push(format!("✗ Citation has {} error(s):", self.errors.len()));
            for error in &self.errors {
                lines.push(format!("  - {}", error));
            }
        }
        lines.push(format!("\n{}", self.completeness.summary()));
        if !self.suggestions.is_empty() {
            lines.push(format!("\nSuggestions ({}):", self.suggestions.len()));
            for suggestion in &self.suggestions {
                lines.push(format!("  • {}", suggestion));
            }
        }
        lines.join("\n")
    }
}
/// Colonial legacy mapper.
#[derive(Debug, Clone, Default)]
pub struct ColonialLegacyMapper {
    /// Legacies indexed by jurisdiction
    legacies: HashMap<String, ColonialLegacy>,
}
impl ColonialLegacyMapper {
    /// Creates a new mapper.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a mapper with default legacies.
    pub fn with_defaults() -> Self {
        let mut mapper = Self::new();
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::British, "India")
                .with_retained_concept("Common law system")
                .with_retained_concept("Adversarial legal procedure")
                .with_retained_concept("Judicial precedent")
                .with_retained_concept("Westminster parliamentary system")
                .with_hybrid_concept("Anglo-Hindu law", "Hindu personal law")
                .with_hybrid_concept("Anglo-Muhammadan law", "Islamic personal law")
                .with_reform("Constitution of India 1950 (republican)")
                .with_reform("Hindu Code Bills (modernization of personal law)"),
        );
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::British, "Hong Kong")
                .with_retained_concept("Common law")
                .with_retained_concept("Basic Law")
                .with_hybrid_concept(
                    "One country, two systems",
                    "Chinese sovereignty + British legal system",
                )
                .with_reform("Handover to China 1997"),
        );
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::French, "Algeria")
                .with_retained_concept("Civil law system")
                .with_retained_concept("Code-based legal framework")
                .with_hybrid_concept("French civil law + Sharia", "Personal status law")
                .with_reform("Arabization of legal system post-independence")
                .with_reform("Family Code 1984 (Islamic family law)"),
        );
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::Spanish, "Philippines")
                .with_retained_concept("Civil law foundation")
                .with_retained_concept("Catholic Canon law influence")
                .with_hybrid_concept(
                    "Spanish civil law + American common law",
                    "Mixed legal system",
                )
                .with_reform("Constitution of 1987 (post-Marcos democratic reforms)"),
        );
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::Portuguese, "Brazil")
                .with_retained_concept("Civil law system")
                .with_retained_concept("Inquisitorial procedure")
                .with_reform("Constitution of 1988 (democratic transition)")
                .with_reform("New Civil Code 2002"),
        );
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::Dutch, "Indonesia")
                .with_retained_concept("Civil law system (based on Dutch Civil Code)")
                .with_hybrid_concept("Adat law", "Customary indigenous law")
                .with_hybrid_concept("Islamic law in Aceh", "Special autonomy for Islamic law")
                .with_reform("Constitution of 1945 (Pancasila principles)"),
        );
        mapper.add_legacy(
            ColonialLegacy::new(ColonialPower::Belgian, "Rwanda")
                .with_retained_concept("Civil law system")
                .with_hybrid_concept(
                    "Gacaca courts",
                    "Traditional community justice + modern genocide trials",
                )
                .with_reform("Post-genocide justice system reforms"),
        );
        mapper
    }
    /// Adds a legacy.
    pub fn add_legacy(&mut self, legacy: ColonialLegacy) {
        self.legacies.insert(legacy.jurisdiction.clone(), legacy);
    }
    /// Gets legacy for a jurisdiction.
    pub fn get_legacy(&self, jurisdiction: &str) -> Option<&ColonialLegacy> {
        self.legacies.get(jurisdiction)
    }
    /// Gets all legacies for a colonial power.
    pub fn get_by_colonial_power(&self, power: ColonialPower) -> Vec<&ColonialLegacy> {
        self.legacies
            .values()
            .filter(|l| l.colonial_power == power)
            .collect()
    }
    /// Returns the number of mapped legacies.
    pub fn legacy_count(&self) -> usize {
        self.legacies.len()
    }
}
/// Regulatory equivalence level between jurisdictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegulatoryEquivalenceLevel {
    /// Full equivalence (mutual recognition).
    Full,
    /// Conditional equivalence (with specific requirements).
    Conditional,
    /// Partial equivalence (limited recognition).
    Partial,
    /// No equivalence.
    NoEquivalence,
}
/// Legal document numbering styles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NumberingStyle {
    /// Article 1, Section 2, Paragraph 3
    Article,
    /// Section 1, Subsection a, Clause i
    Section,
    /// Chapter 1, Part A, Subdivision (1)
    Chapter,
    /// 1. a. i.
    Hierarchical,
    /// (1), (a), (i)
    Parenthetical,
}
/// Glossary enforcer for terminology consistency.
pub struct GlossaryEnforcer {
    /// Mandatory terms (source -> target)
    mandatory_terms: HashMap<String, String>,
    /// Forbidden terms (terms that should not appear)
    forbidden_terms: Vec<String>,
    /// Case-sensitive enforcement
    case_sensitive: bool,
}
impl GlossaryEnforcer {
    /// Creates a new glossary enforcer.
    pub fn new() -> Self {
        Self {
            mandatory_terms: HashMap::new(),
            forbidden_terms: Vec::new(),
            case_sensitive: false,
        }
    }
    /// Adds a mandatory term mapping.
    pub fn add_mandatory_term(
        &mut self,
        source_term: impl Into<String>,
        target_term: impl Into<String>,
    ) {
        self.mandatory_terms
            .insert(source_term.into(), target_term.into());
    }
    /// Adds a forbidden term.
    pub fn add_forbidden_term(&mut self, term: impl Into<String>) {
        self.forbidden_terms.push(term.into());
    }
    /// Enables case-sensitive enforcement.
    pub fn with_case_sensitive(mut self, enabled: bool) -> Self {
        self.case_sensitive = enabled;
        self
    }
    /// Enforces glossary on translation.
    pub fn enforce(&self, source: &str, translation: &str) -> (String, Vec<GlossaryViolation>) {
        let mut enforced = translation.to_string();
        let mut violations = Vec::new();
        for (source_term, target_term) in &self.mandatory_terms {
            let source_match = if self.case_sensitive {
                source.contains(source_term)
            } else {
                source.to_lowercase().contains(&source_term.to_lowercase())
            };
            let target_match = if self.case_sensitive {
                enforced.contains(target_term)
            } else {
                enforced
                    .to_lowercase()
                    .contains(&target_term.to_lowercase())
            };
            if source_match && !target_match {
                violations.push(GlossaryViolation {
                    violation_type: ViolationType::MissingMandatoryTerm,
                    term: source_term.clone(),
                    expected: Some(target_term.clone()),
                    found: None,
                });
            }
        }
        for forbidden in &self.forbidden_terms {
            let contains = if self.case_sensitive {
                enforced.contains(forbidden)
            } else {
                enforced.to_lowercase().contains(&forbidden.to_lowercase())
            };
            if contains {
                violations.push(GlossaryViolation {
                    violation_type: ViolationType::ForbiddenTermUsed,
                    term: forbidden.clone(),
                    expected: None,
                    found: Some(forbidden.clone()),
                });
                if self.case_sensitive {
                    enforced = enforced.replace(forbidden, "[REMOVED]");
                } else {
                    enforced = enforced.replace(forbidden, "[FORBIDDEN]");
                }
            }
        }
        (enforced, violations)
    }
    /// Returns the number of mandatory terms.
    pub fn mandatory_term_count(&self) -> usize {
        self.mandatory_terms.len()
    }
    /// Returns the number of forbidden terms.
    pub fn forbidden_term_count(&self) -> usize {
        self.forbidden_terms.len()
    }
}
/// Treaty type for language standardization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreatyType {
    /// Bilateral treaty between two countries.
    Bilateral,
    /// Multilateral treaty among multiple countries.
    Multilateral,
    /// UN treaty.
    UNTreaty,
    /// Regional treaty (EU, ASEAN, etc.).
    Regional,
    /// Trade agreement.
    TradeAgreement,
    /// Human rights treaty.
    HumanRights,
    /// Environmental treaty.
    Environmental,
}
/// Quality estimation metric for AI translations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualityMetric {
    /// Semantic accuracy (meaning preservation).
    SemanticAccuracy,
    /// Terminological consistency.
    TerminologicalConsistency,
    /// Grammatical correctness.
    GrammaticalCorrectness,
    /// Style appropriateness.
    StyleAppropriateness,
    /// Citation preservation.
    CitationPreservation,
    /// Fluency.
    Fluency,
}
/// Batch translation operations with parallel processing support.
pub struct BatchTranslator {
    manager: Arc<TranslationManager>,
}
impl BatchTranslator {
    /// Creates a new batch translator from a translation manager.
    pub fn new(manager: TranslationManager) -> Self {
        Self {
            manager: Arc::new(manager),
        }
    }
    /// Translates multiple keys in parallel for a given locale.
    /// Returns results in the same order as input keys.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{BatchTranslator, TranslationManager, LegalDictionary, Locale};
    ///
    /// let mut manager = TranslationManager::new();
    /// let mut dict = LegalDictionary::new(Locale::new("ja").with_country("JP"));
    /// dict.add_translation("contract", "契約");
    /// dict.add_translation("law", "法律");
    /// manager.add_dictionary(dict);
    ///
    /// let batch = BatchTranslator::new(manager);
    /// let keys = vec!["contract", "law"];
    /// let locale = Locale::new("ja").with_country("JP");
    ///
    /// let results = batch.translate_batch(&keys, &locale);
    /// assert_eq!(results.len(), 2);
    /// ```
    pub fn translate_batch(&self, keys: &[&str], locale: &Locale) -> Vec<I18nResult<String>> {
        keys.par_iter()
            .map(|key| self.manager.translate(key, locale))
            .collect()
    }
    /// Translates multiple key-locale pairs in parallel.
    /// Useful for translating different terms to different locales simultaneously.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{BatchTranslator, TranslationManager, LegalDictionary, Locale};
    ///
    /// let mut manager = TranslationManager::new();
    ///
    /// let mut ja_dict = LegalDictionary::new(Locale::new("ja").with_country("JP"));
    /// ja_dict.add_translation("contract", "契約");
    /// manager.add_dictionary(ja_dict);
    ///
    /// let mut de_dict = LegalDictionary::new(Locale::new("de").with_country("DE"));
    /// de_dict.add_translation("contract", "Vertrag");
    /// manager.add_dictionary(de_dict);
    ///
    /// let batch = BatchTranslator::new(manager);
    /// let ja_locale = Locale::new("ja").with_country("JP");
    /// let de_locale = Locale::new("de").with_country("DE");
    ///
    /// let pairs = vec![
    ///     ("contract", ja_locale),
    ///     ("contract", de_locale.clone()),
    /// ];
    ///
    /// let results = batch.translate_pairs(&pairs);
    /// assert_eq!(results.len(), 2);
    /// ```
    pub fn translate_pairs(&self, pairs: &[(&str, Locale)]) -> Vec<I18nResult<String>> {
        pairs
            .par_iter()
            .map(|(key, locale)| self.manager.translate(key, locale))
            .collect()
    }
}
/// Historical context annotator.
#[derive(Debug, Clone)]
pub struct HistoricalContextAnnotator {
    /// Contexts indexed by term
    pub(super) contexts: HashMap<String, Vec<HistoricalContext>>,
}
impl HistoricalContextAnnotator {
    /// Creates a new historical context annotator.
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }
    /// Creates an annotator with default historical contexts.
    pub fn with_defaults() -> Self {
        let mut annotator = Self::new();
        annotator.add_context(
            HistoricalContext::new(
                "Magna Carta",
                HistoricalPeriod::MiddleEnglish,
                "Charter signed by King John in 1215 at Runnymede",
                "Established principle that everyone, including the king, is subject to the law",
            )
            .with_modern_relevance("Foundation of constitutional law and due process")
            .add_related_document("Petition of Right (1628)")
            .add_related_document("Bill of Rights (1689)"),
        );
        annotator.add_context(
            HistoricalContext::new(
                "trial by jury",
                HistoricalPeriod::MiddleEnglish,
                "Established in England following the Assize of Clarendon (1166)",
                "Replaced trial by ordeal and compurgation with judgment by peers",
            )
            .with_modern_relevance("Fundamental right in common law jurisdictions")
            .add_related_document("Sixth Amendment (US Constitution)")
            .add_related_document("Seventh Amendment (US Constitution)"),
        );
        annotator.add_context(
            HistoricalContext::new(
                "habeas corpus",
                HistoricalPeriod::MiddleEnglish,
                "Developed in medieval England as protection against unlawful detention",
                "Required authorities to bring detained persons before a court",
            )
            .with_modern_relevance("Core protection against arbitrary detention worldwide")
            .add_related_document("Habeas Corpus Act 1679")
            .add_related_document("US Constitution Article I, Section 9"),
        );
        annotator.add_context(
            HistoricalContext::new(
                "equity",
                HistoricalPeriod::MiddleEnglish,
                "Developed in Court of Chancery to provide remedies unavailable at common law",
                "Offered flexible relief based on fairness when common law was too rigid",
            )
            .with_modern_relevance(
                "Equitable remedies (injunctions, specific performance) still used",
            )
            .add_related_document("Judicature Acts (1873-1875)")
            .add_related_document("Earl of Oxford's Case (1615)"),
        );
        annotator.add_context(
            HistoricalContext::new(
                "Statute of Frauds",
                HistoricalPeriod::EarlyModern,
                "Enacted in 1677 to prevent fraud in certain contracts",
                "Required certain contracts to be in writing to be enforceable",
            )
            .with_modern_relevance(
                "Modern statutes of frauds still require written evidence for land sales, etc.",
            )
            .add_related_document("Statute of Frauds 1677 (29 Car. 2 c. 3)"),
        );
        annotator.add_context(
            HistoricalContext::new(
                "Bill of Rights",
                HistoricalPeriod::Enlightenment,
                "English Bill of Rights 1689 following Glorious Revolution",
                "Established parliamentary supremacy and limited royal prerogative",
            )
            .with_modern_relevance("Model for constitutional rights documents worldwide")
            .add_related_document("US Bill of Rights (1791)")
            .add_related_document("Canadian Charter of Rights (1982)"),
        );
        annotator
    }
    /// Adds a historical context.
    pub fn add_context(&mut self, context: HistoricalContext) {
        self.contexts
            .entry(context.term.clone())
            .or_default()
            .push(context);
    }
    /// Gets contexts for a term.
    pub fn get_contexts(&self, term: &str) -> Vec<&HistoricalContext> {
        self.contexts
            .get(term)
            .map(|contexts| contexts.iter().collect())
            .unwrap_or_default()
    }
    /// Gets all contexts by historical period.
    pub fn get_by_period(&self, period: HistoricalPeriod) -> Vec<&HistoricalContext> {
        self.contexts
            .values()
            .flatten()
            .filter(|c| c.period == period)
            .collect()
    }
    /// Returns the number of annotated terms.
    pub fn context_count(&self) -> usize {
        self.contexts.len()
    }
}
/// Neural machine translation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTTranslation {
    /// Translated text
    pub text: String,
    /// Quality estimation score (0.0 to 1.0)
    pub quality_score: QualityScore,
    /// Source locale
    pub source_locale: Locale,
    /// Target locale
    pub target_locale: Locale,
    /// Engine used
    pub engine: TranslationEngine,
    /// Alternative translations (n-best list)
    pub alternatives: Vec<(String, QualityScore)>,
}
/// Scope of language in ISO 639-3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageScope {
    /// Individual language.
    Individual,
    /// Macrolanguage (group of closely related languages).
    Macrolanguage,
    /// Special code.
    Special,
}
/// International standard type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StandardType {
    /// ISO standard.
    ISO,
    /// IEC standard.
    IEC,
    /// ITU standard.
    ITU,
    /// IETF standard (RFC).
    IETF,
    /// W3C standard.
    W3C,
    /// UNCITRAL standard.
    UNCITRAL,
    /// Hague Conference standard.
    HagueConference,
}
