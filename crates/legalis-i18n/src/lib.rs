//! Legalis-I18n: Internationalization support for Legalis-RS.
//!
//! This crate provides multi-language and multi-jurisdiction support:
//! - Translation of legal terms and statutes
//! - Locale-specific legal formatting (dates, currencies, names)
//! - Jurisdiction mapping and legal system classification
//! - Cultural parameter injection for law porting

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors during internationalization operations.
#[derive(Debug, Error)]
pub enum I18nError {
    #[error("Locale not found: {0}")]
    LocaleNotFound(String),

    #[error("Translation missing for key '{key}' in locale '{locale}'")]
    TranslationMissing { key: String, locale: String },

    #[error("Invalid locale format: {0}")]
    InvalidLocale(String),

    #[error("Jurisdiction not supported: {0}")]
    UnsupportedJurisdiction(String),
}

/// Result type for i18n operations.
pub type I18nResult<T> = Result<T, I18nError>;

/// Language/locale identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Locale {
    /// ISO 639-1 language code (e.g., "ja", "en", "fr")
    pub language: String,
    /// ISO 3166-1 country code (e.g., "JP", "US", "FR")
    pub country: Option<String>,
    /// Script variant (e.g., "Latn", "Hans")
    pub script: Option<String>,
}

impl Locale {
    /// Creates a new locale.
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            country: None,
            script: None,
        }
    }

    /// Sets the country.
    pub fn with_country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Sets the script.
    pub fn with_script(mut self, script: impl Into<String>) -> Self {
        self.script = Some(script.into());
        self
    }

    /// Returns the full locale tag (e.g., "ja-JP", "en-US").
    pub fn tag(&self) -> String {
        let mut tag = self.language.clone();
        if let Some(ref script) = self.script {
            tag.push('-');
            tag.push_str(script);
        }
        if let Some(ref country) = self.country {
            tag.push('-');
            tag.push_str(country);
        }
        tag
    }

    /// Parses a locale from a tag string.
    pub fn parse(tag: &str) -> I18nResult<Self> {
        let parts: Vec<&str> = tag.split('-').collect();
        if parts.is_empty() {
            return Err(I18nError::InvalidLocale(tag.to_string()));
        }

        let mut locale = Self::new(parts[0]);
        for part in parts.iter().skip(1) {
            if part.len() == 2 && part.chars().all(|c| c.is_ascii_uppercase()) {
                locale.country = Some(part.to_string());
            } else if part.len() == 4 {
                locale.script = Some(part.to_string());
            }
        }

        Ok(locale)
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag())
    }
}

/// Legal system classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalSystem {
    /// Civil law (codified statutes) - Japan, France, Germany
    CivilLaw,
    /// Common law (case precedent) - UK, US, Australia
    CommonLaw,
    /// Religious law - Saudi Arabia, Iran
    ReligiousLaw,
    /// Customary law - Indigenous systems
    CustomaryLaw,
    /// Mixed system
    Mixed,
}

/// Jurisdiction definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// Unique identifier (ISO 3166-1 alpha-2 or custom)
    pub id: String,
    /// Display name
    pub name: String,
    /// Primary locale
    pub locale: Locale,
    /// Legal system type
    pub legal_system: LegalSystem,
    /// Parent jurisdiction (for federated systems)
    pub parent: Option<String>,
    /// Cultural parameters affecting law interpretation
    pub cultural_params: CulturalParams,
}

impl Jurisdiction {
    /// Creates a new jurisdiction.
    pub fn new(id: impl Into<String>, name: impl Into<String>, locale: Locale) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            locale,
            legal_system: LegalSystem::CivilLaw,
            parent: None,
            cultural_params: CulturalParams::default(),
        }
    }

    /// Sets the legal system.
    pub fn with_legal_system(mut self, system: LegalSystem) -> Self {
        self.legal_system = system;
        self
    }

    /// Sets cultural parameters.
    pub fn with_cultural_params(mut self, params: CulturalParams) -> Self {
        self.cultural_params = params;
        self
    }
}

/// Cultural parameters affecting legal interpretation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CulturalParams {
    /// Age of majority
    pub age_of_majority: Option<u8>,
    /// Religious considerations
    pub religious_considerations: Vec<String>,
    /// Protected classes/categories
    pub protected_classes: Vec<String>,
    /// Prohibited activities/substances
    pub prohibitions: Vec<String>,
    /// Custom parameters
    pub custom: HashMap<String, String>,
}

impl CulturalParams {
    /// Creates default parameters for Japan.
    pub fn japan() -> Self {
        Self {
            age_of_majority: Some(18),
            religious_considerations: vec![],
            protected_classes: vec![
                "gender".to_string(),
                "disability".to_string(),
                "nationality".to_string(),
            ],
            prohibitions: vec![],
            custom: HashMap::new(),
        }
    }

    /// Creates default parameters for a given country.
    pub fn for_country(country_code: &str) -> Self {
        match country_code {
            "JP" => Self::japan(),
            "US" => Self {
                age_of_majority: Some(18),
                religious_considerations: vec![],
                protected_classes: vec![
                    "race".to_string(),
                    "color".to_string(),
                    "religion".to_string(),
                    "sex".to_string(),
                    "national_origin".to_string(),
                ],
                prohibitions: vec![],
                custom: HashMap::new(),
            },
            _ => Self::default(),
        }
    }
}

/// Translation dictionary for legal terms.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LegalDictionary {
    /// Locale this dictionary is for
    pub locale: Locale,
    /// Term translations: key -> translated term
    translations: IndexMap<String, String>,
    /// Legal definitions: term -> definition
    definitions: IndexMap<String, String>,
}

impl LegalDictionary {
    /// Creates a new dictionary for a locale.
    pub fn new(locale: Locale) -> Self {
        Self {
            locale,
            translations: IndexMap::new(),
            definitions: IndexMap::new(),
        }
    }

    /// Adds a translation.
    pub fn add_translation(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.translations.insert(key.into(), value.into());
    }

    /// Adds a definition.
    pub fn add_definition(&mut self, term: impl Into<String>, definition: impl Into<String>) {
        self.definitions.insert(term.into(), definition.into());
    }

    /// Gets a translation.
    pub fn translate(&self, key: &str) -> Option<&str> {
        self.translations.get(key).map(|s| s.as_str())
    }

    /// Gets a definition.
    pub fn define(&self, term: &str) -> Option<&str> {
        self.definitions.get(term).map(|s| s.as_str())
    }
}

/// Multi-locale translation manager.
#[derive(Debug, Default)]
pub struct TranslationManager {
    dictionaries: HashMap<String, LegalDictionary>,
    fallback_locale: Option<Locale>,
}

impl TranslationManager {
    /// Creates a new translation manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the fallback locale.
    pub fn with_fallback(mut self, locale: Locale) -> Self {
        self.fallback_locale = Some(locale);
        self
    }

    /// Adds a dictionary.
    pub fn add_dictionary(&mut self, dict: LegalDictionary) {
        self.dictionaries.insert(dict.locale.tag(), dict);
    }

    /// Translates a key for a locale.
    pub fn translate(&self, key: &str, locale: &Locale) -> I18nResult<String> {
        // Try exact locale match
        if let Some(dict) = self.dictionaries.get(&locale.tag()) {
            if let Some(translation) = dict.translate(key) {
                return Ok(translation.to_string());
            }
        }

        // Try language-only match
        if let Some(dict) = self.dictionaries.get(&locale.language) {
            if let Some(translation) = dict.translate(key) {
                return Ok(translation.to_string());
            }
        }

        // Try fallback
        if let Some(ref fallback) = self.fallback_locale {
            if let Some(dict) = self.dictionaries.get(&fallback.tag()) {
                if let Some(translation) = dict.translate(key) {
                    return Ok(translation.to_string());
                }
            }
        }

        Err(I18nError::TranslationMissing {
            key: key.to_string(),
            locale: locale.tag(),
        })
    }
}

/// Jurisdiction registry.
#[derive(Debug, Default)]
pub struct JurisdictionRegistry {
    jurisdictions: HashMap<String, Jurisdiction>,
}

impl JurisdictionRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with standard jurisdictions.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        registry.register(
            Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
                .with_legal_system(LegalSystem::CivilLaw)
                .with_cultural_params(CulturalParams::japan()),
        );

        registry.register(
            Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
                .with_legal_system(LegalSystem::CommonLaw)
                .with_cultural_params(CulturalParams::for_country("US")),
        );

        registry.register(
            Jurisdiction::new("DE", "Germany", Locale::new("de").with_country("DE"))
                .with_legal_system(LegalSystem::CivilLaw),
        );

        registry.register(
            Jurisdiction::new("FR", "France", Locale::new("fr").with_country("FR"))
                .with_legal_system(LegalSystem::CivilLaw),
        );

        registry
    }

    /// Registers a jurisdiction.
    pub fn register(&mut self, jurisdiction: Jurisdiction) {
        self.jurisdictions.insert(jurisdiction.id.clone(), jurisdiction);
    }

    /// Gets a jurisdiction by ID.
    pub fn get(&self, id: &str) -> Option<&Jurisdiction> {
        self.jurisdictions.get(id)
    }

    /// Lists all registered jurisdictions.
    pub fn list(&self) -> Vec<&Jurisdiction> {
        self.jurisdictions.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_parsing() {
        let locale = Locale::parse("ja-JP").unwrap();
        assert_eq!(locale.language, "ja");
        assert_eq!(locale.country, Some("JP".to_string()));
    }

    #[test]
    fn test_locale_tag() {
        let locale = Locale::new("en").with_country("US");
        assert_eq!(locale.tag(), "en-US");
    }

    #[test]
    fn test_translation_manager() {
        let mut manager = TranslationManager::new();

        let mut ja_dict = LegalDictionary::new(Locale::new("ja").with_country("JP"));
        ja_dict.add_translation("adult", "成人");
        ja_dict.add_translation("statute", "法律");

        manager.add_dictionary(ja_dict);

        let locale = Locale::new("ja").with_country("JP");
        assert_eq!(manager.translate("adult", &locale).unwrap(), "成人");
    }

    #[test]
    fn test_jurisdiction_registry() {
        let registry = JurisdictionRegistry::with_defaults();
        let japan = registry.get("JP").unwrap();
        assert_eq!(japan.name, "Japan");
        assert_eq!(japan.legal_system, LegalSystem::CivilLaw);
    }

    #[test]
    fn test_cultural_params() {
        let params = CulturalParams::japan();
        assert_eq!(params.age_of_majority, Some(18));
    }
}
